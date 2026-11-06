use mrly_error::{Result, value_error};
use serde::{Deserialize, Serialize};

const NEEDED: [&str; 6] = ["title", "description", "tags", "maker", "date", "origin"];
const SLOWEST: usize = 655_350;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub delay: usize,
    pub maker: String,
    pub date: String,
    pub origin: String,
}

pub fn slug(title: &str) -> String {
    let mut out = String::new();
    for glyph in title.chars() {
        if glyph.is_alphanumeric() {
            out.extend(glyph.to_lowercase());
        } else if glyph == '\'' || glyph == '\u{2019}' {
            continue;
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

fn string(text: &str) -> Result<(String, &str)> {
    let Some(mut rest) = text.strip_prefix('"') else {
        return value_error(format!(
            "'{text}' should be a double-quoted string on one line."
        ));
    };
    let mut out = String::new();
    loop {
        let mut glyphs = rest.chars();
        match glyphs.next() {
            None => return value_error("a string has no closing quote."),
            Some('"') => return Ok((out, glyphs.as_str())),
            Some('\\') => match glyphs.next() {
                Some(glyph @ ('"' | '\\')) => out.push(glyph),
                _ => return value_error("a backslash may only escape a quote or a backslash."),
            },
            Some(glyph) => out.push(glyph),
        }
        rest = glyphs.as_str();
    }
}

fn whole(text: &str) -> Result<String> {
    let (out, rest) = string(text)?;
    if !rest.is_empty() {
        return value_error(format!(
            "'{rest}' follows a closed string; comments are not read."
        ));
    }
    Ok(out)
}

fn list(text: &str) -> Result<Vec<String>> {
    let Some(mut rest) = text.strip_prefix('[') else {
        return value_error(format!(
            "'{text}' should be a one-line list like [\"a\", \"b\"]."
        ));
    };
    let mut out = Vec::new();
    loop {
        rest = rest.trim_start();
        if let Some(tail) = rest.strip_prefix(']') {
            if !tail.is_empty() {
                return value_error(format!(
                    "'{tail}' follows a closed list; comments are not read."
                ));
            }
            return Ok(out);
        }
        let (word, tail) = string(rest)?;
        out.push(word);
        rest = tail.trim_start();
        if let Some(tail) = rest.strip_prefix(',') {
            rest = tail;
        } else if !rest.starts_with(']') {
            return value_error("list items are split by commas.");
        }
    }
}

fn dated(text: &str) -> bool {
    let bytes = text.as_bytes();
    let shaped = bytes.len() == 10
        && bytes.iter().enumerate().all(|(at, byte)| match at {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        });
    let part = |from: usize| text[from..from + 2].parse::<usize>().unwrap_or(0);
    shaped && (1..=12).contains(&part(5)) && (1..=31).contains(&part(8))
}

pub fn read(text: &str) -> Result<Entry> {
    let mut entry = Entry::default();
    let mut seen: Vec<&str> = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let Some((key, value)) = line.split_once('=') else {
            return value_error(format!(
                "'{line}' is not read: an entry holds only lines like key = value, no comments."
            ));
        };
        let (key, value) = (key.trim(), value.trim());
        if seen.contains(&key) {
            return value_error(format!("'{key}' is set twice."));
        }
        match key {
            "title" => entry.title = whole(value)?,
            "description" => entry.description = whole(value)?,
            "tags" => entry.tags = list(value)?,
            "maker" => entry.maker = whole(value)?,
            "date" => entry.date = whole(value)?,
            "origin" => entry.origin = whole(value)?,
            "delay" => match value.parse() {
                Ok(delay) if delay <= SLOWEST => entry.delay = delay,
                _ => {
                    return value_error(format!(
                        "delay '{value}' is not a whole number of ms up to {SLOWEST}."
                    ));
                }
            },
            _ => {
                return value_error(format!(
                    "'{key}' is not a key an entry knows: {}, delay.",
                    NEEDED.join(", ")
                ));
            }
        }
        seen.push(key);
    }
    if let Some(key) = NEEDED.iter().find(|key| !seen.contains(key)) {
        return value_error(format!("'{key}' is missing."));
    }
    if slug(&entry.title).is_empty() {
        return value_error("the title needs a letter or a digit.");
    }
    if !dated(&entry.date) {
        return value_error(format!(
            "date '{}' should read like 2026-11-06.",
            entry.date
        ));
    }
    Ok(entry)
}
