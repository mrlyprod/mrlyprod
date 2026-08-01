use super::field::Field;
use super::mask::Mask;
use super::rng::Rng;
use super::trig::{self, N as TRIG_N};
use super::waves_luts::{ENVELOPE, FPS, STAMP, STAMP_RADIUS};

const LIFETIME_FRAMES: i64 = (ENVELOPE.len() - 1) as i64;

pub const MILLI: i64 = 1000;

pub const WHEEL: i64 = TRIG_N as i64 * MILLI;

const MILLI_F: f32 = MILLI as f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source {
    pub x: i64,
    pub y: i64,
    pub born_frame: i64,
    pub phase: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WavesConfig {
    pub c2: i64,
    pub damp: i64,
    pub freq: i64,
    pub sigma: i64,
    pub amp: i64,
    pub gain: i64,
    pub reflect: i64,
}

impl Default for WavesConfig {
    fn default() -> WavesConfig {
        WavesConfig {
            c2: 200,
            damp: 1,
            freq: 1000,
            sigma: 1500,
            amp: 800,
            gain: 4,
            reflect: 1000,
        }
    }
}

#[inline]
pub fn phase_index(phase: i64) -> usize {
    (phase + MILLI / 2)
        .div_euclid(MILLI)
        .rem_euclid(TRIG_N as i64) as usize
}

#[inline]
fn phase_step(freq: i64) -> i64 {
    (freq * TRIG_N as i64 + FPS / 2) / FPS
}

#[derive(Clone, Debug)]
pub struct Waves {
    mask: Mask,
    config: WavesConfig,
    curr: Field,
    prev: Field,
    next: Field,
    sources: Vec<Source>,
    frame: i64,
    rng: Rng,
    stamp: Vec<f32>,
    stamp_radius: usize,
}

impl Waves {
    pub fn new(mask: Mask, config: WavesConfig, seed: u64) -> Waves {
        let w = mask.width();
        let h = mask.height();
        let (stamp, stamp_radius) = build_stamp(config.sigma);
        Waves {
            mask,
            config,
            curr: Field::new(w, h),
            prev: Field::new(w, h),
            next: Field::new(w, h),
            sources: Vec::new(),
            frame: 0,
            rng: Rng::new(seed),
            stamp,
            stamp_radius,
        }
    }

    pub fn config(&self) -> &WavesConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: WavesConfig) {
        if config.sigma != self.config.sigma {
            let (stamp, radius) = build_stamp(config.sigma);
            self.stamp = stamp;
            self.stamp_radius = radius;
        }
        self.config = config;
    }

    pub fn mask(&self) -> &Mask {
        &self.mask
    }

    pub fn field(&self) -> &Field {
        &self.curr
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    pub fn frame(&self) -> i64 {
        self.frame
    }

    pub fn reset(&mut self) {
        self.curr.clear();
        self.prev.clear();
        self.next.clear();
        self.sources.clear();
        self.frame = 0;
    }

    pub fn drop(&mut self, x: i64, y: i64) {
        if self.solid_milli(x, y) {
            return;
        }
        let phase = self.rng.below(TRIG_N as u64) as i64 * MILLI;
        self.sources.push(Source {
            x,
            y,
            born_frame: self.frame,
            phase,
        });
    }

    pub fn step(&mut self) {
        let w = self.mask.width();
        let h = self.mask.height();
        let c2 = self.config.c2 as f32 / MILLI_F;
        let damp = self.config.damp as f32 / MILLI_F;
        let reflect = self.config.reflect as f32 / MILLI_F;

        for y in 0..h {
            for x in 0..w {
                let wall = self.wall(x as i64, y as i64) * reflect;
                let c = self.curr.at(x as i64, y as i64);
                let p = self.prev.at(x as i64, y as i64);

                let lv = self.curr.at(x as i64 - 1, y as i64)
                    * (1.0 - self.wall(x as i64 - 1, y as i64) * reflect);
                let rv = self.curr.at(x as i64 + 1, y as i64)
                    * (1.0 - self.wall(x as i64 + 1, y as i64) * reflect);
                let uv = self.curr.at(x as i64, y as i64 - 1)
                    * (1.0 - self.wall(x as i64, y as i64 - 1) * reflect);
                let dv = self.curr.at(x as i64, y as i64 + 1)
                    * (1.0 - self.wall(x as i64, y as i64 + 1) * reflect);

                let lap = (lv + rv + uv + dv) - 4.0 * c;
                let mut n = 2.0 * c - p + c2 * lap - damp * (c - p);
                n *= 1.0 - wall;
                self.next.set(x, y, n);
            }
        }

        let amp = self.config.amp as f32 / MILLI_F;
        let radius = self.stamp_radius as i64;
        let span = (2 * self.stamp_radius + 1) as i64;
        for src in &self.sources {
            let age = (self.frame - src.born_frame).max(0) as usize;
            let env = ENVELOPE[age.min(ENVELOPE.len() - 1)];
            let osc = trig::sin_idx(phase_index(src.phase));
            let value = amp * env * osc;
            let sx = src.x.div_euclid(MILLI);
            let sy = src.y.div_euclid(MILLI);
            for ky in 0..span {
                for kx in 0..span {
                    let px = sx + (kx - radius);
                    let py = sy + (ky - radius);
                    if px < 0 || py < 0 || px >= w as i64 || py >= h as i64 {
                        continue;
                    }
                    let k = self.stamp[(ky * span + kx) as usize];
                    let wall = self.wall(px, py) * reflect;
                    let add = value * k * (1.0 - wall);
                    let nv = self.next.at(px, py) + add;
                    self.next.set(px as usize, py as usize, nv);
                }
            }
        }

        let step = phase_step(self.config.freq);
        for src in self.sources.iter_mut() {
            src.phase = (src.phase + step).rem_euclid(WHEEL);
        }
        self.frame += 1;

        let frame = self.frame;
        self.sources
            .retain(|s| frame - s.born_frame < LIFETIME_FRAMES);

        std::mem::swap(&mut self.prev, &mut self.curr);
        std::mem::swap(&mut self.curr, &mut self.next);
    }

    #[inline]
    fn wall(&self, x: i64, y: i64) -> f32 {
        if self.mask.solid(x as f32, y as f32) {
            1.0
        } else {
            0.0
        }
    }

    #[inline]
    fn solid_milli(&self, x: i64, y: i64) -> bool {
        self.mask
            .solid(x.div_euclid(MILLI) as f32, y.div_euclid(MILLI) as f32)
    }
}

fn build_stamp(sigma: i64) -> (Vec<f32>, usize) {
    if sigma == super::waves_luts::STAMP_SIGMA {
        return (STAMP.to_vec(), STAMP_RADIUS);
    }
    let radius = ((2 * sigma + MILLI - 1) / MILLI).clamp(1, 8) as usize;
    let span = 2 * radius + 1;
    let s = sigma as f32 / MILLI_F;
    let s2 = s * s;
    let mut out = Vec::with_capacity(span * span);
    for dy in 0..span {
        for dx in 0..span {
            let rx = dx as f32 - radius as f32;
            let ry = dy as f32 - radius as f32;
            let r2 = rx * rx + ry * ry;
            out.push((-r2 / s2).exp());
        }
    }
    (out, radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_ignored_inside_wall() {
        let mask = Mask::build("carpet", 3, 2, 0, 1, false).unwrap();
        let mut sim = Waves::new(mask, WavesConfig::default(), 1);
        'outer: for y in 0..sim.mask().height() {
            for x in 0..sim.mask().width() {
                if sim.mask().solid(x as f32 + 0.5, y as f32 + 0.5) {
                    sim.drop(x as i64 * MILLI + 500, y as i64 * MILLI + 500);
                    break 'outer;
                }
            }
        }
        assert_eq!(sim.sources().len(), 0);
    }

    #[test]
    fn energy_stays_finite() {
        let mask = Mask::open(21, 21);
        let mut sim = Waves::new(mask, WavesConfig::default(), 3);
        let (cx, cy) = (
            sim.mask().width() as i64 * MILLI / 2,
            sim.mask().height() as i64 * MILLI / 2,
        );
        sim.drop(cx, cy);
        for _ in 0..300 {
            sim.step();
        }
        for v in &sim.field().data {
            assert!(v.is_finite(), "field value not finite: {v}");
        }
    }

    #[test]
    fn centered_drop_is_radially_symmetric() {
        let mask = Mask::open(21, 21);
        let w = mask.width();
        let h = mask.height();
        assert_eq!(w % 2, 1);
        let mut sim = Waves::new(mask, WavesConfig::default(), 0);
        let cx = (w / 2) as i64 * MILLI + 500;
        let cy = (h / 2) as i64 * MILLI + 500;
        sim.drop(cx, cy);
        for _ in 0..40 {
            sim.step();
        }
        let f = sim.field();
        let cxi = w / 2;
        let cyi = h / 2;
        for dy in 0..=(h / 2) as i64 {
            for dx in 0..=(w / 2) as i64 {
                let a = f.at(cxi as i64 + dx, cyi as i64 + dy);
                let b = f.at(cxi as i64 - dx, cyi as i64 + dy);
                let c = f.at(cxi as i64 + dx, cyi as i64 - dy);
                let d = f.at(cxi as i64 - dx, cyi as i64 - dy);
                let tol = 1e-4;
                assert!((a - b).abs() < tol, "x-mirror dx{dx} dy{dy}: {a} vs {b}");
                assert!((a - c).abs() < tol, "y-mirror dx{dx} dy{dy}: {a} vs {c}");
                assert!((a - d).abs() < tol, "xy-mirror dx{dx} dy{dy}: {a} vs {d}");
            }
        }
    }

    #[test]
    fn sources_cull_after_lifetime() {
        let mask = Mask::open(21, 21);
        let mut sim = Waves::new(mask, WavesConfig::default(), 0);
        let (cx, cy) = (
            sim.mask().width() as i64 * MILLI / 2,
            sim.mask().height() as i64 * MILLI / 2,
        );
        sim.drop(cx, cy);
        assert_eq!(sim.sources().len(), 1);
        for _ in 0..LIFETIME_FRAMES {
            sim.step();
        }
        assert_eq!(sim.sources().len(), 0);
    }

    #[test]
    fn deterministic_with_seed() {
        let mask = Mask::open(21, 21);
        let (cx, cy) = (
            mask.width() as i64 * MILLI / 2,
            mask.height() as i64 * MILLI / 2,
        );
        let mut a = Waves::new(mask.clone(), WavesConfig::default(), 77);
        let mut b = Waves::new(mask, WavesConfig::default(), 77);
        a.drop(cx, cy);
        b.drop(cx, cy);
        for _ in 0..50 {
            a.step();
            b.step();
        }
        assert_eq!(a.field(), b.field());
    }
}
