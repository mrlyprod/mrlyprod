use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;

const NAME: &str = "wiki-substitution-tiling";
const DEPTH: usize = 3;

#[derive(Clone, Copy)]
struct Chair {
    x: f64,
    y: f64,
    side: f64,
    turn: usize,
}

fn spin(p: (f64, f64), centre: (f64, f64), turn: usize) -> (f64, f64) {
    let (mut dx, mut dy) = (p.0 - centre.0, p.1 - centre.1);
    for _ in 0..turn % 4 {
        (dx, dy) = (-dy, dx);
    }
    (centre.0 + dx, centre.1 + dy)
}

fn inflate(c: Chair) -> [Chair; 4] {
    let h = c.side / 2.0;
    let q = c.side / 4.0;
    let centre = (c.x + h, c.y + h);
    let place = |x: f64, y: f64, turn: usize| {
        let mid = spin((c.x + x + q, c.y + y + q), centre, c.turn);
        Chair {
            x: mid.0 - q,
            y: mid.1 - q,
            side: h,
            turn: (c.turn + turn) % 4,
        }
    };
    [
        place(0.0, 0.0, 0),
        place(q, q, 0),
        place(h, 0.0, 1),
        place(0.0, h, 3),
    ]
}

fn area(c: Chair) -> f64 {
    0.75 * c.side * c.side
}

fn main() -> Result<()> {
    let root = Chair {
        x: 0.0,
        y: 0.0,
        side: 1.0,
        turn: 0,
    };
    let mut levels = vec![vec![root]];
    for _ in 0..DEPTH {
        let next: Vec<Chair> = levels
            .last()
            .unwrap()
            .iter()
            .flat_map(|c| inflate(*c))
            .collect();
        levels.push(next);
    }
    let counts: Vec<usize> = levels.iter().map(Vec::len).collect();
    assert_eq!(counts, vec![1, 4, 16, 64]);
    let leaves = levels.last().unwrap();
    let total: f64 = leaves.iter().map(|c| area(*c)).sum();
    assert!((total - area(root)).abs() < 1e-12);
    let mut cells = std::collections::HashSet::new();
    let grain = 2.0 / leaves[0].side;
    for c in leaves {
        let h = c.side / 2.0;
        for (u, v) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
            let p = spin(
                (c.x + u * c.side, c.y + v * c.side),
                (c.x + h, c.y + h),
                c.turn,
            );
            let inside = !(u > 0.5 && v > 0.5);
            if inside {
                assert!(cells.insert(((p.0 * grain).floor() as i64, (p.1 * grain).floor() as i64)));
            }
        }
    }
    assert_eq!(cells.len(), 3 * 64);
    assert!(cells
        .iter()
        .all(|&(i, j)| (0..16).contains(&i) && (0..16).contains(&j) && (i < 8 || j < 8)));
    let even = leaves.iter().filter(|c| c.turn.is_multiple_of(2)).count();
    assert_eq!((even, leaves.len() - even), (32, 32));

    let rows: Vec<_> = levels[1..]
        .iter()
        .map(|level| {
            level
                .iter()
                .map(|c| json!([c.x, c.y, c.turn]))
                .collect::<Vec<_>>()
        })
        .collect();
    save(NAME, &json!({"levels": rows}))
}
