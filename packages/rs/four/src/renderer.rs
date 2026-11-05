use crate::geometry::{Point, triangle};
use crate::models::{Cell4d, Result};
use std::collections::{BTreeMap, BTreeSet};

pub fn text(cell: &Cell4d) -> Vec<String> {
    (0..cell.height)
        .map(|y| {
            (0..cell.width)
                .map(|x| format!("{:x}", cell.tag(x, y)))
                .collect()
        })
        .collect()
}

pub fn blocks(cell: &Cell4d) -> Vec<String> {
    cell.types
        .chunks(2 * cell.width)
        .map(|row| {
            row.iter()
                .map(|&value| char::from_digit(u32::from(value), 36).unwrap_or('?'))
                .collect()
        })
        .collect()
}

fn side_at(x: usize, y: usize, scale: usize) -> usize {
    let (u, v) = (
        8 * x as i64 + 5 - 4 * scale as i64,
        8 * y as i64 + 4 - 4 * scale as i64,
    );
    match (v.abs() > u.abs(), v < 0, u > 0) {
        (true, true, _) => 0,
        (true, false, _) => 2,
        (false, _, true) => 1,
        (false, _, false) => 3,
    }
}

pub fn raster(cell: &Cell4d, scale: usize) -> Vec<u8> {
    let (width, height) = (cell.width * scale, cell.height * scale);
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let sides = cell.at(x / scale, y / scale);
            out.push(sides[side_at(x % scale, y % scale, scale)]);
        }
    }
    out
}

fn straighten(ring: Vec<Point>) -> Vec<Point> {
    let count = ring.len();
    (0..count)
        .filter(|&at| {
            let (a, b, c) = (
                ring[(at + count - 1) % count],
                ring[at],
                ring[(at + 1) % count],
            );
            (b.0 - a.0) * (c.1 - b.1) != (b.1 - a.1) * (c.0 - b.0)
        })
        .map(|at| ring[at])
        .collect()
}

fn outline(cell: &Cell4d, keep: impl Fn(u8) -> bool) -> Vec<Vec<Point>> {
    let mut edges = BTreeSet::new();
    for y in 0..cell.height {
        for x in 0..cell.width {
            for (side, &value) in cell.at(x, y).iter().enumerate() {
                if !keep(value) {
                    continue;
                }
                let [a, b, c] = triangle(x, y, side);
                for (from, to) in [(a, b), (b, c), (c, a)] {
                    if !edges.remove(&(to, from)) {
                        edges.insert((from, to));
                    }
                }
            }
        }
    }
    let mut next: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
    for (from, to) in edges {
        next.entry(from).or_default().push(to);
    }
    let mut rings = Vec::new();
    while let Some(&start) = next.keys().next() {
        let (mut ring, mut at) = (vec![start], start);
        while let Some(ends) = next.get_mut(&at) {
            let to = ends.pop().unwrap_or(start);
            if ends.is_empty() {
                next.remove(&at);
            }
            if to == start {
                break;
            }
            ring.push(to);
            at = to;
        }
        rings.push(straighten(ring));
    }
    rings
}

fn path(rings: &[Vec<Point>], scale: usize) -> String {
    let unit = scale as f64 / 2.0;
    let mut out = String::new();
    for ring in rings {
        for (at, point) in ring.iter().enumerate() {
            let verb = if at == 0 { 'M' } else { 'L' };
            out.push_str(&format!(
                "{verb}{} {}",
                point.0 as f64 * unit,
                point.1 as f64 * unit
            ));
        }
        out.push('Z');
    }
    out
}

fn ready(cell: &Cell4d, palette: &[&str]) -> Result<()> {
    match cell.types.iter().max() {
        Some(&peak) if usize::from(peak) >= palette.len() => Err(format!(
            "type {peak} has no colour in a palette of {}.",
            palette.len()
        )),
        _ if palette.is_empty() => Err("a drawing wants at least one colour.".to_string()),
        _ => Ok(()),
    }
}

pub(crate) fn sheet(
    width: usize,
    height: usize,
    attributes: &str,
    background: &str,
) -> Vec<String> {
    vec![
        format!(
            "<svg width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" {attributes}xmlns=\"http://www.w3.org/2000/svg\">"
        ),
        format!("<rect width=\"{width}\" height=\"{height}\" fill=\"{background}\"/>"),
    ]
}

pub fn svg(cell: &Cell4d, scale: usize, palette: &[&str]) -> Result<String> {
    ready(cell, palette)?;
    let mut out = sheet(cell.width * scale, cell.height * scale, "", palette[0]);
    for (layer, color) in palette.iter().enumerate().skip(1) {
        let rings = outline(cell, |value| usize::from(value) >= layer);
        if !rings.is_empty() {
            out.push(format!(
                "<path fill=\"{color}\" d=\"{}\"/>",
                path(&rings, scale)
            ));
        }
    }
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}

pub fn rect_svg(cell: &Cell4d, scale: usize, palette: &[&str]) -> Result<String> {
    ready(cell, palette)?;
    let [rows, cols] = cell.shape();
    let attributes = "shape-rendering=\"crispEdges\" ";
    let mut out = sheet(cols * scale, rows * scale, attributes, palette[0]);
    for (flat, &value) in cell.types.iter().enumerate() {
        if value != 0 {
            let (x, y) = (flat % cols * scale, flat / cols * scale);
            let color = palette[usize::from(value)];
            out.push(format!(
                "<rect x=\"{x}\" y=\"{y}\" width=\"{scale}\" height=\"{scale}\" fill=\"{color}\"/>"
            ));
        }
    }
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}
