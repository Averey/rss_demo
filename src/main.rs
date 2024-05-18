mod models;
use models::Channel;

use std::error::Error;

mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    db::init()?;

    let proxy_url = "http://127.0.0.1:9001";
    let http_proxy = reqwest::Proxy::http(proxy_url)?;
    let https_proxy = reqwest::Proxy::https(proxy_url)?;


    let client = reqwest::Client::builder()
        .proxy(http_proxy)
        .proxy(https_proxy)
        .build()?;

    for c in Channel::list().unwrap_or_default() {
        let res = client
            .get(&c.link)
            .send()
            .await?;
        let headers =  res.headers().clone();
        let content = res
            .bytes()
            .await?;
        match rss::Channel::read_from(&content[..]) {
            Ok(rsc) => {
                let channel = Channel {
                    title: rsc.title,
                    link: rsc.link,
                    description: rsc.description,
                    // image: rsc.image,
                    pub_date: rsc.pub_date,
                    last_build_date: rsc.last_build_date,
                    etag: headers.get("etag")
                        .and_then(|s| s.to_str().ok())
                        .map(|s| s.to_string()),
                    last_modified: headers.get("last-modified")
                        .and_then(|s| s.to_str().ok())
                        .map(|s| s.to_string()),
                    ..Default::default()
                };
                println!("{:#?}\n{:#?}\n\n", channel, headers);
            },
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }

    Ok(())
}
