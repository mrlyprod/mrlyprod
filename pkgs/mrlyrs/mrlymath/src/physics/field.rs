#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub w: usize,
    pub h: usize,
    pub data: Vec<f32>,
}

impl Field {
    pub fn new(w: usize, h: usize) -> Field {
        Field {
            w,
            h,
            data: vec![0.0; w * h],
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    #[inline]
    pub fn at(&self, x: i64, y: i64) -> f32 {
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.h {
            return 0.0;
        }
        self.data[self.idx(x as usize, y as usize)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, v: f32) {
        if x < self.w && y < self.h {
            let i = self.idx(x, y);
            self.data[i] = v;
        }
    }

    #[inline]
    pub fn add(&mut self, x: usize, y: usize, v: f32) {
        if x < self.w && y < self.h {
            let i = self.idx(x, y);
            self.data[i] += v;
        }
    }

    pub fn clear(&mut self) {
        for v in self.data.iter_mut() {
            *v = 0.0;
        }
    }

    #[inline]
    pub fn laplacian(&self, x: usize, y: usize) -> f32 {
        let c = self.at(x as i64, y as i64);
        let l = self.at(x as i64 - 1, y as i64);
        let r = self.at(x as i64 + 1, y as i64);
        let u = self.at(x as i64, y as i64 - 1);
        let d = self.at(x as i64, y as i64 + 1);
        (l + r + u + d) - 4.0 * c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_roundtrip() {
        let mut f = Field::new(4, 3);
        f.set(2, 1, 1.5);
        assert_eq!(f.at(2, 1), 1.5);
        assert_eq!(f.at(0, 0), 0.0);
    }

    #[test]
    fn out_of_bounds_reads_zero() {
        let f = Field::new(4, 3);
        assert_eq!(f.at(-1, 0), 0.0);
        assert_eq!(f.at(0, -1), 0.0);
        assert_eq!(f.at(4, 0), 0.0);
        assert_eq!(f.at(0, 3), 0.0);
    }

    #[test]
    fn laplacian_of_flat_field_is_zero() {
        let mut f = Field::new(5, 5);
        for v in f.data.iter_mut() {
            *v = 2.0;
        }
        assert_eq!(f.laplacian(2, 2), 0.0);
    }

    #[test]
    fn laplacian_of_peak() {
        let mut f = Field::new(5, 5);
        f.set(2, 2, 1.0);
        assert_eq!(f.laplacian(2, 2), -4.0);
    }
}
