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

pub fn corners(x: i64, y: i64, start: i64) -> [Point; 3] {
    if (x + y + start).rem_euclid(2) == 0 {
        north(x, y)
    } else {
        south(x, y)
    }
}

pub fn edges_of(c: &[Point; 3]) -> [Edge; 3] {
    let sorted = |a: Point, b: Point| if a <= b { (a, b) } else { (b, a) };
    [sorted(c[0], c[1]), sorted(c[1], c[2]), sorted(c[0], c[2])]
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub fn census(cell: &Cell6d, include_grid: bool) -> Census {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let (mut fills, mut voids, mut grids) = (0, 0, 0);
    let mut vertices: BTreeSet<Point> = BTreeSet::new();
    let mut edge_count: BTreeMap<Edge, usize> = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            let v = inner.types().get(&[y, x]);
            if v == GRID && !include_grid {
                grids += 1;
                continue;
            }
            match v {
                FILL => fills += 1,
                VOID => voids += 1,
                GRID => grids += 1,
                _ => {}
            }
            let c = corners(x as i64, y as i64, start);
            for p in c {
                vertices.insert(p);
            }
            for e in edges_of(&c) {
                *edge_count.entry(e).or_insert(0) += 1;
            }
        }
    }
    let triangles = fills + voids + if include_grid { grids } else { 0 };
    let edges = edge_count.len();
    let boundary = edge_count.values().filter(|&&n| n == 1).count();
    Census {
        triangles,
        fills,
        voids,
        grids,
        vertices: vertices.len(),
        edges,
        boundary_edges: boundary,
        interior_edges: edges - boundary,
        euler: vertices.len() as i64 - edges as i64 + triangles as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::six::geometry::blank;
    use crate::math::six::{Orientation, Projection};
    use crate::math::two::Cell2d;
    fn hex(radius: usize) -> Cell6d {
        Cell6d::new(
            blank(radius, Orientation::Horizontal, FILL, VOID),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        )
    }
    #[test]
    fn blank_hexagons_match_python_census() {
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
        let mut t = crate::core::Tensor::new(vec![1, 2]);
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
}
