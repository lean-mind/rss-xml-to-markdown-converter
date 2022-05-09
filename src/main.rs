mod xml_definitions;
mod yml_definitions;

use std::fs::{read_to_string, File};
use std::io::Write;
use xml_definitions::*;
use yml_definitions::*;
use clap::Parser;

fn read_rss_feed(filename: &str) -> RSSFeed {
    serde_xml_rs::from_str(&read_to_string(filename).expect("Can't read the RSS file"))
        .expect("File with wrong format")
}

fn rss_feed_to_hugo_markdown(rss_feed: RSSFeed) -> String {
    let channel = rss_feed.channel;
    let podcast = Podcast {
        title: channel.title,
        description: channel.description,
        name: channel.owner.name,
        email: channel.owner.email,
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
    };
    format!("{}---\n", serde_yaml::to_string(&podcast).expect("Error converting to yaml"))
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_filename: String,

    #[clap(short, long)]
    output_filename: String,
}


fn main() {
    let args = Args::parse();
    let input_filename = args.input_filename;
    let output_filename = args.output_filename;

    let markdown = rss_feed_to_hugo_markdown(read_rss_feed(&input_filename));
    let mut file = File::create(output_filename).expect("Error creating file");
    write!(file, "{}", markdown).expect("Error writing to file");
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
                    description: String::from("Un punto de vista diferente, peculiar y atrevido sobre la industria del desarrollo de software y sobre las habilidades que más se necesitan en este mundo tecnológico que en verdad gira en torno a las personas.\n\nCarlos Blé, fundador de varias empresas y actual director de Lean Mind, narra sus experiencias y cuenta con colaboraciones de profesionales de diversos ámbitos."),
                    owner: Owner {
                        name: String::from("Carlos Blé"),
                        email: String::from("carlos@carlosble.com")
                    },
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
        assert_eq!(
            rss_feed_to_hugo_markdown(read_rss_feed("examples/example_input.xml")),
            expected
        )
    }
}
