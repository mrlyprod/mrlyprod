use super::census::{corners, edges_of, fills_only};
use super::graph::slice_core_graph;
use super::models::Cell6d;
use super::{FILL, VOID};
use mrlycore::errors::{value_error, Result};
use mrlynum::graph::largest_component;
use mrlynum::graph::models::Network;
use mrlynum::spectrum::{laplacian_spectrum, spectral_exponent as slope};
use std::collections::BTreeMap;

type Point = (i64, i64);
type Edge = (Point, Point);

fn pieces(network: &Network) -> Vec<usize> {
    let n = network.nodes.len();
    let adjacency = network.adjacency();
    let mut seen = vec![false; n];
    let mut sizes = Vec::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        let mut size = 0;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(current) = stack.pop() {
            size += 1;
            for &neighbor in &adjacency[&current] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        sizes.push(size);
    }
    sizes
}

/// Counts the connected pieces of the fill, triangles joined across shared edges.
///
/// ```
/// let slice = mrlymath::six::cut(&mrlymath::three::carpet(5, 1).unwrap()).unwrap();
/// assert_eq!(mrlymath::six::topology::components(&slice).unwrap(), 7);
/// ```
pub fn components(cell: &Cell6d) -> Result<usize> {
    Ok(pieces(&slice_core_graph(cell)?).len())
}

/// Returns the triangle count of the fill's largest connected piece.
pub fn giant(cell: &Cell6d) -> Result<usize> {
    Ok(pieces(&slice_core_graph(cell)?)
        .into_iter()
        .max()
        .unwrap_or(0))
}

/// Returns the largest connected piece of the filled-triangle network as a network of its own.
pub fn giant_network(cell: &Cell6d) -> Result<Network> {
    Ok(largest_component(&slice_core_graph(cell)?))
}

/// Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.
pub fn spectral_exponent(cell: &Cell6d, window: f64) -> Result<f64> {
    let spectrum = laplacian_spectrum(&giant_network(cell)?, true)?;
    match slope(&spectrum, window) {
        Some(value) => Ok(value),
        None => value_error("The giant piece is too small to fit an exponent."),
    }
}

/// Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.
///
/// ```
/// let slice = mrlymath::six::cut(&mrlymath::three::carpet(3, 1).unwrap()).unwrap();
/// assert_eq!(mrlymath::six::topology::holes(&slice).unwrap(), 1);
/// ```
pub fn holes(cell: &Cell6d) -> Result<usize> {
    let count = components(cell)? as i64;
    Ok((count - fills_only(cell).euler).max(0) as usize)
}

/// Counts the void regions the rim never reaches, the second route to the hole count.
pub fn rim_holes(cell: &Cell6d) -> Result<usize> {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let mut sites = Vec::new();
    let mut mesh: BTreeMap<Edge, usize> = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            let v = inner.types().get(&[y, x]);
            if v != FILL && v != VOID {
                continue;
            }
            for edge in edges_of(&corners(x as i64, y as i64, start)) {
                *mesh.entry(edge).or_insert(0) += 1;
            }
            if v == VOID {
                sites.push((x as i64, y as i64));
            }
        }
    }
    let mut owners: BTreeMap<Edge, Vec<usize>> = BTreeMap::new();
    for (index, &(x, y)) in sites.iter().enumerate() {
        for edge in edges_of(&corners(x, y, start)) {
            owners.entry(edge).or_default().push(index);
        }
    }
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); sites.len()];
    let mut on_rim = vec![false; sites.len()];
    for (edge, shared) in &owners {
        if mesh[edge] == 1 {
            for &index in shared {
                on_rim[index] = true;
            }
        }
        if shared.len() == 2 {
            adjacency[shared[0]].push(shared[1]);
            adjacency[shared[1]].push(shared[0]);
        }
    }
    let mut seen = vec![false; sites.len()];
    let mut enclosed = 0;
    for start in 0..sites.len() {
        if seen[start] {
            continue;
        }
        let mut open = false;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(current) = stack.pop() {
            open |= on_rim[current];
            for &neighbor in &adjacency[current] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        if !open {
            enclosed += 1;
        }
    }
    Ok(enclosed)
}

#[cfg(test)]
mod theorems {
    use super::*;
    use crate::bang::universe::orbit;
    use crate::formulas::six::centered_hexagonal;
    use crate::six::census::census;
    use crate::six::geometry::cut;
    use crate::six::GRID;
    use crate::three::{self, Cell3d};
    use mrlynum::graph::census::components as network_components;

    fn slice(code: u128, number: usize, level: usize) -> Cell6d {
        cut(&three::create(code, number, level, 2).unwrap()).unwrap()
    }

