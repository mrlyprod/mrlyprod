use super::layout::Op;
use super::text;
use super::{Act, Hit};
use crate::tokens::{
    contrast, Theme, CONTENT, CONTROL, EDGE, GAP, LINE, PAD, TEXT, TIGHT, TITLE, WIDTH,
};
use mrlycore::colors;

const DIGITS: &str = "1234567890";
const SYMBOLS: &str = "!@#$%^&*()";
const LETTERS: [&str; 3] = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
const TILES: usize = 11;
const BANDS: usize = 4;
const PAGE: usize = TILES * BANDS;

#[derive(Clone, Debug, PartialEq)]
pub enum Tap {
    Char(char),
    Put(String),
    Back,
    Enter,
    Shift,
    Board(String),
    Page(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cap {
    pub label: String,
    pub fill: Option<[u8; 4]>,
    pub span: usize,
    pub scale: usize,
    pub on: bool,
    pub tap: Tap,
}

fn key(label: &str, tap: Tap) -> Cap {
    Cap {
        label: label.to_string(),
        fill: None,
        span: 1,
        scale: TEXT,
        on: false,
        tap,
    }
}

fn wide(cap: Cap) -> Cap {
    Cap { span: 2, ..cap }
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

fn board_cap(name: &str, label: &str) -> Cap {
    key(label, Tap::Board(name.to_string()))
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
        vec![board_cap("emoji", "\u{1f600}"), space(7), enter()],
    ]
}

fn emoji_board(page: usize) -> Vec<Vec<Cap>> {
    let all = crate::emoji::catalog();
    let pages = pages("emoji");
    let page = page.min(pages - 1);
    let shown = &all[(page * PAGE).min(all.len())..((page + 1) * PAGE).min(all.len())];
    let mut rows: Vec<Vec<Cap>> = shown
        .chunks(TILES)
        .map(|band| {
            band.iter()
                .map(|&c| Cap {
                    scale: TITLE,
                    ..key(&c.to_string(), Tap::Char(c))
                })
                .collect()
        })
        .collect();
    rows.push(vec![
        wide(board_cap("text", "abc")),
        wide(key("\u{2191}", Tap::Page(false))),
        wide(board_cap("emoji", &format!("{}/{}", page + 1, pages))),
        wide(key("\u{2193}", Tap::Page(true))),
        wide(back()),
        wide(enter()),
    ]);
    rows
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

pub fn pages(name: &str) -> usize {
    match name {
        "emoji" => crate::emoji::catalog().len().div_ceil(PAGE).max(1),
        _ => 1,
    }
}

pub fn board(name: &str, up: bool, page: usize) -> Vec<Vec<Cap>> {
    match name {
        "digits" => digit_board(),
        "hex" => hex_board(),
        "colors" => color_board(),
        "emoji" => emoji_board(page),
        _ => text_board(up),
    }
}

pub(crate) fn strip(name: &str, up: bool, page: usize, t: &Theme) -> (Vec<Op>, Vec<Hit>, usize) {
    let rows = board(name, up, page);
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
    let widest = rows
        .iter()
        .map(|band| band.iter().map(|c| c.span).sum::<usize>())
        .max()
        .unwrap_or(1)
        .max(1);
    let unit = CONTENT.saturating_sub((widest - 1) * TIGHT) / widest;
    for band in rows {
        let spans: usize = band.iter().map(|c| c.span).sum::<usize>().max(1);
        let full = unit * spans + (spans - 1) * TIGHT;
        let mut x = PAD + CONTENT.saturating_sub(full) / 2;
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
                let cut = text::truncate(&key.label, kw.saturating_sub(2 * TIGHT), key.scale);
                let tw = text::width(&cut, key.scale);
                ops.push(Op::Text {
                    x: x + (kw.saturating_sub(tw)) / 2,
                    y: y + (CONTROL.saturating_sub(LINE * key.scale)) / 2,
                    text: cut,
                    scale: key.scale,
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
        board(name, false, 0).into_iter().flatten().collect()
    }

    fn shifted() -> Vec<Cap> {
        board("text", true, 0).into_iter().flatten().collect()
    }

    #[test]
    fn every_cap_label_is_drawable() {
        let supported = mrlyfont::supported();
        let mut boards = vec![shifted(), caps("emoji")];
        boards.extend(["text", "digits", "hex"].map(caps));
        for keys in boards {
            for key in keys {
                for c in key.label.chars() {
                    let drawable = supported.contains(&c) || crate::emoji::has(c);
                    assert!(drawable, "cap {:?} draws a hollow box", key.label);
                }
            }
        }
    }

    #[test]
    fn boards_carry_the_expected_caps() {
        assert_eq!(caps("text").len(), 41);
        assert_eq!(caps("digits").len(), 14);
        assert_eq!(caps("hex").len(), 19);
        assert_eq!(caps("colors").len(), 15);
        assert_eq!(board("ghost", false, 0), board("text", false, 0));
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
            assert_eq!(board(name, true, 0), board(name, false, 0));
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
        for name in ["text", "digits", "hex", "colors", "emoji"] {
            for up in [false, true] {
                let (_, hits, h) = strip(name, up, 0, &t);
                assert!(h < super::super::HEIGHT / 3);
                for hit in hits {
                    assert!(hit.x + hit.w <= WIDTH - PAD + 1, "{name}");
                    assert!(hit.y + hit.h < h, "{name}");
                }
            }
        }
    }

    #[test]
    fn every_page_of_the_emoji_board_is_reachable_and_whole() {
        let all = crate::emoji::catalog();
        let mut seen: Vec<char> = Vec::new();
        for page in 0..pages("emoji") {
            let rows = board("emoji", false, page);
            assert!(rows.len() <= BANDS + 1);
            let nav = rows.last().unwrap();
            assert_eq!(nav.iter().filter(|c| c.tap == Tap::Page(true)).count(), 1);
            assert_eq!(nav.iter().filter(|c| c.tap == Tap::Page(false)).count(), 1);
            for band in &rows[..rows.len() - 1] {
                assert!(band.len() <= TILES);
                for key in band {
                    let Tap::Char(c) = key.tap else { panic!() };
                    assert_eq!(key.scale, TITLE);
                    assert!(crate::emoji::has(c), "{c} is not in the atlas");
                    seen.push(c);
                }
            }
        }
        assert_eq!(seen, all);
        assert_eq!(
            board("emoji", false, 9999),
            board("emoji", false, pages("emoji") - 1)
        );
    }

    #[test]
    fn the_text_board_opens_the_emoji_board_and_back() {
        let out = |name: &str| -> Vec<String> {
            board(name, false, 0)
                .into_iter()
                .flatten()
                .filter_map(|c| match c.tap {
                    Tap::Board(to) => Some(to),
                    _ => None,
                })
                .collect()
        };
        assert_eq!(out("text"), vec!["emoji".to_string()]);
        assert_eq!(out("emoji"), vec!["text".to_string(), "emoji".to_string()]);
        assert!(out("digits").is_empty());
        assert!(out("colors").is_empty());
    }

    #[test]
    fn one_board_holds_one_key_width() {
        let t = Theme::new("keys", false);
        for name in ["text", "digits", "hex", "colors", "emoji"] {
            let (_, hits, _) = strip(name, false, 0, &t);
            let narrow = hits.iter().map(|h| h.w).min().unwrap();
            for hit in &hits {
                let spans = (hit.w + TIGHT) / (narrow + TIGHT);
                assert_eq!(hit.w, narrow * spans + (spans - 1) * TIGHT, "{name}");
            }
        }
    }

    #[test]
    fn an_engaged_shift_inverts_its_cap() {
        let t = Theme::new("keys", false);
        let paint = |up| {
            let (ops, _, _) = strip("text", up, 0, &t);
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
