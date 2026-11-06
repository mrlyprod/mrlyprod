// Conway's Life: a cell is born with 3 neighbours and survives with 2 or 3.
// The glider repeats its shape every 4 steps, one cell down and to the right. Printed: steps 0 to 4 on an 8 by 8 board that wraps.
// cargo run -p mrly-life --example glider
use mrly_life::{Boundary, Config, Counts, animate, moore};
use mrly_two::{from_strings, text};
use std::collections::HashMap;

fn main() {
    let seed = from_strings(&["0100".into(), "0010".into(), "1110".into(), "0000".into()]).unwrap();
    let mut config = Config::new(
        moore().unwrap(),
        Counts::list(vec![3]),
        Counts::list(vec![2, 3]),
    );
    config.boundary = Boundary::Wrap;
    config.padding = 2;
    let life = animate(&seed, &config).unwrap();
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let frames: Vec<Vec<String>> = life.grids[..5]
        .iter()
        .map(|g| text(g, Some(&glyphs)))
        .collect();
    for row in 0..frames[0].len() {
        let line: Vec<&str> = frames.iter().map(|f| f[row].as_str()).collect();
        println!("{}", line.join("  "));
    }
    println!();
    println!(
        "fate {}, back to the start after {} steps",
        life.fate.name(),
        life.loop_length
    );
}
