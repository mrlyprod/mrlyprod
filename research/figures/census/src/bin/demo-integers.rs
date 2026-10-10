use census::{integers, save};
use mrlyrs::core::error::Result;
use mrlyrs::core::{Json, Map};

const NAME: &str = "demo-integers";
const WINDOW: usize = 1_000;
const DEPTHS: [usize; 4] = [8, 16, 32, 48];

fn main() -> Result<()> {
    let counts = integers::census(WINDOW as i128, WINDOW, &DEPTHS);
    let missed: Vec<usize> = counts
        .iter()
        .map(|row| row[1..].iter().filter(|&&count| count == 0).count())
        .collect();
    assert!(missed.windows(2).all(|pair| pair[1] < pair[0]));
    assert!(missed[DEPTHS.len() - 1] > 0);
    let mut body = Map::new();
    for (depth, tally) in DEPTHS.iter().zip(counts) {
        body.insert(format!("d{depth}"), Json::from(tally));
    }
    save(NAME, &Json::Object(body))
}
