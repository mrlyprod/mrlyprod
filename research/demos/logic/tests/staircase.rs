use demos::sieve::*;
use mrlyrs::core::error::parse;

#[test]
fn the_staircase_exports_answer() {
    let seven = parse(&staircase_read("7", 2, 3, false).unwrap()).unwrap();
    assert_eq!(seven["profile"].to_string(), "[1,2,0]");
    assert_eq!(seven["drift"].to_string(), "[1,3]");
    assert_eq!(seven["correction"].to_string(), "[17,36]");
    assert_eq!(seven["closed"], "3 pi/(4 Gamma(1/3))");
    assert_eq!(
        format!("{:.12}", seven["constant"].as_f64().unwrap()),
        "0.879525401448"
    );
    assert!(seven["reflection"].is_null());
    assert_eq!(seven["mirror"]["code"], "14");
    assert_eq!(seven["mirror"]["closed"], "3 pi/(8 Gamma(2/3))");
    assert_eq!(
        format!("{:.12}", seven["mirror"]["product"].as_f64().unwrap()),
        "0.765196606786"
    );
    let fills: Vec<String> = seven["levels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["fill"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(fills, ["8", "21", "40"]);

    let diagonal = parse(&staircase_read("9", 2, 2, false).unwrap()).unwrap();
    assert_eq!(diagonal["closed"], "cosh(pi/2)/2");
    assert_eq!(
        format!("{:.12}", diagonal["reflection"].as_f64().unwrap()),
        "1.254589239329"
    );
    assert_eq!(diagonal["parity"]["odd"], false);
    assert_eq!(
        format!("{:.12}", diagonal["parity"]["value"].as_f64().unwrap()),
        "1.254589239329"
    );

    let solid = parse(&staircase_read("23", 3, 2, false).unwrap()).unwrap();
    assert_eq!(solid["closed"], "pi^(3/2)/(2 Gamma(1/4))");
    assert_eq!(
        format!("{:.12}", solid["constant"].as_f64().unwrap()),
        "0.767916038651"
    );
    assert_eq!(solid["floor"], "7");

    let even = parse(&staircase_read("7", 2, 4, true).unwrap()).unwrap();
    let flat = even["levels"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| row["settle"].as_f64() == Some(1.0));
    assert!(flat);

    assert_eq!(seven["coordinates"].to_string(), "[4,2]");
    assert_eq!(
        format!("{:.12}", seven["mirror"]["both"].as_f64().unwrap()),
        "0.765196606786"
    );
    assert_eq!(
        seven["levels"][2]["word"].to_string(),
        r#"["105","6720","11025"]"#
    );

    let walk = staircase_walk("6", 2, 1_000_000, 7, false).unwrap();
    assert_eq!(walk.len(), 28);
    assert_eq!(walk[24], 1_000_000.0);
    assert!((walk[25] - std::f64::consts::FRAC_PI_4).abs() < 1e-6);
    assert!((walk[25] - walk[26]).abs() < 1e-11);
    let seven_walk = staircase_walk("7", 2, 100_000, 6, false).unwrap();
    assert_eq!(
        format!("{:.4}", seven_walk[23]),
        format!("{:.4}", 17.0 / 36.0)
    );

    assert_eq!(staircase_cap(false), 4);
    assert_eq!(staircase_cap(true), 4);
    let grid = staircase_grid("7", 2, 2, false).unwrap();
    assert_eq!((grid.width, grid.height), (15, 15));
    assert_eq!(grid.types.iter().map(|&b| b as usize).sum::<usize>(), 168);
    let line = staircase_grid("1", 1, 3, false).unwrap();
    assert_eq!(line.width, 105);
    assert!(staircase_grid("240", 3, 1, false).is_err());
    assert!(staircase_read("0", 2, 3, false).is_err());
}
