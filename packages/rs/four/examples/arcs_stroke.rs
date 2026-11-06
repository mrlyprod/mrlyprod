// How thick can an arc stroke be before tiles break? Past max_stroke, a tile's own two arcs overlap.
// Columns: edge points tested, points where neighbours disagree on ink, points where a tile's arcs overlap.
// cargo run -p mrly-four --example arcs_stroke
use mrly_four::arcs::{corners, inked, max_stroke, on_ring, svg};
use mrly_four::{E, HALVES, N, S, from_strings};

const SCALE: usize = 32;

// All the maths is integer. Work in doubled pixels, so a pixel centre is 2p + 1, a tile is 2s wide,
// an arc has radius s, and half a stroke is w. A point is on a stroke when (s - w)^2 <= distance^2 <= (s + w)^2.
// The largest stroke that keeps both arcs apart is isqrt(2 s^2) - s.

// Every doubled point along the shared edge of two neighbouring tiles: both must agree on ink.
// All 16 pairs of half tiles, side by side and one above the other.
fn edge_gaps(stroke: usize) -> (usize, usize) {
    let far = 2 * SCALE as i64;
    let (mut tested, mut gaps) = (0, 0);
    for here in HALVES {
        for there in HALVES {
            for t in 0..=far {
                let across =
                    inked(here, (far, t), SCALE, stroke) != inked(there, (0, t), SCALE, stroke);
                let down =
                    inked(here, (t, far), SCALE, stroke) != inked(there, (t, 0), SCALE, stroke);
                gaps += usize::from(across) + usize::from(down);
                tested += 2;
            }
        }
    }
    (tested, gaps)
}

fn overlaps(stroke: usize) -> usize {
    let far = 2 * SCALE as i64;
    let mut count = 0;
    for tag in [N | E, E | S] {
        let Some(pair) = corners(tag, SCALE) else {
            continue;
        };
        for y in 0..=far {
            for x in 0..=far {
                let on = |corner| on_ring((x, y), corner, SCALE, stroke);
                count += usize::from(on(pair[0]) && on(pair[1]));
            }
        }
    }
    count
}

fn main() -> Result<(), String> {
    let peak = max_stroke(SCALE);
    println!("scale {SCALE}: largest stroke = isqrt(2*{SCALE}^2) - {SCALE} = {peak}");
    println!("stroke  tested  gaps  overlap");
    for stroke in [1, 4, peak / 2, peak - 1, peak, peak + 1] {
        let (tested, gaps) = edge_gaps(stroke);
        println!(
            "{stroke:>6}  {tested:>6}  {gaps:>4}  {:>7}",
            overlaps(stroke)
        );
    }
    let tile = from_strings(&["6c", "c6"])?;
    println!(
        "svg at stroke {}: {}",
        peak + 1,
        svg(&tile, SCALE, peak + 1, ["#fff", "#000"], true).unwrap_err()
    );
    Ok(())
}
