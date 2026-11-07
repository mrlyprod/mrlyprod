use crate::engine::{Grid, Mode, Run};
use crate::recipe::{Mask, Recipe, Shape, shapes};
use crate::source::{Counts, Source};
use mrly_error::{Result, value_error};
use mrly_rand::Rng;

pub const X: u32 = 256;
pub const CAP: u32 = 151;
pub const LASTS: u32 = 16;
pub const SIDES: [usize; 3] = [3, 5, 7];
pub const LEVELS: [usize; 2] = [1, 2];
pub const TILINGS: [usize; 3] = [1, 3, 5];
pub const MASK_SIDES: [usize; 4] = [3, 5, 7, 9];
pub const MASK_LEVELS: [usize; 2] = [1, 2];
pub const ROLLS: [(bool, bool); 4] = [(false, false), (true, false), (false, true), (true, true)];

const PAIRS: [(Source, Source); 8] = [
    (Source::CarpetFills, Source::CarpetVoids),
    (Source::NetFills, Source::NetVoids),
    (Source::TreeFills, Source::TreeVoids),
    (Source::VoidFills, Source::VoidVoids),
    (Source::PointFills, Source::PointVoids),
    (Source::DustFills, Source::DustVoids),
    (Source::LineFills, Source::LineVoids),
    (Source::StarFills, Source::StarVoids),
];

const TRIES: usize = 4096;

#[derive(Clone, Debug)]
struct Slot {
    tiling: usize,
    mode: Mode,
    copies: Vec<Mask>,
}

#[derive(Clone, Debug)]
struct Seed {
    shape: Shape,
    side: usize,
    level: usize,
    slots: Vec<Slot>,
}

#[derive(Clone, Debug)]
pub struct Catalog {
    x: u32,
    cap: u32,
    seeds: Vec<Seed>,
    masks: Vec<Mask>,
    rules: Vec<(Counts, Counts)>,
    total: usize,
}

fn distinct(side: usize, level: usize) -> Result<Vec<(Shape, Grid)>> {
    let mut out: Vec<(Shape, Grid)> = Vec::new();
    for shape in shapes() {
        let grid = shape.build(side, level)?;
        if !grid.is_empty() && !out.iter().any(|(_, g)| *g == grid) {
            out.push((shape, grid));
        }
    }
    Ok(out)
}

fn copies(board: &Grid, designs: &[Grid]) -> Vec<Mask> {
    [false, true]
        .into_iter()
        .filter(|&inverted| {
            let grid = if inverted {
                board.invert().popped()
            } else {
                board.popped()
            };
            !grid.is_empty() && !designs.contains(&grid)
        })
        .map(|inverted| Mask::Copy { inverted })
        .collect()
}

fn rules() -> Vec<(Counts, Counts)> {
    let mut out = Vec::new();
    for seq in Source::all() {
        for (zeros, ones) in ROLLS {
            out.push((
                Counts::drawn(seq, zeros, ones),
                Counts::drawn(seq, zeros, ones),
            ));
        }
    }
    out.push((Counts::list(vec![3]), Counts::list(vec![2, 3])));
    for (fills, voids) in PAIRS {
        for (zeros, ones) in ROLLS {
            out.push((
                Counts::drawn(fills, zeros, ones),
                Counts::drawn(voids, zeros, ones),
            ));
        }
    }
    out
}

impl Catalog {
    pub fn new(x: u32, cap: u32) -> Result<Catalog> {
        let mut masks = Vec::new();
        let mut designs: Vec<Grid> = Vec::new();
        for side in MASK_SIDES {
            for level in MASK_LEVELS {
                for (shape, grid) in distinct(side, level)? {
                    let popped = grid.popped();
                    if !popped.is_empty() && !designs.contains(&popped) {
                        masks.push(Mask::Design { shape, side, level });
                        designs.push(popped);
                    }
                }
            }
        }
        let rules = rules();
        let mut seeds = Vec::new();
        let mut total = 0;
        for side in SIDES {
            for level in LEVELS {
                for (shape, grid) in distinct(side, level)? {
                    let mut slots: Vec<Slot> = TILINGS
                        .iter()
                        .map(|&tiling| Slot {
                            tiling,
                            mode: Mode::Island,
                            copies: copies(&grid.tile(tiling), &designs),
                        })
                        .collect();
                    slots.push(Slot {
                        tiling: 1,
                        mode: Mode::Wallpaper,
                        copies: copies(&grid, &designs),
                    });
                    total += slots
                        .iter()
                        .map(|slot| (masks.len() + slot.copies.len()) * rules.len())
                        .sum::<usize>();
                    seeds.push(Seed {
                        shape,
                        side,
                        level,
                        slots,
                    });
                }
            }
        }
        Ok(Catalog {
            x,
            cap,
            seeds,
            masks,
            rules,
            total,
        })
    }

    pub fn standard() -> Result<Catalog> {
        Catalog::new(X, CAP)
    }

    pub fn count(&self) -> usize {
        self.total
    }

    pub fn recipe(&self, index: usize) -> Result<Recipe> {
        let mut rest = index;
        for seed in &self.seeds {
            for slot in &seed.slots {
                let block = (self.masks.len() + slot.copies.len()) * self.rules.len();
                if rest >= block {
                    rest -= block;
                    continue;
                }
                let (at, rule) = (rest / self.rules.len(), rest % self.rules.len());
                let mask = self
                    .masks
                    .get(at)
                    .unwrap_or_else(|| &slot.copies[at - self.masks.len()]);
                let (birth, survive) = &self.rules[rule];
                return Ok(Recipe {
                    seed: seed.shape.token(),
                    side: seed.side as u32,
                    level: seed.level as u32,
                    tiling: slot.tiling as u32,
                    mode: slot.mode,
                    mask: mask.token(),
                    birth: birth.token(),
                    survive: survive.token(),
                    x: self.x,
                    cap: if slot.mode == Mode::Island {
                        self.cap
                    } else {
                        0
                    },
                });
            }
        }
        value_error(format!(
            "recipe {index} lies past the catalog of {}.",
            self.total
        ))
    }

    pub fn masks(&self) -> Vec<String> {
        self.masks.iter().map(|mask| mask.token()).collect()
    }

    pub fn rules(&self) -> Vec<[String; 2]> {
        self.rules
            .iter()
            .map(|(birth, survive)| [birth.token(), survive.token()])
            .collect()
    }

    pub fn draw(&self, rng: &mut Rng, least: u32) -> Result<Recipe> {
        for _ in 0..TRIES {
            let recipe = self.recipe(rng.below(self.total))?;
            if least == 0 || lasts(&recipe)? >= least {
                return Ok(recipe);
            }
        }
        value_error(format!(
            "no recipe lasted {least} generations in {TRIES} draws."
        ))
    }
}

pub fn lasts(recipe: &Recipe) -> Result<u32> {
    Ok(Run::new(recipe)?.finish().generation)
}
