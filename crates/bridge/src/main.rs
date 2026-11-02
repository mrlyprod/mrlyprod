mod cli;
mod config;
mod disk;
mod js;
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

type Backend = fn(&Manifest, &Path, &config::Layout) -> Result<()>;

const BACKENDS: &[(&str, Backend)] = &[
    ("cli", cli::write),
    ("py", py::write),
    ("js", js::write),
    ("version", version::write),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let done = config::load(&config::home()).and_then(|config| {
        let (root, manifest) = (config.root.clone(), config.layout.manifest.clone());
        match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
            [] => run(config),
            ["check"] => check(config),
            ["bump"] => version::bump(&root, &manifest),
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
        layout,
        mounts,
        hands,
    } = config;
    let root = root.as_path();
    model::install(hands);
    let files = parse::load_crates(&mounts)?;
    let version = version::read(root)?;
    let built = resolve::build(&files, &version)?;
    let json = serde_json::to_string_pretty(&built.manifest).map_err(|e| e.to_string())?;
    disk::save(&root.join(&layout.manifest), &(json + "\n"))?;
    for line in report::skip_lines(&built.manifest) {
        println!("{line}");
    }
    for line in report::summary(&built.manifest, built.macro_body_fns) {
        println!("{line}");
    }
    let skip_txt = std::fs::read_to_string(root.join(&layout.skip)).unwrap_or_default();
    let mut errors = built.collisions;
    errors.extend(report::check_skip(&built.manifest, &skip_txt));
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    for (name, write) in BACKENDS {
        write(&built.manifest, root, &layout).map_err(|e| format!("{name}: {e}"))?;
    }
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
