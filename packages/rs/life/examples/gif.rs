// gif() saves a Life run as an animated GIF: one frame a step, black for dead and white for alive.
// Printed: a glider's run on a board that wraps, and a check that every frame reads back as frames() draws it.
// cargo run -p mrly-life --example gif
use mrly_life::{Boundary, Config, Counts, animate, frames, gif, moore};
use mrly_two::from_strings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = from_strings(&["010".into(), "001".into(), "111".into()])?;
    let mut config = Config::new(moore()?, Counts::list(vec![3]), Counts::list(vec![2, 3]));
    config.boundary = Boundary::Wrap;
    config.padding = 5;
    let life = animate(&seed, &config)?;

    // Each cell is drawn 8 pixels wide, and each step shows for 10 hundredths of a second.
    let bytes = gif(&life.grids, 8, 10)?;
    let (width, height, shown, delays) = mrly_gif::decode(&bytes)?;
    println!(
        "{} steps, fate {}: a {width} by {height} gif of {} bytes",
        life.count,
        life.fate.name(),
        bytes.len()
    );
    println!(
        "{} frames, the first delays {:?}",
        shown.len(),
        &delays[..3]
    );

    let drawn = frames(&life.grids, 8)?;
    let same = shown.len() == drawn.len() && shown.iter().zip(&drawn).all(|(a, b)| *a == b.colors);
    println!("every frame reads back as drawn: {same}");
    Ok(())
}
