use mrly_color::{BLUE, Color, WHITE};
use mrly_error::{Result, value_error};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct View {
    pub width: usize,
    pub height: usize,
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
    pub fov: f64,
    pub zoom: f64,
    pub light: [f64; 3],
    pub ink: Color,
    pub background: Color,
    pub samples: usize,
}

impl Default for View {
    fn default() -> View {
        View {
            width: 128,
            height: 128,
            yaw: 30.0,
            pitch: 20.0,
            roll: 0.0,
            fov: 0.0,
            zoom: 1.0,
            light: [-1.0, 4.0, 2.0],
            ink: BLUE,
            background: WHITE,
            samples: 4,
        }
    }
}

pub(crate) type Mat = [[f64; 3]; 3];

pub(crate) fn turn(degrees: f64) -> (f64, f64) {
    let quarter = (degrees / 90.0).round();
    let x = (degrees - quarter * 90.0) * (PI / 180.0);
    let x2 = x * x;
    let (mut sin, mut cos) = (1.0, 1.0);
    for k in (1..=9).rev() {
        let k = k as f64;
        sin = 1.0 - x2 / ((2.0 * k) * (2.0 * k + 1.0)) * sin;
        cos = 1.0 - x2 / ((2.0 * k - 1.0) * (2.0 * k)) * cos;
    }
    let sin = x * sin;
    match (quarter as i64).rem_euclid(4) {
        0 => (sin, cos),
        1 => (cos, -sin),
        2 => (-sin, -cos),
        _ => (-cos, sin),
    }
}

fn times(a: Mat, b: Mat) -> Mat {
    let mut m = [[0.0; 3]; 3];
    for (r, row) in m.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
        }
    }
    m
}

pub(crate) fn apply(m: &Mat, v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

pub(crate) fn unit(v: [f64; 3]) -> Option<[f64; 3]> {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    (length > 0.0 && length.is_finite()).then(|| v.map(|x| x / length))
}

impl View {
    pub(crate) fn rotation(&self) -> Mat {
        let (sy, cy) = turn(self.yaw);
        let (sp, cp) = turn(self.pitch);
        let (sr, cr) = turn(self.roll);
        let yaw = [[cy, 0.0, -sy], [0.0, 1.0, 0.0], [sy, 0.0, cy]];
        let pitch = [[1.0, 0.0, 0.0], [0.0, cp, -sp], [0.0, sp, cp]];
        let roll = [[cr, -sr, 0.0], [sr, cr, 0.0], [0.0, 0.0, 1.0]];
        times(roll, times(pitch, yaw))
    }
    pub(crate) fn check(&self) -> Result<[f64; 3]> {
        if !(1..=4096).contains(&self.width) || !(1..=4096).contains(&self.height) {
            return value_error("width and height must be between 1 and 4096.");
        }
        if !(1..=8).contains(&self.samples) {
            return value_error("samples must be between 1 and 8.");
        }
        if ![self.yaw, self.pitch, self.roll]
            .iter()
            .all(|a| a.is_finite())
        {
            return value_error("yaw, pitch and roll must be finite.");
        }
        if !(self.fov == 0.0 || (1.0..=120.0).contains(&self.fov)) {
            return value_error("fov must be 0, or between 1 and 120 degrees.");
        }
        if !(self.zoom > 0.0 && self.zoom <= 64.0) {
            return value_error("zoom must be above 0 and at most 64.");
        }
        match unit(self.light) {
            Some(light) => Ok(light),
            None => value_error("light must be a finite direction that is not zero."),
        }
    }
}
