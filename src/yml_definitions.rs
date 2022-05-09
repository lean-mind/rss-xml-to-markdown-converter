use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub name: String,
    pub email: String,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub title: String,
    pub link: String,
    pub date: String,
    pub description: String,
    pub link_mp3: String,
}
