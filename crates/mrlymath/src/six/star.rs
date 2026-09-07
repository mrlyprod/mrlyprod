use crate::bang::code_to_corners;
use mrlycore::errors::{value_error, Result};
use mrlynum::series::{CATALAN, EULER};

/// The exact reading of a cut layer: how many cells were inked out of how many were read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Share {
    /// The count of inked cells.
    pub inked: i64,
    /// The count of cells read.
    pub cells: i64,
}

impl Share {
    /// The share as a real number.
    pub fn value(self) -> f64 {
        self.inked as f64 / self.cells as f64
    }

    /// The share in lowest terms, numerator then denominator.
    pub fn reduced(self) -> (i64, i64) {
        let (mut a, mut b) = (self.inked.abs(), self.cells.abs());
        while b != 0 {
            let next = a % b;
            a = b;
            b = next;
        }
        let divisor = a.max(1);
        (self.inked / divisor, self.cells / divisor)
    }
}

/// The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.
///
/// ```
/// use mrlymath::six::star::chi8;
/// assert_eq!([chi8(1), chi8(3), chi8(5), chi8(7), chi8(4)], [1, -1, -1, 1, 0]);
/// ```
pub fn chi8(number: usize) -> i64 {
    [0, 1, 0, -1, 0, -1, 0, 1][number % 8]
}

/// The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.
///
/// The arm `x = y` of the cut is a diameter of `2n` cells with no width to choose, and on it the two
/// block parities agree, so the space rule collapses to `floor(x/4)` even and the inked count is
/// `f(3n) - f(n) = n + chi_8(n)` with `f(m) = 4 floor(m/8) + min(m mod 8, 4)`.
///
/// ```
/// use mrlymath::six::star::arm_law;
/// assert_eq!(arm_law(7).unwrap().reduced(), (4, 7));
/// ```
pub fn arm_law(number: usize) -> Result<Share> {
    if number == 0 || number.is_multiple_of(2) {
        return value_error("the layer number must be odd.");
    }
    Ok(Share {
        inked: number as i64 + chi8(number),
        cells: 2 * number as i64,
    })
}

/// The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.
///
/// Every limb is a closed form the subtraction of the two ink laws hands over: the character part is
/// `L(1, chi_8)/2` by the class number formula for `Q(sqrt 2)`, the square tail is `-G/8` with `G`
/// Catalan's constant, and the harmonic tail carries `-gamma/4 - (ln 2)/2`.
pub fn constant() -> f64 {
    (1.0 + 2f64.sqrt()).ln() / (2.0 * 2f64.sqrt()) - CATALAN / 8.0 - EULER / 4.0 - 2f64.ln() / 2.0
}

/// The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.
///
/// `b` is the block parity of the band edge, one when `floor(K/2)` is even and zero when it is odd.
/// The band's per-layer excess is `kappa chi + (m + q chi_8(n))/n + chi/(8 n^2)`, and summing the
/// `1/n` term over the first `L` odd layers hands this coefficient as `m/2`, so it is a theorem and
/// not a fit. At even `W` it reads `-(W + 2b)/(8(W + 1))` and at odd `W` it reads
/// `-(W - 1 + 2b)/(8W)`, since `x - y` is even on the cut and an odd width is never a new band:
/// `W = 1` returns the arm's `-1/4` and `W = 3` returns `-1/6`. As `W` grows it tends to `-1/8`.
///
/// ```
/// use mrlymath::six::star::width_law;
/// assert_eq!([width_law(0), width_law(2), width_law(4)], [-0.25, -1.0 / 6.0, -0.1]);
/// assert_eq!([width_law(1), width_law(3)], [width_law(0), width_law(2)]);
/// ```
pub fn width_law(half: usize) -> f64 {
    let reach = (half / 2) as f64;
    let parity = f64::from(u8::from((half / 4).is_multiple_of(2)));
    -(reach + parity) / (4.0 * (2.0 * reach + 1.0))
}

