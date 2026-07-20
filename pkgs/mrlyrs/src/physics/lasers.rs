use super::mask::Mask;
use super::rng::Rng;
use super::trig::{self, FracIndex, N as TRIG_N};

const STEP_SIZE: f32 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LasersConfig {
    pub rays: usize,
    pub spread_idx: f32,
    pub bounces: i32,
    pub omega_idx: f32,
}

#[inline]
pub fn radians_to_index(radians: f32) -> f32 {
    const TWO_PI: f32 = std::f32::consts::TAU;
    radians / TWO_PI * TRIG_N as f32
}

impl Default for LasersConfig {
    fn default() -> LasersConfig {
        LasersConfig {
            rays: 32,
            spread_idx: TRIG_N as f32,
            bounces: 16,
            omega_idx: radians_to_index(0.3),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Emitter {
    pub x: f32,
    pub y: f32,
    pub dir: FracIndex,
    pub omega_idx: f32,
    pub spread_idx: f32,
    pub rays: usize,
    pub bounces: i32,
}

#[derive(Clone, Debug)]
pub struct Lasers {
    mask: Mask,
    config: LasersConfig,
    emitters: Vec<Emitter>,
    rng: Rng,
}

impl Lasers {
    pub fn new(mask: Mask, config: LasersConfig, seed: u64) -> Lasers {
        Lasers {
            mask,
            config,
            emitters: Vec::new(),
            rng: Rng::new(seed),
        }
    }

    pub fn config(&self) -> &LasersConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: LasersConfig) {
        self.config = config;
    }

    pub fn mask(&self) -> &Mask {
        &self.mask
    }

    pub fn emitters(&self) -> &[Emitter] {
        &self.emitters
    }

    pub fn reset(&mut self) {
        self.emitters.clear();
    }

    pub fn load_emitters(&mut self, emitters: Vec<Emitter>) {
        self.emitters = emitters;
    }

    pub fn spawn(&mut self, x: f32, y: f32) {
        if self.mask.solid(x, y) {
            return;
        }
        let start = self.rng.below(TRIG_N as u64) as f32;
        self.emitters.push(Emitter {
            x,
            y,
            dir: FracIndex::new(start),
            omega_idx: self.config.omega_idx,
            spread_idx: self.config.spread_idx,
            rays: self.config.rays,
            bounces: self.config.bounces,
        });
    }

    pub fn step(&mut self, dt: f32) {
        for em in self.emitters.iter_mut() {
            em.dir.advance(em.omega_idx * dt);
        }
    }

    pub fn trace_all(&self) -> Vec<Vec<(f32, f32)>> {
        let mut out = Vec::new();
        for em in &self.emitters {
            let base = em.dir.value;
            let half = em.spread_idx / 2.0;
            for i in 0..em.rays {
                let t = if em.rays == 1 {
                    0.5
                } else {
                    i as f32 / (em.rays - 1) as f32
                };
                let a = base - half + em.spread_idx * t;
                let idx = FracIndex::new(a).index();
                out.push(self.trace(em.x, em.y, idx, em.bounces));
            }
        }
        out
    }

    pub fn trace(&self, mut x: f32, mut y: f32, dir_idx: usize, bounces: i32) -> Vec<(f32, f32)> {
        let (mut dx, mut dy) = trig::unit(dir_idx);
        let max_steps = (self.mask.width() + self.mask.height()) * 4;
        let mut poly = Vec::with_capacity(8);
        poly.push((x, y));
        let mut budget = bounces;

        for _ in 0..max_steps {
            let nx = x + dx * STEP_SIZE;
            let ny = y + dy * STEP_SIZE;
            if nx < 0.0
                || ny < 0.0
                || nx >= self.mask.width() as f32
                || ny >= self.mask.height() as f32
            {
                poly.push((nx, ny));
                return poly;
            }
            let wxn = self.mask.solid(nx, y);
            let wyn = self.mask.solid(x, ny);
            if wxn || wyn {
                poly.push((x, y));
                if wxn && wyn {
                    dx = -dx;
                    dy = -dy;
                } else if wxn {
                    dx = -dx;
                } else {
                    dy = -dy;
                }
                budget -= 1;
                if budget < 0 {
                    return poly;
                }
                continue;
            }
            x = nx;
            y = ny;
        }
        poly.push((x, y));
        poly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_mask_and_point() -> (Mask, f32, f32) {
        let m = Mask::build("carpet", 3, 2, 6, 1, false).unwrap();
        (m, 1.5, 1.5)
    }

    #[test]
    fn spawn_ignored_inside_wall() {
        let mask = Mask::build("carpet", 3, 2, 0, 1, false).unwrap();
        let mut sim = Lasers::new(mask, LasersConfig::default(), 3);
        'outer: for y in 0..sim.mask().height() {
            for x in 0..sim.mask().width() {
                if sim.mask().solid(x as f32 + 0.5, y as f32 + 0.5) {
                    sim.spawn(x as f32 + 0.5, y as f32 + 0.5);
                    break 'outer;
                }
            }
        }
        assert_eq!(sim.emitters().len(), 0);
    }

    #[test]
    fn trace_emits_polyline() {
        let (mask, cx, cy) = open_mask_and_point();
        let sim = Lasers::new(mask, LasersConfig::default(), 1);
        let poly = sim.trace(cx, cy, 0, 1);
        assert!(poly.len() >= 2);
        assert_eq!(poly[0], (cx, cy));
    }

    #[test]
    fn ray_bounces_off_wall_adds_vertices() {
        let (mask, _, _) = open_mask_and_point();
        let sim = Lasers::new(mask, LasersConfig::default(), 0);
        let w = sim.mask().width() as f32;
        let poly = sim.trace(w - 1.5, 1.5, 0, 16);
        assert!(poly.len() >= 2);
    }

    #[test]
    fn rays_count_matches_config() {
        let (mask, cx, cy) = open_mask_and_point();
        let cfg = LasersConfig {
            rays: 16,
            ..Default::default()
        };
        let mut sim = Lasers::new(mask, cfg, 7);
        sim.spawn(cx, cy);
        let polys = sim.trace_all();
        assert_eq!(polys.len(), 16);
    }

    #[test]
    fn spin_advances_direction() {
        let (mask, cx, cy) = open_mask_and_point();
        let cfg = LasersConfig {
            omega_idx: 4.0,
            ..Default::default()
        };
        let mut sim = Lasers::new(mask, cfg, 2);
        sim.spawn(cx, cy);
        let before = sim.emitters()[0].dir.value;
        sim.step(1.0);
        let after = sim.emitters()[0].dir.value;
        assert!(
            (after - before - 4.0).abs() < 1e-3
                || (after - before + TRIG_N as f32 - 4.0).abs() < 1e-3
        );
    }

    #[test]
    fn deterministic_with_seed() {
        let (mask, cx, cy) = open_mask_and_point();
        let mut a = Lasers::new(mask.clone(), LasersConfig::default(), 99);
        let mut b = Lasers::new(mask, LasersConfig::default(), 99);
        a.spawn(cx, cy);
        b.spawn(cx, cy);
        for _ in 0..10 {
            a.step(1.0);
            b.step(1.0);
        }
        assert_eq!(a.emitters(), b.emitters());
        assert_eq!(a.trace_all(), b.trace_all());
    }
}
