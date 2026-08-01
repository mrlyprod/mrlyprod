use super::layout::Op;
use super::text;
use super::{Act, Hit};
use crate::tokens::{contrast, Theme, CONTENT, CONTROL, EDGE, GAP, INSET, PAD, TEXT, TIGHT, WIDTH};
use mrlycore::colors;

#[derive(Clone, Debug, PartialEq)]
pub enum Tap {
    Char(char),
    Put(String),
    Back,
    Enter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cap {
    pub label: String,
    pub fill: Option<[u8; 4]>,
    pub span: usize,
    pub tap: Tap,
}

fn cap(c: char) -> Cap {
    Cap {
        label: c.to_string(),
        fill: None,
        span: 1,
        tap: Tap::Char(c),
    }
}

fn back() -> Cap {
    Cap {
        label: "\u{2190}".to_string(),
        fill: None,
        span: 1,
        tap: Tap::Back,
    }
}

fn enter() -> Cap {
    Cap {
        label: "\u{2192}".to_string(),
        fill: None,
        span: 1,
        tap: Tap::Enter,
    }
}

fn space(span: usize) -> Cap {
    Cap {
        label: "space".to_string(),
        fill: None,
        span,
        tap: Tap::Char(' '),
    }
}

fn row(chars: &str) -> Vec<Cap> {
    chars.chars().map(cap).collect()
}

fn text_board() -> Vec<Vec<Cap>> {
    let mut low = row("zxcvbnm");
    low.push(back());
    vec![
        row("1234567890"),
        row("qwertyuiop"),
        row("asdfghjkl"),
        low,
        vec![space(8), enter()],
    ]
}

fn digit_board() -> Vec<Vec<Cap>> {
    vec![
        row("789"),
        row("456"),
        row("123"),
        row("0.-"),
        vec![back(), enter()],
    ]
}

fn hex_board() -> Vec<Vec<Cap>> {
    let mut tail = row("#abcdef");
    tail.push(back());
    tail.push(enter());
    vec![row("1234567890"), tail]
}

fn color_board() -> Vec<Vec<Cap>> {
    colors::NAMES
        .chunks(5)
        .map(|band| {
            band.iter()
                .map(|&name| {
                    let c = colors::PALETTE[colors::NAMES.iter().position(|&n| n == name).unwrap()];
                    Cap {
                        label: String::new(),
                        fill: Some([c.r, c.g, c.b, c.a]),
                        span: 1,
                        tap: Tap::Put(name.to_string()),
                    }
                })
                .collect()
        })
        .collect()
}

pub fn board(name: &str) -> Vec<Vec<Cap>> {
    match name {
        "digits" => digit_board(),
        "hex" => hex_board(),
        "colors" => color_board(),
        _ => text_board(),
    }
}

pub(crate) fn strip(name: &str, t: &Theme) -> (Vec<Op>, Vec<Hit>, usize) {
    let rows = board(name);
    let h = EDGE + 2 * GAP + rows.len() * CONTROL + rows.len().saturating_sub(1) * TIGHT;
    let mut ops = vec![Op::Rect {
        x: 0,
        y: 0,
        w: WIDTH,
        h: EDGE,
        color: t.faint,
    }];
    let mut hits = Vec::new();
    let mut y = EDGE + GAP;
    for band in rows {
        let spans: usize = band.iter().map(|c| c.span).sum::<usize>().max(1);
        let unit = CONTENT.saturating_sub((spans - 1) * TIGHT) / spans;
        let wide = unit * spans + (spans - 1) * TIGHT;
        let mut x = PAD + (CONTENT - wide) / 2;
        for key in band {
            let kw = unit * key.span + (key.span - 1) * TIGHT;
            let fill = key.fill.unwrap_or(t.faint);
            ops.push(Op::Rect {
                x,
                y,
                w: kw,
                h: CONTROL,
                color: fill,
            });
            if key.fill.is_some() {
                for (rx, ry, rw, rh) in [
                    (x, y, kw, EDGE),
                    (x, y + CONTROL - 1, kw, EDGE),
                    (x, y, EDGE, CONTROL),
                    (x + kw - 1, y, EDGE, CONTROL),
                ] {
                    ops.push(Op::Rect {
                        x: rx,
                        y: ry,
                        w: rw,
                        h: rh,
                        color: t.faint,
                    });
                }
            }
            if !key.label.is_empty() {
                let ink = if key.fill.is_some() {
                    contrast(fill)
                } else {
                    t.ink
                };
                let cut = text::truncate(&key.label, kw.saturating_sub(2 * TIGHT), TEXT);
                let tw = text::width(&cut, TEXT);
                ops.push(Op::Text {
                    x: x + (kw.saturating_sub(tw)) / 2,
                    y: y + INSET,
                    text: cut,
                    scale: TEXT,
                    color: ink,
                });
            }
            hits.push(Hit {
                x,
                y,
                w: kw,
                h: CONTROL,
                act: Act::Cap(key.tap),
            });
            x += kw + TIGHT;
        }
        y += CONTROL + TIGHT;
    }
    (ops, hits, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps(name: &str) -> Vec<Cap> {
        board(name).into_iter().flatten().collect()
    }

    #[test]
    fn glyph_boards_stay_in_the_font() {
        let supported = mrlyfont::supported();
        for name in ["text", "digits", "hex"] {
            for key in caps(name) {
                for c in key.label.chars() {
                    assert!(supported.contains(&c), "{name} cap {:?}", key.label);
                }
            }
        }
    }

    #[test]
    fn boards_carry_the_expected_caps() {
        assert_eq!(caps("text").len(), 39);
        assert_eq!(caps("digits").len(), 14);
        assert_eq!(caps("hex").len(), 19);
        assert_eq!(caps("colors").len(), 15);
        assert_eq!(board("ghost"), board("text"));
    }

    #[test]
    fn color_caps_put_palette_names() {
        for (key, name) in caps("colors").iter().zip(colors::NAMES) {
            assert_eq!(key.tap, Tap::Put(name.to_string()));
            assert!(key.fill.is_some());
            assert!(key.label.is_empty());
        }
    }

    #[test]
    fn every_board_fits_the_sheet() {
        let t = Theme::new("keys", false);
        for name in ["text", "digits", "hex", "colors"] {
            let (_, hits, h) = strip(name, &t);
            assert!(h < super::super::HEIGHT / 3);
            for hit in hits {
                assert!(hit.x + hit.w <= WIDTH - PAD + 1, "{name}");
                assert!(hit.y + hit.h < h, "{name}");
            }
        }
    }
}
