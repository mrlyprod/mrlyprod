#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Joins scalars with U+200D into one emoji sequence.
///
/// ```
/// assert_eq!(mrlymoji::zwj(&['👩', '🚀']), "👩\u{200D}🚀");
/// ```
pub fn zwj(parts: &[char]) -> String {
    let mut out = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            out.push('\u{200D}');
        }
        out.push(*part);
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn joins_the_family() {
        assert_eq!(super::zwj(&['👨', '👩', '👧']), "👨\u{200D}👩\u{200D}👧");
        assert_eq!(super::zwj(&[]), "");
    }
}
