use crate::models::{CellWang, Result};
use crate::tag::Move;

pub type Point = (i64, i64);

pub fn triangle(x: usize, y: usize, side: usize) -> [Point; 3] {
    let (x, y) = (2 * x as i64, 2 * y as i64);
    let (top_left, top_right) = ((x, y), (x + 2, y));
    let (bottom_right, bottom_left) = ((x + 2, y + 2), (x, y + 2));
    let center = (x + 1, y + 1);
    match side {
        0 => [top_left, top_right, center],
        1 => [top_right, bottom_right, center],
        2 => [bottom_right, bottom_left, center],
        _ => [bottom_left, top_left, center],
    }
}

fn place(op: Move, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
    match op {
        Move::Turn(k) => match k % 4 {
            0 => (x, y),
            1 => (y, width - 1 - x),
            2 => (width - 1 - x, height - 1 - y),
            _ => (height - 1 - y, x),
        },
        Move::FlipX => (width - 1 - x, y),
        Move::FlipY => (x, height - 1 - y),
        Move::FlipMain => (y, x),
        Move::FlipAnti => (height - 1 - y, width - 1 - x),
    }
}

fn parent(row: usize, col: usize, side: usize, number: usize) -> usize {
    let spread = |at: usize| 6 * at as i64 + 3 - 3 * number as i64;
    let (dx, dy) = [(0, -2), (2, 0), (0, 2), (-2, 0)][side];
    let (u, v) = (spread(col) + dx, spread(row) + dy);
    match (v.abs() > u.abs(), v < 0, u > 0) {
        (true, true, _) => 0,
        (true, false, _) => 2,
        (false, _, true) => 1,
        (false, _, false) => 3,
    }
}

impl CellWang {
    pub fn transform(&self, op: Move) -> CellWang {
        let swapped = matches!(
            op,
            Move::Turn(1) | Move::Turn(3) | Move::FlipMain | Move::FlipAnti
        );
        let (width, height) = if swapped {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        };
        let mut out = CellWang::new(width, height);
        for y in 0..self.height {
            for x in 0..self.width {
                let (to_x, to_y) = place(op, x, y, self.width, self.height);
                out.put(to_x, to_y, op.sides(self.at(x, y)));
            }
        }
        out
    }
    pub fn rotate(&self, k: usize) -> CellWang {
        self.transform(Move::Turn(k % 4))
    }
    pub fn tile(&self, width: usize, height: usize) -> CellWang {
        let mut out = CellWang::new(self.width * width, self.height * height);
        for y in 0..out.height {
            for x in 0..out.width {
                out.put(x, y, self.at(x % self.width, y % self.height));
            }
        }
        out
    }
    pub fn combine(&self, other: &CellWang) -> Result<CellWang> {
        if other.width != other.height {
            return Err("an inner cell must be square to clip along its diagonals.".to_string());
        }
        let number = other.width;
        let mut out = CellWang::new(self.width * number, self.height * number);
        for y in 0..out.height {
            for x in 0..out.width {
                let (outer, inner) = (
                    self.at(x / number, y / number),
                    other.at(x % number, y % number),
                );
                let mut sides = [0; 4];
                for (side, slot) in sides.iter_mut().enumerate() {
                    if outer[parent(y % number, x % number, side, number)] != 0 {
                        *slot = inner[side];
                    }
                }
                out.put(x, y, sides);
            }
        }
        Ok(out)
    }
    pub fn fractal(&self, level: usize) -> Result<CellWang> {
        if level < 1 {
            return Err("level must be at least 1.".to_string());
        }
        let mut out = self.clone();
        for _ in 1..level {
            out = out.combine(self)?;
        }
        Ok(out)
    }
}

pub fn merge(cells: &[CellWang], width: usize, height: usize) -> Result<CellWang> {
    if cells.len() != width * height || cells.is_empty() {
        return Err(format!(
            "expected {} cells, got {}.",
            width * height,
            cells.len()
        ));
    }
    let (inner_w, inner_h) = (cells[0].width, cells[0].height);
    if cells
        .iter()
        .any(|c| c.width != inner_w || c.height != inner_h)
    {
        return Err("all merged cells must share one shape.".to_string());
    }
    let mut out = CellWang::new(inner_w * width, inner_h * height);
    for y in 0..out.height {
        for x in 0..out.width {
            let block = &cells[(y / inner_h) * width + x / inner_w];
            out.put(x, y, block.at(x % inner_w, y % inner_h));
        }
    }
    Ok(out)
}
