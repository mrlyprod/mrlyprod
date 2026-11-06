// A Cell3d has one frame: the first axis runs left to right, the second runs up, the third runs from the back
// to the front. from_strings reads planes from the back to the front, each plane a picture with its top row first.
// A letter F cannot hide a flip or a mirror, so this types one in as a picture and reads it back three ways:
// as text, from the quads that face the front, and from the corners the OBJ text writes.
// cargo run -p mrly-three --example frame
use mrly_three::{from_strings, quads, text, to_obj};

const LETTER: [&str; 7] = [
    "0111110", "0100000", "0100000", "0111100", "0100000", "0100000", "0100000",
];

fn letter() -> Vec<Vec<String>> {
    (0..LETTER.len())
        .map(|z| {
            let blank = "0".repeat(LETTER.len());
            let row = |y: usize| {
                if z == 3 {
                    LETTER[y].to_string()
                } else {
                    blank.clone()
                }
            };
            (0..LETTER.len()).map(row).collect()
        })
        .collect()
}

fn main() {
    let cell = from_strings(&letter()).unwrap();
    let side = LETTER.len();

    println!("the middle plane as text:");
    for row in &text(&cell, None)[3 * side..4 * side] {
        println!("{row}");
    }

    // Quads live in a cube from -1 to 1; this turns a coordinate back into a cell index.
    let half = side as f32 / 2.0;
    let index = |low: f32| (low * half + half).round() as usize;
    let mut seen = vec![vec![false; side]; side];
    for quad in quads(&cell).iter().filter(|quad| quad.normal.z > 0.0) {
        let x = quad.verts.iter().map(|v| v.x).fold(f32::MAX, f32::min);
        let y = quad.verts.iter().map(|v| v.y).fold(f32::MAX, f32::min);
        seen[index(y)][index(x)] = true;
    }
    println!();
    println!("faces that point to the front, top row first:");
    for row in seen.iter().rev() {
        let line: String = row.iter().map(|&on| if on { '#' } else { '.' }).collect();
        println!("{line}");
    }

    // The OBJ text writes the same frame in whole cells: x to the right, y up, z to the front.
    let mut spans = [(f64::MAX, f64::MIN); 3];
    for line in to_obj(&cell).lines().filter(|line| line.starts_with("v ")) {
        for (span, word) in spans.iter_mut().zip(line.split_whitespace().skip(1)) {
            let value: f64 = word.parse().unwrap();
            *span = (span.0.min(value), span.1.max(value));
        }
    }
    println!();
    for (axis, (low, high)) in ["x", "y", "z"].iter().zip(spans) {
        println!("obj {axis} runs from {low} to {high}");
    }
    println!(
        "width {}, height {}, depth {}",
        cell.width(),
        cell.height(),
        cell.depth()
    );
}
