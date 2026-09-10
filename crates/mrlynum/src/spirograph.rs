use crate::factor::gcd;
use mrlycore::errors::{value_error, Result};
use mrlycore::Rng;
use std::collections::HashSet;
use std::f64::consts::{PI, TAU};

/// The largest radius of a ring or a wheel.
pub const RADIUS_CAP: usize = 96;
/// The fewest and the most sides of a polygon track.
pub const SIDES: (usize, usize) = (3, 12);
/// The most laps of a line or a polygon.
pub const LAPS_CAP: usize = 24;
/// The most pencils a wheel seats.
pub const PENCIL_CAP: usize = 4096;
/// The most points one trace returns.
pub const POINT_CAP: usize = 4_000_000;
const REACH: (f64, f64) = (0.05, 2.0);
const RING: usize = 360;

// THE PENCILS

/// What a pencil sits on: a filled cell, an empty cell, or a corner of a filled cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The centre of a filled cell.
    Fill,
    /// The centre of an empty cell.
    Void,
    /// A corner of a filled cell.
    Corner,
}

/// A pencil on the wheel: its seat in units of the wheel's radius with the tile centre at the origin, the exact seat it came from, and its kind.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pencil {
    /// The seat's abscissa, in wheel radii.
    pub x: f64,
    /// The seat's ordinate, up the page, in wheel radii.
    pub y: f64,
    /// The exact seat, twice the cell coordinates from the tile centre, before any jitter.
    pub seat: (i64, i64),
    /// What the pencil sits on.
    pub kind: Kind,
}

/// The mass of a byte grid taken as a wheel: how many pencils of each kind it seats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Seats {
    /// The pencils on filled cells.
    pub fills: usize,
    /// The pencils on empty cells.
    pub voids: usize,
    /// The pencils on corners.
    pub corners: usize,
}

/// Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.
pub fn pencils(
    types: &[u8],
    width: usize,
    height: usize,
    mode: &str,
    reach: f64,
    jitter: f64,
    seed: u32,
) -> Result<Vec<Pencil>> {
    if width == 0 || height == 0 || types.len() != width * height {
        return value_error("the wheel must be a grid of width by height bytes.");
    }
    if !(REACH.0..=REACH.1).contains(&reach) {
        return value_error(format!(
            "the reach must be between {} and {} wheel radii.",
            REACH.0, REACH.1
        ));
    }
    if !(0.0..=1.0).contains(&jitter) {
        return value_error("the jitter must be between 0 and 1 cells.");
    }
    if !["fill", "void", "both", "corners"].contains(&mode) {
        return value_error("the pencils sit on fill, void, both or corners.");
    }
    let (w, h) = (width as i64, height as i64);
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let on = types[i * width + j] != 0;
            let (i, j) = (i as i64, j as i64);
            if mode == "corners" {
                if !on {
                    continue;
                }
                for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                    let seat = (2 * (j + dj) - w, h - 2 * (i + di));
                    if seen.insert(seat) {
                        out.push(seat_pencil(seat, Kind::Corner));
                    }
                }
            } else if (on && mode != "void") || (!on && mode != "fill") {
                let kind = if on { Kind::Fill } else { Kind::Void };
                out.push(seat_pencil((2 * j + 1 - w, h - 2 * i - 1), kind));
            }
        }
    }
    if out.len() > PENCIL_CAP {
        return value_error(format!(
            "{} pencils is past the cap of {PENCIL_CAP}: take a lower level or a smaller tile.",
            out.len()
        ));
    }
    let unit = reach / (width as f64 / 2.0).hypot(height as f64 / 2.0);
    let mut rng = Rng::new(u64::from(seed));
    for pencil in &mut out {
        let (dx, dy) = if jitter > 0.0 {
            ((rng.unit() - 0.5) * jitter, (rng.unit() - 0.5) * jitter)
        } else {
            (0.0, 0.0)
        };
        pencil.x = (pencil.seat.0 as f64 / 2.0 + dx) * unit;
        pencil.y = (pencil.seat.1 as f64 / 2.0 + dy) * unit;
    }
    Ok(out)
}

/// The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.
pub fn cell(width: usize, height: usize, reach: f64) -> f64 {
    reach / (width as f64 / 2.0).hypot(height as f64 / 2.0)
}

/// Counts the pencils by kind.
pub fn seats(pencils: &[Pencil]) -> Seats {
    let mut out = Seats::default();
    for pencil in pencils {
        match pencil.kind {
            Kind::Fill => out.fills += 1,
            Kind::Void => out.voids += 1,
            Kind::Corner => out.corners += 1,
        }
    }
    out
}

