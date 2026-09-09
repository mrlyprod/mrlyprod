use mrlycore::json::parse;
use mrlydemo::crop::{crop_circle, crop_collapse};

struct Circle {
    seen: Vec<u32>,
    inside: Vec<u32>,
    cut: Vec<u32>,
}

fn circle(code: &str, dimension: usize, level: usize, centre: &str) -> Circle {
    let flat = crop_circle(code, 3, level, 2, dimension, centre).unwrap();
    let width = flat.len() / 3;
    Circle {
        seen: flat[..width].to_vec(),
        inside: flat[width..2 * width].to_vec(),
        cut: flat[2 * width..].to_vec(),
    }
}

#[test]
fn the_circle_count_the_page_prints() {
    let carpet = circle("7", 2, 6, "corner");
    println!("crop_circle carpet corner radii {}", carpet.seen.len());
    assert_eq!(carpet.seen.len(), 729);
    println!("crop_circle carpet corner 1..12 {:?}", &carpet.seen[1..=12]);
    assert_eq!(
        carpet.seen[1..=12],
        [1, 3, 7, 12, 16, 22, 30, 38, 48, 63, 77, 91]
    );
    let powers: Vec<u32> = (1..=5).map(|k| carpet.seen[3usize.pow(k)]).collect();
    println!("crop_circle carpet N(3^k) {powers:?}");
    assert_eq!(powers, [7, 48, 385, 3080, 24610]);
    let defect = carpet.seen[81] as i64 - 8 * carpet.seen[27] as i64;
    println!("crop_circle carpet delta(27) {defect}");
    assert_eq!(defect, 0);
    let cuts = [carpet.cut[27], carpet.cut[81], carpet.cut[243]];
    println!("crop_circle carpet C(3^k) {cuts:?}");
    assert_eq!(cuts, [42, 114, 306]);
    println!(
        "crop_circle carpet brackets 243 {} {} {}",
        carpet.inside[243],
        carpet.seen[243],
        carpet.inside[243] + carpet.cut[243]
    );
    for r in 1..carpet.seen.len() {
        assert!(carpet.inside[r] <= carpet.seen[r], "in r={r}");
        assert!(
            carpet.seen[r] <= carpet.inside[r] + carpet.cut[r],
            "out r={r}"
        );
    }
    let sponge = circle("23", 3, 4, "corner");
    println!("crop_circle sponge corner radii {}", sponge.seen.len());
    assert_eq!(sponge.seen.len(), 81);
    println!("crop_circle sponge corner 1..8 {:?}", &sponge.seen[1..=8]);
    assert_eq!(sponge.seen[1..=8], [1, 4, 13, 28, 47, 65, 95, 137]);
    let hole = circle("7", 2, 6, "centre");
    let first = (1..hole.seen.len()).find(|&r| hole.seen[r] > 0).unwrap();
    println!(
        "crop_circle carpet centre radii {} first {first}",
        hole.seen.len()
    );
    assert_eq!((hole.seen.len(), first), (365, 122));
    let deep = circle("23", 3, 4, "centre");
    let hit = (1..deep.seen.len()).find(|&r| deep.seen[r] > 0).unwrap();
    println!(
        "crop_circle sponge centre radii {} first {hit}",
        deep.seen.len()
    );
    assert_eq!((deep.seen.len(), hit), (41, 20));
    assert!(crop_circle("7", 3, 6, 2, 2, "edge").is_err());
    assert!(crop_circle("7", 3, 9, 2, 2, "corner").is_err());
}

fn scales(read: &mrlycore::Json) -> Vec<i64> {
    read["scales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|scale| scale["start"].as_i64().unwrap())
        .collect()
}

fn profile(read: &mrlycore::Json, scale: usize) -> Vec<f64> {
    read["scales"][scale]["main"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

fn ladder(read: &mrlycore::Json, key: &str) -> String {
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

fn ridges(read: &mrlycore::Json) -> String {
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

fn drift(read: &mrlycore::Json, scale: usize) -> Vec<f64> {
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
