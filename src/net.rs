use std::{collections::HashMap};
use reqwest::header::HeaderMap;

type Error = Box<dyn std::error::Error>;

fn get_header_value(headers: &HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .and_then(|s| s.to_str().ok())
        .map(|s| s.to_string())
}

pub async fn fetch(url: &str, is_proxy: bool) -> Result<(&[u8], HashMap<String, Option<String>>), Error> {

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

    let res = client.get(url).send().await?;
    let headers = res.headers().clone();
    let mut header_map = HashMap::new();
    header_map.insert("etag".to_string(), get_header_value(&headers, "etag"));
    header_map.insert("last-modified".to_string(), get_header_value(&headers, "last-modified"));

    let content = res.bytes().await?;
    Ok((&content[..], header_map))
}
