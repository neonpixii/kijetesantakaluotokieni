/*
TO DO
- options
- kijefiles
- other aminals
*/

mod bubbles;
mod critters;

use clap::Parser;
use std::io;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    lukin: Option<String>,

    #[clap(short, long)]
    uta: Option<String>,

    #[clap(short, long)]
    palisa: Option<String>,

    #[clap(short, long)]
    nimi: Option<String>,

    text: Vec<String>,
}

impl Args {
    fn config_from_arguments(&self) -> critters::CritterConfig {
        critters::CritterConfig::config_from_string(
            self.lukin.clone(),
            self.uta.clone(),
            self.palisa.clone(),
            self.nimi.clone(),
        )
    }
}

fn main() {
    let cli = Args::parse();
    let mut text = String::new();
    let config = cli.config_from_arguments();

    if !cli.text.is_empty() {
        text = cli.text.join(" ")
    } else {
        io::stdin()
            .read_to_string(&mut text)
            .expect("failed to read input");
    }
    output(&text, config)
}

fn output(text: &str, config: critters::CritterConfig) -> () {
    print!(
        "{}",
        bubbles::bubble_from_text(text, config.template.anchor, DEFAULT_MAXIMUM_LINE_LENGTH)
    );
    println!("{}", config.format_critter())
}

const DEFAULT_MAXIMUM_LINE_LENGTH: usize = 40;
