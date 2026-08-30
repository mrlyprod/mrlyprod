use mrlycore::json::parse;
use mrlyweb::bang::*;
use mrlyweb::lab::*;
use mrlyweb::lattice::*;
use mrlyweb::life::*;
use mrlyweb::race::Race;
use mrlyweb::six::*;
use mrlyweb::spectrum::*;
use mrlyweb::spin::*;
use mrlyweb::three::*;
use mrlyweb::two::*;
use mrlyweb::volume::*;

fn blinker() -> Vec<u8> {
    let mut types = vec![0u8; 25];
    for site in [7, 12, 17] {
        types[site] = 1;
    }
    types
}

#[test]
fn the_fixture_the_page_prints() {
    let grid = two_grid("7", 3, 3, 0, 2).unwrap();
    assert_eq!((grid.width, grid.height), (27, 27));
    assert_eq!(grid.types.iter().map(|&b| b as usize).sum::<usize>(), 512);
    assert_eq!(fills("23", 3, 3, 3, 2).unwrap(), "8000");
    assert_eq!(voids("23", 3, 3, 3, 2).unwrap(), "11683");
    assert_eq!(counting_sequence(4).unwrap(), ["3", "6", "22", "402"]);
    assert_eq!(baseq_sequence(3, 2).unwrap(), ["4", "26"]);
    let faces = three_faces("23", 3, 3, 2).unwrap();
    assert_eq!(faces[0] as usize / 36, 18048);
    assert_eq!(faces.len(), 2 + faces[0] as usize);
    assert_eq!(three_surface("23", 3, 3, 2).unwrap(), "18048");
    assert_eq!(three_cells("23", 3, 1, 2).unwrap().len() / 3, 20);
    let tally = parse(&three_census("23", 3, 1, 2).unwrap()).unwrap();
    assert_eq!(
        (tally["fills"].clone(), tally["euler"].clone()),
        (20.into(), (-4).into())
    );
    assert_eq!(parse(&universe(3).unwrap()).unwrap()["distinct"], 22);
    assert_eq!(
        parse(&universe(2).unwrap()).unwrap()["designs"][1]["orbit"],
        4
    );
    assert_eq!(name_of("127", 2, 3).unwrap(), "mrly_bang_d2_q3_127");
    assert_eq!(
        parse(&name_parse("mrly_bang_d3_23").unwrap()).unwrap()["code"],
        "23"
    );
    assert_eq!(
        press_members("2", 1, 2, 5).unwrap(),
        ["1", "3", "7", "15", "31"]
    );
    assert_eq!(press_count_below("7", 2, 2, "27").unwrap(), "18");
    let run = parse(&life_run(&blinker(), 5, 5, &[3], &[2, 3], false, 16).unwrap()).unwrap();
    assert_eq!(
        (run["fate"].clone(), run["loop"].clone()),
        ("loop".into(), 2.into())
    );
    assert_eq!(life_sequence("primes", 8).unwrap(), vec![2, 3, 5, 7]);
    assert_eq!(
        moire("heatmap", 9, 32, "fire", 64, false)
            .unwrap()
            .rgba
            .len(),
        4096
    );
    assert!(hex_svg("23", 3, 1, 2, "iso", 10)
        .unwrap()
        .contains("<polygon"));
    let slice = parse(&slice_census("23", 3, 1, 2).unwrap()).unwrap();
    assert_eq!(
        (slice["triangles"].clone(), slice["fills"].clone()),
        (54.into(), 42.into())
    );
    assert_eq!(
        (slice["vertices"].clone(), slice["euler"].clone()),
        (37.into(), 1.into())
    );
    assert_eq!(
        (slice["components"].clone(), slice["holes"].clone()),
        (1.into(), 1.into())
    );
    assert_eq!(slice["closed"]["vertices"], "37");
    assert_eq!(
        parse(&slice_census("232", 3, 1, 2).unwrap()).unwrap()["fills"],
        12
    );
    let deep = parse(&slice_census("23", 3, 2, 2).unwrap()).unwrap();
    assert_eq!(
        (deep["fills"].clone(), deep["holes"].clone()),
        (306.into(), 7.into())
    );
    let series = parse(&slice_series("23", 6).unwrap()).unwrap();
    let column = |key: &str| {
        series
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row[key].to_string())
            .collect::<Vec<String>>()
            .join(",")
    };
    assert_eq!(column("components"), "1,1,7,1,19,1");
    assert_eq!(column("holes"), "0,1,0,7,0,19");
    let flat = parse(&spectrum("flat", "7", 2, 4, true, 0.1).unwrap()).unwrap();
    assert_eq!(
        (flat["nodes"].clone(), flat["distinct"].clone()),
        (81.into(), 43.into())
    );
    assert_eq!(
        (flat["classes"].clone(), flat["one"].clone()),
        (9.into(), 27.into())
    );
    assert_eq!(flat["pair"], parse("[4,4]").unwrap());
    let piece = parse(&spectrum("slice", "23", 3, 1, true, 0.1).unwrap()).unwrap();
    assert_eq!(piece["nodes"], 42);
    let stair = piece["stair"].as_array().unwrap();
    assert_eq!(stair.last().unwrap()[1], 1.0);
    assert!(stair.len() >= piece["distinct"].as_u64().unwrap() as usize && stair.len() < 42);
    assert_eq!(piece["fitted"], 4);
    assert_eq!(
        format!("{:.2}", piece["exponent"].as_f64().unwrap()),
        "0.91"
    );
    assert_eq!(
        format!("{:.4}", piece["fit"][1].as_f64().unwrap() * 2.0),
        format!("{:.4}", piece["exponent"].as_f64().unwrap())
    );
    let plain = parse(&spectrum("flat", "7", 2, 2, false, 0.1).unwrap()).unwrap();
    assert_eq!(
        (plain["nodes"].clone(), plain["distinct"].clone()),
        (9.into(), 8.into())
    );
    assert_eq!(
        (plain["classes"].clone(), plain["one"].clone()),
        (1.into(), 2.into())
    );
    let cut = parse(&diagonal_profile("126", 2, 4, 2).unwrap()).unwrap();
    assert_eq!(cut["side"], 16);
    assert_eq!(cut["support"], parse("[15,30]").unwrap());
    assert_eq!(
        (cut["min"].clone(), cut["max"].clone()),
        ("81".into(), "81".into())
    );
    assert_eq!(cut["constant"], true);
    assert_eq!(diagonal_count("126", 2, 7, 2, 190).unwrap(), "2187");
    let art = diagonal_svg("126", 2, 3, 2, vec![10, 11], 4).unwrap();
    assert!(art.contains("<circle"));
    assert_eq!(art.matches("<circle").count(), 54);
    assert_eq!(
        parse(&diagonal_profile("127", 2, 4, 2).unwrap()).unwrap()["max"],
        "162"
    );
    assert_eq!(Race::new("127", 3, 4, 3, 300, 1).unwrap().side(), 81);
    assert_eq!(parse(&farey(5)).unwrap().as_array().unwrap().len(), 11);
    assert_eq!(totients(6), vec![0, 1, 1, 2, 2, 4, 2]);
    assert!((dimension("127", 3, 2, 3).unwrap() - 1.7712).abs() < 1e-4);
    let carpet: Vec<f32> = two_grid("495", 3, 3, 0, 3)
        .unwrap()
        .types
        .iter()
        .map(|&b| b as f32)
        .collect();
    let rings = profile(&carpet, 27, 1000).unwrap();
    assert_eq!(rings.len(), 1000);
    assert_eq!(rings[0], 0.0);
    assert_eq!(rings.iter().position(|&v| v > 0.0), Some(236));
    assert!((rings[600] - 0.8972).abs() < 1e-4);
    let stats = parse(&spin_stats(&rings, 27)).unwrap();
    assert!((stats["mass"].as_f64().unwrap() - 512.0).abs() < 0.1);
    assert!((stats["disc"].as_f64().unwrap() - 4.5).abs() < 0.02);
    assert_eq!(
        wheel(&rings, 64, "fire", 16, false).unwrap().rgba.len(),
        64 * 64 * 4
    );
    let hexagon = slice_grid("23", 3, 1, 2, 101).unwrap();
    assert_eq!((hexagon.width, hexagon.types[50 * 101 + 50]), (101, 0));
    let cut: Vec<f32> = hexagon.types.iter().map(|&b| b as f32).collect();
    assert_eq!(profile(&cut, 101, 10).unwrap()[0], 0.0);
    assert_eq!(
        profile(&moire_field("heatmap", 9, 32).unwrap(), 32, 16)
            .unwrap()
            .len(),
        16
    );
    let square = vec![1.0f32; 64];
    let star = radial(&square, 8, 64, 2, 45.0, "union", 1).unwrap();
    assert_eq!((star.len(), star[32 * 64 + 32]), (4096, 1.0));
    assert_eq!(turns(&harmonics(&square, 8, 64, 8).unwrap()), 4);
    assert_eq!(petals(6, 4), 12);
    assert_eq!(
        sheet(&star, 64, "heat", 8, false).unwrap().rgba.len(),
        16384
    );
    assert_eq!(moire_field("heatmap", 9, 32).unwrap().len(), 1024);
    let v = volume("23", 2, 3, "sum", 1, 9).unwrap();
    assert_eq!((v.len(), volume_count(&v, 9, 2.0).unwrap()), (729, 540));
    assert_eq!(volume_faces(&v, 9, 2.0).unwrap()[0] as usize / 36, 648);
    let f = parse(&plane_frame(&[1.0, 1.0, 1.0], 0.5).unwrap()).unwrap();
    assert!((f["width"].as_f64().unwrap() - 3.2660).abs() < 1e-3);
    let cut = plane_field(&v, 9, &[1.0, 1.0, 1.0], 0.5, 64).unwrap();
    assert!(cut[0].is_nan() && cut[32 * 64 + 32] == 1.0);
    let sheet = paint_span(&cut, 64, 0.0, 2.0, "fire", 8, false).unwrap();
    assert_eq!(
        (
            sheet.rgba.len(),
            sheet.rgba[3],
            sheet.rgba[(32 * 64 + 32) * 4 + 3]
        ),
        (16384, 0, 255)
    );
    assert_eq!(parse(&volume_stats(&v, 9).unwrap()).unwrap()["max"], 2.0);
}

