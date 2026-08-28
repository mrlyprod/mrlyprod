/// A square grid of f32 samples.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// The samples in row-major order.
    pub data: Vec<f32>,
    /// The side length in samples.
    pub size: usize,
}

impl Field {
    /// Builds a zeroed field of the given side.
    pub fn new(size: usize) -> Field {
        Field {
            data: vec![0.0; size * size],
            size,
        }
    }
    /// Wraps row-major samples of the given side.
    pub fn from_data(data: Vec<f32>, size: usize) -> Field {
        assert_eq!(data.len(), size * size, "data must be size*size");
        Field { data, size }
    }
    /// Returns the smallest sample.
    pub fn min(&self) -> f32 {
        self.data.iter().cloned().fold(f32::INFINITY, f32::min)
    }
    /// Returns the largest sample.
    pub fn max(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }
    /// Returns the mean sample, or zero for an empty field.
    pub fn mean(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.data.iter().map(|&v| v as f64).sum::<f64>() / self.data.len() as f64
    }
    /// Returns the samples widened to f64.
    pub fn as_f64(&self) -> Vec<f64> {
        self.data.iter().map(|&v| v as f64).collect()
    }
    /// Returns the samples scaled into 0..1, symmetric about zero on request.
    pub fn normalized(&self, symmetric: bool) -> Vec<f32> {
        if symmetric {
            let m = self
                .data
                .iter()
                .fold(0.0f32, |acc, &v| acc.max(v.abs()))
                .max(f32::EPSILON);
            self.data.iter().map(|&v| (v / m + 1.0) / 2.0).collect()
        } else {
            let lo = self.min();
            let hi = self.max();
            let span = (hi - lo).max(f32::EPSILON);
            self.data.iter().map(|&v| (v - lo) / span).collect()
        }
    }
}
