use ttf_parser::OutlineBuilder;

pub const SS: usize = 4;

pub struct Outline {
    segments: Vec<[f64; 4]>,
    at: (f64, f64),
    start: (f64, f64),
}

impl Outline {
    pub fn new() -> Outline {
        Outline {
            segments: Vec::new(),
            at: (0.0, 0.0),
            start: (0.0, 0.0),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
    fn push(&mut self, to: (f64, f64)) {
        if to != self.at {
            self.segments.push([self.at.0, self.at.1, to.0, to.1]);
        }
        self.at = to;
    }
}

impl Default for Outline {
    fn default() -> Outline {
        Outline::new()
    }
}

impl OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.at = (x as f64, y as f64);
        self.start = self.at;
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.push((x as f64, y as f64));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (ax, ay) = self.at;
        let (cx, cy) = (x1 as f64, y1 as f64);
        let (bx, by) = (x as f64, y as f64);
        for step in 1..=8 {
            let t = step as f64 / 8.0;
            let u = 1.0 - t;
            let px = u * u * ax + 2.0 * u * t * cx + t * t * bx;
            let py = u * u * ay + 2.0 * u * t * cy + t * t * by;
            self.push((px, py));
        }
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (ax, ay) = self.at;
        let (cx, cy) = (x1 as f64, y1 as f64);
        let (dx, dy) = (x2 as f64, y2 as f64);
        let (bx, by) = (x as f64, y as f64);
        for step in 1..=16 {
            let t = step as f64 / 16.0;
            let u = 1.0 - t;
            let px = u * u * u * ax + 3.0 * u * u * t * cx + 3.0 * u * t * t * dx + t * t * t * bx;
            let py = u * u * u * ay + 3.0 * u * u * t * cy + 3.0 * u * t * t * dy + t * t * t * by;
            self.push((px, py));
        }
    }
    fn close(&mut self) {
        let start = self.start;
        self.push(start);
    }
}

pub fn coverage(outline: &Outline, upem: f64, cell: usize) -> Vec<u8> {
    let grid = cell * SS;
    if outline.is_empty() {
        return vec![0; cell * cell];
    }
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for s in &outline.segments {
        for (x, y) in [(s[0], s[1]), (s[2], s[3])] {
            lo = (lo.0.min(x), lo.1.min(y));
            hi = (hi.0.max(x), hi.1.max(y));
        }
    }
    let side = upem.max(hi.0 - lo.0).max(hi.1 - lo.1).max(1.0);
    let scale = grid as f64 / side;
    let ox = (grid as f64 - (hi.0 - lo.0) * scale) / 2.0 - lo.0 * scale;
    let oy = (grid as f64 - (hi.1 - lo.1) * scale) / 2.0 - lo.1 * scale;
    let device: Vec<[f64; 4]> = outline
        .segments
        .iter()
        .map(|s| {
            [
                s[0] * scale + ox,
                grid as f64 - (s[1] * scale + oy),
                s[2] * scale + ox,
                grid as f64 - (s[3] * scale + oy),
            ]
        })
        .collect();
    let mut mask = vec![0u8; grid * grid];
    for row in 0..grid {
        let ys = row as f64 + 0.5;
        let mut crossings: Vec<(f64, i32)> = Vec::new();
        for s in &device {
            let (x0, y0, x1, y1) = (s[0], s[1], s[2], s[3]);
            if (y0 <= ys) == (y1 <= ys) {
                continue;
            }
            let t = (ys - y0) / (y1 - y0);
            let x = x0 + t * (x1 - x0);
            crossings.push((x, if y1 > y0 { 1 } else { -1 }));
        }
        crossings.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut winding = 0;
        let mut span = 0.0;
        for (x, dir) in crossings {
            if winding == 0 {
                span = x;
            }
            winding += dir;
            if winding == 0 {
                let from = (span - 0.5).ceil().max(0.0) as usize;
                let to = ((x - 0.5).ceil().max(0.0) as usize).min(grid);
                for px in from..to {
                    mask[row * grid + px] = 1;
                }
            }
        }
    }
    let mut out = Vec::with_capacity(cell * cell);
    for cy in 0..cell {
        for cx in 0..cell {
            let mut sum = 0u32;
            for sy in 0..SS {
                for sx in 0..SS {
                    sum += mask[(cy * SS + sy) * grid + cx * SS + sx] as u32;
                }
            }
            out.push((sum * 255 / (SS * SS) as u32) as u8);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Outline {
        let mut o = Outline::new();
        o.move_to(200.0, 200.0);
        o.line_to(800.0, 200.0);
        o.line_to(800.0, 800.0);
        o.line_to(200.0, 800.0);
        o.close();
        o
    }

    #[test]
    fn square_fills_the_center() {
        let cov = coverage(&square(), 1000.0, 32);
        assert_eq!(cov.len(), 32 * 32);
        assert_eq!(cov[16 * 32 + 16], 255);
        assert_eq!(cov[0], 0);
        assert_eq!(cov[31], 0);
        assert_eq!(cov[31 * 32], 0);
    }

    #[test]
    fn square_is_symmetric() {
        let cov = coverage(&square(), 1000.0, 32);
        for y in 0..32 {
            for x in 0..32 {
                assert_eq!(cov[y * 32 + x], cov[y * 32 + 31 - x]);
                assert_eq!(cov[y * 32 + x], cov[(31 - y) * 32 + x]);
            }
        }
    }

    #[test]
    fn hole_stays_empty() {
        let mut o = square();
        o.move_to(400.0, 400.0);
        o.line_to(400.0, 600.0);
        o.line_to(600.0, 600.0);
        o.line_to(600.0, 400.0);
        o.close();
        let cov = coverage(&o, 1000.0, 32);
        assert_eq!(cov[16 * 32 + 16], 0);
        assert_eq!(cov[16 * 32 + 8], 255);
    }

    #[test]
    fn empty_outline_is_blank() {
        let cov = coverage(&Outline::new(), 1000.0, 32);
        assert!(cov.iter().all(|&c| c == 0));
    }
}
