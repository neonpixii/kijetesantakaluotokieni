use voca_rs::*;

pub struct Critter<'a> {
    pub anchor: usize,
    pub critter: &'a str,
}

pub struct CritterConfig {
    pub left_eye: String,
    pub right_eye: String,

    pub left_tongue: String,
    pub right_tongue: String,

    pub right_line: String,
    pub up_line: String,
    pub left_line: String,
}

pub const KIJETESANTAKALU: Critter = Critter {
    anchor: 14,
    critter: r"
             $6
      /__    $6
     / $1$2\  $7
     |  |$3$4 
     |  |
 (III|\||",
};

pub const KIJETESANTAKALU_LITTLE: Critter = Critter {
    anchor: 13,
    critter: r"
            $6
     /__    $6
    / $1$2\  $5
    |  |$3$4
  (I|\||",
};

pub fn config_from_string(eyes: &str, tongue: &str, line: &str) -> CritterConfig {
    let default_config = CritterConfig {
        left_eye: String::from("."),
        right_eye: String::from("."),

        left_tongue: String::from(" "),
        right_tongue: String::from(" "),

        right_line: String::from("/"),
        up_line: String::from("|"),
        left_line: String::from("\\"),
    };

    let (left_eye, right_eye);
    let (left_tongue, right_tongue);
    let (left_line, up_line, right_line);

    match count::count_graphemes(eyes) {
        0 => (left_eye, right_eye) = (default_config.left_eye, default_config.right_eye),
        1 => (left_eye, right_eye) = (chop::grapheme_at(eyes, 0), chop::grapheme_at(eyes, 0)),
        _ => (left_eye, right_eye) = (chop::grapheme_at(eyes, 0), chop::grapheme_at(eyes, 1)),
    }
    match count::count_graphemes(tongue) {
        0 => {
            (left_tongue, right_tongue) = (default_config.left_tongue, default_config.right_tongue)
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
                default_config.left_line,
                default_config.up_line,
                default_config.right_line,
            )
        }
        1 => {
            (left_line, up_line, right_line) = (
                chop::grapheme_at(line, 0),
                default_config.up_line,
                default_config.right_line,
            )
        }
        2 => {
            (left_line, up_line, right_line) = (
                chop::grapheme_at(line, 0),
                chop::grapheme_at(line, 1),
                default_config.right_line,
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
    CritterConfig {
        left_eye,
        right_eye,

        left_tongue,
        right_tongue,

        right_line,
        up_line,
        left_line,
    }
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
