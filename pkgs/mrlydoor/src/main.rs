use mrlycore::errors::{value_error, Result};
use std::fs;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn main() -> Result<()> {
    let font = read(&format!("{ROOT}/files/vendor/emoji.ttf"))?;
    let text = read(&format!("{ROOT}/files/emoji/catalog.txt"))?;
    let Ok(text) = String::from_utf8(text) else {
        return value_error("catalog is not utf-8.");
    };
    let entries = mrlydoor::catalog(&text);
    let (png, manifest) = mrlydoor::emoji_atlas(&font, &entries)?;
    write(&format!("{ROOT}/files/emoji/atlas.png"), &png)?;
    write(
        &format!("{ROOT}/files/emoji/atlas.json"),
        format!("{}\n", manifest.pretty()).as_bytes(),
    )?;
    println!(
        "{} emoji, {}x{} cells of {}px, {} png bytes",
        manifest["count"],
        manifest["cols"],
        manifest["rows"],
        manifest["cell"],
        png.len()
    );
    Ok(())
}

fn read(path: &str) -> Result<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => Ok(bytes),
        Err(e) => value_error(format!("cannot read {path}: {e}.")),
    }
}

fn write(path: &str, bytes: &[u8]) -> Result<()> {
    match fs::write(path, bytes) {
        Ok(()) => Ok(()),
        Err(e) => value_error(format!("cannot write {path}: {e}.")),
    }
}
