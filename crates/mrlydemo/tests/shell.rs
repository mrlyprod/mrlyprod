use mrlycore::json::parse;
use mrlycore::tensor::Tensor;
use mrlymath::bang::factory;
use mrlymath::shape::{crossing_shell, crossing_tree, radial_census};
use mrlydemo::crop::crop_circle;
use mrlydemo::shell::*;

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

    let nodes = shell_nodes("7", 3, 2, 100, None, None).unwrap();
    assert_eq!(nodes.len(), 5 * 302);
    assert_eq!(&nodes[0..5], &[0, 0, 100, 0, 1]);
    assert_eq!(nodes[5 * 301 + 4], 1);
    let leaves = nodes.chunks(5).filter(|node| node[0] == 0).count();
    assert_eq!(leaves, 201);

    let small = parse(&shell_read("7", 3, 2, 7).unwrap()).unwrap();
    assert_eq!(small["depth"], 2);
    assert_eq!(small["leaves"], 15);
    assert_eq!(small["live"], 10);

    let art = shell_pixels("7", 3, 2, 100, 3, None, None).unwrap();
    assert_eq!((art.width, art.height), (486, 486));
    assert_eq!(art.rgba.len(), 4 * 486 * 486);

    assert!(shell_read("7", 3, 2, 0).is_err());
    assert!(shell_read("7", 3, 2, 243).is_err());
    assert!(shell_pixels("7", 3, 2, 100, 6, None, None).is_err());
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

fn widths(nodes: &[u32]) -> Vec<usize> {
    let mut out = Vec::new();
    for node in nodes.chunks(5) {
        let level = node[0] as usize;
        while out.len() <= level {
            out.push(0);
        }
        out[level] += 1;
    }
    out
}

#[test]
fn one_branch_of_the_tree_is_the_zoom_the_page_draws() {
    let whole = shell_nodes("7", 3, 2, 242, None, None).unwrap();
    assert_eq!(
        whole,
        shell_nodes("7", 3, 2, 242, Some(5), Some(0)).unwrap()
    );
    let boxes = widths(&whole);
    println!("shell_nodes r=242 boxes {boxes:?}");
    assert_eq!(boxes, [485, 161, 53, 17, 5, 1]);
    let read = parse(&shell_read("7", 3, 2, 242).unwrap()).unwrap();
    let (mut leaves, mut live, mut shapes) = (0usize, 0usize, Vec::new());
    for seat in 0..boxes[4] {
        let branch = shell_nodes("7", 3, 2, 242, Some(4), Some(seat as u32)).unwrap();
        let wide = widths(&branch);
        assert_eq!(wide.len(), 5, "seat={seat}");
        assert_eq!(*wide.last().unwrap(), 1, "seat={seat}");
        for node in branch.chunks(5) {
            let level = node[0] as usize;
            if level + 1 < wide.len() {
                assert!(
                    (node[3] as usize) < wide[level + 1],
                    "seat={seat} j={level}"
                );
            } else {
                assert_eq!(node[3], u32::MAX, "seat={seat}");
            }
        }
        leaves += wide[0];
        live += branch
            .chunks(5)
            .filter(|node| node[0] == 0 && node[4] == 1)
            .count();
        shapes.push(format!("{wide:?}"));
    }
    println!("shell_nodes r=242 branches {}", shapes.join(" "));
    assert_eq!(
        shapes.join(" "),
        "[95, 31, 10, 3, 1] [130, 44, 15, 5, 1] [35, 11, 3, 1, 1] [130, 44, 15, 5, 1] [95, 31, 10, 3, 1]"
    );
    println!("shell_nodes r=242 branch leaves {leaves} live {live}");
    assert_eq!(leaves, 485);
    assert_eq!(live as u64, read["live"].as_u64().unwrap());
    let plain = shell_pixels("7", 3, 2, 242, 4, None, None).unwrap();
    let ringed = shell_pixels("7", 3, 2, 242, 4, Some(4), Some(2)).unwrap();
    assert_ne!(plain.rgba, ringed.rgba);
    assert!(shell_nodes("7", 3, 2, 242, Some(6), None).is_err());
    assert!(shell_nodes("7", 3, 2, 242, Some(4), Some(5)).is_err());
    assert!(shell_pixels("7", 3, 2, 242, 4, Some(4), Some(9)).is_err());
}
