use crate::model::Result;
use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EDITION: &str = "2021";

thread_local! {
    static STALE: RefCell<Option<Vec<PathBuf>>> = const { RefCell::new(None) };
}

pub fn watch() {
    STALE.with(|stale| *stale.borrow_mut() = Some(Vec::new()));
}

pub fn stale() -> Vec<PathBuf> {
    STALE.with(|stale| stale.borrow_mut().take().unwrap_or_default())
}

pub fn save(path: &Path, text: &str) -> Result<()> {
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
