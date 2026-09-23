use demos::dissection::*;
use mrlyrs::core::error::parse;
use mrlyrs::core::Json;

fn six(v: &Json) -> String {
    format!("{:.6}", v.as_f64().unwrap())
}

#[test]
fn the_read_of_base_ten_missing_seven_sits_outside_every_wall() {
    let read = parse(&dissection_read(10, 7).unwrap()).unwrap();
    assert_eq!(read["kappa"].to_string(), "[5,6]");
    assert_eq!(read["consecutive"].as_bool(), Some(true));
    assert_eq!(read["level"].as_u64(), Some(6));
    assert_eq!(
        format!("{:.2}", read["masses"][6].as_f64().unwrap()),
        "57350894.06"
    );
    assert_eq!(six(&read["reading"]), "0.346831");
    assert_eq!(six(&read["chain"]), "0.494642");
    assert_eq!(
        read["walls"].to_string(),
        r#"{"chain":584,"window":301,"digit":115,"first":65}"#
    );
    assert_eq!(read["reach"].as_str(), Some("none"));
    assert_eq!(read["depths"].to_string(), "[3,7]");
}

#[test]
fn the_tally_holds_the_meter_and_the_prime_count() {
    let tally = dissection_tally(10, 7, 6).unwrap();
    let read = parse(&tally.read).unwrap();
    assert_eq!(read["count"].as_u64(), Some(531440));
    assert_eq!(read["meter"].as_i64(), Some(-9));
    assert_eq!(six(&read["root"]), "-0.012346");
    assert_eq!(six(&read["primes"]), "0.997990");
    assert_eq!(tally.logx.len(), 720);
}

#[test]
fn the_grid_cuts_the_paper_regions_and_weighs_them() {
    let grid = dissection_grid(10, 7, 3, 8).unwrap();
    let read = parse(&grid.read).unwrap();
    assert_eq!(read["counts"].to_string(), "[732,202,26,40]");
    let shares: Vec<String> = (0..4).map(|i| six(&read["shares"][i])).collect();
    assert_eq!(shares, ["0.505350", "0.191985", "0.006843", "0.295822"]);
    assert_eq!(six(&read["reading"]), "0.330865");
    assert_eq!(grid.weight[0], 1.0);
}

#[test]
fn the_chain_first_drops_below_a_fifth_at_584() {
    let chain = dissection_chain();
    let first = chain.iter().position(|&a| a < 0.2).unwrap() + 3;
    assert_eq!((chain.len(), first), (4094, 584));
}
