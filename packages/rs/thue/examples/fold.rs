// A grid "folds" when it is a tile raised to a power: the tile repeated inside itself at every scale.
// The Thue-Morse parity lift is the 2x2 tile [0 1; 1 0] raised to the third power at side 8.
// A grid that is not a tile power reports its first fault.
// cargo run -p mrly-thue --example fold
use mrly_thue::{Lift, fold, lift, power};

fn main() {
    let side = 8;
    let grid = lift(Lift::Parity, side);
    let found = fold(&grid, side, 2).unwrap();
    println!("parity lift, side 8, tile side 2");
    println!("  tile {:?}", found.tile);
    println!(
        "  level {}, folds {}, faults {}",
        found.level, found.folds, found.faults
    );
    println!();
    let mut broken = power(&found.tile, 2, 3).unwrap();
    broken[27] ^= 1;
    let found = fold(&broken, side, 2).unwrap();
    println!("same grid with cell 27 flipped");
    println!(
        "  folds {}, faults {}, first fault at {:?}",
        found.folds, found.faults, found.first
    );
}
