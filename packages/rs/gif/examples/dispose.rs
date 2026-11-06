// The disposal method says what happens to a frame's box once the next frame is due.
// Frame 0 fills a 7 by 3 canvas with dots, frame 1 draws a box of x, frame 2 draws one o. Printed: what frame 2 looks like.
// cargo run -p mrly-gif --example dispose
use mrly_gif::{decode, encode};

fn main() -> Result<(), String> {
    let dots = vec![0u8; 21];
    let mut boxed = dots.clone();
    for at in [1, 2, 3, 8, 9, 10, 15, 16, 17] {
        boxed[at] = 1;
    }
    let mut last = boxed.clone();
    last[12] = 2;
    let palette = [[0, 0, 0, 255], [255, 0, 0, 255], [0, 0, 255, 255]];
    // encode() only sends what changed: frame 1 is the 3 by 3 box, frame 2 the single o.
    let file = encode(&[dots, boxed, last], 7, 3, &palette, &[10])?;

    // Each frame follows a control block: 21 f9 04, then a byte whose bits 2 to 4 hold the method.
    // encode() writes Keep, or Background where pixels must turn clear. Here frame 1's method is set by hand.
    let controls: Vec<usize> = (0..file.len() - 3)
        .filter(|&at| file[at..at + 3] == [0x21, 0xF9, 0x04])
        .collect();
    let names = ["Unset", "Keep", "Background", "Previous"];
    for (method, name) in names.iter().enumerate() {
        let mut bytes = file.clone();
        bytes[controls[1] + 3] = (method as u8) << 2;
        // decode() plays the frames onto one canvas; a cleared pixel has alpha 0.
        let (_, _, shown, _) = decode(&bytes)?;
        let glyph = |c: &[u8; 4]| match c {
            [_, _, _, 0] => ' ',
            [255, 0, 0, _] => 'x',
            [0, 0, 255, _] => 'o',
            _ => '.',
        };
        println!("{name}");
        for row in shown[2].chunks(7) {
            println!("  |{}|", row.iter().map(glyph).collect::<String>());
        }
    }
    println!();
    println!(
        "Unset and Keep leave the box, Background clears it, Previous puts back what was under it."
    );
    Ok(())
}
