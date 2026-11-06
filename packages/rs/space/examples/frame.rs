// The camera follows the frame of a Cell3d: the first axis runs left to right, the second runs up, the third
// runs from the back to the front. A letter F cannot hide a flip or a mirror. From the front it reads as an F;
// a half turn shows its back, mirrored as the back of a letter should be; the last view is from the front-right
// and above.
// cargo run -p mrly-space --example frame
use mrly_image::svg;
use mrly_space::{View, render};
use mrly_three::from_strings;

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
    let frames: Vec<_> = [(0.0, 0.0), (180.0, 0.0), (30.0, 30.0)]
        .iter()
        .map(|&(yaw, pitch)| {
            let view = View {
                width: 32,
                height: 32,
                yaw,
                pitch,
                samples: 2,
                ..View::default()
            };
            render(&cell, &view).unwrap()
        })
        .collect();
    println!("{}", svg(&frames, 4, 4));
}
