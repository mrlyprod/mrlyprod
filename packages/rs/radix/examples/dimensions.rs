// The five named radix fractals. A base b with k digits makes a fractal of dimension 2 ln(k) / ln(norm b).
// When k equals the norm the digits tile the plane (dimension 2). Distinct points can fall below k^level if pieces overlap.
// cargo run -p mrly-radix --example dimensions
use mrly_radix::{flowsnake, gasket, koch, terdragon, twindragon};

fn main() {
    let level = 4;
    println!(
        "{:<11} {:>5} {:>6} {:>6} {:>9} {:>9}",
        "name", "norm", "digits", "dim", "k^level", "distinct"
    );
    let all = [
        ("twindragon", twindragon()),
        ("gasket", gasket()),
        ("koch", koch()),
        ("terdragon", terdragon()),
        ("flowsnake", flowsnake()),
    ];
    for (name, radix) in all {
        let radix = radix.unwrap();
        println!(
            "{name:<11} {:>5} {:>6} {:>6.3} {:>9} {:>9}",
            radix.base().norm(),
            radix.size(),
            radix.dimension(),
            radix.fill(level),
            radix.distinct(level)
        );
    }
    println!("level {level}");
}
