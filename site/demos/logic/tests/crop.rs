use demos::crop::crop_collapse;
use mrlyrs::core::error::parse;

fn scales(read: &mrlyrs::core::Json) -> Vec<i64> {
    read["scales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|scale| scale["start"].as_i64().unwrap())
        .collect()
}

fn profile(read: &mrlyrs::core::Json, scale: usize) -> Vec<f64> {
    read["scales"][scale]["main"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

fn ladder(read: &mrlyrs::core::Json, key: &str) -> String {
    let rows = read["pairs"].as_array().unwrap();
    let step: Vec<String> = rows
        .iter()
        .filter(|pair| pair["high"].as_i64().unwrap() == pair["low"].as_i64().unwrap() + 1)
        .map(|pair| match pair[key].as_f64() {
            Some(v) => format!("{v:.6}"),
            None => "none".to_string(),
        })
        .collect();
    step.join(",")
}

#[test]
fn the_two_scales_the_collapse_panel_lays_on_each_other() {
    let read = |code: &str, dimension: usize, level: usize, centre: &str| {
        parse(&crop_collapse(code, 3, level, 2, dimension, centre, 16).unwrap()).unwrap()
    };
    let carpet = read("7", 2, 6, "corner");
    println!("crop_collapse carpet scales {:?}", scales(&carpet));
    assert_eq!(scales(&carpet), [1, 3, 9, 27, 81, 243]);
    println!(
        "crop_collapse carpet d {:.9} mass {} top {}",
        carpet["d"].as_f64().unwrap(),
        carpet["mass"],
        carpet["top"]
    );
    assert_eq!(
        format!("{:.9}", carpet["d"].as_f64().unwrap()),
        "1.892789261"
    );
    assert_eq!(
        (
            carpet["mass"].as_str().unwrap(),
            carpet["top"].as_i64().unwrap()
        ),
        ("8", 728)
    );
    let deep: Vec<String> = profile(&carpet, 5)[..4]
        .iter()
        .map(|v| format!("{v:.6}"))
        .collect();
    println!("crop_collapse carpet deep {}", deep.join(","));
    assert_eq!(deep.join(","), "0.751038,0.779830,0.803768,0.810867");
    println!("crop_collapse carpet step sup {}", ladder(&carpet, "sup"));
    assert_eq!(
        ladder(&carpet, "sup"),
        "0.353553,0.222183,0.110138,0.042663,0.015114"
    );
    println!(
        "crop_collapse carpet step share {}",
        ladder(&carpet, "share")
    );
    assert_eq!(
        ladder(&carpet, "share"),
        "0.534078,0.305035,0.145250,0.055448,0.019546"
    );
    let shares: Vec<f64> = carpet["pairs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|pair| pair["high"].as_i64().unwrap() == pair["low"].as_i64().unwrap() + 1)
        .map(|pair| pair["share"].as_f64().unwrap())
        .collect();
    for step in shares.windows(2) {
        assert!(step[1] < step[0], "the gap must shrink with the scale");
    }
    let sponge = read("23", 3, 4, "corner");
    println!("crop_collapse sponge scales {:?}", scales(&sponge));
    assert_eq!(scales(&sponge), [1, 3, 9, 27]);
    println!(
        "crop_collapse sponge step share {}",
        ladder(&sponge, "share")
    );
    assert_eq!(ladder(&sponge, "share"), "0.805610,0.366087,0.229755");
    let hole = read("7", 2, 6, "centre");
    let levels: Vec<String> = hole["scales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|scale| format!("{:.6}", scale["level"].as_f64().unwrap()))
        .collect();
    println!("crop_collapse centre levels {}", levels.join(","));
    assert_eq!(
        levels.join(","),
        "0.000000,0.000000,0.000000,0.000000,0.668835"
    );
    println!("crop_collapse centre step share {}", ladder(&hole, "share"));
    assert_eq!(
        ladder(&hole, "share"),
        "0.000000,0.000000,0.000000,3.081886"
    );
    assert!(crop_collapse("7", 3, 6, 2, 2, "corner", 1).is_err());
    assert!(crop_collapse("7", 1, 6, 2, 2, "corner", 16).is_err());
}

fn ridges(read: &mrlyrs::core::Json) -> String {
    let rows: Vec<String> = read["scales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|scale| match scale["ridge"].as_f64() {
            Some(v) => format!("{v:.6}"),
            None => "none".to_string(),
        })
        .collect();
    rows.join(",")
}

fn drift(read: &mrlyrs::core::Json, scale: usize) -> Vec<f64> {
    read["scales"][scale]["drift"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

#[test]
fn the_defect_ridge_the_collapse_panel_folds_beside_it() {
    let carpet = parse(&crop_collapse("7", 3, 6, 2, 2, "corner", 16).unwrap()).unwrap();
    println!("crop_collapse carpet ridges {}", ridges(&carpet));
    assert_eq!(
        ridges(&carpet),
        "0.798324,1.688227,0.600112,0.389870,0.445603,none"
    );
    println!(
        "crop_collapse carpet drift 4 len {}",
        drift(&carpet, 4).len()
    );
    let deep: Vec<String> = drift(&carpet, 4)[..4]
        .iter()
        .map(|v| format!("{v:.6}"))
        .collect();
    println!("crop_collapse carpet drift 4 {}", deep.join(","));
    assert_eq!(deep.join(","), "0.593262,0.167396,0.489821,0.131627");
    println!(
        "crop_collapse carpet drift 3 head {:.6}",
        drift(&carpet, 3)[0]
    );
    assert_eq!(format!("{:.6}", drift(&carpet, 3)[0]), "0.000000");
    println!("crop_collapse carpet ridge sup {}", ladder(&carpet, "rsup"));
    assert_eq!(
        ladder(&carpet, "rsup"),
        "2.000000,2.859375,0.789696,0.714173,none"
    );
    println!(
        "crop_collapse carpet ridge share {}",
        ladder(&carpet, "rshare")
    );
    assert_eq!(
        ladder(&carpet, "rshare"),
        "1.184675,1.693714,1.315914,1.602712,none"
    );
    let sponge = parse(&crop_collapse("23", 3, 4, 2, 3, "corner", 16).unwrap()).unwrap();
    println!("crop_collapse sponge ridges {}", ridges(&sponge));
    assert_eq!(ridges(&sponge), "3.868696,3.649261,1.456937,none");
    assert!(sponge["scales"][3]["drift"].as_array().unwrap().is_empty());
}
