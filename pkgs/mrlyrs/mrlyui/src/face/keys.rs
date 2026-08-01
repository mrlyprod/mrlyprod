use super::layout::Op;
use super::text;
use super::{Act, Hit};
use crate::tokens::{contrast, Theme, CONTENT, CONTROL, EDGE, GAP, INSET, PAD, TEXT, TIGHT, WIDTH};
use mrlycore::colors;

const DIGITS: &str = "1234567890";
const SYMBOLS: &str = "!@#$%^&*()";
const LETTERS: [&str; 3] = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

#[derive(Clone, Debug, PartialEq)]
pub enum Tap {
    Char(char),
    Put(String),
    Back,
    Enter,
    Shift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cap {
    pub label: String,
    pub fill: Option<[u8; 4]>,
    pub span: usize,
    pub on: bool,
    pub tap: Tap,
}

fn key(label: &str, tap: Tap) -> Cap {
    Cap {
        label: label.to_string(),
        fill: None,
        span: 1,
        on: false,
        tap,
    }
}

fn cap(c: char) -> Cap {
    key(&c.to_string(), Tap::Char(c))
}

fn back() -> Cap {
    key("\u{2190}", Tap::Back)
}

fn enter() -> Cap {
    key("\u{2192}", Tap::Enter)
}

fn shift(on: bool) -> Cap {
    Cap {
        on,
        ..key("\u{2191}", Tap::Shift)
    }
}

fn space(span: usize) -> Cap {
    Cap {
        span,
        ..key("space", Tap::Char(' '))
    }
}

fn row(chars: &str) -> Vec<Cap> {
    chars.chars().map(cap).collect()
}

fn letters(band: usize, up: bool) -> Vec<Cap> {
    let plain = LETTERS[band];
    if up {
        row(&plain.to_ascii_uppercase())
    } else {
        row(plain)
    }
}

fn text_board(up: bool) -> Vec<Vec<Cap>> {
    let mut low = vec![shift(up)];
    low.extend(letters(2, up));
    low.push(back());
    vec![
        row(if up { SYMBOLS } else { DIGITS }),
        letters(0, up),
        letters(1, up),
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
    vec![row(DIGITS), tail]
}

fn color_board() -> Vec<Vec<Cap>> {
    colors::NAMES
        .chunks(5)
        .map(|band| {
            band.iter()
                .map(|&name| {
                    let c = colors::PALETTE[colors::NAMES.iter().position(|&n| n == name).unwrap()];
                    Cap {
                        fill: Some([c.r, c.g, c.b, c.a]),
                        ..key("", Tap::Put(name.to_string()))
                    }
                })
                .collect()
        })
        .collect()
}

pub fn board(name: &str, up: bool) -> Vec<Vec<Cap>> {
    match name {
        "digits" => digit_board(),
        "hex" => hex_board(),
        "colors" => color_board(),
        _ => text_board(up),
    }
}

pub(crate) fn strip(name: &str, up: bool, t: &Theme) -> (Vec<Op>, Vec<Hit>, usize) {
    let rows = board(name, up);
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
            let fill = match (key.on, key.fill) {
                (true, _) => t.ink,
                (false, Some(swatch)) => swatch,
                (false, None) => t.faint,
            };
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
                    (x, y + CONTROL - EDGE, kw, EDGE),
                    (x, y, EDGE, CONTROL),
                    (x + kw - EDGE, y, EDGE, CONTROL),
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
                let ink = match (key.on, key.fill) {
                    (true, _) => t.board,
                    (false, Some(_)) => contrast(fill),
                    (false, None) => t.ink,
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
        board(name, false).into_iter().flatten().collect()
    }

    fn shifted() -> Vec<Cap> {
        board("text", true).into_iter().flatten().collect()
    }

    #[test]
    fn glyph_boards_stay_in_the_font() {
        let supported = mrlyfont::supported();
        let mut boards = vec![shifted()];
        boards.extend(["text", "digits", "hex"].map(caps));
        for keys in boards {
            for key in keys {
                for c in key.label.chars() {
                    assert!(supported.contains(&c), "cap {:?}", key.label);
                }
            }
        }
    }

    #[test]
    fn boards_carry_the_expected_caps() {
        assert_eq!(caps("text").len(), 40);
        assert_eq!(caps("digits").len(), 14);
        assert_eq!(caps("hex").len(), 19);
        assert_eq!(caps("colors").len(), 15);
        assert_eq!(board("ghost", false), board("text", false));
    }

    #[test]
    fn the_shift_layer_swaps_case_and_the_top_row() {
        let low = caps("text");
        let up = shifted();
        assert_eq!(low.len(), up.len());
        for (a, b) in low.iter().zip(&up) {
            match (&a.tap, &b.tap) {
                (Tap::Char(x), Tap::Char(y)) if x.is_ascii_alphabetic() => {
                    assert_eq!(*y, x.to_ascii_uppercase());
                    assert_eq!(b.label, y.to_string());
                }
                (Tap::Char(x), Tap::Char(y)) if x.is_ascii_digit() => {
                    assert!(SYMBOLS.contains(*y), "{y} is not a shifted digit");
                }
                (a, b) => assert_eq!(a, b),
            }
        }
        assert_eq!(up.iter().filter(|c| c.tap == Tap::Shift).count(), 1);
        assert!(up.iter().find(|c| c.tap == Tap::Shift).unwrap().on);
        assert!(!low.iter().find(|c| c.tap == Tap::Shift).unwrap().on);
    }

    #[test]
    fn only_the_text_board_shifts() {
        for name in ["digits", "hex", "colors"] {
            assert_eq!(board(name, true), board(name, false));
        }
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
            for up in [false, true] {
                let (_, hits, h) = strip(name, up, &t);
                assert!(h < super::super::HEIGHT / 3);
                for hit in hits {
                    assert!(hit.x + hit.w <= WIDTH - PAD + 1, "{name}");
                    assert!(hit.y + hit.h < h, "{name}");
                }
            }
        }
    }

    #[test]
    fn an_engaged_shift_inverts_its_cap() {
        let t = Theme::new("keys", false);
        let paint = |up| {
            let (ops, _, _) = strip("text", up, &t);
            ops.iter()
                .filter(|op| matches!(op, Op::Rect { h, .. } if *h == CONTROL))
                .map(|op| match op {
                    Op::Rect { color, .. } => *color,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        };
        assert!(!paint(false).contains(&t.ink));
        assert_eq!(paint(true).iter().filter(|c| **c == t.ink).count(), 1);
    }
}
