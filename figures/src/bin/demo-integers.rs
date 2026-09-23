use figures::out::root;
use figures::{ink, save, Board, Frame, Grid};
use ledger::{keys, terms, Cost, Key, Tier};
use mrlyrs::core::error::Result;
use mrlyrs::Error;
use std::path::PathBuf;

const NAME: &str = "demo-integers";
const CAP: usize = 48;
const CELLS: u128 = 100_000;
const WINDOW: i128 = 1_000;
const BLOCK: usize = 8;
const DEPTHS: [usize; 4] = [8, 16, 32, 48];
const COLS: usize = 40;
const LINES: usize = 25;

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

fn tally(rows: &[Key], lane: usize, lanes: usize) -> Vec<Vec<u32>> {
    let width = WINDOW as usize + 1;
    let mut counts = vec![vec![0u32; width]; DEPTHS.len()];
    for key in rows.iter().skip(lane).step_by(lanes) {
        let Some(window) = rendered(key) else {
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
    counts
}

fn census() -> Vec<Vec<u32>> {
    let rows: Vec<Key> = Tier::ALL.into_iter().flat_map(keys).collect();
    assert_eq!(rows.len(), 18066);
    let lanes = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let width = WINDOW as usize + 1;
    let mut counts = vec![vec![0u32; width]; DEPTHS.len()];
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..lanes)
            .map(|lane| {
                let rows = &rows;
                scope.spawn(move || tally(rows, lane, lanes))
            })
            .collect();
        for handle in handles {
            let part = handle.join().expect("a lane of the census");
            for (total, tier) in counts.iter_mut().zip(part) {
                for (sum, count) in total.iter_mut().zip(tier) {
                    *sum += count;
                }
            }
        }
    });
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

fn write_data(counts: &[Vec<u32>]) -> Result<PathBuf> {
    let file = path();
    let folder = file.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&folder)
        .map_err(|e| Error::Value(format!("cannot make {folder:?}: {e}")))?;
    let fields: Vec<String> = DEPTHS
        .iter()
        .zip(counts.iter())
        .map(|(depth, tally)| {
            let body: Vec<String> = tally.iter().map(|count| count.to_string()).collect();
            format!("\"d{depth}\":[{}]", body.join(","))
        })
        .collect();
    std::fs::write(&file, format!("{{{}}}", fields.join(",")))
        .map_err(|e| Error::Value(format!("cannot write {file:?}: {e}")))?;
    Ok(file)
}

fn field(text: &str, name: &str) -> Vec<u32> {
    let head = format!("\"{name}\":[");
    let Some(start) = text.find(&head) else {
        return Vec::new();
    };
    let body = &text[start + head.len()..];
    let end = body.find(']').unwrap_or(0);
    body[..end]
        .split(',')
        .filter_map(|token| token.parse().ok())
        .collect()
}

fn read_data() -> Result<Vec<Vec<u32>>> {
    let file = path();
    let raw = std::fs::read_to_string(&file)
        .map_err(|e| Error::Value(format!("cannot read {file:?}: {e}; run -- compute")))?;
    let text: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    Ok(DEPTHS
        .iter()
        .map(|depth| field(&text, &format!("d{depth}")))
        .collect())
}

// PRESS

fn compute() -> Result<()> {
    let counts = census();
    let missed: Vec<usize> = counts
        .iter()
        .map(|row| row[1..].iter().filter(|&&count| count == 0).count())
        .collect();
    assert!(missed.windows(2).all(|pair| pair[1] < pair[0]));
    assert!(missed[DEPTHS.len() - 1] > 0);
    let file = write_data(&counts)?;
    println!("{NAME} census {:?} missed {missed:?} -> {file:?}", DEPTHS);
    Ok(())
}

fn draw() -> Result<()> {
    let counts = read_data()?;
    assert_eq!(counts.len(), DEPTHS.len());
    assert!(counts
        .iter()
        .all(|tally| tally.len() == WINDOW as usize + 1));
    let mut board = Board::square();
    let area = board.frame(0.08);
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
    save(NAME, &board)?;
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("compute") {
        return compute();
    }
    draw()
}
