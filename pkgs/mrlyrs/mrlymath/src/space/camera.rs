use super::vec::{Mat3, Vec3};
use mrlycore::trig;
use serde_json::{json, Value as Json};

pub const TURN: i64 = trig::N as i64;
pub const NEAR: f32 = 0.25;
pub const FAR: f32 = 4.0;
pub const PITCH_MAX: i64 = 56;
pub const DIST_MIN: i64 = 8;
pub const DIST_MAX: i64 = 32;
pub const PAN_MAX: i64 = 32;
const FOCAL: f32 = 1.2;

pub fn view(yaw: i64, pitch: i64) -> Mat3 {
    Mat3::pitch(pitch) * Mat3::yaw(-yaw)
}

pub fn beam(yaw: i64, pitch: i64) -> Vec3 {
    let p = Mat3::pitch(-pitch).apply(Vec3::new(0.0, 0.0, 1.0));
    Mat3::yaw(yaw).apply(p)
}

pub fn project(u: Vec3, dist: f32, size: f32, focal: f32) -> Option<[f32; 3]> {
    let depth = dist - u.z;
    if depth < NEAR {
        return None;
    }
    let half = size / 2.0;
    Some([
        half + focal * u.x / depth,
        half - focal * u.y / depth,
        depth,
    ])
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub yaw: i64,
    pub pitch: i64,
    pub dist: f32,
    pub pan: [f32; 2],
    pub ortho: bool,
}