#[test]
fn the_rest_of_the_exports_answer() {
    let sheet = two_pixels("7", 3, 1, 0, 2).unwrap();
    assert_eq!(sheet.rgba.chunks(4).filter(|p| p[0] == 0).count(), 8);
    assert_eq!(
        parse(&two_census("7", 3, 1, 0, 2).unwrap()).unwrap()["voids"],
        1
    );
    assert!((ratio("23", 3, 3, 3, 2).unwrap() - 8000.0 / 19683.0).abs() < 1e-12);
    let next = life_next(&blinker(), 5, 5, &[3], &[2, 3], false).unwrap();
    assert_eq!(&next[10..15], &[0, 1, 1, 1, 0]);
    assert_eq!(life_noise(8, 8, 0.0, 1).iter().sum::<u8>(), 0);
    assert_eq!(life_noise(8, 8, 1.0, 1).iter().sum::<u8>(), 64);
    assert_eq!(life_noise(64, 64, 0.5, 7), life_noise(64, 64, 0.5, 7));
    assert_eq!(life_sequences().len(), 15);
    assert_eq!(moire_names(), ["heatmap", "weave", "hive", "carpet"]);
    let mut race = Race::new("239", 3, 3, 3, 40, 9).unwrap();
    let mut twin = Race::new("239", 3, 3, 3, 40, 9).unwrap();
    assert_eq!(race.step(30), twin.step(30));
    assert_eq!(race.positions(), twin.positions());
    assert_eq!(race.steps(), 30);
    assert!(race.distance() > 0.0);
    assert!(race
        .positions()
        .iter()
        .all(|&p| race.types()[p as usize] != 0));
    assert!(race.trail().iter().map(|&v| v as usize).sum::<usize>() <= 40 * 30);
    assert_eq!(race.home(), Race::new("239", 3, 3, 3, 1, 1).unwrap().home());
}

