use demos::sumset::*;
use mrlyrs::core::error::parse;
use mrlyrs::core::Json;

fn read(level: u32, x: u32) -> Json {
    parse(&sumset_read(level, x).unwrap()).unwrap()
}

fn six(v: f64) -> u64 {
    (v * 1e6).floor() as u64
}

#[test]
fn the_read_counts_s_at_the_powers_of_three() {
    let rows: Vec<Json> = (4..=10).map(|k| read(10, 3u32.pow(k))).collect();
    let counts: Vec<u64> = rows.iter().map(|r| r["count"].as_u64().unwrap()).collect();
    let densities: Vec<u64> = rows
        .iter()
        .map(|r| six(r["density"].as_f64().unwrap()))
        .collect();
    assert_eq!(counts, [79, 203, 626, 1941, 5963, 17025, 45968]);
    assert_eq!(
        densities,
        [975308, 835390, 858710, 887517, 908855, 864959, 778472]
    );
}

#[test]
fn the_deepest_dip_sits_just_below_three_to_the_fifteen() {
    let dip = read(16, 14348906);
    assert_eq!(dip["count"].as_u64(), Some(10953840));
    assert_eq!(six(dip["density"].as_f64().unwrap()), 763391);
    assert_eq!(dip["member"].as_bool(), Some(false));
    let envelope = sumset_envelope(16, 720).unwrap();
    let low = envelope
        .iter()
        .filter(|v| !v.is_nan())
        .fold(f32::INFINITY, |a, &b| a.min(b));
    let high = envelope
        .iter()
        .filter(|v| !v.is_nan())
        .fold(0f32, |a, &b| a.max(b));
    assert_eq!((format!("{low:.5}"), high), ("0.76339".to_string(), 1.0));
}

#[test]
fn the_strip_goes_dark_on_a367090() {
    let strip = sumset_strip(6, 0, 250, 250).unwrap();
    let dark: Vec<usize> = (0..250).filter(|&i| strip[i] == 0.0).collect();
    assert_eq!(dark.len(), 40);
    assert_eq!(dark[..6], [62, 63, 143, 144, 207, 208]);
}

#[test]
fn the_pairs_carry_the_census_readings() {
    let rows = parse(&sumset_pairs(16).unwrap()).unwrap();
    let rows = rows.as_array().unwrap();
    let flag = |key: &str| {
        rows.iter()
            .filter(|r| r[key].as_bool() == Some(true))
            .count()
    };
    assert_eq!((rows.len(), flag("clean"), flag("copy")), (17, 6, 5));
    let up: Vec<u64> = rows[..4]
        .iter()
        .map(|r| (r["ratio"].as_f64().unwrap() * 1e6).ceil() as u64)
        .collect();
    assert_eq!(up, [1467705, 1638125, 1664808, 1724517]);
    let centre = rows
        .iter()
        .find(|r| r["three"] == 15 && r["four"] == 12)
        .unwrap();
    assert_eq!(centre["gap"].to_string(), "[12766859,14348906]");
    assert_eq!(centre["energy"].as_str(), Some("2737906338"));
}

#[test]
fn the_height_and_the_cursor_are_refused_outside_their_range() {
    assert!(sumset_read(17, 1).is_err() && sumset_read(5, 1).is_err());
    assert!(sumset_read(8, 0).is_err() && sumset_read(8, 6562).is_err());
    assert!(sumset_strip(8, 10, 12, 3).is_err() && sumset_envelope(8, 0).is_err());
}
