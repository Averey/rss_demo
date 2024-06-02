use reqwest::header::{HeaderMap};
use rusqlite::{named_params, Connection};

use crate::db::Table;
use crate::models::channel_item::ChannelItem;

fn get_header_value(headers: &HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .and_then(|s| s.to_str().ok())
        .map(|s| s.to_string())
}

#[derive(Debug)]
pub struct Image {}

#[derive(Default, Debug)]
pub struct Channel {
    pub id: Option<usize>,
    pub title: String,
    pub link: String,
    pub description: String,
    pub image: Option<Image>,
    pub items: Option<Vec<ChannelItem>>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,

    pub is_proxy: bool,

    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

impl Channel {
    pub async fn subscribe(link: &str, title: Option<&str>, is_proxy: bool) -> Result<Channel, Box<dyn std::error::Error>> {
        let client = if is_proxy {
            let proxy_url = "http://127.0.0.1:9001";
            let http_proxy = reqwest::Proxy::http(proxy_url)?;
            let https_proxy = reqwest::Proxy::https(proxy_url)?;
            reqwest::Client::builder()
                .proxy(http_proxy)
                .proxy(https_proxy)
                .build()?
        } else {
            reqwest::Client::builder().build()?
        };

        let res = client.get(link).send().await?;
        let headers = res.headers().clone();
        let conent = res.bytes().await?;
        let c = rss::Channel::read_from(&conent[..])?;
        Ok(Channel {
            id: None,
            //title: if title.is_some() {title.unwrap().to_string()} else {c.title},
            title: match title { Some(t) => t.to_string(), None => c.title},
            link: String::from(link),
            description: c.description,
            image: None,
            items: None,
            pub_date: c.pub_date,
            last_build_date: c.last_build_date,
            is_proxy,
            etag: get_header_value(&headers, "etag"),
            last_modified: get_header_value(&headers, "last-modified"),
        })
    }
}

impl Table<Channel> for Channel {
   fn create_table(conn: &Connection) -> rusqlite::Result<usize, rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS channel (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            link TEXT UNIQUE NOT NULL,
            description TEXT,
            pub_date TEXT,
            last_build_date TEXT,
            is_proxy INTEGER DEFAULT 0,
            etag TEXT,
            last_modified TEXT
            )";
        conn.execute(&sql, ())
    }

    fn upsert(conn: &Connection, data: &Channel) -> rusqlite::Result<usize, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "INSERT INTO channel (title, link, description, pub_date, last_build_date, is_proxy, etag, last_modified) 
            VALUES (:title, :link, :description, :pub_date, :last_build_date, :is_proxy, :etag, :last_modified)
            ON CONFLICT(link) DO UPDATE SET 
                title=excluded.title,
                description=excluded.description,
                pub_date=excluded.pub_date,
                last_build_date=excluded.last_build_date,
                is_proxy=excluded.is_proxy,
                etag=excluded.etag,
                last_modified=excluded.last_modified")?;
        eprintln!("{:?}", data);
        let res = stmt.execute(named_params! {
            ":title": data.title,
            ":link": data.link,
            ":description": data.description,
            ":pub_date": data.pub_date,
            ":last_build_date": data.last_build_date,
            ":is_proxy": data.is_proxy,
            ":etag": data.etag,
            ":last_modified": data.last_modified,
        })?;
        Ok(res)
    }

    fn list(conn: &Connection, offset: usize, limit: usize) -> Result<Vec<Channel>, rusqlite::Error> {
        //let mut stmt = prepare_and_bind!(conn, "SELECT * FROM channel OFFSET :offset LIMIT :limit;");
        let mut stmt = conn.prepare("SELECT * FROM channel LIMIT :limit OFFSET :offset")?;
        let res = stmt.query_map(named_params! {":offset": offset, ":limit": limit}, |row| {
            Ok(Channel {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                link: row.get(2)?,
                description: row.get(3)?,
                pub_date: row.get(4)?,
                last_build_date: row.get(5)?,
                is_proxy: row.get(6)?,
                etag: row.get(7)?,
                last_modified: row.get(8)?,

                image: None,
                items: None,
            })
        })?;
        res.collect()
    }
}

