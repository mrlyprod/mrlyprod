// A cell that carries its own colours is drawn in them; one without colours is drawn in the view's ink.
// Here the level 1 sponge is painted with Mode::Depth, one colour per slab, then shaded by the same light.
// cargo run -p mrly-space --example painted
use mrly_color::{BLUE, ORANGE, PINK};
use mrly_image::svg;
use mrly_math::Mode;
use mrly_space::{View, render};
use mrly_three::{carpet, paint};
use std::collections::HashMap;

fn main() {
    let sponge = carpet(3, 1).unwrap();
    let inks = HashMap::from([(1, vec![PINK, ORANGE, BLUE])]);
    let painted = paint(sponge.clone(), Some(&inks), Some(Mode::Depth), None).unwrap();
    let view = View {
        width: 48,
        height: 48,
        samples: 2,
        ..View::default()
    };
    let plain = render(&sponge, &view).unwrap();
    let colored = render(&painted, &view).unwrap();
    println!("{}", svg(&[plain, colored], 4, 4));
}
