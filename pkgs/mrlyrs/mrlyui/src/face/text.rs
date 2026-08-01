use mrlyfont::{glyph, trim};

pub(crate) const LINE: usize = 7;

const BOX: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
];

fn block(c: char) -> Vec<Vec<u8>> {
    let c = if c.is_control() { ' ' } else { c };
    if c == ' ' {
        return vec![vec![0u8; 3]; LINE];
    }
    let body: Vec<Vec<u8>> = match glyph(c) {
        Some(g) => trim(&g.rows)
            .iter()
            .map(|row| row.chars().map(|ch| (ch == '1') as u8).collect())
            .collect(),
        None => BOX.iter().map(|row| row.to_vec()).collect(),
    };
    let w = body.first().map(Vec::len).unwrap_or(1);
    let rows = body.len().min(LINE);
    let off = (LINE - rows) / 2;
    let mut out = vec![vec![0u8; w]; LINE];
    for (r, row) in body.iter().take(rows).enumerate() {
        out[off + r] = row.clone();
    }
    out
}

fn advance(c: char) -> usize {
    block(c)[0].len() + 1
}

pub(crate) fn width(text: &str, scale: usize) -> usize {
    let total: usize = text.chars().map(advance).sum();
    total.saturating_sub(1) * scale
}

pub(crate) fn truncate(text: &str, field: usize, scale: usize) -> String {
    if width(text, scale) <= field {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let dots: usize = "..".chars().map(advance).sum();
    let mut acc = 0;
    let mut cut = 0;
    for (i, &c) in chars.iter().enumerate() {
        let next = acc + advance(c);
        if (next + dots).saturating_sub(1) * scale > field {
            break;
        }
        acc = next;
        cut = i + 1;
    }
    if cut == 0 {
        return String::new();
    }
    let mut out: String = chars[..cut].iter().collect();
    out.push_str("..");
    out
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    text: &str,
    x: usize,
    y: usize,
    scale: usize,
    color: [u8; 4],
) {
    let mut cx = x;
    for c in text.chars() {
        let rows = block(c);
        crate::draw::blit(buf, w, h, &rows, cx, y, scale, color);
        cx += (rows[0].len() + 1) * scale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_matches_the_advance_model() {
        assert_eq!(width("", 1), 0);
        let a = width("a", 1);
        let b = width("b", 1);
        assert_eq!(width("ab", 1), a + b + 1);
        assert_eq!(width("ab", 2), (a + b + 1) * 2);
    }

    #[test]
    fn unsupported_chars_get_a_visible_box() {
        let rows = block('\u{1f600}');
        assert_eq!(rows.len(), LINE);
        assert_eq!(rows[1], vec![1, 1, 1, 1, 1]);
        assert_eq!(rows[2], vec![1, 0, 0, 0, 1]);
        assert!(rows.iter().flatten().any(|&v| v == 1));
    }

    #[test]
    fn control_chars_become_spaces() {
        assert_eq!(block('\n'), block(' '));
        assert_eq!(block('\t'), block(' '));
    }

    #[test]
    fn descenders_reach_the_last_row() {
        let rows = block('(');
        assert!(rows[6].contains(&1));
        let plain = block('A');
        assert!(plain[0].iter().all(|&v| v == 0));
        assert!(plain[6].iter().all(|&v| v == 0));
    }

    #[test]
    fn truncate_appends_dots() {
        let cut = truncate("abcdefghijklmnop", 40, 1);
        assert!(cut.ends_with(".."));
        assert!(width(&cut, 1) <= 40);
        assert_eq!(truncate("hi", 100, 1), "hi");
    }

    #[test]
    fn truncate_never_overflows_a_tiny_field() {
        assert_eq!(truncate("abcdef", 1, 1), "");
    }
}
