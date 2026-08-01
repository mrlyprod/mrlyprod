use super::faces::Quad;
use crate::space::Vec3;
use std::f32::consts::TAU;

pub const WIDTH: f32 = 320.0;
pub const HEIGHT: f32 = 452.0;

const HALF: f32 = HEIGHT / 2.0;
const MARGIN: f32 = 16.0;
const MAX_LINE: usize = 48;
const MIN_SEGMENTS: usize = 8;
const MAX_SEGMENTS: usize = 24;
const MAX_FACETS: usize = 9600;
const MIN_THICK: f32 = 1.0;
const MAX_THICK: f32 = 80.0;
const BORE_MAX: f32 = 0.45;
const EPS: f32 = 1e-6;

const UP: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
const DOWN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};

// SHAPE

fn fit(cols: usize, rows: usize, segments: usize) -> (usize, usize, usize) {
    let mut cols = cols.min(MAX_LINE);
    let mut rows = rows.min(MAX_LINE);
    let mut segments = segments.clamp(MIN_SEGMENTS, MAX_SEGMENTS);
    while cols * rows * segments > MAX_FACETS && segments > MIN_SEGMENTS {
        segments -= 1;
    }
    while cols * rows * segments > MAX_FACETS {
        if cols >= rows {
            cols -= 1;
        } else {
            rows -= 1;
        }
    }
    (cols, rows, segments)
}

fn depth(thickness: f32) -> f32 {
    let held = if thickness.is_finite() {
        thickness
    } else {
        0.0
    };
    held.clamp(MIN_THICK, MAX_THICK) / 2.0
}

fn point(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x / HALF, y / HALF, z / HALF)
}

fn reach(hx: f32, hy: f32, c: f32, s: f32) -> f32 {
    let tx = if c.abs() > EPS {
        hx / c.abs()
    } else {
        f32::MAX
    };
    let ty = if s.abs() > EPS {
        hy / s.abs()
    } else {
        f32::MAX
    };
    tx.min(ty)
}

// PARTS

fn plate(out: &mut Vec<Quad>, x0: f32, y0: f32, x1: f32, y1: f32, t: f32) {
    if x1 - x0 <= EPS || y1 - y0 <= EPS {
        return;
    }
    out.push(Quad {
        normal: UP,
        verts: [
            point(x0, y0, t),
            point(x1, y0, t),
            point(x1, y1, t),
            point(x0, y1, t),
        ],
    });
    out.push(Quad {
        normal: DOWN,
        verts: [
            point(x0, y0, -t),
            point(x0, y1, -t),
            point(x1, y1, -t),
            point(x1, y0, -t),
        ],
    });
}

fn rim(out: &mut Vec<Quad>, w: f32, h: f32, t: f32) {
    out.push(Quad {
        normal: Vec3::new(1.0, 0.0, 0.0),
        verts: [
            point(w, -h, -t),
            point(w, h, -t),
            point(w, h, t),
            point(w, -h, t),
        ],
    });
    out.push(Quad {
        normal: Vec3::new(-1.0, 0.0, 0.0),
        verts: [
            point(-w, -h, -t),
            point(-w, -h, t),
            point(-w, h, t),
            point(-w, h, -t),
        ],
    });
    out.push(Quad {
        normal: Vec3::new(0.0, 1.0, 0.0),
        verts: [
            point(-w, h, -t),
            point(-w, h, t),
            point(w, h, t),
            point(w, h, -t),
        ],
    });
    out.push(Quad {
        normal: Vec3::new(0.0, -1.0, 0.0),
        verts: [
            point(-w, -h, -t),
            point(w, -h, -t),
            point(w, -h, t),
            point(-w, -h, t),
        ],
    });
}

fn bored(out: &mut Vec<Quad>, at: [f32; 2], span: [f32; 2], bore: f32, t: f32, segments: usize) {
    let step = TAU / segments as f32;
    let ring: Vec<[f32; 2]> = (0..segments)
        .map(|s| {
            let a = step * s as f32;
            [a.cos(), a.sin()]
        })
        .collect();
    for s in 0..segments {
        let [c0, s0] = ring[s];
        let [c1, s1] = ring[(s + 1) % segments];
        let a = [at[0] + bore * c0, at[1] + bore * s0];
        let b = [at[0] + bore * c1, at[1] + bore * s1];
        let ra = reach(span[0], span[1], c0, s0);
        let rb = reach(span[0], span[1], c1, s1);
        let oa = [at[0] + ra * c0, at[1] + ra * s0];
        let ob = [at[0] + rb * c1, at[1] + rb * s1];
        out.push(Quad {
            normal: UP,
            verts: [
                point(oa[0], oa[1], t),
                point(ob[0], ob[1], t),
                point(b[0], b[1], t),
                point(a[0], a[1], t),
            ],
        });
        out.push(Quad {
            normal: DOWN,
            verts: [
                point(a[0], a[1], -t),
                point(b[0], b[1], -t),
                point(ob[0], ob[1], -t),
                point(oa[0], oa[1], -t),
            ],
        });
        let mid = step * (s as f32 + 0.5);
        out.push(Quad {
            normal: Vec3::new(-mid.cos(), -mid.sin(), 0.0),
            verts: [
                point(a[0], a[1], t),
                point(b[0], b[1], t),
                point(b[0], b[1], -t),
                point(a[0], a[1], -t),
            ],
        });
    }
}

// SHEET

