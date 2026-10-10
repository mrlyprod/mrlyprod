use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::{json, Rng};

const NAME: &str = "wiki-wang-tiles";
const SIDE: usize = 10;
const TILES: [[u8; 4]; 11] = [
    [3, 1, 1, 1],
    [3, 1, 2, 2],
    [3, 3, 3, 1],
    [2, 2, 1, 0],
    [2, 2, 0, 2],
    [0, 0, 1, 0],
    [0, 3, 2, 1],
    [1, 0, 2, 2],
    [1, 1, 0, 2],
    [1, 3, 2, 3],
    [3, 0, 1, 1],
];
const LEFT: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const TOP: usize = 3;

fn fits(patch: &[usize], at: usize, tile: usize) -> bool {
    let (row, col) = (at / SIDE, at % SIDE);
    let t = TILES[tile];
    let left = col == 0 || TILES[patch[at - 1]][RIGHT] == t[LEFT];
    let up = row == 0 || TILES[patch[at - SIDE]][BOTTOM] == t[TOP];
    left && up
}

fn solve(rng: &mut Rng) -> Vec<usize> {
    let cells = SIDE * SIDE;
    let mut orders: Vec<Vec<usize>> = Vec::with_capacity(cells);
    let mut tried = vec![0usize; cells];
    let mut patch = vec![usize::MAX; cells];
    let mut at = 0;
    while at < cells {
        if orders.len() == at {
            let mut order: Vec<usize> = (0..TILES.len()).collect();
            rng.shuffle(&mut order);
            orders.push(order);
            tried[at] = 0;
        }
        let mut placed = false;
        while tried[at] < TILES.len() {
            let tile = orders[at][tried[at]];
            tried[at] += 1;
            if fits(&patch, at, tile) {
                patch[at] = tile;
                placed = true;
                break;
            }
        }
        if placed {
            at += 1;
        } else {
            orders.pop();
            patch[at] = usize::MAX;
            assert!(at > 0);
            at -= 1;
        }
    }
    patch
}

fn kinds(patch: &[usize]) -> usize {
    let mut used = [false; TILES.len()];
    for &tile in patch {
        used[tile] = true;
    }
    used.iter().filter(|&&u| u).count()
}

fn main() -> Result<()> {
    let mut colours: Vec<u8> = TILES.iter().flatten().copied().collect();
    colours.sort_unstable();
    colours.dedup();
    assert_eq!(colours, vec![0, 1, 2, 3]);
    let mut seed = 1u64;
    let mut patch = solve(&mut Rng::new(seed));
    while kinds(&patch) < TILES.len() {
        seed += 1;
        assert!(seed < 1000);
        patch = solve(&mut Rng::new(seed));
    }
    let mut inner = 0;
    for row in 0..SIDE {
        for col in 0..SIDE {
            let t = TILES[patch[row * SIDE + col]];
            if col + 1 < SIDE {
                assert_eq!(t[RIGHT], TILES[patch[row * SIDE + col + 1]][LEFT]);
                inner += 1;
            }
            if row + 1 < SIDE {
                assert_eq!(t[BOTTOM], TILES[patch[(row + 1) * SIDE + col]][TOP]);
                inner += 1;
            }
        }
    }
    assert_eq!(inner, 2 * SIDE * (SIDE - 1));
    assert_eq!(kinds(&patch), TILES.len());
    save(NAME, &json!({"tiles": TILES, "patch": patch}))
}
