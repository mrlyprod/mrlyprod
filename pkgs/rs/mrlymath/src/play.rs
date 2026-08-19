use crate::dim::models::dtype_for;
use crate::prime::is_prime;
use crate::two::Cell2d;
use mrlycore::tensor::Tensor;

const TURNS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

/// Draws the square number spiral of an odd side, one at the center and the walk stepping east, north, west then south with row zero on top, so it turns counterclockwise.
pub fn spiral(side: usize) -> Tensor {
    assert!(
        !side.is_multiple_of(2),
        "spiral side must be odd so the center is central"
    );
    let peak = side * side;
    let mut out = Tensor::typed(vec![side, side], dtype_for(peak as i64));
    let mut row = (side / 2) as isize;
    let mut col = (side / 2) as isize;
    let mut value = 1;
    out.put(row as usize * side + col as usize, 1);
    let mut run = 1;
    let mut leg = 0;
    while value < peak {
        let (dr, dc) = TURNS[leg % 4];
        for _ in 0..run {
            if value == peak {
                break;
            }
            row += dr;
            col += dc;
            value += 1;
            out.put(row as usize * side + col as usize, value as i64);
        }
        leg += 1;
        if leg.is_multiple_of(2) {
            run += 1;
        }
    }
    out
}

/// Masks the number spiral to its primes, one on every prime cell and zero everywhere else.
pub fn ulam(side: usize) -> Cell2d {
    let numbers = spiral(side);
    let mut mask = Tensor::new(vec![side, side]);
    for flat in 0..numbers.size() {
        if is_prime(numbers.at(flat) as usize) {
            mask.put(flat, 1);
        }
    }
    Cell2d::new(mask)
}

/// Builds the first rows of Pascal's triangle reduced by the base, each row left aligned in a square grid and padded with zeros, drawing the Sierpinski gasket at base two.
pub fn pascal(rows: usize, base: usize) -> Tensor {
    assert!(base > 1, "pascal base must be at least two");
    let mut out = Tensor::typed(vec![rows, rows], dtype_for(base as i64 - 1));
    let mut line = vec![0usize; rows];
    for r in 0..rows {
        for k in (1..=r).rev() {
            line[k] = (line[k] + line[k - 1]) % base;
        }
        line[0] = 1;
        for (k, &value) in line.iter().enumerate().take(r + 1) {
            out.put(r * rows + k, value as i64);
        }
    }
    out
}

/// Walks the Collatz trajectory from the number down to one inclusive, empty for zero.
///
/// ```
/// assert_eq!(mrlymath::play::collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub fn collatz(number: usize) -> Vec<usize> {
    if number == 0 {
        return Vec::new();
    }
    let mut out = vec![number];
    let mut step = number;
    while step != 1 {
        step = if step.is_multiple_of(2) {
            step / 2
        } else {
            3 * step + 1
        };
        out.push(step);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas::primes;
    use mrlycore::tensor::Dtype;

    fn rows_of(grid: &Tensor) -> Vec<Vec<i64>> {
        let width = grid.shape[1];
        (0..grid.shape[0])
            .map(|r| (0..width).map(|c| grid.at(r * width + c)).collect())
            .collect()
    }

    fn binomials(rows: usize) -> Vec<Vec<u128>> {
        let mut out = vec![vec![0u128; rows]; rows];
        for r in 0..rows {
            out[r][0] = 1;
            for k in 1..=r {
                out[r][k] = out[r - 1][k - 1] + out[r - 1][k];
            }
        }
        out
    }

    #[test]
    fn the_spiral_pins_its_hand_walked_grids() {
        assert_eq!(
            rows_of(&spiral(3)),
            vec![vec![5, 4, 3], vec![6, 1, 2], vec![7, 8, 9]]
        );
        assert_eq!(
            rows_of(&spiral(5)),
            vec![
                vec![17, 16, 15, 14, 13],
                vec![18, 5, 4, 3, 12],
                vec![19, 6, 1, 2, 11],
                vec![20, 7, 8, 9, 10],
                vec![21, 22, 23, 24, 25],
            ]
        );
    }

    #[test]
    fn the_spiral_holds_every_value_once_around_a_central_one() {
        for side in [1usize, 3, 7, 31] {
            let grid = spiral(side);
            assert_eq!(grid.shape, vec![side, side], "side {side}");
            let mut seen: Vec<i64> = (0..grid.size()).map(|flat| grid.at(flat)).collect();
            seen.sort_unstable();
            let want: Vec<i64> = (1..=(side * side) as i64).collect();
            assert_eq!(seen, want, "side {side}");
            assert_eq!(grid.at(grid.index(&[side / 2, side / 2])), 1, "side {side}");
        }
        assert_eq!(spiral(15).dtype(), Dtype::U8);
        assert_eq!(spiral(17).dtype(), Dtype::U16);
    }

    #[test]
    fn the_ulam_mask_lights_exactly_the_primes_under_its_peak() {
        let mask = ulam(31);
        assert_eq!(mask.types().sum() as usize, primes(31 * 31).len());
        assert_eq!(ulam(5).types().get(&[2, 2]), 0);
    }

    #[test]
    fn pascal_in_base_two_is_the_parity_of_the_binomials() {
        for rows in 1..=8usize {
            let reduced = pascal(rows, 2);
            for (r, line) in binomials(rows).iter().enumerate() {
                for (k, &value) in line.iter().enumerate() {
                    assert_eq!(
                        reduced.at(r * rows + k) as u128,
                        value % 2,
                        "rows {rows} row {r} column {k}"
                    );
                }
            }
        }
    }

    #[test]
    fn pascal_row_sums_pin_the_base_three_triangle() {
        let grid = pascal(9, 3);
        let sums: Vec<i64> = (0..9)
            .map(|r| (0..9).map(|k| grid.at(r * 9 + k)).sum())
            .collect();
        assert_eq!(sums, vec![1, 2, 4, 2, 4, 8, 4, 8, 13]);
    }

    #[test]
    fn pascal_pins_the_first_rows_in_base_ten() {
        assert_eq!(
            rows_of(&pascal(4, 10)),
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 0, 0],
                vec![1, 2, 1, 0],
                vec![1, 3, 3, 1],
            ]
        );
    }

    #[test]
    fn collatz_pins_its_hand_walked_trajectories() {
        assert!(collatz(0).is_empty());
        assert_eq!(collatz(1), vec![1]);
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        let long = collatz(27);
        assert_eq!(long.len(), 112);
        assert_eq!(long.iter().copied().max(), Some(9232));
    }

    #[test]
    fn every_collatz_step_follows_the_rule_and_lands_on_one() {
        for number in 2..=1_000usize {
            let walk = collatz(number);
            assert_eq!(walk[0], number, "{number}");
            assert_eq!(walk.last().copied(), Some(1), "{number}");
            for pair in walk.windows(2) {
                let want = if pair[0].is_multiple_of(2) {
                    pair[0] / 2
                } else {
                    3 * pair[0] + 1
                };
                assert_eq!(pair[1], want, "{number}");
            }
        }
    }
}
