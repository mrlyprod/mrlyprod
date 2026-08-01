use crate::fractal::{julia, mandelbrot, real, Viewport, FEMTO};
use mrlycore::rng::Rng;

const SAMPLES: usize = 200;
const PROBE: i64 = 150;

pub enum Wayfinder {
    Mandelbrot,
    Julia { cr: i64, ci: i64 },
}

impl Wayfinder {
    fn probe(&self, x: f64, y: f64) -> i64 {
        match self {
            Wayfinder::Mandelbrot => mandelbrot(x, y, PROBE),
            Wayfinder::Julia { cr, ci } => julia(x, y, real(*cr), real(*ci), PROBE),
        }
    }
    pub fn pick(&self, v: &Viewport, rng: &mut Rng) -> (i64, i64) {
        let [xmin, xmax, ymin, ymax] = v.reals();
        let (mut bx, mut by) = v.center();
        let mut best = -1i64;
        for _ in 0..SAMPLES {
            let x = xmin + rng.unit() * (xmax - xmin);
            let y = ymin + rng.unit() * (ymax - ymin);
            let iter = self.probe(x, y);
            let score = if iter < PROBE { iter } else { 0 };
            if score > best {
                best = score;
                bx = (x * FEMTO as f64).round() as i64;
                by = (y * FEMTO as f64).round() as i64;
            }
        }
        (bx, by)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pick_is_seeded() {
        let v = crate::fractal::MANDELBROT;
        let a = Wayfinder::Mandelbrot.pick(&v, &mut Rng::new(9));
        let b = Wayfinder::Mandelbrot.pick(&v, &mut Rng::new(9));
        assert_eq!(a, b);
    }
    #[test]
    fn pick_inside_viewport() {
        let v = crate::fractal::MANDELBROT;
        let (x, y) = Wayfinder::Mandelbrot.pick(&v, &mut Rng::new(3));
        assert!(x >= v.xmin && x <= v.xmax && y >= v.ymin && y <= v.ymax);
    }
}
