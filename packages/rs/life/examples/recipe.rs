// A recipe is one line that says everything about a run: the seed design, the mask, the rule, the step limit.
// Here a 3 by 3 carpet is the seed, Conway's rule is B3/S23, and the island runs until it repeats.
// cargo run -p mrly-life --example recipe
use mrly_life::Run;
use mrly_life::recipe::{name, parse};

fn main() {
    let line = "carpet.3.1 x1 island mask:code15.3.1 birth:3 survive:2,3 x:50 cap:40";
    let recipe = parse(line).unwrap();
    println!("{}", name(&recipe));
    let mut run = Run::new(&recipe).unwrap();
    println!("start: population {}", run.population());
    let stop = loop {
        if let Some(stop) = run.step() {
            break stop;
        }
    };
    println!(
        "stopped: {} at generation {}, period {}, population {}",
        stop.kind,
        stop.generation,
        stop.period,
        run.population()
    );
    println!();
    let frame = run.frame();
    let width = frame.shape()[1];
    for row in frame.types.bytes().unwrap().chunks(width) {
        let line: String = row
            .iter()
            .map(|&b| if b == 1 { '#' } else { '.' })
            .collect();
        println!("{line}");
    }
}
