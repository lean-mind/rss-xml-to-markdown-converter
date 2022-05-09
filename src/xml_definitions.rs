use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RSSFeed {
    pub channel: Channel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub owner: Owner,
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub title: String,
    pub link: String,
    #[serde(rename = "pubDate")]
    pub date: String,
    #[serde(rename = "summary")]
    pub description: String,
    pub enclosure: Enclosure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Enclosure {
    #[serde(rename = "url")]
    pub link_mp3: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Owner {
    pub name: String,
    pub email: String,
}
