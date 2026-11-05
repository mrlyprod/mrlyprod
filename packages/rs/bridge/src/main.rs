mod config;
mod disk;
mod doors;
mod js;
mod leaf;
mod model;
mod parse;
mod report;
mod resolve;
mod toml;

use config::{Config, Mount};
use model::{Manifest, Result, TypeKind};
use std::collections::BTreeSet;
use std::path::Path;

fn main() {
    if let Err(error) = config::load().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    let Config {
        root,
        out,
        mounts,
        hands,
    } = config;
    model::install(hands);
    let tree = parse::parse(&parse::load_crates(&mounts)?)?;
    let skip_txt = include_str!("skip.txt");
    let mut owns = Vec::new();
    let mut units = Vec::new();
    for mount in &mounts {
        let seen = leaf::closure(&mounts, &mount.unit)?;
        let built = leaf::manifest(&tree, &seen, &mount.unit)?;
        let own = leaf::only(&built.manifest, &mount.unit);
        if !built.collisions.is_empty() {
            let mut errors = built.collisions.clone();
            errors.sort();
            errors.dedup();
            return Err(errors.join("\n"));
        }
        let shapes = shapes(&built.manifest);
        let table = doors::table(&built.manifest, &own, &mount.unit, &shapes)?;
        for (name, reason) in &table.skipped {
            println!("{name}: {reason}");
        }
        write_unit(&out, &root, mount, &seen, &table)?;
        js::write(&out, &built.manifest, &mount.unit, &table, &shapes)?;
        let json = serde_json::to_string_pretty(&own).map_err(|e| e.to_string())?;
        disk::save(
            &out.join("manifest").join(format!("{}.json", mount.unit)),
            &json,
        )?;
        owns.push(own);
        units.push(mount.unit.clone());
    }
    let errors = report::check_skip(&owns, skip_txt);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    for line in report::summary(&owns, tree.macro_body_fns) {
        println!("{line}");
    }
    write_workspace(&out, &root, &units)
}

fn shapes(manifest: &Manifest) -> Vec<Vec<String>> {
    let mut out: BTreeSet<Vec<String>> = BTreeSet::new();
    for form in [
        ["shape", "data"].as_slice(),
        ["shape", "colors"].as_slice(),
        ["shape", "types"].as_slice(),
        ["shape", "types", "colors"].as_slice(),
        ["shape", "types", "tags"].as_slice(),
        ["shape", "types", "colors", "tags"].as_slice(),
    ] {
        out.insert(form.iter().map(|one| one.to_string()).collect());
    }
    let list = |fields: &[model::Field], t: &model::Type| -> Option<Vec<String>> {
        let names: Vec<String> = fields
            .iter()
            .filter(|field| !field.serde_skip)
            .map(|field| js::field_word(t, field))
            .collect();
        (!names.is_empty()
            && names
                .iter()
                .all(|one| !one.starts_with(|c: char| c.is_ascii_digit())))
        .then_some(names)
    };
    for t in &manifest.types {
        if t.kind == TypeKind::Alias || !t.derives.iter().any(|d| d == "Serialize") {
            continue;
        }
        if let Some(names) = list(&t.fields, t) {
            out.insert(names);
        }
        for variant in &t.variants {
            if let Some(names) = list(&variant.fields, t) {
                out.insert(names);
            }
        }
    }
    out.into_iter().collect()
}

fn write_unit(
    out: &Path,
    root: &Path,
    mount: &Mount,
    seen: &[String],
    table: &doors::Table,
) -> Result<()> {
    let unit = &mount.unit;
    let dir = out.join("units").join(unit);
    let up = "../../../../../../";
    let _ = root;
    let mut deps = format!("mrly-wire = {{ path = \"{up}packages/rs/wire\" }}\n");
    let mut all: Vec<String> = seen.to_vec();
    if !all.iter().any(|one| one == "core") {
        all.push("core".to_string());
    }
    all.sort();
    for one in &all {
        deps.push_str(&format!(
            "mrly-{one} = {{ path = \"{up}packages/rs/{one}\" }}\n"
        ));
    }
    disk::save(
        &dir.join("Cargo.toml"),
        &format!(
            "[package]\nname = \"mrly-{unit}-doors\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[lib]\nname = \"mrly_{unit}_doors\"\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[features]\ndefault = [\"abi\"]\nabi = []\n\n[dependencies]\n{deps}"
        ),
    )?;
    disk::save(&dir.join("src/lib.rs"), &table.rust)
}

fn write_workspace(out: &Path, root: &Path, units: &[String]) -> Result<()> {
    let members: Vec<String> = units
        .iter()
        .map(|unit| format!("\"units/{unit}\""))
        .chain(std::iter::once("\"../../../../packages/cli\"".to_string()))
        .collect();
    disk::save(
        &out.join("Cargo.toml"),
        &format!(
            "[workspace]\nresolver = \"3\"\nmembers = [{}]\n\n[profile.release]\nlto = true\ncodegen-units = 1\nstrip = \"none\"\n\n[profile.wasm]\ninherits = \"release\"\nopt-level = \"s\"\nstrip = true\n",
            members.join(", ")
        ),
    )?;
    let lock = out.join("Cargo.lock");
    if !lock.exists() {
        std::fs::copy(root.join("Cargo.lock"), &lock).map_err(|e| e.to_string())?;
    }
    Ok(())
}
