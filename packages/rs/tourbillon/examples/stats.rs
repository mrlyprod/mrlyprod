// stats() sums up a field: mean and spread of the ink, the value at the centre, the brightest spots, and how many layers share a turn.
// Three schedules over the same odd scales up to 25: unspun (every board aligned), golden, and random with seed 7.
// Aligned boards share one turn class and pair up 78 ways (13 choose 2); spun boards have 13 classes and no pairs.
// cargo run -p mrly-tourbillon --example stats
use mrly_tourbillon::{field, stats};

fn main() {
    let (top, size, increment) = (25, 48, 0.0);
    for (schedule, seed) in [("unspun", 1), ("golden", 1), ("random", 7)] {
        let grid = field(
            top, size, schedule, increment, "odd", "plain", "cells", "mean", seed,
        )
        .unwrap();
        let s = stats(
            &grid, size, top, schedule, increment, "odd", "plain", "mean", seed,
        )
        .unwrap();
        println!("{schedule}");
        println!("  layers {}  cells inside {}", s.layers, s.inside);
        println!(
            "  mean {:.3}  rms {:.3}  centre {:.3}",
            s.mean, s.rms, s.centre
        );
        println!("  low {:.3}  high {:.3}", s.low, s.high);
        println!(
            "  turn classes {}  pairs sharing a turn {}",
            s.classes, s.pairs
        );
        for p in &s.peaks {
            println!("  peak at ({:.2}, {:.2}) value {:.3}", p[0], p[1], p[2]);
        }
    }
}
