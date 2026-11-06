// Crop animation frames to the box that holds every frame's ink, padded to a square so the picture does not jump between frames.
// Three 14 by 14 frames show a small plus moving across the canvas; crop() trims all three by the same amount, and tessellate() tiles small frames up to a minimum canvas.
// cargo run -p mrly-crop --example frames
use mrly_crop::frames::{crop, tessellate};
use mrly_tensor::Tensor;
use mrly_two::{Cell2d, text};

fn plus(side: usize, row: usize, col: usize) -> Cell2d {
    let mut grid = Tensor::new(vec![side, side]);
    for (dr, dc) in [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)] {
        grid.put((row + dr) * side + col + dc, 1);
    }
    Cell2d::new(grid).unwrap()
}

fn side_by_side(frames: &[Cell2d]) {
    let pictures: Vec<Vec<String>> = frames.iter().map(|f| text(f, None)).collect();
    for row in 0..pictures[0].len() {
        let parts: Vec<&str> = pictures.iter().map(|p| p[row].as_str()).collect();
        println!("{}", parts.join(" | "));
    }
}

fn main() {
    let frames = vec![plus(14, 2, 1), plus(14, 4, 4), plus(14, 6, 9)];
    println!("original {}x{}", frames[0].width(), frames[0].height());
    side_by_side(&frames);
    let cropped = crop(&frames).unwrap();
    println!("cropped {}x{}", cropped[0].width(), cropped[0].height());
    side_by_side(&cropped);
    let tiled = tessellate(&cropped, 16).unwrap();
    println!(
        "tessellated to at least 16: {}x{}",
        tiled[0].width(),
        tiled[0].height()
    );
}
