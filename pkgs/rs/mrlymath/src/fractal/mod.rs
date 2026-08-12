/// The named house Julia constants.
pub mod presets;
/// The scout that picks the next zoom point.
pub mod wayfinder;

pub use presets::{Preset, JULIA_PRESETS};
pub use wayfinder::Wayfinder;

/// The fixed-point scale, a quadrillion ticks per unit.
pub const FEMTO: i64 = 1_000_000_000_000_000;

/// Returns the femto fixed-point value as a float.
pub fn real(v: i64) -> f64 {
    v as f64 / FEMTO as f64
}

/// The rectangular window onto the complex plane, in femto ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// The left edge.
    pub xmin: i64,
    /// The right edge.
    pub xmax: i64,
    /// The bottom edge.
    pub ymin: i64,
    /// The top edge.
    pub ymax: i64,
}

/// The home view of the Mandelbrot set.
pub const MANDELBROT: Viewport = Viewport {
    xmin: -2 * FEMTO,
    xmax: FEMTO,
    ymin: -3 * FEMTO / 2,
    ymax: 3 * FEMTO / 2,
};
/// The home view of the Julia sets.
pub const JULIA: Viewport = Viewport {
    xmin: -3 * FEMTO / 2,
    xmax: 3 * FEMTO / 2,
    ymin: -3 * FEMTO / 2,
    ymax: 3 * FEMTO / 2,
};

fn mid(a: i64, b: i64) -> i64 {
    ((a as i128 + b as i128) / 2) as i64
}

impl Viewport {
    /// Returns the viewport's midpoint.
    pub fn center(&self) -> (i64, i64) {
        (mid(self.xmin, self.xmax), mid(self.ymin, self.ymax))
    }
    /// Builds the viewport spanning the given half-extents about a center.
    pub fn around(cx: i64, cy: i64, halfw: i64, halfh: i64) -> Viewport {
        Viewport {
            xmin: cx - halfw,
            xmax: cx + halfw,
            ymin: cy - halfh,
            ymax: cy + halfh,
        }
    }
    /// Returns the viewport widened to the aspect of a w-by-h canvas, keeping its center.
    pub fn fit(&self, w: usize, h: usize) -> Viewport {
        if w == 0 || h == 0 {
            return *self;
        }
        let vw = (self.xmax - self.xmin) as i128;
        let vh = (self.ymax - self.ymin) as i128;
        let (w, h) = (w as i128, h as i128);
        let (cx, cy) = self.center();
        if w * vh > h * vw {
            let nw = vh * w / h;
            Viewport::around(cx, cy, (nw / 2) as i64, (vh / 2) as i64)
        } else {
            let nh = vw * h / w;
            Viewport::around(cx, cy, (vw / 2) as i64, (nh / 2) as i64)
        }
    }
    /// Returns the four edges as floats, xmin then xmax then ymin then ymax.
    pub fn reals(&self) -> [f64; 4] {
        [
            real(self.xmin),
            real(self.xmax),
            real(self.ymin),
            real(self.ymax),
        ]
    }
}

/// Iterates z squared plus c from zero, returning the escape iteration capped at max.
///
/// ```
/// assert_eq!(mrlymath::fractal::mandelbrot(0.0, 0.0, 200), 200);
/// assert!(mrlymath::fractal::mandelbrot(2.0, 2.0, 200) < 5);
/// ```
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

/// Iterates z squared plus c from the given z, returning the escape iteration capped at max.
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

fn log2(x: f64) -> f64 {
    mrlycore::logs::ln(x) / std::f64::consts::LN_2
}

fn triangle(x: f64) -> f64 {
    let p = x - x.floor();
    1.0 - (2.0 * p - 1.0).abs()
}

fn rgb(c: [u8; 4]) -> [f64; 3] {
    [c[0] as f64, c[1] as f64, c[2] as f64]
}

fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn bytes(c: [f64; 3]) -> [u8; 4] {
    [
        c[0].round() as u8,
        c[1].round() as u8,
        c[2].round() as u8,
        255,
    ]
}

/// Returns the smoothed escape level of the z-squared-plus-c orbit, or None for a point inside the set.
///
/// ```
/// assert_eq!(mrlymath::fractal::level(0.0, 0.0, 0.0, 0.0, 200.0), None);
/// assert!(mrlymath::fractal::level(0.0, 0.0, 2.0, 2.0, 200.0).unwrap() < 5.0);
/// ```
pub fn level(mut zr: f64, mut zi: f64, cr: f64, ci: f64, max: f64) -> Option<f64> {
    let mut iter = 0.0f64;
    for i in 0..1000 {
        if i as f64 >= max || zr * zr + zi * zi > 128.0 {
            break;
        }
        let tmp = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = tmp;
        iter += 1.0;
    }
    if iter >= max {
        return None;
    }
    Some(iter - log2(log2((zr * zr + zi * zi).max(2.0))) + 4.0)
}

/// Blends primary toward accent on a triangle wave over the smoothed level, faded back to primary.
///
/// ```
/// let ink = [0, 0, 0, 255];
/// let lit = [30, 200, 240, 255];
/// assert_eq!(mrlymath::fractal::tint(None, 0.0, 12.0, 1.0, ink, lit), ink);
/// assert_eq!(mrlymath::fractal::tint(Some(3.0), 0.0, 12.0, 1.0, ink, lit), [15, 100, 120, 255]);
/// ```
pub fn tint(
    level: Option<f64>,
    time: f64,
    band: f64,
    fade: f64,
    primary: [u8; 4],
    accent: [u8; 4],
) -> [u8; 4] {
    let base = rgb(primary);
    let color = match level {
        Some(level) => mix(base, rgb(accent), triangle((level + time) / band.max(1.0))),
        None => base,
    };
    bytes(mix(base, color, fade.clamp(0.0, 1.0)))
}

/// Rotates a point about a center by the given cosine and sine.
pub fn rotate(cr: f64, ci: f64, center: (f64, f64), ca: f64, sa: f64) -> (f64, f64) {
    if sa == 0.0 && ca == 1.0 {
        return (cr, ci);
    }
    let (mr, mi) = center;
    let dr = cr - mr;
    let di = ci - mi;
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
    fn reals_scale_down() {
        assert_eq!(MANDELBROT.reals(), [-2.0, 1.0, -1.5, 1.5]);
    }
    #[test]
    fn level_is_none_inside_and_small_outside() {
        assert_eq!(level(0.0, 0.0, 0.0, 0.0, 200.0), None);
        assert_eq!(level(0.0, 0.0, -1.0, 0.0, 200.0), None);
        let out = level(0.0, 0.0, 2.0, 2.0, 200.0).unwrap();
        assert!(out > 0.0 && out < 5.0);
    }
    #[test]
    fn tint_falls_back_to_primary() {
        let p = [0, 0, 0, 255];
        let a = [30, 200, 240, 255];
        assert_eq!(tint(None, 0.0, 12.0, 1.0, p, a), p);
        assert_eq!(tint(Some(3.0), 0.0, 12.0, 0.0, p, a), p);
        assert_eq!(tint(Some(90.0), 7.0, 12.0, 0.0, p, a), p);
        assert_ne!(tint(Some(3.0), 0.0, 12.0, 1.0, p, a), p);
    }
    #[test]
    fn rotate_identity_when_zero() {
        assert_eq!(rotate(0.3, 0.4, (0.0, 0.0), 1.0, 0.0), (0.3, 0.4));
    }
}
