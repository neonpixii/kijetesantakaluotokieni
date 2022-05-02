use super::kule::*;
use voca_rs::*;

pub enum CritterName {
    Kije,
    Little,
    File(String),
}

#[derive(Clone)]
pub struct CritterTemplate {
    // text column where speech line joins speech bubble
    pub anchor: usize,
    /* ascii art critter themself, with special optional formatting strings. all formatters render to one grapheme wide each unless otherwise stated
        - $1$2 (left and right eyes)
        - $3$4 (left and right tongue)
        - $5$6$7 (forward leaning, upwards, and back leaning speech line)
        - $8$9 (ansi escape formatting start and formatting stop markers; each renders to zero width)
        - $0 object marker. defaults to a single space but can be as many graphemes as you want
    */
    pub critter: String,
}
#[derive(Clone)]
pub struct CritterConfig {
    pub left_eye: String,
    pub right_eye: String,

    pub left_tongue: String,
    pub right_tongue: String,

    pub right_line: String,
    pub up_line: String,
    pub left_line: String,

    pub object: String,

    pub format: String,

    pub template: CritterTemplate,
}

impl CritterConfig {
    // tries to create a critter from relevant strings, falling back to sensible defaults where possible
    pub fn config_from_string(
        eyes: &Option<String>,
        tongue: &Option<String>,
        line: &Option<String>,
        object: &Option<String>,
        format: &Option<String>,
        name: &Option<String>,
    ) -> CritterConfig {
        let kijetesantakalu = CritterTemplate {
            anchor: 14,
            critter: r"
$8             $9$6
$8      /__    $9$6
$8     / $1$2\  $9$5
$8     |  |$3$4
$8     |  |
$8 (III|\||  $9$0"
                .to_string(),
        };
        let kijetesantakalu_little = CritterTemplate {
            anchor: 13,
            critter: r"
$8            $9$6
$8     /__    $9$6
$8    / $1$2\  $9$5
$8    |  |$3$4
$8  (I|\||  $9$0"
                .to_string(),
        };
        let soweli = CritterTemplate {
            anchor: 10,
            critter: r"
$8         $9$6
$8   ___   $9$6
$8    $1$2) $9$5
$8  |||| $9$0"
                .to_string(),
        };

        let default_config: CritterConfig = CritterConfig {
            left_eye: String::from("o"),
            right_eye: String::from("o"),

            left_tongue: String::from(" "),
            right_tongue: String::from(" "),

            right_line: String::from("/"),
            up_line: String::from("|"),
            left_line: String::from("\\"),

            object: String::from(" "),

            format: reset(), // from kule

            template: kijetesantakalu,
        };

        let mut config = default_config.clone();
        if let Some(eyes) = eyes {
            match count::count_graphemes(&eyes) {
                0 => (),
                1 => {
                    (config.left_eye, config.right_eye) =
                        (chop::grapheme_at(&eyes, 0), chop::grapheme_at(&eyes, 0))
                }
                _ => {
                    (config.left_eye, config.right_eye) =
                        (chop::grapheme_at(&eyes, 0), chop::grapheme_at(&eyes, 1))
                }
            }
        }
        if let Some(tongue) = tongue {
            match count::count_graphemes(&tongue) {
                0 => (),
                1 => {
                    (config.left_tongue, config.right_tongue) =
                        (chop::grapheme_at(&tongue, 0), " ".to_string())
                }
                _ => {
                    (config.left_tongue, config.right_tongue) =
                        (chop::grapheme_at(&tongue, 0), chop::grapheme_at(&tongue, 1))
                }
            }
        }
        if let Some(line) = line {
            match count::count_graphemes(&line) {
                0 => (),
                1 => {
                    (config.right_line, config.up_line, config.left_line) = (
                        chop::grapheme_at(&line, 0),
                        chop::grapheme_at(&line, 0),
                        chop::grapheme_at(&line, 0),
                    )
                }
                2 => {
                    (config.right_line, config.up_line) =
                        (chop::grapheme_at(&line, 0), chop::grapheme_at(&line, 1))
                }
                _ => {
                    (config.right_line, config.up_line, config.left_line) = (
                        chop::grapheme_at(&line, 0),
                        chop::grapheme_at(&line, 1),
                        chop::grapheme_at(&line, 2),
                    )
                }
            }
        }
        if let Some(object) = object {
            match count::count_graphemes(&object) {
                0 => (),
                _ => config.object = object.clone(),
            }
        }
        if let Some(format) = format {
            config.format = format.to_string();
        }
        if let Some(name) = name {
            match name.as_str() {
                "kijetesantakalu" => (),
                "lili" => config.template = kijetesantakalu_little,
                "soweli" => config.template = soweli,
                _ => (),
            }
        }

        return config;
    }

    // gives a fully formatted version of the critter
    pub fn format_critter(&self) -> String {
        return self
            .template
            .critter
            .replace("$1", &self.left_eye)
            .replace("$2", &self.right_eye)
            .replace("$3", &self.left_tongue)
            .replace("$4", &self.right_tongue)
            .replace("$5", &self.right_line)
            .replace("$6", &self.up_line)
            .replace("$7", &self.left_line)
            .replace("$8", &self.format)
            .replace("$9", &reset())
            .replace("$0", &self.object);
    }
}
