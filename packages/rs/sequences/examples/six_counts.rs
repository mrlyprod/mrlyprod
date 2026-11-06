// Triangles in the flat views of the sponge, by level: filled and empty, for the pro and cut views.
// Level 1 gives 48/6 and 42/12, the same as counting the drawn views in the six crate.
// cargo run -p mrly-sequences --example six_counts

use mrly_math::Code;
use mrly_sequences::counts::{cut_fills, cut_voids, pro_fills, pro_voids};

fn main() {
    let sponge = Code::from(23u64);
    println!("level pro (filled/empty)  cut (filled/empty)");
    for level in 1..=5 {
        let pro = format!(
            "{}/{}",
            pro_fills(sponge, 3, level).unwrap(),
            pro_voids(sponge, 3, level).unwrap()
        );
        let cut = format!(
            "{}/{}",
            cut_fills(sponge, 3, level).unwrap(),
            cut_voids(sponge, 3, level).unwrap()
        );
        println!("{level:>5} {pro:<22} {cut}");
    }
}