    fn section_area(cell: &Cell3d) -> u128 {
        let grid = cell.types();
        let side = grid.shape[0];
        let mut total = 0;
        for i in 0..side {
            for j in 0..side {
                for l in 0..side {
                    if grid.get(&[i, j, l]) == 0 {
                        continue;
                    }
                    let layer = 3 * side as i64 - 2 * (i + j + l) as i64;
                    total += match layer {
                        1 | 5 => 1,
                        2 | 4 => 4,
                        3 => 6,
                        _ => 0,
                    };
                }
            }
        }
        total
    }

    #[test]
    fn the_four_families_fill_the_slice_and_name_their_classes() {
        let expected = [
            (23u128, 42usize, 23u128),
            (232, 12, 23),
            (3, 18, 3),
            (129, 12, 24),
        ];
        for (code, fill, class) in expected {
            let cut = slice(code, 3, 1);
            assert_eq!(census(&cut, false).fills, fill, "code={code}");
            assert_eq!(*orbit(code, 3).iter().next().unwrap(), class, "code={code}");
        }
    }

    #[test]
    fn carpet_and_net_partition_the_hexagon_triangle_by_triangle() {
        let pairs = [
            (3usize, 42u128, 12u128),
            (5, 72, 78),
            (7, 204, 90),
            (9, 210, 276),
            (11, 486, 240),
        ];
        for number in (1..32).step_by(2) {
            let carpet = slice(23, number, 1);
            let net = slice(232, number, 1);
            let (left, right) = (carpet.cell.types(), net.cell.types());
            assert_eq!(left.shape, right.shape, "n={number}");
            let mut filled = 0;
            for (index, &value) in left.bytes().iter().enumerate() {
                let other = right.bytes()[index];
                if value == GRID || other == GRID {
                    assert_eq!(value, other, "n={number}");
                    continue;
                }
                assert_ne!(value, other, "n={number}");
                if value == FILL || other == FILL {
                    filled += 1;
                }
            }
            let (a, b) = (
                census(&carpet, false).fills as u128,
                census(&net, false).fills as u128,
            );
            assert_eq!(filled as u128, 6 * (number as u128).pow(2), "n={number}");
            assert_eq!(a + b, 6 * (number as u128).pow(2), "n={number}");
            if let Some(&(_, want_a, want_b)) = pairs.iter().find(|p| p.0 == number) {
                assert_eq!((a, b), (want_a, want_b), "n={number}");
            }
            if number == 31 {
                assert_eq!((a, b, a + b), (3696, 2070, 5766));
            }
        }
    }

    #[test]
    fn the_layer_weighted_area_is_a_second_route_to_the_fill() {
        for number in 1..17usize {
            let carpet = three::create(23, number, 1, 2).unwrap();
            let net = three::create(232, number, 1, 2).unwrap();
            let whole = 6 * (number as u128).pow(2);
            assert_eq!(
                section_area(&carpet) + section_area(&net),
                whole,
                "n={number}"
            );
            if number.is_multiple_of(2) {
                assert_eq!(section_area(&carpet), whole / 2, "n={number}");
                continue;
            }
            for code in [23u128, 232, 3, 129] {
                let cell = three::create(code, number, 1, 2).unwrap();
                assert_eq!(
                    section_area(&cell),
                    census(&cut(&cell).unwrap(), false).fills as u128,
                    "code={code} n={number}"
                );
            }
        }
    }

    #[test]
    fn the_carpet_slice_counts_its_pieces_and_holes_two_ways() {
        let pieces = [1, 1, 7, 1, 19, 1, 37, 1, 61, 1, 91, 1, 127, 1];
        let punctures = [0, 1, 0, 7, 0, 19, 0, 37, 0, 61, 0, 91, 0, 127];
        for (index, (&want_pieces, &want_holes)) in pieces.iter().zip(punctures.iter()).enumerate()
        {
            let k = index + 1;
            let cut = slice(23, 2 * k - 1, 1);
            assert_eq!(components(&cut).unwrap(), want_pieces, "k={k}");
            assert_eq!(
                network_components(&slice_core_graph(&cut).unwrap()),
                want_pieces,
                "k={k}"
            );
            assert_eq!(holes(&cut).unwrap(), want_holes, "k={k}");
            assert_eq!(rim_holes(&cut).unwrap(), want_holes, "k={k}");
            let law = centered_hexagonal(k.div_ceil(2)) as usize;
            assert_eq!(
                if k.is_multiple_of(2) {
                    want_holes
                } else {
                    want_pieces
                },
                law,
                "k={k}"
            );
        }
    }

