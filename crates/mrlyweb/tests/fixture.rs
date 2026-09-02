use mrlycore::json::parse;
use mrlyweb::automata::*;
use mrlyweb::bang::*;
use mrlyweb::blend::*;
use mrlyweb::carry::*;
use mrlyweb::census::*;
use mrlyweb::gauss::*;
use mrlyweb::graph::*;
use mrlyweb::lab::*;
use mrlyweb::lattice::*;
use mrlyweb::ledger::*;
use mrlyweb::life::*;
use mrlyweb::magic::*;
use mrlyweb::morse::*;
use mrlyweb::prime::*;
use mrlyweb::race::Race;
use mrlyweb::six::*;
use mrlyweb::spectrum::*;
use mrlyweb::spin::*;
use mrlyweb::spiral::*;
use mrlyweb::three::*;
use mrlyweb::tile::*;
use mrlyweb::two::*;
use mrlyweb::volume::*;
use mrlyweb::zeta::*;

fn column(rows: &mrlycore::Json, key: &str) -> String {
    rows.as_array()
        .unwrap()
        .iter()
        .map(|row| row[key].to_string())
        .collect::<Vec<String>>()
        .join(",")
}

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
    assert_eq!(classes_sequence(4), ["4", "12", "64", "700"]);
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
    let carpet = parse(&walsh_spectrum("23", 6).unwrap()).unwrap();
    assert_eq!(carpet["spectrum"].to_string(), "[0,-4,-4,0,-4,0,0,4]");
    assert_eq!(carpet["weights"].to_string(), "[1,3,0,0]");
    assert_eq!(crate::column(&carpet["levels"], "sixteenths"), "8,12,0,-4");
    assert_eq!(
        crate::column(&carpet["law"], "fills"),
        "6,42,72,204,210,486"
    );
    assert_eq!(crate::column(&carpet["law"], "s"), "-1,1,-1,1,-1,1");
    let skew = parse(&walsh_spectrum("11", 6).unwrap()).unwrap();
    assert_eq!(skew["spectrum"].to_string(), "[2,2,-2,-2,-6,2,-2,-2]");
    assert_eq!(skew["weights"].to_string(), "[1,1,1,0]");
    assert_eq!(crate::column(&skew["levels"], "sixteenths"), "6,6,2,2");
    assert_eq!(crate::column(&skew["law"], "fills"), "6,20,76,100,230,240");
    for design in [&carpet, &skew] {
        let code = design["code"].as_str().unwrap();
        let counted = (1..=6)
            .map(|k| {
                parse(&slice_census(code, 2 * k - 1, 1, 2).unwrap()).unwrap()["fills"].to_string()
            })
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(crate::column(&design["law"], "fills"), counted);
    }
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
    assert_eq!(cut["central"], parse("[22,23]").unwrap());
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
    assert_eq!(volume_surface(&v, 9, 2.0).unwrap(), 648);
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
    assert_eq!(random_code(3, 2, 7).unwrap(), "160");
    assert_eq!(random_codes(3, 2, 7, 3).unwrap(), ["160", "134", "72"]);
    assert_eq!(random_between(7, &[0, 0, 1], &[3, 1800, 36]), [0, 1023, 17]);
    assert_eq!(
        (
            level_cap(3, 1, 128),
            level_cap(2, 1, 512),
            level_cap(3, 3, 60000)
        ),
        (4, 9, 3)
    );
    assert_eq!(fill_cap("7", 2, 2, 2, 1100).unwrap(), 6);
    assert_eq!(grid_total(3, 2, 4).unwrap(), "6561");
    assert_eq!(odd_scales(9), [1, 3, 5, 7, 9]);
    let stack = parse(&farey_novelty(5)).unwrap();
    assert_eq!(
        (
            stack["lit"].clone(),
            stack["novel"].clone(),
            stack["match"].clone()
        ),
        (11.into(), 11.into(), true.into())
    );
    assert_eq!(stack["primes"], parse("[2,3,5]").unwrap());
    let split = parse(&slice_partition(3).unwrap()).unwrap();
    assert_eq!(
        (
            split["carpet"].clone(),
            split["net"].clone(),
            split["exact"].clone()
        ),
        (42.into(), 12.into(), true.into())
    );
    let shape = parse(&volume_shape(7, 64)).unwrap();
    assert_eq!(
        (shape["layers"].clone(), shape["voxels"].clone()),
        (4.into(), 262144.into())
    );
    assert_eq!(
        format!(
            "{:.1}",
            radial_share(&harmonics(&square, 8, 64, 8).unwrap())
        ),
        "95.3"
    );
    assert_eq!(full_turn(6), 60.0);
    assert_eq!(frame_step(900.0, 60.0), 90.0);
    assert_eq!(diagonal_digits("126", 2, 4, 2, 20).unwrap(), "101");
    assert_eq!(diagonal_total("126", 2, 3, 2, vec![10, 11]).unwrap(), "54");
    let mut sieve = Sieve::new(30).unwrap();
    let mut sweeps = 0;
    while !sieve.done() {
        sieve.step();
        sweeps += 1;
    }
    assert_eq!((sweeps, sieve.count()), (3, 10));
    let mut hundred = Sieve::new(100).unwrap();
    hundred.finish();
    assert_eq!(hundred.count(), 25);
    let stones = parse(&factor("360").unwrap()).unwrap();
    assert_eq!(stones["factors"], parse("[[2,3],[3,2],[5,1]]").unwrap());
    assert_eq!(stones["prime"], false);
    assert_eq!(
        parse(&factor("6").unwrap()).unwrap()["rectangles"],
        parse("[[1,6],[2,3]]").unwrap()
    );
    let chart = parse(&prime_chart(10_000, 400).unwrap()).unwrap();
    assert_eq!(chart["pi"].as_array().unwrap().last().unwrap(), 1229);
    let wide = parse(&prime_chart(100_000, 100).unwrap()).unwrap();
    assert_eq!(wide["pi"].as_array().unwrap().last().unwrap(), 9592);
    let square = parse(&carpet_witness(169).unwrap()).unwrap();
    assert!((square["max"].as_f64().unwrap() - 0.0517383).abs() < 1e-7);
    assert_eq!(square["at"], 13);
    let clear = parse(&carpet_witness(197).unwrap()).unwrap();
    assert_eq!(
        (clear["max"].clone(), clear["prime"].clone()),
        (0.0.into(), true.into())
    );
    assert_eq!(clear["row"].as_array().unwrap().len(), 97);
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
    let mut sieve = Sieve::new(150).unwrap();
    assert_eq!(sieve.step(), 2);
    assert_eq!((sieve.struck(), sieve.rank(), sieve.count()), (74, 1, 1));
    sieve.finish();
    let sheet = sieve.grid(15).unwrap();
    assert_eq!((sheet.width, sheet.height), (15, 10));
    assert_eq!(sheet.types.iter().map(|&b| b as u32).sum::<u32>(), 35);
    assert_eq!(&sheet.types[..7], &[0, 1, 1, 0, 1, 0, 1]);
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
    assert!(walsh_spectrum("256", 6).is_err());
    assert!(walsh_spectrum("23", 17).is_err());
    assert!(diagonal_profile("0", 2, 2, 2).is_err());
    assert!(diagonal_count("126", 2, 0, 2, 1).is_err());
    assert!(Race::new("0", 3, 2, 3, 4, 1).is_err());
    assert!(spectrum("flat", "7", 2, 7, true, 0.1).is_err());
    assert!(spectrum("wobble", "7", 2, 2, true, 0.1).is_err());
    assert!(random_code(3, 6, 1).is_err());
    assert!(fill_cap("256", 3, 3, 2, 10).is_err());
    assert!(grid_total(3, 3, 40).is_err());
    assert!(diagonal_digits("0", 2, 2, 2, 1).is_err());
    assert!(Sieve::new(401).is_err());
    assert!(Sieve::new(10).unwrap().grid(0).is_err());
    assert!(factor("x").is_err());
    assert!(factor("1000000000001").is_err());
    assert!(prime_chart(1_000_001, 10).is_err());
    assert!(carpet_witness(4).is_err());
    assert!(carpet_witness(1001).is_err());
}

