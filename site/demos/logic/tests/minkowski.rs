use demos::minkowski::*;
use mrlyrs::core::error::parse;
use std::f64::consts::SQRT_2;

#[test]
fn the_slice_reads_the_centre_and_a_subcube_centre() {
    let slice = minkowski_slice(0.5, 3).unwrap();
    assert_eq!(slice.len(), 9);
    assert_eq!(slice[4], (SQRT_2 / 6.0) as f32);
    assert_eq!(slice[0], (SQRT_2 / 18.0) as f32);
    assert!(minkowski_slice(0.5, 730).is_err());
}

#[test]
fn the_read_holds_the_enclosures_and_the_open_window() {
    let read = parse(&minkowski_read(1.0 / 12.0)).unwrap();
    let tube = read["tube"].as_f64().unwrap();
    let profile = read["profile"].as_f64().unwrap();
    assert!((0.180947086..=0.180947093).contains(&tube));
    assert!((2.122718..=2.122723).contains(&profile));
    assert!(read["reading"].as_f64().unwrap() < profile);
    let open = parse(&minkowski_read(0.2)).unwrap();
    assert!(open["tube"].is_null() && open["profile"].is_null() && open["reading"].is_null());
}

#[test]
fn the_walk_breaks_on_the_open_phases_and_swings() {
    let walk = parse(&minkowski_walk(8, 72).unwrap()).unwrap();
    let profile = walk["profile"].as_array().unwrap();
    let reading = walk["reading"].as_array().unwrap();
    assert_eq!(walk["u"].as_array().unwrap().len(), 576);
    let open = profile.iter().filter(|v| v.is_null()).count();
    assert_eq!(open, 8 * 23);
    assert!(profile
        .iter()
        .zip(reading)
        .all(|(p, r)| p.is_null() == r.is_null()));
    let (low, high) = (
        walk["low"].as_f64().unwrap(),
        walk["high"].as_f64().unwrap(),
    );
    assert!(low < 2.123 && high > 2.135);
    assert!(walk["swing"].as_f64().unwrap() >= 0.5792);
    assert!(profile
        .iter()
        .zip(reading)
        .all(|(p, r)| p.is_null() || r.as_f64() < p.as_f64()));
}
