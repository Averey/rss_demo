use std::error::Error;

mod models;
use models::Channel;


mod db;
use db::Table;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::open()?;
    Channel::create_table(&conn)?;

    let rss_list = [
        "https://rsshub.app/rustcc/jobs",
        "https://rsshub.app/sec-wiki/weekly",
        "https://rsshub.app/segmentfault/blogs/go"
    ];

    rss_list.iter().for_each(|r| {
        let res = async {
            let channel = Channel::subscribe(r, None, false).await?;
            eprintln!("{:#?}", channel);
            Ok::<Channel, Box<dyn std::error::Error>>(channel)
        };
        ()
    });

    //let channels = Channel::list(&conn, 0, 10)?;
    //eprintln!("{:#?}", channels);

    Ok(())
}
