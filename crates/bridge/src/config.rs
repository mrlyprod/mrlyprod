use crate::model::{Hand, Result};
use crate::toml::{self, Table, Value};
use std::path::{Component, Path, PathBuf};

pub const FILE: &str = "bridge.toml";

const SKIP: &str = "skip.txt";

const MANIFEST: &str = "manifest.json";

pub struct Config {
    pub root: PathBuf,
    pub skip: PathBuf,
    pub mounts: Vec<Mount>,
    pub hands: Vec<Hand>,
}

pub struct Mount {
    pub unit: String,
    pub src: PathBuf,
    pub leaf: Option<Leaf>,
}

pub struct Leaf {
    pub unit: String,
    pub src: PathBuf,
    pub manifest: PathBuf,
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
    pub uses: Vec<String>,
    pub hand: bool,
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
    done(&own.top, home)?;
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&crates)
        .map_err(|e| format!("{}: {e}", crates.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|dir| dir.join(FILE).is_file() && normal(dir) != normal(home))
        .collect();
    dirs.sort();
    let mut mounts = Vec::new();
    let mut hands = Vec::new();
    for dir in dirs {
        let mut doc = read(&dir)?;
        let src = dir.join("src");
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
                    inside(&root, &dir.join(text(&mut doc.top, key, &dir)?))
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
        let leaf = match outs {
            Some((js, py, cli)) => Some(Leaf {
                uses: uses(&dir)?,
                src: inside(&root, &src)?,
                manifest: inside(&root, &dir.join(MANIFEST))?,
                unit: unit.clone(),
                js,
                py,
                cli,
                hand: owns,
            }),
            None => None,
        };
        mounts.push(Mount { unit, src, leaf });
    }
    mounts.sort_by(|a, b| a.unit.cmp(&b.unit));
    let skip = inside(&root, &home.join(SKIP))?;
    Ok(Config {
        root,
        skip,
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

pub fn slashed(path: &Path) -> String {
    path.components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
