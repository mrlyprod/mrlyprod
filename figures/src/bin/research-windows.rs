use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;
use mrlyrs::math::bang::factory;
use mrlyrs::math::bang::Code;
use std::collections::BTreeSet;

const LEVEL: usize = 3;
const SIDE: usize = 27;
const SCAN: usize = 5;
const COUNTS: [usize; 4] = [2, 10, 40, 74];
const RING: usize = 11;
const CELL: usize = 22;
const HAIR: usize = 2;
const GUTTER: usize = 15;

type Window = Vec<u8>;

fn render(level: usize) -> Result<(usize, Vec<u8>)> {
    let design = factory::create(Code::from(495u128), 3, 2, 3, level)?;
    let side = 3usize.pow(level as u32);
    assert_eq!(design.shape, vec![side, side]);
    assert_eq!(design.sum(), 8u64.pow(level as u32));
    let parity = factory::create(Code::from(7u128), 3, 2, 2, level)?;
    assert_eq!(design.bytes()?, parity.bytes()?);
    Ok((side, design.bytes()?.to_vec()))
}

fn windows(side: usize, cells: &[u8], k: usize) -> BTreeSet<Window> {
    let mut out = BTreeSet::new();
    for r in 0..=side - k {
        for c in 0..=side - k {
            out.insert(
                (0..k * k)
                    .map(|i| cells[(r + i / k) * side + c + i % k])
                    .collect(),
            );
        }
    }
    out
}

fn turn(w: &[u8]) -> Window {
    (0..9).map(|i| w[(2 - i % 3) * 3 + i / 3]).collect()
}

fn orbit(w: &[u8]) -> Vec<Window> {
    let mut out = vec![w.to_vec()];
    loop {
        let next = turn(&out[out.len() - 1]);
        if next == out[0] {
            return out;
        }
        out.push(next);
    }
}

fn ones(w: &[u8]) -> usize {
    w.iter().filter(|&&v| v == 1).count()
}

fn low(w: &[u8]) -> usize {
    (0..9).filter(|&i| w[i] == 1).map(|i| i / 3).sum()
}

fn ring(three: &BTreeSet<Window>) -> Vec<(usize, usize, Window)> {
    let mut seen = BTreeSet::new();
    let mut reps = Vec::new();
    let mut fixed = Vec::new();
    for w in three {
        if seen.contains(w) {
            continue;
        }
        let orb = orbit(w);
        seen.extend(orb.iter().cloned());
        if orb.len() == 4 {
            reps.push(orb.into_iter().max_by_key(|o| (low(o), o.clone())).unwrap());
        } else {
            fixed.extend(orb);
        }
    }
    reps.sort_by_key(|w| (ones(w), w.clone()));
    assert_eq!(reps.len(), RING - 2);
    assert_eq!(fixed.len(), 4);
    let pick = |n: usize, head: u8| -> Window {
        let found: Vec<&Window> = fixed
            .iter()
            .filter(|w| ones(w) == n && w[0] == head)
            .collect();
        assert_eq!(found.len(), 1);
        found[0].clone()
    };
    let last = RING - 1;
    let mut slots = vec![
        (0, 0, pick(7, 1)),
        (0, last, pick(7, 0)),
        (last, last, pick(8, 1)),
        (last, 0, pick(0, 0)),
    ];
    assert_eq!(turn(&slots[0].2), slots[1].2);
    for (i, rep) in reps.iter().enumerate() {
        let a = turn(rep);
        let b = turn(&a);
        let c = turn(&b);
        slots.push((0, i + 1, rep.clone()));
        slots.push((i + 1, last, a));
        slots.push((last, last - 1 - i, b));
        slots.push((last - 1 - i, 0, c));
    }
    let drawn: BTreeSet<Window> = slots.iter().map(|s| s.2.clone()).collect();
    assert_eq!(&drawn, three);
    assert_eq!(slots.len(), 4 * RING - 4);
    slots
}

fn cells(
    board: &mut Board,
    x: usize,
    y: usize,
    side: usize,
    w: &[u8],
    on: Color,
    off: Option<Color>,
) {
    for (i, &v) in w.iter().enumerate() {
        let color = if v == 1 { Some(on) } else { off };
        if let Some(color) = color {
            let (cx, cy) = (x + (i % side) * CELL, y + (i / side) * CELL);
            let size = (CELL - HAIR) as f64;
            board.rect(cx as f64, cy as f64, size, size, color);
        }
    }
}

fn main() -> Result<()> {
    let (side, scan) = render(SCAN)?;
    let (before, coarse) = render(SCAN - 1)?;
    for (k, count) in COUNTS.iter().enumerate() {
        let found = windows(side, &scan, k + 1);
        assert_eq!(found.len(), *count);
        assert_eq!(found, windows(before, &coarse, k + 1));
    }
    let three = windows(side, &scan, 3);
    let slots = ring(&three);
    let (ground, carpet) = render(LEVEL)?;
    assert_eq!(ground, SIDE);

    let mut board = Board::square();
    let tile = 3 * CELL - HAIR;
    let span = RING * tile + (RING - 1) * GUTTER;
    let origin = (board.width - span) / 2;
    let inner = (board.width - (SIDE * CELL - HAIR)) / 2;
    cells(
        &mut board,
        inner,
        inner,
        SIDE,
        &carpet,
        ink::fade(ink::dim(), 0.75),
        None,
    );
    for (r, c, w) in &slots {
        let (x, y) = (origin + c * (tile + GUTTER), origin + r * (tile + GUTTER));
        cells(&mut board, x, y, 3, w, ink::blue(), Some(ink::line()));
    }
    save("research-windows", &board)?;
    Ok(())
}
