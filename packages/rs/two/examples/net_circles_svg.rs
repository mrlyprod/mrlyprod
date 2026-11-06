// The net design at level 3 drawn with circles instead of squares.
// Shape picks the mark: Square, Circle or Diamond.
// cargo run -p mrly-two --example net_circles_svg

use mrly_color::{ALPHA, TEAL};
use mrly_two::{Shape, net, paint, svg};
use std::collections::HashMap;

fn main() {
    let colors = HashMap::from([(0u8, vec![ALPHA]), (1u8, vec![TEAL])]);
    let cell = paint(net(3, 3).unwrap(), Some(&colors), None, None).unwrap();
    println!("{}", svg(&cell, 10, None, 0, Shape::Circle));
}
