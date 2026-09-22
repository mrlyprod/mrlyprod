use demos::shell::*;
use mrlyrs::core::error::parse;

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
