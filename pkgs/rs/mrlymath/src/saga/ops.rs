use super::{check, COLORS, SIDE};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::{Dtype, Tensor};

const RULE_MAX: usize = 8;

/// One deterministic grid operation, total over legal grids and erring on illegal params.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    /// Rotates the grid k quarter turns counter-clockwise, k in 1..=3.
    Rotate {
        /// The quarter turns taken.
        k: usize,
    },
    /// Reflects the grid: h mirrors left and right, v mirrors top and bottom.
    Reflect {
        /// True for the top-to-bottom mirror, false for left-to-right.
        vertical: bool,
    },
    /// Swaps the grid's rows and columns.
    Transpose,
    /// Keeps only the box at column x, row y, spanning w by h cells.
    Crop {
        /// The box's left column.
        x: usize,
        /// The box's top row.
        y: usize,
        /// The box's width.
        w: usize,
        /// The box's height.
        h: usize,
    },
    /// Keeps only the bounding box of the nonzero cells.
    Autocrop,
    /// Wraps the grid in a count-thick border of one pen.
    Pad {
        /// The border thickness.
        count: usize,
        /// The border pen.
        color: u8,
    },
    /// Repeats the grid across and down.
    Tile {
        /// The copies across.
        across: usize,
        /// The copies down.
        down: usize,
    },
    /// Repeats every cell into a k by k block.
    Scale {
        /// The block side.
        k: usize,
    },
    /// Rewrites every pen through a ten-entry map.
    Recolor {
        /// The pen each pen becomes.
        map: [u8; 10],
    },
    /// Shifts the cells by dx and dy, filling vacated cells with one pen.
    Translate {
        /// The shift in columns, positive rightward.
        dx: i64,
        /// The shift in rows, positive downward.
        dy: i64,
        /// The pen behind the shift.
        fill: u8,
    },
    /// Sets one cell to a pen.
    Paint {
        /// The cell's column.
        x: usize,
        /// The cell's row.
        y: usize,
        /// The pen painted.
        color: u8,
    },
    /// Recolors the four-connected region under one cell to a pen.
    Flood {
        /// The seed cell's column.
        x: usize,
        /// The seed cell's row.
        y: usize,
        /// The pen flooded in.
        color: u8,
    },
    /// Runs one life step for a single pen over the Moore neighborhood:
    /// that pen's cells live and die by the counts, empty cells birth,
    /// and every other pen stands as scenery counted dead.
    Step {
        /// The pen that lives, 1..=9.
        color: u8,
        /// The neighbor counts that birth a cell.
        birth: Vec<usize>,
        /// The neighbor counts that keep a cell.
        survive: Vec<usize>,
        /// True to wrap the edges into a torus.
        wrap: bool,
    },
    /// Replaces the whole grid with a literal one, the constant floor's op.
    Stamp {
        /// The literal grid's width.
        w: usize,
        /// The literal grid's cells, row by row.
        cells: Vec<u8>,
    },
}