#[test]
fn the_faults_come_back_as_messages() {
    assert!(two_grid("16", 3, 1, 0, 2).is_err());
    assert!(two_grid("7.5", 3, 1, 0, 2).is_err());
    assert!(three_cells("256", 3, 1, 2).is_err());
    assert!(universe(4).is_err());
    assert!(name_of("16", 2, 2).is_err());
    assert!(name_parse("mrly_07").is_err());
    assert!(press_members("16", 2, 2, 5).is_err());
    assert!(press_count_below("7", 2, 2, "x").is_err());
    assert!(life_next(&blinker(), 4, 5, &[3], &[2, 3], false).is_err());
    assert!(life_sequence("soup", 8).is_err());
    assert!(moire("soup", 9, 32, "fire", 64, false).is_err());
    assert!(profile(&[1.0; 60], 8, 10).is_err());
    assert!(moire_field("soup", 9, 32).is_err());
    assert!(wheel(&[1.0, 0.0], 0, "fire", 8, false).is_err());
    assert!(radial(&[1.0; 64], 8, 64, 2, 45.0, "soup", 1).is_err());
    assert!(radial(&[1.0; 60], 8, 64, 2, 45.0, "mean", 1).is_err());
    assert!(harmonics(&[1.0; 60], 8, 16, 4).is_err());
    assert!(volume("23", 2, 3, "soup", 1, 9).is_err());
    assert!(volume_faces(&[1.0; 8], 3, 1.0).is_err());
    assert!(plane_frame(&[0.0, 0.0, 0.0], 0.5).is_err());
    assert!(plane_frame(&[1.0, 1.0], 0.5).is_err());
    assert!(paint_span(&[1.0; 8], 3, 0.0, 1.0, "fire", 8, false).is_err());
    assert!(hex_svg("256", 3, 1, 2, "iso", 10).is_err());
    assert!(slice_census("23", 4, 1, 2).is_err());
    assert!(slice_series("23", 17).is_err());
    assert!(diagonal_profile("0", 2, 2, 2).is_err());
    assert!(diagonal_count("126", 2, 0, 2, 1).is_err());
    assert!(Race::new("0", 3, 2, 3, 4, 1).is_err());
    assert!(spectrum("flat", "7", 2, 7, true, 0.1).is_err());
    assert!(spectrum("wobble", "7", 2, 2, true, 0.1).is_err());
}
