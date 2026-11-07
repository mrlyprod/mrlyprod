// A saga runs Life, stops it, deals a new rule, and runs on from the last frame. Each run is a chapter.
// The deck says what a chapter may deal: "all" deals any mask and rule of the life catalog, "rules" keeps the
// 3 by 3 Moore mask and deals rules, "masks" keeps Conway's B3/S23 and deals masks. The same seed tells the same saga.
// Every knob of a deck is open: its masks, rules and lengths, the least and most chapters, and the generations a chapter must last.
// cargo run -p mrly-saga --example saga
use mrly_life::Catalog;
use mrly_rand::Rng;
use mrly_saga::{Deck, draw, film, name, parse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = Catalog::standard()?;
    let mut four = Deck::all(&catalog);
    (four.least, four.most, four.lasts, four.lengths) = (4, 4, 0, vec![8]);
    let decks = [
        ("all", Deck::all(&catalog)),
        ("rules", Deck::rules(&catalog)),
        ("masks", Deck::masks(&catalog)),
        ("four chapters of 8, any length kept", four),
    ];
    for (label, deck) in decks {
        let saga = draw(&catalog, &deck, &mut Rng::new(7))?;
        let line = name(&saga);
        println!("deck {label}");
        for (i, part) in line.split(" | ").enumerate() {
            println!("  {i} {part}");
        }

        // The name is the saga: parsing it back and filming it replays every chapter bit for bit.
        let (reel, stops) = film(&parse(&line)?)?;
        let told: Vec<String> = stops
            .iter()
            .map(|stop| format!("{} at {}", stop.kind, stop.generation))
            .collect();
        let [height, width] = reel.shape();
        println!("  {}", told.join(", "));
        println!("  {} frames on a {height} by {width} canvas", reel.len());
    }
    Ok(())
}
