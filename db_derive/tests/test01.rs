use db_derive::Table;

pub struct Image {}

/// let channel_table =Channel::table();
///
#[derive(Table)]
pub struct Channel {
    #[col(name = "title", type = "TEXT")]
    pub title: String,
    #[col(type = "TEXT")]
    pub link: String,
    #[col(type = "TEXT")]
    pub description: String,
    //pub image: Option<Image>,
    //pub items: Vec<ChannelItem>,
    //pub pub_date: Option<String>,
    //pub last_build_date: Option<String>,
    //
    //pub is_proxy: bool,
    //
    //pub etag: Option<String>,
    //pub last_modified: Option<String>,
}

#[derive(Table)]
pub struct ChannelItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub content: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
}

#[test]
fn test() {}
