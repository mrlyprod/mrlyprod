use figures::ink::Ramp;
use figures::out::root;
use figures::{ink, save, Board, Grid};
use ledger::{keys, terms, Cost, Key, Tier};
use mrlyrs::core::error::Result;
use mrlyrs::Error;
use std::path::PathBuf;

const NAME: &str = "research-integers";
const CAP: usize = 48;
const CELLS: u128 = 100_000;
const CEILING: i128 = 100_000;
const WINDOW: usize = 10_000;
const SIDE: usize = 100;
const BLOCK: usize = 8;

// CENSUS

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

fn tally(rows: &[Key], lane: usize, lanes: usize) -> Vec<u32> {
    let mut counts = vec![0u32; WINDOW + 1];
    for key in rows.iter().skip(lane).step_by(lanes) {
        let Some(window) = rendered(key) else {
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
    counts
}

fn census() -> Vec<u32> {
    let rows: Vec<Key> = Tier::ALL.into_iter().flat_map(keys).collect();
    assert_eq!(rows.len(), 18066);
    let lanes = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let mut counts = vec![0u32; WINDOW + 1];
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..lanes)
            .map(|lane| {
                let rows = &rows;
                scope.spawn(move || tally(rows, lane, lanes))
            })
            .collect();
        for handle in handles {
            let part = handle.join().expect("a lane of the census");
            for (total, count) in counts.iter_mut().zip(part) {
                *total += count;
            }
        }
    });
    let never = counts[1..].iter().filter(|&&c| c == 0).count();
    let once = counts[1..].iter().filter(|&&c| c == 1).count();
    let many = counts[1..].iter().filter(|&&c| c > 1).count();
    assert_eq!((never, once, many), (3589, 765, 5646));
    counts
}

// FILE

fn path() -> PathBuf {
    root()
        .join("files")
        .join("figures")
        .join("census")
        .join(format!("{NAME}.json"))
}

fn write_data(counts: &[u32]) -> Result<PathBuf> {
    let file = path();
    let folder = file.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&folder)
        .map_err(|e| Error::Value(format!("cannot make {folder:?}: {e}")))?;
    let body: Vec<String> = counts.iter().map(|count| count.to_string()).collect();
    std::fs::write(&file, format!("[{}]", body.join(",")))
        .map_err(|e| Error::Value(format!("cannot write {file:?}: {e}")))?;
    Ok(file)
}

fn read_data() -> Result<Vec<u32>> {
    let file = path();
    let text = std::fs::read_to_string(&file)
        .map_err(|e| Error::Value(format!("cannot read {file:?}: {e}; run -- compute")))?;
    Ok(text
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .filter_map(|token| token.trim().parse().ok())
        .collect())
}

// PRESS

fn compute() -> Result<()> {
    let counts = census();
    let peak = *counts.iter().max().unwrap();
    assert_eq!(peak, counts[16]);
    let file = write_data(&counts)?;
    println!(
        "{NAME} census {} cells peak {peak} -> {file:?}",
        counts.len()
    );
    Ok(())
}

fn draw() -> Result<()> {
    let counts = read_data()?;
    assert_eq!(counts.len(), WINDOW + 1);
    let peak = *counts.iter().max().unwrap() as f64;
    assert!(peak > 0.0);
    let mut board = Board::square();
    let area = board.frame(0.08);
    let ramp = Ramp::tone(ink::dim(), ink::yellow());
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
    save(NAME, &board)?;
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("compute") {
        return compute();
    }
    draw()
}
