use crate::core::trig;

fn sini(i: i64) -> f32 {
    trig::sin_idx(i.rem_euclid(trig::N as i64) as usize)
}

fn cosi(i: i64) -> f32 {
    trig::cos_idx(i.rem_euclid(trig::N as i64) as usize)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    pub m: [[f32; 3]; 3],
}

impl std::ops::Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, o: Mat3) -> Mat3 {
        let mut m = [[0.0f32; 3]; 3];
        for (r, row) in m.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell =
                    self.m[r][0] * o.m[0][c] + self.m[r][1] * o.m[1][c] + self.m[r][2] * o.m[2][c];
            }
        }
        Mat3 { m }
    }
}

impl Mat3 {
    pub fn identity() -> Mat3 {
        Mat3 {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
    pub fn yaw(i: i64) -> Mat3 {
        let (c, s) = (cosi(i), sini(i));
        Mat3 {
            m: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        }
    }
    pub fn pitch(i: i64) -> Mat3 {
        let (c, s) = (cosi(i), sini(i));
        Mat3 {
            m: [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        }
    }
    pub fn apply(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_turns_are_exact() {
        let q = (trig::N / 4) as i64;
        let v = Mat3::yaw(q).apply(Vec3::new(1.0, 0.0, 0.0));
        assert!(v.x.abs() < 1e-6 && (v.z + 1.0).abs() < 1e-6);
        let v = Mat3::pitch(q).apply(Vec3::new(0.0, 1.0, 0.0));
        assert!(v.y.abs() < 1e-6 && (v.z - 1.0).abs() < 1e-6);
    }
    #[test]
    fn rotation_preserves_length() {
        let v = Vec3::new(0.3, -0.7, 0.64);
        for i in [0i64, 17, 100, -40, 255] {
            let r = (Mat3::yaw(i) * Mat3::pitch(i * 3)).apply(v);
            assert!((r.dot(r) - v.dot(v)).abs() < 1e-5);
        }
    }
    #[test]
    fn negative_indices_wrap() {
        assert_eq!(Mat3::yaw(-3).m, Mat3::yaw(253).m);
    }
    #[test]
    fn cross_is_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 1.0);
        let c = a.cross(b);
        assert!(c.dot(a).abs() < 1e-6);
        assert!(c.dot(b).abs() < 1e-6);
    }
    #[test]
    fn identity_leaves_vectors() {
        let v = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(Mat3::identity().apply(v), v);
    }
}
