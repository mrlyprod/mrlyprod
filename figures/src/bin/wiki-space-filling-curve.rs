use figures::{ink, save, Board, Ramp};
use mrlyrs::core::error::Result;
use std::collections::HashSet;

const LEVEL: usize = 5;
const SIDE: i64 = 32;
const CELLS: usize = 1024;
const STEPS: usize = 1023;
const MARGIN: f64 = 0.08;
const WEIGHT: f64 = 0.42;

fn rule(c: char) -> &'static str {
    match c {
        'A' => "+BF-AFA-FB+",
        'B' => "-AF+BFB+FA-",
        _ => "",
    }
}

fn expand(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'A' | 'B' => rule(c).to_string(),
            _ => c.to_string(),
        })
        .collect()
}

fn main() -> Result<()> {
    let mut word = String::from("A");
    for _ in 0..LEVEL {
        word = expand(&word);
    }
    let heads = [(1i64, 0i64), (0, 1), (-1, 0), (0, -1)];
    let mut head = 0usize;
    let mut pts = vec![(0i64, 0i64)];
    for c in word.chars() {
        match c {
            '+' => head = (head + 1) % 4,
            '-' => head = (head + 3) % 4,
            'F' => {
                let (x, y) = pts[pts.len() - 1];
                pts.push((x + heads[head].0, y + heads[head].1));
            }
            _ => {}
        }
    }
    assert_eq!(pts.len() - 1, STEPS);
    let x0 = pts.iter().map(|p| p.0).min().unwrap_or(0);
    let y0 = pts.iter().map(|p| p.1).min().unwrap_or(0);
    let x1 = pts.iter().map(|p| p.0).max().unwrap_or(0);
    let y1 = pts.iter().map(|p| p.1).max().unwrap_or(0);
    assert_eq!((x1 - x0 + 1, y1 - y0 + 1), (SIDE, SIDE));
    let seen: HashSet<(i64, i64)> = pts.iter().copied().collect();
    assert_eq!(seen.len(), CELLS);
    let mut board = Board::square();
    let area = board.frame(MARGIN);
    let cell = area.w / SIDE as f64;
    let at = |(x, y): (i64, i64)| {
        (
            area.x + ((x - x0) as f64 + 0.5) * cell,
            area.y + ((y1 - y) as f64 + 0.5) * cell,
        )
    };
    let ramp = Ramp::tone(ink::blue(), ink::yellow());
    for i in 0..STEPS {
        let t = i as f64 / (STEPS - 1) as f64;
        board.segment(at(pts[i]), at(pts[i + 1]), WEIGHT * cell, ramp.at(t));
    }
    save("wiki-space-filling-curve", &board)?;
    Ok(())
}
