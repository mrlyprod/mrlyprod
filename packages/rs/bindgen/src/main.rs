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

use config::{ALL, Config, Mount};
use model::{Manifest, Result, TypeKind};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn main() {
    if let Err(error) = config::load().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

struct Made {
    unit: String,
    table: doors::Table,
    shapes: BTreeSet<Vec<String>>,
    needs: BTreeSet<String>,
    seen: Vec<String>,
}

fn run(config: Config) -> Result<()> {
    let Config {
        root,
        out,
        mounts,
        hands,
        json,
        blobs,
    } = config;
    model::install(hands, json);
    let tree = parse::parse(&parse::load_crates(&mounts)?)?;
    let _ = std::fs::remove_dir_all(out.join("js"));
    let _ = std::fs::remove_dir_all(out.join("api"));
    let _ = std::fs::remove_dir_all(out.join("manifest"));
    let _ = std::fs::remove_dir_all(out.join("units"));
    let _ = std::fs::remove_dir_all(out.join("blobs"));
    let mut owns = Vec::new();
    let mut made: Vec<Made> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
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
        let table = doors::table(&built.manifest, &own, &mount.unit)?;
        skipped.extend(
            table
                .skipped
                .iter()
                .map(|(name, reason)| format!("{name} {reason}")),
        );
        write_unit(&out, mount, &table)?;
        js::write(&out, &built.manifest, &mount.unit, &table)?;
        let json = mrly_json::pretty(&own).map_err(|e| e.to_string())?;
        disk::save(
            &out.join("manifest").join(format!("{}.json", mount.unit)),
            &json,
        )?;
        made.push(Made {
            unit: mount.unit.clone(),
            needs: js::classes_used(&built.manifest, &mount.unit),
            shapes: shapes(&built.manifest),
            table,
            seen,
        });
        owns.push(own);
    }
    let homes = homes(&blobs);
    for one in &made {
        let home = homes.get(&one.unit).map_or(ALL, String::as_str);
        for need in &one.needs {
            let other = homes.get(need).map_or(ALL, String::as_str);
            if home != other {
                return Err(format!(
                    "site.json wasm: a door of {} holds a class of {need}, so both share a blob; {} is in {home}, {need} is in {other}",
                    one.unit, one.unit
                ));
            }
        }
    }
    let mut report_blobs = mrly_json::Map::new();
    for blob in &blobs {
        let members: Vec<&Made> = made
            .iter()
            .filter(|one| blob.crates.contains(&one.unit))
            .collect();
        let shapes: Vec<Vec<String>> = members
            .iter()
            .flat_map(|one| one.shapes.iter().cloned())
            .chain(forms())
            .collect::<BTreeSet<Vec<String>>>()
            .into_iter()
            .collect();
        let tables: Vec<(&str, &doors::Table)> = members
            .iter()
            .map(|one| (one.unit.as_str(), &one.table))
            .collect();
        let text = doors::blob(&tables, &shapes)?;
        write_blob(&out, &blob.name, &members, &text)?;
        let names: Vec<&str> = members.iter().map(|one| one.unit.as_str()).collect();
        js::blob(&out, &blob.name, &names, &text, &shapes)?;
        report_blobs.insert(
            blob.name.clone(),
            mrly_json::json!({ "at": text.at, "rng": text.rng }),
        );
    }
    let units: Vec<String> = made.iter().map(|one| one.unit.clone()).collect();
    js::units(&out, &units)?;
    let crates: mrly_json::Map = made
        .iter()
        .map(|one| {
            let home = homes.get(&one.unit).map_or(ALL, String::as_str);
            (
                one.unit.clone(),
                mrly_json::json!({ "home": home, "needs": one.needs }),
            )
        })
        .collect();
    disk::save(
        &out.join("js/blobs/index.json"),
        &mrly_json::pretty(&mrly_json::json!({ "crates": crates })).map_err(|e| e.to_string())?,
    )?;
    disk::save(
        &out.join("api/blobs.json"),
        &mrly_json::Json::Object(report_blobs).to_string(),
    )?;
    let mut lines: Vec<String> = report::skips(&owns).lines().map(String::from).collect();
    lines.extend(skipped);
    lines.sort();
    lines.dedup();
    disk::save(&out.join("skipped.txt"), &(lines.join("\n") + "\n"))?;
    if let Some(line) = report::summary(&owns, tree.macro_body_fns).last() {
        println!("{line}");
    }
    println!(
        "{} cannot cross, listed in skipped.txt; blobs: {}",
        lines.len(),
        blobs
            .iter()
            .map(|blob| blob.name.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );
    write_workspace(&out, &root, &blobs)
}

fn homes(blobs: &[config::Blob]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for blob in blobs.iter().filter(|blob| blob.name != ALL) {
        for one in &blob.crates {
            out.insert(one.clone(), blob.name.clone());
        }
    }
    out
}

fn forms() -> Vec<Vec<String>> {
    [
        ["shape", "data"].as_slice(),
        ["shape", "colors"].as_slice(),
        ["shape", "types"].as_slice(),
        ["shape", "types", "colors"].as_slice(),
        ["shape", "types", "tags"].as_slice(),
        ["shape", "types", "colors", "tags"].as_slice(),
    ]
    .iter()
    .map(|form| form.iter().map(|one| one.to_string()).collect())
    .collect()
}

fn shapes(manifest: &Manifest) -> BTreeSet<Vec<String>> {
    let mut out: BTreeSet<Vec<String>> = BTreeSet::new();
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
    out
}

const UP: &str = "../../../../../../";

fn write_unit(out: &Path, mount: &Mount, table: &doors::Table) -> Result<()> {
    disk::save(
        &out.join("units").join(format!("{}.rs", mount.unit)),
        &table.rust,
    )
}

fn write_blob(out: &Path, name: &str, members: &[&Made], text: &doors::BlobText) -> Result<()> {
    let dir = out.join("blobs").join(name);
    let rand = model::hands()
        .iter()
        .find(|hand| hand.stream)
        .and_then(|hand| hand.rust.first())
        .and_then(|path| path.split("::").next())
        .and_then(|head| head.strip_prefix("mrly_"))
        .ok_or("no [[hand]] row is a stream")?;
    let mut crates: BTreeSet<&str> = BTreeSet::from([rand, "wire"]);
    for one in members {
        crates.extend(one.seen.iter().map(String::as_str));
    }
    let mut deps = String::new();
    for one in crates {
        deps.push_str(&format!(
            "mrly-{one} = {{ path = \"{UP}packages/rs/{one}\" }}\n"
        ));
    }
    disk::save(
        &dir.join("Cargo.toml"),
        &format!(
            "[package]\nname = \"mrly-blob-{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[lib]\nname = \"mrly_blob_{name}\"\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[features]\ndefault = [\"abi\"]\nabi = []\n\n[dependencies]\n{deps}"
        ),
    )?;
    disk::save(&dir.join("src/lib.rs"), &text.rust)
}

fn write_workspace(out: &Path, root: &Path, blobs: &[config::Blob]) -> Result<()> {
    let members: Vec<String> = blobs
        .iter()
        .map(|blob| format!("\"blobs/{}\"", blob.name))
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