impl Op {
    /// Applies the op to a grid, or an error for an illegal grid or params.
    pub fn apply(&self, grid: &Tensor) -> Result<Tensor> {
        check(grid)?;
        let (gh, gw) = (grid.shape[0], grid.shape[1]);
        match self {
            Op::Rotate { k } => {
                if !(1..=3).contains(k) {
                    return value_error("a rotation takes 1..=3 quarter turns.");
                }
                Ok(grid.rot90(*k, (0, 1)))
            }
            Op::Reflect { vertical } => Ok(grid.flip(if *vertical { 0 } else { 1 })),
            Op::Transpose => Ok(grid.transpose(0, 1)),
            Op::Crop { x, y, w, h } => {
                if *w == 0 || *h == 0 || x + w > gw || y + h > gh {
                    return value_error("the crop box leaves the grid.");
                }
                let mut next = Tensor::new(vec![*h, *w]);
                for r in 0..*h {
                    for c in 0..*w {
                        next.set(&[r, c], grid.get(&[y + r, x + c]));
                    }
                }
                Ok(next)
            }
            Op::Autocrop => match bounds(grid) {
                Some((x, y, w, h)) => Op::Crop { x, y, w, h }.apply(grid),
                None => value_error("an empty grid has nothing to crop."),
            },
            Op::Pad { count, color } => {
                if *count == 0 {
                    return value_error("a pad of zero adds nothing.");
                }
                pen(*color)?;
                if gw + 2 * count > SIDE || gh + 2 * count > SIDE {
                    return value_error("the pad leaves the board.");
                }
                Ok(grid.pad(*count, *color))
            }
            Op::Tile { across, down } => {
                if *across == 0 || *down == 0 {
                    return value_error("a tiling of zero erases the grid.");
                }
                if gw * across > SIDE || gh * down > SIDE {
                    return value_error("the tiling leaves the board.");
                }
                Ok(grid.tile(&[*down, *across]))
            }
            Op::Scale { k } => {
                if *k == 0 {
                    return value_error("a scale of zero erases the grid.");
                }
                if gw * k > SIDE || gh * k > SIDE {
                    return value_error("the scale leaves the board.");
                }
                Ok(grid.kron(&Tensor::full(vec![*k, *k], 1)))
            }
            Op::Recolor { map } => {
                if map.iter().any(|&c| c as usize >= COLORS) {
                    return value_error("a recolor map keeps its pens in 0..=9.");
                }
                let mut next = grid.clone();
                for cell in next.bytes_mut() {
                    *cell = map[*cell as usize];
                }
                Ok(next)
            }
            Op::Translate { dx, dy, fill } => {
                pen(*fill)?;
                if dx.unsigned_abs() as usize >= SIDE || dy.unsigned_abs() as usize >= SIDE {
                    return value_error("the move leaves the board.");
                }
                let mut next = Tensor::full(vec![gh, gw], *fill);
                for r in 0..gh {
                    for c in 0..gw {
                        let tr = r as i64 + dy;
                        let tc = c as i64 + dx;
                        if (0..gh as i64).contains(&tr) && (0..gw as i64).contains(&tc) {
                            next.set(&[tr as usize, tc as usize], grid.get(&[r, c]));
                        }
                    }
                }
                Ok(next)
            }
            Op::Paint { x, y, color } => {
                pen(*color)?;
                if *x >= gw || *y >= gh {
                    return value_error("the painted cell leaves the grid.");
                }
                let mut next = grid.clone();
                next.set(&[*y, *x], *color);
                Ok(next)
            }
            Op::Flood { x, y, color } => {
                pen(*color)?;
                if *x >= gw || *y >= gh {
                    return value_error("the flooded cell leaves the grid.");
                }
                Ok(flood(grid, *x, *y, *color))
            }
            Op::Step {
                color,
                birth,
                survive,
                wrap,
            } => {
                if *color == 0 || *color as usize >= COLORS {
                    return value_error("the step pen must be 1..=9.");
                }
                if birth.iter().chain(survive).any(|&n| n > RULE_MAX) {
                    return value_error("a step count leaves 0..=8.");
                }
                step(grid, *color, birth, survive, *wrap)
            }
            Op::Stamp { w, cells } => {
                if *w == 0 || *w > SIDE || cells.is_empty() || !cells.len().is_multiple_of(*w) {
                    return value_error("the stamp does not fold into rows.");
                }
                let h = cells.len() / w;
                if h > SIDE {
                    return value_error("the stamp leaves the board.");
                }
                if cells.iter().any(|&c| c as usize >= COLORS) {
                    return value_error("a stamp keeps its cells in 0..=9.");
                }
                Ok(Tensor::of(cells.clone(), vec![h, *w]))
            }
        }
    }
    /// Prints the op's canonical token, letters a-z and digits only.
    pub fn name(&self) -> String {
        match self {
            Op::Rotate { k } => format!("rot{k}"),
            Op::Reflect { vertical } => {
                if *vertical {
                    "refv".to_string()
                } else {
                    "refh".to_string()
                }
            }
            Op::Transpose => "tsp".to_string(),
            Op::Crop { x, y, w, h } => format!("cropc{x}r{y}w{w}h{h}"),
            Op::Autocrop => "auto".to_string(),
            Op::Pad { count, color } => format!("padn{count}c{color}"),
            Op::Tile { across, down } => format!("tilew{across}h{down}"),
            Op::Scale { k } => format!("scalek{k}"),
            Op::Recolor { map } => {
                let digits: String = map.iter().map(u8::to_string).collect();
                format!("map{digits}")
            }
            Op::Translate { dx, dy, fill } => {
                let col = if *dx < 0 { 'l' } else { 'r' };
                let row = if *dy < 0 { 'u' } else { 'd' };
                format!(
                    "mov{col}{}{row}{}f{fill}",
                    dx.unsigned_abs(),
                    dy.unsigned_abs()
                )
            }
            Op::Paint { x, y, color } => format!("paintc{x}r{y}p{color}"),
            Op::Flood { x, y, color } => format!("floodc{x}r{y}p{color}"),
            Op::Step {
                color,
                birth,
                survive,
                wrap,
            } => {
                let edge = if *wrap { "w" } else { "" };
                format!("stepp{color}b{}s{}{edge}", fold(birth), fold(survive))
            }
            Op::Stamp { w, cells } => {
                let digits: String = cells.iter().map(u8::to_string).collect();
                format!("stampw{w}d{digits}")
            }
        }
    }
    /// Parses a canonical op token back, or an error for any other spelling.
    pub fn parse(text: &str) -> Result<Op> {
        if text == "refh" {
            return Ok(Op::Reflect { vertical: false });
        }
        if text == "refv" {
            return Ok(Op::Reflect { vertical: true });
        }
        if text == "tsp" {
            return Ok(Op::Transpose);
        }
        if text == "auto" {
            return Ok(Op::Autocrop);
        }
        if let Some(rest) = text.strip_prefix("rot") {
            let (k, rest) = number(text, rest)?;
            close(text, rest)?;
            if !(1..=3).contains(&k) {
                return value_error(format!("token {text:?} takes 1..=3 quarter turns."));
            }
            return Ok(Op::Rotate { k });
        }
        if let Some(rest) = text.strip_prefix("cropc") {
            let (x, rest) = number(text, rest)?;
            let (y, rest) = tagged(text, rest, 'r')?;
            let (w, rest) = tagged(text, rest, 'w')?;
            let (h, rest) = tagged(text, rest, 'h')?;
            close(text, rest)?;
            return Ok(Op::Crop { x, y, w, h });
        }
        if let Some(rest) = text.strip_prefix("padn") {
            let (count, rest) = number(text, rest)?;
            let (color, rest) = tagged(text, rest, 'c')?;
            close(text, rest)?;
            return Ok(Op::Pad {
                count,
                color: pen_of(text, color)?,
            });
        }
        if let Some(rest) = text.strip_prefix("tilew") {
            let (across, rest) = number(text, rest)?;
            let (down, rest) = tagged(text, rest, 'h')?;
            close(text, rest)?;
            return Ok(Op::Tile { across, down });
        }
        if let Some(rest) = text.strip_prefix("scalek") {
            let (k, rest) = number(text, rest)?;
            close(text, rest)?;
            return Ok(Op::Scale { k });
        }
        if let Some(rest) = text.strip_prefix("map") {
            let cells = digits_of(text, rest)?;
            let Ok(map) = <[u8; 10]>::try_from(cells) else {
                return value_error(format!("token {text:?} wants exactly ten map digits."));
            };
            return Ok(Op::Recolor { map });
        }
        if let Some(rest) = text.strip_prefix("mov") {
            let (col, rest) = sign(text, rest, 'r', 'l')?;
            let (dx, rest) = number(text, rest)?;
            let (row, rest) = sign(text, rest, 'd', 'u')?;
            let (dy, rest) = number(text, rest)?;
            let (fill, rest) = tagged(text, rest, 'f')?;
            close(text, rest)?;
            if (col < 0 && dx == 0) || (row < 0 && dy == 0) {
                return value_error(format!("token {text:?} spells zero with a minus."));
            }
            return Ok(Op::Translate {
                dx: col * dx as i64,
                dy: row * dy as i64,
                fill: pen_of(text, fill)?,
            });
        }
        if let Some(rest) = text.strip_prefix("paintc") {
            let (x, rest) = number(text, rest)?;
            let (y, rest) = tagged(text, rest, 'r')?;
            let (color, rest) = tagged(text, rest, 'p')?;
            close(text, rest)?;
            return Ok(Op::Paint {
                x,
                y,
                color: pen_of(text, color)?,
            });
        }
        if let Some(rest) = text.strip_prefix("floodc") {
            let (x, rest) = number(text, rest)?;
            let (y, rest) = tagged(text, rest, 'r')?;
            let (color, rest) = tagged(text, rest, 'p')?;
            close(text, rest)?;
            return Ok(Op::Flood {
                x,
                y,
                color: pen_of(text, color)?,
            });
        }
        if let Some(rest) = text.strip_prefix("stepp") {
            let (color, rest) = number(text, rest)?;
            let Some(rest) = rest.strip_prefix('b') else {
                return value_error(format!("token {text:?} wants a b field."));
            };
            let (birth, rest) = counts(text, rest)?;
            let Some(rest) = rest.strip_prefix('s') else {
                return value_error(format!("token {text:?} wants an s field."));
            };
            let (survive, rest) = counts(text, rest)?;
            let wrap = match rest {
                "" => false,
                "w" => true,
                _ => return value_error(format!("token {text:?} holds a stray tail.")),
            };
            if color == 0 || color >= COLORS {
                return value_error(format!("token {text:?} steps a pen outside 1..=9."));
            }
            return Ok(Op::Step {
                color: color as u8,
                birth,
                survive,
                wrap,
            });
        }
        if let Some(rest) = text.strip_prefix("stampw") {
            let (w, rest) = number(text, rest)?;
            let Some(rest) = rest.strip_prefix('d') else {
                return value_error(format!("token {text:?} wants a d field."));
            };
            let cells = digits_of(text, rest)?;
            let op = Op::Stamp { w, cells };
            op.apply(&Tensor::new(vec![1, 1]))?;
            return Ok(op);
        }
        value_error(format!("token {text:?} names no op."))
    }
}

