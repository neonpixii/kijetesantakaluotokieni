use super::kule::*;
use std::env;
use std::fs;
use std::io;
use voca_rs::*;

// represents inherent structural information about a critter
#[derive(Clone)]
pub struct CritterTemplate {
    // text column where speech line joins speech bubble
    pub anchor: usize,
    pub default_left_eye: String,
    pub default_right_eye: String,
    pub default_left_tongue: String,
    pub default_right_tongue: String,
    /* ascii art critter themself, with special optional formatting strings. all formatters render to one grapheme wide each unless otherwise stated
        - $1$2 (left and right eyes)
        - $3$4 (left and right tongue)
        - $5$6$7 (forward leaning, upwards, and back leaning speech line)
        - $8$9 (ansi escape formatting start and formatting stop markers; each renders to zero width)
        - $0 object marker. defaults to a single space but can be as many graphemes as you want
    */
    pub critter: String,
}

// pairs a critter with formatting information for the optional formatting strings.
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
    ) -> Result<CritterConfig, String> {
        let template: CritterTemplate;
        if let Some(name) = name {
            template = match name.as_str() {
                // when you add a new hardcoded value here, also add it in list_files
                "kijetesantakalu" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"             $6
$8      /__    $9$6
$8     / $1$2\  $9$5
$8     |  |$3$4
$8     |  |
$8 (III|\||  $9$0"
                        .to_string(),
                },
                "kuletesantakalu" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: "             $6
$8\x1b[38;2;255;34;41m      /__    $9$6
$8\x1b[38;2;253;232;26m     / $1$2\\  $9$5
$8\x1b[38;2;0;230;78m     |  |$3$4
$8\x1b[38;2;64;128;255m     |  |
$8\x1b[38;2;200;41;200m (III|\\||  $9$0"
                        .to_string(),
                },
                "kijetonsitakalu" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: "             $6
$8\x1b[38;2;244;244;48m      /__    $9$6
$8\x1b[38;2;232;232;232m     / $1$2\\  $9$5
$8\x1b[38;2;209;89;209m     |  |$3$4
$8\x1b[38;2;78;78;78m   (I|\\||  $9$0"
                        .to_string(),
                },
                "kijetonsitakatu" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: "             $6
$8\x1b[38;2;48;164;250m      /__    $9$6
$8\x1b[38;2;245;169;184m     / $1$2\\  $9$5
$8\x1b[38;2;232;232;232m     |  |$3$4
$8\x1b[38;2;245;169;184m     |  |
$8\x1b[38;2;48;164;250m (III|\\||  $9$0"
                        .to_string(),
                },
                "lili" => CritterTemplate {
                    anchor: 13,
                    default_left_eye: "o".to_string(),
                    default_right_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),

                    critter: r"            $6
$8     /__    $9$6
$8    / $1$2\  $9$5
$8    |  |$3$4
$8  (I|\||  $9$0"
                        .to_string(),
                },
                "soweli" => CritterTemplate {
                    anchor: 10,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"         $6
$8   ___   $9$6
$8    $1$2) $9$5
$8  ||||$3$9$0"
                        .to_string(),
                },
                "soweli-a" => CritterTemplate {
                    anchor: 10,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"        $5
$8   ___ ,
$8    $1$2)-
$8  ||||$3`$9$0"
                        .to_string(),
                },
                "waso" => CritterTemplate {
                    anchor: 9,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"        $6
$8  \     $9$6
$8 $1$2\    $9$6
$8  __\  $9$5
$8 |$3$4
$8 |   $9$0"
                        .to_string(),
                },
                "kala" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"             $6
$8 _   ___     $9$6
$8  -_- $1$2-_  $9$5
$8 _--_ $4$3_-
$8     ---  $9$0"
                        .to_string(),
                },
                "pipi" => CritterTemplate {
                    anchor: 10,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"$8   $4     $9$6
$8  $1$3$2   $9$5
$8  _|_   
$8  _|_
$8  _|_
$8   |   $9$0"
                        .to_string(),
                },
                "akesi" => CritterTemplate {
                    anchor: 11,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: "_".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"    $4     $6
$8   $1$3$2   $9$5
$8  _/_\_   
$8   | |
$8  -|-|-
$8    V   $9$0"
                        .to_string(),
                },
                "soko" => CritterTemplate {
                    anchor: 13,
                    default_right_eye: "_".to_string(),
                    default_left_eye: "_".to_string(),
                    default_left_tongue: "|".to_string(),
                    default_right_tongue: "|".to_string(),
                    critter: r"            $6
$8    _--_    $9$6
$8   (_$1$2_)  $9$5
$8     $3$4
$8     ||
$8     ||  $9$0"
                        .to_string(),
                },
                "kasi" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: "_".to_string(),
                    default_left_eye: "_".to_string(),
                    default_left_tongue: "|".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"             $6
$8        _    $9$6
$8  _    ($2)  $9$5
$8 ($1)$4 /    
$8    \$3
$8     |
$8     |  $9$0"
                        .to_string(),
                },
                "toki-pona" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: " ".to_string(),
                    default_left_eye: " ".to_string(),
                    default_left_tongue: "-".to_string(),
                    default_right_tongue: "´".to_string(),
                    critter: r"             $6
$8   \ | /     $9$6
$8   _---_     $9$6
$8  - $1 $2 -   $9$5
$8  -     -
$8  - `$3$4 - 
$8   `---´   $9$0"
                        .to_string(),
                },
                "mu" => CritterTemplate {
                    anchor: 14,
                    default_right_eye: " ".to_string(),
                    default_left_eye: " ".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: ".".to_string(),
                    critter: r"             $9$6
$8  ()_---_()  $9$6
$8   -     -  $9$5
$8   - $1$4$2 -
$8   -   $3 -
$8    `---´   $9$0"
                        .to_string(),
                },
                "mani" => CritterTemplate {
                    anchor: 13,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: "_".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"            $6
$8  (_---_)   $9$6
$8  -     -  $9$6
$8  - $1$4$2 -
$8  -  $3  -
$8   `---´   $9$0"
                        .to_string(),
                },
                "mani-majuna" => CritterTemplate {
                    anchor: 9,
                    default_right_eye: "o".to_string(),
                    default_left_eye: "o".to_string(),
                    default_left_tongue: " ".to_string(),
                    default_right_tongue: " ".to_string(),
                    critter: r"        $9$7   $8^__^
         $9$7  $8($1$2)\_______
            $8(__)\       )\/\
             $8$3$4 ||----w |
            $9$0$8   ||     ||$9"
                        .to_string(),
                },
                name => CritterConfig::template_from_file(&name)?,
            }
        } else {
            template = CritterTemplate {
                anchor: 14,
                default_right_eye: "o".to_string(),
                default_left_eye: "o".to_string(),
                default_left_tongue: " ".to_string(),
                default_right_tongue: " ".to_string(),
                critter: r"             $6
$8      /__    $9$6
$8     / $1$2\  $9$5
$8     |  |$3$4
$8     |  |
$8 (III|\||  $9$0"
                    .to_string(),
            }
        }
        let mut config = CritterConfig {
            left_eye: template.default_left_eye.clone(),
            right_eye: template.default_right_eye.clone(),

            left_tongue: template.default_left_tongue.clone(),
            right_tongue: template.default_right_tongue.clone(),

            right_line: String::from("/"),
            up_line: String::from("|"),
            left_line: String::from("\\"),

            object: String::from(" "),

            format: reset(), // from kule

            template: template,
        };
        if let Some(eyes) = eyes {
            match count::count_graphemes(&eyes) {
                0 => {
                    (config.left_eye, config.right_eye) = (
                        config.template.default_left_eye.clone(),
                        config.template.default_right_eye.clone(),
                    )
                }
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
                0 => {
                    (config.left_tongue, config.right_tongue) = (
                        config.template.default_left_tongue.clone(),
                        config.template.default_right_tongue.clone(),
                    )
                }
                1 => {
                    (config.left_tongue, config.right_tongue) = (
                        chop::grapheme_at(&tongue, 0),
                        config.template.default_right_tongue.clone(),
                    )
                }
                _ => {
                    (config.left_tongue, config.right_tongue) =
                        (chop::grapheme_at(&tongue, 0), chop::grapheme_at(&tongue, 1))
                }
            }
        } else if let Some(line) = line {
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

        return Ok(config);
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
    // attempts to interpret file as a path, and if this fails, tries appending it to every location in the kijepath environment variable.
    fn template_from_file(name: &str) -> Result<CritterTemplate, String> {
        let mut file = fs::read_to_string(name);
        let paths = path();
        if file.is_err() && !paths.is_empty() {
            for path in path() {
                match fs::read_to_string(&format!("{}{}", manipulate::finish(&path, "/"), name)) {
                    Ok(f) => {
                        file = Ok(f);
                        break;
                    }
                    Err(e) => file = Err(e),
                }
            }
        }
        let file = file.map_err(|_| format!("mi ken ala lukin e nimi kije {}.\n - sina wile lukin e kije ale la o `kijetesantakaluotokieni --seme`\n - sina ken kepeken nimi suli, sama ni: /home/mi/kije\n - nimi poki li lon nimi $NASINKIJE la ilo kijetesantakaluotokieni li\n   alasa lon poki ni. o kipisi e nimi poki kepeken sitelen \":\".\n\ncouldn't find/read kijefile {}. check available critters with -l or --seme, try again with a full file path, or add colon-separated directories to $NASINKIJE", name, name))?;

        let mut lines = file.lines().skip_while(|l| l.starts_with('#')); // skips comments

        let anchor: usize;
        if let Some(anchor_line) = lines.next() {
            anchor = anchor_line.trim().parse().map_err(|_| {
                "nanpa li nasa lon lipu kije. o pona e ona.\n\ncouldn't parse anchor as number"
                    .to_string()
            })?;
        } else {
            return Err("ale li weka tan lipu kije. ona li wile e nanpa e sitelen.\n\nkijefile missing content".to_string());
        }
        let default_left_eye = lines
            .next()
            .ok_or(
                "lukin nanpa wan li lon ala lipu kije\nleft eye missing from kijefile".to_string(),
            )?
            .to_string();
        let default_right_eye = lines
            .next()
            .ok_or(
                "lukin nanpa tu li lon ala lipu kije\nright eye missing from kijefile".to_string(),
            )?
            .to_string();
        let default_left_tongue = lines
            .next()
            .ok_or(
                "uta nanpa wan li lon ala lipu kije\nleft tongue missing from kijefile".to_string(),
            )?
            .to_string();
        let default_right_tongue = lines
            .next()
            .ok_or(
                "uta nanpa tu li lon ala lipu kije\nright tongue missing from kijefile".to_string(),
            )?
            .to_string();
        let mut critter = String::new();
        lines.for_each(|l| critter.push_str(&format!("{}\n", l).to_string()));

        Ok(CritterTemplate {
            default_left_eye,
            default_right_eye,
            default_left_tongue,
            default_right_tongue,
            anchor,
            critter,
        })
    }
}

fn path() -> Vec<String> {
    match env::var("NASINKIJE") {
        Err(_) => Vec::new(),
        Ok(s) => s.split(":").map(|s| s.trim().to_string()).collect(),
    }
}

pub fn list_files() -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    // must be updated alongside the name match statement in CritterConfig::config_from_string
    for builtin in [
        "kijetesantakalu",
        "kuletesantakalu",
        "kijetonsitakalu",
        "kijetonsitakatu",
        "lili",
        "soweli",
        "soweli-a",
        "waso",
        "kala",
        "pipi",
        "akesi",
        "soko",
        "kasi",
        "toki-pona",
        "mu",
        "mani",
        "mani-majuna",
    ] {
        files.push(builtin.to_string());
    }
    for i in path() {
        match fs::read_dir(&i) {
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    return Err(
                        format!("mi ken ala lukin e poki ni: {}\n\npermission denied", i)
                            .to_string(),
                    )
                }
                io::ErrorKind::NotFound => {
                    return Err(format!(
                        "poki ni li lon ala: {}\n\ndirectory not found",
                        i.to_string()
                    ))
                }
                _ => {
                    return Err(format!("ijo li pakala lon ni: {}\n\n{:?}", i, e.kind()).to_string())
                }
            },
            Ok(entries) => {
                for read in entries {
                    let filename = read
                        .map_err(|e|format!("mi ken ala lukin e lipu lon ni: {}\n\n{}", i, e.to_string()).to_string())?
                        .file_name()
                        .into_string()
                        .map_err(|_|format!("mi ken ala sitelen UTF-8 e nimi lipu lon ni: {}\n\ncould not display file name as utf-8", i).to_string())?;
                    files.push(filename);
                }
            }
        }
    }
    return Ok(files);
}
