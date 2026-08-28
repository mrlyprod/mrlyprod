use mrlycore::json::parse;
use mrlyweb::bang::*;
use mrlyweb::lab::*;
use mrlyweb::lattice::*;
use mrlyweb::life::*;
use mrlyweb::race::Race;
use mrlyweb::six::hex_svg;
use mrlyweb::three::*;
use mrlyweb::two::*;

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
    assert_eq!(Race::new("127", 3, 4, 3, 300, 1).unwrap().side(), 81);
    assert_eq!(parse(&farey(5)).unwrap().as_array().unwrap().len(), 11);
    assert_eq!(totients(6), vec![0, 1, 1, 2, 2, 4, 2]);
    assert!((dimension("127", 3, 2, 3).unwrap() - 1.7712).abs() < 1e-4);
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
    assert!(hex_svg("256", 3, 1, 2, "iso", 10).is_err());
    assert!(Race::new("0", 3, 2, 3, 4, 1).is_err());
}
