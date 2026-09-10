use mrlymath::bang::universe::{orbit, Code};
use mrlymath::life::{Boundary, Counts, Sequence};

/// The Moore neighbourhood's code at side 3 in the plane.
pub const MOORE: Code = 7;
/// The Menger tile's code at side 3 in space.
pub const MENGER: Code = 23;
/// The 26-cell neighbourhood's code at side 3 in space.
pub const MOORE_3D: Code = 127;

/// One seed design: a universe code drawn at a side and a Kronecker level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seed {
    /// The universe code.
    pub code: Code,
    /// The odd side of the level-1 tile.
    pub side: usize,
    /// The Kronecker level.
    pub level: usize,
}

/// One neighbourhood mask: a popped design, or the seed board copied and optionally inverted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask {
    /// The design a code names at a side and level, centre popped.
    Design {
        /// The universe code.
        code: Code,
        /// The odd side.
        side: usize,
        /// The Kronecker level.
        level: usize,
    },
    /// The tessellated seed board itself, centre popped.
    Copy {
        /// Whether the board is inverted before popping.
        inverted: bool,
    },
}

impl Mask {
    /// Returns the mask's printable name.
    pub fn name(&self) -> String {
        match self {
            Mask::Design { code, side, level } if *code == MOORE && *side == 3 && *level == 1 => {
                "moore".to_string()
            }
            Mask::Design { code, side, level } => format!("d{code}s{side}l{level}"),
            Mask::Copy { inverted: false } => "copy".to_string(),
            Mask::Copy { inverted: true } => "copyinv".to_string(),
        }
    }
}

/// One rule: a sequence drawn into both sides with its zero and one rolls, Life, or the whole outer-totalistic space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Birth and survive both drawn from one sequence.
    Drawn {
        /// The sequence behind both sides.
        seq: Sequence,
        /// Whether zero stays in the counts.
        zeros: bool,
        /// Whether one stays in the counts.
        ones: bool,
    },
    /// Conway's B3/S23.
    Life,
    /// Every outer-totalistic rule of the mask, declared for the largest preset only.
    Every,
}

impl Rule {
    /// Returns the rule's printable name.
    pub fn name(&self) -> String {
        match self {
            Rule::Drawn { seq, zeros, ones } => {
                let mut name = seq.name();
                if *zeros {
                    name.push_str("+0");
                }
                if *ones {
                    name.push_str("+1");
                }
                name
            }
            Rule::Life => "b3s23".to_string(),
            Rule::Every => "every".to_string(),
        }
    }
    /// Returns the rule's family: number, design, life or every.
    pub fn family(&self) -> &'static str {
        match self {
            Rule::Drawn { seq, .. } if Sequence::numbers().contains(seq) => "number",
            Rule::Drawn { .. } => "design",
            Rule::Life => "life",
            Rule::Every => "every",
        }
    }
    /// Returns the counts that create a cell.
    pub fn birth(&self) -> Counts {
        match self {
            Rule::Drawn { seq, zeros, ones } => Counts::drawn(*seq, *zeros, *ones),
            Rule::Life => Counts::List(vec![3]),
            Rule::Every => Counts::List(Vec::new()),
        }
    }
    /// Returns the counts that keep a cell.
    pub fn survive(&self) -> Counts {
        match self {
            Rule::Drawn { seq, zeros, ones } => Counts::drawn(*seq, *zeros, *ones),
            Rule::Life => Counts::List(vec![2, 3]),
            Rule::Every => Counts::List(Vec::new()),
        }
    }
}

/// One census preset: every axis the loop sweeps and the cut it stops at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preset {
    /// The preset's name.
    pub name: &'static str,
    /// The dimension the seeds live in.
    pub dimension: usize,
    /// The seed designs.
    pub seeds: Vec<Seed>,
    /// The odd tessellation factors.
    pub tessellations: Vec<usize>,
    /// The masks.
    pub masks: Vec<Mask>,
    /// The rules.
    pub rules: Vec<Rule>,
    /// The boundaries.
    pub boundaries: Vec<Boundary>,
    /// The canvas side every board is padded to.
    pub canvas: usize,
    /// The generation cap.
    pub generations: usize,
}

impl Preset {
    /// Returns the nominal run count before any dedupe.
    pub fn runs(&self) -> usize {
        let rules: usize = self
            .rules
            .iter()
            .map(|rule| if *rule == Rule::Every { 1 << 18 } else { 1 })
            .sum();
        self.seeds.len()
            * self.tessellations.len()
            * self.masks.len()
            * rules
            * self.boundaries.len()
    }
    /// Returns the preset as printable lines.
    pub fn describe(&self) -> Vec<String> {
        let seeds: Vec<String> = self
            .seeds
            .iter()
            .map(|s| format!("{}s{}l{}", s.code, s.side, s.level))
            .collect();
        let masks: Vec<String> = self.masks.iter().map(Mask::name).collect();
        let rules: Vec<String> = self.rules.iter().map(Rule::name).collect();
        let bounds: Vec<String> = self
            .boundaries
            .iter()
            .map(|b| format!("{b:?}").to_lowercase())
            .collect();
        vec![
            format!("preset {} dimension {}", self.name, self.dimension),
            format!("seeds {} : {}", seeds.len(), seeds.join(" ")),
            format!("tessellations {:?}", self.tessellations),
            format!("masks {} : {}", masks.len(), masks.join(" ")),
            format!("rules {} : {}", rules.len(), rules.join(" ")),
            format!("boundaries {}", bounds.join(" ")),
            format!("canvas {} generations {}", self.canvas, self.generations),
            format!("nominal runs {}", self.runs()),
        ]
    }
}

