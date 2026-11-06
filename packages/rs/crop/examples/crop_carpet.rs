// Crop a Sierpinski carpet (level 3, 27 by 27) to an octagon. Only cells that lie wholly inside the shape survive; keep_cut = true also keeps the cells the edge passes through.
// '#' is a filled cell kept, '.' is a filled cell cropped away, ' ' is a hole of the carpet.
// cargo run -p mrly-crop --example crop_carpet
use mrly_crop::{Frac, census, crop, named};
use mrly_two::carpet;

fn main() {
    let grid = carpet(3, 3).unwrap();
    let types = grid.types();
    let shape = named("octagon", 2, Frac::new(1, 3).unwrap()).unwrap();
    let kept = crop(types, &shape, false).unwrap();
    let found = census(&shape, types).unwrap();
    println!(
        "filled before {}  after {}  cells by region (out, cut, in) {:?}  filled by region {:?}",
        types.sum(),
        kept.sum(),
        found.cells,
        found.filled
    );
    let side = types.shape[0];
    for row in 0..side {
        let line: String = (0..side)
            .map(|col| {
                let at = row * side + col;
                match (types.at(at) != 0, kept.at(at) != 0) {
                    (true, true) => "##",
                    (true, false) => "..",
                    _ => "  ",
                }
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
