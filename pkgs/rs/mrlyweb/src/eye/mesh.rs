use super::flat::Flat;
use mrlycore::colors::board;
use mrlycore::trig;
use std::f64::consts::TAU;

const NEAR: f64 = 0.25;
const FOCAL: f64 = 1.2;
const SLOTS: usize = 24;
const TURN: f64 = trig::N as f64;
const TRI_STRIDE: usize = 6;
const LINE_STRIDE: usize = 8;

// CAMERA

fn wave(angle: f64) -> (f64, f64) {
    let step = ((angle * TURN / TAU).round() as i64).rem_euclid(TURN as i64);
    let (c, s) = trig::unit(step as usize);
    (c as f64, s as f64)
}

fn aim(route: &str, u: &mut [f32]) {
    let rigs = crate::registry::rigs();
    let Some(rig) = rigs.iter().find(|rig| rig.route == route) else {
        return;
    };
    let rad = (TAU / TURN) as f32;
    u[13] = rig.yaw as f32 * rad;
    u[14] = rig.pitch as f32 * rad;
    u[15] = rig.dist as f32 / 4.0;
    u[16] = rig.pan[0] as f32 / 16.0;
    u[17] = rig.pan[1] as f32 / 16.0;
    u[18] = if rig.ortho { 1.0 } else { 0.0 };
}

// LENS

struct Lens {
    spin: (f64, f64),
    view: [[f64; 3]; 3],
    beam: [f64; 3],
    base: [f64; 3],
    bands: f64,
    dist: f64,
    pan: [f64; 2],
    ortho: bool,
    aspect: f64,
    alpha: f64,
    width: f64,
    height: f64,
}

impl Lens {
    fn read(u: &[f32], width: usize, height: usize) -> Lens {
        let at = |i: usize| u[i] as f64;
        let (cy, sy) = wave(at(13));
        let (cp, sp) = wave(at(14));
        let (cx, sx) = wave(at(20));
        let (cl, sl) = wave(at(21));
        Lens {
            spin: wave(at(12)),
            view: [
                [cy, -sp * sy, cp * sy],
                [0.0, cp, sp],
                [-sy, -sp * cy, cp * cy],
            ],
            beam: [sx * cl, sl, cx * cl],
            base: [at(8), at(9), at(10)],
            bands: at(11),
            dist: at(15),
            pan: [at(16), at(17)],
            ortho: at(18) > 0.5,
            aspect: at(1) / at(0).max(1.0),
            alpha: at(19),
            width: width as f64,
            height: height as f64,
        }
    }

    fn spun(&self, v: [f64; 3]) -> [f64; 3] {
        let (c, s) = self.spin;
        [c * v[0] + s * v[2], v[1], -s * v[0] + c * v[2]]
    }

    fn cast(&self, v: [f64; 3]) -> [f64; 3] {
        let m = &self.view;
        [
            m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2],
            m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2],
            m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2],
        ]
    }

    fn project(&self, e: [f64; 3]) -> [f64; 4] {
        let far = self.dist + 4.0;
        let mut a = 2.0 * FOCAL;
        let mut q = far / (far - NEAR);
        let mut w = self.dist - e[2];
        if self.ortho {
            a = 2.0 * FOCAL / self.dist;
            q = 1.0 / (far - NEAR);
            w = 1.0;
        }
        [
            a * (e[0] + self.pan[0]) * self.aspect,
            a * (e[1] + self.pan[1]),
            q * (self.dist - NEAR - e[2]),
            w,
        ]
    }

    fn facing(&self, n: [f64; 3], e: [f64; 3]) -> f64 {
        if self.ortho {
            return n[2];
        }
        -(n[0] * e[0] + n[1] * e[1] + n[2] * (e[2] - self.dist))
    }

    fn shade(&self, n: [f64; 3]) -> [f64; 3] {
        let lit = (n[0] * self.beam[0] + n[1] * self.beam[1] + n[2] * self.beam[2] + 1.0) * 0.5;
        let band = (lit * self.bands)
            .floor()
            .clamp(0.0, (self.bands - 1.0).max(0.0));
        let span = (self.bands - 1.0).max(1.0);
        let t = (64.0 + (191.0 * band / span).floor()) / 255.0;
        [self.base[0] * t, self.base[1] * t, self.base[2] * t]
    }

    fn screen(&self, clip: [f64; 4]) -> [f64; 3] {
        let ndc = [clip[0] / clip[3], clip[1] / clip[3], clip[2] / clip[3]];
        [
            (ndc[0] * 0.5 + 0.5) * self.width,
            (0.5 - ndc[1] * 0.5) * self.height,
            ndc[2],
        ]
    }
}

// SHAPE

struct Face {
    points: [[f64; 3]; 3],
    color: [f64; 3],
    face: f64,
}

fn triple(buf: &[f32], at: usize) -> [f64; 3] {
    [buf[at] as f64, buf[at + 1] as f64, buf[at + 2] as f64]
}

