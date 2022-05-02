use voca_rs::*;

pub fn bubble_from_text(text: &str, bubble_anchor: usize, max_length: usize) -> String {
    bubble_from_lines(wrap_block(text, max_length), bubble_anchor)
}

fn bubble_from_lines(lines: Vec<String>, min_length: usize) -> String {
    let longest_length: usize;
    let lengths: Vec<(&String, usize)> = lines
        .iter()
        .zip(lines.iter().map(|s| count::count_graphemes(s)))
        .collect();
    match lines.iter().map(|s| count::count_graphemes(s)).max() {
        None => return "".to_string(),
        Some(l) => longest_length = l,
    };

    // let line_length = cmp::max(longest_length, min_length);
    let line_length = longest_length;
    let pad_length = if longest_length < min_length {
        min_length + (longest_length / 2) + 2
    } else {
        0
    };
    let bubble_top = manipulate::pad_left(
        &format!(" _{}_ \n", "_".repeat(line_length)),
        pad_length,
        " ",
    );
    let bubble_bottom = manipulate::pad_left(
        &format!(" -{}-  ", "-".repeat(line_length)),
        pad_length,
        " ",
    );
    let mut bubble_body = String::new();

    match lines.len() {
        1 => {
            return format!(
                "{}{}{}",
                bubble_top,
                manipulate::pad_left(&format!("< {} >\n", lines[0]), pad_length, " "),
                bubble_bottom
            )
        }
        n => {
            bubble_body.push_str(&manipulate::pad_left(
                &format!("/ {} \\\n", lines[0]),
                pad_length,
                " ",
            ));
            if n > 2 {
                for i in 1..n - 1 {
                    bubble_body.push_str(&manipulate::pad_left(
                        &format!("| {} |\n", lines[i]),
                        pad_length,
                        " ",
                    ));
                }
            }
            bubble_body.push_str(&manipulate::pad_left(
                &format!("\\ {} /\n", lines[n - 1]),
                pad_length,
                " ",
            ));
            return format!("{}{}{}", bubble_top, bubble_body, bubble_bottom);
        }
    }
}

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