#[test]
fn the_spiral_exports_answer() {
    assert_eq!(spiral_xy("square", 10).unwrap(), vec![2, -1, 2]);
    assert_eq!(spiral_xy("square", 25).unwrap(), vec![2, -2, 2]);
    assert_eq!(spiral_xy("hex", 8).unwrap(), vec![1, 1, 2]);
    assert_eq!(spiral_xy("hex", 19).unwrap(), vec![0, 2, 2]);
    assert_eq!(spiral_xy("hex", 20).unwrap(), vec![1, 2, 3]);
    let euler = parse(&spiral_polynomial("square", 201, 4, -2, 41).unwrap()).unwrap();
    assert_eq!(euler["top"], 40401);
    assert_eq!(euler["primes"], 4236);
    assert_eq!(
        (euler["count"].clone(), euler["hits"].clone()),
        (101.into(), 80.into())
    );
    assert_eq!(euler["streak"], 21);
    assert!((euler["density"].as_f64().unwrap() - 4236.0 / 40401.0).abs() < 1e-12);
    assert!((euler["share"].as_f64().unwrap() - 80.0 / 101.0).abs() < 1e-12);
    assert_eq!(euler["values"].as_array().unwrap()[20], 1601);
    assert_eq!(
        euler["cells"].as_array().unwrap()[100],
        parse("[60,100]").unwrap()
    );
    let spoke = parse(&spiral_polynomial("hex", 41, 3, 3, 1).unwrap()).unwrap();
    assert_eq!(spoke["top"], 1261);
    assert_eq!(
        spoke["cells"].as_array().unwrap()[20],
        parse("[0,20]").unwrap()
    );
    let sheet = spiral_pixels("square", 61, 4, -2, 41, "prime", false, 180).unwrap();
    assert_eq!(
        (sheet.width, sheet.height, sheet.rgba.len()),
        (180, 180, 129_600)
    );
    let at =
        |px: usize, py: usize| sheet.rgba[(py * 180 + px) * 4..(py * 180 + px) * 4 + 3].to_vec();
    assert_eq!(at(90, 90), vec![7, 9, 11]);
    assert_eq!(at(92, 90), vec![255, 209, 102]);
    assert_eq!(at(81, 92), vec![255, 138, 92]);
    let hexes = spiral_pixels("hex", 21, 3, 3, 1, "mobius", true, 200).unwrap();
    assert_eq!(hexes.rgba[..3], [7, 9, 11]);
    assert_eq!(
        hexes.rgba[(100 * 200 + 100) * 4..(100 * 200 + 100) * 4 + 3],
        [92, 200, 255]
    );
    let centre = parse(&spiral_at("square", 201, 384.0, 384.0, 768).unwrap()).unwrap();
    assert_eq!(
        (centre["n"].clone(), centre["ring"].clone()),
        (1.into(), 0.into())
    );
    let hit = parse(&spiral_at("square", 61, 81.5, 92.5, 180).unwrap()).unwrap();
    assert_eq!(
        (hit["n"].clone(), hit["prime"].clone()),
        (41.into(), true.into())
    );
    assert_eq!(hit["factors"], parse("[[41,1]]").unwrap());
    assert!((hit["span"].as_f64().unwrap() - 180.0 / 61.0).abs() < 1e-12);
    let corner = parse(&spiral_at("hex", 21, 195.0, 100.0, 200).unwrap()).unwrap();
    assert_eq!(
        (corner["x"].clone(), corner["y"].clone()),
        (10.into(), 0.into())
    );
    assert_eq!(corner["n"], 281);
    let centres = spiral_centers("square", 21, 420).unwrap();
    assert_eq!((centres.len(), centres[0], centres[1]), (882, 210.0, 210.0));
    assert_eq!((centres[2], centres[3]), (230.0, 210.0));
    assert_eq!(spiral_centers("hex", 21, 420).unwrap().len(), 662);
    assert_eq!(prime_from(90), 97);
    assert!(spiral_pixels("cube", 21, 1, 0, 2, "prime", true, 100).is_err());
    assert!(spiral_pixels("square", 20, 1, 0, 2, "prime", true, 100).is_err());
    assert!(spiral_pixels("square", 403, 1, 0, 2, "prime", true, 100).is_err());
    assert!(spiral_pixels("square", 21, 0, 0, 2, "prime", true, 100).is_err());
    assert!(spiral_pixels("square", 21, 1, 0, 2, "odd", true, 100).is_err());
    assert!(spiral_pixels("square", 21, 1, 0, 2, "prime", true, 2000).is_err());
    assert!(spiral_polynomial("square", 21, 1, 2_000_000, 2).is_err());
    assert!(spiral_at("hex", 21, 1.0, 1.0, 200).is_err());
    assert!(spiral_centers("square", 101, 400).is_err());
    assert!(spiral_xy("tri", 5).is_err());
}

#[test]
fn the_gauss_exports_answer() {
    let two = parse(&ring_census("gaussian", 2).unwrap()).unwrap();
    assert_eq!(
        (
            two["points"].clone(),
            two["primes"].clone(),
            two["split"].clone()
        ),
        (25.into(), 12.into(), 8.into())
    );
    assert_eq!(
        (
            two["ramified"].clone(),
            two["inert"].clone(),
            two["units"].clone()
        ),
        (4.into(), 0.into(), 4.into())
    );
    assert_eq!(
        (
            two["composites"].clone(),
            two["symmetry"].clone(),
            two["top"].clone()
        ),
        (8.into(), 8.into(), 8.into())
    );
    let three = parse(&ring_census("gaussian", 3).unwrap()).unwrap();
    assert_eq!(
        (three["primes"].clone(), three["inert"].clone()),
        (24.into(), 4.into())
    );
    let hex = parse(&ring_census("eisenstein", 2).unwrap()).unwrap();
    assert_eq!(
        (
            hex["points"].clone(),
            hex["primes"].clone(),
            hex["composites"].clone()
        ),
        (19.into(), 12.into(), 0.into())
    );
    assert_eq!(
        (
            hex["ramified"].clone(),
            hex["inert"].clone(),
            hex["symmetry"].clone()
        ),
        (6.into(), 6.into(), 12.into())
    );
    let r2 = ring_weights("gaussian", 25).unwrap();
    assert_eq!((r2[3], r2[5], r2[25]), (0, 8, 12));
    assert_eq!(
        ring_weights("eisenstein", 7).unwrap(),
        vec![1, 6, 0, 6, 6, 0, 0, 12]
    );
    assert_eq!(ring_peak("gaussian", 60).unwrap(), vec![25, 12]);
    assert_eq!(ring_peak("eisenstein", 60).unwrap(), vec![49, 18]);
    assert_eq!(
        ring_fates("gaussian", 7).unwrap(),
        vec![0, 0, 3, 2, 0, 1, 0, 2]
    );
    assert_eq!(
        ring_fates("eisenstein", 7).unwrap(),
        vec![0, 0, 2, 3, 0, 2, 0, 1]
    );
    let sheet = ring_pixels("gaussian", 2, "class", true, 100).unwrap();
    let pixel =
        |px: usize, py: usize| sheet.rgba[(py * 100 + px) * 4..(py * 100 + px) * 4 + 3].to_vec();
    assert_eq!((sheet.width, sheet.height), (100, 100));
    assert_eq!(pixel(70, 30), vec![255, 122, 182]);
    assert_eq!(pixel(90, 30), vec![92, 200, 255]);
    assert_eq!(pixel(90, 50), vec![31, 38, 46]);
    assert_eq!(pixel(50, 50), vec![7, 9, 11]);
    assert_eq!(pixel(60, 30), vec![7, 9, 11]);
    let hexes = ring_pixels("eisenstein", 2, "class", false, 100).unwrap();
    let cell =
        |px: usize, py: usize| hexes.rgba[(py * 100 + px) * 4..(py * 100 + px) * 4 + 3].to_vec();
    assert_eq!(cell(80, 67), vec![255, 122, 182]);
    assert_eq!(cell(90, 50), vec![255, 138, 92]);
    assert_eq!(cell(70, 50), vec![110, 231, 168]);
    assert_eq!(cell(50, 50), vec![7, 9, 11]);
    let plain = ring_pixels("gaussian", 3, "plain", false, 70).unwrap();
    assert_eq!(
        plain.rgba[(25 * 70 + 45) * 4..(25 * 70 + 45) * 4 + 3],
        [255, 209, 102]
    );
    let glow = ring_pixels("gaussian", 3, "norm", false, 70).unwrap();
    assert_ne!(
        glow.rgba[(25 * 70 + 45) * 4..(25 * 70 + 45) * 4 + 3],
        [255, 209, 102]
    );
    let hit = parse(&ring_at("gaussian", 40, 403.0, 374.0, 768).unwrap()).unwrap();
    assert_eq!(
        (
            hit["a"].clone(),
            hit["b"].clone(),
            hit["norm"].clone(),
            hit["class"].clone()
        ),
        (2.into(), 1.into(), 5.into(), "split".into())
    );
    assert_eq!(hit["prime"], true);
    assert_eq!(hit["associates"].as_array().unwrap().len(), 4);
    assert_eq!(hit["associates"][1][0], -1.0);
    assert_eq!(hit["associates"][1][1], 2.0);
    assert_eq!(hit["conjugate"][1], -1.0);
    assert!((hit["px"].as_f64().unwrap() - 402.963).abs() < 1e-3);
    assert!((hit["py"].as_f64().unwrap() - 374.519).abs() < 1e-3);
    let flake = parse(&ring_at("eisenstein", 5, 140.0, 127.0, 220).unwrap()).unwrap();
    assert_eq!(
        (
            flake["a"].clone(),
            flake["b"].clone(),
            flake["norm"].clone(),
            flake["class"].clone()
        ),
        (1.into(), (-1).into(), 3.into(), "ramified".into())
    );
    assert_eq!(flake["associates"].as_array().unwrap().len(), 6);
    assert_eq!(flake["conjugate"][0], 2.0);
    assert_eq!(flake["conjugate"][1], 1.0);
    assert_eq!(flake["factors"], parse("[[3,1]]").unwrap());
    assert!(ring_pixels("quaternion", 2, "class", true, 100).is_err());
    assert!(ring_pixels("gaussian", 0, "class", true, 100).is_err());
    assert!(ring_pixels("gaussian", 201, "class", true, 100).is_err());
    assert!(ring_pixels("gaussian", 2, "bad", true, 100).is_err());
    assert!(ring_pixels("gaussian", 2, "class", true, 2000).is_err());
    assert!(ring_weights("gaussian", 20_000).is_err());
    assert!(ring_fates("eisenstein", 20_000).is_err());
    assert!(ring_at("eisenstein", 5, 1.0, 1.0, 220).is_err());
    assert!(ring_census("gaussian", 300).is_err());
}

