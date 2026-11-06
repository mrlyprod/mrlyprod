// Yes or no facts about a rule, for a few well-known ones.
// affine: the new cell is a sum mod 2 of the neighbours. surjective: every row has a parent row.
// reversible: the rule can be run backwards on a ring.
// cargo run -p mrly-wolfram --example properties
use mrly_wolfram::{affine, reversible, rule_degree, surjective};

fn main() {
    println!("rule  degree  affine  surjective  reversible");
    for rule in [0u8, 30, 51, 90, 110, 170, 204] {
        println!(
            "{:>4}  {:>6}  {:>6}  {:>10}  {:>10}",
            rule,
            rule_degree(rule),
            affine(rule),
            surjective(rule),
            reversible(rule)
        );
    }
}