fn pen(color: u8) -> Result<()> {
    if color as usize >= COLORS {
        return value_error("a pen leaves 0..=9.");
    }
    Ok(())
}

// READERS

fn number<'a>(token: &str, text: &'a str) -> Result<(usize, &'a str)> {
    let end = text.bytes().take_while(u8::is_ascii_digit).count();
    if end == 0 {
        return value_error(format!("token {token:?} wants a number."));
    }
    if end > 1 && text.starts_with('0') {
        return value_error(format!("token {token:?} pads a number with zero."));
    }
    let Ok(value) = text[..end].parse() else {
        return value_error(format!("token {token:?} holds a number too large."));
    };
    Ok((value, &text[end..]))
}

fn tagged<'a>(token: &str, text: &'a str, tag: char) -> Result<(usize, &'a str)> {
    let Some(rest) = text.strip_prefix(tag) else {
        return value_error(format!("token {token:?} wants a {tag} field."));
    };
    number(token, rest)
}

fn sign<'a>(token: &str, text: &'a str, plus: char, minus: char) -> Result<(i64, &'a str)> {
    if let Some(rest) = text.strip_prefix(plus) {
        return Ok((1, rest));
    }
    if let Some(rest) = text.strip_prefix(minus) {
        return Ok((-1, rest));
    }
    value_error(format!("token {token:?} wants {plus} or {minus}."))
}