    #[test]
    fn the_other_families_puncture_in_opposite_phase() {
        for k in 1..11usize {
            for code in [3u128, 129] {
                let cut = slice(code, 2 * k - 1, 1);
                assert_eq!(holes(&cut).unwrap(), 0, "code={code} k={k}");
                assert_eq!(rim_holes(&cut).unwrap(), 0, "code={code} k={k}");
            }
        }
        for (k, want) in [(3usize, 1usize), (5, 7), (7, 19), (9, 37)] {
            let cut = slice(232, 2 * k - 1, 1);
            assert_eq!(holes(&cut).unwrap(), want, "k={k}");
            assert_eq!(rim_holes(&cut).unwrap(), want, "k={k}");
        }
    }

    #[test]
    fn the_carpet_slice_percolates_at_base_three() {
        for (level, triangles) in [(1usize, 42usize), (2, 306), (3, 2250), (4, 16578)] {
            let cut = slice(23, 3, level);
            assert_eq!(census(&cut, false).fills, triangles, "l={level}");
            assert_eq!(components(&cut).unwrap(), 1, "l={level}");
            assert_eq!(giant(&cut).unwrap(), triangles, "l={level}");
            if level == 4 {
                let core = slice_core_graph(&cut).unwrap();
                assert_eq!((core.nodes.len(), core.branches.len()), (16578, 21546));
            }
        }
    }

    #[test]
    fn the_other_slices_shatter_or_never_grow_at_base_three() {
        for level in 1..5usize {
            let net = slice(232, 3, level);
            assert_eq!(census(&net, false).fills, 12, "l={level}");
            assert_eq!(components(&net).unwrap(), 1, "l={level}");
        }
        let mut tree = 0;
        let mut anti = 0;
        for level in 1..4usize {
            let cut = slice(3, 3, level);
            assert_eq!(giant(&cut).unwrap(), 8, "l={level}");
            assert!(components(&cut).unwrap() > tree, "l={level}");
            tree = components(&cut).unwrap();
            let cut = slice(129, 3, level);
            assert_eq!(giant(&cut).unwrap(), 6, "l={level}");
            assert!(components(&cut).unwrap() > anti, "l={level}");
            anti = components(&cut).unwrap();
        }
        assert_eq!((tree, anti), (40, 55));
    }

    #[test]
    fn the_carpet_slice_fails_to_percolate_at_base_five() {
        let cut = slice(23, 5, 2);
        assert_eq!(census(&cut, false).fills, 1164);
        assert_eq!(components(&cut).unwrap(), 20);
        assert_eq!(giant(&cut).unwrap(), 192);
    }
}

#[cfg(test)]
mod spectra {
    use super::*;
    use crate::six::geometry::cut;
    use crate::three;
    use mrlynum::spectrum::laplacian_spectrum;

    fn slice(code: u128, number: usize, level: usize) -> Cell6d {
        cut(&three::create(code, number, level, 2).unwrap()).unwrap()
    }

    fn reading(code: u128, number: usize, level: usize) -> (usize, usize, f64) {
        let cell = slice(code, number, level);
        let piece = giant_network(&cell).unwrap();
        let spectrum = laplacian_spectrum(&piece, true).unwrap();
        let zeros = spectrum.iter().filter(|v| **v < 1e-10).count();
        (
            piece.nodes.len(),
            zeros,
            spectral_exponent(&cell, 0.1).unwrap(),
        )
    }

    #[test]
    fn the_slice_exponent_climbs_with_the_size_it_is_read_at() {
        let rows = [
            (23u128, 3usize, 1usize, 42usize, 0.91),
            (23, 3, 2, 306, 1.25),
            (255, 9, 1, 486, 1.61),
            (23, 5, 2, 192, 1.05),
        ];
        for (code, number, level, nodes, want) in rows {
            let (got_nodes, zeros, exponent) = reading(code, number, level);
            assert_eq!(got_nodes, nodes, "code={code} n={number} l={level}");
            assert_eq!(zeros, 1, "code={code} n={number} l={level}");
            assert!(
                (exponent - want).abs() < 0.005,
                "code={code} n={number} l={level} {exponent}"
            );
        }
    }

    #[test]
    #[ignore = "two thousand nodes each; run it in release"]
    fn the_deep_slices_hold_their_spectral_exponents() {
        for (code, number, level, nodes, want) in [
            (23u128, 3usize, 3usize, 2250usize, 1.44),
            (255, 19, 1, 2166, 1.79),
        ] {
            let (got_nodes, zeros, exponent) = reading(code, number, level);
            assert_eq!(got_nodes, nodes, "code={code} n={number}");
            assert_eq!(zeros, 1, "code={code} n={number}");
            assert!(
                (exponent - want).abs() < 0.005,
                "code={code} n={number} {exponent}"
            );
        }
    }
}
