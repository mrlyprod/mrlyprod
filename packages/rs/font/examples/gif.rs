// gif() renders an animation as a GIF: each lit cell a block of black ink on white paper.
// Printed: "Hi" written, merged and written back, and how its frames and delays come out.
// cargo run -p mrly-font --example gif
use mrly_font::{HOLD, animate, cycle, gif, merge};
use mrly_gif::decode;

fn main() -> Result<(), String> {
    let write = animate("Hi", 1);
    let anim = cycle(&write, &merge("Hi", 1), HOLD);
    // Each cell becomes a 6 by 6 block of pixels.
    let bytes = gif(&anim, 6)?;
    let (width, height, shown, delays) = decode(&bytes)?;
    println!(
        "{} frames at {} a second: a {width} by {height} gif of {} bytes",
        anim.frames.len(),
        anim.fps,
        bytes.len()
    );

    // At 25 frames a second each frame lasts 4 hundredths. A frame held still for a while
    // becomes one GIF frame with a longer delay, so the holds cost nothing.
    let total: u32 = delays.iter().map(|&d| d as u32).sum();
    println!(
        "{} gif frames, {:.2} seconds in all",
        shown.len(),
        total as f64 / 100.0
    );
    let held: Vec<u16> = delays.iter().copied().filter(|&d| d > 4).collect();
    println!("held frames last {held:?} hundredths");
    Ok(())
}
