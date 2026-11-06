// What one round of the sponge does to a state of 8 by 8 bits that holds a single set bit.
// A round has four steps: the automaton, a 4-bit substitution, a rotation, then a constant.
// The automaton alone spreads the bit like a fire; the other three scramble where it lands.
// Rounds 0 to 5 are shown left to right, with the count of set bits under each.
// cargo run -p mrly-avatar --example rounds
use mrly_avatar::{Config, round};

const SIDE: usize = 8;
const SHOWN: usize = 6;

fn main() {
    let config = Config {
        side: SIDE,
        rounds: SHOWN,
        ..Config::default()
    };
    let mut state = vec![0u8; SIDE * SIDE];
    state[3 * SIDE + 3] = 1;
    let mut states = vec![state.clone()];
    for index in 0..SHOWN - 1 {
        state = round(&state, &config, index).unwrap();
        states.push(state.clone());
    }
    for row in 0..SIDE {
        let line: Vec<String> = states
            .iter()
            .map(|s| {
                s[row * SIDE..(row + 1) * SIDE]
                    .iter()
                    .map(|&b| if b == 1 { '#' } else { '.' })
                    .collect()
            })
            .collect();
        println!("{}", line.join("  "));
    }
    let counts: Vec<String> = states
        .iter()
        .map(|s| format!("{:<8}", s.iter().filter(|&&b| b == 1).count()))
        .collect();
    println!("{}", counts.join("  ").trim_end());
}
