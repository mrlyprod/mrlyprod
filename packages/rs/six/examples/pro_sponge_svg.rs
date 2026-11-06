// The level 1 Menger sponge in the pro projection, a different way to flatten a cube onto triangles.
// Filled triangles are indigo; empty triangles are transparent.
// cargo run -p mrly-six --example pro_sponge_svg

use mrly_color::{ALPHA, INDIGO};
use mrly_six::{paint, pro, svg};
use mrly_three::carpet;
use std::collections::HashMap;

fn main() {
    let colors = HashMap::from([(0u8, vec![ALPHA]), (1u8, vec![INDIGO]), (2u8, vec![ALPHA])]);
    let flat = pro(&carpet(3, 1).unwrap()).unwrap();
    let flat = paint(flat, Some(&colors), None, None).unwrap();
    println!("{}", svg(&flat, 16, None, 0, None).unwrap());
}
