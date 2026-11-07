// Conway's Life: a cell is born with 3 neighbours and survives with 2 or 3.
// The glider repeats its shape every 4 steps, one cell down and to the right. Printed: steps 0 to 4 on an 8 by 8 wallpaper,
// a board that wraps at its edges. tick() takes one step of any board; a Run steps on and says how it ends.
// gif() saves the run as an animated GIF: one frame a step, black for dead and white for alive.
// cargo run -p mrly-life --example glider
use mrly_life::{Mode, Reel, Run, frames, gif, moore, tick};
use mrly_two::{from_strings, text};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed =
        from_strings(&["0100".into(), "0010".into(), "1110".into(), "0000".into()])?.pad(2, 0);
    let glyphs = HashMap::from([(0u8, ".".to_string()), (1u8, "#".to_string())]);
    let mut shown = vec![text(&seed, Some(&glyphs))];
    let mut cell = seed.clone();
    for _ in 0..4 {
        cell = tick(&cell, &moore(), &[3], &[2, 3], Mode::Wallpaper)?;
        shown.push(text(&cell, Some(&glyphs)));
    }
    for row in 0..shown[0].len() {
        let line: Vec<&str> = shown.iter().map(|f| f[row].as_str()).collect();
        println!("{}", line.join("  "));
    }
    println!();

    let mut run = Run::of(&seed, &moore(), &[3], &[2, 3], Mode::Wallpaper, 0, 100)?;
    let mut reel = Reel::new();
    reel.shoot(&run);
    let stop = reel.roll(&mut run);
    println!(
        "{} after {} steps, period {}, counted the {} way",
        stop.kind,
        stop.generation,
        stop.period,
        run.plan()
    );

    // Each cell is drawn 8 pixels wide, and each step shows for 10 hundredths of a second.
    let grids = reel.frames()?;
    let bytes = gif(&grids, 8, 10)?;
    let (width, height, read, _) = mrly_gif::decode(&bytes)?;
    let drawn = frames(&grids, 8)?;
    let same = read.len() == drawn.len() && read.iter().zip(&drawn).all(|(a, b)| *a == b.colors);
    println!(
        "a {width} by {height} gif of {} frames and {} bytes, every frame as drawn: {same}",
        read.len(),
        bytes.len()
    );
    Ok(())
}
