use crate::config::Mount;
use crate::model::{Manifest, Result};
use crate::parse::Tree;
use crate::resolve::{self, Built};

pub fn closure(mounts: &[Mount], unit: &str) -> Result<Vec<String>> {
    let find = |unit: &str| {
        mounts
            .iter()
            .find(|mount| mount.unit == unit)
            .ok_or_else(|| format!("unit {unit} is not in the bridge"))
    };
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
}

pub fn manifest(tree: &Tree, seen: &[String], unit: &str) -> Result<Built> {
    resolve::build(&tree.only(seen), unit, env!("CARGO_PKG_VERSION"))
}

pub fn only(manifest: &Manifest, unit: &str) -> Manifest {
    let inside = |path: &str| path.split("::").next() == Some(unit);
    let mut out = manifest.clone();
    out.modules.retain(|m| inside(&m.path));
    out.functions.retain(|f| inside(&f.module));
    out.consts.retain(|c| inside(&c.path));
    out
}
