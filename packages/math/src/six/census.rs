use super::models::Cell6d;
use super::{FILL, GRID, VOID};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

type Point = (i64, i64);
type Edge = (Point, Point);

fn north(x: i64, y: i64) -> [Point; 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

fn south(x: i64, y: i64) -> [Point; 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

pub(crate) fn corners(x: i64, y: i64, start: i64) -> [Point; 3] {
    if (x + y + start).rem_euclid(2) == 0 {
        north(x, y)
    } else {
        south(x, y)
    }
}

pub(crate) fn edges_of(c: &[Point; 3]) -> [Edge; 3] {
    let sorted = |a: Point, b: Point| if a <= b { (a, b) } else { (b, a) };
    [sorted(c[0], c[1]), sorted(c[1], c[2]), sorted(c[0], c[2])]
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Census {
    pub triangles: usize,
    pub fills: usize,
    pub voids: usize,
    pub grids: usize,
    pub vertices: usize,
    pub edges: usize,
    pub boundary_edges: usize,
    pub interior_edges: usize,
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

pub fn census(cell: &Cell6d, include_grid: bool) -> Census {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let (mut fills, mut voids, mut grids) = (0, 0, 0);
    let mut mesh = Mesh::default();
    for y in 0..height {
        for x in 0..width {
            let v = inner.types().at(y * width + x) as u8;
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

pub fn fills(cell: &Cell6d) -> usize {
    cell.cell.types().count(FILL)
}

pub fn euler(cell: &Cell6d, include_grid: bool) -> i64 {
    census(cell, include_grid).euler
}

pub fn fills_only(cell: &Cell6d) -> Census {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let mut fills = 0;
    let mut mesh = Mesh::default();
    for y in 0..height {
        for x in 0..width {
            if inner.types().at(y * width + x) != i64::from(FILL) {
                continue;
            }
            fills += 1;
            mesh.add(x as i64, y as i64, start);
        }
    }
    mesh.tally(fills, fills, 0, 0)
}
