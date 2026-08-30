use super::geometry::orientation;
use super::models::Cell6d;
use super::{Orientation, FILL, GRID};
use mrlycore::errors::{value_error, Result};

const ROW: f64 = 0.866_025_403_784_438_6;

/// Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.
pub fn raster(cell: &Cell6d, size: usize) -> Result<Vec<f32>> {
    if size == 0 {
        return value_error("size must be at least 1.");
    }
    let (width, height) = (cell.width(), cell.height());
    let types = cell.cell.cell.types.bytes();
    let flipped = orientation(width, height)? == Orientation::Vertical;
    let (cols, rows) = if flipped {
        (height, width)
    } else {
        (width, height)
    };
    let start = cell.start as i64 + flipped as i64;
    let at = |col: i64, row: i64| -> Option<u8> {
        if col < 0 || row < 0 || col >= cols as i64 || row >= rows as i64 {
            return None;
        }
        let (col, row) = (col as usize, row as usize);
        let index = if flipped {
            col * width + row
        } else {
            row * width + col
        };
        Some(types[index])
    };
    let mut bounds: Option<(usize, usize, usize, usize)> = None;
    for row in 0..rows {
        for col in 0..cols {
            let value = at(col as i64, row as i64).unwrap();
            if value == GRID {
                continue;
            }
            bounds = Some(match bounds {
                None => (col, col, row, row),
                Some((a, b, c, d)) => (a.min(col), b.max(col), c.min(row), d.max(row)),
            });
        }
    }
    let Some((min_col, max_col, min_row, max_row)) = bounds else {
        return value_error("nothing to rasterize.");
    };
    let (left, right) = (min_col as f64, max_col as f64 + 2.0);
    let (top, bottom) = (
        2.0 * min_row as f64 * ROW,
        (2.0 * max_row as f64 + 2.0) * ROW,
    );
    let (wide, tall) = (right - left, bottom - top);
    let unit = wide.max(tall) / size as f64;
    let (x0, y0) = (
        left - (size as f64 * unit - wide) / 2.0,
        top - (size as f64 * unit - tall) / 2.0,
    );
    let mut out = Vec::with_capacity(size * size);
    for i in 0..size {
        for j in 0..size {
            let px = x0 + (j as f64 + 0.5) * unit;
            let py = (y0 + (i as f64 + 0.5) * unit) / ROW;
            let row = (py / 2.0).floor();
            let t = py / 2.0 - row;
            let base = px.floor() as i64;
            let row = row as i64;
            let mut hit = 0.0;
            for col in [base - 1, base] {
                let north = (col + row + start).rem_euclid(2) == 0;
                let (lo, hi) = if north {
                    (col as f64 + 1.0 - t, col as f64 + 1.0 + t)
                } else {
                    (col as f64 + t, col as f64 + 2.0 - t)
                };
                if px >= lo && px < hi {
                    if at(col, row) == Some(FILL) {
                        hit = 1.0;
                    }
                    break;
                }
            }
            out.push(hit);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::six::{census, cut_design};

    fn fraction(cell: &Cell6d, size: usize) -> f64 {
        let pixels = raster(cell, size).unwrap();
        pixels.iter().map(|&v| v as f64).sum::<f64>() / (size * size) as f64
    }

    fn hexagon_share(cell: &Cell6d) -> f64 {
        let tally = census::census(cell, false);
        let n = ((tally.triangles as f64) / 6.0).sqrt();
        let wide = 4.0 * n;
        let area = (tally.fills as f64) * 3f64.sqrt();
        area / (wide * wide)
    }

    #[test]
    fn the_solid_cut_fills_its_hexagon() {
        let cell = cut_design(255, 3, 1, 2).unwrap();
        let expect = hexagon_share(&cell);
        assert!((fraction(&cell, 400) - expect).abs() < 0.01, "{expect}");
        let centre = raster(&cell, 101).unwrap()[50 * 101 + 50];
        assert_eq!(centre, 1.0);
    }

    #[test]
    fn the_carpet_cut_is_pierced_at_the_centre() {
        let cell = cut_design(23, 3, 1, 2).unwrap();
        let expect = hexagon_share(&cell);
        assert!((fraction(&cell, 400) - expect).abs() < 0.01, "{expect}");
        let centre = raster(&cell, 101).unwrap()[50 * 101 + 50];
        assert_eq!(centre, 0.0);
    }

    #[test]
    fn an_empty_size_is_refused() {
        assert!(raster(&cut_design(23, 3, 1, 2).unwrap(), 0).is_err());
    }
}
