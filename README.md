
# RSS-XML to Markdown Converter

This repository contains a tool which aims to make generating static sites (with SSGs such as Hugo) from podcast RSS Feeds easier.

The code strives to be self-explanatory and easy to adapt. To change the XML output you should check [this template](./xml_template.handlebars).

## Table Of Contents <!-- omit in toc -->
- [RSS-XML to Markdown Converter](#rss-xml-to-markdown-converter)
  - [Usage](#usage)
    - [Arguments](#arguments)
    - [Options](#options)
  - [Examples](#examples)


## Usage
```bash
rss-xml-to-markdown-converter [OPTIONS] <INPUT_FILENAME> <OUTPUT_PATH>
```

### Arguments
```
  <INPUT_FILENAME>    The name of a file from which the RSS feed will be read
  <OUTPUT_PATH>       Where to save the output. By default a file will be created, but if
                      separate_by_episodes is set, a directory will be created with one file
                      per episode
```

### Options

```
  -e, --separate-by-episodes    If this flag is set, a file will be created for each episode
  -h, --help                    Print help information
  -r, --reverse                 If this flag is set, input_filename must be a markdown file to
                                generate the RSS feed in the output_path
  -V, --version                 Print version information
```

## Examples

Generate a single markdown from the RSS Feed:

```
cargo run -- examples/input_example.xml examples/podcast_example.md
```

Generate an individual markdown for each episode from the RSS Feed:

```
cargo run -- examples/input_example.xml examples/output --separate-by-episodes
```

Generate an RSS Feed from the podcast markdown:

```
cargo run -- examples/podcast_example.md examples/output_example.xml --reverse
```
