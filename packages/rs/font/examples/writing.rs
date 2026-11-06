// animate() lists the frames of a hand writing a text: each frame lights one more cell along the pen paths.
// Printed: 6 of the frames for "Hi", with one cell of padding.
// cargo run -p mrly-font --example writing
use mrly_font::animate;

fn main() {
    let anim = animate("Hi", 1);
    println!(
        "{} frames at {} per second, board {} by {}",
        anim.frames.len(),
        anim.fps,
        anim.rows,
        anim.cols
    );
    println!();
    let last = anim.frames.len() - 1;
    let shown: Vec<usize> = (0..6).map(|i| i * last / 5).collect();
    for row in 0..anim.rows {
        let line: Vec<String> = shown
            .iter()
            .map(|&f| {
                (0..anim.cols)
                    .map(|col| {
                        if anim.frames[f].contains(&(row * anim.cols + col)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();
        println!("{}", line.join("  "));
    }
}
