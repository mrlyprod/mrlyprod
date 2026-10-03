use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::bang::factory;
use mrlyrs::math::bang::Code;
use std::f64::consts::{FRAC_PI_2, PI};

const LEVEL: usize = 3;
const SIDE: usize = 27;
const LOOPS: usize = 50;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Dsu {
        Dsu {
            parent: (0..size).collect(),
        }
    }
    fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.parent[ra] = rb;
        true
    }
}

fn across(x: usize, y: usize) -> usize {
    y * SIDE + x
}

fn upright(x: usize, y: usize) -> usize {
    SIDE * (SIDE + 1) + y * (SIDE + 1) + x
}

fn arcs(filled: bool, x: usize, y: usize) -> [(usize, usize); 2] {
    let (bottom, top) = (across(x, y), across(x, y + 1));
    let (left, right) = (upright(x, y), upright(x + 1, y));
    if filled {
        [(left, bottom), (right, top)]
    } else {
        [(bottom, right), (left, top)]
    }
}

fn mirrors(filled: &[bool]) -> usize {
    let w = SIDE + 1;
    let mut dsu = Dsu::new(w * w);
    let mut parts = w * w;
    for y in 0..SIDE {
        for x in 0..SIDE {
            let (a, b) = if filled[y * SIDE + x] {
                (y * w + x + 1, (y + 1) * w + x)
            } else {
                (y * w + x, (y + 1) * w + x + 1)
            };
            if dsu.union(a, b) {
                parts -= 1;
            }
        }
    }
    parts - 2 * SIDE - 1
}

fn main() -> Result<()> {
    let design = factory::create(Code::from(495u128), 3, 2, 3, LEVEL)?;
    assert_eq!(design.shape, vec![SIDE, SIDE]);
    assert_eq!(design.sum(), 8u64.pow(LEVEL as u32));
    let parity = factory::create(Code::from(7u128), 3, 2, 2, LEVEL)?;
    assert_eq!(design.bytes()?, parity.bytes()?);
    let filled: Vec<bool> = (0..SIDE * SIDE).map(|f| design.at(f) == 1).collect();

    let nodes = 2 * SIDE * (SIDE + 1);
    let mut dsu = Dsu::new(nodes);
    let mut degree = vec![0u8; nodes];
    for y in 0..SIDE {
        for x in 0..SIDE {
            for (a, b) in arcs(filled[y * SIDE + x], x, y) {
                degree[a] += 1;
                degree[b] += 1;
                dsu.union(a, b);
            }
        }
    }
    let mut open = vec![false; nodes];
    let mut root = vec![false; nodes];
    for (node, d) in degree.iter().enumerate() {
        let r = dsu.find(node);
        root[r] = true;
        open[r] |= *d == 1;
    }
    let strands = (0..nodes).filter(|&n| root[n] && open[n]).count();
    let loops = (0..nodes).filter(|&n| root[n] && !open[n]).count();
    assert_eq!(degree.iter().filter(|&&d| d == 1).count(), 4 * SIDE);
    assert_eq!(strands, 2 * SIDE);
    assert_eq!(loops, LOOPS);
    assert_eq!(mirrors(&filled), LOOPS);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let cell = frame.w / SIDE as f64;
    let thick = cell * 0.15;
    let radius = cell / 2.0;
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    for y in 0..SIDE {
        for x in 0..SIDE {
            if filled[y * SIDE + x] {
                grid.fill(&mut board, x, SIDE - 1 - y, ink::line());
            }
        }
    }
    for y in 0..SIDE {
        for x in 0..SIDE {
            let on = filled[y * SIDE + x];
            let left = frame.x + x as f64 * cell;
            let top = frame.y + (SIDE - 1 - y) as f64 * cell;
            let sweeps = if on {
                [
                    ((left, top + cell), (-FRAC_PI_2, 0.0)),
                    ((left + cell, top), (FRAC_PI_2, PI)),
                ]
            } else {
                [
                    ((left + cell, top + cell), (PI, PI + FRAC_PI_2)),
                    ((left, top), (0.0, FRAC_PI_2)),
                ]
            };
            for ((a, _), (centre, angles)) in arcs(on, x, y).into_iter().zip(sweeps) {
                let r = dsu.find(a);
                let color = if open[r] { ink::blue() } else { ink::orange() };
                board.arc(centre, radius, angles, thick, color);
            }
        }
    }
    save("research-arcs", &board)?;
    Ok(())
}
