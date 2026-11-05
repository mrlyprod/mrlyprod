use crate::model::{Back, Hand, Result};
use crate::toml::{self, Table, Value};
use std::path::{Path, PathBuf};

const UNITS: [&str; 6] = ["core", "num", "math", "tile", "life", "font"];

pub struct Config {
    pub root: PathBuf,
    pub out: PathBuf,
    pub mounts: Vec<Mount>,
    pub hands: Vec<Hand>,
}

pub struct Mount {
    pub unit: String,
    pub src: PathBuf,
    pub uses: Vec<String>,
}

pub fn load() -> Result<Config> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = here
        .ancestors()
        .nth(3)
        .ok_or("the bridge lies outside its repo")?
        .to_path_buf();
    let relative = here.strip_prefix(&root).map_err(|e| e.to_string())?;
    let out = root.join("data").join(relative);
    let mut doc =
        toml::parse(include_str!("units.toml")).map_err(|e| format!("units.toml: {e}"))?;
    let mut hands = Vec::new();
    for (name, mut row) in std::mem::take(&mut doc.arrays) {
        if name != "hand" {
            return Err(format!("units.toml: unknown table [[{name}]]"));
        }
        hands.push(hand(&mut row)?);
    }
    let mut mounts = Vec::new();
    for unit in UNITS {
        let dir = root.join("packages/rs").join(unit);
        mounts.push(Mount {
            unit: unit.to_string(),
            uses: uses(&dir)?,
            src: dir.join("src"),
        });
    }
    mounts.sort_by(|a, b| a.unit.cmp(&b.unit));
    Ok(Config {
        root,
        out,
        mounts,
        hands,
    })
}

fn uses(dir: &Path) -> Result<Vec<String>> {
    let path = dir.join("Cargo.toml");
    let body = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut table = "";
    let mut out = Vec::new();
    for line in body.lines().map(str::trim) {
        if line.starts_with('[') {
            table = line;
        } else if table == "[dependencies]"
            && let Some(rest) = line.strip_prefix("mrly-")
        {
            let unit: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if unit != "wire" {
                out.push(unit);
            }
        }
    }
    out.sort();
    Ok(out)
}

fn hand(row: &mut Table) -> Result<Hand> {
    let hand = Hand {
        name: text(row, "name")?,
        rust: list(row, "rust")?,
        word: text(row, "word")?,
        ts: text(row, "ts")?,
        ts_data: text(row, "ts_data")?,
        stream: flag(row, "stream")?,
        big: flag(row, "big")?,
        rank: flag(row, "rank")?,
        write_back: match row.remove("write_back") {
            None => None,
            Some(Value::Text(back)) if back == "ref" => Some(Back::Ref),
            Some(Value::Text(back)) if back == "value" => Some(Back::Value),
            Some(_) => return Err("units.toml: write_back is \"ref\" or \"value\"".to_string()),
        },
    };
    match row.keys().next() {
        Some(key) => Err(format!("units.toml: unknown key {key}")),
        None => Ok(hand),
    }
}

fn flag(table: &mut Table, key: &str) -> Result<bool> {
    match table.remove(key) {
        None => Ok(false),
        Some(Value::Flag(flag)) => Ok(flag),
        Some(_) => Err(format!("units.toml: {key} is true or false")),
    }
}

fn text(table: &mut Table, key: &str) -> Result<String> {
    match table.remove(key) {
        Some(Value::Text(text)) => Ok(text),
        _ => Err(format!("units.toml: {key} is a string")),
    }
}

fn list(table: &mut Table, key: &str) -> Result<Vec<String>> {
    match table.remove(key) {
        Some(Value::List(list)) => Ok(list),
        _ => Err(format!("units.toml: {key} is a list of strings")),
    }
}
