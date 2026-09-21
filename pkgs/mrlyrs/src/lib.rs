#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The substrate: tensors, cells, colors, images, codecs, resampling and seeded chance.
pub mod core;
/// The alphabet: the stroked pixel glyphs, their rasters and their writing animations.
pub mod font;
/// The generator: the whole pipeline from a recipe to a file.
pub mod gen;
/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub mod life;
/// The designs in space: codes, cells, cubes, hexagons, their counts, graphs and names.
pub mod math;
/// The integers: primes, divisors, series, lattices, spectra and networks.
pub mod num;

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    const MODULES: [&str; 6] = ["core", "num", "math", "gen", "life", "font"];

    fn allowed(module: &str) -> &'static [&'static str] {
        match module {
            "num" | "font" => &["core"],
            "math" => &["core", "num"],
            "gen" | "life" => &["core", "num", "math"],
            _ => &[],
        }
    }

    fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    fn live_lines(text: &str) -> Vec<(usize, &str)> {
        let mut out = Vec::new();
        let mut pending = false;
        let mut depth = 0i64;
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                continue;
            }
            let opens = line.matches('{').count() as i64;
            let closes = line.matches('}').count() as i64;
            if depth > 0 {
                depth += opens - closes;
                continue;
            }
            if pending {
                if opens > 0 {
                    pending = false;
                    depth = opens - closes;
                } else if trimmed.ends_with(';') {
                    pending = false;
                }
                continue;
            }
            if trimmed.starts_with("#[cfg(test)]") {
                pending = true;
                continue;
            }
            out.push((index + 1, line));
        }
        out
    }

    fn ident(text: &str) -> &str {
        let end = text
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(text.len());
        &text[..end]
    }

    fn group_heads(text: &str) -> (Vec<&str>, usize) {
        let mut heads = Vec::new();
        let mut depth = 0usize;
        let mut start = 1;
        for (at, c) in text.char_indices() {
            match c {
                '{' => {
                    depth += 1;
                    if depth == 1 {
                        start = at + 1;
                    }
                }
                ',' if depth == 1 => {
                    heads.push(ident(text[start..at].trim()));
                    start = at + 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        heads.push(ident(text[start..at].trim()));
                        return (heads, at + 1);
                    }
                }
                _ => {}
            }
        }
        (heads, text.len())
    }

    fn heads<'a>(line: &'a str, prefix: &str) -> Vec<&'a str> {
        let mut out = Vec::new();
        let mut rest = line;
        while let Some(at) = rest.find(prefix) {
            let after = &rest[at + prefix.len()..];
            if after.starts_with('{') {
                let (found, used) = group_heads(after);
                out.extend(found);
                rest = &after[used..];
            } else {
                let head = ident(after);
                out.push(head);
                rest = &after[head.len()..];
            }
        }
        out
    }

    #[test]
    fn the_modules_layer() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        sources(&root, &mut files);
        files.sort();
        let mut violations = Vec::new();
        for file in files {
            let relative = file.strip_prefix(&root).unwrap();
            let mut parts = relative.components();
            let Some(top) = parts.next() else { continue };
            let from = top.as_os_str().to_str().unwrap();
            if !MODULES.contains(&from) {
                continue;
            }
            let is_top_mod = relative.components().count() == 2
                && relative.file_name().is_some_and(|name| name == "mod.rs");
            let text = std::fs::read_to_string(&file).unwrap();
            for (number, line) in live_lines(&text) {
                let mut named = heads(line, "crate::");
                if is_top_mod {
                    named.extend(heads(line, "super::"));
                }
                for to in named {
                    if MODULES.contains(&to) && to != from && !allowed(from).contains(&to) {
                        violations.push(format!(
                            "{}:{} {from} -> {to}",
                            relative.display(),
                            number
                        ));
                    }
                }
            }
        }
        for violation in &violations {
            println!("{violation}");
        }
        assert!(
            violations.is_empty(),
            "{} layering violations",
            violations.len()
        );
    }
}
