// The embedding of mrly-two into mrly-wang commutes with rendering, scaling, fractal levels and turns.
// cargo run -p mrly-wang --example embed_commutes
//
// "Scale up by four" is about the array: four entries per square,
// which is the mrly-two array Kronecker-multiplied by a 2x2 block of ones.
use mrly_two as two;
use mrly_wang::{CellWang, blocks, from_squares, raster};

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

    // Pixel proof against the real mrly-two renderer, at several scales.
    for scale in [1, 2, 3, 8] {
        let image = two::image(&flat, scale, None, 0, two::Shape::Square)?;
        let ours = raster(&wang, scale);
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
        "wang array == two kron 2x2 ones: {}",
        blown == blocks(&wang)
    );

    // Fractal levels and quarter turns commute with the embedding.
    let grown = embed(&two::carpet(number, 1)?)?.fractal(level)?;
    println!(
        "embed(carpet level {level}) == embed(carpet level 1).fractal({level}): {}",
        grown == wang
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

    Ok(())
}
