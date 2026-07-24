use super::models::Cell6d;
use super::painter::paint;
use super::Orientation;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use mrlycore::errors::Result;

pub type Triangle = ([(i64, i64); 3], [u8; 4]);

fn north(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

fn south(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

fn east(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x, y), (2 * x, y + 2), (2 * x + 2, y + 1)]
}

fn west(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x + 2, y), (2 * x + 2, y + 2), (2 * x, y + 1)]
}

fn painted(cell: &Cell6d) -> Vec<[u8; 4]> {
    match &cell.cell.cell.colors {
        Some(colors) => colors.clone(),
        None => paint(cell.clone(), None, Some(Mode::Type))
            .cell
            .cell
            .colors
            .unwrap(),
    }
}

pub fn triangles(cell: &Cell6d) -> Result<Vec<Triangle>> {
    let inner = &cell.cell;
    let (height, width) = (inner.height(), inner.width());
    let colors = painted(cell);
    let orient = super::geometry::orientation(width, height)?;
    let start = cell.start as i64;
    let mut out = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let rgba = colors[y * width + x];
            if rgba[3] == 0 {
                continue;
            }
            let flip = (x as i64 + y as i64 + start).rem_euclid(2);
            let points = match orient {
                Orientation::Horizontal => {
                    if flip == 0 {
                        north(x as i64, y as i64)
                    } else {
                        south(x as i64, y as i64)
                    }
                }
                Orientation::Vertical => {
                    if flip == 0 {
                        east(x as i64, y as i64)
                    } else {
                        west(x as i64, y as i64)
                    }
                }
            };
            out.push((points, rgba));
        }
    }
    Ok(out)
}

fn bounds(tris: &[Triangle]) -> (i64, i64, i64, i64) {
    let xs = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.0));
    let ys = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.1));
    let min_x = xs.clone().min().unwrap();
    let max_x = xs.max().unwrap();
    let min_y = ys.clone().min().unwrap();
    let max_y = ys.max().unwrap();
    (min_x, max_x, min_y, max_y)
}

pub fn svg(cell: &Cell6d, scale: usize) -> Result<String> {
    let tris = triangles(cell)?;
    if tris.is_empty() {
        return Ok("<svg></svg>".to_string());
    }
    let (min_x, max_x, min_y, max_y) = bounds(&tris);
    let img_w = (max_x - min_x) * scale as i64;
    let img_h = (max_y - min_y) * scale as i64;
    let mut out = vec![format!(
        "<svg width=\"{img_w}\" height=\"{img_h}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for (points, [r, g, b, a]) in &tris {
        let fill = Color::rgba(*r, *g, *b, *a).to_hex();
        let pts: Vec<String> = points
            .iter()
            .map(|(x, y)| {
                format!(
                    "{},{}",
                    (x - min_x) * scale as i64,
                    (y - min_y) * scale as i64
                )
            })
            .collect();
        out.push(format!(
            "<polygon points=\"{}\" fill=\"{fill}\" stroke=\"none\"/>",
            pts.join(" ")
        ));
    }
    out.push("</svg>".to_string());
    Ok(out.join("\n"))
}

pub fn png(cell: &Cell6d, scale: usize) -> Result<Vec<u8>> {
    let tris = triangles(cell)?;
    if tris.is_empty() {
        return mrlycore::errors::value_error("nothing to render.");
    }
    let (min_x, max_x, min_y, max_y) = bounds(&tris);
    let img_w = ((max_x - min_x) * scale as i64) as usize;
    let img_h = ((max_y - min_y) * scale as i64) as usize;
    let mut pixels = vec![[0u8; 4]; img_w * img_h];
    for (points, rgba) in &tris {
        let scaled: Vec<(f64, f64)> = points
            .iter()
            .map(|(x, y)| {
                (
                    ((x - min_x) * scale as i64) as f64,
                    ((y - min_y) * scale as i64) as f64,
                )
            })
            .collect();
        let xs: Vec<f64> = scaled.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = scaled.iter().map(|p| p.1).collect();
        let x0 = xs.iter().cloned().fold(f64::MAX, f64::min).floor().max(0.0) as usize;
        let x1 = (xs.iter().cloned().fold(f64::MIN, f64::max).ceil() as usize).min(img_w);
        let y0 = ys.iter().cloned().fold(f64::MAX, f64::min).floor().max(0.0) as usize;
        let y1 = (ys.iter().cloned().fold(f64::MIN, f64::max).ceil() as usize).min(img_h);
        let edge = |a: (f64, f64), b: (f64, f64), p: (f64, f64)| -> f64 {
            (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)
        };
        for py in y0..y1 {
            for px in x0..x1 {
                let p = (px as f64 + 0.5, py as f64 + 0.5);
                let e0 = edge(scaled[0], scaled[1], p);
                let e1 = edge(scaled[1], scaled[2], p);
                let e2 = edge(scaled[2], scaled[0], p);
                let inside =
                    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0);
                if inside {
                    pixels[py * img_w + px] = *rgba;
                }
            }
        }
    }
    mrlycore::io::png(&pixels, img_w, img_h, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::six::designs::iso_design;
    use crate::six::geometry::blank;
    use crate::six::models::Cell6d;
    use crate::six::{Orientation, Projection};
    #[test]
    fn triangle_geometry() {
        assert_eq!(north(0, 0), [(0, 2), (1, 0), (2, 2)]);
        assert_eq!(south(0, 0), [(0, 0), (1, 2), (2, 0)]);
        assert_eq!(east(1, 1), [(2, 1), (2, 3), (4, 2)]);
    }
    #[test]
    fn iso_renders_triangles() {
        let i = iso_design(23, 3, 1, 2).unwrap();
        let tris = triangles(&i).unwrap();
        assert!(!tris.is_empty());
        let s = svg(&i, 10).unwrap();
        assert!(s.contains("<polygon"));
        let bytes = png(&i, 10).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
    #[test]
    fn hexagon_renders() {
        let hex = Cell6d::new(
            blank(3, Orientation::Horizontal, 1, 0),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        let tris = triangles(&hex).unwrap();
        assert!(!tris.is_empty());
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::six::designs::iso_design;
    #[test]
    fn golden_parity_with_pre_io_bytes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let cases: Vec<Vec<u8>> = vec![
            png(&iso_design(23, 3, 1, 2).unwrap(), 10).unwrap(),
            png(&iso_design(5, 4, 1, 2).unwrap(), 3).unwrap(),
        ];
        let pins: [(usize, u64); 2] = [(3500, 5809523862898589966), (1924, 5773799449462974685)];
        for (bytes, (len, hash)) in cases.iter().zip(pins) {
            let mut h = DefaultHasher::new();
            bytes.hash(&mut h);
            assert_eq!(
                bytes.len(),
                len,
                "byte length drifted from pre-io::png bytes"
            );
            assert_eq!(h.finish(), hash, "bytes drifted from pre-io::png rendering");
        }
    }
}
