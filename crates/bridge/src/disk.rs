use crate::model::Result;
use std::cell::RefCell;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

const EDITION: &str = "2024";

thread_local! {
    static STALE: RefCell<Option<Vec<PathBuf>>> = const { RefCell::new(None) };
    static MADE: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) };
}

pub const ATTRIBUTES: &str = ".gitattributes";

const MARK: &str = " linguist-generated=true";

pub fn attributes(root: &Path, before: &str) -> String {
    let mut paths: Vec<String> = MADE.with(|made| {
        made.borrow()
            .iter()
            .filter_map(|path| path.strip_prefix(root).ok())
            .map(|path| path.display().to_string())
            .collect()
    });
    paths.sort();
    paths.dedup();
    let kept = before
        .lines()
        .filter(|line| !line.ends_with(MARK))
        .map(|line| format!("{line}\n"));
    kept.chain(paths.iter().map(|path| format!("{path}{MARK}\n")))
        .collect()
}

pub fn orphans(root: &Path, before: &str) -> Vec<PathBuf> {
    let made = MADE.with(|made| made.borrow().clone());
    let mut out: Vec<PathBuf> = before
        .lines()
        .filter_map(|line| line.strip_suffix(MARK))
        .filter(|path| {
            let plain = !path.contains(['*', '?', '[', '\\', ' ']);
            plain
                && Path::new(path)
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
        })
        .map(|path| root.join(path))
        .filter(|path| !made.contains(path) && path.is_file())
        .collect();
    out.sort();
    out.dedup();
    out
}

pub fn watching() -> bool {
    STALE.with(|stale| stale.borrow().is_some())
}

pub fn watch() {
    STALE.with(|stale| *stale.borrow_mut() = Some(Vec::new()));
}

pub fn stale() -> Vec<PathBuf> {
    STALE.with(|stale| stale.borrow_mut().take().unwrap_or_default())
}

pub fn save(path: &Path, text: &str) -> Result<()> {
    MADE.with(|made| made.borrow_mut().push(path.to_path_buf()));
    edit(path, text)
}

pub fn edit(path: &Path, text: &str) -> Result<()> {
    let at = |e: String| format!("{}: {e}", path.display());
    let text = if path.extension().is_some_and(|e| e == "rs") {
        rustfmt(text).map_err(at)?
    } else {
        text.to_string()
    };
    if std::fs::read_to_string(path).ok().as_deref() == Some(text.as_str()) {
        return Ok(());
    }
    let watched = STALE.with(|stale| {
        let mut stale = stale.borrow_mut();
        if let Some(list) = stale.as_mut() {
            list.push(path.to_path_buf());
        }
        stale.is_some()
    });
    if watched {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| at(e.to_string()))?;
    }
    std::fs::write(path, text).map_err(|e| at(e.to_string()))
}

pub fn rustfmt(text: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", EDITION])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("rustfmt: {e}"))?;
    let mut stdin = child.stdin.take().ok_or("rustfmt: no stdin")?;
    let out = std::thread::scope(|scope| {
        scope.spawn(move || stdin.write_all(text.as_bytes()));
        child.wait_with_output()
    })
    .map_err(|e| format!("rustfmt: {e}"))?;
    if !out.status.success() {
        let said = String::from_utf8_lossy(&out.stderr);
        return Err(format!("rustfmt: {}", said.trim()));
    }
    String::from_utf8(out.stdout).map_err(|e| format!("rustfmt: {e}"))
}
