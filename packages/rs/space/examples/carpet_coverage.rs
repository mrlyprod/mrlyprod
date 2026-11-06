// Seen straight on, the Menger sponge is a Sierpinski carpet: each level keeps 8 of every 9 squares.
// At 81 by 81 pixels the smallest level 3 hole is 3 pixels wide, so the covered pixels must be exactly
// 81 * 81 * (8/9)^level. Any gap or double count along the face edges would break the equality.
// cargo run -p mrly-space --example carpet_coverage
use mrly_color::ALPHA;
use mrly_space::{View, render};
use mrly_three::carpet;

fn main() {
    // Zoom by the square root of 3 so the cube's face, not its corners, fills the frame.
    let view = View {
        width: 81,
        height: 81,
        yaw: 0.0,
        pitch: 0.0,
        zoom: 3f64.sqrt(),
        background: ALPHA,
        ..View::default()
    };
    println!("level  covered  expected");
    for level in 1..=3 {
        let image = render(&carpet(3, level).unwrap(), &view).unwrap();
        let covered = image.colors.iter().filter(|c| c[3] == 255).count();
        let expected = 81 * 81 * 8usize.pow(level as u32) / 9usize.pow(level as u32);
        println!("{level:>5} {covered:>8} {expected:>9}");
    }
}
