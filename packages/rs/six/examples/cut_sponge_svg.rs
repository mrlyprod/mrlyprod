// The level 1 Menger sponge in the cut projection.
// Filled triangles are teal; empty triangles are transparent.
// cargo run -p mrly-six --example cut_sponge_svg

use mrly_color::{ALPHA, TEAL};
use mrly_six::{cut, paint, svg};
use mrly_three::carpet;
use std::collections::HashMap;

fn main() {
    let colors = HashMap::from([(0u8, vec![ALPHA]), (1u8, vec![TEAL]), (2u8, vec![ALPHA])]);
    let flat = cut(&carpet(3, 1).unwrap()).unwrap();
    let flat = paint(flat, Some(&colors), None, None).unwrap();
    println!("{}", svg(&flat, 16, None, 0, None).unwrap());
}