fn faces(lens: &Lens, tris: &[f32]) -> Vec<Face> {
    let mut out = Vec::new();
    let span = TRI_STRIDE * 3;
    let mut at = 0;
    while at + span <= tris.len() {
        let mut points = [[0.0f64; 3]; 3];
        let mut clipped = false;
        let mut color = [0.0f64; 3];
        let mut face = 0.0;
        for (v, point) in points.iter_mut().enumerate() {
            let head = at + v * TRI_STRIDE;
            let world = lens.spun(triple(tris, head));
            let n = lens.spun(triple(tris, head + 3));
            let e = lens.cast(world);
            let clip = lens.project(e);
            if clip[3] <= 0.0 {
                clipped = true;
                break;
            }
            *point = lens.screen(clip);
            if v == 0 {
                color = lens.shade(n);
                face = lens.facing(lens.cast(n), e);
            }
        }
        if !clipped {
            out.push(Face {
                points,
                color,
                face,
            });
        }
        at += span;
    }
    out
}

fn strokes(flat: &mut Flat, lens: &Lens, lines: &[f32]) {
    let span = LINE_STRIDE * 2;
    let mut at = 0;
    while at + span <= lines.len() {
        let mut ends = [[0.0f64; 3]; 2];
        let mut clipped = false;
        for (v, end) in ends.iter_mut().enumerate() {
            let head = at + v * LINE_STRIDE;
            let raw = triple(lines, head);
            let world = if lines[head + 3] > 0.5 {
                lens.spun(raw)
            } else {
                raw
            };
            let mut clip = lens.project(lens.cast(world));
            clip[2] -= 0.005 * clip[3];
            if clip[3] <= 0.0 {
                clipped = true;
                break;
            }
            *end = lens.screen(clip);
        }
        if !clipped {
            let color = triple(lines, at + 4);
            flat.line(ends[0], ends[1], color, lines[at + 7] as f64, true, true);
        }
        at += span;
    }
}

// PAINT

pub(crate) fn paint(
    route: &str,
    u: &mut [f32],
    geometry: &[f32],
    width: usize,
    height: usize,
) -> Vec<[u8; 4]> {
    let mut flat = Flat::new(width, height, board(false));
    if u.len() < SLOTS || geometry.len() < 2 {
        return flat.pixels;
    }
    aim(route, u);
    let lens = Lens::read(u, width, height);
    let room = geometry.len() - 2;
    let tri_floats = (geometry[0].max(0.0) as usize).min(room);
    let line_floats = (geometry[1].max(0.0) as usize).min(room - tri_floats);
    let tris = &geometry[2..2 + tri_floats];
    let lines = &geometry[2 + tri_floats..2 + tri_floats + line_floats];
    let shape = faces(&lens, tris);
    let alpha = lens.alpha;
    if alpha >= 1.0 {
        for f in &shape {
            flat.tri(f.points, f.color, 1.0, true, true);
        }
    } else {
        for f in shape.iter().filter(|f| f.face <= 0.0) {
            flat.tri(f.points, f.color, alpha, true, false);
        }
        for f in shape.iter().filter(|f| f.face > 0.0) {
            flat.tri(f.points, f.color, alpha, true, false);
        }
    }
    strokes(&mut flat, &lens, lines);
    flat.pixels
}

#[cfg(test)]
mod tests {
    use super::{paint, wave};
    use crate::eye::frame;
    use crate::registry::boot;
    use mrlycore::colors::board;
    use mrlycore::json;
    use mrlyos::kernel::Call;
    use std::collections::HashSet;

    const SIDE: usize = 128;

    fn shot(route: &str, tweaks: Vec<Call>) -> Vec<[u8; 4]> {
        let mut os = boot("full");
        os.open(route).unwrap();
        for call in tweaks {
            assert!(os.call(call).ok);
        }
        let pixels = frame(&os, route, SIDE, SIDE).unwrap();
        assert_eq!(pixels.len(), SIDE * SIDE);
        pixels
    }

    fn shades(pixels: &[[u8; 4]]) -> usize {
        pixels.iter().collect::<HashSet<_>>().len()
    }

    #[test]
    fn the_opaque_pass_shades_the_carpet() {
        let pixels = shot("three", vec![]);
        assert!(shades(&pixels) > 1, "three drew one flat colour");
        assert!(pixels.iter().all(|p| p[3] == 255));
    }

    #[test]
    fn the_line_pass_draws_the_network() {
        let pixels = shot("graph", vec![]);
        assert!(shades(&pixels) > 1, "graph drew one flat colour");
    }

    #[test]
    fn the_glass_pass_lays_two_coats() {
        let dim = Call::new("three.set", json!({ "key": "alpha", "value": 96 }));
        let glass = shot("three", vec![dim]);
        assert!(shades(&glass) > 1, "glass three drew one flat colour");
        assert_ne!(glass, shot("three", vec![]));
    }

    #[test]
    fn the_clear_is_the_board_and_never_the_void() {
        let mut u = vec![0.0f32; 24];
        u[0] = 8.0;
        u[1] = 8.0;
        u[7] = 1.0;
        let pixels = paint("bang", &mut u, &[0.0, 0.0], 8, 8);
        assert_eq!(pixels, vec![board(false); 64]);
        assert_eq!(u[15], 3.0);
        assert_eq!(u[18], 0.0);
        assert_eq!(paint("three", &mut u, &[0.0, 0.0], 8, 8).len(), 64);
        assert_eq!(u[18], 1.0);
    }

    #[test]
    fn the_table_survives_a_negative_angle() {
        let back = wave(-super::TAU / 4.0);
        assert!(back.0.abs() < 1e-6);
        assert_eq!(back.1, -1.0);
    }
}
