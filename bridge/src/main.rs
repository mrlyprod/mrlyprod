mod cli;
mod js;
mod model;
mod parse;
mod py;
mod report;
mod resolve;
#[cfg(test)]
mod tests;

use model::{Manifest, Result};
use std::path::{Path, PathBuf};

type Backend = fn(&Manifest, &Path) -> Result<()>;

const BACKENDS: &[(&str, Backend)] = &[("cli", cli::write), ("py", py::write), ("js", js::write)];

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = root.parent().expect("bridge sits in the workspace");
    if let Err(error) = run(root) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(root: &Path) -> Result<()> {
    let src = root.join("pkgs/mrlyrs/src");
    let files = parse::load(&src)?;
    let version = version(&root.join("pkgs/mrlyrs/Cargo.toml"))?;
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

fn version(cargo_toml: &Path) -> Result<String> {
    let text = std::fs::read_to_string(cargo_toml).map_err(|e| e.to_string())?;
    text.lines()
        .find_map(|l| l.strip_prefix("version = \""))
        .and_then(|l| l.strip_suffix('"'))
        .map(str::to_string)
        .ok_or_else(|| format!("no version in {}", cargo_toml.display()))
}
