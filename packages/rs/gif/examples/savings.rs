// What sending only the changes saves on a run of Conway's Life.
// Most of a Life board stays put from one step to the next, so most of each frame need not be sent again.
// cargo run -p mrly-gif --example savings
use mrly_gif::{encode, scale};

// One step of Life on a square board that wraps: a cell lives with 3 neighbors, or with 2 if it was alive.
fn step(board: &[u8], side: usize) -> Vec<u8> {
    (0..side * side)
        .map(|i| {
            let (x, y) = (i % side, i / side);
            let mut near = 0;
            for dy in [side - 1, 0, 1] {
                for dx in [side - 1, 0, 1] {
                    near += board[(y + dy) % side * side + (x + dx) % side];
                }
            }
            near -= board[i];
            (near == 3 || near == 2 && board[i] == 1) as u8
        })
        .collect()
}

fn main() -> Result<(), String> {
    // The R-pentomino: five cells that churn for over a thousand steps.
    let side = 63;
    let mut board = vec![0u8; side * side];
    for (x, y) in [(31, 30), (32, 30), (30, 31), (31, 31), (31, 32)] {
        board[y * side + x] = 1;
    }
    // 150 steps, each cell drawn as a 3 by 3 block of pixels.
    let mut frames = Vec::new();
    for _ in 0..150 {
        frames.push(scale(&board, side, 3)?);
        board = step(&board, side);
    }
    let width = side * 3;
    let palette = [[0, 0, 0, 255], [255, 255, 255, 255]];
    println!(
        "{} frames of {width} by {width}, {} pixels each",
        frames.len(),
        width * width
    );

    // Every frame sent whole: each one saved as its own still picture.
    let mut stills = 0;
    for frame in &frames {
        stills += encode(std::slice::from_ref(frame), width, width, &palette, &[0])?.len();
    }
    // One animation: encode() crops each frame to the box that changed, and inside the box
    // marks unchanged pixels transparent when that compresses better.
    let movie = encode(&frames, width, width, &palette, &[8])?;

    println!();
    println!("as stills, every frame whole    {stills:>7} bytes");
    println!("one animation, changes only     {:>7} bytes", movie.len());
    println!(
        "the animation is {:.0}% of the stills",
        100.0 * movie.len() as f64 / stills as f64
    );
    Ok(())
}
