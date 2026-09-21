use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board, Grid};
use std::f64::consts::TAU;

const STATES: usize = 3;
const LENGTH: usize = 8;
const RULE: [[u64; STATES]; STATES] = [[1, 1, 0], [0, 0, 1], [1, 0, 0]];

fn times(a: &[[u64; STATES]; STATES], b: &[[u64; STATES]; STATES]) -> [[u64; STATES]; STATES] {
    let mut out = [[0u64; STATES]; STATES];
    for row in 0..STATES {
        for mid in 0..STATES {
            for col in 0..STATES {
                out[row][col] += a[row][mid] * b[mid][col];
            }
        }
    }
    out
}

fn total(a: &[[u64; STATES]; STATES]) -> u64 {
    a.iter().map(|row| row.iter().sum::<u64>()).sum()
}

fn dart(board: &mut Board, tip: (f64, f64), way: (f64, f64), head: f64) {
    let side = (-way.1, way.0);
    let base = (tip.0 - way.0 * head, tip.1 - way.1 * head);
    board.triangle(
        tip,
        (base.0 + side.0 * head * 0.5, base.1 + side.1 * head * 0.5),
        (base.0 - side.0 * head * 0.5, base.1 - side.1 * head * 0.5),
        ink::yellow(),
    );
}

fn main() -> Result<()> {
    let mut power = RULE;
    let mut walks = Vec::with_capacity(LENGTH);
    for _ in 0..LENGTH {
        walks.push(total(&power));
        power = times(&power, &RULE);
    }
    assert_eq!(walks, vec![4, 6, 9, 13, 19, 28, 41, 60]);
    let ratio = walks[LENGTH - 1] as f64 / walks[LENGTH - 2] as f64;
    assert!((ratio - 1.465571).abs() < 0.01);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let halves = frame.rows(2);
    let panels = halves[0].cols(2);

    let stage = panels[0].square().inset(panels[0].w * 0.06);
    let (cx, cy) = stage.center();
    let reach = stage.w * 0.34;
    let knob = stage.w * 0.10;
    let seats: Vec<(f64, f64)> = (0..STATES)
        .map(|state| {
            let turn = TAU * (state as f64 / STATES as f64 - 0.25);
            (cx + reach * turn.cos(), cy + reach * turn.sin())
        })
        .collect();
    let head = knob * 0.62;
    for (from, to) in [(0usize, 1usize), (1, 2), (2, 0)] {
        let (a, b) = (seats[from], seats[to]);
        let span = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        let way = ((b.0 - a.0) / span, (b.1 - a.1) / span);
        let start = (a.0 + way.0 * knob * 1.12, a.1 + way.1 * knob * 1.12);
        let tip = (b.0 - way.0 * knob * 1.12, b.1 - way.1 * knob * 1.12);
        board.segment(start, tip, knob * 0.22, ink::yellow());
        dart(&mut board, tip, way, head);
    }
    let loop_at = (seats[0].0, seats[0].1 - knob * 1.35);
    board.ring(
        loop_at.0,
        loop_at.1,
        knob * 0.92,
        knob * 0.22,
        ink::yellow(),
    );
    for seat in &seats {
        board.disc(seat.0, seat.1, knob, ink::blue());
    }
    let turn = TAU / 6.0;
    let tip = (
        loop_at.0 + knob * 0.92 * turn.cos(),
        loop_at.1 + knob * 0.92 * turn.sin(),
    );
    dart(&mut board, tip, (-turn.sin(), turn.cos()), head);

    let table = panels[1].square().inset(panels[1].w * 0.12);
    let grid = Grid::new(table, STATES, STATES, 0.10);
    for row in 0..STATES {
        for col in 0..STATES {
            let paint = if RULE[row][col] == 1 {
                ink::yellow()
            } else {
                ink::fade(ink::line(), 0.45)
            };
            grid.fill(&mut board, col, row, paint);
        }
    }

    let chart = halves[1].inset(halves[1].h * 0.14);
    let tall: Vec<f64> = walks.iter().map(|&n| n as f64).collect();
    plot::bars(&mut board, chart, &tall, 0.34, ink::blue());
    plot::baseline(&mut board, chart, ink::line());
    save("wiki-transfer-matrix", &board)?;
    Ok(())
}