/// Returns the smallest code of every non-empty orbit of a dimension's designs.
pub fn classes(dimension: usize) -> Vec<Code> {
    let total: Code = 1 << (1 << dimension);
    (1..total)
        .filter(|&code| orbit(code, dimension).iter().next() == Some(&code))
        .collect()
}

fn drawn(zeros: bool, ones: bool) -> Vec<Rule> {
    Sequence::all()
        .into_iter()
        .map(|seq| Rule::Drawn { seq, zeros, ones })
        .collect()
}

/// The mini preset: side-3 classes at level 1, tessellation 1 and 3, side-3 masks and Copy, rolls off, Wrap, canvas 27, 64 generations.
pub fn mini() -> Preset {
    let seeds = classes(2)
        .into_iter()
        .map(|code| Seed {
            code,
            side: 3,
            level: 1,
        })
        .collect();
    let mut masks = vec![Mask::Design {
        code: MOORE,
        side: 3,
        level: 1,
    }];
    masks.extend(
        classes(2)
            .into_iter()
            .filter(|&code| code != MOORE)
            .map(|code| Mask::Design {
                code,
                side: 3,
                level: 1,
            }),
    );
    masks.push(Mask::Copy { inverted: false });
    masks.push(Mask::Copy { inverted: true });
    let mut rules = drawn(false, false);
    rules.push(Rule::Life);
    Preset {
        name: "mini",
        dimension: 2,
        seeds,
        tessellations: vec![1, 3],
        masks,
        rules,
        boundaries: vec![Boundary::Wrap],
        canvas: 27,
        generations: 64,
    }
}

/// The medi preset: sides 3 and 5 at levels 1 and 2, tessellation to 5, masks to side 9 and Copy, all rolls, both boundaries, canvas 243, 256 generations.
pub fn medi() -> Preset {
    let mut seeds = Vec::new();
    for side in [3, 5] {
        for level in [1, 2] {
            seeds.extend(
                classes(2)
                    .into_iter()
                    .map(|code| Seed { code, side, level }),
            );
        }
    }
    let mut masks = Vec::new();
    for side in [3, 5, 7, 9] {
        masks.extend(classes(2).into_iter().map(|code| Mask::Design {
            code,
            side,
            level: 1,
        }));
    }
    masks.push(Mask::Copy { inverted: false });
    masks.push(Mask::Copy { inverted: true });
    let mut rules = Vec::new();
    for zeros in [false, true] {
        for ones in [false, true] {
            rules.extend(drawn(zeros, ones));
        }
    }
    rules.push(Rule::Life);
    Preset {
        name: "medi",
        dimension: 2,
        seeds,
        tessellations: vec![1, 3, 5],
        masks,
        rules,
        boundaries: vec![Boundary::Wrap, Boundary::Constant],
        canvas: 243,
        generations: 256,
    }
}

/// The maxi preset: the side-3 classes in space, the Menger and Moore masks to level 3, tessellation to 11, every outer-totalistic rule, canvas 243, 1024 generations.
pub fn maxi() -> Preset {
    let seeds = classes(3)
        .into_iter()
        .map(|code| Seed {
            code,
            side: 3,
            level: 1,
        })
        .collect();
    let mut masks = Vec::new();
    for level in [1, 2, 3] {
        masks.push(Mask::Design {
            code: MENGER,
            side: 3,
            level,
        });
    }
    masks.push(Mask::Design {
        code: MOORE_3D,
        side: 3,
        level: 1,
    });
    Preset {
        name: "maxi",
        dimension: 3,
        seeds,
        tessellations: vec![1, 3, 5, 7, 9, 11],
        masks,
        rules: vec![Rule::Every],
        boundaries: vec![Boundary::Wrap, Boundary::Constant],
        canvas: 243,
        generations: 1024,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_plane_has_five_filled_classes_and_the_mini_preset_a_thousand_runs() {
        assert_eq!(classes(2), vec![1, 3, 6, 7, 15]);
        assert_eq!(classes(3).len(), 21);
        let preset = mini();
        assert_eq!(preset.masks[0].name(), "moore");
        assert_eq!(preset.rules.len(), 24);
        assert_eq!(preset.runs(), 5 * 2 * 7 * 24);
        assert_eq!(maxi().rules[0].family(), "every");
    }
}
