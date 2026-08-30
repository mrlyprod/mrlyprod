use super::sample::{membership, pack};
use super::stack::merge;
use super::{Combine, Spec};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

/// A cubic grid of f32 samples, x-major.
#[derive(Clone, Debug, PartialEq)]
pub struct Volume {
    /// The samples, x-major, then y, then z.
    pub data: Vec<f32>,
    /// The side in samples.
    pub size: usize,
}

/// A plane through the unit box, framed for sampling: its centre, its two in-plane axes and the width of the square window that holds the whole section, all in the box `[-1, 1]^3`.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    /// The point of the plane the window is centred on.
    pub centre: [f64; 3],
    /// The unit axis the window's columns run along.
    pub u: [f64; 3],
    /// The unit axis the window's rows run along.
    pub v: [f64; 3],
    /// The unit normal.
    pub normal: [f64; 3],
    /// The side of the square window.
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
    /// Builds a zeroed volume of the side.
    pub fn new(size: usize) -> Volume {
        Volume {
            data: vec![0.0; size * size * size],
            size,
        }
    }
    /// Wraps x-major samples of the side.
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<Volume> {
        if data.len() != size * size * size {
            return value_error("data must be size*size*size.");
        }
        Ok(Volume { data, size })
    }
    /// Returns the smallest sample.
    pub fn min(&self) -> f32 {
        self.data.iter().cloned().fold(f32::INFINITY, f32::min)
    }
    /// Returns the largest sample.
    pub fn max(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }
    /// Reads the sample at a voxel.
    pub fn at(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[(x * self.size + y) * self.size + z]
    }
    /// Reads the voxel a point of the unit cube falls in, or zero outside it.
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
    /// Thresholds into a byte tensor: one where a sample reaches the level, zero below.
    pub fn solid(&self, level: f32) -> Tensor {
        let mut grid = Tensor::new(vec![self.size; 3]);
        for (site, &v) in grid.bytes_mut().iter_mut().zip(self.data.iter()) {
            *site = (v >= level) as u8;
        }
        grid
    }
    /// Counts the samples at or above the level.
    pub fn count(&self, level: f32) -> usize {
        self.data.iter().filter(|&&v| v >= level).count()
    }
    /// Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.
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

/// Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.
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

/// Layers one cube design at several side numbers into a volume under the chosen combine.
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

#[cfg(test)]
mod tests {
    use super::*;
    use mrlymath::bang::corners_to_code;

    fn low() -> Spec {
        Spec::new(corners_to_code(&[vec![0, 0, 0]], 3, 2), 2, 3)
    }

    #[test]
    fn the_low_corner_at_scale_one_fills_the_first_octant() {
        let v = volume(low(), &[1], Combine::Sum, 1, 4).unwrap();
        assert_eq!(v.count(1.0), 64);
        let v = volume(low(), &[2], Combine::Sum, 1, 4).unwrap();
        assert_eq!(v.count(1.0), 8);
        assert_eq!(v.at(0, 0, 0), 1.0);
        assert_eq!(v.at(3, 0, 0), 0.0);
    }

    #[test]
    fn the_sponge_stack_counts_its_layers() {
        let sponge = Spec::new(23, 2, 3);
        let v = volume(sponge, &[1, 3], Combine::Sum, 1, 9).unwrap();
        assert_eq!((v.min(), v.max()), (1.0, 2.0));
        assert_eq!(v.count(2.0), 20 * 27);
        let x = volume(sponge, &[1, 3], Combine::Xor, 1, 9).unwrap();
        assert_eq!(x.count(1.0), 7 * 27);
        let a = volume(sponge, &[3, 9], Combine::And, 1, 9).unwrap();
        assert_eq!(a.count(1.0), 64 + 240 + 48);
        assert!(volume(Spec::new(1, 2, 2), &[1], Combine::Sum, 1, 4).is_err());
        assert!(volume(sponge, &[1], Combine::Sum, 1, 0).is_err());
    }

    #[test]
    fn the_diagonal_frame_holds_the_hexagon() {
        let f = frame([1.0, 1.0, 1.0], 0.5).unwrap();
        assert!((dot(f.centre, f.normal)).abs() < 1e-12);
        assert!((dot(f.u, f.v)).abs() < 1e-12);
        assert!((f.width - 2.0 * (8.0f64 / 3.0).sqrt()).abs() < 1e-9);
        let solid = Volume::from_data(vec![1.0; 27], 3).unwrap();
        let (_, inside) = solid.plane(&f, 200);
        let share = inside.iter().map(|&b| b as f64).sum::<f64>() / 40000.0;
        let hexagon = 3.0 * 3f64.sqrt() / (f.width * f.width);
        assert!((share - hexagon).abs() < 0.01, "{share} {hexagon}");
        let x = frame([1.0, 0.0, 0.0], 0.25).unwrap();
        assert!((x.centre[0] + 0.5).abs() < 1e-12);
        assert_eq!(x.width, 2.0);
        assert!(frame([0.0, 0.0, 0.0], 0.5).is_err());
    }
}
