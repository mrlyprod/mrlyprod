use mrlycore::json::parse;
use mrlydemo::spirograph::*;
use mrlydemo::two::two_grid;

fn carpet() -> Vec<u8> {
    let grid = two_grid("495", 3, 1, 0, 3).unwrap();
    assert_eq!((grid.width, grid.height), (3, 3));
    assert_eq!(grid.types.iter().map(|&b| b as usize).sum::<usize>(), 8);
    assert_eq!(grid.types[4], 0);
    grid.types
}

fn read(pens: &str, track: &str, ring: usize, wheel: usize, laps: usize) -> mrlycore::Json {
    parse(
        &spirograph_read(
            &carpet(),
            3,
            3,
            pens,
            track,
            ring,
            wheel,
            4,
            laps,
            0.9,
            0.0,
            1,
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn the_carpet_seats_eight_pencils_and_the_law_counts_the_curves() {
    let four = read("fill", "in", 7, 4, 1);
    assert_eq!(four["pencils"], 8);
    assert_eq!(four["fills"], 8);
    assert_eq!(four["distinct"], 2);
    assert_eq!(read("fill", "in", 7, 2, 1)["distinct"], 4);
    assert_eq!(read("fill", "in", 7, 3, 1)["distinct"], 8);
    assert_eq!(read("fill", "line", 7, 3, 2)["distinct"], 2);
    assert_eq!(read("void", "in", 7, 3, 1)["pencils"], 1);
    assert_eq!(read("corners", "in", 7, 3, 1)["corners"], 16);
}

#[test]
fn the_ring_over_the_wheel_closes_the_circle() {
    let seven = read("fill", "in", 7, 3, 1);
    assert_eq!(
        format!(
            "{} {} {} {}",
            seven["a"], seven["b"], seven["orbits"], seven["fold"]
        ),
        "7 3 3 7"
    );
    assert_eq!(
        format!("{:.6}", seven["turns"].as_f64().unwrap()),
        "4.000000"
    );
    let trace = spirograph(&carpet(), 3, 3, "fill", "in", 7, 3, 4, 1, 0.9, 0.0, 1, 720).unwrap();
    assert_eq!(trace.len(), 8 * 720 * 2);
    assert!((trace[0] - trace[1438]).abs() < 1e-4 && (trace[1] - trace[1439]).abs() < 1e-4);
    let start = spirograph_pose("in", 7, 3, 4, 1, 0.0).unwrap();
    let end = spirograph_pose("in", 7, 3, 4, 1, 1.0).unwrap();
    assert_eq!(
        format!("{:.6} {:.6} {:.6}", start[0], start[1], start[2]),
        "4.000000 0.000000 0.000000"
    );
    assert_eq!(
        format!("{:.6} {:.6}", end[0], end[2] / std::f64::consts::PI),
        "4.000000 -8.000000"
    );
}

#[test]
fn the_nodes_come_back_only_inside_the_seat_window() {
    assert_eq!(read("fill", "in", 7, 3, 1)["nodes"], 1288);
    assert_eq!(read("fill", "in", 7, 4, 1)["nodes"], 98);
    assert_eq!(read("fill", "out", 5, 8, 1)["nodes"], 150);
    assert!(read("fill", "in", 7, 6, 1)["nodes"].is_null());
    assert!(read("fill", "line", 7, 3, 2)["nodes"].is_null());
    assert!(read("void", "in", 7, 3, 1)["nodes"].is_null());
    let past =
        parse(&spirograph_read(&carpet(), 3, 3, "fill", "in", 7, 4, 4, 1, 1.35, 0.0, 1).unwrap())
            .unwrap();
    assert_eq!(past["distinct"], 2);
    assert!(past["nodes"].is_null());
    let far =
        parse(&spirograph_read(&carpet(), 3, 3, "corners", "in", 7, 3, 4, 1, 1.2, 0.0, 1).unwrap())
            .unwrap();
    assert_eq!(far["corners"], 16);
    assert!(far["nodes"].is_null());
}

#[test]
fn a_wheel_that_does_not_fit_is_refused() {
    assert!(spirograph_read(&carpet(), 3, 3, "fill", "in", 3, 3, 4, 1, 0.9, 0.0, 1).is_err());
    assert!(spirograph_read(&carpet(), 3, 3, "fill", "polyin", 4, 3, 3, 1, 0.9, 0.0, 1).is_err());
    assert!(spirograph_read(&carpet(), 3, 3, "edges", "in", 7, 3, 4, 1, 0.9, 0.0, 1).is_err());
}

#[test]
fn the_cover_walls_the_carpet_and_the_winding_checks_it() {
    let side = 256;
    let cover =
        spirograph_cover(&carpet(), 3, 3, "fill", "in", 7, 3, 4, 1, 0.9, 0.0, 1, side).unwrap();
    assert_eq!(cover.side as usize, side);
    assert_eq!(cover.mask.len(), side * side);
    assert_eq!(
        format!(
            "{:.6} {:.6} {:.6} {:.6} {:.6}",
            cover.covered, cover.hole, cover.wall, cover.winding, cover.areas
        ),
        "0.814487 0.140825 0.269255 9.104609 9.103448"
    );
    let disc = &cover.disc;
    assert_eq!(
        format!("{:.6} {:.6} {:.6}", disc[0], disc[2], disc[3]),
        "0.000000 5.800000 2.200000"
    );
    let inside = cover.mask.iter().filter(|&&code| code > 0).count();
    let shape = cover.mask.iter().filter(|&&code| code == 3).count();
    assert_eq!(format!("{:.6}", shape as f64 / inside as f64), "0.814487");
    let d = 0.9 / (1.5_f64).hypot(0.5);
    for (kind, rho, want) in [
        ("in", 2.0, "0.014922 0.500622 0.508005 0.507814"),
        ("out", 4.0, "0.016671 0.819733 0.828554 0.828445"),
    ] {
        let one = spirograph_cover(
            &[1, 0, 0],
            3,
            1,
            "fill",
            kind,
            3,
            1,
            4,
            1,
            0.9,
            0.0,
            1,
            side,
        )
        .unwrap();
        let sign = if kind == "in" { -1.0 } else { 1.0 };
        let form = rho * (rho + sign * d * d) / (rho + d).powi(2);
        assert!((one.areas - form).abs() < 1e-12);
        assert_eq!(
            format!(
                "{:.6} {:.6} {:.6} {:.6}",
                one.covered, one.hole, one.winding, form
            ),
            want
        );
    }
    assert!(spirograph_cover(
        &carpet(),
        3,
        3,
        "fill",
        "line",
        7,
        3,
        4,
        2,
        0.9,
        0.0,
        1,
        side
    )
    .is_err());
}
