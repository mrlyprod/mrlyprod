use mrlycore::errors::Result;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save, Board, Grid};
use mrlylab::ledger::{keys, terms, Cost, Key, Tier};

const CAP: usize = 48;
const CELLS: u128 = 100_000;
const CEILING: i128 = 100_000;
const WINDOW: usize = 10_000;
const SIDE: usize = 100;
const BLOCK: usize = 8;

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
        if term > CEILING {
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

fn census() -> Vec<u32> {
    let mut counts = vec![0u32; WINDOW + 1];
    let mut rows = 0usize;
    for tier in Tier::ALL {
        for key in keys(tier) {
            rows += 1;
            let Some(window) = rendered(&key) else {
                continue;
            };
            let mut written: Vec<usize> = window
                .iter()
                .filter(|&&term| term >= 1 && term <= WINDOW as i128)
                .map(|&term| term as usize)
                .collect();
            written.sort_unstable();
            written.dedup();
            for value in written {
                counts[value] += 1;
            }
        }
    }
    assert_eq!(rows, 18066);
    let never = counts[1..].iter().filter(|&&c| c == 0).count();
    let once = counts[1..].iter().filter(|&&c| c == 1).count();
    let many = counts[1..].iter().filter(|&&c| c > 1).count();
    assert_eq!((never, once, many), (3589, 765, 5646));
    counts
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let counts = census();
    let peak = *counts.iter().max().unwrap() as f64;
    assert_eq!(peak as u32, counts[16]);
    let ramp = Ramp::tone(ink::DIM, ink::GOLD);
    let grid = Grid::new(area, SIDE, SIDE, 0.12);
    let scale = (1.0 + peak).ln();
    for row in 0..SIDE {
        for col in 0..SIDE {
            let count = counts[row * SIDE + col + 1];
            if count == 0 {
                continue;
            }
            let tone = ramp.at((1.0 + count as f64).ln() / scale);
            grid.fill(&mut board, col, row, tone);
        }
    }
    save("research-integers", &board)?;
    Ok(())
}
