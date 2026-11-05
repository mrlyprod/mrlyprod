// Smith's arcs: the same half-square tags, drawn as quarter circles.
//
// A half tile joins its edge midpoints in two pairs, with quarter circles around two
// opposite corners: c and 3 around top-right and bottom-left, 6 and 9 around top-left
// and bottom-right. Neighbours meet at a midpoint with one shared tangent, so arcs
// stitch into smooth strands: open paths, closed loops, and true circles.
//
// All the maths is integer. Work in doubled pixels, so a pixel centre is 2p + 1, a tile
// is 2s wide, an arc has radius s, and half a stroke is w. A point is on a stroke when
// (s - w)^2 <= distance^2 <= (s + w)^2.
//
// Usage: cargo run -p mrly-four --example arcs -- [stroke]
use mrly_four::arcs::{Strand, corners, inked, max_stroke, on_ring, raster, strands, svg};
use mrly_four::{Cell4d, E, HALVES, N, Point, S, W, from_strings, from_tags, random};
use mrly_math::arcs::{curves, draw, trace};
use mrly_math::bang::Code;
use std::env;
use std::error::Error;
use std::fs;

const DATA_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../data/packages/rs/four/examples"
);
const SEED: u64 = 1987;
const SCALE: usize = 64;
const DEFAULT_STROKE: usize = 16;
const COLORS: [&str; 2] = ["#fff6e5", "#1f2a44"];

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

// Turn the one bit per cell used by mrly-math into tags, mixing both tags of each pair.
fn layout(side: usize, on: &[bool]) -> Result<Cell4d, String> {
    let tags: Vec<u8> = on
        .iter()
        .enumerate()
        .map(|(flat, &bit)| match (bit, flat % 2 == 0) {
            (true, true) => E | S,
            (true, false) => W | N,
            (false, true) => N | E,
            (false, false) => S | W,
        })
        .collect();
    from_tags(side, side, &tags)
}

fn counts(found: &[Strand]) -> (u64, u64) {
    let loops = found.iter().filter(|strand| strand.closed).count() as u64;
    (loops, found.len() as u64 - loops)
}

fn lengths(found: impl Iterator<Item = (usize, bool)>) -> Vec<(usize, bool)> {
    let mut out: Vec<(usize, bool)> = found.collect();
    out.sort_unstable();
    out
}

// Compare strand and loop counts with mrly-math, on random layouts and on bang designs.
fn compare_with_math(state: &mut u64) -> Result<(), Box<dyn Error>> {
    let (mut layouts, mut wrong, mut loops, mut open) = (0, 0, 0, 0);
    for side in [1, 2, 3, 5, 8, 13, 21, 34, 55] {
        for _ in 0..8 {
            let on: Vec<bool> = (0..side * side)
                .map(|_| splitmix(state).is_multiple_of(2))
                .collect();
            let theirs = trace(side, &on)?;
            let ours = counts(&strands(&layout(side, &on)?, false));
            wrong += usize::from(ours != (theirs.loops, theirs.strands));
            (loops, open) = (loops + ours.0, open + ours.1);
            layouts += 1;
        }
    }
    let designs = (0..16u64)
        .map(|code| (code, 2, 5))
        .chain((0..512u64).step_by(61).map(|code| (code, 3, 3)));
    for (code, base, level) in designs {
        let code = Code::from(code);
        let theirs = draw(code, base, level)?;
        let on: Vec<bool> = theirs.cells.iter().map(|byte| byte & 1 == 1).collect();
        let found = strands(&layout(theirs.side, &on)?, false);
        let ours = counts(&found);
        let same_counts = ours == (theirs.loops, theirs.strands);
        let their_curves = curves(code, base, level)?;
        let same_lengths = lengths(found.iter().map(|s| (s.arcs.len(), s.closed)))
            == lengths(their_curves.iter().map(|c| (c.arcs.len(), c.closed)));
        wrong += usize::from(!(same_counts && same_lengths));
        (loops, open) = (loops + ours.0, open + ours.1);
        layouts += 1;
    }
    println!("mrly-math: {layouts} layouts, {loops} loops, {open} open strands, {wrong} disagree");
    Ok(())
}

// Every tile edge, every doubled point on it: both tiles must ink the same points.
// Returns the points tested, the disagreements, and the least and most ink on one edge.
fn edge_gaps(cell: &Cell4d, stroke: usize) -> (usize, usize, usize, usize) {
    let far = 2 * SCALE as i64;
    let (mut tested, mut gaps, mut least, mut most) = (0, 0, usize::MAX, 0);
    let mut check = |here: u8, there: u8, vertical: bool| {
        let mut ink = 0;
        for t in 0..=far {
            let (a, b) = if vertical {
                ((far, t), (0, t))
            } else {
                ((t, far), (t, 0))
            };
            let (left, right) = (
                inked(here, a, SCALE, stroke),
                inked(there, b, SCALE, stroke),
            );
            gaps += usize::from(left != right);
            ink += usize::from(left);
            tested += 1;
        }
        least = least.min(ink);
        most = most.max(ink);
    };
    for y in 0..cell.height {
        for x in 0..cell.width {
            if x + 1 < cell.width {
                check(cell.tag(x, y), cell.tag(x + 1, y), true);
            }
            if y + 1 < cell.height {
                check(cell.tag(x, y), cell.tag(x, y + 1), false);
            }
        }
    }
    (tested, gaps, least, most)
}

