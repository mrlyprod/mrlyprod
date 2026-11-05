use crate::model::{Back, Hand, Result};
use crate::toml::{self, Table, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const FILE: &str = "bridge.toml";

const SKIP: &str = "skip.txt";

const MANIFEST: &str = "manifest.json";

pub struct Config {
    pub root: PathBuf,
    pub skip: PathBuf,
    pub own: Leaf,
    pub mounts: Vec<Mount>,
    pub hands: Vec<Hand>,
}

pub struct Mount {
    pub unit: String,
    pub src: PathBuf,
    pub uses: Vec<String>,
    pub leaf: bool,
    pub hand: bool,
}

pub struct Leaf {
    pub unit: String,
    pub manifest: PathBuf,
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
}

pub fn root() -> Result<PathBuf> {
    let here = std::env::current_dir().map_err(|e| format!("the working folder: {e}"))?;
    here.ancestors()
        .find(|dir| dir.join(FILE).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("{} sits in no workspace", here.display()))
}

pub fn load(root: &Path) -> Result<Config> {
    if !root.join(FILE).is_file() {
        return Err(format!(
            "{}: no {FILE}; the bridge runs inside a unit repo",
            root.display()
        ));
    }
    let mut dirs = members(root)?;
    dirs.retain(|dir| dir.join(FILE).is_file());
    if !dirs.iter().any(|dir| same(dir, root)) {
        dirs.push(root.to_path_buf());
    }
    let mut mounts = Vec::new();
    let mut hands = Vec::new();
    let mut own = None;
    for dir in dirs {
        let mut doc = read(&dir)?;
        let Some(unit) = doc.top.remove("unit") else {
            return Err(format!(
                "{}: a crate without a unit",
                dir.join(FILE).display()
            ));
        };
        let Value::Text(unit) = unit else {
            return Err(format!("{}: unit is a string", dir.display()));
        };
        let outs = match ["js", "py", "cli"].map(|key| doc.top.contains_key(key)) {
            [false, false, false] => None,
            [true, true, true] => {
                let mut out = |key: &str| -> Result<PathBuf> {
                    inside(&dir.join(text(&mut doc.top, key, &dir)?), &dir)
                };
                Some((out("js")?, out("py")?, out("cli")?))
            }
            _ => {
                return Err(format!(
                    "{}: js, py and cli come together",
                    dir.join(FILE).display()
                ));
            }
        };
        done(&doc.top, &dir)?;
        let owns = !doc.arrays.is_empty();
        for (name, mut row) in doc.arrays {
            if name != "hand" {
                return Err(format!("{}: unknown table [[{name}]]", dir.display()));
            }
            let row = hand(&mut row, &dir)?;
            if hands.iter().any(|h: &Hand| h.name == row.name) {
                return Err(format!("{}: hand {} twice", dir.display(), row.name));
            }
            hands.push(row);
        }
        if same(&dir, root) {
            let Some((js, py, cli)) = &outs else {
                return Err(format!(
                    "{}: the unit of this repo has no js, py and cli",
                    dir.join(FILE).display()
                ));
            };
            own = Some(Leaf {
                unit: unit.clone(),
                manifest: PathBuf::from(MANIFEST),
                js: js.clone(),
                py: py.clone(),
                cli: cli.clone(),
            });
        }
        mounts.push(Mount {
            uses: uses(&dir)?,
            src: dir.join("src"),
            unit,
            leaf: outs.is_some(),
            hand: owns,
        });
    }
    if hands.iter().filter(|h| h.big).count() > 1 {
        return Err("two [[hand]] rows are big; a code crosses as the one big row".to_string());
    }
    mounts.sort_by(|a, b| a.unit.cmp(&b.unit));
    Ok(Config {
        root: root.to_path_buf(),
        skip: PathBuf::from(SKIP),
        own: own.ok_or("the repo's own unit was not read")?,
        mounts,
        hands,
    })
}

fn members(root: &Path) -> Result<Vec<PathBuf>> {
    let out = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "cargo metadata: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let meta: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("cargo metadata: {e}"))?;
    let packages = meta["packages"].as_array().cloned().unwrap_or_default();
    let manifest = |id: &str| {
        packages
            .iter()
            .find(|package| package["id"].as_str() == Some(id))
            .and_then(|package| package["manifest_path"].as_str())
            .and_then(|path| Path::new(path).parent())
            .map(Path::to_path_buf)
    };
    let nodes = meta["resolve"]["nodes"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut todo: Vec<String> = nodes
        .iter()
        .filter_map(|node| node["id"].as_str())
        .filter(|id| manifest(id).is_some_and(|dir| within(&dir, root)))
        .map(str::to_string)
        .collect();
    let mut seen: Vec<String> = Vec::new();
    while let Some(id) = todo.pop() {
        if seen.contains(&id) {
            continue;
        }
        if let Some(node) = nodes.iter().find(|node| node["id"].as_str() == Some(&id)) {
            todo.extend(
                node["deps"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|dep| dep["pkg"].as_str())
                    .map(str::to_string),
            );
        }
        seen.push(id);
    }
    let mut dirs: Vec<PathBuf> = seen.iter().filter_map(|id| manifest(id)).collect();
    dirs.sort();
    dirs.dedup();
    Ok(dirs)
}

fn within(dir: &Path, root: &Path) -> bool {
    match (dir.canonicalize(), root.canonicalize()) {
        (Ok(dir), Ok(root)) => dir.starts_with(root),
        _ => dir.starts_with(root),
    }
}

fn same(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
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
            out.push(unit);
        }
    }
    out.sort();
    Ok(out)
}

