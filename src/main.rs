mod md_to_xml;
mod xml_definitions;
mod xml_to_md;
mod yml_definitions;

use clap::Parser;
use std::path::Path;
use xml_definitions::*;
use xml_to_md::{generate_episode_files, generate_podcast_file, read_rss_feed};
use yml_definitions::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of a file from which the RSS feed will be read
    input_filename: String,
    /// Where to save the output. By default a file will be created, but
    /// if separate_by_episodes is set, a directory will be created with
    /// one file per episode
    output_path: String,
    /// If this flag is set, a file will be created for each episode
    #[clap(short = 'e', long)]
    separate_by_episodes: bool,
}

fn main() {
    let args = Args::parse();
    let podcast = Podcast::from_rss_feed(read_rss_feed(&args.input_filename));
    let output_path = Path::new(&args.output_path);

    match args.separate_by_episodes {
        true => generate_episode_files(&podcast, output_path),
        false => generate_podcast_file(&podcast, output_path),
    }
}
