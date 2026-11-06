use crate::model::{Back, Hand, Result};
use crate::toml::{self, Table, Value};
use std::path::{Path, PathBuf};

const INFRA: [&str; 2] = ["wire", "bindgen"];
const RESERVED: [&str; 3] = ["all", "blobs", "units"];
pub const ALL: &str = "all";

pub struct Config {
    pub root: PathBuf,
    pub out: PathBuf,
    pub mounts: Vec<Mount>,
    pub hands: Vec<Hand>,
    pub json: String,
    pub blobs: Vec<Blob>,
}

pub struct Blob {
    pub name: String,
    pub crates: Vec<String>,
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
    let json = text(&mut doc.top, "json")?;
    let mut hands = Vec::new();
    for (name, mut row) in std::mem::take(&mut doc.arrays) {
        if name != "hand" {
            return Err(format!("units.toml: unknown table [[{name}]]"));
        }
        hands.push(hand(&mut row)?);
    }
    let mut mounts = Vec::new();
    for unit in folders(&root.join("packages/rs"))? {
        let dir = root.join("packages/rs").join(&unit);
        if INFRA.contains(&unit.as_str()) || !dir.join("src/lib.rs").exists() {
            continue;
        }
        if RESERVED.contains(&unit.as_str()) {
            return Err(format!("packages/rs/{unit}: the name {unit} is reserved"));
        }
        mounts.push(Mount {
            unit: unit.clone(),
            uses: uses(&dir)?,
            src: dir.join("src"),
        });
    }
    mounts.sort_by(|a, b| a.unit.cmp(&b.unit));
    let blobs = blobs(&root, &mounts)?;
    Ok(Config {
        root,
        out,
        mounts,
        hands,
        json,
        blobs,
    })
}

fn folders(dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            out.push(name.to_string());
        }
    }
    out.sort();
    Ok(out)
}

fn blobs(root: &Path, mounts: &[Mount]) -> Result<Vec<Blob>> {
    let mut lists: Vec<Vec<Blob>> = Vec::new();
    for site in folders(&root.join("sites"))? {
        let path = root.join("sites").join(&site).join("site/site.json");
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let doc = mrly_json::parse(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let Some(list) = doc.get("wasm") else {
            continue;
        };
        let list = list
            .as_object()
            .ok_or_else(|| format!("{}: wasm maps a blob to its crates", path.display()))?;
        let mut made = Vec::new();
        for (name, crates) in list {
            let crates = crates
                .as_array()
                .ok_or_else(|| format!("{}: wasm.{name} is a list", path.display()))?
                .iter()
                .map(|one| {
                    one.as_str()
                        .map(String::from)
                        .ok_or_else(|| format!("{}: wasm.{name} holds strings", path.display()))
                })
                .collect::<Result<Vec<String>>>()?;
            made.push(Blob {
                name: name.clone(),
                crates,
            });
        }
        lists.push(made);
    }
    let mut list = match lists.len() {
        0 => Vec::new(),
        1 => lists.remove(0),
        _ => return Err("more than one site.json sets wasm; one blob list holds".to_string()),
    };
    let mut seen: Vec<&str> = Vec::new();
    for blob in &list {
        if blob.name == ALL {
            return Err("site.json wasm: all is the built-in blob of every crate".to_string());
        }
        for one in &blob.crates {
            if !mounts.iter().any(|mount| mount.unit == *one) {
                return Err(format!("site.json wasm.{}: no crate {one}", blob.name));
            }
            if seen.contains(&one.as_str()) {
                return Err(format!("site.json wasm: {one} is in two blobs"));
            }
            seen.push(one);
        }
    }
    list.push(Blob {
        name: ALL.to_string(),
        crates: mounts.iter().map(|mount| mount.unit.clone()).collect(),
    });
    Ok(list)
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
        decl: match row.remove("decl") {
            None => String::new(),
            Some(Value::Text(decl)) => decl,
            Some(_) => return Err("units.toml: decl is a string".to_string()),
        },
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
