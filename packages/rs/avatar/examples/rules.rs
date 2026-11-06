// Why the sponge uses the replicator rule and 16 rounds.
// Each cell shows the avalanche (ideal 0.5) for one rule and number of rounds on the 32 by 32 state.
// Replicator and anneal reach 0.5 from 12 rounds on; life and maze are slow and uneven.
// Below 12 rounds nothing is mixed enough, and the numbers there jump around.
// Replicator with 16 rounds is the default: it holds 0.5 with four rounds to spare.
// cargo run -p mrly-avatar --example rules
use mrly_avatar::metrics::avalanche;
use mrly_avatar::{Config, Rule, hash};

const ROUNDS: [usize; 5] = [4, 8, 12, 16, 20];
const TRIALS: usize = 24;

fn main() {
    print!("{:<11}", "rule");
    for rounds in ROUNDS {
        print!(" {rounds:>6}");
    }
    println!();
    for rule in Rule::all() {
        print!("{:<11}", rule.name());
        for rounds in ROUNDS {
            let config = Config {
                rounds,
                rule,
                ..Config::default()
            };
            let hasher = |bytes: &[u8]| hash(bytes, &config).unwrap();
            print!(" {:>6.3}", avalanche(&hasher, TRIALS));
        }
        println!();
    }
}
