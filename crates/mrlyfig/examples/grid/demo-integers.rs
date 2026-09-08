use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Frame, Grid};
use mrlylab::ledger::{keys, terms, Cost, Key, Tier};

const CAP: usize = 48;
const CELLS: u128 = 100_000;
const WINDOW: i128 = 1_000;
const BLOCK: usize = 8;
const DEPTHS: [usize; 4] = [8, 16, 32, 48];
const COLS: usize = 40;
const LINES: usize = 25;

fn footprint(key: &Key, index: usize) -> Option<u128> {
    let (number, level) = key.axis.place(index, key.number());
    let number = number as u128;
    let dimension = key.dimension as u32;
    match key.measure.cost() {
        Cost::Closed => Some(1),
        Cost::Convolved => {
            let tile = number.checked_pow(dimension)?;
            let side = number.checked_pow(level)?;
            let span = key.dimension as u128 * (side - 1) + 1;
            tile.checked_add(span.checked_mul(level as u128)?)
        }
        Cost::Grid => number.checked_pow(dimension.checked_mul(level)?),
    }
}

fn allowance(key: &Key) -> usize {
    (0..CAP)
        .take_while(|&index| footprint(key, index).is_some_and(|cells| cells <= CELLS))
        .count()
}

fn ceiling_stop(read: &[i128]) -> Option<usize> {
    let mut previous: Option<i128> = None;
    for (index, &term) in read.iter().enumerate() {
        if previous.is_some_and(|last| term <= last) {
            return None;
        }
        if term > WINDOW {
            return Some(index);
        }
        previous = Some(term);
    }
    None
}

fn rendered(key: &Key) -> Option<Vec<i128>> {
    let allowed = allowance(key);
    let mut count = BLOCK.min(allowed);
    loop {
        let (read, capped) = terms(key, count, CELLS).ok()?;
        if let Some(edge) = ceiling_stop(&read) {
            return Some(read[..=edge].to_vec());
        }
        if capped || read.len() < count || count >= allowed {
            return Some(read);
        }
        count = (count * 2).min(allowed);
    }
}

fn census() -> Vec<Vec<u32>> {
    let width = WINDOW as usize + 1;
    let mut counts = vec![vec![0u32; width]; DEPTHS.len()];
    let mut rows = 0usize;
    for tier in Tier::ALL {
        for key in keys(tier) {
            rows += 1;
            let Some(window) = rendered(&key) else {
                continue;
            };
            for (slot, depth) in DEPTHS.iter().enumerate() {
                let head = &window[..window.len().min(*depth)];
                let mut written: Vec<usize> = head
                    .iter()
                    .filter(|&&term| (1..=WINDOW).contains(&term))
                    .map(|&term| term as usize)
                    .collect();
                written.sort_unstable();
                written.dedup();
                for value in written {
                    counts[slot][value] += 1;
                }
            }
        }
    }
    assert_eq!(rows, 18066);
    counts
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let counts = census();
    let missed: Vec<usize> = counts
        .iter()
        .map(|row| row[1..].iter().filter(|&&count| count == 0).count())
        .collect();
    assert!(missed.windows(2).all(|pair| pair[1] < pair[0]));
    assert!(missed[DEPTHS.len() - 1] > 0);
    let panels = Grid::new(area, 2, 2, 0.06);
    let mut drawn = 0usize;
    for (slot, tally) in counts.iter().enumerate() {
        let (x, y, w, h) = panels.cell(slot % 2, slot / 2);
        let cells = Grid::new(Frame::new(x, y, w, h), COLS, LINES, 0.10);
        for row in 0..LINES {
            for col in 0..COLS {
                let tone = match tally[row * COLS + col + 1] {
                    0 => ink::orange(),
                    1 => ink::dim(),
                    _ => ink::blue(),
                };
                cells.fill(&mut board, col, row, tone);
                drawn += 1;
            }
        }
    }
    assert_eq!(drawn, 4 * WINDOW as usize);
    save("demo-integers", &board)?;
    Ok(())
}
