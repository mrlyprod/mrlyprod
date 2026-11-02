use crate::model::Result;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Text(String),
    List(Vec<String>),
}

pub type Table = BTreeMap<String, Value>;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Doc {
    pub top: Table,
    pub arrays: Vec<(String, Table)>,
}

pub fn parse(text: &str) -> Result<Doc> {
    let mut doc = Doc::default();
    for (at, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fail = |what: &str| format!("line {}: {what}", at + 1);
        if let Some(name) = line.strip_prefix("[[").and_then(|l| l.strip_suffix("]]")) {
            let name = name.trim();
            if !bare(name) {
                return Err(fail("a table name is a bare key"));
            }
            doc.arrays.push((name.to_string(), Table::new()));
            continue;
        }
        let (key, rest) = line
            .split_once('=')
            .ok_or_else(|| fail("expected key = value"))?;
        let key = key.trim();
        if !bare(key) {
            return Err(fail("a key is bare: letters, digits, _ and -"));
        }
        let mut chars = rest.trim().chars().peekable();
        let value = match chars.peek() {
            Some('"') => Value::Text(text_of(&mut chars).map_err(|e| fail(&e))?),
            Some('[') => Value::List(list_of(&mut chars).map_err(|e| fail(&e))?),
            _ => return Err(fail("a value is a string or a list of strings")),
        };
        let tail: String = chars.collect();
        let tail = tail.trim();
        if !tail.is_empty() && !tail.starts_with('#') {
            return Err(fail(&format!("unexpected {tail}")));
        }
        let table = match doc.arrays.last_mut() {
            Some((_, table)) => table,
            None => &mut doc.top,
        };
        if table.insert(key.to_string(), value).is_some() {
            return Err(fail(&format!("{key} twice")));
        }
    }
    Ok(doc)
}

fn bare(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn text_of(chars: &mut Chars) -> std::result::Result<String, String> {
    chars.next();
    let mut out = String::new();
    loop {
        match chars.next().ok_or("an unclosed string")? {
            '"' => return Ok(out),
            '\\' => out.push(match chars.next().ok_or("an unclosed string")? {
                '"' => '"',
                '\\' => '\\',
                'n' => '\n',
                't' => '\t',
                other => return Err(format!("an unknown escape \\{other}")),
            }),
            c => out.push(c),
        }
    }
}

fn list_of(chars: &mut Chars) -> std::result::Result<Vec<String>, String> {
    chars.next();
    let mut out = Vec::new();
    loop {
        while chars.next_if(|c| c.is_whitespace()).is_some() {}
        match chars.peek() {
            Some(']') => {
                chars.next();
                return Ok(out);
            }
            Some('"') => out.push(text_of(chars)?),
            _ => return Err("a list holds strings on one line".into()),
        }
        while chars.next_if(|c| c.is_whitespace()).is_some() {}
        match chars.next() {
            Some(',') => {}
            Some(']') => return Ok(out),
            _ => return Err("a list item ends with , or ]".into()),
        }
    }
}
