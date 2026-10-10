use demos::arrays::*;
use mrlyrs::core::error::parse;

fn counts(generator: &str, base: u32, levels: std::ops::RangeInclusive<u32>) -> String {
    levels
        .map(|r| parse(&arrays_read(generator, base, r).unwrap()).unwrap()["count"].to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn the_arrays_exports_answer() {
    let figure = parse(&arrays_read("0, 1, 2, 4", 9, 4).unwrap()).unwrap();
    assert_eq!(figure["total"], 256);
    assert_eq!(figure["count"], 81);
    assert_eq!(figure["law"]["count"], 81);
    assert_eq!(figure["paired"].to_string(), "[0,1,4]");
    assert_eq!(figure["holes"], 0);

    assert_eq!(counts("0 1 2", 5, 1..=3), "3,4,8");
    assert_eq!(counts("0 1 2", 4, 1..=6), "3,2,2,2,2,2");
    assert_eq!(counts("0 1 4 6", 8, 1..=5), "4,11,25,53,109");
    assert_eq!(counts("0 1 2 3 7 11", 15, 1..=4), "6,16,36,76");

    let three = parse(&arrays_read("{1, 2, 3, 4}", 7, 2).unwrap()).unwrap();
    assert_eq!(three["generator"].to_string(), "[0,1,2,3]");
    assert_eq!(three["count"], 4);
    assert_eq!(three["total"], 16);
    assert_eq!(three["bounds"]["cohen"].to_string(), "[2,4]");
    assert_eq!(three["bounds"]["yang"].to_string(), "[4,16]");
    assert!(parse(&arrays_read("0 1 2", 5, 1).unwrap()).unwrap()["law"].is_null());
    assert!(parse(&arrays_read("0 1 2", 4, 2).unwrap()).unwrap()["law"].is_null());

    let knock = parse(&arrays_knock("0 1 2 4", 9, 2, 1).unwrap()).unwrap();
    assert_eq!(knock["sensor"], 1);
    assert_eq!(knock["digits"].to_string(), "[1,0]");
    assert_eq!(knock["paired"], true);
    let lost: Vec<i64> = knock["lost"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t.as_i64())
        .collect();
    assert!(lost.contains(&-39));
    let whole = arrays_weights("0 1 2 4", 9, 2, 99).unwrap();
    let less = arrays_weights("0 1 2 4", 9, 2, 1).unwrap();
    assert_eq!((whole.len(), whole[40], less[40]), (81, 16, 15));
    assert!(lost.iter().all(|&t| less[(t + 40) as usize] == 0));
    assert_eq!(less.iter().filter(|&&w| w == 0).count(), lost.len());
    assert_eq!(
        parse(&arrays_knock("0 1 2 4", 9, 2, 2).unwrap()).unwrap()["lost"].to_string(),
        "[]"
    );

    assert_eq!(arrays_cap("0 1 2 4", 9).unwrap(), 6);
    assert!(arrays_read("0 1 2", 6, 2).is_err());
    assert!(arrays_read("0 1 2", 5, 9).is_err());
    assert!(arrays_read("5", 2, 1).is_err());
}
