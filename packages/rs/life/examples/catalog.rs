// The catalog is every basic recipe of mrly life in one fixed order: 66 seeds, each as an island
// tiled 1, 3 and 5 times and as a wallpaper, under 82 design masks (plus copies of the seed) and 125 rules.
// recipe(i) names the i-th; draw() picks one at random that lasts at least the generations asked (0 takes any).
// film() runs a recipe and keeps every frame on one canvas, ready for gif().
// cargo run -p mrly-life --example catalog
use mrly_life::catalog::LASTS;
use mrly_life::recipe::{film, name};
use mrly_life::{Catalog, gif};
use mrly_rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = Catalog::standard()?;
    println!("{} recipes", catalog.count());
    for i in [0, 1_000_000, catalog.count() - 1] {
        println!("{i:>7} {}", name(&catalog.recipe(i)?));
    }
    println!();
    println!("three draws that last {LASTS} generations or more:");
    let mut rng = Rng::new(1);
    for _ in 0..3 {
        let recipe = catalog.draw(&mut rng, LASTS)?;
        let (reel, stop) = film(&recipe)?;
        let [height, width] = reel.shape();
        let bytes = gif(&reel.frames()?, 1, 10)?;
        println!("{}", name(&recipe));
        println!(
            "  {} at generation {}, {} frames on {height} by {width}, a gif of {} bytes",
            stop.kind,
            stop.generation,
            reel.len(),
            bytes.len()
        );
    }
    Ok(())
}
