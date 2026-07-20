use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

fn walk(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            walk(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-env-changed=MRLY_RELEASE");
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let full = if std::env::var("MRLY_RELEASE").is_ok() {
        version
    } else {
        let mut files = Vec::new();
        walk(Path::new("src"), &mut files);
        let mut hasher = DefaultHasher::new();
        for path in &files {
            path.to_string_lossy().hash(&mut hasher);
            std::fs::read(path).unwrap_or_default().hash(&mut hasher);
        }
        format!("{version}+{:08x}", hasher.finish() as u32)
    };
    println!("cargo:rustc-env=MRLY_VERSION={full}");
}
