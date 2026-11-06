// Turning the camera a quarter turn gives the same pixels as turning the cell itself a quarter turn.
// The angle maths is exact at multiples of 90 degrees, so the two pictures match pixel for pixel.
// Turning the cell the wrong way gives a different picture, so the match is not luck.
// cargo run -p mrly-space --example quarter_turns
use mrly_color::{BLUE, GREEN, ORANGE, RED, YELLOW};
use mrly_math::Mode;
use mrly_space::{View, render};
use mrly_three::{Cell3d, carpet, paint};
use std::collections::HashMap;

fn matching(cell: &Cell3d, k: usize, axes: (usize, usize), view: &View) -> usize {
    let turned = render(&cell.clone().rotate(k, axes).unwrap(), view).unwrap();
    let yawed = View {
        yaw: view.yaw + 90.0 * k as f64,
        ..*view
    };
    let camera = render(cell, &yawed).unwrap();
    turned
        .colors
        .iter()
        .zip(&camera.colors)
        .filter(|(a, b)| a == b)
        .count()
}

fn main() {
    // Five colours dealt out cube by cube, so no turn or mirror image of the sponge looks the same.
    let inks = HashMap::from([(1, vec![RED, ORANGE, YELLOW, GREEN, BLUE])]);
    let sponge = paint(carpet(3, 1).unwrap(), Some(&inks), Some(Mode::Index), None).unwrap();
    let view = View {
        width: 48,
        height: 48,
        ..View::default()
    };
    let total = view.width * view.height;
    println!("yaw + 90k against the cell turned k times in the plane of axes 0 and 2");
    for k in 0..4 {
        let same = matching(&sponge, k, (0, 2), &view);
        println!("k {k}: {same} of {total} pixels match");
    }
    let wrong = matching(&sponge, 1, (2, 0), &view);
    println!("turned the wrong way: {wrong} of {total} pixels match");
}
