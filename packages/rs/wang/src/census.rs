use crate::geometry::{Point, triangle};
use crate::models::CellWang;
use std::collections::{BTreeMap, BTreeSet};

type Edge = (Point, Point);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Census {
    pub fills: usize,
    pub voids: usize,
    pub straight: usize,
    pub diagonal: usize,
    pub vertices: usize,
    pub edges: usize,
    pub euler: i64,
}

pub fn fills(cell: &CellWang) -> usize {
    cell.types.iter().filter(|&&value| value != 0).count()
}

pub fn voids(cell: &CellWang) -> usize {
    cell.types.iter().filter(|&&value| value == 0).count()
}

fn mesh(cell: &CellWang) -> (BTreeSet<Point>, BTreeMap<Edge, usize>) {
    let sorted = |a: Point, b: Point| if a <= b { (a, b) } else { (b, a) };
    let mut points = BTreeSet::new();
    let mut edges = BTreeMap::new();
    for y in 0..cell.height {
        for x in 0..cell.width {
            for (side, &value) in cell.at(x, y).iter().enumerate() {
                if value == 0 {
                    continue;
                }
                let [a, b, c] = triangle(x, y, side);
                points.extend([a, b, c]);
                for edge in [sorted(a, b), sorted(b, c), sorted(a, c)] {
                    *edges.entry(edge).or_insert(0) += 1;
                }
            }
        }
    }
    (points, edges)
}

pub fn perimeter(cell: &CellWang) -> (usize, usize) {
    let (_, edges) = mesh(cell);
    let exposed = edges.iter().filter(|&(_, &count)| count == 1);
    let (straight, diagonal): (Vec<_>, Vec<_>) =
        exposed.partition(|&(&(a, b), _)| a.0 == b.0 || a.1 == b.1);
    (straight.len(), diagonal.len())
}

pub fn euler(cell: &CellWang) -> i64 {
    let (points, edges) = mesh(cell);
    points.len() as i64 - edges.len() as i64 + fills(cell) as i64
}

pub fn census(cell: &CellWang) -> Census {
    let (points, edges) = mesh(cell);
    let (straight, diagonal) = perimeter(cell);
    Census {
        fills: fills(cell),
        voids: voids(cell),
        straight,
        diagonal,
        vertices: points.len(),
        edges: edges.len(),
        euler: euler(cell),
    }
}

pub fn seams(cell: &CellWang, wrap: bool) -> usize {
    let mut count = 0;
    for y in 0..cell.height {
        for x in 0..cell.width {
            let here = cell.at(x, y);
            if wrap || x + 1 < cell.width {
                count += usize::from(here[1] != cell.at((x + 1) % cell.width, y)[3]);
            }
            if wrap || y + 1 < cell.height {
                count += usize::from(here[2] != cell.at(x, (y + 1) % cell.height)[0]);
            }
        }
    }
    count
}