impl Camera {
    pub fn view(&self) -> Mat3 {
        view(self.yaw, self.pitch)
    }
    pub fn facing(&self, n: Vec3, u: Vec3) -> bool {
        if self.ortho {
            n.z > 0.0
        } else {
            n.x * u.x + n.y * u.y + n.z * (u.z - self.dist) < 0.0
        }
    }
    pub fn project(&self, u: Vec3, size: f32) -> Option<[f32; 3]> {
        let depth = self.dist - u.z;
        if depth < NEAR {
            return None;
        }
        let div = if self.ortho { self.dist } else { depth };
        let half = size / 2.0;
        let focal = size * FOCAL;
        Some([
            half + focal * (u.x + self.pan[0]) / div,
            half - focal * (u.y + self.pan[1]) / div,
            depth,
        ])
    }
    pub fn matrix(&self) -> [f32; 16] {
        let r = self.view().m;
        let far = self.dist + FAR;
        let a = if self.ortho {
            2.0 * FOCAL / self.dist
        } else {
            2.0 * FOCAL
        };
        let mut m = [0.0f32; 16];
        for c in 0..3 {
            m[c * 4] = a * r[0][c];
            m[c * 4 + 1] = a * r[1][c];
        }
        m[12] = a * self.pan[0];
        m[13] = a * self.pan[1];
        if self.ortho {
            let q = 1.0 / (far - NEAR);
            for c in 0..3 {
                m[c * 4 + 2] = -q * r[2][c];
            }
            m[14] = (self.dist - NEAR) * q;
            m[15] = 1.0;
        } else {
            let q = far / (far - NEAR);
            for c in 0..3 {
                m[c * 4 + 2] = -q * r[2][c];
                m[c * 4 + 3] = -r[2][c];
            }
            m[14] = (self.dist - NEAR) * q;
            m[15] = self.dist;
        }
        m
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rig {
    pub yaw: i64,
    pub pitch: i64,
    pub dist: i64,
    pub pan: [i64; 2],
    pub ortho: bool,
}

impl Default for Rig {
    fn default() -> Rig {
        Rig::new()
    }
}

impl Rig {
    pub fn new() -> Rig {
        Rig {
            yaw: 0,
            pitch: 0,
            dist: 12,
            pan: [0, 0],
            ortho: false,
        }
    }
    pub fn orbit(&mut self, dyaw: i64, dpitch: i64) {
        self.yaw = (self.yaw + dyaw).rem_euclid(TURN);
        self.pitch = (self.pitch + dpitch).clamp(-PITCH_MAX, PITCH_MAX);
    }
    pub fn zoom(&mut self, d: i64) {
        self.dist = (self.dist + d).clamp(DIST_MIN, DIST_MAX);
    }
    pub fn pan(&mut self, dx: i64, dy: i64) {
        self.pan[0] = (self.pan[0] + dx).clamp(-PAN_MAX, PAN_MAX);
        self.pan[1] = (self.pan[1] + dy).clamp(-PAN_MAX, PAN_MAX);
    }
    pub fn view(&mut self, name: &str) -> bool {
        let (yaw, pitch) = match name {
            "front" => (0, 0),
            "top" => (0, TURN / 4),
            "iso" => (32, 20),
            _ => return false,
        };
        self.yaw = yaw;
        self.pitch = pitch;
        self.pan = [0, 0];
        true
    }
    pub fn camera(&self) -> Camera {
        Camera {
            yaw: self.yaw,
            pitch: self.pitch,
            dist: self.dist as f32 / 4.0,
            pan: [self.pan[0] as f32 / 16.0, self.pan[1] as f32 / 16.0],
            ortho: self.ortho,
        }
    }
    pub fn to_json(&self) -> Json {
        json!({
            "yaw": self.yaw,
            "pitch": self.pitch,
            "dist": self.dist,
            "pan": self.pan,
            "ortho": self.ortho,
        })
    }
    pub fn load(&mut self, value: &Json) {
        if let Some(yaw) = value["yaw"].as_i64() {
            self.yaw = yaw.rem_euclid(TURN);
        }
        if let Some(pitch) = value["pitch"].as_i64() {
            if (-TURN / 4..=TURN / 4).contains(&pitch) {
                self.pitch = pitch;
            }
        }
        if let Some(dist) = value["dist"].as_i64() {
            if (DIST_MIN..=DIST_MAX).contains(&dist) {
                self.dist = dist;
            }
        }
        for i in 0..2 {
            if let Some(p) = value["pan"][i].as_i64() {
                if (-PAN_MAX..=PAN_MAX).contains(&p) {
                    self.pan[i] = p;
                }
            }
        }
        if let Some(ortho) = value["ortho"].as_bool() {
            self.ortho = ortho;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_origin_projects_to_center() {
        let p = project(Vec3::new(0.0, 0.0, 0.0), 3.0, 96.0, 115.2).unwrap();
        assert_eq!(p[0], 48.0);
        assert_eq!(p[1], 48.0);
        assert_eq!(p[2], 3.0);
    }
    #[test]
    fn points_behind_the_eye_clip() {
        assert!(project(Vec3::new(0.0, 0.0, 3.0), 3.0, 96.0, 115.2).is_none());
    }
    #[test]
    fn the_view_undoes_the_orbit() {
        let cam = Mat3::yaw(37) * Mat3::pitch(-21);
        let eye = cam.apply(Vec3::new(0.0, 0.0, 3.0));
        let u = view(37, 21).apply(eye);
        assert!(u.x.abs() < 1e-5 && u.y.abs() < 1e-5 && (u.z - 3.0).abs() < 1e-5);
    }
    #[test]
    fn positive_pitch_looks_from_above() {
        let cam = Mat3::yaw(0) * Mat3::pitch(-21);
        assert!(cam.apply(Vec3::new(0.0, 0.0, 3.0)).y > 0.0);
        assert!(beam(0, 21).y > 0.0);
    }
    #[test]
    fn beams_are_unit_length() {
        for (y, p) in [(0i64, 0i64), (40, 28), (200, -56), (128, 56)] {
            let b = beam(y, p);
            assert!((b.dot(b) - 1.0).abs() < 1e-5);
        }
    }
    #[test]
    fn the_default_beam_faces_the_camera() {
        assert_eq!(beam(0, 0), Vec3::new(0.0, 0.0, 1.0));
    }
    #[test]
    fn the_camera_matches_the_free_project() {
        let cam = Camera {
            yaw: 0,
            pitch: 0,
            dist: 3.0,
            pan: [0.0, 0.0],
            ortho: false,
        };
        for v in [
            Vec3::new(0.3, -0.4, 0.5),
            Vec3::new(-0.9, 0.1, -0.7),
            Vec3::new(0.0, 0.0, 0.0),
        ] {
            assert_eq!(cam.project(v, 96.0), project(v, 3.0, 96.0, 96.0 * 1.2));
        }
    }
    #[test]
    fn ortho_projects_parallel() {
        let cam = Camera {
            yaw: 0,
            pitch: 0,
            dist: 3.0,
            pan: [0.0, 0.0],
            ortho: true,
        };
        let near = cam.project(Vec3::new(0.5, 0.5, 0.9), 96.0).unwrap();
        let far = cam.project(Vec3::new(0.5, 0.5, -0.9), 96.0).unwrap();
        assert_eq!(near[0], far[0]);
        assert_eq!(near[1], far[1]);
        assert!(near[2] < far[2]);
    }
    #[test]
    fn pan_shifts_the_screen() {
        let center = Camera {
            yaw: 0,
            pitch: 0,
            dist: 3.0,
            pan: [0.0, 0.0],
            ortho: false,
        };
        let panned = Camera {
            pan: [0.5, 0.25],
            ..center
        };
        let a = center.project(Vec3::new(0.0, 0.0, 0.0), 96.0).unwrap();
        let b = panned.project(Vec3::new(0.0, 0.0, 0.0), 96.0).unwrap();
        assert!(b[0] > a[0]);
        assert!(b[1] < a[1]);
    }
    #[test]
    fn facing_follows_the_lens() {
        let persp = Camera {
            yaw: 0,
            pitch: 0,
            dist: 3.0,
            pan: [0.0, 0.0],
            ortho: false,
        };
        let ortho = Camera {
            ortho: true,
            ..persp
        };
        let toward = Vec3::new(0.0, 0.0, 1.0);
        let away = Vec3::new(0.0, 0.0, -1.0);
        let u = Vec3::new(0.0, 0.0, 1.0);
        assert!(persp.facing(toward, u));
        assert!(!persp.facing(away, u));
        assert!(ortho.facing(toward, u));
        assert!(!ortho.facing(away, u));
    }
    #[test]
    fn the_matrix_agrees_with_project() {
        for ortho in [false, true] {
            let cam = Camera {
                yaw: 37,
                pitch: -21,
                dist: 3.0,
                pan: [0.25, -0.5],
                ortho,
            };
            let m = cam.matrix();
            let eye = cam.view();
            for v in [
                Vec3::new(0.3, -0.4, 0.5),
                Vec3::new(-0.9, 0.1, -0.7),
                Vec3::new(0.6, 0.8, 0.0),
            ] {
                let clip = [
                    m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12],
                    m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13],
                    m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14],
                    m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15],
                ];
                let p = cam.project(eye.apply(v), 96.0).unwrap();
                let sx = 48.0 * (1.0 + clip[0] / clip[3]);
                let sy = 48.0 * (1.0 - clip[1] / clip[3]);
                let z = clip[2] / clip[3];
                assert!((sx - p[0]).abs() < 1e-3, "x {sx} vs {}", p[0]);
                assert!((sy - p[1]).abs() < 1e-3, "y {sy} vs {}", p[1]);
                assert!((0.0..=1.0).contains(&z), "depth {z}");
            }
        }
    }
    #[test]
    fn the_rig_orbit_wraps_and_clamps() {
        let mut rig = Rig::new();
        rig.orbit(-8, 0);
        assert_eq!(rig.yaw, TURN - 8);
        rig.orbit(TURN, 999);
        assert_eq!(rig.yaw, TURN - 8);
        assert_eq!(rig.pitch, PITCH_MAX);
        rig.orbit(0, -9999);
        assert_eq!(rig.pitch, -PITCH_MAX);
    }
    #[test]
    fn the_rig_zoom_and_pan_clamp() {
        let mut rig = Rig::new();
        rig.zoom(-100);
        assert_eq!(rig.dist, DIST_MIN);
        rig.zoom(100);
        assert_eq!(rig.dist, DIST_MAX);
        rig.pan(-100, 100);
        assert_eq!(rig.pan, [-PAN_MAX, PAN_MAX]);
    }
    #[test]
    fn the_rig_views_are_presets() {
        let mut rig = Rig::new();
        rig.pan(4, 4);
        assert!(rig.view("iso"));
        assert_eq!((rig.yaw, rig.pitch, rig.pan), (32, 20, [0, 0]));
        assert!(rig.view("top"));
        assert_eq!((rig.yaw, rig.pitch), (0, TURN / 4));
        assert!(rig.view("front"));
        assert_eq!((rig.yaw, rig.pitch), (0, 0));
        assert!(!rig.view("side"));
    }
    #[test]
    fn the_rig_round_trips_json() {
        let mut a = Rig::new();
        a.orbit(24, -16);
        a.zoom(6);
        a.pan(3, -5);
        a.ortho = true;
        let mut b = Rig::new();
        b.load(&a.to_json());
        assert_eq!(a, b);
    }
    #[test]
    fn the_rig_load_survives_garbage() {
        let mut rig = Rig::new();
        rig.load(&json!({ "yaw": "soup", "pitch": 999, "dist": 4, "pan": [999, 1], "ortho": 7 }));
        assert_eq!(
            rig,
            Rig {
                pan: [0, 1],
                ..Rig::new()
            }
        );
    }
    #[test]
    fn the_rig_camera_scales_units() {
        let mut rig = Rig::new();
        rig.zoom(4);
        rig.pan(8, -16);
        let cam = rig.camera();
        assert_eq!(cam.dist, 4.0);
        assert_eq!(cam.pan, [0.5, -1.0]);
    }
}
