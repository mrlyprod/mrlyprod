// A stack is a list of layers: a scale (squares per side), a weight, and a turn in degrees.
// One column per schedule shows how the turns are chosen. Moebius weights are -1, 0 or 1, then rescaled so their absolute values average 1.
// "degrees" adds 15 per layer, "primes" turns by the primes 2, 3, 5..., "gaussian" turns by the angle of a + bi where a^2 + b^2 = scale.
// cargo run -p mrly-tourbillon --example layers
use mrly_tourbillon::layers;

fn main() {
    let names = [
        "unspun", "degrees", "golden", "primes", "gaussian", "random",
    ];
    let all: Vec<_> = names
        .iter()
        .map(|name| layers(15, name, 15.0, "odd", "mobius", 1).unwrap())
        .collect();
    print!("scale weight");
    for name in names {
        print!(" {name:>8}");
    }
    println!();
    for (i, layer) in all[0].iter().enumerate() {
        print!("{:>5} {:>6.2}", layer.scale, layer.weight);
        for list in &all {
            print!(" {:>8.2}", list[i].degrees);
        }
        println!();
    }
}
