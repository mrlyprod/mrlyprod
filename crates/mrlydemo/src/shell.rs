use crate::{code_of, rgba, theme, Fault, Pixels};
use mrlycore::json;
use mrlycore::tensor::Tensor;
use mrlymath::bang::factory;
use mrlymath::shape::{crossing_tree, Shell};
use wasm_bindgen::prelude::*;

const RADIUS_CAP: u32 = 242;
const SHEET: usize = 486;
const EDGE: usize = 2;
const MARK: usize = 5;

fn guarded(code: &str, number: usize, base: usize, radius: u32) -> Result<(Shell, usize), Fault> {
    if number < 2 {
        return Err(Fault::new("the side number must be at least 2."));
    }
    if !(1..=RADIUS_CAP).contains(&radius) {
        return Err(Fault::new(format!(
            "the radius must be between 1 and {RADIUS_CAP} cells."
        )));
    }
    let tile = factory::create(code_of(code)?, number, 2, base, 1)?;
    let keep: Vec<bool> = tile.bytes().iter().map(|&b| b != 0).collect();
    let tree = crossing_tree(radius as u64, number as u64, &keep);
    let depth = tree.levels.len() - 1;
    Ok((tree, depth))
}

fn body(code: &str, number: usize, base: usize, depth: usize) -> Result<Tensor, Fault> {
    Ok(factory::create(code_of(code)?, number, 2, base, depth)?)
}

fn seated(
    tree: &Shell,
    depth: usize,
    level: Option<u32>,
    index: Option<u32>,
) -> Result<(usize, usize), Fault> {
    let root = level.map_or(depth, |j| j as usize);
    if root > depth {
        return Err(Fault::new(format!(
            "level {root} lies above the tree's depth {depth}."
        )));
    }
    let seat = index.unwrap_or(0) as usize;
    if seat >= tree.levels[root].len() {
        return Err(Fault::new(format!(
            "level {root} holds {} boxes, so there is no box {seat}.",
            tree.levels[root].len()
        )));
    }
    Ok((root, seat))
}

fn spans(tree: &Shell, root: usize, seat: usize) -> Vec<(usize, usize)> {
    let mut out = vec![(0usize, 0usize); root + 1];
    out[root] = (seat, seat + 1);
    for level in (0..root).rev() {
        let (lo, hi) = out[level + 1];
        let row = &tree.levels[level];
        let held = |k: usize| row[k].parent >= lo && row[k].parent < hi;
        let start = (0..row.len()).find(|&k| held(k)).unwrap_or(row.len());
        let mut stop = start;
        while stop < row.len() && held(stop) {
            stop += 1;
        }
        out[level] = (start, stop);
    }
    out
}

/// Reads the crossing shell of one radius as a rooted tree: its depth, its leaves and one row per level.
///
/// Every row carries the boxes the circle crosses at that level, the count `2 * floor(radius / number^level) + 1` the identity asks for, the live boxes whose path never takes a seat the design drops, and the mean number of children a box of the level holds. `exact` is true when every level meets its count and no box lost its parent, so one flag says whether the drawing and the mathematics agree.
#[wasm_bindgen]
pub fn shell_read(code: &str, number: usize, base: usize, radius: u32) -> Result<String, Fault> {
    let (tree, depth) = guarded(code, number, base, radius)?;
    let scale = |level: usize| (number as u64).pow(level as u32);
    let mut rows = Vec::with_capacity(depth + 1);
    let mut exact = tree.orphans == 0;
    for level in 0..=depth {
        let boxes = tree.levels[level].len() as u64;
        let want = 2 * (radius as u64 / scale(level)) + 1;
        exact = exact && boxes == want;
        let live = tree.levels[level].iter().filter(|cell| cell.live).count();
        let below = if level == 0 {
            0.0
        } else {
            tree.levels[level - 1].len() as f64 / boxes as f64
        };
        rows.push(json!({
            "level": level,
            "boxes": boxes,
            "want": want,
            "live": live,
            "branch": below,
            "three": level > 0 && (radius as u64 / scale(level - 1)) % 3 == 1,
        }));
    }
    Ok(json!({
        "radius": radius,
        "number": number,
        "depth": depth,
        "side": scale(depth),
        "leaves": tree.levels[0].len(),
        "live": tree.levels[0].iter().filter(|cell| cell.live).count(),
        "orphans": tree.orphans,
        "exact": exact,
        "levels": rows,
    })
    .to_string())
}

