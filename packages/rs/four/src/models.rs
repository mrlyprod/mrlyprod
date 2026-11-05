use crate::tag;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell4d {
    pub width: usize,
    pub height: usize,
    pub types: Vec<u8>,
}

impl Cell4d {
    pub fn new(width: usize, height: usize) -> Cell4d {
        Cell4d {
            width,
            height,
            types: vec![0; 4 * width * height],
        }
    }
    pub fn shape(&self) -> [usize; 2] {
        [2 * self.height, 2 * self.width]
    }
    fn rows(&self, x: usize, y: usize) -> (usize, usize) {
        let stride = 2 * self.width;
        (2 * y * stride + 2 * x, (2 * y + 1) * stride + 2 * x)
    }
    pub fn at(&self, x: usize, y: usize) -> [u8; 4] {
        let (top, bottom) = self.rows(x, y);
        [
            self.types[top],
            self.types[top + 1],
            self.types[bottom + 1],
            self.types[bottom],
        ]
    }
    pub fn put(&mut self, x: usize, y: usize, [n, e, s, w]: [u8; 4]) {
        let (top, bottom) = self.rows(x, y);
        self.types[top] = n;
        self.types[top + 1] = e;
        self.types[bottom + 1] = s;
        self.types[bottom] = w;
    }
    pub fn tag(&self, x: usize, y: usize) -> u8 {
        tag::of(self.at(x, y))
    }
    pub fn anti(mut self) -> Cell4d {
        for value in &mut self.types {
            *value = match *value {
                0 => 1,
                1 => 0,
                other => other,
            };
        }
        self
    }
}
