use voca_rs::*;

pub enum CritterName {
    Kije,
    Little,
    File(String),
}

#[derive(Clone)]
pub struct CritterTemplate {
    pub anchor: usize,
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

    pub template: CritterTemplate,
}

const KIJETESANTAKALU: CritterTemplate = CritterTemplate {
    anchor: 14,
    critter: r"
             $6
      /__    $6
     / $1$2\  $5
     |  |$3$4 
     |  |
 (III|\||"
        .to_string(),
};

const KIJETESANTAKALU_LITTLE: CritterTemplate = CritterTemplate {
    anchor: 13,
    critter: r"
            $6
     /__    $6
    / $1$2\  $5
    |  |$3$4
  (I|\||"
        .to_string(),
};

const DEFAULT_CONFIG: CritterConfig = CritterConfig {
    left_eye: String::from("."),
    right_eye: String::from("."),

    left_tongue: String::from(" "),
    right_tongue: String::from(" "),

    right_line: String::from("/"),
    up_line: String::from("|"),
    left_line: String::from("\\"),

    template: KIJETESANTAKALU,
};

pub fn config_from_string(
    eyes: Option<&str>,
    tongue: Option<&str>,
    line: Option<&str>,
) -> CritterConfig {
    let mut config = DEFAULT_CONFIG.clone();

    match count::count_graphemes(eyes) {
        0 => break,

        1 => (left_eye, right_eye) = (chop::grapheme_at(eyes, 0), chop::grapheme_at(eyes, 0)),
        _ => (left_eye, right_eye) = (chop::grapheme_at(eyes, 0), chop::grapheme_at(eyes, 1)),
    }
    match count::count_graphemes(tongue) {
        0 => {
            (left_tongue, right_tongue) = (DEFAULT_CONFIG.left_tongue, DEFAULT_CONFIG.right_tongue)
        }
        1 => (left_tongue, right_tongue) = (chop::grapheme_at(tongue, 0), " ".to_string()),
        _ => {
            (left_tongue, right_tongue) =
                (chop::grapheme_at(tongue, 0), chop::grapheme_at(tongue, 1))
        }
    }
    match count::count_graphemes(line) {
        0 => {
            (left_line, up_line, right_line) = (
                DEFAULT_CONFIG.left_line,
                DEFAULT_CONFIG.up_line,
                DEFAULT_CONFIG.right_line,
            )
        }
        1 => {
            (left_line, up_line, right_line) = (
                chop::grapheme_at(line, 0),
                DEFAULT_CONFIG.up_line,
                DEFAULT_CONFIG.right_line,
            )
        }
        2 => {
            (left_line, up_line, right_line) = (
                chop::grapheme_at(line, 0),
                chop::grapheme_at(line, 1),
                DEFAULT_CONFIG.right_line,
            )
        }
        _ => {
            (left_line, up_line, right_line) = (
                chop::grapheme_at(line, 0),
                chop::grapheme_at(line, 1),
                chop::grapheme_at(line, 2),
            )
        }
    }

    return config;
}

pub fn format_critter(critter: &str, critter_config: CritterConfig) -> String {
    return critter
        .replace("$1", &critter_config.left_eye)
        .replace("$2", &critter_config.right_eye)
        .replace("$3", &critter_config.left_tongue)
        .replace("$4", &critter_config.right_tongue)
        .replace("$5", &critter_config.right_line)
        .replace("$6", &critter_config.up_line)
        .replace("$7", &critter_config.left_line);
}
