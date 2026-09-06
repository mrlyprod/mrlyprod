use mrlyweb::modes::*;
use mrlyweb::two::two_grid;
use std::f64::consts::TAU;

fn direct(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    t1: usize,
    t2: usize,
) -> (f64, f64) {
    let grid = two_grid(code, number, level, 0, base).unwrap();
    let span = grid.width as usize;
    let (mut re, mut im) = (0.0, 0.0);
    for x1 in 0..span {
        for x2 in 0..span {
            if grid.types[x1 * span + x2] == 0 {
                continue;
            }
            let angle = TAU * ((t1 * x1 + t2 * x2) % span) as f64 / span as f64;
            re += angle.cos();
            im += angle.sin();
        }
    }
    (re, im)
}

fn gap(code: &str, number: usize, level: usize, base: usize) -> f64 {
    let span = number.pow(level as u32);
    let mass = f64::from(
        two_grid(code, number, 1, 0, base)
            .unwrap()
            .types
            .iter()
            .map(|&b| u32::from(b != 0))
            .sum::<u32>(),
    )
    .powi(level as i32);
    let field = modes_field(code, number, level, base).unwrap();
    assert_eq!(field.len(), span * span);
    let mut worst: f64 = 0.0;
    for t1 in 0..span {
        for t2 in 0..span {
            let (re, im) = direct(code, number, level, base, t1, t2);
            let read = modes_value(code, number, level, base, t1, t2).unwrap();
            worst = worst.max((read[0] - re).abs());
            worst = worst.max((read[1] - im).abs());
            worst = worst.max((read[2] - re.hypot(im) / mass).abs());
            worst = worst.max((f64::from(field[t1 * span + t2]) - re.hypot(im) / mass).abs());
        }
    }
    worst
}

#[test]
fn the_product_formula_is_the_masks_transform() {
    assert!(gap("7", 2, 3, 2) < 1e-6);
    assert!(gap("9", 2, 3, 2) < 1e-6);
    assert!(gap("495", 3, 2, 3) < 1e-6);
    assert!(gap("127", 3, 2, 3) < 1e-6);
}

#[test]
fn the_modes_exports_answer() {
    let gasket = modes_field("7", 2, 3, 2).unwrap();
    let carpet = modes_field("495", 3, 2, 3).unwrap();
    let runner = modes_field("127", 3, 2, 3).unwrap();
    let top = |field: &[f32]| format!("{:.6}", field[1..].iter().copied().fold(f32::MIN, f32::max));
    assert_eq!(format!("{},{}", gasket.len(), carpet.len()), "64,81");
    assert_eq!(format!("{},{}", gasket[0], carpet[0]), "1,1");
    assert_eq!(format!("{:.6}", gasket[1]), "0.231717");
    assert_eq!(top(&gasket), "0.333333");
    assert_eq!(format!("{:.6}", carpet[1]), "0.103067");
    assert_eq!(top(&carpet), "0.125000");
    assert_eq!(format!("{:.6}", runner[9]), "0.253018");
    assert_eq!(top(&runner), "0.285714");

    assert_eq!(modes_large("9", 2, 3, 2, 0.5).unwrap(), 8);
    assert_eq!(modes_large("7", 2, 3, 2, 0.25).unwrap(), 4);
    assert_eq!(modes_large("495", 3, 2, 3, 0.1).unwrap(), 13);
    assert_eq!(modes_large("127", 3, 2, 3, 0.1).unwrap(), 17);
    assert_eq!(modes_large("495", 3, 5, 3, 0.1).unwrap(), 25);

    let one = modes_value("495", 3, 2, 3, 1, 1).unwrap();
    assert_eq!(
        format!("{:.6},{:.6},{:.6}", one[0], one[1], one[2]),
        "-4.145430,3.478429,0.084554"
    );
    let two = modes_value("127", 3, 2, 3, 1, 2).unwrap();
    assert_eq!(
        format!("{:.6},{:.6},{:.6}", two[0], two[1], two[2]),
        "3.145430,-1.508813,0.071196"
    );

    let mode = modes_pattern("495", 3, 2, 3, 1, 2).unwrap();
    assert_eq!(format!("{},{}", mode.len(), mode[0]), "81,1");
    assert_eq!(modes_digits("495", 3, 3).unwrap(), 8);
    assert_eq!(modes_digits("127", 3, 3).unwrap(), 7);
    assert_eq!(modes_digits("7", 2, 2).unwrap(), 3);
}

#[test]
fn the_torus_is_capped_and_the_empty_design_refused() {
    assert!(modes_field("495", 3, 6, 3).is_err());
    assert!(modes_field("0", 3, 2, 3).is_err());
}