fn seat_pencil(seat: (i64, i64), kind: Kind) -> Pencil {
    Pencil {
        x: 0.0,
        y: 0.0,
        seat,
        kind,
    }
}

// THE TRACK

/// One piece of the centre path: a straight run, or a turn about a point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// A straight run from one point to another.
    Run {
        /// Where the run starts.
        from: (f64, f64),
        /// Where the run ends.
        to: (f64, f64),
    },
    /// A turn about a point at a radius, counterclockwise from one angle to another.
    Turn {
        /// The point turned about.
        about: (f64, f64),
        /// The radius of the turn.
        radius: f64,
        /// The angle the turn starts at.
        from: f64,
        /// The angle the turn ends at, past the start.
        to: f64,
    },
}

impl Piece {
    fn len(&self) -> f64 {
        match *self {
            Piece::Run { from, to } => (to.0 - from.0).hypot(to.1 - from.1),
            Piece::Turn {
                radius, from, to, ..
            } => radius * (to - from),
        }
    }

    fn at(&self, s: f64) -> (f64, f64) {
        match *self {
            Piece::Run { from, to } => {
                let len = self.len();
                let t = if len > 0.0 {
                    (s / len).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                (from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t)
            }
            Piece::Turn {
                about,
                radius,
                from,
                to,
            } => {
                let angle = (from + s / radius).min(to);
                (
                    about.0 + radius * angle.cos(),
                    about.1 + radius * angle.sin(),
                )
            }
        }
    }

    fn bounds(&self) -> [f64; 4] {
        match *self {
            Piece::Run { from, to } => [
                from.0.min(to.0),
                from.1.min(to.1),
                from.0.max(to.0),
                from.1.max(to.1),
            ],
            Piece::Turn { about, radius, .. } => [
                about.0 - radius,
                about.1 - radius,
                about.0 + radius,
                about.1 + radius,
            ],
        }
    }
}

/// A track and the path the wheel's centre takes along it: the wheel of radius `wheel` rolls without slipping, on the left of the track when `side` is minus one and on the right when it is plus one, and turns by `side` times the centre's path length over the wheel's radius.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    /// The kind: `line`, `in`, `out`, `polyin` or `polyout`.
    pub kind: String,
    /// The wheel's radius.
    pub wheel: f64,
    /// The pieces of the centre path, in order.
    pub pieces: Vec<Piece>,
    /// The length of the centre path.
    pub total: f64,
    /// Minus one inside the track, plus one outside it.
    pub side: f64,
    /// The track itself as a polyline, closed when `closed` says so.
    pub outline: Vec<(f64, f64)>,
    /// Whether the outline closes on itself.
    pub closed: bool,
    /// The ring's radius over the wheel's in lowest terms on a circle, zero over zero elsewhere.
    pub ratio: (usize, usize),
    /// How many times the centre goes round: the ratio's denominator on a circle, the laps on a polygon, one on a line.
    pub orbits: usize,
    /// The rotation order of the whole picture: the ratio's numerator on a circle, none elsewhere.
    pub fold: usize,
}

/// Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.
pub fn track(kind: &str, ring: usize, wheel: usize, sides: usize, laps: usize) -> Result<Track> {
    if !(1..=RADIUS_CAP).contains(&wheel) || !(1..=RADIUS_CAP).contains(&ring) {
        return value_error(format!(
            "the ring and the wheel must have radius between 1 and {RADIUS_CAP}."
        ));
    }
    if !(1..=LAPS_CAP).contains(&laps) {
        return value_error(format!("the laps must be between 1 and {LAPS_CAP}."));
    }
    let r = wheel as f64;
    let big = ring as f64;
    let mut out = Track {
        kind: kind.to_string(),
        wheel: r,
        pieces: Vec::new(),
        total: 0.0,
        side: -1.0,
        outline: Vec::new(),
        closed: false,
        ratio: (0, 0),
        orbits: laps,
        fold: 0,
    };
    match kind {
        "line" => {
            let run = laps as f64 * TAU * r;
            out.pieces.push(Piece::Run {
                from: (0.0, r),
                to: (run, r),
            });
            out.outline = vec![(-r, 0.0), (run + r, 0.0)];
            out.orbits = 1;
        }
        "in" | "out" => {
            let inside = kind == "in";
            if inside && ring <= wheel {
                return value_error("the wheel must be smaller than the ring it rolls inside.");
            }
            let g = gcd(ring, wheel);
            let (a, b) = (ring / g, wheel / g);
            let rho = if inside { big - r } else { big + r };
            out.side = if inside { -1.0 } else { 1.0 };
            out.pieces.push(Piece::Turn {
                about: (0.0, 0.0),
                radius: rho,
                from: 0.0,
                to: TAU * b as f64,
            });
            out.outline = (0..RING)
                .map(|k| {
                    let angle = TAU * k as f64 / RING as f64;
                    (big * angle.cos(), big * angle.sin())
                })
                .collect();
            out.closed = true;
            out.ratio = (a, b);
            out.orbits = b;
            out.fold = a;
        }
        "polyin" | "polyout" => {
            if !(SIDES.0..=SIDES.1).contains(&sides) {
                return value_error(format!(
                    "the polygon must have between {} and {} sides.",
                    SIDES.0, SIDES.1
                ));
            }
            let corners = polygon(big, sides);
            out.outline = corners.clone();
            out.closed = true;
            if kind == "polyin" {
                let inset = big * (PI / sides as f64).cos() - r;
                if inset <= 0.0 {
                    return value_error("the wheel does not fit inside the polygon.");
                }
                let inner = polygon(inset / (PI / sides as f64).cos(), sides);
                for _ in 0..laps {
                    for k in 0..sides {
                        out.pieces.push(Piece::Run {
                            from: inner[k],
                            to: inner[(k + 1) % sides],
                        });
                    }
                }
            } else {
                out.side = 1.0;
                let normal = |k: usize| {
                    let (p, q) = (corners[k], corners[(k + 1) % sides]);
                    let (dx, dy) = (q.0 - p.0, q.1 - p.1);
                    let len = dx.hypot(dy);
                    (dy / len, -dx / len)
                };
                for _ in 0..laps {
                    for k in 0..sides {
                        let (p, q) = (corners[k], corners[(k + 1) % sides]);
                        let (m, next) = (normal(k), normal((k + 1) % sides));
                        out.pieces.push(Piece::Run {
                            from: (p.0 + r * m.0, p.1 + r * m.1),
                            to: (q.0 + r * m.0, q.1 + r * m.1),
                        });
                        let from = m.1.atan2(m.0);
                        let mut to = next.1.atan2(next.0);
                        while to < from {
                            to += TAU;
                        }
                        out.pieces.push(Piece::Turn {
                            about: q,
                            radius: r,
                            from,
                            to,
                        });
                    }
                }
            }
        }
        _ => return value_error("the track is line, in, out, polyin or polyout."),
    }
    out.total = out.pieces.iter().map(Piece::len).sum();
    Ok(out)
}

