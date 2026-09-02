use crate::board::{Board, Frame};
use mrlycore::Color;
use mrlymath::two::Cell2d;

// GRID

/// A lattice of cells laid over a frame, each cell drawn inside its own gap.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grid {
    /// The frame the lattice covers.
    pub frame: Frame,
    /// The number of columns.
    pub cols: usize,
    /// The number of rows.
    pub rows: usize,
    /// The share of a cell left empty between one cell and the next.
    pub gap: f64,
}

impl Grid {
    /// Lays a lattice of cols by rows over the frame, keeping a gap of that fraction of a cell.
    pub fn new(frame: Frame, cols: usize, rows: usize, gap: f64) -> Grid {
        Grid {
            frame,
            cols: cols.max(1),
            rows: rows.max(1),
            gap,
        }
    }
    /// Returns the drawn box of one cell as its corner and its size; at gap zero the edges snap to whole pixels so neighbours meet without a seam.
    pub fn cell(&self, col: usize, row: usize) -> (f64, f64, f64, f64) {
        let w = self.frame.w / self.cols as f64;
        let h = self.frame.h / self.rows as f64;
        if self.gap <= 0.0 {
            let x0 = (self.frame.x + col as f64 * w).round();
            let x1 = (self.frame.x + (col + 1) as f64 * w).round();
            let y0 = (self.frame.y + row as f64 * h).round();
            let y1 = (self.frame.y + (row + 1) as f64 * h).round();
            return (x0, y0, x1 - x0, y1 - y0);
        }
        let pad = self.gap * w.min(h) / 2.0;
        (
            self.frame.x + col as f64 * w + pad,
            self.frame.y + row as f64 * h + pad,
            w - 2.0 * pad,
            h - 2.0 * pad,
        )
    }
    /// Fills one cell.
    pub fn fill(&self, board: &mut Board, col: usize, row: usize, color: Color) {
        let (x, y, w, h) = self.cell(col, row);
        board.rect(x, y, w, h, color);
    }
    /// Fills every cell of a flat design whose type byte the ink maps to a color.
    pub fn paint(&self, board: &mut Board, cells: &Cell2d, ink: impl Fn(u8) -> Option<Color>) {
        let types = cells.types();
        let (height, width) = (cells.height(), cells.width());
        for row in 0..self.rows.min(height) {
            for col in 0..self.cols.min(width) {
                if let Some(color) = ink(types.get(&[row, col])) {
                    self.fill(board, col, row, color);
                }
            }
        }
    }
    /// Fills every true cell of a mask.
    pub fn carpet(&self, board: &mut Board, mask: &[Vec<bool>], color: Color) {
        for (row, line) in mask.iter().enumerate().take(self.rows) {
            for (col, on) in line.iter().enumerate().take(self.cols) {
                if *on {
                    self.fill(board, col, row, color);
                }
            }
        }
    }
}

// MASKS

/// The MrlyProd logo, the five by five seed the mark grows from.
pub const LOGO: [&str; 5] = ["11111", "10101", "11111", "10101", "11111"];

/// Grows a 0/1 string mask by Kronecker substitution, level one being the seed itself.
pub fn mask(rows: &[&str], level: usize) -> Vec<Vec<bool>> {
    let seed: Vec<Vec<bool>> = rows
        .iter()
        .map(|row| row.chars().map(|c| c == '1').collect())
        .collect();
    let mut out = seed.clone();
    for _ in 1..level.max(1) {
        let width = seed.first().map_or(0, |row| row.len());
        let mut next = Vec::with_capacity(out.len() * seed.len());
        for row in &out {
            for inner in &seed {
                let mut line = Vec::with_capacity(row.len() * width);
                for on in row {
                    if *on {
                        line.extend_from_slice(inner);
                    } else {
                        line.extend(std::iter::repeat_n(false, width));
                    }
                }
                next.push(line);
            }
        }
        out = next;
    }
    out
}

/// Fills every true cell of a mask laid over a frame.
pub fn carpet(board: &mut Board, frame: Frame, mask: &[Vec<bool>], gap: f64, color: Color) {
    let rows = mask.len();
    let cols = mask.first().map_or(0, |row| row.len());
    if rows == 0 || cols == 0 {
        return;
    }
    Grid::new(frame, cols, rows, gap).carpet(board, mask, color);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_logo_at_level_two_is_twenty_five_wide_and_squares_its_ones() {
        let rows = mask(&LOGO, 2);
        let ones: usize = rows.iter().flatten().filter(|on| **on).count();
        assert_eq!(rows.len(), 25);
        assert_eq!(rows[0].len(), 25);
        assert_eq!(ones, 21 * 21);
    }
    #[test]
    fn at_gap_zero_neighbouring_cells_meet_on_a_whole_pixel() {
        let lattice = Grid::new(Frame::new(3.3, 0.0, 389.12, 389.12), 5, 5, 0.0);
        let (x0, _, w0, _) = lattice.cell(0, 0);
        let (x1, _, _, _) = lattice.cell(1, 0);
        assert_eq!(x0 + w0, x1);
        assert_eq!(x1, x1.round());
    }
}
