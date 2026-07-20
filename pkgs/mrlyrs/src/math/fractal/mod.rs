pub mod presets;
pub mod wayfinder;

pub use presets::{Preset, JULIA_PRESETS};
pub use wayfinder::Wayfinder;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

pub const MANDELBROT: Viewport = Viewport {
    xmin: -2.0,
    xmax: 1.0,
    ymin: -1.5,
    ymax: 1.5,
};
pub const JULIA: Viewport = Viewport {
    xmin: -1.5,
    xmax: 1.5,
    ymin: -1.5,
    ymax: 1.5,
};

impl Viewport {
    pub fn center(&self) -> (f64, f64) {
        ((self.xmin + self.xmax) * 0.5, (self.ymin + self.ymax) * 0.5)
    }
    pub fn around(cx: f64, cy: f64, halfw: f64, halfh: f64) -> Viewport {
        Viewport {
            xmin: cx - halfw,
            xmax: cx + halfw,
            ymin: cy - halfh,
            ymax: cy + halfh,
        }
    }
    pub fn fit(&self, w: usize, h: usize) -> Viewport {
        if w == 0 || h == 0 {
            return *self;
        }
        let vw = self.xmax - self.xmin;
        let vh = self.ymax - self.ymin;
        let ca = w as f64 / h as f64;
        let va = vw / vh;
        let (cx, cy) = self.center();
        if ca > va {
            let nw = vh * ca;
            Viewport::around(cx, cy, nw * 0.5, vh * 0.5)
        } else {
            let nh = vw / ca;
            Viewport::around(cx, cy, vw * 0.5, nh * 0.5)
        }
    }
}

pub fn mandelbrot(cr: f64, ci: f64, max: i64) -> i64 {
    let mut zr = 0.0f64;
    let mut zi = 0.0f64;
    let mut iter = 0i64;
    while zr * zr + zi * zi <= 4.0 && iter < max {
        let tmp = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = tmp;
        iter += 1;
    }
    iter
}

pub fn julia(zr0: f64, zi0: f64, cr: f64, ci: f64, max: i64) -> i64 {
    let mut zr = zr0;
    let mut zi = zi0;
    let mut iter = 0i64;
    while zr * zr + zi * zi <= 4.0 && iter < max {
        let tmp = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = tmp;
        iter += 1;
    }
    iter
}

pub fn auto_max_iter(zoom: f64) -> i64 {
    100 + (50.0 * zoom.max(1.0).log2()).floor() as i64
}

fn triangle(x: f64) -> f64 {
    let p = x - x.floor();
    1.0 - (2.0 * p - 1.0).abs()
}

fn mix(a: [u8; 4], b: [u8; 4], t: f64) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t) as u8;
    [lerp(a[0], b[0]), lerp(a[1], b[1]), lerp(a[2], b[2]), 255]
}

pub fn shade(
    iter: i64,
    max: i64,
    phase: f64,
    period: f64,
    primary: [u8; 4],
    accent: [u8; 4],
) -> [u8; 4] {
    if iter >= max {
        return primary;
    }
    let period = if period <= 0.0 { 1.0 } else { period };
    mix(primary, accent, triangle((iter as f64 + phase) / period))
}

pub fn rotate(cr: f64, ci: f64, center: (f64, f64), angle: f64) -> (f64, f64) {
    if angle == 0.0 {
        return (cr, ci);
    }
    let (mr, mi) = center;
    let dr = cr - mr;
    let di = ci - mi;
    let ca = angle.cos();
    let sa = angle.sin();
    (dr * ca - di * sa + mr, dr * sa + di * ca + mi)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interior_is_deep() {
        assert_eq!(mandelbrot(0.0, 0.0, 200), 200);
        assert!(mandelbrot(2.0, 2.0, 200) < 5);
    }
    #[test]
    fn fit_preserves_center() {
        let v = MANDELBROT.fit(200, 100);
        assert_eq!(v.center(), MANDELBROT.center());
        assert!((v.xmax - v.xmin) >= (v.ymax - v.ymin));
    }
    #[test]
    fn shade_interior_primary() {
        let p = [0, 0, 0, 255];
        let a = [30, 200, 240, 255];
        assert_eq!(shade(50, 50, 0.0, 12.0, p, a), p);
        assert_ne!(shade(10, 50, 0.0, 12.0, p, a), p);
    }
    #[test]
    fn rotate_identity_when_zero() {
        assert_eq!(rotate(0.3, 0.4, (0.0, 0.0), 0.0), (0.3, 0.4));
    }
}
