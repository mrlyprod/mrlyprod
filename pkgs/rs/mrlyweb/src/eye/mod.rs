use crate::registry;
use flat::Flat;
use mrlycore::colors::board;
use mrlycore::Json;
use mrlyos::kernel::Os;

mod flat;
mod fragment;
mod mesh;

const STRIDE: usize = 10;

/// Renders a route's canvas as rgba pixels, no browser and no adapter, or None where the route draws nothing.
///
/// ```
/// let mut os = mrlyweb::registry::boot("full");
/// os.open("mandelbrot").unwrap();
/// let pixels = mrlyweb::eye::frame(&os, "mandelbrot", 32, 32).unwrap();
/// assert_eq!(pixels.len(), 32 * 32);
/// assert!(mrlyweb::eye::frame(&os, "calculator", 32, 32).is_none());
/// ```
pub fn frame(os: &Os, route: &str, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
    if width == 0 || height == 0 {
        return None;
    }
    let shade = os
        .read(&format!("{route}/shade"), None)
        .filter(|s| s.as_object().is_some());
    if let Some(shade) = shade {
        let program = shade["program"].as_str()?.to_string();
        let mut u = os.floats(&format!("{route}/uniforms"))?;
        if u.len() < 2 {
            return None;
        }
        u[0] = width as f32;
        u[1] = height as f32;
        if program == "mesh" {
            let geometry = os.floats(&format!("{route}/geometry"))?;
            return Some(mesh::paint(route, &mut u, &geometry, width, height));
        }
        return fragment::paint(&program, &u, width, height);
    }
    let cells = os
        .read(&format!("{route}/cells"), None)
        .filter(|c| c.as_object().is_some());
    if let Some(cells) = cells {
        return dress(route, &cells, width, height);
    }
    carve(&os.floats(&format!("{route}/tris"))?, width, height)
}

fn dress(route: &str, cells: &Json, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
    let ids = cells["ids"].as_array()?;
    let rows = ids.len();
    let cols = ids.first()?.as_array()?.len();
    if rows == 0 || cols == 0 {
        return None;
    }
    let tile = (width / cols).min(height / rows).max(1);
    let raster = registry::raster(route, cells, tile, false)?;
    let source = raster.composite().cell.colors.unwrap_or_default();
    let (rw, rh) = (raster.width(), raster.height());
    let mut pixels = vec![raster.background; width * height];
    let ox = (width as i64 - rw as i64) / 2;
    let oy = (height as i64 - rh as i64) / 2;
    for y in 0..rh {
        let fy = oy + y as i64;
        if fy < 0 || fy >= height as i64 {
            continue;
        }
        for x in 0..rw {
            let fx = ox + x as i64;
            if fx < 0 || fx >= width as i64 {
                continue;
            }
            pixels[fy as usize * width + fx as usize] = source[y * rw + x];
        }
    }
    Some(pixels)
}

fn carve(buf: &[f32], width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
    if buf.len() <= STRIDE {
        return None;
    }
    let mut min = [f64::MAX; 2];
    let mut max = [f64::MIN; 2];
    let mut i = 1;
    while i + STRIDE <= buf.len() {
        for p in 0..3 {
            for axis in 0..2 {
                let v = buf[i + 2 * p + axis] as f64;
                min[axis] = min[axis].min(v);
                max[axis] = max[axis].max(v);
            }
        }
        i += STRIDE;
    }
    let span = (max[0] - min[0]).max(max[1] - min[1]).max(1.0);
    let scale = width.min(height) as f64 * 0.9 / span;
    let center = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0];
    let mut flat = Flat::new(width, height, board(false));
    let mut i = 1;
    while i + STRIDE <= buf.len() {
        let mut points = [[0.0f64; 3]; 3];
        for p in 0..3 {
            points[p][0] = width as f64 / 2.0 + (buf[i + 2 * p] as f64 - center[0]) * scale;
            points[p][1] = height as f64 / 2.0 + (buf[i + 2 * p + 1] as f64 - center[1]) * scale;
        }
        let color = [buf[i + 6] as f64, buf[i + 7] as f64, buf[i + 8] as f64];
        flat.tri(points, color, buf[i + 9] as f64, false, false);
        i += STRIDE;
    }
    Some(flat.pixels)
}
