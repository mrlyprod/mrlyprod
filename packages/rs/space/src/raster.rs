use crate::view::{apply, turn, unit};
use crate::{Mesh, View, mesh};
use mrly_error::{Result, value_error};
use mrly_image::Image;
use mrly_three::Cell3d;

const SUB: i64 = 256;
const BAND: usize = 16;
const EMPTY: u32 = u32::MAX;

struct Tri {
    v: [(i64, i64); 3],
    q: [f64; 3],
    face: u32,
    rows: (usize, usize),
    cols: (usize, usize),
}

struct Scene {
    tris: Vec<Tri>,
    light: Vec<f64>,
    eye: Option<f64>,
    radius: f64,
}

fn span(low: i64, high: i64, count: usize) -> Option<(usize, usize)> {
    let first = (low - SUB / 2 + SUB - 1).div_euclid(SUB).max(0);
    let last = (high - SUB / 2).div_euclid(SUB).min(count as i64 - 1);
    (first <= last).then_some((first as usize, last as usize))
}

fn scene(mesh: &Mesh, view: &View) -> Result<Scene> {
    let light = view.check()?;
    mesh.check()?;
    if u32::try_from(mesh.faces.len()).is_err() {
        return value_error("a mesh holds at most 2^32 - 1 faces.");
    }
    let rot = view.rotation();
    let s = view.samples;
    let (sw, sh) = (view.width * s, view.height * s);
    let points: Vec<[f64; 3]> = mesh
        .verts
        .iter()
        .map(|v| apply(&rot, [v.x as f64, v.y as f64, v.z as f64]))
        .collect();
    let radius = points
        .iter()
        .map(|p| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
        .fold(0.0, f64::max);
    let mut out = Scene {
        tris: Vec::new(),
        light: vec![0.0; mesh.faces.len()],
        eye: None,
        radius,
    };
    if radius == 0.0 {
        return Ok(out);
    }
    let half = (view.width.min(view.height) * s) as f64 / 2.0 * view.zoom;
    let (cx, cy) = (sw as f64 / 2.0, sh as f64 / 2.0);
    let lens = (view.fov > 0.0).then(|| {
        let (sin, cos) = turn(view.fov / 2.0);
        (radius / sin, half * cos / sin)
    });
    out.eye = lens.map(|(eye, _)| eye);
    let snap = |x: f64| (x * SUB as f64).round() as i64;
    let screen: Vec<(i64, i64, f64)> = points
        .iter()
        .map(|p| {
            let (scale, q) = match lens {
                None => (half / radius, p[2]),
                Some((eye, focal)) => (focal / (eye - p[2]), 1.0 / (eye - p[2])),
            };
            (snap(cx + scale * p[0]), snap(cy - scale * p[1]), q)
        })
        .collect();
    for (face, (&[a, b, c], normal)) in mesh.faces.iter().zip(&mesh.normals).enumerate() {
        let Some(n) = unit([normal.x as f64, normal.y as f64, normal.z as f64]) else {
            continue;
        };
        let n = apply(&rot, n);
        let p = points[a];
        let facing = match out.eye {
            None => n[2],
            Some(eye) => -n[0] * p[0] - n[1] * p[1] + n[2] * (eye - p[2]),
        };
        if facing <= 0.0 {
            continue;
        }
        out.light[face] = 0.65 + 0.35 * (n[0] * light[0] + n[1] * light[1] + n[2] * light[2]);
        let [pa, mut pb, mut pc] = [screen[a], screen[b], screen[c]];
        let area = (pb.0 - pa.0) * (pc.1 - pa.1) - (pb.1 - pa.1) * (pc.0 - pa.0);
        if area == 0 {
            continue;
        }
        if area < 0 {
            (pb, pc) = (pc, pb);
        }
        let cols = span(pa.0.min(pb.0).min(pc.0), pa.0.max(pb.0).max(pc.0), sw);
        let rows = span(pa.1.min(pb.1).min(pc.1), pa.1.max(pb.1).max(pc.1), sh);
        let (Some(cols), Some(rows)) = (cols, rows) else {
            continue;
        };
        let area = area.abs() as f64;
        let (db, dc) = (pb.2 - pa.2, pc.2 - pa.2);
        let qx = (db * (pc.1 - pa.1) as f64 - dc * (pb.1 - pa.1) as f64) / area;
        let qy = (dc * (pb.0 - pa.0) as f64 - db * (pc.0 - pa.0) as f64) / area;
        out.tris.push(Tri {
            v: [(pa.0, pa.1), (pb.0, pb.1), (pc.0, pc.1)],
            q: [pa.2, qx, qy],
            face: face as u32,
            rows,
            cols,
        });
    }
    Ok(out)
}

fn fill(tri: &Tri, top: usize, bottom: usize, width: usize, depth: &mut [f64], faces: &mut [u32]) {
    let [a, b, c] = tri.v;
    let edges = [(b, c), (c, a), (a, b)].map(|(u, v)| {
        let (dx, dy) = (v.0 - u.0, v.1 - u.1);
        let bias = if dy > 0 || (dy == 0 && dx < 0) { 0 } else { -1 };
        (u, dx, dy, bias)
    });
    let first = tri.cols.0 as i64;
    let x0 = first * SUB + SUB / 2;
    for row in tri.rows.0.max(top)..=tri.rows.1.min(bottom - 1) {
        let y = row as i64 * SUB + SUB / 2;
        let (mut lo, mut hi) = (first, tri.cols.1 as i64);
        for (u, dx, dy, bias) in edges {
            let e = dx * (y - u.1) - dy * (x0 - u.0) + bias;
            let step = dy * SUB;
            match step.signum() {
                1 => hi = hi.min(first + e.div_euclid(step)),
                -1 => lo = lo.max(first + (-e - step - 1).div_euclid(-step)),
                _ if e < 0 => hi = lo - 1,
                _ => {}
            }
        }
        let line = (row - top) * width;
        for col in lo..=hi {
            let x = col * SUB + SUB / 2;
            let q = tri.q[0] + tri.q[1] * (x - a.0) as f64 + tri.q[2] * (y - a.1) as f64;
            let at = line + col as usize;
            if q > depth[at] {
                depth[at] = q;
                faces[at] = tri.face;
            }
        }
    }
}

fn raster(scene: &Scene, view: &View, shade: impl Fn(u32, f64) -> [u8; 4]) -> Image {
    let s = view.samples;
    let (width, height) = (view.width, view.height);
    let sw = width * s;
    let tall = BAND * s;
    let mut bands: Vec<Vec<usize>> = vec![Vec::new(); height.div_ceil(BAND)];
    for (i, tri) in scene.tris.iter().enumerate() {
        for band in &mut bands[tri.rows.0 / tall..=tri.rows.1 / tall] {
            band.push(i);
        }
    }
    let ground = view.background;
    let ground = [ground.r, ground.g, ground.b, ground.a];
    let mut colors = Vec::with_capacity(width * height);
    let mut depth = vec![0.0; sw * tall];
    let mut faces = vec![EMPTY; sw * tall];
    for (band, list) in bands.iter().enumerate() {
        depth.fill(f64::NEG_INFINITY);
        faces.fill(EMPTY);
        let top = band * tall;
        let rows = (height - band * BAND).min(BAND);
        for &i in list {
            fill(
                &scene.tris[i],
                top,
                top + rows * s,
                sw,
                &mut depth,
                &mut faces,
            );
        }
        for y in 0..rows {
            for x in 0..width {
                let mut sum = [0u64; 4];
                for sy in y * s..(y + 1) * s {
                    for at in sy * sw + x * s..sy * sw + (x + 1) * s {
                        let [r, g, b, a] = match faces[at] {
                            EMPTY => ground,
                            face => shade(face, depth[at]),
                        };
                        let a = a as u64;
                        sum[0] += r as u64 * a;
                        sum[1] += g as u64 * a;
                        sum[2] += b as u64 * a;
                        sum[3] += a;
                    }
                }
                let n = (s * s) as u64;
                let round = |x: u64, y: u64| {
                    if y == 0 {
                        0
                    } else {
                        ((2 * x + y) / (2 * y)) as u8
                    }
                };
                colors.push([
                    round(sum[0], sum[3]),
                    round(sum[1], sum[3]),
                    round(sum[2], sum[3]),
                    round(sum[3], n),
                ]);
            }
        }
    }
    Image {
        width,
        height,
        colors,
    }
}

pub fn draw(mesh: &Mesh, view: &View) -> Result<Image> {
    let scene = scene(mesh, view)?;
    let ink = [view.ink.r, view.ink.g, view.ink.b];
    let shades: Vec<[u8; 4]> = (0..mesh.faces.len())
        .map(|face| {
            let base = match &mesh.colors {
                Some(colors) => [colors[face][0], colors[face][1], colors[face][2]],
                None => ink,
            };
            let t = scene.light[face];
            let [r, g, b] = base.map(|c| (c as f64 * t).round() as u8);
            [r, g, b, 255]
        })
        .collect();
    Ok(raster(&scene, view, |face, _| shades[face as usize]))
}

pub fn depth(mesh: &Mesh, view: &View) -> Result<Image> {
    let scene = scene(mesh, view)?;
    let (eye, radius) = (scene.eye, scene.radius);
    Ok(raster(&scene, view, |_, q| {
        let z = match eye {
            None => q,
            Some(eye) => eye - 1.0 / q,
        };
        let g = (255.0 * (z + radius) / (2.0 * radius)).round() as u8;
        [g, g, g, 255]
    }))
}

pub fn render(cell: &Cell3d, view: &View) -> Result<Image> {
    draw(&mesh(cell), view)
}
