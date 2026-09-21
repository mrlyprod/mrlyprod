use super::geometry::orientation;
use super::models::Cell6d;
use super::painter::paint;
use super::Orientation;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use mrlycore::errors::{value_error, Result};

/// A screen triangle: three grid points and an RGBA color.
pub type Triangle = ([(i64, i64); 3], [u8; 4]);

#[derive(Clone, Debug)]
struct Rect {
    triangles: Vec<Triangle>,
    origin: (i64, i64),
    size: (usize, usize),
}

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

/// Folds a cell into colored screen triangles, dropping the transparent ones.
pub fn triangles(cell: &Cell6d) -> Result<Vec<Triangle>> {
    let inner = &cell.cell;
    let (height, width) = (inner.height(), inner.width());
    let colors = painted(cell);
    let orient = orientation(width, height)?;
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

fn window(cell: &Cell6d, padding: i64) -> Result<Rect> {
    let triangles = triangles(cell)?;
    if triangles.is_empty() {
        return value_error("nothing to render.");
    }
    let (min_x, max_x, min_y, max_y) = bounds(&triangles);
    Ok(Rect {
        triangles,
        origin: (min_x - padding, min_y - padding),
        size: (
            (max_x - min_x + 2 * padding) as usize,
            (max_y - min_y + 2 * padding) as usize,
        ),
    })
}

fn stroke_of(outline: Option<Color>, width: usize) -> String {
    match outline {
        Some(c) => format!("stroke=\"{}\" stroke-width=\"{width}\"", c.to_hex()),
        None => "stroke=\"none\"".to_string(),
    }
}

fn polygon(
    points: &[(i64, i64); 3],
    rgba: [u8; 4],
    origin: (i64, i64),
    scale: usize,
    stroke: &str,
) -> String {
    let [r, g, b, a] = rgba;
    let fill = Color::rgba(r, g, b, a).to_hex();
    let pts: Vec<String> = points
        .iter()
        .map(|(x, y)| {
            format!(
                "{},{}",
                (x - origin.0) * scale as i64,
                (y - origin.1) * scale as i64
            )
        })
        .collect();
    format!(
        "<polygon points=\"{}\" fill=\"{fill}\" {stroke}/>",
        pts.join(" ")
    )
}

fn sheet(view: &Rect, scale: usize, stroke: &str) -> String {
    let (img_w, img_h) = (view.size.0 * scale, view.size.1 * scale);
    let mut out = vec![format!(
        "<svg width=\"{img_w}\" height=\"{img_h}\" viewBox=\"0 0 {img_w} {img_h}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for (points, rgba) in &view.triangles {
        out.push(polygon(points, *rgba, view.origin, scale, stroke));
    }
    out.push("</svg>".to_string());
    out.join("\n")
}

/// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given, or an error when nothing renders.
pub fn svg(cell: &Cell6d, scale: usize, outline: Option<Color>, width: usize) -> Result<String> {
    let padding = if outline.is_some() { width as i64 } else { 0 };
    Ok(sheet(
        &window(cell, padding)?,
        scale,
        &stroke_of(outline, width),
    ))
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
        let s = svg(&i, 10, None, 1).unwrap();
        assert!(s.contains("<polygon"));
        assert!(s.contains("stroke=\"none\""));
    }
    #[test]
    fn outline_strokes_and_pads_the_svg() {
        let i = iso_design(23, 3, 1, 2).unwrap();
        let plain = svg(&i, 4, None, 1).unwrap();
        let lined = svg(&i, 4, Some(Color::rgba(255, 0, 0, 255)), 2).unwrap();
        assert!(lined.contains("stroke-width=\"2\""));
        assert!(lined.contains(&Color::rgba(255, 0, 0, 255).to_hex()));
        let size = |svg: &str| -> usize {
            svg.split("width=\"")
                .nth(1)
                .unwrap()
                .split('"')
                .next()
                .unwrap()
                .parse()
                .unwrap()
        };
        assert_eq!(size(&lined), size(&plain) + 4 * 4);
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
    #[test]
    fn nothing_to_render_errors_in_both_doors() {
        let bare = Cell6d::new(
            crate::two::Cell2d::new(mrlycore::Tensor::full(vec![2, 3], crate::six::GRID)),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        assert!(triangles(&bare).unwrap().is_empty());
        assert!(svg(&bare, 4, None, 1).is_err());
    }
}
