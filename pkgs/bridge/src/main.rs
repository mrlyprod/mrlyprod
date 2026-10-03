mod cli;
mod disk;
mod js;
mod model;
mod parse;
mod py;
mod report;
mod resolve;
#[cfg(test)]
mod tests;
mod version;

use model::{Manifest, Result};
use std::path::{Path, PathBuf};

type Backend = fn(&Manifest, &Path) -> Result<()>;

const BACKENDS: &[(&str, Backend)] = &[
    ("cli", cli::write),
    ("py", py::write),
    ("js", js::write),
    ("version", version::write),
];

fn main() {
    let home = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = home.ancestors().nth(2).expect("bridge sits in pkgs");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let done = match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => run(root),
        ["check"] => check(root),
        ["bump"] => version::bump(root),
        _ => Err("usage: bridge [check|bump]".to_string()),
    };
    if let Err(error) = done {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn check(root: &Path) -> Result<()> {
    disk::watch();
    let done = run(root);
    let mut lines: Vec<String> = disk::stale()
        .iter()
        .map(|path| path.strip_prefix(root).unwrap_or(path))
        .map(|path| format!("stale: {}", path.display()))
        .collect();
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

fn run(root: &Path) -> Result<()> {
    let src = root.join("pkgs/mrlyrs/src");
    let files = parse::load(&src)?;
    let version = version::read(root)?;
    let built = resolve::build(&files, &version)?;
    let json = serde_json::to_string_pretty(&built.manifest).map_err(|e| e.to_string())?;
    disk::save(&root.join(version::MANIFEST), &(json + "\n"))?;
    for line in report::skip_lines(&built.manifest) {
        println!("{line}");
    }
    for line in report::summary(&built.manifest, built.macro_body_fns) {
        println!("{line}");
    }
    let skip_txt = std::fs::read_to_string(root.join("pkgs/bridge/skip.txt")).unwrap_or_default();
    let mut errors = built.collisions;
    errors.extend(report::check_skip(&built.manifest, &skip_txt));
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    for (name, write) in BACKENDS {
        write(&built.manifest, root).map_err(|e| format!("{name}: {e}"))?;
    }
    Ok(())
}
