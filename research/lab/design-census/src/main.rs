mod classes;
mod coprime;
mod fills;
mod quasi;
mod tables;

use std::path::Path;

fn main() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    coprime::report();
    println!();
    fills::orbits_report();
    println!();
    fills::report(&here.join("sequences.csv"));
    println!();
    classes::report(&here.join("counts.csv"));
}
