/// The font as glyph tables in JSON.
pub const JSON: &[u8] = include_bytes!("../assets/MrlyFont.json");

/// The font as a TrueType binary.
pub const TTF: &[u8] = include_bytes!("../assets/MrlyFont.ttf");

/// The font packed as WOFF.
pub const WOFF: &[u8] = include_bytes!("../assets/MrlyFont.woff");

/// The font packed as WOFF2.
pub const WOFF2: &[u8] = include_bytes!("../assets/MrlyFont.woff2");

/// Returns the bytes and mime type of a named format, or None when the name is not one of json, ttf, woff, woff2.
pub fn format(name: &str) -> Option<(&'static [u8], &'static str)> {
    match name {
        "json" => Some((JSON, "application/json")),
        "ttf" => Some((TTF, "font/ttf")),
        "woff" => Some((WOFF, "font/woff")),
        "woff2" => Some((WOFF2, "font/woff2")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_format_carries_bytes() {
        for name in ["json", "ttf", "woff", "woff2"] {
            let (bytes, mime) = super::format(name).expect("a known format");
            assert!(!bytes.is_empty());
            assert!(!mime.is_empty());
        }
        assert!(super::format("otf").is_none());
    }
}
