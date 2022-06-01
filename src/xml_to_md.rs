use crate::xml_definitions::*;
use crate::yml_definitions::*;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;

pub fn read_rss_feed(filename: &str) -> RSSFeed {
    serde_xml_rs::from_str(&read_to_string(filename).expect("Can't read the RSS file"))
        .expect("File with wrong format")
}

fn podcast_to_hugo_markdown(podcast: &Podcast) -> String {
    format!(
        "{}---\n",
        serde_yaml::to_string(&podcast).expect("Error converting to yaml")
    )
}

fn episode_to_hugo_markdown(podcast: &Podcast, episode_index: usize) -> String {
    let episode = EpisodeWithPodcastData::from_podcast(podcast, episode_index);
    format!(
        "{}---\n",
        serde_yaml::to_string(&episode).expect("Error converting to yaml")
    )
}

pub fn generate_podcast_file(podcast: &Podcast, output_path: &Path) {
    let markdown = podcast_to_hugo_markdown(podcast);
    let mut file = File::create(output_path).expect("Error creating file");
    write!(file, "{}", markdown).expect("Error writing to file");
}

pub fn generate_episode_files(podcast: &Podcast, output_path: &Path) {
    create_dir_all(output_path).expect("Error creating directory");
    for episode_index in 0..podcast.episodes.len() {
        let markdown = episode_to_hugo_markdown(podcast, episode_index);
        let episode_title = &podcast.episodes[episode_index].title;
        let filename = episode_title.replace(" ", "_") + ".md";
        let mut file = File::create(output_path.join(filename)).expect("Error creating file");
        write!(file, "{}", markdown).expect("Error writing to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_rss_feed() {
        assert_eq!(
            read_rss_feed("examples/example_input.xml"),
            RSSFeed {
                channel: Channel {
                    title: String::from("Ni cero, ni uno"),
                    links: vec![Link {
                        feed_link: Some(String::from("https://podcast.carlosble.com/feed/podcast")),
                        page_link: None,
                    }, Link{
                        feed_link: None,
                        page_link: Some(String::from("https://podcast.carlosble.com/")),
                    }],
                    description: String::from("Un punto de vista diferente, peculiar y atrevido sobre la industria del desarrollo de software y sobre las habilidades que más se necesitan en este mundo tecnológico que en verdad gira en torno a las personas.\n\nCarlos Blé, fundador de varias empresas y actual director de Lean Mind, narra sus experiencias y cuenta con colaboraciones de profesionales de diversos ámbitos."),
                    subtitle: String::from("Habilidades esenciales en un mundo tecnológico"),
                    owner: Owner {
                        name: String::from("Carlos Blé"),
                        email: String::from("carlos@carlosble.com")
                    },
                    images: vec![Image {
                        link: Some(String::from("https://podcast.carlosble.com/wp-content/uploads/2021/10/nicero_niuno.jpg")),
                    }, Image { link: None}
                    ],
                    items: vec![
                        Item {
                            title: String::from("E32: ¿Para qué impartir una ponencia?"),
                            link: String::from("https://podcast.carlosble.com/podcast/e32-para-que-impartir-una-ponencia/"),
                            date: String::from("Thu, 14 Apr 2022 15:58:24 +0000"),
                            description: String::from("¿Para qué dar una charla?, ¿por qué exponerse?, ¿qué puedes aportar cuando parece que ya está todo en internet?. En este episodio converso con Adrián Ferrera, Francisco Mesa y Miguel Cabrera. Además, contamos con mensajes grabados de ponentes que me encantan, Yodra López, Jorge J. Barroso, Aida Albarrán y Alexandra Rivero. Contribuciones: Yodra López (1:07:09)..."),
                            enclosure: Enclosure {
                                link_mp3: String::from("https://podcast.carlosble.com/podcast-download/1220/e32-para-que-impartir-una-ponencia.mp3"),
                            }
                        },
                        Item {
                            title: String::from("E31: El sendero de la maestría (III)"),
                            link: String::from("https://podcast.carlosble.com/podcast/e31-el-sendero-de-la-maestria-iii/"),
                            date: String::from("Fri, 18 Mar 2022 09:43:27 +0000"),
                            description: String::from("Terminamos la serie de epidosodios sobre aprendizaje (por ahora), con el gran Jose Enrique Rodríguez Huerta, actualmente director de Codurance España. En esta conversación José nos habla de cultura empresarial, liderazgo, carrera profesional, trabajo en equipo y por supuesto, aprendizaje. Recursos citados en el episodio (y otros que olvidé citar): Libro: Tribal Leadership Trabajar en..."),
                            enclosure: Enclosure {
                                link_mp3: String::from("https://podcast.carlosble.com/podcast-download/1216/e31-el-sendero-de-la-maestria-iii.mp3"),
                            }
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn can_generate_markdown() {
        let expected = read_to_string("examples/example_output.md").expect("Can't read the file");
        let podcast = Podcast::from_rss_feed(read_rss_feed("examples/example_input.xml"));
        assert_eq!(podcast_to_hugo_markdown(&podcast), expected)
    }

    #[test]
    fn can_generate_episode_markdown() {
        let expected =
            read_to_string("examples/example_episode_output.md").expect("Can't read the file");
        let podcast = Podcast::from_rss_feed(read_rss_feed("examples/example_input.xml"));
        assert_eq!(episode_to_hugo_markdown(&podcast, 0), expected)
    }
}
