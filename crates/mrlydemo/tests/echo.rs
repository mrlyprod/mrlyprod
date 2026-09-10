use mrlycore::json::parse;
use mrlydemo::echo::*;
use mrlynum::design;

fn read(base: u32, mask: u32, depth: usize, subtract: bool) -> (Echo, mrlycore::Json) {
    let echo = echo_read(base, mask, depth, subtract).unwrap();
    let json = parse(&echo.read).unwrap();
    (echo, json)
}

fn shown(value: &mrlycore::Json, places: usize) -> String {
    format!("{:.*}", places, value.as_f64().unwrap())
}

fn census(base: u64, digits: &[u64], depth: usize) -> (i64, i64) {
    let values = design::elements(base, digits, depth);
    let running = design::meter(&design::mobius_of(&values));
    (
        running.last().copied().unwrap(),
        running.iter().map(|value| value.abs()).max().unwrap(),
    )
}

#[test]
fn the_meter_reproduces_the_design_census() {
    let anchors = [
        (3u64, vec![0u64, 1], 14usize, (11i64, 105i64)),
        (3, vec![0, 1], 16, (149, 173)),
        (3, vec![0, 2], 14, (-10, 67)),
        (3, vec![1, 2], 8, (-31, 33)),
    ];
    for (base, digits, depth, want) in anchors {
        assert_eq!(
            census(base, &digits, depth),
            want,
            "lab/mobius-designs and lab/design-meter read base {base} {digits:?} at depth {depth} as (M_F, max abs M_F) {want:?}"
        );
    }
    assert_eq!(design::size(&[0, 1], 14), 16383);
    assert_eq!(design::size(&[0, 1], 20), 1048575);
}

#[test]
fn the_full_set_is_the_classical_mertens_control() {
    let full: Vec<u64> = (0..10).collect();
    let running = design::meter(&design::mobius_of(&design::elements(10, &full, 6)));
    assert_eq!(
        running.last().copied().unwrap(),
        212,
        "A084237 reads M(10^6) as 212 and mu(10^6) is zero"
    );
    let (_, page) = read(10, 0b11_1111_1111, 5, false);
    assert_eq!(page["count"], 99999);
    assert_eq!(page["last"], -48, "A084237 reads M(10^5) as -48");
    assert_eq!(page["peak"], 132);
    assert_eq!(shown(&page["alpha"], 6), "1.000000");
}

#[test]
fn the_control_peaks_land_on_the_zeta_ordinates() {
    let (_, page) = read(10, 0b11_1111_1111, 5, false);
    assert_eq!(page["zeros"].as_array().unwrap().len(), 13);
    assert_eq!(page["lattice"].as_array().unwrap().len(), 20);
    assert_eq!(shown(&page["bin"], 6), "0.545618");
    assert_eq!(
        format!("{} {} {}", page["found"], page["hits"], page["lines"]),
        "13 10 6"
    );
    assert_eq!(shown(&page["chance"], 6), "0.253323");
    assert_eq!(shown(&page["chanceLines"], 6), "0.389727");
    let leading: Vec<String> = page["peaks"]
        .as_array()
        .unwrap()
        .iter()
        .take(5)
        .map(|row| shown(&row["gamma"], 4))
        .collect();
    assert_eq!(leading.join(" "), "30.5546 32.7371 25.0984 21.2791 14.1861");
    let (two, wide) = read(2, 0b11, 18, false);
    assert_eq!(
        format!("{} {} {}", wide["found"], wide["hits"], wide["lines"]),
        "13 10 0"
    );
    assert_eq!(wide["last"], 24);
    assert_eq!(wide["lattice"].as_array().unwrap().len(), 6);
    assert_eq!(two.gamma.len(), 2049);
}

#[test]
fn the_full_set_is_its_own_echo() {
    let (page, full) = read(10, 0b11_1111_1111, 5, false);
    assert_eq!(shown(&full["share"], 6), "1.000000");
    assert_eq!(shown(&full["residual"], 6), "0.000000");
    assert_eq!(
        page.rest.iter().map(|v| v.abs()).fold(0.0f32, f32::max),
        0.0
    );
    assert_eq!(page.echo, page.meter);
}

#[test]
fn the_design_carries_the_zeros_through_its_echo() {
    let (page, meter) = read(10, 0b1_1111_1111, 5, false);
    assert_eq!(meter["count"], 59048);
    assert_eq!(meter["last"], 201);
    assert_eq!(meter["peak"], 268);
    assert_eq!(shown(&meter["alpha"], 6), "0.954243");
    assert_eq!(shown(&meter["span"], 6), "11.395132");
    assert_eq!(shown(&meter["bin"], 6), "0.551257");
    assert_eq!(
        format!("{} {} {}", meter["found"], meter["hits"], meter["lines"]),
        "5 5 2"
    );
    assert_eq!(shown(&meter["chance"], 6), "0.255941");
    assert_eq!(shown(&meter["chanceLines"], 6), "0.393755");
    assert_eq!(shown(&meter["peaks"][0]["gamma"], 4), "14.3327");
    assert_eq!(shown(&meter["peaks"][0]["score"], 4), "32.2266");
    assert_eq!(shown(&meter["peaks"][0]["zeta"], 4), "0.1980");
    assert_eq!(shown(&meter["share"], 6), "0.354137");
    assert_eq!(shown(&meter["residual"], 6), "0.967810");
    assert_eq!(shown(&meter["rate"], 6), "-0.022879");
    assert_eq!(page.logx.len(), 4096);

    let (_, rest) = read(10, 0b1_1111_1111, 5, true);
    assert_eq!(
        format!("{} {}", rest["found"], rest["hits"]),
        "0 0",
        "the residual carries no peak once the echo is taken out"
    );
}

#[test]
fn the_caps_hold_the_depth_and_the_sieve() {
    let caps = parse(&echo_caps(3, 0b011).unwrap()).unwrap();
    assert_eq!(
        format!("{} {} {}", caps["deepest"], caps["sieved"], caps["least"]),
        "17 15 3"
    );
    assert_eq!(shown(&caps["alpha"], 6), "0.630930");
    assert_eq!(caps["samples"], 4096);
    let (_, deep) = read(3, 0b011, 17, false);
    assert_eq!(deep["sieve"], false);
    assert_eq!(deep["share"], mrlycore::Json::Null);
    let (thin, shallow) = read(3, 0b011, 14, false);
    assert_eq!(deep["last"], 157);
    assert_eq!(shallow["last"], 11);
    assert_eq!(shallow["peak"], 105);
    assert_eq!(shallow["sieve"], true);
    assert_eq!(thin.echo.len(), 4096);
    assert_eq!(shown(&shallow["rate"], 6), "-0.184535");

    assert!(echo_caps(11, 0b011).is_err());
    assert!(echo_caps(3, 0b1011).is_err());
    assert!(echo_caps(3, 0b001).is_err());
    assert!(echo_read(3, 0b011, 18, false).is_err());
    assert!(echo_read(3, 0b011, 2, false).is_err());
}
