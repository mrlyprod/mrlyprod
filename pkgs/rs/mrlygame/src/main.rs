use mrlygame::{emit, SEED};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("emit") => run(&args[2..]),
        _ => {
            usage();
            std::process::exit(2);
        }
    }
}

fn usage() {
    eprintln!("mrlygame - the content generator");
    eprintln!();
    eprintln!("usage:");
    eprintln!("  mrlygame emit <dir> [seed]   generate one quest (seed {SEED} by default)");
}

fn run(args: &[String]) {
    let Some(dir) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let seed = args
        .get(1)
        .and_then(|text| text.parse().ok())
        .unwrap_or(SEED);
    match emit(Path::new(dir), seed) {
        Ok(record) => {
            let key = record["key"].as_str().unwrap_or("?").to_string();
            let frames = record["manifest"]["segments"]
                .as_array()
                .map_or(0, |lengths| {
                    lengths.iter().filter_map(|n| n.as_u64()).sum::<u64>()
                });
            println!("{key}: {frames} frames pressed into {dir}");
        }
        Err(error) => {
            eprintln!("emit failed: {error}");
            std::process::exit(1);
        }
    }
}
