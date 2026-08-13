use mrlycore::json::parse;
use mrlycore::Json;
use mrlydata::{find, pour, wells, SEED};
use std::path::Path;

const SAMPLE: usize = 4;
const PREVIEW: usize = 3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("emit") => emit(&args[2..]),
        Some("preview") => preview(&args[2..]),
        Some("verify") => verify(),
        _ => {
            usage();
            std::process::exit(2);
        }
    }
}

fn usage() {
    eprintln!("mrlydata - the dataset press");
    eprintln!();
    eprintln!("usage:");
    eprintln!("  mrlydata emit <dir> [seed]   pour every dataset (seed {SEED} by default)");
    eprintln!("  mrlydata preview <name>      print {PREVIEW} rows of one well");
    eprintln!("  mrlydata verify              re-pour samples twice and check the bytes");
}

fn emit(args: &[String]) {
    let Some(dir) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let seed = args
        .get(1)
        .and_then(|text| text.parse().ok())
        .unwrap_or(SEED);
    match pour(Path::new(dir), seed) {
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
            eprintln!("emit failed: {error}");
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
