use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Rng};

const NAME: &str = "wiki-truchet-tiles";
const SIDE: usize = 16;
const SEED: u64 = 7;
const LOOPS: usize = 19;

fn top(row: usize, col: usize) -> usize {
    row * SIDE + col
}

fn left(row: usize, col: usize) -> usize {
    (SIDE + 1) * SIDE + row * (SIDE + 1) + col
}

fn arcs(turn: bool, row: usize, col: usize) -> [(usize, usize); 2] {
    let (t, b) = (top(row, col), top(row + 1, col));
    let (l, r) = (left(row, col), left(row, col + 1));
    if turn {
        [(t, r), (l, b)]
    } else {
        [(t, l), (r, b)]
    }
}

fn main() -> Result<()> {
    let mut rng = Rng::new(SEED);
    let turns: Vec<bool> = (0..SIDE * SIDE).map(|_| rng.boolean()).collect();
    let nodes = 2 * SIDE * (SIDE + 1);
    let mut links: Vec<Vec<usize>> = vec![Vec::new(); nodes];
    let mut ends = Vec::new();
    for row in 0..SIDE {
        for col in 0..SIDE {
            for (a, b) in arcs(turns[row * SIDE + col], row, col) {
                let arc = ends.len();
                ends.push((a, b));
                links[a].push(arc);
                links[b].push(arc);
            }
        }
    }
    assert_eq!(ends.len(), 2 * SIDE * SIDE);
    let border = links.iter().filter(|l| l.len() == 1).count();
    assert_eq!(border, 4 * SIDE);

    let mut curve = vec![usize::MAX; ends.len()];
    let mut open = Vec::new();
    for start in 0..ends.len() {
        if curve[start] != usize::MAX {
            continue;
        }
        let id = open.len();
        let mut stack = vec![start];
        let mut touches = false;
        while let Some(arc) = stack.pop() {
            if curve[arc] != usize::MAX {
                continue;
            }
            curve[arc] = id;
            let (a, b) = ends[arc];
            for node in [a, b] {
                touches |= links[node].len() == 1;
                stack.extend(links[node].iter().filter(|&&n| curve[n] == usize::MAX));
            }
        }
        open.push(touches);
    }
    let strands = open.iter().filter(|o| **o).count();
    assert_eq!(strands, 2 * SIDE);
    assert_eq!(open.len() - strands, LOOPS);

    let on: Vec<u8> = curve.iter().map(|&id| u8::from(open[id])).collect();
    save(NAME, &json!({"open": on}))
}
