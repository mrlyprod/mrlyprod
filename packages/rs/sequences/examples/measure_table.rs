// Every measure a sequence can count, and where it applies.
// cost says how it is computed: Closed is a formula, Convolved uses polynomial products, Grid builds the shape.
// cargo run -p mrly-sequences --example measure_table

use mrly_sequences::{Axis, Measure};

fn main() {
    println!("{:<9} {:<9} 2D  3D  axes", "measure", "cost");
    for measure in Measure::ALL {
        let axes: Vec<&str> = measure.axes().iter().map(|a| a.slug()).collect();
        println!(
            "{:<9} {:<9} {:<3} {:<3} {}",
            measure.slug(),
            format!("{:?}", measure.cost(Axis::Level)),
            if measure.applies(2, 2) { "yes" } else { "-" },
            if measure.applies(3, 2) { "yes" } else { "-" },
            axes.join(",")
        );
    }
}
