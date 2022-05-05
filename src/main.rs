/*
TO DO
- kijefiles
- kijepath
- other aminals
*/

mod bubbles;
mod critters;
mod kule;

use bubbles::*;
use clap::Parser;
use critters::*;
use kule::Formats;
use kule::FourBit;
use std::io;
use std::io::Read;
use voca_rs::*;

fn main() {
    let cli = Args::parse();
    let mut text = String::new();
    let (critter_config, bubble_config);
    match cli.configs_from_arguments() {
        Err(s) => {
            println!("pakala a!\n{}", s);
            return;
        }
        Ok((c, b)) => {
            (critter_config, bubble_config) = (c, b);
        }
    }
    if !cli.text.is_empty() {
        text = cli.text.join(" ")
    } else {
        io::stdin()
            .read_to_string(&mut text)
            .expect("failed to read input");
    }
    output(&text, critter_config, bubble_config);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'e', long, help = "o ante e sitelen lukin")]
    lukin: Option<String>,

    #[clap(short = 'T', long, help = "o pana e uta tawa kijetesantakalu")]
    uta: Option<String>,

    #[clap(short = 'o', long, help = "o pana e ijo tawa kijetesantakalu")]
    ijo: Option<String>,

    #[clap(short = 'i', long, help = "o ante e palisa pi poki toki")]
    palisa: Option<String>,

    #[clap(short = 'u', long, help = "o ante e poki toki")]
    poki: Option<String>,

    #[clap(short = 'f', long, help = "o kepeken sijelo ante")]
    nimi: Option<String>,

    #[clap(
        short = 'W',
        long,
        default_value = "40",
        help = "o pakala e toki lon sitelen nanpa ni"
    )]
    pakala: usize,

    #[clap(short = 'n', long, help = "o pakala ala e toki")]
    pakala_ala: bool,

    #[clap(short = 'k', long, help = "o ante e kule kijetesantakalu")]
    kule: Vec<String>,

    #[clap(short = 'l', long, help = "FINISH sijelo seme li lon?")]
    seme: bool,

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

    #[clap(short = 'I', long)]
    pilin: bool,

    // optional text input
    text: Vec<String>,
}

impl Args {
    fn configs_from_arguments(&self) -> Result<(CritterConfig, BubbleConfig), String> {
        let mut eyes = self.lukin.clone();
        let mut tongue = self.uta.clone();
        let mut line = self.palisa.clone();
        let object = self.ijo.clone();
        let mut format = "".to_string();
        let name = self.nimi.clone();

        let mut border = self.poki.clone();

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
            line = Some("zZz".to_string());
            border = Some("_()-".to_string());
        } else if self.wawa {
            eyes = Some("OO".to_string());
        } else if self.lili {
            eyes = Some("..".to_string());
        }
        if self.pilin {
            line = Some("oOo".to_string());
            border = Some("_()-".to_string());
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
        let critter_config = CritterConfig::config_from_string(
            &eyes,
            &tongue,
            &line,
            &object,
            &Some(format),
            &name,
        )?;
        let bubble_config = BubbleConfig::config_from_string(
            critter_config.template.anchor,
            self.pakala,
            self.pakala_ala,
            &border,
        );

        Ok((critter_config, bubble_config))
    }
}

fn output(text: &str, critter_config: CritterConfig, bubble_config: BubbleConfig) -> () {
    print!("{}", bubble_config.bubble_from_text(text));
    println!("{}", critter_config.format_critter())
}