pub fn sheet(
    cols: usize,
    rows: usize,
    diameter: f32,
    thickness: f32,
    segments: usize,
) -> Vec<Quad> {
    let (cols, rows, segments) = fit(cols, rows, segments);
    let t = depth(thickness);
    let (w, h) = (WIDTH / 2.0, HALF);
    let mut out = Vec::new();
    rim(&mut out, w, h, t);
    let pitch = [
        (WIDTH - 2.0 * MARGIN) / cols.max(1) as f32,
        (HEIGHT - 2.0 * MARGIN) / rows.max(1) as f32,
    ];
    let wide = if diameter.is_finite() { diameter } else { 0.0 };
    let bore = (wide / 2.0).clamp(0.0, BORE_MAX * pitch[0].min(pitch[1]));
    if cols == 0 || rows == 0 || bore <= EPS {
        plate(&mut out, -w, -h, w, h, t);
        return out;
    }
    plate(&mut out, -w, -h, w, -h + MARGIN, t);
    plate(&mut out, -w, h - MARGIN, w, h, t);
    plate(&mut out, -w, -h + MARGIN, -w + MARGIN, h - MARGIN, t);
    plate(&mut out, w - MARGIN, -h + MARGIN, w, h - MARGIN, t);
    let span = [pitch[0] / 2.0, pitch[1] / 2.0];
    for j in 0..rows {
        for i in 0..cols {
            let at = [
                -w + MARGIN + (i as f32 + 0.5) * pitch[0],
                -h + MARGIN + (j as f32 + 0.5) * pitch[1],
            ];
            bored(&mut out, at, span, bore, t, segments);
        }
    }
    out
}

pub fn sheet_edges(thickness: f32) -> Vec<[Vec3; 2]> {
    let t = depth(thickness);
    let (w, h) = (WIDTH / 2.0, HALF);
    let mut out = Vec::new();
    for z in [-t, t] {
        let ring = [
            point(-w, -h, z),
            point(w, -h, z),
            point(w, h, z),
            point(-w, h, z),
        ];
        for k in 0..4 {
            out.push([ring[k], ring[(k + 1) % 4]]);
        }
    }
    for [x, y] in [[-w, -h], [w, -h], [w, h], [-w, h]] {
        out.push([point(x, y, -t), point(x, y, t)]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Pack;

    fn buffer(quads: &[Quad]) -> Vec<f32> {
        let mut pack = Pack::new();
        for quad in quads {
            pack.quad(quad.verts, quad.normal);
        }
        pack.buffer()
    }

    #[test]
    fn the_buffer_holds_its_header() {
        let quads = sheet(6, 8, 24.0, 8.0, 12);
        let buf = buffer(&quads);
        assert_eq!(buf[0] as usize, quads.len() * 36);
        assert_eq!(buf[1], 0.0);
        assert_eq!(buf.len(), 2 + quads.len() * 36);
        assert_eq!(buf[0] as usize % 18, 0);
        assert!(quads.len() > 6 * 8 * 12);
    }
    #[test]
    fn the_plate_keeps_the_sheet_law() {
        let quads = sheet(4, 6, 20.0, 6.0, 8);
        let mut wide = 0.0f32;
        let mut tall = 0.0f32;
        for quad in &quads {
            for v in quad.verts {
                wide = wide.max(v.x.abs());
                tall = tall.max(v.y.abs());
                assert!(v.z.abs() <= 1.0);
            }
        }
        assert!((tall - 1.0).abs() < 1e-6);
        assert!((wide - WIDTH / HEIGHT).abs() < 1e-6);
    }
    #[test]
    fn the_normals_stay_flat_and_true() {
        for quad in sheet(3, 4, 30.0, 10.0, 8) {
            let n = quad.normal;
            assert!((n.dot(n) - 1.0).abs() < 1e-5);
            for k in 0..3 {
                let e = quad.verts[k + 1] - quad.verts[0];
                assert!(n.dot(e).abs() < 1e-5);
            }
        }
    }
    #[test]
    fn the_holes_open_through_the_plate() {
        let solid = sheet(0, 0, 0.0, 8.0, 12);
        let holed = sheet(5, 7, 24.0, 8.0, 12);
        assert_eq!(solid.len(), 6);
        assert!(holed.len() > solid.len());
    }
    #[test]
    fn the_params_clamp_to_the_budget() {
        let (cols, rows, segments) = fit(4000, 4000, 999);
        assert!(cols <= MAX_LINE && rows <= MAX_LINE);
        assert!((MIN_SEGMENTS..=MAX_SEGMENTS).contains(&segments));
        assert!(cols * rows * segments <= MAX_FACETS);
        let buf = buffer(&sheet(4000, 4000, 999.0, 999.0, 999));
        assert!(buf.len() * 4 < 5_000_000);
        let (_, _, tiny) = fit(2, 2, 0);
        assert_eq!(tiny, MIN_SEGMENTS);
    }
    #[test]
    fn the_degenerate_sheets_hold_together() {
        assert_eq!(sheet(0, 8, 20.0, 8.0, 12).len(), 6);
        assert_eq!(sheet(8, 0, 20.0, 8.0, 12).len(), 6);
        assert_eq!(sheet(8, 8, 0.0, 8.0, 12).len(), 6);
        assert!(!sheet(8, 8, 4000.0, 0.0, 12).is_empty());
        assert!(!sheet(8, 8, -4.0, -4.0, 12).is_empty());
        assert!(!sheet(1, 1, f32::NAN, f32::INFINITY, 12).is_empty());
    }
    #[test]
    fn the_frame_wires_the_box() {
        let wires = sheet_edges(8.0);
        assert_eq!(wires.len(), 12);
        for [a, b] in wires {
            assert!(a != b);
        }
    }
}
