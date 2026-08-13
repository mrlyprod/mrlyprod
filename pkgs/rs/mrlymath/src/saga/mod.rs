mod factory;
mod metrics;
mod ops;

/// The guided search that reads a saga back off example pairs, with its floors.
pub mod solve;

pub use factory::{pose, sample, Dials};
pub use metrics::{chaos, spread};
pub use ops::Op;

use crate::name::Named;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::{Dtype, Tensor};

/// The largest side a saga grid may have.
pub const SIDE: usize = 30;

/// The number of pens a saga grid may hold, zero being the background.
pub const COLORS: usize = 10;

/// The largest rep count one saga step may carry.
pub const REPS: usize = 99;

const RULES: [(&[usize], &[usize]); 4] = [
    (&[3], &[2, 3]),
    (&[3, 6], &[2, 3]),
    (&[2], &[]),
    (&[1], &[1]),
];

/// Checks that a tensor is a legal saga grid, or says how it strays.
pub fn check(grid: &Tensor) -> Result<()> {
    if grid.dtype() != Dtype::U8 {
        return value_error("a saga grid holds u8 cells.");
    }
    if grid.shape.len() != 2 {
        return value_error("a saga grid has two axes.");
    }
    if grid.shape.iter().any(|&n| n == 0 || n > SIDE) {
        return value_error("a saga grid keeps its sides in 1..=30.");
    }
    if grid.bytes().iter().any(|&cell| cell as usize >= COLORS) {
        return value_error("a saga grid keeps its cells in 0..=9.");
    }
    Ok(())
}

/// Builds a saga grid from rows of cells, or an error for ragged or illegal rows.
pub fn from_rows(rows: &[Vec<u8>]) -> Result<Tensor> {
    let h = rows.len();
    let w = rows.first().map(Vec::len).unwrap_or(0);
    if rows.iter().any(|row| row.len() != w) {
        return value_error("a saga grid keeps every row the same width.");
    }
    let cells: Vec<u8> = rows.iter().flatten().copied().collect();
    let grid = Tensor::of(cells, vec![h, w]);
    check(&grid)?;
    Ok(grid)
}

/// Returns a grid's cells as rows.
pub fn to_rows(grid: &Tensor) -> Vec<Vec<u8>> {
    let w = grid.shape[1];
    grid.bytes().chunks(w).map(<[u8]>::to_vec).collect()
}

/// A short grid program: ops run in order, each repeated its rep count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Saga {
    /// The ops and how many times each one runs.
    pub steps: Vec<(Op, usize)>,
}

impl Default for Saga {
    fn default() -> Saga {
        Saga::new()
    }
}

impl Saga {
    /// Builds the empty saga, which leaves every grid unchanged.
    pub fn new() -> Saga {
        Saga { steps: Vec::new() }
    }
    /// Builds a saga running each op once.
    pub fn of(ops: Vec<Op>) -> Saga {
        Saga {
            steps: ops.into_iter().map(|op| (op, 1)).collect(),
        }
    }
    /// Appends an op with its rep count.
    pub fn push(&mut self, op: Op, reps: usize) {
        self.steps.push((op, reps));
    }
    /// Counts the op applications the saga makes when run.
    pub fn len(&self) -> usize {
        self.steps.iter().map(|(_, reps)| reps).sum()
    }
    /// Returns true when the saga holds no ops.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Prints the saga as a short program: op tokens joined by underscores,
    /// a trailing `xN` marking reps above one, and `id` for the empty saga.
    pub fn name(&self) -> String {
        if self.steps.is_empty() {
            return "id".to_string();
        }
        let tokens: Vec<String> = self
            .steps
            .iter()
            .map(|(op, reps)| match reps {
                0 | 1 => op.name(),
                n => format!("{}x{n}", op.name()),
            })
            .collect();
        tokens.join("_")
    }
    /// Parses a saga program back from its canonical name, or an error for any other spelling.
    pub fn parse(text: &str) -> Result<Saga> {
        if text == "id" {
            return Ok(Saga::new());
        }
        if text.is_empty() {
            return value_error("a saga name holds at least one token.");
        }
        let mut saga = Saga::new();
        for token in text.split('_') {
            let (body, reps) = match token.split_once('x') {
                Some((body, digits)) => {
                    let Ok(reps) = digits.parse::<usize>() else {
                        return value_error(format!("token {token:?} carries a bad rep mark."));
                    };
                    if !(2..=REPS).contains(&reps) {
                        return value_error(format!("token {token:?} keeps its reps in 2..=99."));
                    }
                    (body, reps)
                }
                None => (token, 1),
            };
            saga.push(Op::parse(body)?, reps);
        }
        Ok(saga)
    }
}

