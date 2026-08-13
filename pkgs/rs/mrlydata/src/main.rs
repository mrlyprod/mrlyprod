use mrlycore::json::parse;
use mrlycore::Json;
use mrlydata::{carry, emit, find, pour, tiles, wells, BATCH, SEED};
use std::path::Path;

const SAMPLE: usize = 4;
const PREVIEW: usize = 3;
const TILES: &str = "data/tiles";
const CARRY: &str = "data/carry";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("pour") => run_pour(&args[2..]),
        Some("emit") => run_emit(&args[2..]),
        Some("tiles") => run_tiles(&args[2..]),
        Some("carry") => run_carry(&args[2..]),
        Some("preview") => preview(&args[2..]),
        Some("verify") => verify(),
        _ => {
            usage();
            std::process::exit(2);
        }
    }
}

fn usage() {
    eprintln!("mrlydata - the press");
    eprintln!();
    eprintln!("usage:");
    eprintln!(
        "  mrlydata pour <dir> [seed]            pour every dataset (seed {SEED} by default)"
    );
    eprintln!("  mrlydata emit <dir> [count] [seed]    press {BATCH} artworks by default");
    eprintln!("  mrlydata tiles [count] [seed]         press {BATCH} tile pngs into {TILES}");
    eprintln!("  mrlydata carry <text> [count] [seed]  press {BATCH} carried sheets into {CARRY}");
    eprintln!("  mrlydata preview <name>               print {PREVIEW} rows of one well");
    eprintln!("  mrlydata verify                       re-pour samples twice and check the bytes");
}

fn number(args: &[String], at: usize, fallback: u64) -> u64 {
    args.get(at)
        .and_then(|text| text.parse().ok())
        .unwrap_or(fallback)
}

fn run_pour(args: &[String]) {
    let Some(dir) = args.first() else {
        usage();
        std::process::exit(2);
    };
    match pour(Path::new(dir), number(args, 1, SEED)) {
        Ok(manifest) => {
            if let Some(datasets) = manifest["datasets"].as_array() {
                for dataset in datasets {
                    println!(
                        "{}: {} rows, {} bytes",
                        dataset["name"].as_str().unwrap_or("?"),
                        dataset["rows"],
                        dataset["bytes"]
                    );
                }
                println!("pressed {} datasets into {dir}", datasets.len());
            }
        }
        Err(error) => {
            eprintln!("pour failed: {error}");
            std::process::exit(1);
        }
    }
}

fn run_emit(args: &[String]) {
    let Some(dir) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let count = number(args, 1, BATCH as u64) as usize;
    match emit(Path::new(dir), number(args, 2, SEED), count) {
        Ok(manifest) => {
            if let Some(artworks) = manifest["artworks"].as_array() {
                for artwork in artworks {
                    println!(
                        "{}: {}, {} files",
                        artwork["key"].as_str().unwrap_or("?"),
                        artwork["edition"].as_str().unwrap_or("?"),
                        artwork["files"].as_array().map_or(0, Vec::len)
                    );
                }
                println!("pressed {} artworks into {dir}", artworks.len());
            }
        }
        Err(error) => {
            eprintln!("emit failed: {error}");
            std::process::exit(1);
        }
    }
}

fn run_tiles(args: &[String]) {
    let count = number(args, 0, BATCH as u64) as usize;
    match tiles(Path::new(TILES), number(args, 1, SEED), count) {
        Ok(manifest) => {
            if let Some(rows) = manifest["tiles"].as_array() {
                for row in rows {
                    println!(
                        "{}: {}x{}",
                        row["name"].as_str().unwrap_or("?"),
                        row["width"],
                        row["height"]
                    );
                }
                println!("pressed {} tiles into {TILES}", rows.len());
            }
        }
        Err(error) => {
            eprintln!("tiles failed: {error}");
            std::process::exit(1);
        }
    }
}

fn run_carry(args: &[String]) {
    let Some(text) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let count = number(args, 1, BATCH as u64) as usize;
    match carry(
        Path::new(CARRY),
        number(args, 2, SEED),
        count,
        text.as_bytes(),
    ) {
        Ok(manifest) => {
            if let Some(rows) = manifest["sheets"].as_array() {
                for row in rows {
                    println!(
                        "{}: {}x{}",
                        row["name"].as_str().unwrap_or("?"),
                        row["width"],
                        row["height"]
                    );
                }
                println!("pressed {} sheets into {CARRY}", rows.len());
            }
        }
        Err(error) => {
            eprintln!("carry failed: {error}");
            std::process::exit(1);
        }
    }
}

fn preview(args: &[String]) {
    let Some(name) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let Some(well) = find(name) else {
        eprintln!("no such well: {name}");
        std::process::exit(1);
    };
    println!("{} - {}", well.name(), well.about());
    for row in well.pour(SEED, PREVIEW) {
        println!("{}", row.pretty());
    }
}

fn verify() {
    let mut errors = 0;
    for well in wells() {
        let name = well.name().to_string();
        let a: Vec<String> = well
            .pour(SEED, SAMPLE)
            .iter()
            .map(Json::to_string)
            .collect();
        let b: Vec<String> = well
            .pour(SEED, SAMPLE)
            .iter()
            .map(Json::to_string)
            .collect();
        if a != b {
            println!("ERROR {name}: two pours differ");
            errors += 1;
            continue;
        }
        if a.is_empty() {
            println!("ERROR {name}: poured nothing");
            errors += 1;
            continue;
        }
        let short: Vec<String> = well
            .pour(SEED, SAMPLE / 2)
            .iter()
            .map(Json::to_string)
            .collect();
        if short[..] != a[..short.len()] {
            println!("ERROR {name}: a short pour is not a prefix");
            errors += 1;
            continue;
        }
        if a.iter().any(|line| parse(line).is_err()) {
            println!("ERROR {name}: a row does not parse");
            errors += 1;
            continue;
        }
        println!("verified {name} ({} rows)", a.len());
    }
    if errors > 0 {
        println!("FAILURE: {errors} wells failed");
        std::process::exit(1);
    }
    println!("SUCCESS: every well verified");
}