/// The three classes of layer count the `1/L^2` term of the decay reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Branch {
    /// `L` divisible by four, where the residual is `-23/192`.
    Zero,
    /// `L` twice an odd number, where the residual is `+25/192`.
    Two,
    /// Odd `L`, where the constant shifts by `1/8` and the error is `O(1/L)`.
    Odd,
}

impl Branch {
    /// The branch of a layer count.
    pub fn of(layers: usize) -> Branch {
        match layers % 4 {
            0 => Branch::Zero,
            2 => Branch::Two,
            _ => Branch::Odd,
        }
    }

    /// The exact `1/L^2` coefficient at even `L`, absent at odd `L`.
    pub fn residual(self) -> Option<f64> {
        match self {
            Branch::Zero => Some(-23.0 / 192.0),
            Branch::Two => Some(25.0 / 192.0),
            Branch::Odd => None,
        }
    }

    /// The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.
    pub fn constant(self) -> f64 {
        constant() + if self == Branch::Odd { 0.125 } else { 0.0 }
    }

    /// The name of the branch.
    pub fn name(self) -> &'static str {
        match self {
            Branch::Zero => "0 mod 4",
            Branch::Two => "2 mod 4",
            Branch::Odd => "odd",
        }
    }
}

/// The `L`-layer reading of the ghost star's decay in the cell frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Decay {
    /// The layer count `L`.
    pub layers: usize,
    /// The mean excess of the star over the background across the first `L` odd layers.
    pub excess: f64,
    /// That excess times `L`.
    pub scaled: f64,
    /// The scaled excess plus `(ln L)/4`, which settles on the branch constant.
    pub logged: f64,
    /// The miss of the settled value against the branch constant.
    pub miss: f64,
    /// The miss times `L`, the reading odd `L` leaves behind.
    pub linear: f64,
    /// The miss times `L` squared, the reading even `L` leaves behind.
    pub residual: f64,
    /// The slope of the scaled excess against `ln L`, read from `L/2` to `L`, absent unless `L` is divisible by four.
    pub slope: Option<f64>,
}

/// The ghost star of a coded cube's hexagonal cut stack, read in the cell frame.
///
/// The cut of the level-one cube of side `n` lives on the cube's own `4n` cell lattice, one layer at
/// a time and nothing resampled, so the half-cell displacement `1/(2n)` between consecutive layers
/// is carried rather than averaged away. A cell `(x, y, z)` of the plane `x + y + z = 6n - 2` is
/// inked when the design's corner mask holds the three block parities `floor(c/4) mod 2`.
pub struct Star {
    corners: [bool; 8],
}

impl Star {
    /// Reads the star of a base-2 space code, the carpet being `23`.
    pub fn new(code: u128) -> Result<Star> {
        let filled = code_to_corners(code, 3, 2)?;
        let mut corners = [false; 8];
        for corner in filled {
            corners[usize::from(corner[0]) << 2
                | usize::from(corner[1]) << 1
                | usize::from(corner[2])] = true;
        }
        Ok(Star { corners })
    }

    fn inked(&self, x: i64, y: i64, z: i64) -> bool {
        let bit = |c: i64| (c.div_euclid(4) & 1) as usize;
        self.corners[bit(x) << 2 | bit(y) << 1 | bit(z)]
    }

    /// The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.
    ///
    /// The answer is absent where the plane `x + y + z = 6n - 2` leaves the cube, which is what draws
    /// the hexagon's own edge.
    pub fn cell(&self, number: usize, x: i64, z: i64) -> Option<bool> {
        let n = number as i64;
        let y = 6 * n - 2 - x - z;
        let inside = (0..4 * n).contains(&x) && (0..4 * n).contains(&z) && (0..4 * n).contains(&y);
        inside.then(|| self.inked(x, y, z))
    }

