use crate::geometry::Point;
use crate::models::{Cell4d, Result};
use crate::renderer::sheet;
use crate::tag::{E, N, S, W};

const STEPS: [Point; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const NORTH_EAST: u8 = N | E;
const SOUTH_WEST: u8 = S | W;
const EAST_SOUTH: u8 = E | S;
const WEST_NORTH: u8 = W | N;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arc {
    pub center: Point,
    pub end: Point,
    pub sweep: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Strand {
    pub start: Point,
    pub arcs: Vec<Arc>,
    pub closed: bool,
}

impl Strand {
    pub fn ring(&self) -> Option<Point> {
        let center = self.arcs.first()?.center;
        let round = self.arcs.len() == 4 && self.arcs.iter().all(|arc| arc.center == center);
        (self.closed && round).then_some(center)
    }
    pub fn drifts(&self) -> bool {
        self.closed && self.arcs.last().is_some_and(|arc| arc.end != self.start)
    }
}

fn swing(tag: u8) -> Option<usize> {
    match tag {
        NORTH_EAST | SOUTH_WEST => Some(1),
        EAST_SOUTH | WEST_NORTH => Some(3),
        _ => None,
    }
}

fn midpoint(x: i64, y: i64, side: usize) -> Point {
    let (dx, dy) = STEPS[side];
    (2 * x + 1 + dx, 2 * y + 1 + dy)
}

fn arc(x: i64, y: i64, entry: usize, exit: usize) -> Arc {
    let (from, to) = (midpoint(x, y, entry), midpoint(x, y, exit));
    let center = (from.0 + to.0 - 2 * x - 1, from.1 + to.1 - 2 * y - 1);
    let turn = (from.0 - center.0) * (to.1 - center.1) - (from.1 - center.1) * (to.0 - center.0);
    Arc {
        center,
        end: to,
        sweep: turn > 0,
    }
}

fn wrapped(cell: &Cell4d, x: i64, y: i64) -> (usize, usize) {
    (
        x.rem_euclid(cell.width as i64) as usize,
        y.rem_euclid(cell.height as i64) as usize,
    )
}

fn across(cell: &Cell4d, wrap: bool, x: i64, y: i64, side: usize) -> Option<(i64, i64, usize)> {
    let (dx, dy) = STEPS[side];
    let (next_x, next_y) = (x + dx, y + dy);
    let inside =
        (0..cell.width as i64).contains(&next_x) && (0..cell.height as i64).contains(&next_y);
    if !wrap && !inside {
        return None;
    }
    let (at_x, at_y) = wrapped(cell, next_x, next_y);
    swing(cell.tag(at_x, at_y)).map(|_| (next_x, next_y, (side + 2) % 4))
}

fn walk(cell: &Cell4d, wrap: bool, seen: &mut [bool], x: i64, y: i64, side: usize) -> Strand {
    let slot = |(at_x, at_y): (usize, usize), side: usize| 4 * (at_y * cell.width + at_x) + side;
    let first = (wrapped(cell, x, y), side);
    let (mut x, mut y, mut entry) = (x, y, side);
    let mut strand = Strand {
        start: midpoint(x, y, entry),
        arcs: Vec::new(),
        closed: false,
    };
    loop {
        let at = wrapped(cell, x, y);
        let Some(turn) = swing(cell.tag(at.0, at.1)) else {
            return strand;
        };
        let exit = entry ^ turn;
        seen[slot(at, entry)] = true;
        seen[slot(at, exit)] = true;
        strand.arcs.push(arc(x, y, entry, exit));
        let Some(next) = across(cell, wrap, x, y, exit) else {
            return strand;
        };
        (x, y, entry) = next;
        if (wrapped(cell, x, y), entry) == first {
            strand.closed = true;
            return strand;
        }
    }
}

pub fn strands(cell: &Cell4d, wrap: bool) -> Vec<Strand> {
    let mut seen = vec![false; 4 * cell.width * cell.height];
    let mut out = Vec::new();
    for open in [true, false] {
        for flat in 0..seen.len() {
            let (index, side) = (flat / 4, flat % 4);
            let (x, y) = ((index % cell.width) as i64, (index / cell.width) as i64);
            let half = swing(cell.tag(index % cell.width, index / cell.width)).is_some();
            if half && !seen[flat] && across(cell, wrap, x, y, side).is_none() == open {
                out.push(walk(cell, wrap, &mut seen, x, y, side));
            }
        }
    }
    out
}

pub fn max_stroke(scale: usize) -> usize {
    (2 * scale * scale).isqrt() - scale
}

fn checked(scale: usize, stroke: usize) -> Result<()> {
    if scale == 0 || !scale.is_multiple_of(2) {
        return Err(format!("scale {scale} must be even and above zero."));
    }
    let peak = max_stroke(scale);
    if stroke == 0 || stroke > peak {
        return Err(format!(
            "stroke {stroke} is outside 1..={peak} at scale {scale}."
        ));
    }
    Ok(())
}

pub fn corners(tag: u8, scale: usize) -> Option<[Point; 2]> {
    let far = 2 * scale as i64;
    swing(tag).map(|turn| match turn {
        1 => [(far, 0), (0, far)],
        _ => [(0, 0), (far, far)],
    })
}

pub fn on_ring((x, y): Point, (cx, cy): Point, scale: usize, stroke: usize) -> bool {
    let inner = scale.saturating_sub(stroke) as i64;
    let outer = (scale + stroke) as i64;
    let distance = (x - cx).pow(2) + (y - cy).pow(2);
    inner * inner <= distance && distance <= outer * outer
}

pub fn inked(tag: u8, point: Point, scale: usize, stroke: usize) -> bool {
    corners(tag, scale).is_some_and(|pair| {
        pair.iter()
            .any(|&corner| on_ring(point, corner, scale, stroke))
    })
}

pub fn raster(cell: &Cell4d, scale: usize, stroke: usize) -> Result<Vec<u8>> {
    checked(scale, stroke)?;
    let (width, height) = (cell.width * scale, cell.height * scale);
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let point = ((2 * (x % scale) + 1) as i64, (2 * (y % scale) + 1) as i64);
            let tag = cell.tag(x / scale, y / scale);
            out.push(u8::from(inked(tag, point, scale, stroke)));
        }
    }
    Ok(out)
}

