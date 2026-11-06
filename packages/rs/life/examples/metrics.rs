// Two numbers that describe a run. Entropy is how mixed the first picture is: 0 for all one colour, 1000 for half and half.
// Churn is the share of cells that change per step, averaged over the run.
// cargo run -p mrly-life --example metrics
use mrly_life::{Boundary, Config, Counts, animate, churn, entropy, moore};
use mrly_two::from_strings;

fn main() {
    let cases: [(&str, &[&str]); 3] = [
        ("block", &["11", "11"]),
        ("blinker", &["111"]),
        ("glider", &["0100", "0010", "1110", "0000"]),
    ];
    println!("{:<8} entropy  churn", "");
    for (name, rows) in cases {
        let rows: Vec<String> = rows.iter().map(|r| r.to_string()).collect();
        let seed = from_strings(&rows).unwrap();
        let mut config = Config::new(
            moore().unwrap(),
            Counts::list(vec![3]),
            Counts::list(vec![2, 3]),
        );
        config.boundary = Boundary::Wrap;
        config.padding = 2;
        let life = animate(&seed, &config).unwrap();
        println!(
            "{:<8} {:>7}  {:.4}",
            name,
            entropy(&life.grids[0]),
            churn(&life.grids)
        );
    }
}
