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
                    }
                }
            }
        );
    }
}
