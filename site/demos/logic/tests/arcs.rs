use demos::arcs::*;
use mrlyrs::core::error::parse;

fn loops(code: &str, base: usize, levels: std::ops::RangeInclusive<usize>) -> String {
    levels
        .map(|n| parse(&arcs_read(code, base, n).unwrap()).unwrap()["loops"].to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn the_arcs_exports_answer() {
    assert_eq!(loops("495", 3, 1..=4), "0,3,50,509");
    assert_eq!(loops("7", 2, 1..=4), "0,0,2,12");

    let carpet = parse(&arcs_read("495", 3, 4).unwrap()).unwrap();
    assert_eq!(carpet["side"], 81);
    assert_eq!(carpet["filled"], 4096);
    assert_eq!(carpet["strands"], 162);
    assert_eq!(carpet["law"]["formula"], "(8^n - 1)/7 - 3^n + n + 1");
    assert_eq!(carpet["law"]["loops"], 509);

    let seven = parse(&arcs_read("7", 2, 6).unwrap()).unwrap();
    assert_eq!(seven["loops"], 180);
    assert_eq!(seven["law"]["loops"], 180);
    assert_eq!(parse(&arcs_read("9", 2, 8).unwrap()).unwrap()["loops"], 255);
    assert!(parse(&arcs_read("511", 3, 2).unwrap()).unwrap()["law"].is_null());

    let grid = arcs_grid("9", 2, 1).unwrap();
    assert_eq!((grid.width, grid.height), (2, 2));
    assert_eq!(grid.types, [5, 4, 2, 3]);

    assert_eq!([2, 3, 4, 5].map(arcs_cap), [8, 5, 4, 3]);
    assert!(arcs_read("16", 2, 2).is_err());
    assert!(arcs_grid("7", 2, 9).is_err());
}
