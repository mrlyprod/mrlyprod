use super::sample::{membership, pack};
use super::stack::merge;
use super::{Combine, Spec};
use mrly_error::{Result, value_error};
use mrly_tensor::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub data: Vec<f32>,
    pub size: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub centre: [f64; 3],
    pub u: [f64; 3],
    pub v: [f64; 3],
    pub normal: [f64; 3],
    pub width: f64,
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> Option<[f64; 3]> {
    let n = dot(a, a).sqrt();
    (n > 1e-12).then(|| [a[0] / n, a[1] / n, a[2] / n])
}

impl Volume {
    pub fn new(size: usize) -> Volume {
        Volume {
            data: vec![0.0; size * size * size],
            size,
        }
    }
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<Volume> {
        if data.len() != size * size * size {
            return value_error("data must be size*size*size.");
        }
        Ok(Volume { data, size })
    }
    pub fn min(&self) -> f32 {
        self.data.iter().cloned().fold(f32::INFINITY, f32::min)
    }
    pub fn max(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }
    pub fn at(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[(x * self.size + y) * self.size + z]
    }
    pub fn sample(&self, p: [f64; 3]) -> Option<f32> {
        if p.iter().any(|&c| !(0.0..1.0).contains(&c)) {
            return None;
        }
        let s = self.size as f64;
        Some(self.at(
            (p[0] * s) as usize,
            (p[1] * s) as usize,
            (p[2] * s) as usize,
        ))
    }
    pub fn solid(&self, level: f32) -> Tensor {
        let mut grid = Tensor::new(vec![self.size; 3]);
        for (flat, &v) in self.data.iter().take(grid.size()).enumerate() {
            grid.put(flat, i64::from(v >= level));
        }
        grid
    }
    pub fn count(&self, level: f32) -> usize {
        self.data.iter().filter(|&&v| v >= level).count()
    }
    pub fn plane(&self, frame: &Frame, out: usize) -> (Vec<f32>, Vec<u8>) {
        let mut values = Vec::with_capacity(out * out);
        let mut inside = Vec::with_capacity(out * out);
        for i in 0..out {
            for j in 0..out {
                let a = ((j as f64 + 0.5) / out as f64 - 0.5) * frame.width;
                let b = ((i as f64 + 0.5) / out as f64 - 0.5) * frame.width;
                let p: Vec<f64> = (0..3)
                    .map(|k| (frame.centre[k] + a * frame.u[k] + b * frame.v[k] + 1.0) / 2.0)
                    .collect();
                match self.sample([p[0], p[1], p[2]]) {
                    Some(v) => {
                        values.push(v);
                        inside.push(1);
                    }
                    None => {
                        values.push(0.0);
                        inside.push(0);
                    }
                }
            }
        }
        (values, inside)
    }
}

pub fn frame(normal: [f64; 3], offset: f64) -> Result<Frame> {
    let Some(n) = unit(normal) else {
        return value_error("the normal must not be zero.");
    };
    let seed = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = unit(cross(seed, n)).unwrap();
    let v = cross(n, u);
    let corners = (0..8).map(|c| {
        [
            if c & 1 == 0 { -1.0 } else { 1.0 },
            if c & 2 == 0 { -1.0 } else { 1.0 },
            if c & 4 == 0 { -1.0 } else { 1.0 },
        ]
    });
    let mut span = [[f64::INFINITY, f64::NEG_INFINITY]; 3];
    for c in corners {
        for (k, axis) in [u, v, n].iter().enumerate() {
            let t = dot(c, *axis);
            span[k][0] = span[k][0].min(t);
            span[k][1] = span[k][1].max(t);
        }
    }
    let mid = |k: usize| (span[k][0] + span[k][1]) / 2.0;
    let depth = span[2][0] + offset.clamp(0.0, 1.0) * (span[2][1] - span[2][0]);
    let centre = [
        mid(0) * u[0] + mid(1) * v[0] + depth * n[0],
        mid(0) * u[1] + mid(1) * v[1] + depth * n[1],
        mid(0) * u[2] + mid(1) * v[2] + depth * n[2],
    ];
    Ok(Frame {
        centre,
        u,
        v,
        normal: n,
        width: (span[0][1] - span[0][0]).max(span[1][1] - span[1][0]),
    })
}

fn layer(spec: Spec, number: usize, level: usize, size: usize) -> Result<Vec<bool>> {
    let Spec {
        code,
        base: q,
        dimension: d,
    } = spec;
    if d != 3 {
        return value_error("a volume needs dimension 3.");
    }
    let table = membership(code, q, 3)?;
    let mut mask = vec![true; size * size * size];
    let inv = 1.0 / size as f64;
    for k in 0..level.max(1) {
        let s = (number * q.pow(k as u32)) as f64;
        let residue =
            |i: usize| ((s * (i as f64 + 0.5) * inv).floor() as i64).rem_euclid(q as i64) as usize;
        let digits: Vec<usize> = (0..size).map(residue).collect();
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let cell = (x * size + y) * size + z;
                    if mask[cell] && !table[pack(&[digits[x], digits[y], digits[z]], q)] {
                        mask[cell] = false;
                    }
                }
            }
        }
    }
    Ok(mask)
}

pub fn volume(
    spec: Spec,
    numbers: &[usize],
    combine: Combine,
    level: usize,
    size: usize,
) -> Result<Volume> {
    if size == 0 {
        return value_error("size must be at least 1.");
    }
    let mut acc = vec![0.0f32; size * size * size];
    let mut first = true;
    for &n in numbers {
        let mask = layer(spec, n, level, size)?;
        merge(&mut acc, &mask, combine, first);
        first = false;
    }
    Volume::from_data(acc, size)
}
