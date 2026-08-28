use mrlymath::bang::baseq::{axis_maps, distinct_designs, group_order};
use mrlymath::bang::counting;
use mrlymath::bang::factory::{corners_to_code, levels_code, residue_corners};
use mrlymath::bang::universe::permutations;
use mrlymath::bang::Code;
use std::collections::BTreeSet;

pub const WALK_LIMIT: usize = 20;

pub fn cell_index(cell: &[u8], base: usize) -> usize {
    cell.iter().fold(0, |acc, &r| acc * base + r as usize)
}

fn choices(count: usize, dimension: usize) -> Vec<Vec<usize>> {
    let mut out = vec![Vec::new()];
    for _ in 0..dimension {
        let mut next = Vec::with_capacity(out.len() * count);
        for prefix in &out {
            for index in 0..count {
                let mut choice = prefix.clone();
                choice.push(index);
                next.push(choice);
            }
        }
        out = next;
    }
    out
}

pub fn group(base: usize, dimension: usize) -> Vec<Vec<usize>> {
    let cells = residue_corners(dimension, base);
    let maps = axis_maps(base);
    let mut out = Vec::new();
    for order in permutations(dimension) {
        for choice in choices(maps.len(), dimension) {
            let element: Vec<usize> = cells
                .iter()
                .map(|cell| {
                    let image: Vec<u8> = (0..dimension)
                        .map(|axis| maps[choice[axis]][cell[order[axis]] as usize] as u8)
                        .collect();
                    cell_index(&image, base)
                })
                .collect();
            out.push(element);
        }
    }
    out
}

pub fn carry(element: &[usize], code: Code) -> Code {
    element
        .iter()
        .enumerate()
        .filter(|(index, _)| code >> index & 1 == 1)
        .map(|(_, &image)| 1u128 << image)
        .sum()
}

pub fn orbit(group: &[Vec<usize>], code: Code) -> BTreeSet<Code> {
    group.iter().map(|element| carry(element, code)).collect()
}

pub fn canonical(group: &[Vec<usize>], code: Code) -> Code {
    orbit(group, code)
        .into_iter()
        .next()
        .expect("the group is not empty")
}

pub fn representatives(base: usize, dimension: usize) -> Vec<(Code, usize)> {
    let group = group(base, dimension);
    let cells = base.pow(dimension as u32);
    assert!(cells <= WALK_LIMIT, "the walk stays under the code limit");
    let mut seen = vec![false; 1 << cells];
    let mut out = Vec::new();
    for code in 0..1u128 << cells {
        if seen[code as usize] {
            continue;
        }
        let orbit = orbit(&group, code);
        out.push((code, orbit.len()));
        for member in orbit {
            seen[member as usize] = true;
        }
    }
    out
}

pub fn burnside(base: usize, dimension: usize) -> u128 {
    distinct_designs(base, dimension).expect("the Burnside average is an integer")
}

pub fn named(base: usize, dimension: usize) -> Vec<(&'static str, Code)> {
    let cells = residue_corners(dimension, base);
    let select = |keep: &dyn Fn(&[u8]) -> bool| {
        let filled: Vec<Vec<u8>> = cells.iter().filter(|cell| keep(cell)).cloned().collect();
        corners_to_code(&filled, dimension, base)
    };
    let free = if dimension == 2 { 0 } else { dimension - 1 };
    vec![
        ("carpet", levels_code(dimension, base, &[0, 1])),
        (
            "net",
            select(&|cell| cell.iter().map(|&r| r as usize).sum::<usize>() + 1 >= dimension),
        ),
        ("void", select(&|cell| cell.iter().all(|&r| r == cell[0]))),
        (
            "tree",
            select(&|cell| (0..dimension).filter(|&a| a != free).all(|a| cell[a] == 0)),
        ),
    ]
}

pub fn report() {
    println!("base 2 cube group: order 2^D D!, designs up to symmetry three ways");
    for dimension in 1..=4 {
        let walk = representatives(2, dimension).len();
        let order = group_order(2, dimension);
        let burnside = burnside(2, dimension);
        let classes = counting::distinct_designs(dimension).expect("the class sums close");
        println!(
            "D {dimension}: order {order} orbit walk {walk} Burnside {burnside} class sum {classes}"
        );
    }
}
