use super::mask::Mask;
use super::rng::Rng;
use super::trig::{self, N as TRIG_N};

pub const MILLI: i64 = 1000;
pub const CENTI: i64 = 100;

const SPAWN_SPEED: i64 = 400;
const MAX_SUBSTEPS: usize = 8;
const SUBSTEP: i64 = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Particle {
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BilliardsConfig {
    pub speed: i64,
    pub trail: i64,
    pub size: i64,
    pub count: usize,
}

impl Default for BilliardsConfig {
    fn default() -> BilliardsConfig {
        BilliardsConfig {
            speed: 100,
            trail: 10,
            size: 150,
            count: 16,
        }
    }
}

pub fn cell(milli: i64) -> i64 {
    milli.div_euclid(MILLI)
}

pub fn solid_cell(mask: &Mask, x: i64, y: i64) -> bool {
    if x < 0 || y < 0 || x >= mask.width() as i64 || y >= mask.height() as i64 {
        return true;
    }
    mask.cell().types().get(&[y as usize, x as usize]) == 1
}

fn unit_milli(i: usize) -> (i64, i64) {
    let (ux, uy) = trig::unit(i);
    (
        (ux * MILLI as f32).round() as i64,
        (uy * MILLI as f32).round() as i64,
    )
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

    pub fn spawn(&mut self, x: i64, y: i64) {
        if solid_cell(&self.mask, cell(x), cell(y)) {
            return;
        }
        let n = self.config.count.max(1);
        let stride = (TRIG_N / n).max(1);
        let start = self.rng.below(TRIG_N as u64) as usize;
        for i in 0..n {
            let idx = (start + i * stride) % TRIG_N;
            let (ux, uy) = unit_milli(idx);
            self.particles.push(Particle {
                x,
                y,
                vx: ux * SPAWN_SPEED / MILLI,
                vy: uy * SPAWN_SPEED / MILLI,
            });
        }
    }

    pub fn step(&mut self, dt: i64) {
        let total = dt * self.config.speed / CENTI;
        for part in self.particles.iter_mut() {
            let mut remaining = total;
            let mut safety = 0;
            while remaining > 0 && safety < MAX_SUBSTEPS {
                safety += 1;
                let s = remaining.min(SUBSTEP);
                let nx = part.x + part.vx * s / MILLI;
                let ny = part.y + part.vy * s / MILLI;
                let wxn = solid_cell(&self.mask, cell(nx), cell(part.y));
                let wyn = solid_cell(&self.mask, cell(part.x), cell(ny));
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

    fn open_mask_and_point() -> (Mask, i64, i64) {
        let m = Mask::build("carpet", 3, 2, 6, 1, false).unwrap();
        (m, 1500, 1500)
    }

    #[test]
    fn spawn_ignored_inside_wall() {
        let mask = Mask::build("carpet", 3, 2, 0, 1, false).unwrap();
        let mut sim = Billiards::new(mask, BilliardsConfig::default(), 7);
        let mut spawned_in_wall = false;
        'outer: for y in 0..sim.mask().height() {
            for x in 0..sim.mask().width() {
                if solid_cell(sim.mask(), x as i64, y as i64) {
                    sim.spawn(x as i64 * MILLI + 500, y as i64 * MILLI + 500);
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
        assert!(!solid_cell(sim.mask(), cell(cx), cell(cy)));
        sim.spawn(cx, cy);
        assert_eq!(sim.particles().len(), 8);
        for p in sim.particles() {
            let speed = p.vx * p.vx + p.vy * p.vy;
            assert!((speed - SPAWN_SPEED * SPAWN_SPEED).abs() < 2000);
        }
    }

    #[test]
    fn reflects_off_a_known_wall() {
        let mask = Mask::build("carpet", 3, 1, 0, 1, true).unwrap();
        let mut sim = Billiards::new(mask, BilliardsConfig::default(), 0);
        let w = sim.mask().width() as i64;
        let mut placed = false;
        for y in 0..sim.mask().height() {
            let yy = y as i64 * MILLI + 500;
            if !solid_cell(sim.mask(), w - 2, cell(yy)) {
                sim.particles.push(Particle {
                    x: (w - 2) * MILLI + 500,
                    y: yy,
                    vx: 400,
                    vy: 0,
                });
                placed = true;
                break;
            }
        }
        assert!(placed, "no open cell near right edge");
        for _ in 0..50 {
            sim.step(MILLI);
        }
        assert!(sim.particles()[0].vx < 0);
        let p = sim.particles()[0];
        assert!(!solid_cell(sim.mask(), cell(p.x), cell(p.y)));
    }

    #[test]
    fn deterministic_with_seed() {
        let (mask, cx, cy) = open_mask_and_point();
        let mut a = Billiards::new(mask.clone(), BilliardsConfig::default(), 42);
        let mut b = Billiards::new(mask, BilliardsConfig::default(), 42);
        a.spawn(cx, cy);
        b.spawn(cx, cy);
        for _ in 0..20 {
            a.step(MILLI);
            b.step(MILLI);
        }
        assert_eq!(a.particles(), b.particles());
    }

    #[test]
    fn particles_actually_travel() {
        let (mask, cx, cy) = open_mask_and_point();
        let mut sim = Billiards::new(mask, BilliardsConfig::default(), 3);
        sim.spawn(cx, cy);
        let before: Vec<(i64, i64)> = sim.particles().iter().map(|p| (p.x, p.y)).collect();
        for _ in 0..4 {
            sim.step(MILLI);
        }
        let moved = sim
            .particles()
            .iter()
            .zip(before.iter())
            .filter(|(p, (x, y))| p.x != *x || p.y != *y)
            .count();
        assert!(moved > 0);
    }
}
