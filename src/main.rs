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
                    title: String::from("Ni cero, ni uno")
                }
            }
        );
    }
}