fn polygon(radius: f64, sides: usize) -> Vec<(f64, f64)> {
    (0..sides)
        .map(|k| {
            let angle = PI / sides as f64 + TAU * k as f64 / sides as f64;
            (radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}

/// The wheel's centre after `s` of path length.
pub fn pose(track: &Track, s: f64) -> (f64, f64) {
    let mut left = s.clamp(0.0, track.total);
    let last = track.pieces.len() - 1;
    for (k, piece) in track.pieces.iter().enumerate() {
        let len = piece.len();
        if left <= len || k == last {
            return piece.at(left);
        }
        left -= len;
    }
    (0.0, 0.0)
}

/// The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.
pub fn turn(track: &Track, s: f64) -> f64 {
    let angle = track.side * s / track.wheel;
    if angle == 0.0 {
        0.0
    } else {
        angle
    }
}

/// Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.
pub fn point(track: &Track, pencil: &Pencil, s: f64) -> (f64, f64) {
    let (cx, cy) = pose(track, s);
    let phi = turn(track, s);
    let (c, sn) = (phi.cos(), phi.sin());
    let r = track.wheel;
    (
        cx + r * (pencil.x * c - pencil.y * sn),
        cy + r * (pencil.x * sn + pencil.y * c),
    )
}

/// Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.
pub fn trace(track: &Track, pencils: &[Pencil], samples: usize) -> Result<Vec<f32>> {
    if samples < 2 {
        return value_error("a trace needs at least two samples.");
    }
    if pencils.len() * samples > POINT_CAP {
        return value_error(format!(
            "{} points is past the cap of {POINT_CAP}: fewer samples or fewer pencils.",
            pencils.len() * samples
        ));
    }
    let mut out = Vec::with_capacity(pencils.len() * samples * 2);
    for pencil in pencils {
        for k in 0..samples {
            let s = track.total * k as f64 / (samples - 1) as f64;
            let (x, y) = point(track, pencil, s);
            out.push(x as f32);
            out.push(y as f32);
        }
    }
    Ok(out)
}

/// How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.
pub fn distinct(track: &Track, pencils: &[Pencil], exact: bool) -> usize {
    representatives(track, pencils, exact).len()
}

/// One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.
pub fn representatives(track: &Track, pencils: &[Pencil], exact: bool) -> Vec<usize> {
    if !exact {
        return (0..pencils.len()).collect();
    }
    let mut out = Vec::new();
    match track.kind.as_str() {
        "line" => {
            let mut seen = HashSet::new();
            for (k, p) in pencils.iter().enumerate() {
                if seen.insert(p.seat.0 * p.seat.0 + p.seat.1 * p.seat.1) {
                    out.push(k);
                }
            }
        }
        "in" | "out" => {
            let order = gcd(track.ratio.1, 4);
            let quarter = |(u, v): (i64, i64)| (-v, u);
            let mut seen = HashSet::new();
            for (k, p) in pencils.iter().enumerate() {
                let mut least = p.seat;
                let mut seat = p.seat;
                for _ in 1..order {
                    for _ in 0..4 / order {
                        seat = quarter(seat);
                    }
                    least = least.min(seat);
                }
                if seen.insert(least) {
                    out.push(k);
                }
            }
        }
        _ => out.extend(0..pencils.len()),
    }
    out
}

/// The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.
pub fn frame(track: &Track, pencils: &[Pencil]) -> [f64; 4] {
    let reach = pencils
        .iter()
        .map(|p| p.x.hypot(p.y))
        .fold(1.0_f64, f64::max);
    let pad = track.wheel * reach;
    let mut box_ = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
    let mut take = |x: f64, y: f64| {
        box_[0] = box_[0].min(x);
        box_[1] = box_[1].min(y);
        box_[2] = box_[2].max(x);
        box_[3] = box_[3].max(y);
    };
    for piece in &track.pieces {
        let b = piece.bounds();
        take(b[0] - pad, b[1] - pad);
        take(b[2] + pad, b[3] + pad);
    }
    for &(x, y) in &track.outline {
        take(x, y);
    }
    box_
}

// THE NODES

/// The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.
pub fn nodes(track: &Track, pencils: &[Pencil], exact: bool) -> Option<u64> {
    if track.kind != "in" && track.kind != "out" {
        return None;
    }
    let (a, b) = (track.ratio.0 as u64, track.ratio.1 as u64);
    let window = (a as f64 / b as f64 + track.side).abs().min(1.0);
    let out = pencils.iter().any(|p| {
        let seat = p.x.hypot(p.y);
        !(seat > 0.0 && seat < window)
    });
    if out {
        return None;
    }
    let k = distinct(track, pencils, exact) as u64;
    Some(2 * a * b * (k * k.saturating_sub(1) / 2) + k * a * (b - 1))
}

// THE COVER

/// The largest raster side a cover rasters.
pub const RASTER_CAP: usize = 4096;

/// The disc a circle roulette sits in, in the track's units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Disc {
    /// The centre's abscissa in the track's units, the centre of the ring.
    pub x: f64,
    /// The centre's ordinate in the track's units.
    pub y: f64,
    /// The radius no curve leaves, in the track's units: `rho + d` with `rho` the centre circle's radius and `d = r abs(p)` the outermost seat, which is `(a - b)/b + abs(p)` wheel radii inside and `(a + b)/b + abs(p)` outside.
    pub radius: f64,
    /// The radius no curve enters, in the track's units: the least of `abs(rho - d)` over the seats, which is not `rho` less the outermost seat when the seats straddle `rho`, and `rho` itself when no pencil is seated.
    pub hole: f64,
}

/// The shape between the walls on a raster, with its numbers.
#[derive(Clone, Debug, PartialEq)]
pub struct Cover {
    /// The raster row by row, four codes: zero outside the disc, one the flood from the raster's edge, two the flood from the centre, three the shape, the walls and their pockets included.
    pub mask: Vec<u8>,
    /// The raster's side in pixels.
    pub side: usize,
    /// The shape's share of the disc's pixels, the walls counted in.
    pub covered: f64,
    /// The centre flood's share of the disc's pixels.
    pub hole: f64,
    /// The share of the disc's pixels the polylines themselves mark, the boundary error `covered` carries and loses as the raster grows.
    pub wall: f64,
    /// The mean signed winding number of the disc's pixel centres, read by scanline off the polylines.
    pub winding: f64,
    /// The closed form `winding` converges to: the distinct curves' signed areas summed and divided by the disc's area.
    pub areas: f64,
}

/// The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.
pub fn signed_area(track: &Track, pencil: &Pencil) -> Option<f64> {
    let (_, rho) = ring(track).ok()?;
    let r = track.wheel;
    let d = r * pencil.x.hypot(pencil.y);
    let b = track.ratio.1 as f64;
    Some(PI * b * rho * (rho + track.side * d * d / r))
}

/// The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.
pub fn disc(track: &Track, pencils: &[Pencil]) -> Result<Disc> {
    let ((x, y), rho) = ring(track)?;
    let reach = |p: &Pencil| track.wheel * p.x.hypot(p.y);
    let far = pencils.iter().map(reach).fold(0.0_f64, f64::max);
    let near = pencils
        .iter()
        .map(|p| (rho - reach(p)).abs())
        .fold(f64::MAX, f64::min);
    Ok(Disc {
        x,
        y,
        radius: rho + far,
        hole: if pencils.is_empty() { rho } else { near },
    })
}

/// The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.
pub fn cover(
    track: &Track,
    pencils: &[Pencil],
    exact: bool,
    samples: usize,
    side: usize,
) -> Result<Cover> {
    if !(16..=RASTER_CAP).contains(&side) {
        return value_error(format!(
            "the raster side must be between 16 and {RASTER_CAP}."
        ));
    }
    if samples < 2 {
        return value_error("a cover needs at least two samples.");
    }
    let (_, rho) = ring(track)?;
    let bounds = disc(track, pencils)?;
    let (n, r) = (side, track.wheel);
    let h = 2.0 * bounds.radius / n as f64;
    let left = bounds.x - bounds.radius;
    let top = bounds.y + bounds.radius;
    let drawn = representatives(track, pencils, exact);
    let mut steps = Vec::with_capacity(drawn.len());
    for &k in &drawn {
        let d = r * pencils[k].x.hypot(pencils[k].y);
        let arc = TAU * track.ratio.1 as f64 * rho * (1.0 + d / r);
        steps.push(samples.max((2.0 * arc / h).ceil().max(2.0) as usize));
    }
    let work: usize = steps.iter().sum();
    if work > POINT_CAP {
        return value_error(format!(
            "{work} samples is past the cap of {POINT_CAP}: a coarser raster or fewer curves."
        ));
    }
    let mut wall = vec![false; n * n];
    let mut rows: Vec<Vec<(f64, i32)>> = vec![Vec::new(); n];
    for (&k, &count) in drawn.iter().zip(&steps) {
        let mut last = point(track, &pencils[k], 0.0);
        mark(&mut wall, n, left, top, h, last);
        for step in 1..=count {
            let s = track.total * step as f64 / count as f64;
            let next = point(track, &pencils[k], s);
            mark(&mut wall, n, left, top, h, next);
            scan(&mut rows, n, top, h, last, next);
            last = next;
        }
    }
    let mut code = vec![0u8; n * n];
    let mut stack = Vec::new();
    for j in 0..n {
        for at in [j, (n - 1) * n + j, j * n, j * n + n - 1] {
            if !wall[at] && code[at] == 0 {
                code[at] = 1;
                stack.push(at);
            }
        }
    }
    flood(&mut code, &wall, n, &mut stack);
    let middle = (n / 2) * n + n / 2;
    if !wall[middle] && code[middle] == 0 {
        code[middle] = 2;
        stack.push(middle);
        flood(&mut code, &wall, n, &mut stack);
    }
    let (mut total, mut shape, mut hollow, mut edge, mut turns) =
        (0usize, 0usize, 0usize, 0usize, 0i64);
    for (row, line) in rows.iter_mut().enumerate() {
        let y = top - (row as f64 + 0.5) * h;
        line.sort_by(|a, b| b.0.total_cmp(&a.0));
        let (mut seen, mut winding) = (0usize, 0i64);
        for column in (0..n).rev() {
            let x = left + (column as f64 + 0.5) * h;
            while seen < line.len() && line[seen].0 > x {
                winding += i64::from(line[seen].1);
                seen += 1;
            }
            let at = row * n + column;
            if (x - bounds.x).hypot(y - bounds.y) > bounds.radius {
                code[at] = 0;
                continue;
            }
            total += 1;
            turns += winding;
            if wall[at] {
                edge += 1;
            }
            match code[at] {
                1 => {}
                2 => hollow += 1,
                _ => {
                    code[at] = 3;
                    shape += 1;
                }
            }
        }
    }
    let count = total.max(1) as f64;
    let sum: f64 = drawn
        .iter()
        .filter_map(|&k| signed_area(track, &pencils[k]))
        .sum();
    Ok(Cover {
        mask: code,
        side: n,
        covered: shape as f64 / count,
        hole: hollow as f64 / count,
        wall: edge as f64 / count,
        winding: turns as f64 / count,
        areas: sum / (PI * bounds.radius * bounds.radius),
    })
}

fn ring(track: &Track) -> Result<((f64, f64), f64)> {
    match track.pieces.first() {
        Some(&Piece::Turn { about, radius, .. }) if track.kind == "in" || track.kind == "out" => {
            Ok((about, radius))
        }
        _ => value_error("the cover needs a circle track: the wall of a line or a polygon roulette need not close."),
    }
}

fn mark(wall: &mut [bool], n: usize, left: f64, top: f64, h: f64, p: (f64, f64)) {
    let column = (((p.0 - left) / h) as isize).clamp(0, n as isize - 1) as usize;
    let row = (((top - p.1) / h) as isize).clamp(0, n as isize - 1) as usize;
    wall[row * n + column] = true;
}

fn scan(rows: &mut [Vec<(f64, i32)>], n: usize, top: f64, h: f64, p: (f64, f64), q: (f64, f64)) {
    if p.1 == q.1 {
        return;
    }
    let (sign, low, high) = if q.1 > p.1 {
        (1, p.1, q.1)
    } else {
        (-1, q.1, p.1)
    };
    let last = n as f64 - 1.0;
    let first = ((top - high) / h - 1.5).clamp(0.0, last) as usize;
    let stop = ((top - low) / h + 0.5).clamp(0.0, last) as usize;
    for (step, line) in rows[first..=stop].iter_mut().enumerate() {
        let y = top - ((first + step) as f64 + 0.5) * h;
        if y < low || y >= high {
            continue;
        }
        line.push((p.0 + (q.0 - p.0) * (y - p.1) / (q.1 - p.1), sign));
    }
}

fn flood(code: &mut [u8], wall: &[bool], n: usize, stack: &mut Vec<usize>) {
    while let Some(at) = stack.pop() {
        let tint = code[at];
        let (row, column) = (at / n, at % n);
        let step = |to: usize, code: &mut [u8], stack: &mut Vec<usize>| {
            if !wall[to] && code[to] == 0 {
                code[to] = tint;
                stack.push(to);
            }
        };
        if row > 0 {
            step(at - n, code, stack);
        }
        if row + 1 < n {
            step(at + n, code, stack);
        }
        if column > 0 {
            step(at - 1, code, stack);
        }
        if column + 1 < n {
            step(at + 1, code, stack);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ring() -> Vec<u8> {
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1]
    }

    #[test]
    fn the_carpet_seats_eight_pencils_and_the_law_folds_them_into_curves_and_shifts() {
        let fills = pencils(&ring(), 3, 3, "fill", 0.9, 0.0, 1).unwrap();
        assert_eq!(fills.len(), 8);
        assert_eq!(seats(&fills).fills, 8);
        assert_eq!(
            pencils(&ring(), 3, 3, "void", 0.9, 0.0, 1).unwrap()[0].seat,
            (0, 0)
        );
        assert_eq!(
            pencils(&ring(), 3, 3, "corners", 0.9, 0.0, 1)
                .unwrap()
                .len(),
            16
        );
        let curves = |kind: &str, ring: usize, wheel: usize| {
            distinct(&track(kind, ring, wheel, 4, 1).unwrap(), &fills, true)
        };
        assert_eq!(curves("in", 7, 4), 2);
        assert_eq!(curves("in", 7, 2), 4);
        assert_eq!(curves("in", 7, 3), 8);
        assert_eq!(curves("out", 5, 8), 2);
        assert_eq!(curves("in", 7, 6), 4);
        let shapes_up_to_a_shift = curves("line", 7, 3);
        assert_eq!(shapes_up_to_a_shift, 2);
        let jittered = pencils(&ring(), 3, 3, "fill", 0.9, 0.3, 1).unwrap();
        assert_eq!(
            distinct(&track("in", 7, 4, 4, 1).unwrap(), &jittered, false),
            8
        );
    }

    #[test]
    fn a_pencil_traces_the_textbook_hypotrochoid() {
        let path = track("in", 7, 3, 4, 1).unwrap();
        let pencil = Pencil {
            x: 0.5,
            y: 0.0,
            seat: (0, 0),
            kind: Kind::Fill,
        };
        let (big, r, d) = (7.0, 3.0, 1.5);
        for k in 0..12 {
            let theta = TAU * k as f64 / 7.0;
            let s = (big - r) * theta;
            let (x, y) = point(&path, &pencil, s);
            let want = (
                (big - r) * theta.cos() + d * ((big - r) / r * theta).cos(),
                (big - r) * theta.sin() - d * ((big - r) / r * theta).sin(),
            );
            assert!((x - want.0).abs() < 1e-9 && (y - want.1).abs() < 1e-9);
        }
        assert_eq!(path.ratio, (7, 3));
        assert_eq!((path.orbits, path.fold), (3, 7));
        assert!((path.total - TAU * 4.0 * 3.0).abs() < 1e-9);
    }

    #[test]
    fn the_circle_closes_and_the_line_and_polygon_run_on() {
        let fills = pencils(&ring(), 3, 3, "fill", 0.9, 0.0, 1).unwrap();
        let closed = trace(&track("in", 7, 3, 4, 1).unwrap(), &fills, 500).unwrap();
        assert!((closed[0] - closed[998]).abs() < 1e-5 && (closed[1] - closed[999]).abs() < 1e-5);
        let open = trace(&track("line", 7, 3, 4, 2).unwrap(), &fills, 500).unwrap();
        assert!((open[0] - open[998]).abs() > 1.0);
        let square = track("polyout", 7, 3, 4, 1).unwrap();
        assert_eq!(square.pieces.len(), 8);
        assert!((square.total - (8.0 * 7.0 / 2f64.sqrt() + TAU * 3.0)).abs() < 1e-9);
        let inner = track("polyin", 7, 3, 4, 2).unwrap();
        let (x0, y0) = pose(&inner, 0.0);
        let (x1, y1) = pose(&inner, inner.total / 2.0);
        assert!((x0 - x1).abs() < 1e-9 && (y0 - y1).abs() < 1e-9);
    }

    #[test]
    fn the_nodes_count_only_inside_the_seat_window() {
        let fills = pencils(&ring(), 3, 3, "fill", 0.9, 0.0, 1).unwrap();
        let seven = track("in", 7, 3, 4, 1).unwrap();
        assert_eq!(distinct(&seven, &fills, true), 8);
        assert_eq!(nodes(&seven, &fills, true), Some(1288));
        let four = track("in", 7, 4, 4, 1).unwrap();
        assert_eq!(nodes(&four, &fills, true), Some(98));
        let past = pencils(&ring(), 3, 3, "fill", 1.35, 0.0, 1).unwrap();
        assert_eq!(nodes(&four, &past, true), None);
        assert_eq!(nodes(&track("in", 7, 6, 4, 1).unwrap(), &fills, true), None);
        assert_eq!(
            nodes(&track("out", 5, 8, 4, 1).unwrap(), &fills, true),
            Some(150)
        );
        assert_eq!(
            nodes(&track("line", 7, 3, 4, 1).unwrap(), &fills, true),
            None
        );
        let far = pencils(&ring(), 3, 3, "corners", 1.2, 0.0, 1).unwrap();
        assert_eq!(nodes(&seven, &far, true), None);
        let centre = pencils(&ring(), 3, 3, "void", 0.9, 0.0, 1).unwrap();
        assert_eq!(centre[0].seat, (0, 0));
        assert_eq!(nodes(&seven, &centre, true), None);
        let empty = pencils(&[1u8; 9], 3, 3, "void", 0.9, 0.0, 1).unwrap();
        assert_eq!(nodes(&seven, &empty, true), Some(0));
    }

    #[test]
    fn the_simple_curve_holds_its_whole_inside_in_the_hole() {
        let d = 0.9;
        for (kind, rho) in [("in", 2.0), ("out", 4.0)] {
            let path = track(kind, 3, 1, 4, 1).unwrap();
            let pencil = Pencil {
                x: d,
                y: 0.0,
                seat: (1, 0),
                kind: Kind::Fill,
            };
            let bounds = disc(&path, &[pencil]).unwrap();
            assert!((bounds.radius - (rho + d)).abs() < 1e-12);
            assert!((bounds.hole - (rho - d)).abs() < 1e-12);
            let form = rho * (rho + path.side * d * d) / (rho + d).powi(2);
            let side = 512;
            let slack = 2.0 * TAU * rho * (1.0 + d) * (2.0 * (rho + d) / side as f64)
                / (PI * (rho + d).powi(2));
            let cover = cover(&path, &[pencil], true, 2, side).unwrap();
            assert_eq!(cover.mask.len(), side * side);
            assert!((cover.areas - form).abs() < 1e-12);
            assert!(cover.covered < slack);
            assert!((cover.hole - form).abs() < slack);
            assert!((cover.winding - form).abs() < slack);
        }
    }

    #[test]
    fn the_disc_reads_the_innermost_curve_and_not_the_farthest_seat() {
        let corners = pencils(&ring(), 3, 3, "corners", 1.2, 0.0, 1).unwrap();
        let straddle = track("in", 3, 2, 4, 1).unwrap();
        let bounds = disc(&straddle, &corners).unwrap();
        assert_eq!(
            format!("{:.6} {:.6}", bounds.radius, bounds.hole),
            "3.400000 0.200000"
        );
        let inscribed = (bounds.hole / bounds.radius).powi(2);
        let cover = cover(&straddle, &corners, true, 2, 512).unwrap();
        assert!(cover.hole >= inscribed && cover.hole < 2.0 * inscribed);
        let out = track("in", 5, 4, 4, 1).unwrap();
        let far = Pencil {
            x: 0.9,
            y: 0.0,
            seat: (1, 0),
            kind: Kind::Fill,
        };
        let past = disc(&out, &[far]).unwrap();
        assert_eq!(
            format!("{:.6} {:.6}", past.radius, past.hole),
            "4.600000 2.600000"
        );
        let empty = disc(&straddle, &[]).unwrap();
        assert_eq!(
            format!("{:.6} {:.6}", empty.radius, empty.hole),
            "1.000000 1.000000"
        );
    }

    #[test]
    fn the_mean_winding_is_the_signed_areas_over_the_disc() {
        let fills = pencils(&ring(), 3, 3, "fill", 0.9, 0.0, 1).unwrap();
        let seven = track("in", 7, 3, 4, 1).unwrap();
        let drawn = representatives(&seven, &fills, true);
        assert_eq!(drawn.len(), 8);
        let bounds = disc(&seven, &fills).unwrap();
        let area = PI * bounds.radius * bounds.radius;
        let want: f64 = drawn
            .iter()
            .map(|&k| signed_area(&seven, &fills[k]).unwrap())
            .sum::<f64>()
            / area;
        let side = 384;
        let arcs: f64 = drawn
            .iter()
            .map(|&k| TAU * 3.0 * 4.0 * (1.0 + fills[k].x.hypot(fills[k].y)))
            .sum();
        let slack = 2.0 * arcs * (2.0 * bounds.radius / side as f64) / area;
        let cover = cover(&seven, &fills, true, 2, side).unwrap();
        assert!((cover.areas - want).abs() < 1e-12);
        assert!((cover.winding - want).abs() < slack);
        assert!(cover.covered > 0.0 && cover.hole > 0.0);
        assert!((cover.covered + cover.hole) < 1.0);
    }

    #[test]
    fn the_cover_refuses_what_has_no_wall() {
        let fills = pencils(&ring(), 3, 3, "fill", 0.9, 0.0, 1).unwrap();
        let straight = track("line", 7, 3, 4, 1).unwrap();
        assert!(disc(&straight, &fills).is_err());
        assert!(signed_area(&straight, &fills[0]).is_none());
        assert!(cover(&track("polyin", 7, 3, 4, 1).unwrap(), &fills, true, 2, 64).is_err());
        let seven = track("in", 7, 3, 4, 1).unwrap();
        assert!(cover(&seven, &fills, true, 2, 8).is_err());
        assert!(cover(&seven, &fills, true, 1, 64).is_err());
    }

    #[test]
    fn a_wheel_too_big_for_its_track_is_refused() {
        assert!(track("in", 3, 3, 4, 1).is_err());
        assert!(track("polyin", 4, 3, 3, 1).is_err());
        assert!(track("orbit", 7, 3, 4, 1).is_err());
        assert!(pencils(&ring(), 3, 3, "edges", 0.9, 0.0, 1).is_err());
        assert!(pencils(&ring(), 3, 2, "fill", 0.9, 0.0, 1).is_err());
    }
}
