// Arcs on a torus: the tiles 6c / c6 close every strand into a full circle.
// With wrap on, strands that cross the edge are drawn again on the other side, so the patch tiles seamlessly.
// cargo run -p mrly-wang --example arcs_rings
use mrly_wang::arcs::svg;
use mrly_wang::from_strings;

const COLORS: [&str; 2] = ["#fff6e5", "#1f2a44"];

fn main() -> Result<(), String> {
    let tile = from_strings(&["6c", "c6"])?.tile(4, 4);
    println!("{}", svg(&tile, 32, 8, COLORS, true)?);
    Ok(())
}
