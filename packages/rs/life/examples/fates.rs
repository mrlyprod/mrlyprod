// Every run of Life ends one way: dead, still (a picture that stays), loop (it repeats), mover (it repeats
// somewhere else), wall (an island grew into its cap), or limit (it ran out of generations).
// Five small starts under Conway's rule on three kinds of world: a board with dead cells all round,
// a wallpaper that wraps at its edges, and an island on the open plane.
// Entropy is how mixed the first picture is: 0 for all one colour, 1000 for half and half.
// Churn is the share of cells that change per step, averaged over the run.
// cargo run -p mrly-life --example fates
use mrly_life::{Mode, Reel, Run, churn, entropy, moore};
use mrly_two::from_strings;

fn run(name: &str, rows: &[&str], mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let rows: Vec<String> = rows.iter().map(|r| r.to_string()).collect();
    let seed = from_strings(&rows)?.pad(2, 0);
    let mut life = Run::of(&seed, &moore(), &[3], &[2, 3], mode, 40, 64)?;
    let mut reel = Reel::new();
    reel.shoot(&life);
    let stop = reel.roll(&mut life);
    let frames = reel.frames()?;
    println!(
        "{name:<22} {:<9} {:<6} at {:>2}  period {:>2}  moved {:>2},{:>2}  entropy {:>3}  churn {:.4}",
        mode.name(),
        stop.kind,
        stop.generation,
        stop.period,
        stop.dr,
        stop.dc,
        entropy(&frames[0]),
        churn(&frames)
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let glider = ["0100", "0010", "1110", "0000"];
    run("one cell", &["1"], Mode::Board)?;
    run("block", &["11", "11"], Mode::Board)?;
    run("blinker", &["111"], Mode::Board)?;
    run("glider against a wall", &glider, Mode::Board)?;
    run("glider on a wallpaper", &glider, Mode::Wallpaper)?;
    run("glider on an island", &glider, Mode::Island)?;
    Ok(())
}
