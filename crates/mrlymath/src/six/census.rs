use super::models::Cell6d;
use super::{FILL, GRID, VOID};
use std::collections::{BTreeMap, BTreeSet};

type Point = (i64, i64);
type Edge = (Point, Point);

fn north(x: i64, y: i64) -> [Point; 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

fn south(x: i64, y: i64) -> [Point; 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

/// Returns the three corner points of the triangle at x, y under the start parity.
pub fn corners(x: i64, y: i64, start: i64) -> [Point; 3] {
    if (x + y + start).rem_euclid(2) == 0 {
        north(x, y)
    } else {
        south(x, y)
    }
}

/// Returns a triangle's three edges, each sorted low corner first.
pub fn edges_of(c: &[Point; 3]) -> [Edge; 3] {
    let sorted = |a: Point, b: Point| if a <= b { (a, b) } else { (b, a) };
    [sorted(c[0], c[1]), sorted(c[1], c[2]), sorted(c[0], c[2])]
}

/// The tally of a triangle mesh.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    /// The count of tallied triangles.
    pub triangles: usize,
    /// The count of filled triangles.
    pub fills: usize,
    /// The count of void triangles.
    pub voids: usize,
    /// The count of backdrop triangles.
    pub grids: usize,
    /// The count of distinct corners.
    pub vertices: usize,
    /// The count of distinct edges.
    pub edges: usize,
    /// The count of edges touching one triangle.
    pub boundary_edges: usize,
    /// The count of edges shared by two triangles.
    pub interior_edges: usize,
    /// The Euler characteristic of the mesh.
    pub euler: i64,
}

#[derive(Default)]
struct Mesh {
    vertices: BTreeSet<Point>,
    edges: BTreeMap<Edge, usize>,
}

impl Mesh {
    fn add(&mut self, x: i64, y: i64, start: i64) {
        let c = corners(x, y, start);
        for p in c {
            self.vertices.insert(p);
        }
        for e in edges_of(&c) {
            *self.edges.entry(e).or_insert(0) += 1;
        }
    }
    fn tally(&self, triangles: usize, fills: usize, voids: usize, grids: usize) -> Census {
        let edges = self.edges.len();
        let boundary = self.edges.values().filter(|&&n| n == 1).count();
        Census {
            triangles,
            fills,
            voids,
            grids,
            vertices: self.vertices.len(),
            edges,
            boundary_edges: boundary,
            interior_edges: edges - boundary,
            euler: self.vertices.len() as i64 - edges as i64 + triangles as i64,
        }
    }
}

/// Tallies a cell's triangles, corners and edges, counting the backdrop only on request.
pub fn census(cell: &Cell6d, include_grid: bool) -> Census {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let (mut fills, mut voids, mut grids) = (0, 0, 0);
    let mut mesh = Mesh::default();
    for y in 0..height {
        for x in 0..width {
            let v = inner.types().get(&[y, x]);
            match v {
                FILL => fills += 1,
                VOID => voids += 1,
                GRID => grids += 1,
                _ => {}
            }
            if v == GRID && !include_grid {
                continue;
            }
            mesh.add(x as i64, y as i64, start);
        }
    }
    let triangles = fills + voids + if include_grid { grids } else { 0 };
    mesh.tally(triangles, fills, voids, grids)
}

/// Counts the filled triangles of the cell.
///
/// ```
/// use mrlymath::six::{blank, Cell6d, Orientation, Projection, FILL, VOID};
/// let hex = blank(2, Orientation::Horizontal, FILL, VOID);
/// let cell = Cell6d::new(hex, Projection::Cut, Orientation::Horizontal, 0);
/// assert_eq!(mrlymath::six::census::fills(&cell), 24);
/// ```
pub fn fills(cell: &Cell6d) -> usize {
    cell.cell
        .types()
        .bytes()
        .iter()
        .filter(|&&v| v == FILL)
        .count()
}

/// Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.
pub fn euler(cell: &Cell6d, include_grid: bool) -> i64 {
    census(cell, include_grid).euler
}

/// Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.
pub fn fills_only(cell: &Cell6d) -> Census {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let mut fills = 0;
    let mut mesh = Mesh::default();
    for y in 0..height {
        for x in 0..width {
            if inner.types().get(&[y, x]) != FILL {
                continue;
            }
            fills += 1;
            mesh.add(x as i64, y as i64, start);
        }
    }
    mesh.tally(fills, fills, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::six::geometry::blank;
    use crate::six::{Orientation, Projection};
    use crate::two::Cell2d;
    fn hex(radius: usize) -> Cell6d {
        Cell6d::new(
            blank(radius, Orientation::Horizontal, FILL, VOID),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        )
    }
    #[test]
    fn blank_hexagon_tallies_are_pinned() {
        let expected = [
            (1, 6, 6, 0, 7, 12, 6, 1),
            (2, 28, 24, 4, 22, 49, 14, 1),
            (3, 66, 54, 12, 45, 110, 22, 1),
        ];
        for (radius, triangles, fills, voids, vertices, edges, boundary, euler) in expected {
            let c = census(&hex(radius), false);
            assert_eq!(c.triangles, triangles, "r={radius}");
            assert_eq!(c.fills, fills);
            assert_eq!(c.voids, voids);
            assert_eq!(c.vertices, vertices);
            assert_eq!(c.edges, edges);
            assert_eq!(c.boundary_edges, boundary);
            assert_eq!(c.euler, euler);
        }
    }
    #[test]
    fn single_triangle() {
        let mut t = mrlycore::Tensor::new(vec![1, 2]);
        t.set(&[0, 0], FILL);
        t.set(&[0, 1], GRID);
        let c = census(
            &Cell6d::new(Cell2d::new(t), Projection::Cut, Orientation::Horizontal, 0),
            false,
        );
        assert_eq!(c.triangles, 1);
        assert_eq!(c.vertices, 3);
        assert_eq!(c.edges, 3);
        assert_eq!(c.boundary_edges, 3);
        assert_eq!(c.euler, 1);
    }
    #[test]
    fn fills_only_ignores_the_voids() {
        let solid = hex(2);
        let whole = census(&solid, false);
        let filled = fills_only(&solid);
        assert_eq!(fills(&solid), whole.fills);
        assert_eq!(euler(&solid, false), whole.euler);
        assert_eq!(filled.triangles, whole.fills);
        assert_eq!(filled.voids, 0);
        assert!(filled.edges < whole.edges);
        assert_eq!(fills_only(&solid.clone().anti()).triangles, whole.voids);
    }
}

#[cfg(test)]
mod theorems {
    use super::*;
    use crate::formulas::six as formulas;
    use crate::six::geometry::cut;
    use crate::six::graph::slice_core_graph;
    use crate::three;

    fn solid_slice(number: usize) -> Cell6d {
        cut(&three::ones(number, 1).unwrap()).unwrap()
    }

    fn readings(rec: &Census) -> [i64; 5] {
        [
            rec.triangles as i64,
            rec.boundary_edges as i64,
            rec.edges as i64,
            rec.interior_edges as i64,
            rec.vertices as i64,
        ]
    }

    fn lagrange(seed: [i64; 3], k: i64) -> i64 {
        seed[0] * (k - 2) * (k - 3) / 2 - seed[1] * (k - 1) * (k - 3)
            + seed[2] * (k - 1) * (k - 2) / 2
    }

    #[test]
    fn frame_is_family_invariant() {
        for number in [3, 5, 7, 9, 11] {
            let reference = census(&solid_slice(number), false);
            assert_eq!(reference.euler, 1, "n={number}");
            let families = [
                three::carpet(number, 1).unwrap(),
                three::net(number, 1).unwrap(),
                three::ztree(number, 1).unwrap(),
                three::void(number, 1).unwrap(),
            ];
            for family in families {
                let rec = census(&cut(&family).unwrap(), false);
                assert_eq!(rec.triangles, reference.triangles, "n={number}");
                assert_eq!(rec.vertices, reference.vertices, "n={number}");
                assert_eq!(rec.edges, reference.edges, "n={number}");
                assert_eq!(rec.boundary_edges, reference.boundary_edges, "n={number}");
                assert_eq!(rec.euler, reference.euler, "n={number}");
                assert_eq!(rec.fills + rec.voids, reference.fills, "n={number}");
            }
        }
    }

    #[test]
    fn the_five_closed_forms_match_the_census_to_eight() {
        for index in 1..9usize {
            let number = 2 * index - 1;
            let rec = census(&solid_slice(number), false);
            let k = index as i64;
            assert_eq!(readings(&rec)[0], 24 * k * k - 24 * k + 6, "k={k}");
            assert_eq!(readings(&rec)[1], 12 * k - 6, "k={k}");
            assert_eq!(readings(&rec)[2], 36 * k * k - 30 * k + 6, "k={k}");
            assert_eq!(readings(&rec)[3], 36 * k * k - 42 * k + 12, "k={k}");
            assert_eq!(readings(&rec)[4], 12 * k * k - 6 * k + 1, "k={k}");
            assert_eq!(rec.euler, 1, "k={k}");
            let closed = [
                formulas::solid_slice_triangles(number).unwrap(),
                formulas::solid_slice_boundary(number).unwrap(),
                formulas::solid_slice_edges(number).unwrap(),
                formulas::solid_slice_interior(number).unwrap(),
                formulas::solid_slice_vertices(number).unwrap(),
            ];
            for (got, want) in readings(&rec).iter().zip(closed) {
                assert_eq!(*got as u128, want, "k={k}");
            }
        }
        let three = census(&solid_slice(3), false);
        assert_eq!(readings(&three), [54, 18, 90, 72, 37]);
    }

    #[test]
    fn a_blind_quadratic_fit_reproduces_the_wider_slices() {
        let seed: Vec<[i64; 5]> = (1..4)
            .map(|k| readings(&census(&solid_slice(2 * k - 1), false)))
            .collect();
        for k in 4..11i64 {
            let rec = census(&solid_slice(2 * k as usize - 1), false);
            assert_eq!(rec.euler, 1, "k={k}");
            let got = readings(&rec);
            for j in 0..5 {
                let fitted = lagrange([seed[0][j], seed[1][j], seed[2][j]], k);
                assert_eq!(got[j], fitted, "k={k} count={j}");
            }
        }
    }

    #[test]
    fn fresh_builds_at_the_wide_sides_hold_the_forms() {
        for k in [12usize, 16, 20] {
            let number = 2 * k - 1;
            let rec = census(&solid_slice(number), false);
            assert_eq!(
                readings(&rec).map(|v| v as u128),
                [
                    formulas::solid_slice_triangles(number).unwrap(),
                    formulas::solid_slice_boundary(number).unwrap(),
                    formulas::solid_slice_edges(number).unwrap(),
                    formulas::solid_slice_interior(number).unwrap(),
                    formulas::solid_slice_vertices(number).unwrap(),
                ],
                "k={k}"
            );
            assert_eq!(rec.euler, 1, "k={k}");
            if number == 39 {
                assert_eq!(readings(&rec), [9126, 234, 13806, 13572, 4681]);
            }
        }
    }

    #[test]
    fn the_fill_adjacency_counts_the_sub_mesh_interior_edges() {
        let mut meshes = 0;
        for level in 1..5usize {
            for design in [None, Some(23u128), Some(232), Some(3), Some(129)] {
                let cell = match design {
                    None => three::ones(3, level).unwrap(),
                    Some(code) => three::create(code, 3, level, 2).unwrap(),
                };
                let slice = cut(&cell).unwrap();
                let sub = fills_only(&slice);
                let core = slice_core_graph(&slice).unwrap();
                assert_eq!(
                    core.nodes.len(),
                    sub.triangles,
                    "design={design:?} l={level}"
                );
                assert_eq!(
                    core.branches.len(),
                    sub.edges - sub.boundary_edges,
                    "design={design:?} l={level}"
                );
                if design == Some(23) && level == 4 {
                    assert_eq!(
                        (sub.edges, sub.boundary_edges, core.branches.len()),
                        (28188, 6642, 21546)
                    );
                }
                meshes += 1;
            }
        }
        assert_eq!(meshes, 20);
    }

    #[test]
    fn the_lemma_needs_the_sub_mesh_and_not_the_hexagon() {
        let slice = cut(&three::carpet(3, 3).unwrap()).unwrap();
        let sub = fills_only(&slice);
        let whole = census(&slice, false);
        assert_eq!(slice_core_graph(&slice).unwrap().branches.len(), 2880);
        assert_eq!(sub.edges - sub.boundary_edges, 2880);
        assert_eq!(whole.edges - whole.boundary_edges, 6480);
    }
}
