use mrlycore::json::parse;
use mrlyweb::census::*;

#[test]
#[ignore = "the whole registry to 48 terms takes minutes; run it with --release --ignored"]
fn the_pinned_window_matches_the_research_page() {
    loop {
        let walk = parse(&census_walk(500)).unwrap();
        if walk["complete"] == true {
            break;
        }
    }
    let report = parse(&census_report()).unwrap();
    assert_eq!(report["rows"], 18066);
    assert_eq!(report["depth"], 48);
    assert_eq!(
        (
            report["never"].clone(),
            report["once"].clone(),
            report["multiple"].clone()
        ),
        (41.into(), 31.into(), 928.into())
    );
    assert_eq!(report["written"], 959);
    assert_eq!(report["first_miss"], 269);
    assert_eq!(report["run"], 268);
    assert_eq!(report["low"], 29144);
    assert_eq!(report["incidences"], 193419);
    assert_eq!(report["bands"][2]["missed"], 41);
    let ladder: Vec<u64> = report["depths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|depth| depth["written"].as_u64().unwrap())
        .collect();
    assert_eq!(ladder, [783, 929, 955, 959]);
    let champions = parse(&census_champions(10)).unwrap();
    let top: Vec<(u64, u64)> = champions
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["value"].as_u64().unwrap(),
                row["rows"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        top,
        [
            (16, 2858),
            (9, 2811),
            (4, 2559),
            (12, 2303),
            (36, 2270),
            (64, 2176),
            (3, 1951),
            (6, 1883),
            (8, 1790),
            (33, 1777)
        ]
    );
    assert_eq!(
        parse(&census_misses(10)).unwrap(),
        parse("[269, 362, 422, 443, 446, 487, 502, 538, 607, 611]").unwrap()
    );
    assert_eq!(parse(&census_writers(269, 0, 1)).unwrap()["rows"], 0);
    let champion = parse(&census_writers(16, 0, 1)).unwrap();
    assert_eq!(champion["rows"], 2858);
    let by_tier: Vec<u64> = champion["tiers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tier| tier["rows"].as_u64().unwrap())
        .collect();
    assert_eq!(by_tier, [666, 1529, 530, 133]);
}

#[test]
fn the_pinned_prefix_matches_the_research_page() {
    loop {
        let walk = parse(&census_walk(500)).unwrap();
        if walk["done"] == walk["total"] {
            assert_eq!(walk["depth"], 8);
            assert_eq!(walk["total"], 18066);
            break;
        }
    }
    let report = parse(&census_report()).unwrap();
    assert_eq!(report["depth"], 8);
    assert_eq!(report["rows"], 18066);
    assert_eq!(report["first_miss"], 269);
    assert_eq!(report["written"], 783);
    let ladder: Vec<u64> = report["depths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|depth| depth["written"].as_u64().unwrap())
        .collect();
    assert_eq!(ladder, [783]);
}
