use crate::model::Result;
use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EDITION: &str = "2024";

thread_local! {
    static STALE: RefCell<Option<Vec<PathBuf>>> = const { RefCell::new(None) };
    static SAVED: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) };
}

pub fn watching() -> bool {
    STALE.with(|stale| stale.borrow().is_some())
}

pub fn orphans(dir: &Path) -> Vec<PathBuf> {
    let saved = SAVED.with(|saved| saved.borrow().clone());
    let mut out = vec![];
    walk(dir, &mut out);
    out.retain(|path| !saved.contains(path));
    out.sort();
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for path in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if name == "__pycache__" || name.ends_with(".so") || name == ".DS_Store" {
            continue;
        }
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

pub fn watch() {
    STALE.with(|stale| *stale.borrow_mut() = Some(Vec::new()));
}

pub fn stale() -> Vec<PathBuf> {
    STALE.with(|stale| stale.borrow_mut().take().unwrap_or_default())
}

pub fn save(path: &Path, text: &str) -> Result<()> {
    SAVED.with(|saved| saved.borrow_mut().push(path.to_path_buf()));
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
