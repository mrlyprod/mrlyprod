/// A three-component vector of f32.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
    /// The z component.
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
    /// Builds a vector from its components.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    /// Multiplies every component by the scalar.
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    /// Returns the dot product of the two vectors.
    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    /// Returns the cross product, perpendicular to both vectors.
    ///
    /// ```
    /// use mrlymath::space::Vec3;
    /// let n = Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(n, Vec3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_is_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 1.0);
        let c = a.cross(b);
        assert!(c.dot(a).abs() < 1e-6);
        assert!(c.dot(b).abs() < 1e-6);
    }
}
