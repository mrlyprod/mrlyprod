use crate::config::{self, Mount};
use crate::model::{Manifest, Result};
use crate::parse::Tree;
use crate::resolve::{self, Built};
use crate::{cli, js, py};
use std::path::{Path, PathBuf};

pub struct Leaf {
    pub unit: String,
    pub manifest: PathBuf,
    pub js: PathBuf,
    pub py: PathBuf,
    pub cli: PathBuf,
    pub seen: Vec<String>,
    pub pys: Vec<String>,
    pub hand: String,
    pub base: String,
}

pub fn own(mounts: &[Mount], own: &config::Leaf) -> Result<Leaf> {
    let find = |unit: &str| {
        mounts
            .iter()
            .find(|mount| mount.unit == unit)
            .ok_or_else(|| format!("unit {unit} is not a dependency of this repo"))
    };
    let closure = |unit: &str| -> Result<Vec<String>> {
        let mut seen = vec![unit.to_string()];
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
        Ok(seen)
    };
    let mut owners = Vec::new();
    for mount in mounts.iter().filter(|mount| mount.hand && mount.leaf) {
        owners.push((closure(&mount.unit)?.len(), mount.unit.clone()));
    }
    let base = owners
        .iter()
        .min()
        .map(|(_, unit)| unit.clone())
        .ok_or("no unit holds [[hand]] rows")?;
    let seen = closure(&own.unit)?;
    for unit in &seen {
        if !find(unit)?.leaf {
            return Err(format!(
                "unit {unit} has no js, py and cli in its bridge.toml"
            ));
        }
    }
    let hand = owners
        .iter()
        .filter(|(_, unit)| seen.contains(unit))
        .max()
        .map_or(base.clone(), |(_, unit)| unit.clone());
    let pys = seen
        .iter()
        .filter(|unit| **unit != own.unit)
        .cloned()
        .collect();
    Ok(Leaf {
        unit: own.unit.clone(),
        manifest: own.manifest.clone(),
        js: own.js.clone(),
        py: own.py.clone(),
        cli: own.cli.clone(),
        seen,
        pys,
        hand,
        base,
    })
}

pub fn manifest(tree: &Tree, leaf: &Leaf, version: &str) -> Result<Built> {
    resolve::build(
        &tree.only(&leaf.seen),
        &format!("mrly-{}", leaf.unit),
        version,
    )
}

pub fn only(manifest: &Manifest, unit: &str) -> Manifest {
    let inside = |path: &str| path.split("::").next() == Some(unit);
    let mut out = manifest.clone();
    out.modules.retain(|m| inside(&m.path));
    out.functions.retain(|f| inside(&f.module));
    out.consts.retain(|c| inside(&c.path));
    out
}

pub fn deps(leaf: &Leaf, owner: &str, feature: &str) -> String {
    let mut units = leaf.seen.clone();
    if !units.iter().any(|unit| unit == owner) {
        units.push(owner.to_string());
        units.sort();
    }
    units
        .iter()
        .map(|unit| {
            if unit == owner {
                format!("mrly-{unit} = {{ workspace = true, features = [\"{feature}\"] }}\n")
            } else {
                format!("mrly-{unit}.workspace = true\n")
            }
        })
        .collect()
}

pub fn write(leaf: &Leaf, built: &Built, root: &Path) -> Result<()> {
    let manifest = &built.manifest;
    let at = |e: String| format!("{}: {e}", leaf.unit);
    cli::write_leaf(manifest, root, leaf).map_err(at)?;
    py::write_leaf(manifest, root, leaf).map_err(at)?;
    js::write_leaf(manifest, root, leaf).map_err(at)
}