#[test]
fn the_zeta_exports_answer() {
    let zeros = zeta_zeros(5).unwrap();
    let known = [14.134_725, 21.022_040, 25.010_858, 30.424_876, 32.935_062];
    for (got, want) in zeros.iter().zip(known) {
        assert!((got - want).abs() < 1e-6, "{got} {want}");
    }
    assert_eq!(zeta_count(100.0).unwrap(), 29);
    assert_eq!(zeta_count(200.0).unwrap(), 79);
    let root = zeta_at(14.134_725).unwrap();
    assert!(root[0].hypot(root[1]) < 1e-5);
    assert!(root[2].abs() < 1e-5);
    let half = zeta_at(0.0).unwrap();
    assert!((half[0] + 1.460_354_5).abs() < 1e-7);
    assert_eq!(half[3], 0.0);
    let walk = zeta_line(0.0, 50.0, 600).unwrap();
    assert_eq!(walk.len(), 2404);
    assert_eq!(walk[0], 0.0);
    assert_eq!(walk[2400], 50.0);
    assert!((walk[1] + 1.460_354_5).abs() < 1e-7);
    let seam = zeta_seam(250.0, 500).unwrap();
    assert_eq!(seam[0], 20.0);
    assert!(seam[1] < 5e-5);
    let stair = psi_stair(100).unwrap();
    assert!((stair[9] - 7.832_0).abs() < 1e-4);
    assert!((stair[99] - 94.045_3).abs() < 1e-4);
    let smooth = psi_formula(10.0, &[], 3).unwrap();
    assert_eq!(smooth.len(), 6);
    assert!((smooth[5] - 8.167_1).abs() < 1e-4);
    let hundred = zeta_zeros(100).unwrap();
    let folded = psi_formula(100.0, &hundred, 2).unwrap();
    assert!((folded[3] - stair[99]).abs() < 1.0);
    let gap = psi_gap(100, &hundred).unwrap();
    assert!((gap - (stair[99] - folded[3])).abs() < 1e-12);
    assert!(zeta_line(0.0, 300.0, 10).is_err());
    assert!(zeta_line(0.0, 10.0, 0).is_err());
    assert!(zeta_at(-1.0).is_err());
    assert!(zeta_zeros(101).is_err());
    assert!(zeta_count(251.0).is_err());
    assert!(zeta_seam(100.0, 0).is_err());
    assert!(psi_stair(1).is_err());
    assert!(psi_stair(1001).is_err());
    assert!(psi_formula(10.0, &[], 1).is_err());
    assert!(psi_formula(10.0, &vec![14.0; 101], 10).is_err());
    assert!(psi_gap(1, &[]).is_err());
}

#[test]
fn the_graph_exports_answer() {
    let carpet = parse(&graph_census("flat", "7", 3, 1, 2, "core").unwrap()).unwrap();
    assert_eq!(
        (carpet["nodes"].clone(), carpet["branches"].clone()),
        (8.into(), 8.into())
    );
    assert_eq!(carpet["components"], 1);
    let knots = graph_nodes("flat", "7", 3, 1, 2, "core").unwrap();
    assert_eq!((knots[0], knots[1], knots.len()), (2.0, 8.0, 18));
    assert_eq!(
        graph_branches("flat", "7", 3, 1, 2, "core").unwrap().len(),
        16
    );
    assert_eq!(
        graph_roles("flat", "7", 3, 1, 2, "core").unwrap(),
        vec![2; 8]
    );
    let sponge = graph_nodes("cube", "23", 3, 1, 2, "core").unwrap();
    assert_eq!((sponge[0], sponge[1], sponge.len()), (3.0, 20.0, 62));
    assert_eq!(
        graph_branches("cube", "255", 1, 1, 2, "edge")
            .unwrap()
            .len(),
        24
    );
    let tally = parse(&graph_census("cube", "23", 3, 1, 2, "core").unwrap()).unwrap();
    assert_eq!(tally["nodes"], 20);
    assert_eq!(tally["branches"], 24);
    assert_eq!(tally["junctions"], 8);
    assert_eq!(tally["euler"], -4);
    let slice = parse(&graph_census("hex", "23", 3, 1, 2, "core").unwrap()).unwrap();
    assert_eq!(
        (slice["nodes"].clone(), slice["branches"].clone()),
        (42.into(), 48.into())
    );
    assert!((slice["length"].as_f64().unwrap() - 48.0 / 3f64.sqrt()).abs() < 1e-9);
    let rim = parse(&graph_census("hex", "23", 3, 1, 2, "edge").unwrap()).unwrap();
    assert_eq!(
        (rim["nodes"].clone(), rim["length"].clone()),
        (36.into(), 78.0.into())
    );
    assert_eq!(graph_size("flat", "495", 3, 2, 3, "core").unwrap(), "64");
    assert_eq!(graph_size("cube", "23", 3, 2, 2, "tunnel").unwrap(), "329");
    assert_eq!(graph_cap("flat", "7", 3, 2, "core", 20000).unwrap(), 4);
    assert_eq!(graph_cap("cube", "23", 3, 2, "core", 2000).unwrap(), 2);
    assert_eq!(graph_cap("hex", "23", 3, 2, "edge", 2000).unwrap(), 2);
    let ring = graph_nodes("flat", "15", 2, 1, 2, "core").unwrap();
    let loop_ = graph_branches("flat", "15", 2, 1, 2, "core").unwrap();
    let mut relax = Layout::new(&ring[2..], &loop_, 2, 1).unwrap();
    let rest = relax.step(500);
    assert!(rest < 1e-3, "energy {rest}");
    let p = relax.positions();
    let gaps: Vec<f32> = loop_
        .chunks(2)
        .map(|b| {
            let (a, c) = (b[0] as usize, b[1] as usize);
            ((p[2 * a] - p[2 * c]).powi(2) + (p[2 * a + 1] - p[2 * c + 1]).powi(2)).sqrt()
        })
        .collect();
    let spread = gaps.iter().cloned().fold(0.0, f32::max)
        - gaps.iter().cloned().fold(f32::INFINITY, f32::min);
    assert!(spread < 1e-3, "gaps {gaps:?}");
    assert_eq!(relax.ticks(), 500);
    assert!(relax.moved() < relax.temperature());
    assert!(graph_nodes("wobble", "7", 3, 1, 2, "core").is_err());
    assert!(graph_roles("flat", "7", 3, 1, 2, "dual").is_err());
    assert!(graph_census("hex", "23", 3, 1, 2, "tunnel").is_err());
    assert!(graph_nodes("flat", "7", 3, 9, 2, "core").is_err());
    assert!(graph_cap("flat", "16", 3, 2, "core", 100).is_err());
    assert!(Layout::new(&[0.0, 0.0, 1.0, 1.0], &[0, 2], 2, 1).is_err());
}

