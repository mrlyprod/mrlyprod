// The sponge's surface follows a short rule: a(level) = 28 a(level-1) - 160 a(level-2).
// The rule is found from the 3x3x3 tile alone, then checked against the counts.
// cargo run -p mrly-sequences --example surface_recurrence

use mrly_math::Code;
use mrly_math::factory::create;
use mrly_sequences::counts::{Closed, exposure_recurrence, surface};

fn main() {
    let tile = create(Code::from(23u64), 3, 3, 2, 1).unwrap();
    let rule = exposure_recurrence(&tile);
    println!("{}", Closed::Recurrence(rule.clone()).text());
    for level in 3..=6u32 {
        let a = |l: u32| surface(Code::from(23u64), 3, l, 2).unwrap() as i128;
        let predicted = rule[0] * a(level - 1) + rule[1] * a(level - 2);
        println!("level {level}: rule {predicted}, counted {}", a(level));
    }
}
