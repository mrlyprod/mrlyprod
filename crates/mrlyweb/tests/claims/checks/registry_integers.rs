use mrlycore::json::parse;
use mrlycore::Json;
use mrlyweb::census::{
    census_champions, census_misses, census_report, census_walk, census_window, census_writers,
};

const TIERS: [(&str, u64); 4] = [
    ("closed", 7692),
    ("convolved", 5044),
    ("side", 2665),
    ("level", 2665),
];

const LADDER: [u64; 4] = [783, 929, 955, 959];

const MISSES: [u64; 10] = [269, 362, 422, 443, 446, 487, 502, 538, 607, 611];

const CHAMPIONS: [(u64, u64); 10] = [
    (16, 2858),
    (9, 2811),
    (4, 2559),
    (12, 2303),
    (36, 2270),
    (64, 2176),
    (3, 1951),
    (6, 1883),
    (8, 1790),
    (33, 1777),
];

// THE WINDOW

pub fn the_census_of_1_100000_over_the() -> Result<(), String> {
    let window = read(&census_window())?;
    let rows = number("registry", &window["registry"])?;
    if rows != 18066 {
        return Err(format!("the registry lists {rows} rows"));
    }
    let mut total = 0;
    for (slot, (tier, keys)) in TIERS.iter().enumerate() {
        let listed = &window["tiers"][slot];
        if listed["tier"] != *tier {
            return Err(format!("tier {slot} is not {tier}"));
        }
        let count = number(tier, &listed["keys"])?;
        if count != *keys {
            return Err(format!("the {tier} tier lists {count} rows"));
        }
        total += count;
    }
    if total != rows {
        return Err(format!("the tiers list {total} rows against {rows}"));
    }
    Ok(())
}

// THE DEEP SWEEP

pub fn the_miss_set_s_arithmetic_269_a() -> Result<(), String> {
    loop {
        let walk = read(&census_walk(500))?;
        if walk["complete"] == true {
            break;
        }
    }
    let report = read(&census_report())?;
    for (field, want) in [
        ("rows", 18066),
        ("depth", 48),
        ("never", 41),
        ("once", 31),
        ("multiple", 928),
        ("written", 959),
        ("first_miss", 269),
        ("run", 268),
        ("low", 29144),
        ("incidences", 193419),
    ] {
        let got = number(field, &report[field])?;
        if got != want {
            return Err(format!("the sweep reads {field} at {got}"));
        }
    }
    let missed = number("missed", &report["bands"][2]["missed"])?;
    if missed != 41 {
        return Err(format!("the third band misses {missed}"));
    }
    let ladder = column(&report["depths"], "written")?;
    if ladder != LADDER {
        return Err(format!("the depth ladder reads {ladder:?}"));
    }
    let champions = read(&census_champions(10))?;
    let top: Vec<(u64, u64)> = column(&champions, "value")?
        .into_iter()
        .zip(column(&champions, "rows")?)
        .collect();
    if top != CHAMPIONS {
        return Err(format!("the champions read {top:?}"));
    }
    let misses = read(&census_misses(10))?;
    let opening: Vec<u64> = misses
        .as_array()
        .ok_or("the miss list is not an array")?
        .iter()
        .filter_map(Json::as_u64)
        .collect();
    if opening != MISSES {
        return Err(format!("the miss set opens {opening:?}"));
    }
    let writers = number("rows", &read(&census_writers(269, 0, 1))?["rows"])?;
    if writers != 0 {
        return Err(format!("the first miss is written by {writers} rows"));
    }
    let champion = read(&census_writers(16, 0, 1))?;
    let rows = number("rows", &champion["rows"])?;
    if rows != 2858 {
        return Err(format!("the champion is written by {rows} rows"));
    }
    let by_tier = column(&champion["tiers"], "rows")?;
    if by_tier != [666, 1529, 530, 133] {
        return Err(format!("the champion splits {by_tier:?}"));
    }
    Ok(())
}

// READING

fn read(text: &str) -> Result<Json, String> {
    parse(text).map_err(|_| "the census does not print json".to_string())
}

fn number(field: &str, value: &Json) -> Result<u64, String> {
    value.as_u64().ok_or(format!("{field} is not a number"))
}

fn column(rows: &Json, field: &str) -> Result<Vec<u64>, String> {
    rows.as_array()
        .ok_or(format!("the {field} column is not an array"))?
        .iter()
        .map(|row| number(field, &row[field]))
        .collect()
}
