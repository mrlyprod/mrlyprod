use demos::star::*;

#[test]
fn the_stack_and_its_band_share_one_raster() {
    let field = star_field("23", 28, 64).unwrap();
    assert_eq!(field.len(), 4096);
    let inside: Vec<f64> = field
        .iter()
        .filter(|value| !value.is_nan())
        .map(|value| f64::from(*value))
        .collect();
    assert_eq!(inside.len(), 2746);
    assert_eq!(
        format!("{:.6}", inside.iter().sum::<f64>() / inside.len() as f64),
        "0.527208"
    );
    assert_eq!(format!("{}", field[32 * 64 + 32]), "0.5");

    let arm = star_band("23", 28, 64, 0).unwrap();
    let tally = |grid: &demos::Grid, kind: u8| grid.types.iter().filter(|t| **t == kind).count();
    assert_eq!(
        format!("{},{},{}", tally(&arm, 0), tally(&arm, 1), tally(&arm, 2)),
        "34,2712,1350"
    );
    let wide = star_band("23", 28, 64, 4).unwrap();
    assert_eq!(
        format!(
            "{},{},{}",
            tally(&wide, 0),
            tally(&wide, 1),
            tally(&wide, 2)
        ),
        "257,2489,1350"
    );
    assert_eq!(format!("{}x{}", arm.width, arm.height), "64x64");
}

#[test]
fn the_caps_and_the_odd_layers_are_held() {
    assert!(star_field("23", 1, 64).is_err());
    assert!(star_field("23", 200, 64).is_err());
    assert!(star_field("23", 28, 512).is_err());
    assert!(star_decay("23", 801, 0).is_err());
    assert!(star_layers("23", 8, 65).is_err());
    assert!(star_layers("512", 8, 0).is_err());
}
