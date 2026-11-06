// Closed loops among Truchet arcs follow simple laws. Each row draws the design at a level, counts its loops, and compares with law().
// Base 2 codes are 4-bit sets of corners of a 2 by 2 tile; base 3 code 495 is a 3 by 3 tile without its middle.
// cargo run -p mrly-arcs --example loop_law
use mrly_arcs::{draw, law};
use mrly_math::code::Code;

fn main() {
    let designs = [(2usize, 9u128), (2, 7), (2, 11), (3, 495)];
    for (base, code) in designs {
        let rule = law(Code(code), base, 1).unwrap().unwrap();
        println!("base {base} code {code}: loops = {}", rule.formula);
        println!("  level  side  drawn  law");
        let top = if base == 3 { 3 } else { 5 };
        for level in 1..=top {
            let drawn = draw(Code(code), base, level).unwrap();
            let rule = law(Code(code), base, level).unwrap().unwrap();
            println!(
                "  {level:>5} {:>5} {:>6} {:>4}",
                drawn.side, drawn.loops, rule.loops
            );
        }
    }
}