fn close(token: &str, rest: &str) -> Result<()> {
    if rest.is_empty() {
        Ok(())
    } else {
        value_error(format!("token {token:?} holds a stray tail."))
    }
}

fn pen_of(token: &str, value: usize) -> Result<u8> {
    if value >= COLORS {
        return value_error(format!("token {token:?} names a pen outside 0..=9."));
    }
    Ok(value as u8)
}

fn digits_of(token: &str, text: &str) -> Result<Vec<u8>> {
    if text.is_empty() {
        return value_error(format!("token {token:?} wants digits."));
    }
    text.bytes()
        .map(|b| {
            if b.is_ascii_digit() {
                Ok(b - b'0')
            } else {
                value_error(format!("token {token:?} holds a stray character."))
            }
        })
        .collect()
}

fn counts<'a>(token: &str, text: &'a str) -> Result<(Vec<usize>, &'a str)> {
    let end = text.bytes().take_while(u8::is_ascii_digit).count();
    let mut out = Vec::new();
    for b in text[..end].bytes() {
        let n = (b - b'0') as usize;
        if n > RULE_MAX {
            return value_error(format!("token {token:?} counts past 8."));
        }
        if out.last().is_some_and(|&last| last >= n) {
            return value_error(format!("token {token:?} counts out of order."));
        }
        out.push(n);
    }
    Ok((out, &text[end..]))
}

