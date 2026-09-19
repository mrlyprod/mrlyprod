use mrlycore::errors::Result;
use mrlycore::Rng;
use mrlyfig::{ink, save, Board};
use mrlymath::life::design_mask;
use mrlynum::fft::{convolve_with, embed_kernel, transform};

const SIZE: usize = 256;
const STEPS: usize = 64;
const SEED: u64 = 1729;
const DENSITY: f64 = 0.48;
const CELL: f64 = 3.0;
const STAMP: f64 = 6.0;
const CODE: u128 = 7;
const BASE: usize = 3;
const LEVEL: usize = 3;
const SPAN: usize = 27;
const BUDGET: usize = 512;
const BIRTH: (f64, f64) = (0.283, 0.375);
const SURVIVE: (f64, f64) = (0.283, 0.483);
const BORN: (usize, usize) = (145, 192);
const KEPT: (usize, usize) = (145, 247);

fn soup(seed: u64, density: f64) -> Vec<u8> {
    let mut rng = Rng::new(seed);
    (0..SIZE * SIZE)
        .map(|_| u8::from(rng.chance(density)))
        .collect()
}

fn window(lo: f64, hi: f64) -> Vec<bool> {
    (0..=BUDGET)
        .map(|k| {
            let share = k as f64 / BUDGET as f64;
            lo <= share && share <= hi
        })
        .collect()
}

fn counts(table: &[bool]) -> (usize, usize) {
    let on: Vec<usize> = (0..=BUDGET).filter(|&k| table[k]).collect();
    (on[0], on[on.len() - 1])
}

struct Rule {
    kernel_re: Vec<f64>,
    kernel_im: Vec<f64>,
    born: Vec<bool>,
    kept: Vec<bool>,
    field: Vec<f64>,
}

impl Rule {
    fn new(mask: &[u8]) -> Rule {
        let (kernel_re, kernel_im) = transform(&embed_kernel(mask, SPAN, SIZE), SIZE);
        Rule {
            kernel_re,
            kernel_im,
            born: window(BIRTH.0, BIRTH.1),
            kept: window(SURVIVE.0, SURVIVE.1),
            field: vec![0.0; SIZE * SIZE],
        }
    }
    fn step(&mut self, types: &mut [u8]) {
        for (slot, &t) in self.field.iter_mut().zip(types.iter()) {
            *slot = f64::from(t.min(1));
        }
        let sums = convolve_with(&self.field, &self.kernel_re, &self.kernel_im, SIZE);
        for (slot, &sum) in types.iter_mut().zip(&sums) {
            let n = (sum.round().max(0.0) as usize).min(BUDGET);
            let lives = if *slot != 0 {
                self.kept[n]
            } else {
                self.born[n]
            };
            *slot = u8::from(lives);
        }
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.04);
    let mask = design_mask(2, CODE, BASE, LEVEL)?;
    assert_eq!(mask.shape, vec![SPAN, SPAN]);
    assert_eq!(BASE.pow(LEVEL as u32), SPAN);
    assert_eq!(mask.sum() as usize, BUDGET);
    assert_eq!(mask.get(&[SPAN / 2, SPAN / 2]), 0);
    let mut rule = Rule::new(mask.bytes());
    assert_eq!(counts(&rule.born), BORN);
    assert_eq!(counts(&rule.kept), KEPT);

    let mut types = soup(SEED, DENSITY);
    for _ in 0..STEPS {
        rule.step(&mut types);
    }
    let live = types.iter().filter(|&&t| t != 0).count();
    assert!(live > 0);

    let block = SIZE as f64 * CELL;
    let ox = (area.x + area.w - block).round();
    let oy = (area.y + area.h - block).round();
    for row in 0..SIZE {
        for col in 0..SIZE {
            if types[row * SIZE + col] != 0 {
                let x = ox + col as f64 * CELL;
                let y = oy + row as f64 * CELL;
                board.rect(x, y, CELL, CELL, ink::blue());
            }
        }
    }

    let sx = area.x.round();
    let sy = area.y.round();
    let mut stamped = 0usize;
    for row in 0..SPAN {
        for col in 0..SPAN {
            if mask.get(&[row, col]) == 1 {
                let x = sx + col as f64 * STAMP;
                let y = sy + row as f64 * STAMP;
                board.rect(x, y, STAMP, STAMP, ink::yellow());
                stamped += 1;
            }
        }
    }
    assert_eq!(stamped, BUDGET);
    save("demo-chladni", &board)?;
    Ok(())
}
