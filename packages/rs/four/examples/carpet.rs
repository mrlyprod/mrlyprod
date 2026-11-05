// Every mrly-two pattern is a mrly-four pattern: a filled square is 1111, a void is 0000.
// The grid keeps its size. "Scale up by four" is about the array: four entries per square,
// which is the mrly-two array Kronecker-multiplied by a 2x2 block of ones.
use mrly_four::{Cell4d, blocks, census, from_squares, raster, rect_svg, svg, text};
use mrly_math::two;
use std::collections::BTreeSet;
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const INK: [&str; 2] = ["#ffffff", "#000000"];

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

    // Pixel proof against the real mrly-two renderer, at several scales.
    for scale in [1, 2, 3, 8] {
        let image = two::image(&flat, scale, None, 0, two::Shape::Square)?;
        let ours = raster(&four, scale);
        let wrong = image
            .colors
            .iter()
            .zip(&ours)
            .filter(|&(rgba, &value)| (rgba[0] == 0) != (value != 0))
            .count();
        println!(
            "pixels at scale {scale}: {} compared, {wrong} differ",
            ours.len()
        );
    }

    // The array of N E / W S blocks is the mrly-two array times a 2x2 block of ones.
    let blown = two::text(&flat.combine(&two::ones(2, 1)?), None);
    println!(
        "four array == two kron 2x2 ones: {}",
        blown == blocks(&four)
    );

    // Fractal levels and quarter turns commute with the embedding.
    let grown = embed(&two::carpet(number, 1)?)?.fractal(level)?;
    println!(
        "embed(carpet level {level}) == embed(carpet level 1).fractal({level}): {}",
        grown == four
    );
    let chiral = two::from_strings(&[
        "11001".to_string(),
        "01000".to_string(),
        "01111".to_string(),
    ])?;
    let turned = embed(&chiral.clone().rotate(1)?)?;
    println!(
        "embed(turn(c)) == turn(embed(c)): {}",
        turned == embed(&chiral)?.rotate(1)
    );

    fs::create_dir_all(DATA_DIR)?;
    let small = two::carpet(number, 2)?;
    fs::write(
        format!("{DATA_DIR}/carpet5.svg"),
        svg(&embed(&small)?, 16, &INK)?,
    )?;
    fs::write(
        format!("{DATA_DIR}/carpet5_two.svg"),
        two::svg(&small, 16, None, 0, two::Shape::Square),
    )?;
    fs::write(
        format!("{DATA_DIR}/carpet5_grid.svg"),
        rect_svg(&embed(&small)?, 8, &INK)?,
    )?;
    Ok(())
}
