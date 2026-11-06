// Every mrly-two pattern is a mrly-wang pattern: a filled square is 1111, a void is 0000.
// This prints the carpet's counts in both crates.
// cargo run -p mrly-wang --example carpet
//
// The grid keeps its size; the array holds four entries per square.
use mrly_two as two;
use mrly_wang::{CellWang, census, from_squares, text};
use std::collections::BTreeSet;

fn embed(cell: &two::Cell2d) -> Result<CellWang, String> {
    let squares: Vec<u8> = (0..cell.types().size())
        .map(|flat| cell.types().at(flat) as u8)
        .collect();
    from_squares(cell.width(), cell.height(), &squares)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (number, level) = (5, 3);
    let flat = two::carpet(number, level)?;
    let wang = embed(&flat)?;
    let (w, h) = (flat.width(), flat.height());
    let found = census(&wang);

    println!("carpet{number} level {level}");
    println!(
        "grid   two {w}x{h}  wang {}x{}  (same size)",
        wang.width, wang.height
    );
    println!(
        "array  two {}  wang {:?} = {} entries (4x)",
        w * h,
        wang.shape(),
        wang.types.len()
    );
    let tags: BTreeSet<String> = text(&wang).concat().chars().map(String::from).collect();
    println!("tags   {tags:?}");
    println!(
        "area   two fills {}  wang fills/4 {}",
        two::fills(&flat),
        found.fills as f64 / 4.0
    );
    println!(
        "perim  two {}  wang straight {} diagonal {}",
        two::perimeter(&flat),
        found.straight,
        found.diagonal
    );
    println!("euler  two {}  wang {}", two::euler(&flat)?, found.euler);

    Ok(())
}