impl Named for Saga {
    fn to_str(&self) -> String {
        format!("mrly_saga_{}", self.name())
    }
    fn from_str(text: &str) -> Result<Saga> {
        let Some(body) = text.strip_prefix("mrly_saga_") else {
            return value_error(format!("name {text:?} does not start with \"mrly_saga_\"."));
        };
        Saga::parse(body)
    }
}

/// Runs a saga from a grid and returns every frame, the input first,
/// or the first error a bad grid, a zero rep or an illegal op raises.
pub fn run(grid: &Tensor, saga: &Saga) -> Result<Vec<Tensor>> {
    check(grid)?;
    let mut frames = vec![grid.clone()];
    for (op, reps) in &saga.steps {
        if *reps == 0 {
            return value_error("a rep count of zero stalls the saga.");
        }
        if *reps > REPS {
            return value_error("a rep count leaves 1..=99.");
        }
        for _ in 0..*reps {
            let next = op.apply(frames.last().expect("frames open with the input"))?;
            frames.push(next);
        }
    }
    Ok(frames)
}

/// Runs a saga from a grid and returns the last frame alone, or the first error raised.
pub fn out(grid: &Tensor, saga: &Saga) -> Result<Tensor> {
    let mut frames = run(grid, saga)?;
    Ok(frames.pop().expect("frames open with the input"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cross() -> Tensor {
        from_rows(&[vec![0, 1, 0], vec![1, 2, 1], vec![0, 1, 0]]).unwrap()
    }

    #[test]
    fn rows_round_trip() {
        let rows = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let grid = from_rows(&rows).unwrap();
        assert_eq!(grid.shape, vec![2, 3]);
        assert_eq!(to_rows(&grid), rows);
    }
    #[test]
    fn bad_rows_are_errors() {
        assert!(from_rows(&[]).is_err());
        assert!(from_rows(&[vec![1, 2], vec![3]]).is_err());
        assert!(from_rows(&[vec![10]]).is_err());
        assert!(from_rows(&[vec![0; 31]]).is_err());
    }
    #[test]
    fn check_guards_the_board_rules() {
        assert!(check(&cross()).is_ok());
        assert!(check(&Tensor::new(vec![3])).is_err());
        assert!(check(&Tensor::new(vec![0, 3])).is_err());
        assert!(check(&Tensor::typed(vec![2, 2], Dtype::U16)).is_err());
        assert!(check(&Tensor::full(vec![2, 2], 10)).is_err());
    }
    #[test]
    fn run_keeps_every_frame_and_replays_exactly() {
        let saga = Saga::parse("rot1x2_refh").unwrap();
        let frames = run(&cross(), &saga).unwrap();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], cross());
        assert_eq!(run(&cross(), &saga).unwrap(), frames);
        assert_eq!(out(&cross(), &saga).unwrap(), frames[3]);
    }
    #[test]
    fn the_empty_saga_is_the_identity() {
        let frames = run(&cross(), &Saga::new()).unwrap();
        assert_eq!(frames, vec![cross()]);
        assert_eq!(Saga::new().name(), "id");
        assert_eq!(Saga::parse("id").unwrap(), Saga::new());
    }
    #[test]
    fn zero_reps_stall_the_saga() {
        let mut saga = Saga::new();
        saga.push(Op::Transpose, 0);
        assert!(run(&cross(), &saga).is_err());
    }
    #[test]
    fn saga_names_round_trip() {
        for name in [
            "rot2",
            "refh_refv",
            "tsp_cropc0r1w2h1",
            "auto_padn1c0",
            "tilew2h1_scalek2",
            "map1023456789",
            "movr1d0f0_paintc0r0p3",
            "floodc1r1p4",
            "stepp1b3s23x5",
            "stepp2b36s23w",
            "stampw2d0123",
        ] {
            let saga = Saga::parse(name).unwrap();
            assert_eq!(saga.name(), name);
            assert_eq!(Saga::from_str(&saga.to_str()).unwrap(), saga);
        }
    }
    #[test]
    fn stray_spellings_do_not_parse() {
        for bad in [
            "",
            "rot0",
            "rot4",
            "rot1x1",
            "rot1x0",
            "rot1x100",
            "rot1xx2",
            "id_rot1",
            "rot1_",
            "spin3",
            "REFH",
            "mrly_saga_rot1",
        ] {
            assert!(Saga::parse(bad).is_err(), "{bad}");
        }
        assert!(Saga::from_str("rot1").is_err());
    }
    #[test]
    fn a_saga_prints_reps_compactly() {
        let mut saga = Saga::new();
        saga.push(Op::Rotate { k: 1 }, 3);
        saga.push(Op::Transpose, 1);
        assert_eq!(saga.name(), "rot1x3_tsp");
        assert_eq!(saga.len(), 4);
        assert!(!saga.is_empty());
    }
}
