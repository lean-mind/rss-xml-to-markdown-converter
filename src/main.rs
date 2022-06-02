mod md_to_xml;
mod xml_definitions;
mod xml_to_md;
mod yml_definitions;

use clap::Parser;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use xml_definitions::*;
use xml_to_md::{generate_episode_files, generate_podcast_file, read_rss_feed};
use yml_definitions::*;
use md_to_xml::{read_md, markdown_to_xml};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of a file from which the RSS feed will be read
    input_filename: String,
    /// Where to save the output. By default a file will be created, but
    /// if separate_by_episodes is set, a directory will be created with
    /// one file per episode
    output_path: String,
    /// If this flag is set, input_filename must be a markdown file to
    /// generate the RSS feed in the output_path
    #[clap(short, long, group="exclusive")]
    reverse: bool,
    /// If this flag is set, a file will be created for each episode
    #[clap(short = 'e', long, group="exclusive")]
    separate_by_episodes: bool,
}

fn main() {
    let args = Args::parse();
    match args.reverse {
        false => {
            let podcast = Podcast::from_rss_feed(read_rss_feed(&args.input_filename));
            let output_path = Path::new(&args.output_path);
            
            match args.separate_by_episodes {
                true => generate_episode_files(&podcast, output_path),
                false => generate_podcast_file(&podcast, output_path),
            }
        }
        true => {
            let podcast = read_md(&args.input_filename);
            let output_path = Path::new(&args.output_path);
            let xml = markdown_to_xml(podcast);

            let mut file = File::create(output_path).expect("Error creating file");
            write!(file, "{}", xml).expect("Error writing to file");
        }
    }
}