fn read(dir: &Path) -> Result<toml::Doc> {
    let path = dir.join(FILE);
    let body = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::parse(&body).map_err(|e| format!("{}: {e}", path.display()))
}

fn hand(row: &mut Table, dir: &Path) -> Result<Hand> {
    let hand = Hand {
        name: text(row, "name", dir)?,
        rust: list(row, "rust", dir)?,
        js: list(row, "js", dir)?,
        word: text(row, "word", dir)?,
        ts: text(row, "ts", dir)?,
        ts_data: text(row, "ts_data", dir)?,
        py: list(row, "py", dir)?,
        py_into: text(row, "py_into", dir)?,
        pyi: text(row, "pyi", dir)?,
        stream: flag(row, "stream", dir)?,
        big: flag(row, "big", dir)?,
        rank: flag(row, "rank", dir)?,
        write_back: match row.remove("write_back") {
            None => None,
            Some(Value::Text(back)) if back == "ref" => Some(Back::Ref),
            Some(Value::Text(back)) if back == "value" => Some(Back::Value),
            Some(_) => {
                return Err(format!(
                    "{}: write_back is \"ref\" or \"value\"",
                    dir.join(FILE).display()
                ));
            }
        },
        cli: match row.remove("cli") {
            None => None,
            Some(Value::List(pair)) if pair.len() == 2 => Some((pair[0].clone(), pair[1].clone())),
            Some(_) => {
                return Err(format!(
                    "{}: cli is a reader and a writer",
                    dir.join(FILE).display()
                ));
            }
        },
    };
    done(row, dir)?;
    Ok(hand)
}

fn flag(table: &mut Table, key: &str, dir: &Path) -> Result<bool> {
    match table.remove(key) {
        None => Ok(false),
        Some(Value::Flag(flag)) => Ok(flag),
        Some(_) => Err(format!(
            "{}: {key} is true or false",
            dir.join(FILE).display()
        )),
    }
}

fn text(table: &mut Table, key: &str, dir: &Path) -> Result<String> {
    match table.remove(key) {
        Some(Value::Text(text)) => Ok(text),
        _ => Err(format!("{}: {key} is a string", dir.join(FILE).display())),
    }
}

fn list(table: &mut Table, key: &str, dir: &Path) -> Result<Vec<String>> {
    match table.remove(key) {
        Some(Value::List(list)) => Ok(list),
        _ => Err(format!(
            "{}: {key} is a list of strings",
            dir.join(FILE).display()
        )),
    }
}

fn done(table: &Table, dir: &Path) -> Result<()> {
    match table.keys().next() {
        Some(key) => Err(format!("{}: unknown key {key}", dir.join(FILE).display())),
        None => Ok(()),
    }
}

fn inside(path: &Path, dir: &Path) -> Result<PathBuf> {
    let rel = path
        .strip_prefix(dir)
        .map_err(|_| format!("{} lies outside its crate", path.display()))?;
    if rel
        .components()
        .any(|part| !matches!(part, std::path::Component::Normal(_)))
    {
        return Err(format!("{} lies outside its crate", path.display()));
    }
    Ok(rel.to_path_buf())
}

pub fn slashed(path: &Path) -> String {
    path.components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
