use mrlycore::errors::Result;
use mrlycore::Rng;
use mrlyfig::{ink, save, Board, Color, Frame, Grid};
use mrlymath::two;

const NUMBER: usize = 3;
const BASE: usize = 3;
const LEVEL: usize = 4;
const SIDE: usize = 81;
const WALKERS: usize = 300;
const STEPS: usize = 1200;
const SEED: u64 = 7;

struct Swarm {
    filled: Vec<bool>,
    home: usize,
    at: Vec<usize>,
}

impl Swarm {
    fn spread(&self) -> f64 {
        let (hr, hc) = ((self.home / SIDE) as f64, (self.home % SIDE) as f64);
        let total: f64 = self
            .at
            .iter()
            .map(|&flat| {
                let (r, c) = ((flat / SIDE) as f64 - hr, (flat % SIDE) as f64 - hc);
                r * r + c * c
            })
            .sum();
        (total / self.at.len() as f64).sqrt()
    }
}

fn race(code: u128, seed: u64) -> Result<Swarm> {
    let cell = two::create(code, NUMBER, LEVEL, 0, BASE)?;
    let types = cell.types();
    let filled: Vec<bool> = (0..SIDE * SIDE)
        .map(|flat| types.get(&[flat / SIDE, flat % SIDE]) != 0)
        .collect();
    let centre = ((SIDE - 1) / 2) as i64;
    let home = (0..SIDE * SIDE)
        .filter(|&flat| filled[flat])
        .min_by_key(|&flat| {
            let (r, c) = ((flat / SIDE) as i64 - centre, (flat % SIDE) as i64 - centre);
            r * r + c * c
        })
        .expect("a filled site");
    let mut rng = Rng::new(seed);
    let mut at = vec![home; WALKERS];
    for _ in 0..STEPS {
        for walker in at.iter_mut() {
            let (r, c) = (*walker / SIDE, *walker % SIDE);
            let (nr, nc) = match rng.below(4) {
                0 => (r + 1, c),
                1 => (r.wrapping_sub(1), c),
                2 => (r, c + 1),
                _ => (r, c.wrapping_sub(1)),
            };
            if nr < SIDE && nc < SIDE && filled[nr * SIDE + nc] {
                *walker = nr * SIDE + nc;
            }
        }
    }
    Ok(Swarm { filled, home, at })
}

fn team(board: &mut Board, frame: Frame, swarm: &Swarm, tint: Color) {
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    for flat in 0..SIDE * SIDE {
        if swarm.filled[flat] {
            grid.fill(
                board,
                flat % SIDE,
                flat / SIDE,
                ink::mix(ink::line(), ink::dim(), 0.35),
            );
        }
    }
    let unit = frame.w / SIDE as f64;
    let spot = |flat: usize| {
        (
            frame.x + (flat % SIDE) as f64 * unit + unit / 2.0,
            frame.y + (flat / SIDE) as f64 * unit + unit / 2.0,
        )
    };
    let (hx, hy) = spot(swarm.home);
    board.ring(hx, hy, swarm.spread() * unit, unit * 0.5, tint);
    for &walker in &swarm.at {
        let (x, y) = spot(walker);
        board.disc(x, y, unit * 0.62, tint);
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let fast = race(127, SEED)?;
    let slow = race(239, SEED + 777)?;
    for swarm in [&fast, &slow] {
        assert_eq!(swarm.filled.len(), 6561);
        assert_eq!(swarm.filled.iter().filter(|on| **on).count(), 2401);
        assert_eq!(swarm.at.len(), WALKERS);
        assert!(swarm.at.iter().all(|&flat| swarm.filled[flat]));
    }
    assert!(fast.spread() > slow.spread());
    let side = frame.w * 0.55;
    team(
        &mut board,
        Frame::new(frame.x, frame.y, side, side),
        &fast,
        ink::blue(),
    );
    team(
        &mut board,
        Frame::new(
            frame.x + frame.w - side,
            frame.y + frame.h - side,
            side,
            side,
        ),
        &slow,
        ink::orange(),
    );
    save("demo-race", &board)?;
    Ok(())
}
