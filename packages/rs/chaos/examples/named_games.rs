// Corner games: a word of corners (0 to 3) is legal if every window of the given width obeys the rule.
// named_games() lists eight classic rules. For each, perron() gives how fast the number of legal words grows per step; the best possible is 4.
// cargo run -p mrly-chaos --example named_games
use mrly_automaton::memory::{Rule, counts, perron};
use mrly_chaos::named_games;

fn main() {
    println!(
        "{:<30} {:>5} {:>8}  first counts",
        "game", "width", "growth"
    );
    for game in named_games() {
        let rule = Rule::new(game.dimension, game.width, game.code as u64).unwrap();
        println!(
            "{:<30} {:>5} {:>8.4}  {:?}",
            game.name,
            game.width,
            perron(&rule),
            counts(&rule, 5)
        );
    }
}
