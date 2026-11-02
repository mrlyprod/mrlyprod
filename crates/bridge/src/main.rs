mod cli;
mod config;
mod disk;
mod js;
mod leaf;
mod model;
mod parse;
mod py;
mod report;
mod resolve;
#[cfg(test)]
mod tests;
mod toml;
mod version;

use config::Config;
use model::{Manifest, Result};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let done = config::load(&config::home()).and_then(|config| {
        let root = config.root.clone();
        let leaves: Vec<_> = config
            .mounts
            .iter()
            .filter_map(|m| m.leaf.as_ref())
            .map(|leaf| (leaf.unit.clone(), leaf.manifest.clone()))
            .collect();
        match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
            [] => run(config),
            ["check"] => check(config),
            ["bump"] => version::bump(&root, &leaves),
            _ => Err("usage: bridge [check|bump]".to_string()),
        }
    });
    if let Err(error) = done {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn check(config: Config) -> Result<()> {
    let root = config.root.clone();
    let root = root.as_path();
    disk::watch();
    let done = run(config);
    let mut lines: Vec<String> = disk::stale()
        .iter()
        .map(|path| path.strip_prefix(root).unwrap_or(path))
        .map(|path| format!("stale: {}", path.display()))
        .collect();
    let orphans = match done {
        Ok(()) => disk::orphans(root, &listed(root)),
        Err(_) => Vec::new(),
    };
    for path in orphans {
        let path = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        lines.push(format!("orphan: {path}"));
    }
    if !lines.is_empty() {
        lines.push("the bridge is stale: run scripts/bridge.sh".to_string());
    }
    lines.extend(done.err());
    if lines.is_empty() {
        Ok(())
    } else {
        Err(lines.join("\n"))
    }
}

fn listed(root: &Path) -> String {
    std::fs::read_to_string(root.join(disk::ATTRIBUTES)).unwrap_or_default()
}

fn run(config: Config) -> Result<()> {
    let Config {
        root,
        skip,
        mounts,
        hands,
    } = config;
    let root = root.as_path();
    model::install(hands);
    let version = version::read(root)?;
    let built = leaf::build(&mounts, &version)?;
    let mut owns: Vec<Manifest> = Vec::new();
    let mut errors = Vec::new();
    for (leaf, built) in &built {
        let own = leaf::only(&built.manifest, &leaf.unit);
        let json = serde_json::to_string_pretty(&own).map_err(|e| e.to_string())?;
        disk::save(&root.join(&leaf.manifest), &(json + "\n"))?;
        owns.push(own);
        errors.extend(built.collisions.iter().cloned());
    }
    let units: Vec<String> = mounts.iter().map(|m| m.unit.clone()).collect();
    let macros = parse::parse(&parse::load_crates(&mounts, &units)?)?.macro_body_fns;
    for line in report::skip_lines(&owns) {
        println!("{line}");
    }
    for line in report::summary(&owns, macros) {
        println!("{line}");
    }
    let skip_txt = std::fs::read_to_string(root.join(&skip)).unwrap_or_default();
    errors.sort();
    errors.dedup();
    errors.extend(report::check_skip(&owns, &skip_txt));
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    leaf::write(&built, root).map_err(|e| format!("leaf: {e}"))?;
    version::write(root, &version).map_err(|e| format!("version: {e}"))?;
    let before = listed(root);
    let orphans = disk::orphans(root, &before);
    disk::save(
        &root.join(disk::ATTRIBUTES),
        &disk::attributes(root, &before),
    )?;
    if disk::watching() {
        return Ok(());
    }
    for path in orphans {
        std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        for dir in path.ancestors().skip(1).take_while(|dir| *dir != root) {
            if std::fs::remove_dir(dir).is_err() {
                break;
            }
        }
        println!(
            "removed orphan {}",
            path.strip_prefix(root).unwrap_or(&path).display()
        );
    }
    Ok(())
}
