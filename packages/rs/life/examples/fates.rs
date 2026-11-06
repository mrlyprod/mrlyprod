// Every run of Life ends one of four ways: dead, alive (a still picture), loop (it repeats), or timeout.
// Five small starts under the same rule. A loop reports its period.
// cargo run -p mrly-life --example fates
use mrly_life::{Boundary, Config, Counts, animate, moore};
use mrly_two::from_strings;

fn run(name: &str, rows: &[&str], padding: usize, boundary: Boundary) {
    let rows: Vec<String> = rows.iter().map(|r| r.to_string()).collect();
    let seed = from_strings(&rows).unwrap();
    let mut config = Config::new(
        moore().unwrap(),
        Counts::list(vec![3]),
        Counts::list(vec![2, 3]),
    );
    config.boundary = boundary;
    config.padding = padding;
    let life = animate(&seed, &config).unwrap();
    println!(
        "{:<22} {:<6} frames {:>2}  period {}",
        name,
        life.fate.name(),
        life.count,
        life.loop_length
    );
}

fn main() {
    run("one cell", &["1"], 2, Boundary::Constant);
    run("block", &["11", "11"], 2, Boundary::Constant);
    run("blinker", &["111"], 2, Boundary::Constant);
    run(
        "glider on a torus",
        &["0100", "0010", "1110", "0000"],
        2,
        Boundary::Wrap,
    );
    run(
        "glider against a wall",
        &["0100", "0010", "1110", "0000"],
        2,
        Boundary::Constant,
    );
}
