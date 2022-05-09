use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RSSFeed {
    channel: Channel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Channel {
    title: String,
    description: String,
    owner: Owner,
    #[serde(rename="item")]
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    title: String,
    link: String,
    #[serde(rename="pubDate")]
    date: String,
    #[serde(rename="summary")]
    description: String,
    enclosure: Enclosure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Enclosure {
    #[serde(rename="url")]
    link_mp3: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Owner {
    name: String,
    email: String,
}

fn read_xml(filename: &str) -> RSSFeed {
    from_str(&read_to_string(filename).expect("Can't read the file"))
        .expect("File with wrong format")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_xml() {
        assert_eq!(
            read_xml("example.xml"),
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
}
