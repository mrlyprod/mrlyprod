use super::mask::Mask;
use super::rng::Rng;
use super::trig::{self, N as TRIG_N};

const SPAWN_SPEED: f32 = 0.4;
const MAX_SUBSTEPS: usize = 8;
const SUBSTEP: f32 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BilliardsConfig {
    pub speed: f32,
    pub trail: f32,
    pub size: f32,
    pub count: usize,
}

impl Default for BilliardsConfig {
    fn default() -> BilliardsConfig {
        BilliardsConfig {
            speed: 1.0,
            trail: 0.1,
            size: 1.5,
            count: 16,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Billiards {
    mask: Mask,
    config: BilliardsConfig,
    particles: Vec<Particle>,
    rng: Rng,
}

impl Billiards {
    pub fn new(mask: Mask, config: BilliardsConfig, seed: u64) -> Billiards {
        Billiards {
            mask,
            config,
            particles: Vec::new(),
            rng: Rng::new(seed),
        }
    }

    pub fn config(&self) -> &BilliardsConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: BilliardsConfig) {
        self.config = config;
    }

    pub fn mask(&self) -> &Mask {
        &self.mask
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn reset(&mut self) {
        self.particles.clear();
    }

    pub fn load_particles(&mut self, particles: Vec<Particle>) {
        self.particles = particles;
    }

    pub fn spawn(&mut self, x: f32, y: f32) {
        if self.mask.solid(x, y) {
            return;
        }
        let n = self.config.count.max(1);
        let stride = TRIG_N / n;
        let start = self.rng.below(TRIG_N as u64) as usize;
        for i in 0..n {
            let idx = (start + i * stride) % TRIG_N;
            let (ux, uy) = trig::unit(idx);
            self.particles.push(Particle {
                x,
                y,
                vx: ux * SPAWN_SPEED,
                vy: uy * SPAWN_SPEED,
            });
        }
    }

    pub fn step(&mut self, dt: f32) {
        let total = dt * self.config.speed;
        for part in self.particles.iter_mut() {
            let mut remaining = total;
            let mut safety = 0;
            while remaining > 0.0 && safety < MAX_SUBSTEPS {
                safety += 1;
                let s = remaining.min(SUBSTEP);
                let nx = part.x + part.vx * s;
                let ny = part.y + part.vy * s;
                let wxn = self.mask.solid(nx, part.y);
                let wyn = self.mask.solid(part.x, ny);
                if wxn && wyn {
                    part.vx = -part.vx;
                    part.vy = -part.vy;
                } else if wxn {
                    part.vx = -part.vx;
                } else if wyn {
                    part.vy = -part.vy;
                } else {
                    part.x = nx;
                    part.y = ny;
                }
                remaining -= s;
            }
        }
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
        let mut sim = Billiards::new(mask, BilliardsConfig::default(), 7);
        let mut spawned_in_wall = false;
        'outer: for y in 0..sim.mask().height() {
            for x in 0..sim.mask().width() {
                if sim.mask().solid(x as f32 + 0.5, y as f32 + 0.5) {
                    sim.spawn(x as f32 + 0.5, y as f32 + 0.5);
                    spawned_in_wall = true;
                    break 'outer;
                }
            }
        }
        assert!(spawned_in_wall);
        assert_eq!(sim.particles().len(), 0);
    }

    #[test]
    fn spawn_fan_count() {
        let (mask, cx, cy) = open_mask_and_point();
        let cfg = BilliardsConfig {
            count: 8,
            ..Default::default()
        };
        let mut sim = Billiards::new(mask, cfg, 1);
        assert!(!sim.mask().solid(cx, cy));
        sim.spawn(cx, cy);
        assert_eq!(sim.particles().len(), 8);
        for p in sim.particles() {
            let speed = (p.vx * p.vx + p.vy * p.vy).sqrt();
            assert!((speed - SPAWN_SPEED).abs() < 1e-4);
        }
    }

    #[test]
    fn reflects_off_a_known_wall() {
        let mask = Mask::build("carpet", 3, 1, 0, 1, true).unwrap();
        let mut sim = Billiards::new(mask, BilliardsConfig::default(), 0);
        let w = sim.mask().width() as f32;
        let mut placed = false;
        for y in 0..sim.mask().height() {
            let yy = y as f32 + 0.5;
            if !sim.mask().solid(w - 1.5, yy) {
                sim.particles.push(Particle {
                    x: w - 1.5,
                    y: yy,
                    vx: 0.4,
                    vy: 0.0,
                });
                placed = true;
                break;
            }
        }
        assert!(placed, "no open cell near right edge");
        for _ in 0..50 {
            sim.step(1.0);
        }
        assert!(sim.particles()[0].vx < 0.0);
        assert!(!sim.mask().solid(sim.particles()[0].x, sim.particles()[0].y));
    }

    #[test]
    fn deterministic_with_seed() {
        let (mask, cx, cy) = open_mask_and_point();
        let mut a = Billiards::new(mask.clone(), BilliardsConfig::default(), 42);
        let mut b = Billiards::new(mask, BilliardsConfig::default(), 42);
        a.spawn(cx, cy);
        b.spawn(cx, cy);
        for _ in 0..20 {
            a.step(1.0);
            b.step(1.0);
        }
        assert_eq!(a.particles(), b.particles());
    }
}
