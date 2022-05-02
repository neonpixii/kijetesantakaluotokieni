/*
TO DO
- kijefiles
- other aminals
- pakala
- pilin
*/

mod bubbles;
mod critters;
mod kule;

use clap::Parser;
use kule::Formats;
use kule::FourBit;
use std::io;
use std::io::Read;
use voca_rs::*;
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
    output(&text, config);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'e', long)]
    lukin: Option<String>,

    #[clap(short = 'T', long)]
    uta: Option<String>,

    #[clap(short = 'o', long)]
    ijo: Option<String>,

    #[clap(long)]
    palisa: Option<String>,

    #[clap(short = 'f', long)]
    nimi: Option<String>,

    #[clap(short = 'W', long)]
    pakala: Option<String>,

    #[clap(short = 'k', long)]
    kule: Vec<String>,

    // implementation of classic cowsay flags
    #[clap(short = 'b', long)]
    ilo: bool,

    #[clap(short = 'd', long)]
    moli: bool,

    #[clap(short = 'g', long)]
    wile_mani: bool,

    #[clap(short = 'p', long)]
    monsuta: bool,

    #[clap(short = 's', long)]
    kasi_nasa: bool,

    #[clap(short = 't', long)]
    lape: bool,

    #[clap(short = 'w', long)]
    wawa: bool,

    #[clap(short = 'y', long)]
    lili: bool,

    #[clap(long)]
    pilin: bool,

    // optional text input
    text: Vec<String>,
}

impl Args {
    fn config_from_arguments(&self) -> critters::CritterConfig {
        let mut eyes = self.lukin.clone();
        let mut tongue = self.uta.clone();
        let mut line = self.palisa.clone();
        let mut object = self.ijo.clone();
        let mut format = "".to_string();
        let mut name = self.nimi.clone();

        if self.ilo {
            eyes = Some("==".to_string());
        } else if self.moli {
            eyes = Some("xx".to_string());
            tongue = Some("U".to_string());
        } else if self.wile_mani {
            eyes = Some("$$".to_string());
        } else if self.monsuta {
            eyes = Some("@@".to_string());
        } else if self.kasi_nasa {
            eyes = Some("**".to_string());
            tongue = Some("U".to_string());
        } else if self.lape {
            eyes = Some("--".to_string());
        } else if self.wawa {
            eyes = Some("OO".to_string());
        } else if self.lili {
            eyes = Some("..".to_string());
        }
        for i in &self.kule {
            let lower = query::is_lowercase(&i);
            let lower_i = case::lower_case(&i);
            format.push_str(&match lower_i.as_str() {
                "walo" => {
                    if lower {
                        FourBit::White.escape(false)
                    } else {
                        FourBit::BrightWhite.escape(false)
                    }
                }
                "pimeja" => {
                    if lower {
                        FourBit::Black.escape(false)
                    } else {
                        FourBit::BrightBlack.escape(false)
                    }
                }
                "laso" => {
                    if lower {
                        FourBit::Cyan.escape(false)
                    } else {
                        FourBit::BrightCyan.escape(false)
                    }
                }
                "jelo" => {
                    if lower {
                        FourBit::Yellow.escape(false)
                    } else {
                        FourBit::BrightYellow.escape(false)
                    }
                }
                "loje" => {
                    if lower {
                        FourBit::Red.escape(false)
                    } else {
                        FourBit::BrightRed.escape(false)
                    }
                }
                "kasi" => {
                    if lower {
                        FourBit::Green.escape(false)
                    } else {
                        FourBit::BrightGreen.escape(false)
                    }
                }
                "sewi" => {
                    if lower {
                        FourBit::Blue.escape(false)
                    } else {
                        FourBit::BrightBlue.escape(false)
                    }
                }
                "unu" => {
                    if lower {
                        FourBit::Magenta.escape(false)
                    } else {
                        FourBit::BrightMagenta.escape(false)
                    }
                }
                "suli" => Formats::Bright.escape(false),
                "len" => Formats::Dim.escape(false),
                "mamamije" => Formats::Italic.escape(false),
                "sike" => Formats::Blink.escape(false),
                _ => String::new(),
            })
        }
        critters::CritterConfig::config_from_string(
            &eyes,
            &tongue,
            &line,
            &object,
            &Some(format),
            &name,
        )
    }
}

fn output(text: &str, config: critters::CritterConfig) -> () {
    print!(
        "{}",
        bubbles::bubble_from_text(text, config.template.anchor, DEFAULT_MAXIMUM_LINE_LENGTH)
    );
    println!("{}", config.format_critter())
}

const DEFAULT_MAXIMUM_LINE_LENGTH: usize = 40;
