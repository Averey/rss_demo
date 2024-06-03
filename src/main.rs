use std::error::Error;

mod net;
mod models;
use models::Channel;

mod db;
use db::Table;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = db::open()?;
    Channel::create_table(&conn)?;

    //let rss_list = [
    //    "https://rsshub.app/segmentfault/blogs/go",
    //    "https://rsshub.app/rustcc/jobs",
    //    "https://rsshub.app/sec-wiki/weekly",
    //];
    //let rss_list = [
    //    "https://www.dmhy.org/topics/rss/rss.xml?keyword=%E8%91%AC%E9%80%81%E7%9A%84%E8%8A%99%E8%8E%89%E8%8E%B2%7C%E8%91%AC%E9%80%81%E3%81%AE%E3%83%95%E3%83%AA%E3%83%BC%E3%83%AC%E3%83%B3",
    //    "https://www.dmhy.org/topics/rss/rss.xml?keyword=%E4%B8%8D%E6%AD%BB%E4%B8%8D%E5%B9%B8%7C%E3%82%A2%E3%83%B3%E3%83%87%E3%83%83%E3%83%89%E3%82%A2%E3%83%B3%E3%83%A9%E3%83%83%E3%82%AF",
    //    "https://www.dmhy.org/topics/rss/rss.xml?keyword=%E6%B5%B7%E8%B3%8A%E7%8E%8B",
    //    "https://www.zhihu.com/rss"
    //];

    //let task_handle = rss_list.iter().map(|&r| {
    //    tokio::spawn(async move {
    //        Channel::subscribe(r, None, true).await.unwrap()
    //    })
    ////}).collect::<Vec<JoinHandle<Channel>>>();
    //});


    //for (i, c) in task_handle.enumerate() {
    ////for handle in task_handle {
    //    if let Ok(c) = c.await {
    //        println!("{}--{:#?}",i, c);
    //    } else {
    //        println!("task panic");
    //    }
    //}

    Ok(())
}
