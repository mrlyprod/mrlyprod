use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;
use std::f64::consts::{FRAC_PI_2, TAU};

const DEPTH: usize = 8;

fn allowed(word: u32, length: usize) -> bool {
    let bits = word & ((1u32 << length) - 1);
    bits & (bits >> 1) == 0
}

fn sector(
    board: &mut Board,
    center: (f64, f64),
    radii: (f64, f64),
    turns: (f64, f64),
    gap: f64,
    color: Color,
) {
    let (inner, outer) = radii;
    let at = |r: f64, turn: f64| {
        let angle = TAU * turn - FRAC_PI_2;
        (center.0 + r * angle.cos(), center.1 + r * angle.sin())
    };
    let side = |r: f64| {
        let trim = gap / (2.0 * r * TAU);
        (turns.0 + trim, turns.1 - trim)
    };
    let steps =
        |r: f64, span: (f64, f64)| ((span.1 - span.0) * TAU * r / 3.0).ceil().max(1.0) as usize;
    let (far, near) = (side(outer), side(inner));
    let mut pts = Vec::new();
    let n = steps(outer, far);
    for i in 0..=n {
        pts.push(at(outer, far.0 + (far.1 - far.0) * i as f64 / n as f64));
    }
    let n = steps(inner, near);
    for i in (0..=n).rev() {
        pts.push(at(inner, near.0 + (near.1 - near.0) * i as f64 / n as f64));
    }
    board.polygon(&pts, color);
}

fn main() -> Result<()> {
    let counts: Vec<usize> = (1..=DEPTH)
        .map(|length| (0..1u32 << length).filter(|&w| allowed(w, length)).count())
        .collect();
    assert_eq!(counts, vec![2, 3, 5, 8, 13, 21, 34, 55]);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let center = frame.center();
    let reach = frame.w.min(frame.h) / 2.0;
    let hole = reach * 0.16;
    let step = (reach - hole) / DEPTH as f64;
    let rim = 4.0;
    let gap = 4.0;

    board.disc(center.0, center.1, hole * 0.42, ink::dim());
    for length in 1..=DEPTH {
        let inner = hole + step * (length - 1) as f64 + rim / 2.0;
        let outer = inner + step - rim;
        let slot = 1.0 / (1u32 << length) as f64;
        for word in 0..1u32 << length {
            let parent = word >> 1;
            if length > 1 && !allowed(parent, length - 1) {
                continue;
            }
            let color = if allowed(word, length) {
                if word & 1 == 1 {
                    ink::yellow()
                } else {
                    ink::blue()
                }
            } else {
                ink::fade(ink::line(), 0.7)
            };
            let turns = (word as f64 * slot, (word + 1) as f64 * slot);
            sector(&mut board, center, (inner, outer), turns, gap, color);
        }
    }
    save("wiki-subshift-of-finite-type", &board)?;
    Ok(())
}
