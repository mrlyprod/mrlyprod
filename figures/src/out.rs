use crate::board::Board;
use crate::ink;
use mrlyrs::core::error::{value_error, Result};
use std::path::PathBuf;

// OUTPUT

/// Returns the workspace root, one level above this crate.
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Writes the board to files/figures/<name>-<theme>.png, dark or light, and announces the one line it printed.
pub fn save(name: &str, board: &Board) -> Result<PathBuf> {
    let name = format!("{name}-{}", ink::name());
    let folder = root().join("files").join("figures");
    std::fs::create_dir_all(&folder)
        .map_err(|e| mrlyrs::Error::Value(format!("cannot make {folder:?}: {e}")))?;
    let path = folder.join(format!("{name}.png"));
    let bytes = board.png()?;
    if bytes.is_empty() {
        return value_error("the png came back empty.");
    }
    std::fs::write(&path, bytes)
        .map_err(|e| mrlyrs::Error::Value(format!("cannot write {path:?}: {e}")))?;
    println!("figure {name} {}x{}", board.width, board.height);
    Ok(path)
}
