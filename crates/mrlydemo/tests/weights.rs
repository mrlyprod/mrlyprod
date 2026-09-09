use mrlycore::colors::DARK;
use mrlydemo::bang::random_between;
use mrlydemo::weights::*;

const OBJECT: &str = "69";
const CARPET: &str = "495";
const MASSES: [f64; 3] = [3.0, 2.0, 3.0];

fn point(weights: &[f64], s: f64) -> Vec<f64> {
    weights_point(OBJECT, 3, 3, weights, s).unwrap()
}

fn nine(values: &[f64]) -> String {
    values
        .iter()
        .map(|v| format!("{v:.9}"))
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn the_corner_order_is_the_row_major_tile() {
    assert_eq!(
        weights_corners(OBJECT, 3, 3).unwrap(),
        vec![0, 0, 0, 2, 2, 0]
    );
    assert_eq!(weights_corners(CARPET, 3, 3).unwrap().len(), 16);
    assert!(weights_corners("0", 3, 3).is_err());
    assert!(weights_dims(OBJECT, 3, 3, &[1.0, 1.0]).is_err());
    assert!(weights_dims(OBJECT, 3, 3, &[1.0, 0.0, 1.0]).is_err());
}

#[test]
fn the_pressure_is_the_research_pages_table() {
    assert_eq!(
        nine(&point(&MASSES, -2.0)),
        "-2.000000000 3.102620937 1.088179391 0.926262155"
    );
    assert_eq!(
        nine(&point(&MASSES, -1.0)),
        "-1.000000000 2.033103256 1.050962223 0.982141033"
    );
    assert_eq!(
        nine(&point(&MASSES, 0.0)),
        "0.000000000 1.000000000 1.015812676 1.000000000"
    );
    assert_eq!(
        nine(&point(&MASSES, 1.0)),
        "1.000000000 0.000000000 0.985056822 0.985056822"
    );
    assert_eq!(
        nine(&point(&MASSES, 2.0)),
        "2.000000000 -0.971990429 0.959892942 0.947795455"
    );
    assert_eq!(
        nine(&point(&MASSES, 3.0)),
        "3.000000000 -1.921688171 0.940411228 0.899545513"
    );
    let walk = weights_pressure(OBJECT, 3, 3, &MASSES, -2.0, 3.0, 6).unwrap();
    assert_eq!(walk.len(), 12);
    let read: Vec<String> = walk
        .chunks(2)
        .map(|pair| format!("{:.6}", pair[1]))
        .collect();
    assert_eq!(
        read.join(" "),
        "3.102621 2.033103 1.000000 0.000000 -0.971990 -1.921688"
    );
    assert_eq!(walk[0], -2.0);
    assert!(weights_pressure(OBJECT, 3, 3, &MASSES, 3.0, -2.0, 6).is_err());
}

#[test]
fn the_local_dimensions_bracket_the_information_exponent() {
    assert_eq!(
        nine(&weights_dims(OBJECT, 3, 3, &MASSES).unwrap()),
        "0.892789261 1.261859507 0.985056822 1.000000000"
    );
    let seed = random_between(7, &[1; 8], &[32; 8]);
    let drawn: Vec<f64> = seed.iter().map(|n| f64::from(*n)).collect();
    assert!(drawn.iter().any(|w| *w != drawn[0]));
    let read = weights_dims(CARPET, 3, 3, &drawn).unwrap();
    assert!(read[0] < read[2] && read[2] < read[1], "{read:?}");
    assert!((read[3] - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
}

#[test]
fn equal_weights_reproduce_the_zero_one_design() {
    for (code, count) in [(OBJECT, 3.0), (CARPET, 8.0)] {
        let flat = vec![1.0; count as usize];
        let dimension = f64::ln(count) / 3f64.ln();
        for step in -6..=6 {
            let s = f64::from(step) / 2.0;
            let read = weights_point(code, 3, 3, &flat, s).unwrap();
            assert!((read[1] - (1.0 - s) * dimension).abs() < 1e-12, "{read:?}");
            assert!((read[2] - dimension).abs() < 1e-12);
        }
        assert_eq!(
            nine(&weights_point(code, 3, 3, &flat, 2.5).unwrap()),
            format!(
                "2.500000000 {:.9} {dimension:.9} {dimension:.9}",
                -1.5 * dimension
            )
        );
        let curve = weights_spectrum(code, 3, 3, &flat, 64).unwrap();
        assert!(curve
            .chunks(2)
            .all(|pair| (f64::from(pair[0]) - dimension).abs() < 1e-6
                && (f64::from(pair[1]) - dimension).abs() < 1e-6));
    }
}

#[test]
fn the_masses_are_integers_over_the_denominator_to_the_level() {
    let field = weights_mass(OBJECT, 3, 2, 3, &MASSES).unwrap();
    assert_eq!(field.len(), 81);
    assert_eq!(field[0], 9.0 / 64.0);
    assert_eq!(field[2], 6.0 / 64.0);
    assert_eq!(field[8], 4.0 / 64.0);
    assert_eq!(field.iter().filter(|mass| **mass > 0.0).count(), 9);
    let counted: f64 = field.iter().map(|mass| f64::from(*mass) * 64.0).sum();
    assert!((counted - 64.0).abs() < 1e-12);
    let deep = weights_mass(OBJECT, 3, 6, 3, &MASSES).unwrap();
    assert_eq!(deep.len(), 531441);
    assert_eq!(deep.iter().filter(|mass| **mass > 0.0).count(), 729);
    let peak = deep.iter().copied().fold(f32::MIN, f32::max);
    let least = deep
        .iter()
        .copied()
        .filter(|mass| *mass > 0.0)
        .fold(f32::MAX, f32::min);
    assert!((f64::from(peak / least) - 1.5f64.powi(6)).abs() < 1e-5);
    assert!(weights_mass(OBJECT, 3, 7, 3, &MASSES).is_err());
    assert!(weights_mass(OBJECT, 3, 0, 3, &MASSES).is_err());
}

#[test]
fn the_spectrum_spans_the_whole_range_under_the_transform() {
    let read = weights_dims(OBJECT, 3, 3, &MASSES).unwrap();
    let curve = weights_spectrum(OBJECT, 3, 3, &MASSES, 241).unwrap();
    assert_eq!(curve.len(), 482);
    assert!((f64::from(curve[0]) - read[1]).abs() < 1e-4);
    assert!((f64::from(curve[480]) - read[0]).abs() < 1e-4);
    assert_eq!(
        format!("{:.6},{:.6}", curve[0], curve[480]),
        "1.261856,0.892790"
    );
    for pair in curve.chunks(2) {
        let (alpha, f) = (f64::from(pair[0]), f64::from(pair[1]));
        assert!(
            alpha >= read[0] - 1e-6 && alpha <= read[1] + 1e-6,
            "{alpha}"
        );
        assert!(f <= read[3] + 1e-6 && f >= -1e-6, "{f}");
        for step in -8..=8 {
            let s = f64::from(step) / 2.0;
            let held = point(&MASSES, s);
            assert!(f <= alpha * s + held[1] + 1e-6, "{alpha} {f} at {s}");
        }
    }
    let held = point(&MASSES, 1.0);
    assert!((held[2] - held[3]).abs() < 1e-12);
}

#[test]
fn the_painted_field_is_the_support_on_the_ground() {
    let sheet = weights_pixels(OBJECT, 3, 4, 3, &MASSES, 0.35).unwrap();
    assert_eq!((sheet.width, sheet.height), (81, 81));
    let ground = sheet
        .rgba
        .chunks(4)
        .filter(|c| c[..3] == [DARK.ground.r, DARK.ground.g, DARK.ground.b])
        .count();
    assert_eq!(ground, 81 * 81 - 81);
    let yellow = DARK.yellow;
    assert_eq!(&sheet.rgba[0..4], &[yellow.r, yellow.g, yellow.b, 255]);
    assert!(weights_pixels(OBJECT, 3, 4, 3, &MASSES, 0.0).is_err());
}
