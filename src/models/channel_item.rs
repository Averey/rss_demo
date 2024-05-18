#[derive(Debug, Default)]
pub struct ChannelItem {
  pub title: String,
  pub link: String,
  pub description: String,
  pub content: Option<String>,
  pub author: Option<String>,
  pub pub_date: Option<String>,
  pub last_build_date: Option<String>,
}

impl ChannelItem {

}