    /// The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.
    ///
    /// The rows are the `2n` even heights `z`, and each runs over the `x` the plane leaves inside the
    /// cube. The fill depends on `x` only through `x mod 8`, so a row is counted in eight steps and
    /// the whole layer in `O(n)`.
    ///
    /// ```
    /// use mrlymath::six::star::Star;
    /// assert_eq!(Star::new(23).unwrap().hexagon(3).unwrap().reduced(), (7, 9));
    /// ```
    pub fn hexagon(&self, number: usize) -> Result<Share> {
        if number == 0 || number.is_multiple_of(2) {
            return value_error("the layer number must be odd.");
        }
        let n = number as i64;
        let size = 4 * n;
        let (mut cells, mut inked) = (0i64, 0i64);
        for index in 0..2 * n {
            let z = 2 * index;
            let target = 6 * n - 2 - z;
            let low = 0.max(target - size + 1);
            let high = (size - 1).min(target);
            if high < low {
                continue;
            }
            for rest in 0..8 {
                let count = (high - rest).div_euclid(8) - (low - 1 - rest).div_euclid(8);
                if count <= 0 {
                    continue;
                }
                cells += count;
                if self.inked(rest, (target - rest).rem_euclid(8), z) {
                    inked += count;
                }
            }
        }
        Ok(Share { inked, cells })
    }

    /// The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.
    ///
    /// At `W = 0` the band is the arm itself, the `2n` cells of the diameter, whose share is the
    /// closed form of [`arm_law`].
    ///
    /// ```
    /// use mrlymath::six::star::{arm_law, Star};
    /// assert_eq!(Star::new(23).unwrap().arm(9, 0).unwrap(), arm_law(9).unwrap());
    /// ```
    pub fn arm(&self, number: usize, half: usize) -> Result<Share> {
        if number == 0 || number.is_multiple_of(2) {
            return value_error("the layer number must be odd.");
        }
        let (n, half) = (number as i64, half as i64);
        let size = 4 * n;
        let (mut cells, mut inked) = (0i64, 0i64);
        for index in 0..2 * n {
            let z = 2 * index;
            let target = 6 * n - 2 - z;
            let base = target / 2;
            for x in base - half - 1..=base + half + 1 {
                let y = target - x;
                if !(0..size).contains(&x) || !(0..size).contains(&y) || (x - y).abs() > half {
                    continue;
                }
                cells += 1;
                inked += i64::from(self.inked(x, y, z));
            }
        }
        Ok(Share { inked, cells })
    }

    /// The per-layer excess of the star band over the hexagon across the first `L` odd layers.
    pub fn excesses(&self, layers: usize, half: usize) -> Result<Vec<f64>> {
        if layers == 0 {
            return value_error("the layer count must be at least one.");
        }
        (0..layers)
            .map(|step| {
                let number = 2 * step + 1;
                Ok(self.arm(number, half)?.value() - self.hexagon(number)?.value())
            })
            .collect()
    }
}

