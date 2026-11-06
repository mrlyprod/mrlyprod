// The distinct 2D shapes. Each line is the smallest code of its family.
// orbit is how many codes share that shape, anf is the code as an XOR formula, degree its largest term.
// cargo run -p mrly-bang --example shapes_2d

use mrly_bang::bang;

fn main() {
    let universe = bang(2).unwrap();
    for design in universe.canonical() {
        println!(
            "code {:>2} orbit {} degree {:>2} anf {}",
            design.i.get(),
            design.orbit_size,
            design.degree(),
            design.anf()
        );
    }
}
