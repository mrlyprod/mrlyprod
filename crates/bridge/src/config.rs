use crate::model::{Hand, Result};
use crate::toml::{self, Table, Value};
use std::path::{Component, Path, PathBuf};

pub const FILE: &str = "bridge.toml";

const SKIP: &str = "skip.txt";

pub struct Config {
    pub root: PathBuf,
    pub layout: Layout,
    pub mounts: Vec<Mount>,
    pub hands: Vec<Hand>,
}

pub struct Layout {
    pub manifest: PathBuf,
    pub skip: PathBuf,
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
}

pub struct Mount {
    pub unit: Option<String>,
    pub src: PathBuf,
}

pub fn home() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn load(home: &Path) -> Result<Config> {
    let root = home
        .ancestors()
        .skip(1)
        .find(|dir| dir.join("Cargo.lock").is_file())
        .ok_or("the bridge sits in no workspace")?
        .to_path_buf();
    let mut own = read(home)?;
    let crates = normal(&home.join(text(&mut own.top, "crates", home)?));
    let manifest = inside(&root, &home.join(text(&mut own.top, "manifest", home)?))?;
    done(&own.top, home)?;
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&crates)
        .map_err(|e| format!("{}: {e}", crates.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|dir| dir.join(FILE).is_file() && normal(dir) != normal(home))
        .collect();
    dirs.sort();
    let mut facade = None;
    let mut mounts = Vec::new();
    let mut hands = Vec::new();
    for dir in dirs {
        let mut doc = read(&dir)?;
        let src = dir.join("src");
        let Some(unit) = doc.top.remove("unit") else {
            if facade.is_some() {
                return Err(format!("{}: a second crate without a unit", dir.display()));
            }
            let mut out = |key: &str| -> Result<PathBuf> {
                inside(&root, &dir.join(text(&mut doc.top, key, &dir)?))
            };
            facade = Some((out("js")?, out("py")?, out("cli")?));
            done(&doc.top, &dir)?;
            if !doc.arrays.is_empty() {
                return Err(format!("{}: hand rows belong to a unit", dir.display()));
            }
            mounts.insert(0, Mount { unit: None, src });
            continue;
        };
        let Value::Text(unit) = unit else {
            return Err(format!("{}: unit is a string", dir.display()));
        };
        done(&doc.top, &dir)?;
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
        mounts.push(Mount {
            unit: Some(unit),
            src,
        });
    }
    let (js, py, cli) = facade.ok_or("no crate without a unit: the facade")?;
    mounts[1..].sort_by(|a, b| a.unit.cmp(&b.unit));
    let skip = inside(&root, &home.join(SKIP))?;
    Ok(Config {
        root,
        layout: Layout {
            manifest,
            skip,
            js,
            py,
            cli,
        },
        mounts,
        hands,
    })
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
    };
    done(row, dir)?;
    Ok(hand)
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

fn normal(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

fn inside(root: &Path, path: &Path) -> Result<PathBuf> {
    normal(path)
        .strip_prefix(normal(root))
        .map(Path::to_path_buf)
        .map_err(|_| format!("{} lies outside the workspace", path.display()))
}

pub fn climb(dir: &Path) -> usize {
    dir.components().count()
}

pub fn slashed(path: &Path) -> String {
    path.components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