/// The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.
///
/// At `W = 0` and even `L` the reading is `L * excess_L = -(ln L)/4 + C + O(1/L^2)`, so the slope
/// settles on `-1/4` and the residual on the branch's exact `1/L^2` coefficient. The sliding window
/// carries a residue trap: the per-layer excess opens on a term in the ink law's character whose sum
/// vanishes at even `L` alone, so a window from `L/2` to `L` cancels it only when `L` is divisible by
/// four. At `L = 2 mod 4` the half is odd and the window converges somewhere else entirely, so the
/// slope is refused there rather than reported wrong.
pub fn decay(excesses: &[f64], layers: usize) -> Result<Decay> {
    if layers < 2 || layers > excesses.len() {
        return value_error("the layer count must be at least two and within the excesses read.");
    }
    let mean = |count: usize| excesses[..count].iter().sum::<f64>() / count as f64;
    let scaled = mean(layers) * layers as f64;
    let near = mean(layers / 2) * (layers / 2) as f64;
    let logged = scaled + (layers as f64).ln() / 4.0;
    let miss = logged - Branch::of(layers).constant();
    Ok(Decay {
        layers,
        excess: mean(layers),
        scaled,
        logged,
        miss,
        linear: miss * layers as f64,
        residual: miss * layers as f64 * layers as f64,
        slope: layers
            .is_multiple_of(4)
            .then(|| (scaled - near) / 2f64.ln()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arm_ink_is_the_closed_form() {
        let star = Star::new(23).unwrap();
        let read: Vec<(i64, i64)> = (0..8)
            .map(|step| star.arm(2 * step + 1, 0).unwrap().reduced())
            .collect();
        assert_eq!(
            read,
            vec![
                (1, 1),
                (1, 3),
                (2, 5),
                (4, 7),
                (5, 9),
                (5, 11),
                (6, 13),
                (8, 15)
            ]
        );
        for step in 0..8 {
            let number = 2 * step + 1;
            assert_eq!(star.arm(number, 0).unwrap(), arm_law(number).unwrap());
        }
        assert!((0..1001).all(|step| {
            let number = 2 * step + 1;
            star.arm(number, 0).unwrap() == arm_law(number).unwrap()
        }));
    }

    #[test]
    fn the_hexagon_ink_is_the_cut_ink_law() {
        let star = Star::new(23).unwrap();
        for step in 0..28 {
            let number = 2 * step + 1;
            let n = number as i64;
            let chi = if (3 * n - 1) / 2 % 2 == 0 { 1 } else { -1 };
            let want =
                0.5 + chi as f64 / 8.0 + 1.0 / (2 * n) as f64 - chi as f64 / (8 * n * n) as f64;
            assert!((star.hexagon(number).unwrap().value() - want).abs() < 1e-12);
        }
    }

    #[test]
    fn the_cell_frame_decay_is_minus_a_quarter() {
        let star = Star::new(23).unwrap();
        let excesses = star.excesses(400, 0).unwrap();
        let read = decay(&excesses, 28).unwrap();
        assert_eq!(format!("{:.6}", read.scaled), "-1.126964");
        assert_eq!(format!("{:.10}", read.logged), "-0.2939128437");
        assert_eq!(format!("{:.8}", read.residual), "-0.11937029");
        let deep = decay(&excesses, 400).unwrap();
        assert_eq!(format!("{:.6}", deep.scaled), "-1.791627");
        assert_eq!(format!("{:.10}", deep.logged), "-0.2937613344");
        assert_eq!(format!("{:.8}", deep.residual), "-0.11978958");
        assert!((deep.slope.expect("a window at L = 0 mod 4") + 0.25).abs() < 1e-4);
        assert_eq!(decay(&excesses, 102).unwrap().slope, None);
        assert_eq!(format!("{:.10}", Branch::Zero.constant()), "-0.2937605857");
        assert_eq!(Branch::of(400).residual(), Some(-23.0 / 192.0));
        assert_eq!(Branch::of(402).residual(), Some(25.0 / 192.0));
        assert_eq!(Branch::of(401).name(), "odd");
    }

    #[test]
    fn a_wider_band_walks_off_the_quarter() {
        let star = Star::new(23).unwrap();
        assert_eq!(width_law(1), width_law(0));
        assert_eq!(width_law(3), width_law(2));
        assert_eq!(format!("{:.6}", width_law(3)), "-0.166667");
        for half in [1usize, 2, 3, 4, 6] {
            let excesses = star.excesses(400, half).unwrap();
            let slope = decay(&excesses, 400)
                .unwrap()
                .slope
                .expect("a window at L = 0 mod 4");
            assert!((slope - width_law(half)).abs() < 2e-4);
        }
    }

    #[test]
    fn even_layers_are_refused_and_the_code_is_checked() {
        let star = Star::new(23).unwrap();
        assert!(star.hexagon(4).is_err());
        assert!(star.arm(0, 0).is_err());
        assert!(Star::new(1 << 9).is_err());
    }
}
