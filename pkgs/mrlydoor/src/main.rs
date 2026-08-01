use mrlycore::errors::{value_error, Result};
use std::fs;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn main() -> Result<()> {
    emoji()?;
    symbols()?;
    Ok(())
}

fn emoji() -> Result<()> {
    let font = read(&format!("{ROOT}/files/vendor/emoji.ttf"))?;
    let entries = mrlydoor::catalog(&text(&format!("{ROOT}/files/emoji/catalog.txt"))?);
    let (png, manifest) = mrlydoor::emoji_atlas(&font, &entries)?;
    ship("emoji", &png, &manifest)
}

fn symbols() -> Result<()> {
    let material = read(&format!("{ROOT}/files/vendor/symbols.ttf"))?;
    let codepoints = text(&format!("{ROOT}/files/vendor/symbols.codepoints"))?;
    let symbols2 = read(&format!("{ROOT}/files/vendor/symbols2.ttf"))?;
    let entries = mrlydoor::catalog(&text(&format!("{ROOT}/files/symbols/catalog.txt"))?);
    let (png, manifest) = mrlydoor::symbol_atlas(&material, &codepoints, &symbols2, &entries)?;
    ship("symbols", &png, &manifest)
}

fn ship(kind: &str, png: &[u8], manifest: &mrlycore::Json) -> Result<()> {
    write(&format!("{ROOT}/files/{kind}/atlas.png"), png)?;
    write(
        &format!("{ROOT}/files/{kind}/atlas.json"),
        format!("{}\n", manifest.pretty()).as_bytes(),
    )?;
    println!(
        "{kind}: {} glyphs, {}x{} cells of {}px, {} png bytes",
        manifest["count"],
        manifest["cols"],
        manifest["rows"],
        manifest["cell"],
        png.len()
    );
    Ok(())
}

fn text(path: &str) -> Result<String> {
    match String::from_utf8(read(path)?) {
        Ok(text) => Ok(text),
        Err(_) => value_error(format!("{path} is not utf-8.")),
    }
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
