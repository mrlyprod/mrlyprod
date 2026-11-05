use super::Cell2d;
use crate::cell::push_glyph;
use mrly_core::cell::Mode;
use mrly_core::cell::mapping;
use mrly_core::colors::Color;
use mrly_core::error::{Result, value_error};
use mrly_core::image::Image;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Square,
    Circle,
    Diamond,
}

fn painted(cell: &Cell2d) -> Vec<[u8; 4]> {
    match &cell.cell.colors {
        Some(colors) => colors.clone(),
        None => cell
            .clone()
            .paint(&mapping(), Mode::Type, None)
            .ok()
            .and_then(|fresh| fresh.cell.colors)
            .unwrap_or_default(),
    }
}

pub fn text(cell: &Cell2d, glyphs: Option<&HashMap<u8, String>>) -> Vec<String> {
    let (h, w) = (cell.height(), cell.width());
    let mut rows = Vec::with_capacity(h);
    for y in 0..h {
        let mut row = String::with_capacity(w);
        for x in 0..w {
            let v = cell.types().at(y * w + x) as u8;
            push_glyph(&mut row, v, glyphs);
        }
        rows.push(row);
    }
    rows
}

fn diamond_points(scale: usize) -> [(f64, f64); 4] {
    let (s, h) = (scale as f64, (scale / 2) as f64);
    [(h, 0.0), (s, h), (h, s), (0.0, h)]
}

fn in_diamond(points: &[(f64, f64); 4], x: f64, y: f64) -> bool {
    let (mut low, mut high) = (false, false);
    for k in 0..4 {
        let (ax, ay) = points[k];
        let (bx, by) = points[(k + 1) % 4];
        let cross = (bx - ax) * (y - ay) - (by - ay) * (x - ax);
        low |= cross < 0.0;
        high |= cross > 0.0;
    }
    !(low && high)
}

fn stencil(shape: Shape, scale: usize) -> Vec<bool> {
    let side = scale + 1;
    let mut out = vec![true; side * side];
    match shape {
        Shape::Square => {}
        Shape::Circle => {
            let c = side as f64 / 2.0;
            for i in 0..side {
                for j in 0..side {
                    let (dy, dx) = (i as f64 + 0.5 - c, j as f64 + 0.5 - c);
                    out[i * side + j] = dx * dx + dy * dy <= c * c;
                }
            }
        }
        Shape::Diamond => {
            let points = diamond_points(scale);
            for i in 0..side {
                for j in 0..side {
                    out[i * side + j] = in_diamond(&points, j as f64, i as f64);
                }
            }
        }
    }
    out
}

fn rim(stencil: &[bool], side: usize, width: usize) -> Vec<bool> {
    let inside = |i: i64, j: i64| {
        i >= 0
            && j >= 0
            && (i as usize) < side
            && (j as usize) < side
            && stencil[i as usize * side + j as usize]
    };
    let reach = width as i64;
    let mut out = vec![false; side * side];
    for i in 0..side {
        for j in 0..side {
            if !stencil[i * side + j] {
                continue;
            }
            out[i * side + j] = (-reach..=reach)
                .any(|di| (-reach..=reach).any(|dj| !inside(i as i64 + di, j as i64 + dj)));
        }
    }
    out
}

pub fn image(
    cell: &Cell2d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
    shape: Shape,
) -> Result<Image> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    let colors = painted(cell);
    let padding = if outline.is_some() { width } else { 0 };
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + padding * 2, h * scale + padding * 2);
    let side = scale + 1;
    let body = if shape == Shape::Square { scale } else { side };
    let stencil = stencil(shape, scale);
    let edge = outline.map(|c| (rim(&stencil, side, width), [c.r, c.g, c.b, c.a]));
    let mut pixels = vec![[0u8; 4]; img_w * img_h];
    for y in 0..h {
        for x in 0..w {
            let rgba = colors[y * w + x];
            if rgba[3] == 0 {
                continue;
            }
            let (x0, y0) = (x * scale + padding, y * scale + padding);
            for i in 0..side {
                for j in 0..side {
                    if !stencil[i * side + j] || y0 + i >= img_h || x0 + j >= img_w {
                        continue;
                    }
                    pixels[(y0 + i) * img_w + x0 + j] = match &edge {
                        Some((rim, ink)) if rim[i * side + j] => *ink,
                        _ if i.max(j) >= body => continue,
                        _ => rgba,
                    };
                }
            }
        }
    }
    Ok(Image {
        width: img_w,
        height: img_h,
        colors: pixels,
    })
}

fn stroke_of(outline: Option<Color>, width: usize) -> String {
    match outline {
        Some(c) => format!("stroke=\"{}\" stroke-width=\"{width}\"", c.to_hex()),
        None => "stroke=\"none\"".to_string(),
    }
}

fn element(shape: Shape, x0: usize, y0: usize, scale: usize, fill: &str, stroke: &str) -> String {
    match shape {
        Shape::Square => format!(
            "<rect x=\"{x0}\" y=\"{y0}\" width=\"{scale}\" height=\"{scale}\" fill=\"{fill}\" {stroke}/>"
        ),
        Shape::Circle => {
            let r = scale / 2;
            let (cx, cy) = (x0 + r, y0 + r);
            format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"{fill}\" {stroke}/>")
        }
        Shape::Diamond => {
            let h = scale / 2;
            let points = [
                (x0 + h, y0),
                (x0 + scale, y0 + h),
                (x0 + h, y0 + scale),
                (x0, y0 + h),
            ];
            let pts: Vec<String> = points.iter().map(|(x, y)| format!("{x},{y}")).collect();
            format!(
                "<polygon points=\"{}\" fill=\"{fill}\" {stroke}/>",
                pts.join(" ")
            )
        }
    }
}

pub fn svg(
    cell: &Cell2d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
    shape: Shape,
) -> String {
    let colors = painted(cell);
    let padding = if outline.is_some() { width } else { 0 };
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + padding * 2, h * scale + padding * 2);
    let stroke = stroke_of(outline, width);
    let mut out = vec![format!(
        "<svg width=\"{img_w}\" height=\"{img_h}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for y in 0..h {
        for x in 0..w {
            let [r, g, b, a] = colors[y * w + x];
            if a == 0 {
                continue;
            }
            let fill = Color::rgba(r, g, b, a).to_hex();
            let (x0, y0) = (x * scale + padding, y * scale + padding);
            out.push(element(shape, x0, y0, scale, &fill, &stroke));
        }
    }
    out.push("</svg>".to_string());
    out.join("\n")
}