/// Lays the crossing tree out flat for drawing: the level, both coordinates, the parent's place in the level above and the live flag, five numbers a box, the crossed cells first and the root last.
///
/// `level` and `index` name the box the walk is rooted at, and without them the walk is the whole tree rooted at its own single top box. A box's children are contiguous among its level, so a subtree is one range a level and the parent's place is counted from the start of that range, which lets a page draw one branch of a wide tree with every count in it still exact.
///
/// A box that lost its parent carries the largest number the type holds in that place, which the shell identity forbids and `shell_read` counts.
#[wasm_bindgen]
pub fn shell_nodes(
    code: &str,
    number: usize,
    base: usize,
    radius: u32,
    level: Option<u32>,
    index: Option<u32>,
) -> Result<Vec<u32>, Fault> {
    let (tree, depth) = guarded(code, number, base, radius)?;
    let (root, seat) = seated(&tree, depth, level, index)?;
    let spans = spans(&tree, root, seat);
    let mut out = Vec::new();
    for level in 0..=root {
        let (lo, hi) = spans[level];
        for k in lo..hi {
            let cell = tree.levels[level][k];
            let parent = if level == root {
                usize::MAX
            } else {
                cell.parent.wrapping_sub(spans[level + 1].0)
            };
            out.push(level as u32);
            out.push(cell.x as u32);
            out.push(cell.y as u32);
            out.push(parent.min(u32::MAX as usize) as u32);
            out.push(u32::from(cell.live));
        }
    }
    Ok(out)
}

/// Paints the design at the tree's own depth with the crossed cells lit, the pruned ones in their own ink and one level's boxes outlined.
///
/// The grid is the design at the least level that holds the circle, so one cell is one leaf of the tree and the circle's own corner sits at the bottom left: the design's other cells are the faint ground, a crossed cell the design keeps is gold, a crossed cell it drops is blue, and the boxes of level `at` are outlined under the crossed cells so the `2 * floor(radius / number^at) + 1` of them can be counted on the picture. The sheet is the same width at every depth, a whole number of pixels to the cell, so the outline stays a hairline however deep the tree runs.
///
/// `root` and `pick` name one box to ring in its own ink over everything else, so a page drawing one branch of the tree can show which box of the circle that branch is.
#[wasm_bindgen]
pub fn shell_pixels(
    code: &str,
    number: usize,
    base: usize,
    radius: u32,
    at: u32,
    root: Option<u32>,
    pick: Option<u32>,
) -> Result<Pixels, Fault> {
    let (tree, depth) = guarded(code, number, base, radius)?;
    if at as usize > depth {
        return Err(Fault::new(format!(
            "level {at} lies above the tree's depth {depth}."
        )));
    }
    let ringed = match (root, pick) {
        (Some(level), Some(index)) => Some(seated(&tree, depth, Some(level), Some(index))?),
        _ => None,
    };
    let grid = body(code, number, base, depth)?;
    let side = grid.shape[0];
    let scale = (SHEET / side).max(1);
    let wide = side * scale;
    let ink = theme();
    let mut colors = vec![rgba(ink.ground); wide * wide];
    let mut block = |x0: usize, y0: usize, x1: usize, y1: usize, color: [u8; 4]| {
        for row in (wide - x1.min(wide))..(wide - x0.min(wide)) {
            for column in y0.min(wide)..y1.min(wide) {
                colors[row * wide + column] = color;
            }
        }
    };
    for x in 0..side {
        for y in 0..side {
            if grid.bytes()[x * side + y] != 0 {
                block(
                    x * scale,
                    y * scale,
                    (x + 1) * scale,
                    (y + 1) * scale,
                    rgba(ink.line),
                );
            }
        }
    }
    let step = number.pow(at) * scale;
    for cell in &tree.levels[at as usize] {
        let (x0, y0) = (cell.x as usize * step, cell.y as usize * step);
        let (x1, y1) = (x0 + step, y0 + step);
        let pink = rgba(ink.pink);
        block(x0, y0, x1, y0 + EDGE, pink);
        block(x0, y1 - EDGE, x1, y1, pink);
        block(x0, y0, x0 + EDGE, y1, pink);
        block(x1 - EDGE, y0, x1, y1, pink);
    }
    for cell in &tree.levels[0] {
        let (x0, y0) = (cell.x as usize * scale, cell.y as usize * scale);
        let color = rgba(if cell.live { ink.yellow } else { ink.blue });
        block(x0, y0, x0 + scale, y0 + scale, color);
    }
    if let Some((level, seat)) = ringed {
        let cell = tree.levels[level][seat];
        let span = number.pow(level as u32) * scale;
        let (x0, y0) = (cell.x as usize * span, cell.y as usize * span);
        let (x1, y1) = (x0 + span, y0 + span);
        let green = rgba(ink.green);
        block(x0, y0, x1, y0 + MARK, green);
        block(x0, y1.saturating_sub(MARK), x1, y1, green);
        block(x0, y0, x0 + MARK, y1, green);
        block(x1.saturating_sub(MARK), y0, x1, y1, green);
    }
    Ok(Pixels::of(wide, wide, colors))
}
