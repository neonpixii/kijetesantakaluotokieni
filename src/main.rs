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
}
fn main() {
    let cli = Args::parse();
    let mut text = String::new();
    let default_config = critters::config_from_string("", "", "");

    // io::stdin()
    //     .read_to_string(&mut text)
    //     .expect("failed to read input");

    print!(
        "{}",
        bubbles::bubble_from_text(
            &text,
            critters::KIJETESANTAKALU.anchor,
            DEFAULT_MAXIMUM_LINE_LENGTH
        )
    );
    println!(
        "{}",
        critters::format_critter(critters::KIJETESANTAKALU_LITTLE.critter, default_config)
    );
}

// functions for producing formatted text

const DEFAULT_MAXIMUM_LINE_LENGTH: usize = 40;
