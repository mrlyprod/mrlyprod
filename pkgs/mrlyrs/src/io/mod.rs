use crate::core::cell::Cell;
use crate::core::colors::Color;
use crate::core::errors::{value_error, Result};
use crate::core::ramp::Colorizer;
use std::collections::HashMap;

pub use crate::core::codec::{base64, png};

pub fn colorize(types: &[u8], colorizer: &Colorizer, max: usize) -> Vec<[u8; 4]> {
    let values: Vec<usize> = types.iter().map(|&v| v as usize).collect();
    colorizer.colors(&values, max)
}

pub fn recolor(grid: &Cell, palette: &[Color]) -> Result<Cell> {
    if palette.is_empty() {
        return value_error("palette must not be empty.");
    }
    let colors = grid
        .colors
        .clone()
        .unwrap_or_else(|| vec![[0, 0, 0, 0]; grid.size()]);
    let mut out = grid.clone();
    out.colors = Some(colors.iter().map(|&c| nearest(c, palette)).collect());
    Ok(out)
}

fn nearest(c: [u8; 4], palette: &[Color]) -> [u8; 4] {
    let mut best = palette[0];
    let mut best_dist = i32::MAX;
    for &p in palette {
        let dr = p.r as i32 - c[0] as i32;
        let dg = p.g as i32 - c[1] as i32;
        let db = p.b as i32 - c[2] as i32;
        let dist = dr * dr + dg * dg + db * db;
        if dist < best_dist {
            best_dist = dist;
            best = p;
        }
    }
    [best.r, best.g, best.b, c[3]]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Binary,
    Grayscale,
    Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Analysis {
    pub kind: Kind,
    pub dominant: Color,
    pub mean_luminance: f64,
    pub min_luminance: f64,
    pub max_luminance: f64,
}

fn luminance(c: [u8; 4]) -> f64 {
    0.299 * c[0] as f64 + 0.587 * c[1] as f64 + 0.114 * c[2] as f64
}

fn dominant(colors: &[[u8; 4]]) -> [u8; 4] {
    let mut counts: HashMap<[u8; 4], usize> = HashMap::new();
    for &c in colors {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, n)| n)
        .map(|(c, _)| c)
        .unwrap_or([0, 0, 0, 0])
}

fn luminance_stats(colors: &[[u8; 4]]) -> (f64, f64, f64) {
    if colors.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let values: Vec<f64> = colors.iter().map(|&c| luminance(c)).collect();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}

pub fn analyze(grid: &Cell) -> Analysis {
    match &grid.colors {
        Some(colors) => {
            let binary = colors
                .iter()
                .all(|&c| (c[0], c[1], c[2]) == (0, 0, 0) || (c[0], c[1], c[2]) == (255, 255, 255));
            let grayscale = !binary && colors.iter().all(|&c| c[0] == c[1] && c[1] == c[2]);
            let kind = if binary {
                Kind::Binary
            } else if grayscale {
                Kind::Grayscale
            } else {
                Kind::Color
            };
            let rgb = dominant(colors);
            let (mean, min, max) = luminance_stats(colors);
            Analysis {
                kind,
                dominant: Color::rgba(rgb[0], rgb[1], rgb[2], rgb[3]),
                mean_luminance: mean,
                min_luminance: min,
                max_luminance: max,
            }
        }
        None => {
            let bytes = grid.types.bytes();
            let binary = bytes.iter().all(|&v| v == 0 || v == 1);
            let kind = if binary {
                Kind::Binary
            } else {
                Kind::Grayscale
            };
            let colors: Vec<[u8; 4]> = bytes.iter().map(|&v| [v, v, v, 255]).collect();
            let rgb = dominant(&colors);
            let (mean, min, max) = luminance_stats(&colors);
            Analysis {
                kind,
                dominant: Color::rgb(rgb[0], rgb[1], rgb[2]),
                mean_luminance: mean,
                min_luminance: min,
                max_luminance: max,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    use crate::core::colors::{BLACK, BLUE, RED, WHITE};
    #[test]
    fn colorize_maps_types_through_colorizer() {
        let types = [0u8, 1, 0, 1];
        let colorizer = Colorizer::two_tone(WHITE, BLACK);
        let colors = colorize(&types, &colorizer, 1);
        assert_eq!(
            colors,
            vec![
                [255, 255, 255, 255],
                [0, 0, 0, 255],
                [255, 255, 255, 255],
                [0, 0, 0, 255]
            ]
        );
    }
    #[test]
    fn recolor_snaps_to_nearest_palette_entry() {
        let mut grid = Cell::new(atoms::carpet_2d(2));
        grid.colors = Some(vec![
            [10, 10, 10, 255],
            [240, 20, 20, 255],
            [20, 20, 240, 255],
            [250, 250, 250, 255],
        ]);
        let out = recolor(&grid, &[BLACK, RED, BLUE, WHITE]).unwrap();
        let colors = out.colors.unwrap();
        assert_eq!(colors[0], [BLACK.r, BLACK.g, BLACK.b, 255]);
        assert_eq!(colors[1], [RED.r, RED.g, RED.b, 255]);
        assert_eq!(colors[2], [BLUE.r, BLUE.g, BLUE.b, 255]);
        assert_eq!(colors[3], [WHITE.r, WHITE.g, WHITE.b, 255]);
    }
    #[test]
    fn recolor_rejects_empty_palette() {
        let grid = Cell::new(atoms::carpet_2d(2));
        assert!(recolor(&grid, &[]).is_err());
    }
    #[test]
    fn analyze_detects_binary_types_without_colors() {
        let grid = Cell::new(atoms::carpet_2d(3));
        let a = analyze(&grid);
        assert_eq!(a.kind, Kind::Binary);
    }
    #[test]
    fn analyze_detects_grayscale_and_color() {
        let mut gray = Cell::new(atoms::carpet_2d(2));
        gray.colors = Some(vec![[10, 10, 10, 255], [200, 200, 200, 255]]);
        assert_eq!(analyze(&gray).kind, Kind::Grayscale);
        let mut color = Cell::new(atoms::carpet_2d(2));
        color.colors = Some(vec![[10, 200, 30, 255], [200, 10, 30, 255]]);
        assert_eq!(analyze(&color).kind, Kind::Color);
    }
    #[test]
    fn analyze_reports_luminance_bounds() {
        let mut grid = Cell::new(atoms::carpet_2d(2));
        grid.colors = Some(vec![
            [0, 0, 0, 255],
            [255, 255, 255, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
        ]);
        let a = analyze(&grid);
        assert_eq!(a.min_luminance, 0.0);
        assert!((a.max_luminance - 255.0).abs() < f64::EPSILON);
        assert_eq!(a.dominant, Color::rgba(0, 0, 0, 255));
    }
}