fn shifts(strand: &Strand, (width, height): Point) -> Vec<Point> {
    let points = strand.arcs.iter().flat_map(|arc| [arc.center, arc.end]);
    let (mut low, mut high) = (strand.start, strand.start);
    for (x, y) in points {
        low = (low.0.min(x), low.1.min(y));
        high = (high.0.max(x), high.1.max(y));
    }
    let mut out = Vec::new();
    for across in (-high.0).div_euclid(width)..=(width - low.0).div_euclid(width) {
        for down in (-high.1).div_euclid(height)..=(height - low.1).div_euclid(height) {
            let (dx, dy) = (across * width, down * height);
            let seen_x = low.0 + dx < width && high.0 + dx > 0;
            let seen_y = low.1 + dy < height && high.1 + dy > 0;
            if seen_x && seen_y {
                out.push((dx, dy));
            }
        }
    }
    out
}

fn element(strand: &Strand, (dx, dy): Point, radius: i64) -> String {
    let place = |(x, y): Point| ((x + dx) * radius, (y + dy) * radius);
    if let Some(center) = strand.ring() {
        let (cx, cy) = place(center);
        return format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{radius}\"/>");
    }
    let (x, y) = place(strand.start);
    let mut path = format!("M{x} {y}");
    for arc in &strand.arcs {
        let (x, y) = place(arc.end);
        let sweep = u8::from(arc.sweep);
        path.push_str(&format!("A{radius} {radius} 0 0 {sweep} {x} {y}"));
    }
    if strand.closed {
        path.push('Z');
    }
    format!("<path d=\"{path}\"/>")
}

pub fn svg(
    cell: &Cell4d,
    scale: usize,
    stroke: usize,
    [paper, ink]: [&str; 2],
    wrap: bool,
) -> Result<String> {
    checked(scale, stroke)?;
    let radius = (scale / 2) as i64;
    let frame = (2 * cell.width as i64, 2 * cell.height as i64);
    let mut out = sheet(cell.width * scale, cell.height * scale, "", paper);
    out.push(format!(
        "<g fill=\"none\" stroke=\"{ink}\" stroke-width=\"{stroke}\" stroke-linecap=\"butt\">"
    ));
    for strand in strands(cell, wrap) {
        if strand.drifts() {
            return Err("a strand circles the torus: draw a tiled patch flat instead.".to_string());
        }
        let images = if wrap {
            shifts(&strand, frame)
        } else {
            vec![(0, 0)]
        };
        for shift in images {
            out.push(element(&strand, shift, radius));
        }
    }
    out.push("</g>".to_string());
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}
