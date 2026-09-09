use mrlycore::json::parse;
use mrlydemo::snail::*;

#[test]
fn the_snail_exports_answer() {
    let cells = snail_cells(3, 100, "every").unwrap();
    assert_eq!(cells.len(), 500);
    assert_eq!(&cells[0..10], &[0, 0, 1, 0, 0, 1, 0, 1, 0, 1]);
    assert_eq!(&cells[10..20], &[1, 1, 3, 1, 1, -2, 1, 3, 1, 0]);
    assert_eq!(&cells[40..50], &[1, -5, 9, 2, 0, 10, -5, 9, 2, 0]);
    let every = parse(&snail_read(3, 100, "every").unwrap()).unwrap();
    assert_eq!(every["tiles"], 100);
    assert_eq!(every["primes"], 25);
    assert_eq!(every["grown"], 98);
    assert_eq!(every["levels"].to_string(), "[2,6,18,54,20]");
    assert_eq!(every["peak"], 4);
    assert_eq!(every["side"], 81);
    assert_eq!(every["area"], 172_100);
    assert_eq!(every["low"].to_string(), "[-602,-86]");
    assert_eq!(every["high"].to_string(), "[208,724]");
    assert_eq!(every["width"], 810);
    assert_eq!(every["height"], 810);

    let marked = snail_cells(3, 100, "prime").unwrap();
    assert_eq!(&marked[15..25], &[-2, 1, 1, 0, 0, -3, 1, 3, 1, 1]);
    let prime = parse(&snail_read(3, 100, "prime").unwrap()).unwrap();
    assert_eq!(prime["primes"], 25);
    assert_eq!(prime["grown"], 24);
    assert_eq!(prime["levels"].to_string(), "[76,3,5,13,3]");
    assert_eq!(prime["area"], 29_668);
    assert_eq!(prime["width"], 170);
    assert_eq!(prime["height"], 250);

    let shell = parse(&snail_read(3, 300, "prime").unwrap()).unwrap();
    assert_eq!(shell["tiles"], 300);
    assert_eq!(shell["primes"], 62);
    assert_eq!(shell["levels"].to_string(), "[239,3,5,13,31,9]");
    assert_eq!(shell["side"], 243);
    assert_eq!(shell["area"], 744_980);
    assert_eq!(shell["width"], 986);
    assert_eq!(shell["height"], 1227);

    let full = parse(&snail_read(3, 300, "every").unwrap()).unwrap();
    assert_eq!(full["levels"].to_string(), "[2,6,18,54,162,58]");
    assert_eq!(full["area"], 4_528_604);
    assert_eq!(full["width"], 4374);
    assert_eq!(full["height"], 4131);

    let wide = parse(&snail_read(2, 2000, "every").unwrap()).unwrap();
    assert_eq!(wide["peak"], 10);
    assert_eq!(wide["side"], 1024);

    assert!(snail_cells(3, 100, "some").is_err());
    assert!(snail_cells(1, 100, "every").is_err());
    assert!(snail_cells(9, 100, "every").is_err());
    assert!(snail_cells(3, 0, "every").is_err());
    assert!(snail_cells(3, 2001, "every").is_err());
}
