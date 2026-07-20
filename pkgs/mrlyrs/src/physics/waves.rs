use super::field::Field;
use super::mask::Mask;
use super::rng::Rng;
use super::trig::{self, FracIndex, N as TRIG_N};
use super::waves_luts::{ENVELOPE, FPS, STAMP, STAMP_RADIUS};

const LIFETIME_FRAMES: u64 = (ENVELOPE.len() - 1) as u64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source {
    pub x: f32,
    pub y: f32,
    pub born_frame: u64,
    pub phase: FracIndex,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WavesConfig {
    pub c2: f32,
    pub damp: f32,
    pub freq: f32,
    pub sigma: f32,
    pub amp: f32,
    pub gain: f32,
    pub reflect: f32,
}

impl Default for WavesConfig {
    fn default() -> WavesConfig {
        WavesConfig {
            c2: 0.2,
            damp: 0.001,
            freq: 1.0,
            sigma: 1.5,
            amp: 0.8,
            gain: 4.0,
            reflect: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Waves {
    mask: Mask,
    config: WavesConfig,
    curr: Field,
    prev: Field,
    next: Field,
    sources: Vec<Source>,
    frame: u64,
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

    pub fn frame(&self) -> u64 {
        self.frame
    }

    pub fn reset(&mut self) {
        self.curr.clear();
        self.prev.clear();
        self.next.clear();
        self.sources.clear();
        self.frame = 0;
    }

    pub fn drop(&mut self, x: f32, y: f32) {
        if self.mask.solid(x, y) {
            return;
        }
        let phase = self.rng.below(TRIG_N as u64) as f32;
        self.sources.push(Source {
            x,
            y,
            born_frame: self.frame,
            phase: FracIndex::new(phase),
        });
    }

    pub fn step(&mut self) {
        let w = self.mask.width();
        let h = self.mask.height();
        let c2 = self.config.c2;
        let damp = self.config.damp;
        let reflect = self.config.reflect;

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

        let amp = self.config.amp;
        let radius = self.stamp_radius as i64;
        let span = (2 * self.stamp_radius + 1) as i64;
        for src in &self.sources {
            let age = (self.frame - src.born_frame) as usize;
            let env = ENVELOPE[age.min(ENVELOPE.len() - 1)];
            let osc = trig::sin_idx(src.phase.index());
            let value = amp * env * osc;
            let sx = src.x.floor() as i64;
            let sy = src.y.floor() as i64;
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

        let phase_step = trig::index_from_turns(self.config.freq / FPS);
        for src in self.sources.iter_mut() {
            src.phase.advance(phase_step);
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
}

fn build_stamp(sigma: f32) -> (Vec<f32>, usize) {
    if (sigma - super::waves_luts::STAMP_SIGMA).abs() < 1e-6 {
        return (STAMP.to_vec(), STAMP_RADIUS);
    }
    let radius = ((sigma * 2.0).ceil() as usize).clamp(1, 8);
    let span = 2 * radius + 1;
    let s2 = sigma * sigma;
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
                    sim.drop(x as f32 + 0.5, y as f32 + 0.5);
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
            sim.mask().width() as f32 / 2.0,
            sim.mask().height() as f32 / 2.0,
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
        let cx = (w / 2) as f32 + 0.5;
        let cy = (h / 2) as f32 + 0.5;
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
            sim.mask().width() as f32 / 2.0,
            sim.mask().height() as f32 / 2.0,
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
        let (cx, cy) = (mask.width() as f32 / 2.0, mask.height() as f32 / 2.0);
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