fn fold(counts: &[usize]) -> String {
    let mut folded = counts.to_vec();
    folded.sort_unstable();
    folded.dedup();
    assert!(
        folded.iter().all(|&n| n <= RULE_MAX),
        "a step count leaves 0..=8; the Moore neighborhood holds eight"
    );
    folded.iter().map(usize::to_string).collect()
}

// GEOMETRY

fn bounds(grid: &Tensor) -> Option<(usize, usize, usize, usize)> {
    let (gh, gw) = (grid.shape[0], grid.shape[1]);
    let (mut x0, mut y0, mut x1, mut y1) = (gw, gh, 0, 0);
    for r in 0..gh {
        for c in 0..gw {
            if grid.get(&[r, c]) != 0 {
                x0 = x0.min(c);
                y0 = y0.min(r);
                x1 = x1.max(c);
                y1 = y1.max(r);
            }
        }
    }
    if x0 == gw {
        return None;
    }
    Some((x0, y0, x1 - x0 + 1, y1 - y0 + 1))
}

fn flood(grid: &Tensor, x: usize, y: usize, color: u8) -> Tensor {
    let (gh, gw) = (grid.shape[0], grid.shape[1]);
    let from = grid.get(&[y, x]);
    let mut next = grid.clone();
    if from == color {
        return next;
    }
    let mut stack = vec![(x, y)];
    while let Some((cx, cy)) = stack.pop() {
        if next.get(&[cy, cx]) != from {
            continue;
        }
        next.set(&[cy, cx], color);
        if cx > 0 {
            stack.push((cx - 1, cy));
        }
        if cx + 1 < gw {
            stack.push((cx + 1, cy));
        }
        if cy > 0 {
            stack.push((cx, cy - 1));
        }
        if cy + 1 < gh {
            stack.push((cx, cy + 1));
        }
    }
    next
}

