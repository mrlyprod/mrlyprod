use mrlycore::json::parse;
use mrlycore::tensor::Tensor;
use mrlymath::bang::factory;
use mrlymath::shape::{crossing_shell, crossing_tree, radial_census};
use mrlyweb::crop::crop_circle;
use mrlyweb::shell::*;

fn keep(code: u128) -> Vec<bool> {
    factory::create(code, 3, 2, 2, 1)
        .unwrap()
        .bytes()
        .iter()
        .map(|&byte| byte != 0)
        .collect()
}

fn depth_of(radius: u64) -> u32 {
    let mut depth = 0;
    while 3u64.pow(depth) <= radius {
        depth += 1;
    }
    depth
}

#[test]
fn every_level_holds_two_floor_plus_one_boxes() {
    for radius in 1..=400u64 {
        for level in 0..8u32 {
            let want = 2 * (radius / 3u64.pow(level)) + 1;
            assert_eq!(
                crossing_shell(radius, 3, level).len() as u64,
                want,
                "r={radius} j={level}"
            );
        }
    }
    for radius in [728u64, 729, 1000, 2186, 2187, 6560] {
        for level in 0..9u32 {
            let want = 2 * (radius / 3u64.pow(level)) + 1;
            assert_eq!(crossing_shell(radius, 3, level).len() as u64, want);
        }
    }
}

#[test]
fn the_whole_grid_shell_is_the_trees_leaf_row() {
    let ones = Tensor::full(vec![243, 243], 1);
    let table = radial_census(&ones, &[0, 0], 242);
    for radius in 1..=242u64 {
        assert_eq!(table[radius as usize].cut, 2 * radius + 1, "r={radius}");
    }
}

#[test]
fn every_crossed_box_has_a_crossed_parent() {
    let carpet = keep(7);
    for radius in 1..=242u64 {
        let tree = crossing_tree(radius, 3, &carpet);
        assert_eq!(tree.orphans, 0, "r={radius}");
        assert_eq!(tree.levels.len() as u32, depth_of(radius) + 1);
        assert_eq!(tree.levels.last().unwrap().len(), 1);
        for level in 0..tree.levels.len() - 1 {
            for cell in &tree.levels[level] {
                let parent = tree.levels[level + 1][cell.parent];
                assert_eq!((parent.x, parent.y), (cell.x / 3, cell.y / 3));
                assert!(!cell.live || parent.live);
                assert_eq!(cell.live, parent.live && carpet[cell.seat]);
            }
        }
    }
}

#[test]
fn the_live_leaves_are_the_crossings_the_page_draws() {
    for code in [7u128, 11, 15] {
        let seats = keep(code);
        for depth in 1..=5u32 {
            let side = 3u64.pow(depth);
            let grid = factory::create(code, 3, 2, 2, depth as usize).unwrap();
            let table = radial_census(&grid, &[0, 0], side - 1);
            for radius in side / 3..side {
                let tree = crossing_tree(radius, 3, &seats);
                let live = tree.levels[0].iter().filter(|cell| cell.live).count() as u64;
                assert_eq!(
                    live, table[radius as usize].cut,
                    "code={code} r={radius} depth={depth}"
                );
            }
        }
    }
}

#[test]
fn the_shell_exports_answer() {
    let read = parse(&shell_read("7", 3, 2, 100).unwrap()).unwrap();
    assert_eq!(read["depth"], 5);
    assert_eq!(read["side"], 243);
    assert_eq!(read["leaves"], 201);
    assert_eq!(read["orphans"], 0);
    assert_eq!(read["exact"], true);
    let column = |key: &str| {
        read["levels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row[key].to_string())
            .collect::<Vec<String>>()
            .join(",")
    };
    assert_eq!(column("boxes"), "201,67,23,7,3,1");
    assert_eq!(column("want"), column("boxes"));
    assert_eq!(column("live"), "134,48,18,7,3,1");
    assert_eq!(column("three"), "false,true,false,false,false,true");

    let nodes = shell_nodes("7", 3, 2, 100).unwrap();
    assert_eq!(nodes.len(), 5 * 302);
    assert_eq!(&nodes[0..5], &[0, 0, 100, 0, 1]);
    assert_eq!(nodes[5 * 301 + 4], 1);
    let leaves = nodes.chunks(5).filter(|node| node[0] == 0).count();
    assert_eq!(leaves, 201);

    let small = parse(&shell_read("7", 3, 2, 7).unwrap()).unwrap();
    assert_eq!(small["depth"], 2);
    assert_eq!(small["leaves"], 15);
    assert_eq!(small["live"], 10);

    let art = shell_pixels("7", 3, 2, 100, 3).unwrap();
    assert_eq!((art.width, art.height), (486, 486));
    assert_eq!(art.rgba.len(), 4 * 486 * 486);

    assert!(shell_read("7", 3, 2, 0).is_err());
    assert!(shell_read("7", 3, 2, 243).is_err());
    assert!(shell_pixels("7", 3, 2, 100, 6).is_err());
}

#[test]
fn the_page_reads_the_same_count_as_the_crop_sweep() {
    let flat = crop_circle("7", 3, 5, 2, 2, "corner").unwrap();
    let width = flat.len() / 3;
    let cut = &flat[2 * width..3 * width];
    for radius in [1u32, 7, 40, 100, 242] {
        let read = parse(&shell_read("7", 3, 2, radius).unwrap()).unwrap();
        assert_eq!(
            read["live"].as_u64().unwrap(),
            u64::from(cut[radius as usize]),
            "r={radius}"
        );
    }
}
