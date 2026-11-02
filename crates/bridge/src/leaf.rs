use crate::config::{self, Mount};
use crate::model::{Manifest, Result};
use crate::resolve::{self, Built};
use crate::{cli, js, parse, py};
use std::path::{Component, Path, PathBuf};

pub struct Leaf {
    pub unit: String,
    pub manifest: PathBuf,
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
    pub seen: Vec<String>,
    pub pys: Vec<(String, PathBuf)>,
    pub keys: Vec<PathBuf>,
    pub hand: String,
    pub base: String,
    pub hands: Hands,
}

#[derive(Default)]
pub struct Hands {
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
}

// LEAVES

pub fn leaves(mounts: &[Mount]) -> Result<Vec<Leaf>> {
    let all: Vec<&config::Leaf> = mounts.iter().filter_map(|m| m.leaf.as_ref()).collect();
    let find = |unit: &str| {
        all.iter()
            .copied()
            .find(|leaf| leaf.unit == unit)
            .ok_or_else(|| format!("unit {unit} has no js, py and cli in its bridge.toml"))
    };
    let mut seen_of = Vec::new();
    for leaf in &all {
        let mut seen = vec![leaf.unit.clone()];
        let mut at = 0;
        while at < seen.len() {
            for unit in &find(&seen[at])?.uses {
                if !seen.contains(unit) {
                    seen.push(unit.clone());
                }
            }
            at += 1;
        }
        seen.sort();
        seen_of.push((leaf.unit.clone(), seen));
    }
    let closure = |unit: &str| {
        seen_of
            .iter()
            .find(|(u, _)| u == unit)
            .map_or(&[][..], |(_, seen)| seen.as_slice())
    };
    let owners: Vec<&str> = all
        .iter()
        .filter(|leaf| leaf.hand)
        .map(|leaf| leaf.unit.as_str())
        .collect();
    let base = owners
        .iter()
        .copied()
        .min_by_key(|unit| (closure(unit).len(), *unit))
        .ok_or("no unit holds [[hand]] rows")?;
    let mut out = Vec::new();
    for leaf in &all {
        let seen = closure(&leaf.unit).to_vec();
        let hand = owners
            .iter()
            .copied()
            .filter(|unit| seen.iter().any(|s| s == unit))
            .max_by_key(|unit| (closure(unit).len(), *unit))
            .unwrap_or(base);
        let mut pys = Vec::new();
        let mut keys = Vec::new();
        for unit in &seen {
            let other = find(unit)?;
            keys.push(other.src.clone());
            if *unit != leaf.unit {
                pys.push((unit.clone(), other.py.clone()));
            }
        }
        keys.push(find(base)?.py.join("hand"));
        if hand != base {
            keys.push(find(hand)?.py.join("hand"));
        }
        out.push(Leaf {
            pys,
            keys,
            unit: leaf.unit.clone(),
            manifest: leaf.manifest.clone(),
            js: leaf.js.clone(),
            py: leaf.py.clone(),
            cli: leaf.cli.clone(),
            hands: Hands {
                js: find(hand)?.js.join("hand"),
                py: find(hand)?.py.join("hand"),
                cli: find(base)?.cli.join("hand"),
            },
            hand: hand.to_string(),
            base: base.to_string(),
            seen,
        });
    }
    Ok(out)
}

// MANIFEST

pub fn manifest(mounts: &[Mount], leaf: &Leaf, version: &str) -> Result<Built> {
    let files = parse::load_crates(mounts, &leaf.seen)?;
    resolve::build(&files, &format!("mrly-{}", leaf.unit), version)
}

pub fn build(mounts: &[Mount], version: &str) -> Result<Vec<(Leaf, Built)>> {
    leaves(mounts)?
        .into_iter()
        .map(|leaf| {
            let built =
                manifest(mounts, &leaf, version).map_err(|e| format!("{}: {e}", leaf.unit))?;
            Ok((leaf, built))
        })
        .collect()
}

pub fn only(manifest: &Manifest, unit: &str) -> Manifest {
    let inside = |path: &str| path.split("::").next() == Some(unit);
    let mut out = manifest.clone();
    out.modules.retain(|m| inside(&m.path));
    out.functions.retain(|f| inside(&f.module));
    out.consts.retain(|c| inside(&c.path));
    out
}

// PATHS

pub fn deps(leaf: &Leaf) -> String {
    leaf.seen
        .iter()
        .map(|unit| format!("mrly-{unit}.workspace = true\n"))
        .collect()
}

pub fn relative(from: &Path, to: &Path) -> String {
    let from: Vec<Component> = from.components().collect();
    let to: Vec<Component> = to.components().collect();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = vec!["..".to_string(); from.len() - common];
    parts.extend(
        to[common..]
            .iter()
            .map(|part| part.as_os_str().to_string_lossy().into_owned()),
    );
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

// WRITE

pub fn write(built: &[(Leaf, Built)], root: &Path) -> Result<()> {
    for (leaf, built) in built {
        let manifest = &built.manifest;
        let at = |e: String| format!("{}: {e}", leaf.unit);
        cli::write_leaf(manifest, root, leaf).map_err(at)?;
        py::write_leaf(manifest, root, leaf).map_err(at)?;
        js::write_leaf(manifest, root, leaf).map_err(at)?;
    }
    Ok(())
}
