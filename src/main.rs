/*
TO DO
- options
- kijefiles
- other aminals
*/

mod bubbles;
mod critters;

use std::io;
use std::io::Read;

fn main() {
    let mut text = String::new();
    let default_config = critters::config_from_string("", "", "");

    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

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
