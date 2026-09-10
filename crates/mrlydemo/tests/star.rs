use mrlycore::json::parse;
use mrlydemo::star::*;
use mrlymath::six::star::{arm_law, width_law, Star};

fn read(layers: usize, half: usize) -> mrlycore::Json {
    parse(&star_decay("23", layers, half).unwrap()).unwrap()
}

fn shown(value: &mrlycore::Json, places: usize) -> String {
    format!("{:.*}", places, value.as_f64().unwrap())
}

#[test]
fn the_arm_ink_is_the_closed_form_cell_for_cell() {
    let table = parse(&star_layers("23", 8, 0).unwrap()).unwrap();
    assert_eq!(table["exact"], 8);
    let fractions: Vec<String> = (0..8)
        .map(|at| {
            let row = &table["rows"][at];
            format!("{}/{}", row["numer"], row["denom"])
        })
        .collect();
    assert_eq!(fractions.join(" "), "1/1 1/3 2/5 4/7 5/9 5/11 6/13 8/15");
    let chi: Vec<String> = (0..8)
        .map(|at| table["rows"][at]["chi"].to_string())
        .collect();
    assert_eq!(chi.join(","), "1,-1,-1,1,1,-1,-1,1");
    let star = Star::new(23).unwrap();
    for at in 0..8 {
        let number = 2 * at + 1;
        assert_eq!(star.arm(number, 0).unwrap(), arm_law(number).unwrap());
    }
}

#[test]
fn the_background_is_the_hexagons_own_ink() {
    let table = parse(&star_layers("23", 4, 0).unwrap()).unwrap();
    let hexes: Vec<String> = (0..4)
        .map(|at| shown(&table["rows"][at]["hex"], 6))
        .collect();
    assert_eq!(hexes.join(" "), "1.000000 0.777778 0.480000 0.693878");
    assert_eq!(shown(&table["rows"][1]["excess"], 6), "-0.444444");
    let wide = parse(&star_layers("23", 4, 2).unwrap()).unwrap();
    assert_eq!(wide["exact"], 0);
    let banded: Vec<String> = (0..4)
        .map(|at| {
            let row = &wide["rows"][at];
            format!("{}/{}", row["numer"], row["denom"])
        })
        .collect();
    assert_eq!(banded.join(" "), "1/1 5/9 4/15 16/21");
}

#[test]
fn the_cell_frame_decay_settles_on_minus_a_quarter() {
    let deep = read(100, 0);
    assert_eq!(shown(&deep["scaled"], 6), "-1.445065");
    assert_eq!(shown(&deep["logged"], 10), "-0.2937725615");
    assert_eq!(shown(&deep["residual"], 8), "-0.11975831");
    assert_eq!(shown(&deep["slope"], 6), "-0.250092");
    assert_eq!(deep["rows"][0]["slope"], mrlycore::Json::Null);
    assert_eq!(shown(&deep["constant"], 10), "-0.2937605857");
    assert_eq!(deep["branch"], "0 mod 4");
    assert_eq!(shown(&deep["predicted"], 8), "-0.11979167");
    assert_eq!(shown(&deep["target"], 8), "-0.25000000");
    assert_eq!(deep["arm"], true);
    assert_eq!(deep["deepest"], 199);

    let shallow = read(28, 0);
    assert_eq!(shown(&shallow["scaled"], 6), "-1.126964");
    assert_eq!(shown(&shallow["logged"], 10), "-0.2939128437");
    assert_eq!(shown(&shallow["residual"], 8), "-0.11937029");
    assert_eq!(shallow["rows"].as_array().unwrap().len(), 2);
    assert_eq!(shallow["rows"][1]["layers"], 28);
    assert_eq!(shallow["rows"][0]["layers"], 14);
    let ladder = read(128, 0);
    let rungs: Vec<String> = ladder["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["layers"].to_string())
        .collect();
    assert_eq!(rungs.join(","), "4,8,16,32,64,128");
    assert_eq!(shown(&ladder["slope"], 6), "-0.249968");
    let walk = ladder["walk"].as_array().unwrap();
    assert_eq!(walk.len(), 63);
    assert_eq!(walk[0][0], 4);
    assert_eq!(walk[62][0], 128);
    assert_eq!(shown(&walk[62][1], 6), "-1.506775");
}

#[test]
fn the_square_term_reads_the_layer_count_mod_four() {
    let two = read(102, 0);
    assert_eq!(two["branch"], "2 mod 4");
    assert_eq!(shown(&two["predicted"], 8), "0.13020833");
    assert_eq!(shown(&two["residual"], 8), "0.13017437");
    assert_eq!(two["slope"], mrlycore::Json::Null);
    let odd = read(101, 0);
    assert_eq!(odd["branch"], "odd");
    assert_eq!(odd["slope"], mrlycore::Json::Null);
    assert_eq!(odd["predicted"], mrlycore::Json::Null);
    assert_eq!(shown(&odd["linear"], 6), "0.249724");
    assert_eq!(shown(&odd["branchConstant"], 10), "-0.1687605857");
}

#[test]
fn a_wider_band_walks_the_coefficient_off() {
    let wide = read(200, 2);
    assert_eq!(shown(&wide["slope"], 6), "-0.166654");
    assert_eq!(shown(&wide["target"], 8), "-0.16666667");
    assert_eq!(wide["arm"], false);
    let odds: Vec<String> = [1usize, 3, 5, 7]
        .iter()
        .map(|half| format!("{:.6}", width_law(*half)))
        .collect();
    assert_eq!(odds.join(" "), "-0.250000 -0.166667 -0.100000 -0.107143");
    let family: Vec<String> = [0usize, 2, 4, 6, 8, 10, 12]
        .iter()
        .map(|half| format!("{:.6}", width_law(*half)))
        .collect();
    assert_eq!(
        family.join(" "),
        "-0.250000 -0.166667 -0.100000 -0.107143 -0.138889 -0.136364 -0.115385"
    );
}

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
    let tally = |grid: &mrlydemo::Grid, kind: u8| grid.types.iter().filter(|t| **t == kind).count();
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
