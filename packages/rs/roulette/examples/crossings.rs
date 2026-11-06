// How many times does a spirograph curve cross itself? Count the crossings of the drawn curve, then compare with the formula in nodes().
// For one pencil the formula is a * (b - 1), where ring / wheel = a / b in lowest terms.
// cargo run -p mrly-roulette --example crossings
use mrly_roulette::crossings;
use mrly_roulette::{nodes, pencils, track};

fn main() {
    let tile = [0, 0, 0, 1, 0, 0, 0, 0, 0];
    let list = pencils(&tile, 3, 3, "fill", 0.9, 0.0, 1).unwrap();
    println!("ring wheel   a   b  counted  formula");
    for (ring, wheel) in [(5, 3), (7, 3), (8, 3), (7, 4), (9, 4), (11, 5)] {
        let path = track("in", ring, wheel, 0, 1).unwrap();
        let counted = crossings::nodes(&path, &list, 4000, 1e-4).unwrap();
        let formula = nodes(&path, &list, false).unwrap();
        println!(
            "{ring:>4} {wheel:>5} {:>3} {:>3} {:>8} {:>8}",
            path.ratio.0,
            path.ratio.1,
            counted.total(),
            formula
        );
    }
}