#[test]
fn the_ledger_exports_answer() {
    assert_eq!(ledger_measures().len(), 12);
    assert_eq!(ledger_measures()[0], "fills");
    assert_eq!(
        ledger_designs(2, 2).unwrap(),
        ["0", "1", "3", "6", "7", "15"]
    );
    assert_eq!(ledger_designs(2, 3).unwrap().len(), 26);
    assert!(ledger_designs(3, 3).is_err());
    let budget = "500000";
    assert_eq!(
        ledger_terms("7", 2, 2, "fills", "level", 3, budget).unwrap(),
        ["8", "64", "512"]
    );
    assert_eq!(
        ledger_terms("23", 3, 2, "surface", "level", 3, budget).unwrap(),
        ["72", "1056", "18048"]
    );
    assert_eq!(
        ledger_terms("7", 2, 2, "fills", "side", 4, budget).unwrap(),
        ["8", "21", "40", "65"]
    );
    assert_eq!(
        ledger_terms("3", 2, 2, "fills", "side", 3, budget).unwrap(),
        ["6", "15", "28"]
    );
    assert_eq!(
        ledger_terms("23", 3, 2, "euler", "level", 8, "1000").unwrap(),
        ["-4", "-80"]
    );
    assert!(ledger_terms("7", 2, 2, "faces", "level", 1, budget).is_err());
    let found = parse(&ledger_identify("6, 42, 306, 2250")).unwrap();
    assert_eq!(
        (found[0]["id"].clone(), found[0]["shift"].clone()),
        ("A299916".into(), 1.into())
    );
    let octagonal = parse(&ledger_identify("8, 21, 40, 65")).unwrap();
    assert_eq!(
        (octagonal[0]["id"].clone(), octagonal[0]["shift"].clone()),
        ("A000567".into(), 2.into())
    );
    assert!(parse(&ledger_identify("x"))
        .unwrap()
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        ledger_closed("7", 2, 2, "fills", "side").unwrap(),
        "3k^2 - 2k"
    );
    assert_eq!(
        ledger_closed("23", 3, 2, "surface", "level").unwrap(),
        "a(L) = 28 a(L-1) - 160 a(L-2)"
    );
    assert_eq!(ledger_closed("7", 2, 2, "euler", "level").unwrap(), "");
    let records = parse(&ledger_records()).unwrap();
    let records = records.as_array().unwrap();
    assert_eq!(records.len(), 60);
    let octagonal = records.iter().find(|r| r["id"] == "A000567").unwrap();
    assert_eq!(
        (octagonal["key"].clone(), octagonal["shift"].clone()),
        ("mrly_bang_d2_7.fills.side".into(), 0.into())
    );
    assert_eq!(ledger_build("closed", 4).unwrap(), 7692);
    assert_eq!(ledger_build("closed", 4).unwrap(), 7692);
    assert!(ledger_build("deep", 4).is_err());
    let hits = parse(&ledger_search("8, 21, 40, 65", "", 2, 2, 0, 25)).unwrap();
    assert_eq!(hits["total"], 1);
    let row = &hits["rows"][0];
    assert_eq!(row["name"], "mrly_bang_d2_7.fills.side");
    assert_eq!(
        (
            row["oeis"].clone(),
            row["shift"].clone(),
            row["tag"].clone()
        ),
        ("A000567".into(), 0.into(), "Proved".into())
    );
    assert_eq!(row["closed"], "3k^2 - 2k");
    let surfaces = parse(&ledger_search("", "surface", 3, 2, 0, 5)).unwrap();
    assert_eq!(surfaces["total"], 44);
    assert_eq!(surfaces["rows"].as_array().unwrap().len(), 5);
    assert_eq!(
        parse(&ledger_search("mrly_bang_d3_23.", "", 0, 0, 0, 100)).unwrap()["total"],
        6
    );
    assert_eq!(
        parse(&ledger_search("A381517", "", 0, 0, 0, 100)).unwrap()["rows"][0]["terms"][1],
        "80"
    );
    let grown = parse(&ledger_grow("convolved", 4, 100).unwrap()).unwrap();
    assert_eq!(
        (
            grown["rows"].clone(),
            grown["done"].clone(),
            grown["total"].clone()
        ),
        (7792.into(), 100.into(), 5044.into())
    );
    assert!(ledger_grow("deep", 4, 100).is_err());
    let void = parse(&ledger_row("9", 2, 2, "voids", "side", 3, "500000").unwrap()).unwrap();
    assert_eq!(void["name"], "mrly_bang_d2_9.voids.side");
    assert_eq!(void["terms"], parse(r#"["4", "12", "24"]"#).unwrap());
    assert_eq!(void["closed"], "2k^2 - 2k");
    assert_eq!(void["number"], 3);
    assert!(ledger_row("7", 2, 2, "faces", "level", 3, "500000").is_err());
    let gasket = ledger_profile("126", 3, 2, 2, 4).unwrap();
    assert_eq!(gasket.len(), 46);
    assert!(gasket[15..=30].iter().all(|count| count == "81"));
    assert_eq!(
        ledger_profile("1", 1, 2, 3, 2).unwrap().join(""),
        "101000101"
    );
}

#[test]
fn the_tour_exports_answer() {
    let farey = parse(&farey_novelty(7)).unwrap();
    assert_eq!(
        (farey["lit"].clone(), farey["novel"].clone()),
        (19.into(), 19.into())
    );
    let cut = parse(&diagonal_profile("126", 2, 5, 2).unwrap()).unwrap();
    assert_eq!(
        (cut["max"].clone(), cut["constant"].clone()),
        ("243".into(), true.into())
    );
    assert_eq!(
        parse(&slice_census("23", 9, 1, 2).unwrap()).unwrap()["vertices"],
        271
    );
    assert_eq!(
        parse(&slice_census("23", 1, 1, 2).unwrap()).unwrap()["fills"],
        6
    );
    assert_eq!(
        parse(&two_census("9", 5, 1, 0, 2).unwrap()).unwrap()["fills"],
        13
    );
    assert_eq!(baseq_sequence(5, 2).unwrap(), ["8", "172112"]);
    let first = |terms: &str| {
        let found = parse(&ledger_identify(terms)).unwrap();
        (found[0]["id"].clone(), found[0]["shift"].clone())
    };
    assert_eq!(first("3, 9, 27, 81"), ("A000244".into(), 1.into()));
    assert_eq!(first("2, 3, 5, 7, 11, 13"), ("A005728".into(), 1.into()));
    assert_eq!(first("7, 37, 91, 169"), ("A154105".into(), 0.into()));
    assert_eq!(first("4, 12, 64, 700"), ("A129824".into(), 1.into()));
    assert_eq!(first("20, 81, 208"), ("A103532".into(), 1.into()));
}

#[test]
fn the_census_exports_answer() {
    let window = parse(&census_window()).unwrap();
    assert_eq!(window["registry"], 18066);
    assert_eq!(window["cap"], 48);
    assert_eq!(window["cells"], "100000");
    assert_eq!(window["ceiling"], "1000");
    assert_eq!(window["head"], 8);
    assert_eq!(window["depths"], parse("[8, 16, 32, 48]").unwrap());
    let tiers = window["tiers"].as_array().unwrap();
    let keyed: Vec<(String, u64)> = tiers
        .iter()
        .map(|tier| {
            (
                tier["tier"].as_str().unwrap().to_string(),
                tier["keys"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        keyed,
        [
            ("closed".to_string(), 7692),
            ("convolved".to_string(), 5044),
            ("side".to_string(), 2665),
            ("level".to_string(), 2665),
        ]
    );
    let walk = parse(&census_walk(7692)).unwrap();
    assert_eq!(
        (
            walk["depth"].clone(),
            walk["done"].clone(),
            walk["total"].clone()
        ),
        (8.into(), 7692.into(), 18066.into())
    );
    assert_eq!(
        (
            walk["never"].clone(),
            walk["once"].clone(),
            walk["multiple"].clone()
        ),
        (396.into(), 102.into(), 502.into())
    );
    let report = parse(&census_report()).unwrap();
    assert_eq!(report["rows"], 7692);
    assert_eq!(report["written"], 604);
    assert_eq!(report["first_miss"], 83);
    assert_eq!(report["incidences"], 30865);
    assert_eq!(report["low"], 452);
    assert_eq!(report["ceiling_stopped"], 5048);
    assert_eq!(report["cap_stopped"], 2644);
    assert_eq!(report["blank"], 54);
    assert_eq!(report["bands"][1]["missed"], 2);
    assert_eq!(report["tiers"][0]["written"], 604);
    let counts = census_counts();
    assert_eq!(counts.len(), 1000);
    assert_eq!((counts[0], counts[15]), (102, 633));
    let writers = parse(&census_writers(16, 0, 2)).unwrap();
    assert_eq!(
        (writers["inside"].clone(), writers["rows"].clone()),
        (true.into(), 633.into())
    );
    assert_eq!(writers["tiers"][0]["rows"], 633);
    let first = &writers["shown"][0];
    assert_eq!(first["name"], "mrly_bang_d1_1.fills.level");
    assert_eq!(first["closed"], "2^L");
    assert_eq!(
        (first["index"].clone(), first["term"].clone()),
        (3.into(), 4.into())
    );
    assert_eq!(first["head"][3], "16");
    assert_eq!(writers["shown"][1]["closed"], "a(L) = 3 a(L-1) - 2 a(L-2)");
    let paged = parse(&census_writers(16, 0, 633)).unwrap();
    let sided = paged["shown"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["axis"] == "side")
        .unwrap();
    assert_eq!(
        (
            sided["name"].clone(),
            sided["term"].clone(),
            sided["side"].clone()
        ),
        ("mrly_bang_d1_1.surface.side".into(), 8.into(), 15.into())
    );
    let outside = parse(&census_writers(1001, 0, 1)).unwrap();
    assert_eq!(
        (outside["inside"].clone(), outside["rows"].clone()),
        (false.into(), 0.into())
    );
    let champions = parse(&census_champions(2)).unwrap();
    assert_eq!(
        (champions[0]["value"].clone(), champions[0]["rows"].clone()),
        (16.into(), 633.into())
    );
    let misses = parse(&census_misses(3)).unwrap();
    assert_eq!(misses, parse("[83, 86, 107]").unwrap());
}

fn word(codes: &[&str], numbers: &[u32], bases: &[u32]) -> (Vec<String>, Vec<u32>, Vec<u32>) {
    (
        codes.iter().map(|c| c.to_string()).collect(),
        numbers.to_vec(),
        bases.to_vec(),
    )
}

fn census(codes: &[&str], numbers: &[u32], dimension: usize, bases: &[u32]) -> mrlycore::Json {
    let (codes, numbers, bases) = word(codes, numbers, bases);
    parse(&magic_census(codes, numbers, dimension, bases).unwrap()).unwrap()
}

#[test]
fn the_word_fixture_the_page_prints() {
    let doctest = census(&["7", "14", "9"], &[3, 7, 5], 2, &[2, 2, 2]);
    assert_eq!(doctest["side"], "105");
    assert_eq!(doctest["cells"], "11025");
    assert_eq!(doctest["fill"], "3432");
    assert_eq!(
        format!("{:.9}", doctest["dimension"].as_f64().unwrap()),
        "1.749241044"
    );
    assert_eq!(doctest["components"], "2496");
    assert_eq!(doctest["counted"], "drawn");
    let (codes, numbers, bases) = word(&["7", "14", "9"], &[3, 7, 5], &[2, 2, 2]);
    assert_eq!(
        word_count(codes.clone(), numbers.clone(), 2, bases.clone()).unwrap(),
        doctest["fill"].as_str().unwrap()
    );
    assert_eq!(
        word_profile(codes.clone(), numbers.clone(), 2, bases.clone())
            .unwrap()
            .len(),
        209
    );
    assert_eq!(
        magic_name(codes, numbers).unwrap(),
        "mrly_word_d2_c7n3_c14n7_c9n5"
    );
    let back = parse(&magic_parse("mrly_word_d2_c7n3_c14n7_c9n5").unwrap()).unwrap();
    assert_eq!(back["codes"][1], "14");
    assert_eq!(back["numbers"][2], 5);

    let one = census(&["7", "9"], &[3, 5], 2, &[2, 2]);
    let twice = census(&["7", "9", "7", "9"], &[3, 5, 3, 5], 2, &[2, 2, 2, 2]);
    let square = |text: &str| text.parse::<u128>().unwrap().pow(2).to_string();
    assert_eq!(twice["side"], square(one["side"].as_str().unwrap()));
    assert_eq!(twice["fill"], square(one["fill"].as_str().unwrap()));
    assert_eq!(twice["dimension"], one["dimension"]);
    assert_eq!(
        (twice["periodic"].clone(), one["periodic"].clone()),
        (true.into(), false.into())
    );

    let ahead = census(&["3", "6"], &[2, 2], 2, &[2, 2]);
    let behind = census(&["6", "3"], &[2, 2], 2, &[2, 2]);
    assert_eq!(ahead["fill"], behind["fill"]);
    assert_eq!(ahead["side"], behind["side"]);
    assert_eq!(
        (ahead["components"].clone(), behind["components"].clone()),
        ("4".into(), "2".into())
    );
    assert_eq!(ahead["counted"], "closed");

    let ladder = census(
        &["7", "7", "7", "7", "7"],
        &[3, 5, 7, 9, 11],
        2,
        &[2, 2, 2, 2, 2],
    );
    for letter in ladder["letters"].as_array().unwrap() {
        let side = letter["number"].as_u64().unwrap();
        let law = side * side - ((side - 1) / 2).pow(2);
        assert_eq!(letter["fill"], law.to_string(), "side {side}");
    }
    let stair = parse(&magic_staircase(5).unwrap()).unwrap();
    let read = |row: usize| format!("{:.9}", stair["rows"][row]["dimension"].as_f64().unwrap());
    assert_eq!(read(0), "1.892789261");
    assert_eq!(read(1), "1.892315261");
    assert_eq!(stair["rows"][2]["length"], 6);
    assert!(stair["rows"][1]["dimension"].as_f64() < stair["rows"][0]["dimension"].as_f64());
    assert_eq!(
        format!("{:.9}", stair["constant"].as_f64().unwrap()),
        read(0)
    );

    assert_eq!(magic_cap(vec![3, 7, 5, 3], 2, 243).unwrap(), 3);
    assert_eq!(magic_cap(vec![3, 3, 3, 3, 3], 3, 128).unwrap(), 4);
    let menger = census(&["23", "23", "23"], &[3, 3, 3], 3, &[2, 2, 2]);
    assert_eq!(menger["fill"], "8000");
    assert_eq!(menger["constant"], true);
    let (codes, numbers, bases) = word(&["23", "23", "23"], &[3, 3, 3], &[2, 2, 2]);
    assert_eq!(
        magic_cells(codes.clone(), numbers.clone(), bases.clone())
            .unwrap()
            .len()
            / 3,
        8000
    );
    assert_eq!(magic_surface(codes, numbers, bases).unwrap(), "18048");

    let rates = parse(
        &magic_rates(
            vec!["3".into(), "7".into()],
            vec![2, 2],
            vec![2, 2],
            "thue-morse",
            64,
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        format!("{:.15}", rates["limit"].as_f64().unwrap()),
        "1.292481250360578"
    );
    assert_eq!(rates["phi"], 0.0);
    assert_eq!(rates["length"], 64);
    let last = rates["rows"][63][0].as_f64().unwrap();
    assert_eq!(
        rates["rows"][63][1].as_f64().unwrap(),
        rates["limit"].as_f64().unwrap()
    );
    assert!(last < rates["limit"].as_f64().unwrap());
    assert!(last > rates["rows"][31][0].as_f64().unwrap());
    assert!(rates["control"][63].as_f64().unwrap() > 0.0);

    let ahead = magic_grid(vec!["9".into(), "273".into()], vec![2, 3], vec![2, 3]).unwrap();
    let behind = magic_grid(vec!["273".into(), "9".into()], vec![3, 2], vec![3, 2]).unwrap();
    assert_eq!((ahead.width, behind.width), (6, 6));
    assert_eq!(ahead.types, behind.types);
    assert_eq!(ahead.types.iter().map(|&b| b as usize).sum::<usize>(), 6);
}

fn lift(kind: &str, level: usize) -> Vec<u8> {
    morse_lift(kind, level).unwrap().types
}

#[test]
fn the_morse_fixture_the_page_prints() {
    let read = parse(&morse_word(64).unwrap()).unwrap();
    assert_eq!(read["agree"], true);
    assert_eq!(read["ones"], 32);
    assert_eq!(read["longest"], 2);
    assert_eq!(read["cube_free"], true);
    assert_eq!(read["singles"], 22);
    assert_eq!(read["doubles"], 21);
    assert_eq!(read["doubling_agree"], true);
    assert_eq!(read["digits"].as_array().unwrap().len(), 64);
    assert_eq!(read["boundary"].as_array().unwrap().len(), 63);
    assert_eq!(read["boundary"][0], 1);
    assert_eq!(morse_stage(3).unwrap(), vec![0, 1, 1, 0, 1, 0, 0, 1]);
    assert_eq!(morse_stage(6).unwrap().len(), 64);

    let gallery = parse(&morse_gallery(6).unwrap()).unwrap();
    let row = |at: usize| gallery[at].clone();
    assert_eq!(row(0)["formula"], "t(i) xor t(j)");
    assert_eq!(row(0)["folds"], true);
    assert_eq!(row(0)["tile"], parse("[0,1,1,0]").unwrap());
    assert_eq!(row(0)["design"], "9");
    assert_eq!(row(1)["formula"], "t(i and j)");
    assert_eq!(row(1)["folds"], true);
    assert_eq!(row(1)["tile"], parse("[0,0,0,1]").unwrap());
    assert_eq!(row(1)["design"], "7");
    assert_eq!(row(2)["formula"], "t(i xor j)");
    assert_eq!(row(2)["folds"], true);
    assert_eq!(row(2)["twin"], "parity");
    assert_eq!(row(3)["formula"], "t(i + j)");
    assert_eq!(row(3)["folds"], false);
    assert_eq!(row(3)["faults"], 1376);
    assert_eq!(row(3)["first"], parse("[1,3]").unwrap());
    assert_eq!(row(3)["design"], mrlycore::Json::Null);

    for level in 1..10 {
        let rows = parse(&morse_gallery(level).unwrap()).unwrap();
        assert_eq!(rows[2]["twin"], "parity", "level {level}");
        assert_eq!(rows[3]["folds"], level == 1, "level {level}");
    }

    let side = morse_lift("parity", 6).unwrap();
    assert_eq!((side.width, side.height), (64, 64));
    assert_eq!(lift("xor", 6), lift("parity", 6));
    assert_eq!(morse_signs("9", 2, 2, 6).unwrap().types, lift("parity", 6));
    assert_eq!(morse_signs("7", 2, 2, 6).unwrap().types, lift("and", 6));
    assert_eq!(
        lift("parity", 6).iter().map(|&b| b as usize).sum::<usize>(),
        2048
    );

    let sign = parse(&morse_filter("9", 2, 2, 3, "sign").unwrap()).unwrap();
    assert_eq!(sign["morse_tile"], true);
    assert_eq!(sign["form"], "the base tile repeated");
    assert_eq!(sign["closed_exact"], true);
    assert_eq!(sign["morse_exact"], false);
    assert_eq!(sign["side"], 16);
    assert_eq!(sign["morse_faults"], 128);
    assert_eq!(sign["lit"], 128);
    for code in 0..16u32 {
        for level in 1..5 {
            let read =
                parse(&morse_filter(&code.to_string(), 2, 2, level, "sign").unwrap()).unwrap();
            let side = read["side"].as_u64().unwrap();
            assert_eq!(read["closed_exact"], true, "code {code} level {level}");
            assert_eq!(
                read["morse_faults"],
                side * side / 2,
                "code {code} level {level}"
            );
        }
    }

    let flat = parse(&morse_filter("7", 2, 2, 3, "design").unwrap()).unwrap();
    assert_eq!(
        flat["form"],
        "the level below, punched by the tile's complement"
    );
    assert_eq!(flat["closed_exact"], true);
    assert_eq!(flat["morse_exact"], false);
    assert_eq!(
        (flat["side"].clone(), flat["lit"].clone()),
        (16.into(), 27.into())
    );
    assert_eq!(flat["morse_faults"], 127);
    let wide = parse(&morse_filter("495", 3, 3, 2, "design").unwrap()).unwrap();
    assert_eq!(wide["closed_exact"], true);
    assert_eq!(wide["morse_faults"], mrlycore::Json::Null);
    assert_eq!(wide["side"], 27);

    assert!(morse_word(0).is_err());
    assert!(morse_lift("cube", 4).is_err());
    assert!(morse_lift("parity", 10).is_err());
    assert!(morse_filter("7", 2, 2, 3, "product").is_err());
}

#[test]
fn the_tile_fixture_the_page_prints() {
    let read = |dimension: usize,
                code: &str,
                number: usize,
                level: usize,
                base: usize,
                projection: &str,
                reps: Vec<u32>,
                crop: bool| {
        parse(&tile_census(code, number, level, base, dimension, projection, reps, crop).unwrap())
            .unwrap()
    };
    let count = |cell: &mrlycore::Json, key: &str| cell[key].as_str().unwrap().to_string();

    let wide = read(2, "495", 3, 2, 3, "", vec![5, 5], false);
    assert_eq!(wide["tile"], parse("[9,9]").unwrap());
    assert_eq!(wide["sheet"], parse("[45,45]").unwrap());
    assert_eq!(count(&wide, "fills"), "1600");
    assert_eq!(count(&wide, "voids"), "425");
    assert_eq!(count(&wide, "exposed"), "1280");
    assert_eq!(count(&wide, "tile_exposed"), "80");
    assert_eq!(count(&wide, "buried"), "720");
    assert_eq!(
        (wide["vertices"].clone(), wide["euler"].clone()),
        (2016.into(), (-224).into())
    );

    let tall = read(2, "495", 3, 2, 3, "", vec![3, 9], false);
    assert_eq!(tall["sheet"], parse("[27,81]").unwrap());
    assert_eq!(count(&tall, "fills"), "1728");
    assert_eq!(count(&tall, "exposed"), "1404");
    assert_eq!(count(&tall, "buried"), "756");
    assert_eq!(
        (tall["vertices"].clone(), tall["euler"].clone()),
        (2188.into(), (-242).into())
    );

    let block = read(3, "23", 3, 1, 2, "", vec![5, 5, 5], false);
    assert_eq!(block["sheet"], parse("[15,15,15]").unwrap());
    assert_eq!(count(&block, "fills"), "2500");
    assert_eq!(count(&block, "voids"), "875");
    assert_eq!(count(&block, "exposed"), "4200");
    assert_eq!(count(&block, "tile_exposed"), "72");
    assert_eq!(count(&block, "buried"), "4800");
    assert_eq!(
        (block["faces"].clone(), block["euler"].clone()),
        (9600.into(), (-324).into())
    );

    let slab = read(3, "23", 3, 1, 2, "", vec![3, 9, 3], false);
    assert_eq!(slab["sheet"], parse("[9,27,9]").unwrap());
    assert_eq!(count(&slab, "fills"), "1620");
    assert_eq!(count(&slab, "exposed"), "2952");
    assert_eq!(count(&slab, "buried"), "2880");
    assert_eq!(
        (slab["faces"].clone(), slab["euler"].clone()),
        (6336.into(), (-224).into())
    );

    let mesh = read(6, "23", 3, 1, 2, "cut", vec![5, 5], false);
    assert_eq!(mesh["tile"], parse("[11,6]").unwrap());
    assert_eq!(mesh["sheet"], parse("[47,33]").unwrap());
    assert_eq!(mesh["triangles"], 1350);
    assert_eq!(count(&mesh, "fills"), "1050");
    assert_eq!(count(&mesh, "voids"), "300");
    assert_eq!(count(&mesh, "exposed"), "414");
    assert_eq!(count(&mesh, "tile_exposed"), "30");
    assert_eq!(count(&mesh, "buried"), "336");
    assert_eq!(mesh["euler"], 1);

    let strip = read(6, "23", 3, 1, 2, "cut", vec![3, 9], false);
    assert_eq!(strip["sheet"], parse("[29,57]").unwrap());
    assert_eq!(strip["triangles"], 1458);
    assert_eq!(count(&strip, "fills"), "1134");
    assert_eq!(count(&strip, "exposed"), "462");
    assert_eq!(strip["euler"], 1);

    let trimmed = read(6, "23", 3, 1, 2, "cut", vec![5, 5], true);
    assert_eq!(trimmed["sheet"], parse("[43,27]").unwrap());
    assert_eq!(trimmed["triangles"], 1161);
    assert_eq!(count(&trimmed, "fills"), "891");
    assert_eq!(count(&trimmed, "exposed"), "357");
    assert_eq!(trimmed["euler"], 1);

    let narrow = read(6, "23", 3, 1, 2, "cut", vec![3, 9], true);
    assert_eq!(narrow["sheet"], parse("[25,51]").unwrap());
    assert_eq!(narrow["triangles"], 1275);
    assert_eq!(count(&narrow, "fills"), "969");
    assert_eq!(narrow["euler"], 1);

    let grid = tile_grid("495", 3, 2, 3, 5, 5).unwrap();
    assert_eq!((grid.width, grid.height), (45, 45));
    assert_eq!(grid.types.iter().map(|&b| b as usize).sum::<usize>(), 1600);
    assert_eq!(tile_cells("23", 3, 1, 2, 5, 5, 5).unwrap().len() / 3, 2500);
    let art = tile_svg("23", 3, 1, 2, "cut", 5, 5, true, 6).unwrap();
    assert_eq!(art.matches("<polygon").count(), 1161);

    assert!(tile_census("495", 3, 2, 3, 4, "", vec![5, 5], false).is_err());
    assert!(tile_census("495", 3, 2, 3, 2, "", vec![5, 5, 5], false).is_err());
    assert!(tile_svg("23", 3, 1, 2, "cut", 1, 5, true, 6).is_err());
    assert!(tile_grid("495", 3, 6, 3, 5, 5).is_err());
}

#[test]
fn the_word_reaches_every_dimension_the_tower_draws() {
    let word = |list: [&str; 2]| list.iter().map(|c| c.to_string()).collect::<Vec<String>>();
    let sponge = word(["23", "23"]);
    let sides = vec![3u32, 3];
    let bases = vec![2u32, 2];

    assert_eq!(
        magic_perimeter(word(["7", "9"]), vec![3, 5], bases.clone()).unwrap(),
        "368"
    );
    assert_eq!(
        magic_perimeter(
            vec!["7".into(), "14".into(), "9".into()],
            vec![3, 7, 5],
            vec![2, 2, 2]
        )
        .unwrap(),
        "11856"
    );

    let cut =
        parse(&magic_hex_census(sponge.clone(), sides.clone(), bases.clone(), "cut").unwrap())
            .unwrap();
    assert_eq!(cut["grid"], parse("[35,18]").unwrap());
    assert_eq!(cut["triangles"], 486);
    assert_eq!(cut["fills"], 306);
    assert_eq!(cut["voids"], 180);
    assert_eq!(cut["exposed"], 162);
    assert_eq!(cut["euler"], 1);
    let solo = parse(&slice_census("23", 3, 2, 2).unwrap()).unwrap();
    assert_eq!(cut["fills"], solo["fills"]);
    assert_eq!(cut["triangles"], solo["triangles"]);

    let iso =
        parse(&magic_hex_census(sponge.clone(), sides.clone(), bases.clone(), "iso").unwrap())
            .unwrap();
    assert_eq!(iso["grid"], parse("[18,35]").unwrap());
    assert_eq!(iso["fills"], 486);
    assert_eq!(iso["voids"], 0);
    assert_eq!(iso["exposed"], 88);

    let art = magic_hex(sponge, sides, bases, "cut", 2).unwrap();
    assert_eq!(art.matches("<polygon").count(), 486);
}

#[test]
fn the_blend_exports_answer() {
    let budget = "500000";
    let surface = ledger_terms("23", 3, 2, "surface", "level", 8, budget).unwrap();
    assert_eq!(surface[7], "51267108864");
    let rule = parse(&blend_recurrence(surface.clone()).unwrap()).unwrap();
    assert_eq!(rule["order"], 2);
    assert_eq!(rule["coefficients"], parse("[[28,1],[-160,1]]").unwrap());
    assert_eq!(rule["recurrence"], "a(n) = 28 a(n-1) - 160 a(n-2)");
    let poly = blend_characteristic(&rule["coefficients"].to_string()).unwrap();
    assert_eq!(
        parse(&poly).unwrap(),
        parse("[[1,1],[-28,1],[160,1]]").unwrap()
    );
    let root = blend_growth(&rule["coefficients"].to_string()).unwrap();
    assert!((root - 20.0).abs() < 1e-12, "root {root}");
    assert_eq!(
        blend_recurrence(vec![
            "2".into(),
            "3".into(),
            "5".into(),
            "7".into(),
            "11".into(),
            "13".into(),
            "17".into(),
            "19".into()
        ])
        .unwrap(),
        "null"
    );

    let series =
        parse(&blend_series("23", 3, 2, "surface", "level", 8, budget, 4).unwrap()).unwrap();
    assert_eq!(series["name"], "mrly_bang_d3_23.surface.level");
    assert_eq!(series["oeis"], "A332705");
    assert_eq!(series["closed"], "a(L) = 28 a(L-1) - 160 a(L-2)");
    assert_eq!(series["recurrence"], "a(n) = 28 a(n-1) - 160 a(n-2)");
    assert_eq!(series["polynomial"], "x^2 - 28 x + 160");
    assert_eq!(series["growth_from"], "the recurrence root");
    assert_eq!(series["order"], 2);
    assert_eq!(series["ratios"][0], "14.6667");
    assert_eq!(series["differences"][1][0], "984");
    assert_eq!(series["differences"].as_array().unwrap().len(), 4);
    let growth = series["growth"].as_f64().unwrap();
    let exponent = series["exponent"].as_f64().unwrap();
    assert!((growth - 20.0).abs() < 1e-12, "growth {growth}");
    assert!(
        (exponent - 20f64.log10()).abs() < 1e-12,
        "exponent {exponent}"
    );
    let logs = series["log10"].as_array().unwrap();
    assert!((logs[0].as_f64().unwrap() - 72f64.log10()).abs() < 1e-12);

    let side = parse(&blend_series("7", 2, 2, "fills", "side", 12, budget, 5).unwrap()).unwrap();
    assert_eq!(side["closed"], "3k^2 - 2k");
    assert_eq!(side["order"], 3);
    assert_eq!(side["coefficients"], parse("[[3,1],[-3,1],[1,1]]").unwrap());
    assert_eq!(side["polynomial"], "x^3 - 3 x^2 + 3 x - 1");
    assert_eq!(side["differences"][3][0], "0");

    let family = parse(&blend_family(2, 2, "fills", "level", 3, budget).unwrap()).unwrap();
    let family = family.as_array().unwrap();
    assert_eq!(family.len(), 6);
    assert_eq!(family[4]["code"], "7");
    assert_eq!(family[4]["name"], "mrly_bang_d2_7.fills.level");
    assert_eq!(family[4]["terms"], parse(r#"["8", "64", "512"]"#).unwrap());
    assert_eq!(
        parse(&blend_family(2, 3, "fills", "level", 2, budget).unwrap())
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        26
    );

    let fills = ledger_terms("7", 2, 2, "fills", "level", 8, budget).unwrap();
    let faces = ledger_terms("7", 2, 2, "surface", "level", 8, budget).unwrap();
    let mix = parse(&blend_mix(faces.clone(), fills.clone(), "hadamard", 0, 3).unwrap()).unwrap();
    assert_eq!(mix["terms"][0], "128");
    assert_eq!(mix["terms"][3], "14483456");
    assert_eq!(mix["order"], 2);
    assert_eq!(mix["coefficients"], parse("[[88,1],[-1536,1]]").unwrap());
    assert_eq!(mix["polynomial"], "x^2 - 88 x + 1536");
    assert_eq!(mix["recurrence"], "a(n) = 88 a(n-1) - 1536 a(n-2)");
    let mixed = mix["growth"].as_f64().unwrap();
    assert!((mixed - 64.0).abs() < 1e-12, "growth {mixed}");
    let sums = parse(&blend_mix(fills.clone(), vec![], "sigma", 0, 2).unwrap()).unwrap();
    assert_eq!(sums["terms"][2], "584");
    assert_eq!(sums["order"], 2);
    let cut = parse(&blend_mix(fills.clone(), vec![], "decimate", 2, 2).unwrap()).unwrap();
    assert_eq!(
        cut["terms"],
        parse(r#"["8", "512", "32768", "2097152"]"#).unwrap()
    );

    assert_eq!(blend_ops().len(), 9);
    assert_eq!(moire_correlation(3, 5), 0.0);
    let paired = moire_correlation(3, 9);
    assert!(
        (paired - 0.219_264_504_826_757_3).abs() < 1e-12,
        "r {paired}"
    );
    assert_eq!(moire_correlation(1, 9), 0.0);

    assert!(blend_recurrence(vec!["x".into()]).is_err());
    assert!(blend_characteristic("[[1,0]]").is_err());
    assert!(blend_growth("nonsense").is_err());
    assert!(blend_series("7", 2, 2, "faces", "level", 3, budget, 3).is_err());
    assert!(blend_family(3, 3, "fills", "level", 3, budget).is_err());
    assert!(blend_mix(fills, faces, "twist", 0, 3).is_err());
}

#[test]
fn the_carry_automaton_reads_the_published_block() {
    assert_eq!(carry_cap(3).unwrap(), 15);
    assert_eq!(carry_cap(5).unwrap(), 11);
    let anchor = parse(&carry_block(3, 3, 6).unwrap()).unwrap();
    assert_eq!(anchor["digits"].to_string(), "[1,3,3,6,3,3,1]");
    assert_eq!(anchor["block"].to_string(), "[[6,6],[1,3]]");
    assert_eq!(anchor["characteristic"].to_string(), r#"["1","-9","12"]"#);
    assert_eq!(anchor["polynomial"], "x^2 - 9 x + 12");
    assert_eq!(
        (
            anchor["trace"].clone(),
            anchor["determinant"].clone(),
            anchor["fill"].clone()
        ),
        ("9".into(), "12".into(), "20".into())
    );
    let root = anchor["read"]["root"].as_f64().unwrap();
    assert!((root - (9.0 + 33f64.sqrt()) / 2.0).abs() < 1e-9);
    assert!((anchor["read"]["log_root"].as_f64().unwrap() - 1.818_410).abs() < 1e-6);
    assert!((anchor["read"]["log_fill"].as_f64().unwrap() - 1.726_833).abs() < 1e-6);
    assert_eq!(anchor["read"]["sign"], 1);
    assert_eq!(
        anchor["terms"].to_string(),
        r#"["1","6","42","306","2250","16578","122202"]"#
    );
    assert_eq!(
        anchor["ratios"].to_string(),
        r#"["6","7","7.2857","7.3529","7.368","7.3713"]"#
    );
    let traces: Vec<String> = (2..=7)
        .map(|dimension| {
            parse(&carry_block(3, dimension, 1).unwrap()).unwrap()["trace"].to_string()
        })
        .collect();
    assert_eq!(traces.join(","), r#""2","9","11","60","47","336""#);
    let ladder = |base: usize, dimension: usize, levels: usize| {
        parse(&carry_block(base, dimension, levels).unwrap()).unwrap()["terms"].to_string()
    };
    assert_eq!(
        ladder(3, 4, 6),
        r#"["1","6","132","1848","29040","441408","6772128"]"#
    );
    assert_eq!(ladder(3, 5, 4), r#"["1","30","1000","35700","1321600"]"#);
    assert_eq!(ladder(3, 6, 4), r#"["1","20","4030","242300","24642700"]"#);
    assert_eq!(ladder(5, 3, 4), r#"["1","18","414","9702","227646"]"#);
    let deep = parse(&carry_block(3, 15, 32).unwrap()).unwrap();
    assert_eq!(
        (deep["levels"].clone(), deep["capped"].clone()),
        (7.into(), true.into())
    );
    assert!(carry_block(3, 16, 4).is_err());
    assert!(carry_block(5, 12, 4).is_err());
    assert!(carry_block(4, 3, 4).is_err());
    assert!(carry_block(3, 3, 0).is_err());
    assert!(carry_signs(1).is_err());
    assert!(carry_ratios(3, 3).is_err());
}

#[test]
fn the_carry_ladder_is_the_sponge_diagonal_count() {
    let anchor = parse(&carry_block(3, 3, 5).unwrap()).unwrap();
    let counted: Vec<String> = (1..=5)
        .map(|level| {
            let height = 3 * (3usize.pow(level as u32) - 1) / 2;
            diagonal_count("23", 3, level, 2, height).unwrap()
        })
        .collect();
    assert_eq!(counted.join(","), "6,42,306,2250,16578");
    assert_eq!(
        anchor["terms"].to_string(),
        r#"["1","6","42","306","2250","16578"]"#
    );
    assert_eq!(
        parse(&slice_census("23", 3, 1, 2).unwrap()).unwrap()["fills"],
        42
    );
    assert_eq!(
        parse(&slice_census("23", 3, 2, 2).unwrap()).unwrap()["fills"],
        306
    );
    assert_eq!(
        column(&parse(&slice_series("23", 2).unwrap()).unwrap(), "fills"),
        "6,42"
    );
    for dimension in 2..=6 {
        let order = dimension / 2 + dimension % 2;
        let row = parse(&carry_block(3, dimension, 2 * order + 1).unwrap()).unwrap();
        assert_eq!(row["order"], order, "dimension {dimension}");
        assert_eq!(row["found"], order, "dimension {dimension}");
        assert_eq!(row["fits"], true, "dimension {dimension}");
    }
}

#[test]
fn the_carry_sign_law_alternates_at_both_bases() {
    let table = parse(&carry_signs(10).unwrap()).unwrap();
    let rows = table.as_array().unwrap();
    let read = |key: &str| {
        rows.iter()
            .map(|row| row[key]["sign"].to_string())
            .collect::<Vec<String>>()
            .join(",")
    };
    assert_eq!(column(&table, "law"), "-1,1,-1,1,-1,1,-1,1,-1");
    assert_eq!(read("three"), "-1,1,-1,1,-1,1,-1,1,-1");
    assert_eq!(read("five"), "-1,1,-1,1,-1,1,-1,1,-1");
    assert_eq!(column(&table, "order"), "1,2,2,3,3,4,4,5,5");
    assert_eq!(
        column(&table, "open"),
        "false,false,false,false,false,true,false,false,false"
    );
    let wide = parse(&carry_signs(13).unwrap()).unwrap();
    let past = wide.as_array().unwrap().last().unwrap();
    assert_eq!(past["three"]["sign"], 1);
    assert_eq!(past["five"], mrlycore::Json::Null);
    let ladder = parse(&carry_ratios(3, 50).unwrap()).unwrap();
    let last = ladder.as_array().unwrap().last().unwrap();
    assert_eq!(last["dimension"], 50);
    assert!((last["ratio"].as_f64().unwrap() - 13.0 / 12.0).abs() < 1e-9);
    assert!((last["free"].as_f64().unwrap() - 13.0 / 12.0).abs() < 1e-12);
}

#[test]
fn the_automata_exports_answer() {
    assert_eq!(eca_next(&[0, 0, 1, 0, 0], 110, false), vec![0, 1, 1, 0, 0]);
    assert_eq!(eca_next(&[1, 0, 0, 0, 0], 170, true), vec![0, 0, 0, 0, 1]);
    let run = eca_history(&[0, 0, 1, 0, 0], 110, 3, false);
    assert_eq!((run.width, run.height), (5, 4));
    assert_eq!(&run.types[5..10], &[0, 1, 1, 0, 0]);
    let cone = eca_seed(110, 31);
    assert_eq!((cone.width, cone.height), (63, 32));
    assert_eq!(cone.types.iter().map(|&b| b as u32).sum::<u32>(), 326);
    assert_eq!(
        eca_seed(90, 8).types.iter().map(|&b| b as u32).sum::<u32>(),
        29
    );
    let card = parse(&eca_card(110)).unwrap();
    assert_eq!(card["name"], "mrly_bang_d3_110");
    assert_eq!(
        (card["popcount"].clone(), card["degree"].clone()),
        (5.into(), 3.into())
    );
    assert_eq!(card["lambda"], 0.625);
    assert_eq!(card["genus"], "comp");
    assert_eq!(
        (card["b3_rep"].clone(), card["wolfram_rep"].clone()),
        (61.into(), 110.into())
    );
    assert_eq!(card["npn_rep"], 25);
    assert_eq!(card["b3_orbit"].as_array().unwrap().len(), 24);
    assert_eq!(card["wolfram_class"].to_string(), "[110,124,137,193]");
    assert!(!card["surjective"].as_bool().unwrap());
    assert!(!card["reversible"].as_bool().unwrap());
    assert!(card["outer_totalistic"].is_null());
    assert!(card["gasket"].is_null());
    let gasket = parse(&eca_card(60)).unwrap();
    assert_eq!(gasket["gasket"], "mrly_bang_d2_13");
    assert_eq!(gasket["b3_rep"], 60);
    let conway = parse(&eca_card(90)).unwrap();
    assert_eq!(conway["outer_totalistic"]["birth"].to_string(), "[1]");
    assert_eq!(conway["outer_totalistic"]["survive"].to_string(), "[1]");
    assert!(conway["surjective"].as_bool().unwrap());
    assert_eq!(eca_soup(64, 0.0, 1).iter().sum::<u8>(), 0);
    assert_eq!(eca_soup(64, 1.0, 1).iter().sum::<u8>(), 64);
    assert_eq!(eca_soup(64, 0.5, 7), eca_soup(64, 0.5, 7));
    let moore = life_mask(2, "7", 3, 1).unwrap();
    assert_eq!((moore.width, moore.height), (3, 3));
    assert_eq!(moore.types.iter().map(|&b| b as u32).sum::<u32>(), 8);
    let deep = life_mask(2, "7", 3, 2).unwrap();
    assert_eq!((deep.width, deep.height), (9, 9));
    assert_eq!(deep.types.iter().map(|&b| b as u32).sum::<u32>(), 64);
    let line = life_mask(1, "1", 3, 1).unwrap();
    assert_eq!((line.width, line.height), (3, 1));
    assert_eq!(line.types, vec![1, 0, 1]);
    let wide = life_mask(1, "1", 5, 1).unwrap();
    assert_eq!(wide.types, vec![1, 0, 0, 0, 1]);
    assert_eq!(life_mask_index(&deep.types, 9, 9).unwrap(), 1);
    assert_eq!(life_mask_index(&line.types, 3, 1).unwrap(), 1);
    assert_eq!(life_mask_index(&wide.types, 5, 1).unwrap(), 2);
    let diagonal = life_mask(2, "9", 3, 1).unwrap();
    assert_eq!(life_mask_index(&diagonal.types, 3, 3).unwrap(), 2);
    let row = [0, 1, 1, 0, 1, 0, 0];
    let stepped = life_next_masked(&row, 7, 1, &[1], &[0, 1], &line.types, 3, 1, false).unwrap();
    assert_eq!(stepped, eca_next(&row, 94, false));
    let paced = parse(
        &life_run_masked(
            &blinker(),
            5,
            5,
            &[3],
            &[2, 3],
            &moore.types,
            3,
            3,
            false,
            16,
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        (paced["fate"].clone(), paced["loop"].clone()),
        ("loop".into(), 2.into())
    );
    assert!(life_mask(3, "7", 3, 1).is_err());
    assert!(life_mask(2, "7", 4, 1).is_err());
    assert!(life_mask_index(&[1, 0, 1], 2, 2).is_err());
    assert!(life_next_masked(&row, 7, 1, &[1], &[], &line.types, 4, 1, false).is_err());
    assert!(life_next_masked(&row, 7, 1, &[1], &[], &wide.types, 5, 1, false).is_ok());
}
