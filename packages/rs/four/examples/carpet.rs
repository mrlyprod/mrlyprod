// Every mrly-two pattern is a mrly-four pattern: a filled square is 1111, a void is 0000.
// This prints the carpet's counts in both crates.
// cargo run -p mrly-four --example carpet
//
// The grid keeps its size; the array holds four entries per square.
use mrly_four::{Cell4d, census, from_squares, text};
use mrly_two as two;
use std::collections::BTreeSet;

fn embed(cell: &two::Cell2d) -> Result<Cell4d, String> {
    let squares: Vec<u8> = (0..cell.types().size())
        .map(|flat| cell.types().at(flat) as u8)
        .collect();
    from_squares(cell.width(), cell.height(), &squares)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (number, level) = (5, 3);
    let flat = two::carpet(number, level)?;
    let four = embed(&flat)?;
    let (w, h) = (flat.width(), flat.height());
    let found = census(&four);

    println!("carpet{number} level {level}");
    println!(
        "grid   two {w}x{h}  four {}x{}  (same size)",
        four.width, four.height
    );
    println!(
        "array  two {}  four {:?} = {} entries (4x)",
        w * h,
        four.shape(),
        four.types.len()
    );
    let tags: BTreeSet<String> = text(&four).concat().chars().map(String::from).collect();
    println!("tags   {tags:?}");
    println!(
        "area   two fills {}  four fills/4 {}",
        two::fills(&flat),
        found.fills as f64 / 4.0
    );
    println!(
        "perim  two {}  four straight {} diagonal {}",
        two::perimeter(&flat),
        found.straight,
        found.diagonal
    );
    println!("euler  two {}  four {}", two::euler(&flat)?, found.euler);

    Ok(())
}
