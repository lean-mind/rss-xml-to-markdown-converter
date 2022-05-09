use serde::{Deserialize, Serialize};
use crate::RSSFeed;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub author_name: String,
    pub author_email: String,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Episode {
    pub title: String,
    pub link: String,
    pub date: String,
    pub description: String,
    pub link_mp3: String,
}

impl Podcast {
    pub fn from_rss_feed(rss_feed: RSSFeed) -> Self {
        let channel = rss_feed.channel;
        Self {
            title: channel.title,
            description: channel.description,
            author_name: channel.owner.name,
            author_email: channel.owner.email,
            episodes: channel
                .items
                .into_iter()
                .map(|item| Episode {
                    title: item.title,
                    link: item.link,
                    date: item.date,
                    description: item.description,
                    link_mp3: item.enclosure.link_mp3,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EpisodeWithPodcastData {
    pub podcast_title: String,
    pub podcast_description: String,
    pub author_name: String,
    pub author_email: String,
    pub title: String,
    pub link: String,
    pub date: String,
    pub description: String,
    pub link_mp3: String,
}

impl EpisodeWithPodcastData {
    pub fn from_podcast(podcast: &Podcast, episode_index: usize) -> Self {
        let episode = podcast.episodes[episode_index].clone();
        Self {
            podcast_title: podcast.title.clone(),
            podcast_description: podcast.description.clone(),
            author_name: podcast.author_name.clone(),
            author_email: podcast.author_email.clone(),
            title: episode.title,
            link: episode.link,
            date: episode.date,
            description: episode.description,
            link_mp3: episode.link_mp3,
        }
    }
}