fn step(
    grid: &Tensor,
    color: u8,
    birth: &[usize],
    survive: &[usize],
    wrap: bool,
) -> Result<Tensor> {
    let mut alive = Tensor::new(grid.shape.clone());
    for (slot, &cell) in alive.bytes_mut().iter_mut().zip(grid.bytes()) {
        *slot = u8::from(cell == color);
    }
    let mut moore = Tensor::full(vec![3, 3], 1);
    moore.set(&[1, 1], 0);
    let counted = alive.neighbors(&moore, 1, wrap, Dtype::U8)?;
    let mut next = grid.clone();
    for (i, slot) in next.bytes_mut().iter_mut().enumerate() {
        let n = counted.bytes()[i] as usize;
        if *slot == color {
            *slot = if survive.contains(&n) { color } else { 0 };
        } else if *slot == 0 && birth.contains(&n) {
            *slot = color;
        }
    }
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::super::from_rows;
    use super::*;

    fn quad() -> Tensor {
        from_rows(&[vec![1, 2], vec![3, 4]]).unwrap()
    }

    #[test]
    fn rotate_walks_the_quarter_turns() {
        let one = Op::Rotate { k: 1 }.apply(&quad()).unwrap();
        assert_eq!(one, from_rows(&[vec![2, 4], vec![1, 3]]).unwrap());
        let two = Op::Rotate { k: 2 }.apply(&quad()).unwrap();
        assert_eq!(two, from_rows(&[vec![4, 3], vec![2, 1]]).unwrap());
        assert!(Op::Rotate { k: 0 }.apply(&quad()).is_err());
        assert!(Op::Rotate { k: 4 }.apply(&quad()).is_err());
    }
    #[test]
    fn reflect_and_transpose_are_involutions() {
        for op in [
            Op::Reflect { vertical: false },
            Op::Reflect { vertical: true },
            Op::Transpose,
        ] {
            let once = op.apply(&quad()).unwrap();
            assert_ne!(once, quad());
            assert_eq!(op.apply(&once).unwrap(), quad());
        }
        let h = Op::Reflect { vertical: false }.apply(&quad()).unwrap();
        assert_eq!(h, from_rows(&[vec![2, 1], vec![4, 3]]).unwrap());
    }
    #[test]
    fn crop_cuts_the_box_and_guards_the_edges() {
        let grid = from_rows(&[vec![0, 0, 0], vec![0, 5, 6], vec![0, 7, 8]]).unwrap();
        let cut = Op::Crop {
            x: 1,
            y: 1,
            w: 2,
            h: 2,
        }
        .apply(&grid)
        .unwrap();
        assert_eq!(cut, from_rows(&[vec![5, 6], vec![7, 8]]).unwrap());
        assert!(Op::Crop {
            x: 2,
            y: 0,
            w: 2,
            h: 1
        }
        .apply(&grid)
        .is_err());
        assert_eq!(Op::Autocrop.apply(&grid).unwrap(), cut);
        assert!(Op::Autocrop.apply(&Tensor::new(vec![2, 2])).is_err());
    }
    #[test]
    fn pad_tile_and_scale_grow_within_the_board() {
        let padded = Op::Pad { count: 1, color: 9 }.apply(&quad()).unwrap();
        assert_eq!(padded.shape, vec![4, 4]);
        assert_eq!(padded.get(&[0, 0]), 9);
        assert_eq!(padded.get(&[1, 1]), 1);
        let tiled = Op::Tile { across: 3, down: 1 }.apply(&quad()).unwrap();
        assert_eq!(tiled.shape, vec![2, 6]);
        assert_eq!(tiled.get(&[0, 4]), 1);
        let scaled = Op::Scale { k: 2 }.apply(&quad()).unwrap();
        assert_eq!(scaled.shape, vec![4, 4]);
        assert_eq!(scaled.get(&[1, 1]), 1);
        assert_eq!(scaled.get(&[3, 3]), 4);
        let wide = Tensor::full(vec![16, 16], 1);
        assert!(Op::Pad { count: 8, color: 0 }.apply(&wide).is_err());
        assert!(Op::Tile { across: 2, down: 1 }.apply(&wide).is_err());
        assert!(Op::Scale { k: 2 }.apply(&wide).is_err());
        assert!(Op::Scale { k: 0 }.apply(&wide).is_err());
    }
    #[test]
    fn recolor_rewrites_the_pens() {
        let map = [1, 0, 3, 2, 4, 5, 6, 7, 8, 9];
        let swapped = Op::Recolor { map }.apply(&quad()).unwrap();
        assert_eq!(swapped, from_rows(&[vec![0, 3], vec![2, 4]]).unwrap());
        assert!(Op::Recolor { map: [10; 10] }.apply(&quad()).is_err());
    }
    #[test]
    fn translate_slides_and_fills() {
        let moved = Op::Translate {
            dx: 1,
            dy: 0,
            fill: 7,
        }
        .apply(&quad())
        .unwrap();
        assert_eq!(moved, from_rows(&[vec![7, 1], vec![7, 3]]).unwrap());
        let gone = Op::Translate {
            dx: 0,
            dy: -2,
            fill: 0,
        }
        .apply(&quad())
        .unwrap();
        assert_eq!(gone.sum(), 0);
        assert!(Op::Translate {
            dx: 30,
            dy: 0,
            fill: 0
        }
        .apply(&quad())
        .is_err());
    }
    #[test]
    fn paint_and_flood_ink_cells() {
        let dot = Op::Paint {
            x: 1,
            y: 0,
            color: 9,
        }
        .apply(&quad())
        .unwrap();
        assert_eq!(dot.get(&[0, 1]), 9);
        assert!(Op::Paint {
            x: 2,
            y: 0,
            color: 1
        }
        .apply(&quad())
        .is_err());
        let grid = from_rows(&[vec![1, 1, 0], vec![0, 1, 0], vec![0, 0, 2]]).unwrap();
        let flooded = Op::Flood {
            x: 0,
            y: 0,
            color: 5,
        }
        .apply(&grid)
        .unwrap();
        assert_eq!(
            flooded,
            from_rows(&[vec![5, 5, 0], vec![0, 5, 0], vec![0, 0, 2]]).unwrap()
        );
    }
    #[test]
    fn step_runs_a_blinker_in_any_pen() {
        let grid = from_rows(&[vec![0, 0, 0], vec![3, 3, 3], vec![0, 0, 0]]).unwrap();
        let op = Op::Step {
            color: 3,
            birth: vec![3],
            survive: vec![2, 3],
            wrap: false,
        };
        let next = op.apply(&grid).unwrap();
        assert_eq!(
            next,
            from_rows(&[vec![0, 3, 0], vec![0, 3, 0], vec![0, 3, 0]]).unwrap()
        );
        assert_eq!(op.apply(&next).unwrap(), grid);
        assert!(Op::Step {
            color: 0,
            birth: vec![3],
            survive: vec![],
            wrap: false
        }
        .apply(&grid)
        .is_err());
        assert!(Op::Step {
            color: 1,
            birth: vec![9],
            survive: vec![],
            wrap: false
        }
        .apply(&grid)
        .is_err());
    }
    #[test]
    fn step_leaves_other_pens_as_scenery() {
        let grid = from_rows(&[vec![5, 1, 0], vec![0, 1, 0], vec![0, 1, 5]]).unwrap();
        let next = Op::Step {
            color: 1,
            birth: vec![3],
            survive: vec![2, 3],
            wrap: false,
        }
        .apply(&grid)
        .unwrap();
        assert_eq!(next.get(&[0, 0]), 5);
        assert_eq!(next.get(&[2, 2]), 5);
        assert_eq!(next.get(&[1, 0]), 1);
        assert_eq!(next.get(&[1, 2]), 1);
        assert_eq!(next.get(&[0, 1]), 0);
    }
    #[test]
    fn stamp_replaces_the_grid() {
        let op = Op::Stamp {
            w: 3,
            cells: vec![1, 2, 3, 4, 5, 6],
        };
        let out = op.apply(&quad()).unwrap();
        assert_eq!(out, from_rows(&[vec![1, 2, 3], vec![4, 5, 6]]).unwrap());
        assert!(Op::Stamp {
            w: 4,
            cells: vec![1, 2, 3]
        }
        .apply(&quad())
        .is_err());
    }
    #[test]
    fn every_op_token_round_trips() {
        for op in [
            Op::Rotate { k: 3 },
            Op::Reflect { vertical: true },
            Op::Reflect { vertical: false },
            Op::Transpose,
            Op::Crop {
                x: 0,
                y: 2,
                w: 10,
                h: 1,
            },
            Op::Autocrop,
            Op::Pad { count: 2, color: 0 },
            Op::Tile { across: 1, down: 3 },
            Op::Scale { k: 2 },
            Op::Recolor {
                map: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            },
            Op::Translate {
                dx: -2,
                dy: 1,
                fill: 4,
            },
            Op::Paint {
                x: 12,
                y: 0,
                color: 7,
            },
            Op::Flood {
                x: 0,
                y: 0,
                color: 2,
            },
            Op::Step {
                color: 4,
                birth: vec![1, 3],
                survive: Vec::new(),
                wrap: true,
            },
            Op::Stamp {
                w: 2,
                cells: vec![0, 1, 2, 3],
            },
        ] {
            let token = op.name();
            assert_eq!(Op::parse(&token).unwrap(), op, "{token}");
        }
    }
    #[test]
    #[should_panic(expected = "a step count leaves 0..=8")]
    fn a_step_count_above_eight_is_never_named() {
        let _ = Op::Step {
            color: 1,
            birth: vec![3, 12],
            survive: vec![2, 3],
            wrap: false,
        }
        .name();
    }
    #[test]
    fn stray_op_tokens_do_not_parse() {
        for bad in [
            "rot",
            "rot01",
            "cropc1r1w2",
            "padn1c10",
            "movl0d1f0",
            "movr1u0f0",
            "stepp0b3s23",
            "stepp1b32s2",
            "stepp1b3s23q",
            "map012",
            "map01234567890",
            "stampw0d1",
            "stampw2d123",
            "paintc1r1p10",
            "tsp2",
            "autox",
        ] {
            assert!(Op::parse(bad).is_err(), "{bad}");
        }
    }
}
