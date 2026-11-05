use crate::model::Result;
use std::path::Path;

pub fn save(path: &Path, text: &str) -> Result<()> {
    let at = |e: String| format!("{}: {e}", path.display());
    if std::fs::read_to_string(path).ok().as_deref() == Some(text) {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| at(e.to_string()))?;
    }
    std::fs::write(path, text).map_err(|e| at(e.to_string()))
}
