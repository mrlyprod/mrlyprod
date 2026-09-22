use crate::model::{Manifest, Result};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

const SOURCE: &str = "pkgs/mrlyrs/Cargo.toml";

const STAMPS: &[(&str, &str)] = &[
    ("pkgs/mrlypy/Cargo.toml", "version = \""),
    ("pkgs/mrlyjs/package.json", "\"version\": \""),
];

pub type Semver = (u64, u64, u64);

pub struct Diff {
    pub added: usize,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
}

// STAMP

pub fn read(root: &Path) -> Result<String> {
    let text = load(&root.join(SOURCE))?;
    let (start, end) =
        find(&text, "version = \"").ok_or_else(|| format!("no version in {SOURCE}"))?;
    Ok(text[start..end].to_string())
}

pub fn write(manifest: &Manifest, root: &Path) -> Result<()> {
    for (file, key) in STAMPS {
        let path = root.join(file);
        let text = load(&path)?;
        let stamped = stamp(&text, key, &manifest.version).map_err(|e| format!("{file}: {e}"))?;
        if stamped != text {
            std::fs::write(&path, stamped).map_err(|e| format!("{file}: {e}"))?;
        }
    }
    Ok(())
}

pub fn stamp(text: &str, key: &str, version: &str) -> Result<String> {
    let (start, end) = find(text, key).ok_or_else(|| format!("no line starting {key}"))?;
    Ok(format!("{}{version}{}", &text[..start], &text[end..]))
}

fn find(text: &str, key: &str) -> Option<(usize, usize)> {
    let mut at = 0;
    for line in text.split_inclusive('\n') {
        let indent = line.len() - line.trim_start().len();
        if line[indent..].starts_with(key) {
            let start = at + indent + key.len();
            return Some((start, start + text[start..].find('"')?));
        }
        at += line.len();
    }
    None
}

fn load(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

// BUMP

pub fn bump(root: &Path) -> Result<()> {
    let new = json(&load(&root.join("bridge/manifest.json"))?)?;
    let now = new["version"]
        .as_str()
        .ok_or("the manifest has no version")?;
    let current = semver(now).ok_or_else(|| format!("{now} is not X.Y.Z"))?;
    let tags = git(root, &["tag", "-l", "v*"])?;
    let previous = tags
        .lines()
        .filter_map(|tag| Some((semver(tag.strip_prefix('v')?)?, tag)))
        .filter(|(version, _)| *version != current)
        .max();
    let Some((before, tag)) = previous else {
        println!("   no other v* tag; {now} is the first release");
        return Ok(());
    };
    let old = json(&git(
        root,
        &["show", &format!("{tag}:bridge/manifest.json")],
    )?)?;
    let diff = diff(&old, &new);
    for path in &diff.removed {
        println!("   removed {path}");
    }
    for path in &diff.changed {
        println!("   changed {path}");
    }
    println!(
        "   {tag} to {now}: {} added, {} removed, {} changed",
        diff.added,
        diff.removed.len(),
        diff.changed.len()
    );
    let breaking = !diff.removed.is_empty() || !diff.changed.is_empty();
    let floor = need(before, breaking);
    if current < floor {
        let what = if breaking {
            "a breaking change"
        } else {
            "a release"
        };
        return Err(format!(
            "{now} is too small a step over {tag}: {what} needs {} at least",
            show(floor)
        ));
    }
    Ok(())
}

pub fn diff(old: &Value, new: &Value) -> Diff {
    let (old_all, old_ok) = entries(old);
    let (new_all, new_ok) = entries(new);
    let removed: BTreeSet<&String> = old_all
        .keys()
        .filter(|path| !new_all.contains_key(*path))
        .chain(old_ok.iter().filter(|path| !new_ok.contains(*path)))
        .collect();
    let changed = old_all
        .iter()
        .filter(|(path, sigs)| {
            !removed.contains(path) && new_all.get(*path).is_some_and(|new| !same(sigs, new))
        })
        .map(|(path, _)| path.clone())
        .collect();
    Diff {
        added: new_all
            .keys()
            .filter(|path| !old_all.contains_key(*path))
            .count(),
        removed: removed.into_iter().cloned().collect(),
        changed,
    }
}

fn entries(manifest: &Value) -> (BTreeMap<String, Vec<Value>>, BTreeSet<String>) {
    let mut all: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut ok = BTreeSet::new();
    for f in manifest["functions"].as_array().into_iter().flatten() {
        let status = f["cross"]["status"].as_str().unwrap_or_default();
        let path = f["path"].as_str().unwrap_or_default().to_string();
        if status == "private" {
            continue;
        }
        if status == "ok" {
            ok.insert(path.clone());
        }
        let sig = ["self_kind", "params", "ret", "dims"].map(|key| f[key].clone());
        all.entry(path).or_default().push(Value::from(sig.to_vec()));
    }
    (all, ok)
}

fn same(a: &[Value], b: &[Value]) -> bool {
    a.len() == b.len() && a.iter().all(|sig| b.contains(sig))
}

pub fn need(before: Semver, breaking: bool) -> Semver {
    let (major, minor, patch) = before;
    match (breaking, major) {
        (false, _) => (major, minor, patch + 1),
        (true, 0) => (0, minor + 1, 0),
        (true, _) => (major + 1, 0, 0),
    }
}

pub fn semver(text: &str) -> Option<Semver> {
    let mut parts = text.split('.').map(|part| part.parse::<u64>().ok());
    let version = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(version)
}

fn show((major, minor, patch): Semver) -> String {
    format!("{major}.{minor}.{patch}")
}

fn json(text: &str) -> Result<Value> {
    serde_json::from_str(text).map_err(|e| e.to_string())
}

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|e| e.to_string())
}
