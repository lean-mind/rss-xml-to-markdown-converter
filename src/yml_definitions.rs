use crate::RSSFeed;
use serde::{Deserialize, Serialize};

/// Podcast is a curated representation of the data present in the XML
/// It is the structure that the markdown will use 
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Podcast {
    pub title: String,
    pub feed_link: String,
    pub page_link: String,
    pub description: String,
    pub subtitle: String,
    pub author_name: String,
    pub author_email: String,
    pub image_link: String,
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
        let links = channel.links;
        let feed_link = links[0]
            .feed_link
            .as_ref()
            .or(links[1].feed_link.as_ref())
            .unwrap()
            .clone();
        let page_link = links[1]
            .page_link
            .as_ref()
            .or(links[0].page_link.as_ref())
            .unwrap()
            .clone();
        let image_link = channel.images[0]
            .link
            .as_ref()
            .or(channel.images[1].link.as_ref())
            .unwrap()
            .clone();
        Self {
            title: channel.title,
            feed_link,
            page_link,
            description: channel.description,
            subtitle: channel.subtitle,
            author_name: channel.owner.name,
            author_email: channel.owner.email,
            image_link,
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
    pub podcast_subtitle: String,
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
            podcast_subtitle: podcast.subtitle.clone(),
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
