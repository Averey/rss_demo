use rusqlite::Connection;

use crate::models::channel_item::ChannelItem;

#[derive(Debug)]
pub struct Image {

}

#[derive(Default, Debug)]
pub struct Channel {
  pub title: String,
  pub link: String,
  pub description: String,
  pub image: Option<Image>,
  pub items: Vec<ChannelItem>,
  pub pub_date: Option<String>,
  pub last_build_date: Option<String>,

  pub is_proxy: bool,

  pub etag: Option<String>,
  pub last_modified: Option<String>,
}

impl Channel {
  pub fn new(rsc: rss::Channel) -> Self {
    Self {
      title: rsc.title,
      link: rsc.link,
      description: rsc.description,
      // image: rsc.image,
      pub_date: rsc.pub_date,
      last_build_date: rsc.last_build_date,
      ..Default::default()
    }
  }

  pub fn save() -> Result<> {
    let sql = 
    "INSERT OR REPLACE INTO channel (
      title,
      link,
      description,
      pub_date,
      last_build_date,
      is_proxy,
      etag,
      last_modified
    ) VALUES ( ?1, ?2, ?3, ?4, ?4, ?6, ?7, ?8)
    ";
    
  }

  pub fn creat_table(conn: &Connection) -> Result<usize, rusqlite::Error>{
    let sql = 
      "CREATE TABLE IF NOT EXISTS channel (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT,
        link TEXT UNIQUE NOT NULL,
        description TEXT,
        pub_date TEXT,
        last_build_date TEXT,
        is_proxy INTEGER DEFAULT 0,
        etag TEXT,
        last_modified TEXT,
      )";
    conn.execute(&sql, ())
  }
}

// mock
impl Channel {
  pub fn list() -> Option<Vec<Self>> {
    // mock
    let data = Some(vec![
      Channel {
        title: "channel1".to_string(),
        // link: "https://rsshub.app/rustcc/news".to_string(),
        link: "http://baidu.com".to_string(),
        description: "description1".to_string(),
        // items: [],
        is_proxy: false,
        ..Default::default()
      },
      Channel {
        title: "channel2".to_string(),
        link: "https://rsshub.app/rustcc/jobs".to_string(),
        description: "description2".to_string(),
        // items: [],
        is_proxy: false,
        ..Default::default()
      },
      Channel {
        title: "channel3".to_string(),
        link: "https://rsshub.app/sec-wiki/weekly".to_string(),
        description: "description3".to_string(),
        is_proxy: false,
        ..Default::default()
      },
      Channel {
        title: "channel4".to_string(),
        link: "https://rsshub.app/segmentfault/blogs/go".to_string(),
        description: "description4".to_string(),
        is_proxy: false,
        ..Default::default()
      },
    ]);
    data
  }

}
