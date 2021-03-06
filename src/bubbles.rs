use voca_rs::*;

// carry's information about the anchor point for a bubble, what column to wrap words at, and what the bubbles border looks like
pub struct BubbleConfig {
    anchor: usize,
    wrap: usize,
    no_wrap: bool,

    top: String,
    left: String,
    right: String,
    bottom: String,
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
    middle_left: String,
    middle_right: String,
}

impl BubbleConfig {
    // assembles a config from critical information, attempting to use sensible defaults where possible
    pub fn config_from_string(
        anchor: usize,
        wrap: usize,
        no_wrap: bool,
        border: &Option<String>,
    ) -> BubbleConfig {
        if let Some(border) = border {
            let chars = split::graphemes(&border);
            let top = if let Some(item) = chars.get(0) {
                item.to_string()
            } else {
                // Some("") results in the default like None
                return BubbleConfig {
                    anchor,
                    wrap,
                    no_wrap,

                    top: "_".to_string(),
                    left: "<".to_string(),
                    right: ">".to_string(),
                    bottom: "-".to_string(),
                    top_left: "/".to_string(),
                    top_right: "\\".to_string(),
                    bottom_left: "\\".to_string(),
                    bottom_right: "/".to_string(),
                    middle_left: "|".to_string(),
                    middle_right: "|".to_string(),
                };
            };
            //
            let left = if let Some(item) = chars.get(1) {
                item.to_string()
            } else {
                top.clone()
            };
            let right = if let Some(item) = chars.get(2) {
                item.to_string()
            } else {
                left.clone()
            };
            let bottom = if let Some(item) = chars.get(3) {
                item.to_string()
            } else {
                top.clone()
            };
            let top_left = if let Some(item) = chars.get(4) {
                item.to_string()
            } else {
                left.clone()
            };
            let top_right = if let Some(item) = chars.get(5) {
                item.to_string()
            } else {
                right.clone()
            };
            let bottom_left = if let Some(item) = chars.get(6) {
                item.to_string()
            } else {
                left.clone()
            };
            let bottom_right = if let Some(item) = chars.get(7) {
                item.to_string()
            } else {
                right.clone()
            };
            let middle_left = if let Some(item) = chars.get(8) {
                item.to_string()
            } else {
                left.clone()
            };
            let middle_right = if let Some(item) = chars.get(9) {
                item.to_string()
            } else {
                right.clone()
            };
            BubbleConfig {
                anchor,
                wrap,
                no_wrap,

                top,
                left,
                right,
                bottom,
                top_left,
                top_right,
                bottom_left,
                bottom_right,
                middle_left,
                middle_right,
            }
        } else {
            BubbleConfig {
                anchor,
                wrap,
                no_wrap,

                top: "_".to_string(),
                left: "<".to_string(),
                right: ">".to_string(),
                bottom: "-".to_string(),
                top_left: "/".to_string(),
                top_right: "\\".to_string(),
                bottom_left: "\\".to_string(),
                bottom_right: "/".to_string(),
                middle_left: "|".to_string(),
                middle_right: "|".to_string(),
            }
        }
    }

    // front end to bubble_from_lines which takes normal text.
    pub fn bubble_from_text(&self, text: &str) -> String {
        if self.no_wrap {
            self.bubble_from_lines(&text.lines().map(|s| s.to_string()).collect())
        } else {
            self.bubble_from_lines(&wrap_block(text, self.wrap))
        }
    }

    // generates a bubble (doesn't end in newline) using a vector of strings as lines.
    fn bubble_from_lines(&self, lines: &Vec<String>) -> String {
        let longest_length: usize;
        match lines.iter().map(|s| count::count_graphemes(s)).max() {
            None => return "".to_string(),
            Some(l) => longest_length = l,
        };

        let left_pad_length = if longest_length < self.anchor {
            self.anchor + (longest_length / 2) + 2
        } else {
            0
        };

        let bubble_top = manipulate::pad_left(
            &format!(" {} \n", self.top.repeat(longest_length + 2),),
            left_pad_length,
            " ",
        );
        let bubble_bottom = manipulate::pad_left(
            &format!(" {} \n", self.bottom.repeat(longest_length + 2),),
            left_pad_length,
            " ",
        );
        let mut bubble_body = String::new();
        match lines.len() {
            1 => bubble_body.push_str(&manipulate::pad_left(
                &format!("{} {} {}\n", self.left, lines[0], self.right),
                left_pad_length,
                " ",
            )),
            n => {
                for (i, line) in lines.iter().enumerate() {
                    let (left, right) = if i == 0 {
                        (self.top_left.clone(), self.top_right.clone())
                    } else if i == n - 1 {
                        (self.bottom_left.clone(), self.bottom_right.clone())
                    } else {
                        (self.middle_left.clone(), self.middle_right.clone())
                    };
                    bubble_body.push_str(&manipulate::pad_left(
                        &format!(
                            "{} {}{} {}\n",
                            left,
                            line,
                            " ".repeat(longest_length - count::count_graphemes(&line)),
                            right
                        ),
                        left_pad_length,
                        " ",
                    ));
                }
            }
        }
        return format!("{}{}{}", bubble_top, bubble_body, bubble_bottom);
    }
}

// "breaks off" a wrapped line of text and returns it with the rests of the text.
fn wrap_once(text: &str, max_length: usize) -> (String, Option<String>) {
    let whitespace_not_newline = |c: char| c.is_whitespace() && c != '\n';
    let mut length: usize = 0;
    let mut last_space: Option<usize> = None;

    for s in split::graphemes(text) {
        length = length + 1;
        if s == "\n" {
            // chop::substring treats 0 as 'till the end of the string' so if we want to cut off initial
            // newlines into empty strings we need special handling
            let broken_line = if length == 1 {
                "".to_string()
            } else {
                chop::substring(text, 0, length - 1)
            };
            let rest = chop::substring(text, length, count::count_graphemes(text))
                .trim_start_matches(whitespace_not_newline)
                .to_string();
            return (broken_line, Some(rest));
        }
        if query::is_blank(s) {
            // minus one because last_space is an index
            last_space = Some(length - 1);
        }
        if length == max_length {
            match last_space {
                None => {
                    // breaks in middle of word as in cowsay
                    let broken_line = chop::substring(text, 0, length);
                    let rest = chop::substring(text, length, count::count_graphemes(text))
                        .trim_start_matches(whitespace_not_newline)
                        .to_string();
                    return (broken_line, Some(rest));
                }
                Some(last_space) => {
                    // break at last whitespace, discarding it
                    let broken_line = chop::substring(text, 0, last_space);
                    let rest = chop::substring(text, last_space + 1, count::count_graphemes(text))
                        .trim_start_matches(whitespace_not_newline)
                        .to_string();
                    return (broken_line, Some(rest));
                }
            };
        };
    }
    // if we get here it means the string wasn't long enough to be broken
    return (text.to_string(), None);
}

/*
splits a bunch of text into wrapped lines
*/
fn wrap_block(text: &str, max_length: usize) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let mut remaining = text.trim_end().to_string();
    loop {
        let (line, rest) = wrap_once(&remaining, max_length);
        lines.push(line);
        match rest {
            None => break,
            Some(rest) => remaining = rest,
        };
    }
    return lines;
}
