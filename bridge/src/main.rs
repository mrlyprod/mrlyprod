mod cli;
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
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = root.parent().expect("bridge sits in the workspace");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let done = match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => run(root),
        ["bump"] => version::bump(root),
        _ => Err("usage: bridge [bump]".to_string()),
    };
    if let Err(error) = done {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(root: &Path) -> Result<()> {
    let src = root.join("pkgs/mrlyrs/src");
    let files = parse::load(&src)?;
    let version = version::read(root)?;
    let built = resolve::build(&files, &version)?;
    let json = serde_json::to_string_pretty(&built.manifest).map_err(|e| e.to_string())?;
    std::fs::write(root.join("bridge/manifest.json"), json + "\n").map_err(|e| e.to_string())?;
    for line in report::skip_lines(&built.manifest) {
        println!("{line}");
    }
    for line in report::summary(&built.manifest, built.macro_body_fns) {
        println!("{line}");
    }
    let skip_txt = std::fs::read_to_string(root.join("bridge/skip.txt")).unwrap_or_default();
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