// A stroke stays inside its tile, so overlap can only happen between a tile's own two arcs.
fn overlaps(stroke: usize) -> usize {
    let far = 2 * SCALE as i64;
    let mut count = 0;
    for tag in [N | E, E | S] {
        let Some(pair) = corners(tag, SCALE) else {
            continue;
        };
        for y in 0..=far {
            for x in 0..=far {
                let on = |corner: Point| on_ring((x, y), corner, SCALE, stroke);
                count += usize::from(on(pair[0]) && on(pair[1]));
            }
        }
    }
    count
}

// Paint the rings from the circle equation alone, then compare with the tile-by-tile raster.
fn ring_errors(
    tile: &Cell4d,
    rings: &[Strand],
    stroke: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    let (width, height) = (tile.width * SCALE, tile.height * SCALE);
    let mut painted = vec![0u8; width * height];
    let (period_x, period_y) = (2 * width as i64, 2 * height as i64);
    let reach = (SCALE + stroke) as i64 + 1;
    for center in rings.iter().filter_map(Strand::ring) {
        for across in -1..=1 {
            for down in -1..=1 {
                let c = (
                    center.0 * SCALE as i64 + across * period_x,
                    center.1 * SCALE as i64 + down * period_y,
                );
                let xs = ((c.0 - reach) / 2).max(0)..((c.0 + reach) / 2 + 1).min(width as i64);
                let ys = ((c.1 - reach) / 2).max(0)..((c.1 + reach) / 2 + 1).min(height as i64);
                for y in ys {
                    for x in xs.clone() {
                        if on_ring((2 * x + 1, 2 * y + 1), c, SCALE, stroke) {
                            painted[y as usize * width + x as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    let tiled = raster(tile, SCALE, stroke)?;
    let wrong = painted
        .iter()
        .zip(&tiled)
        .filter(|&(&count, &ink)| (count > 0) != (ink > 0))
        .count();
    let twice = painted.iter().filter(|&&count| count > 1).count();
    Ok((wrong, twice))
}

fn main() -> Result<(), Box<dyn Error>> {
    let stroke = match env::args().nth(1) {
        Some(text) => text.parse()?,
        None => DEFAULT_STROKE,
    };
    let peak = max_stroke(SCALE);
    let mut state = SEED;
    compare_with_math(&mut state)?;

    let tangle = random(16, 16, &HALVES, |n| {
        (splitmix(&mut state) % n as u64) as usize
    })?;
    let tile = from_strings(&["6c", "c6"])?.tile(6, 6);
    let rings = strands(&tile, true);
    let round = rings
        .iter()
        .filter(|strand| strand.ring().is_some())
        .count();
    println!(
        "all-rings on the torus: {} strands, {round} rings",
        rings.len()
    );
    let (loops, open) = counts(&strands(&tangle, false));
    println!("random tiling: {loops} closed loops, {open} open strands");

    println!("scale {SCALE}: largest stroke = isqrt(2*{SCALE}^2) - {SCALE} = {peak}");
    println!(
        "stroke  edge points  gaps  ink per edge  self-overlap  ring-equation wrong  painted twice"
    );
    let mut widths = vec![1, 2, peak / 2, peak - 1, peak, stroke];
    widths.sort_unstable();
    widths.dedup();
    for width in widths {
        let (tested, gaps, least, most) = edge_gaps(&tangle, width);
        let (wrong, twice) = ring_errors(&tile, &rings, width)?;
        println!(
            "{width:>6}  {tested:>11}  {gaps:>4}  {least:>5}..{most:<5}  {:>12}  {wrong:>19}  {twice:>13}",
            overlaps(width)
        );
    }
    println!(
        "stroke {} is one too many: {} doubled points overlap, and svg says: {}",
        peak + 1,
        overlaps(peak + 1),
        svg(&tile, SCALE, peak + 1, COLORS, true).unwrap_err()
    );

    fs::create_dir_all(DATA_DIR)?;
    let random_svg = svg(&tangle, SCALE, stroke, COLORS, false)?;
    let rings_svg = svg(&tile, SCALE, stroke, COLORS, true)?;
    for (name, text) in [("arcs_random", &random_svg), ("arcs_rings", &rings_svg)] {
        fs::write(format!("{DATA_DIR}/{name}.svg"), text)?;
        println!(
            "{name}: stroke {stroke}, {} paths ending in Z, {} other paths, {} circles, {} arc commands",
            text.matches("Z\"").count(),
            text.matches("<path").count() - text.matches("Z\"").count(),
            text.matches("<circle").count(),
            text.matches('A').count(),
        );
    }
    Ok(())
}
