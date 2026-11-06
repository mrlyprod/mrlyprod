// A chaos game on a square picks a corner (0 to 3) at each step and jumps halfway towards it. After k steps the point sits in one of 4^k cells, decided by the last k corners.
// A de Bruijn cycle holds every word of k corners exactly once, so playing it visits every cell exactly once.
// The grid shows the step on which each cell is first reached (level 3: 64 cells, 64 steps). word_cell() turns a word into a cell index.
// cargo run -p mrly-chaos --example de_bruijn_cells
use mrly_chaos::{de_bruijn, word_cell};
use std::collections::HashSet;

fn main() {
    let (letters, level) = (4u32, 3usize);
    let cycle = de_bruijn(letters, level as u32).unwrap();
    println!("cycle {:?}", cycle);
    let side = 1usize << level;
    let mut first = vec![0usize; side * side];
    let mut seen = HashSet::new();
    for step in 0..cycle.len() {
        let word: Vec<u8> = (0..level)
            .map(|j| cycle[(step + j) % cycle.len()])
            .collect();
        let cell = word_cell(&word, 2, 2).unwrap() as usize;
        if seen.insert(cell) {
            first[cell] = step + 1;
        }
    }
    println!("{} steps, {} distinct cells", cycle.len(), seen.len());
    for row in first.chunks(side) {
        let line: Vec<String> = row.iter().map(|n| format!("{n:>2}")).collect();
        println!("{}", line.join(" "));
    }
}
