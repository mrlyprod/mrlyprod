use crate::bang::Code;
use crate::bang::factory;
use mrly_core::error::{Result, value_error};
use mrly_core::tensor::Tensor;
use std::collections::HashMap;

/// The face counts of the union of closed unit cubes a design's level power fills: `counts[k]` is its distinct unit `k`-faces, the cells last, and `euler` their alternating sum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Skeleton {
    /// The distinct unit `k`-faces for `k = 0..=dimension`.
    pub counts: Vec<u128>,
    /// The Euler characteristic, `sum (-1)^k counts[k]`.
    pub euler: i128,
}

impl Skeleton {
    fn from_counts(counts: Vec<u128>) -> Option<Skeleton> {
        let euler = counts
            .iter()
            .enumerate()
            .try_fold(0i128, |sum, (k, &count)| {
                let signed = i128::try_from(count).ok()?;
                if k % 2 == 0 {
                    sum.checked_add(signed)
                } else {
                    sum.checked_sub(signed)
                }
            })?;
        Some(Skeleton { counts, euler })
    }
    /// The distinct corners.
    pub fn vertices(&self) -> u128 {
        self.counts[0]
    }
    /// The distinct unit edges.
    pub fn edges(&self) -> u128 {
        self.counts.get(1).copied().unwrap_or(0)
    }
    /// The distinct unit squares, the cells themselves in the plane.
    pub fn faces(&self) -> u128 {
        self.counts.get(2).copied().unwrap_or(0)
    }
    /// The filled cells.
    pub fn cells(&self) -> u128 {
        self.counts[self.counts.len() - 1]
    }
}

fn normalize(mask: u32, dimension: usize) -> u32 {
    let mut shift = 0u32;
    for axis in 0..dimension {
        let bit = 1u32 << axis;
        let low = (0..1u32 << dimension).any(|cell| cell & bit == 0 && mask >> cell & 1 == 1);
        if !low {
            shift |= bit;
        }
    }
    if shift == 0 {
        return mask;
    }
    let mut moved = 0u32;
    for cell in 0..1u32 << dimension {
        if mask >> cell & 1 == 1 {
            moved |= 1 << (cell & !shift);
        }
    }
    moved
}

fn support(mask: u32, dimension: usize) -> u32 {
    (0..1u32 << dimension)
        .filter(|&cell| mask >> cell & 1 == 1)
        .fold(0, |acc, cell| acc | cell)
}

/// The placement counts the face recurrence runs on: for each pattern `P` of cells inside a `2^d` window and each lowest digit `w` of the tile, whether `w + P` fits the tile with wrap-around, and which pattern `P ^ W` the carried digits must then form one level up.
///
/// Every face count of the level power is a signed sum of the placement counts `N_P(L)` of the
/// patterns `P`, and `N_P(L + 1) = sum over Q of M[P][Q] N_Q(L)` with `M[P][Q]` the number of
/// lowest digits `w` that fit `P` and carry it to `Q`, so every count is a linear recurrence
/// whose roots are the diagonal entries `M[P][P]`, the spanning counts of the patterns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placements {
    dimension: usize,
    states: Vec<u32>,
    transitions: Vec<Vec<(usize, u128)>>,
}

impl Placements {
    /// Reads the placement counts off a rendered tile of two to four dimensions.
    ///
    /// # Errors
    ///
    /// Errors when the tile has more than four dimensions, where the window patterns pass a `u32`.
    pub fn of_tile(tile: &Tensor) -> Result<Placements> {
        let dimension = tile.shape.len();
        if dimension == 0 || dimension > 4 {
            return value_error("the face recurrence runs in one to four dimensions.");
        }
        let cells = 1usize << dimension;
        let states: Vec<u32> = (1..1u32 << cells)
            .filter(|&mask| normalize(mask, dimension) == mask)
            .collect();
        let index: HashMap<u32, usize> = states
            .iter()
            .enumerate()
            .map(|(at, &mask)| (mask, at))
            .collect();
        let shape = &tile.shape;
        let total = tile.size();
        let transitions = states
            .iter()
            .map(|&mask| {
                let mut counts: HashMap<usize, u128> = HashMap::new();
                for flat in 0..total {
                    let mut digit = vec![0usize; dimension];
                    let mut rest = flat;
                    for axis in (0..dimension).rev() {
                        digit[axis] = rest % shape[axis];
                        rest /= shape[axis];
                    }
                    let mut wrapped = 0u32;
                    for axis in 0..dimension {
                        if digit[axis] + 1 == shape[axis] {
                            wrapped |= 1 << axis;
                        }
                    }
                    let fits =
                        (0..cells as u32)
                            .filter(|&cell| mask >> cell & 1 == 1)
                            .all(|cell| {
                                let moved: Vec<usize> = (0..dimension)
                                    .map(|axis| {
                                        (digit[axis] + (cell as usize >> axis & 1)) % shape[axis]
                                    })
                                    .collect();
                                tile.at(tile.index(&moved)) != 0
                            });
                    if !fits {
                        continue;
                    }
                    let mut carried = 0u32;
                    for cell in 0..cells as u32 {
                        if mask >> cell & 1 == 1 {
                            carried |= 1 << (cell & wrapped);
                        }
                    }
                    *counts.entry(index[&carried]).or_insert(0) += 1;
                }
                let mut sorted: Vec<(usize, u128)> = counts.into_iter().collect();
                sorted.sort_unstable();
                sorted
            })
            .collect();
        Ok(Placements {
            dimension,
            states,
            transitions,
        })
    }
    fn state(&self, mask: u32) -> usize {
        let wanted = normalize(mask, self.dimension);
        self.states.binary_search(&wanted).unwrap_or(usize::MAX)
    }
    fn vector(&self, level: u32) -> Option<Vec<u128>> {
        let mut values = vec![0u128; self.states.len()];
        values[self.state(1)] = 1;
        for _ in 0..level {
            let mut next = vec![0u128; self.states.len()];
            for (at, edges) in self.transitions.iter().enumerate() {
                for &(to, count) in edges {
                    next[at] = next[at].checked_add(count.checked_mul(values[to])?)?;
                }
            }
            values = next;
        }
        Some(values)
    }
    fn functional(&self, k: usize) -> Vec<i128> {
        let dimension = self.dimension;
        let mut weights = vec![0i128; self.states.len()];
        for points in 0..1u32 << dimension {
            if points.count_ones() as usize != dimension - k {
                continue;
            }
            let window: Vec<u32> = (0..1u32 << dimension)
                .filter(|&cell| cell & !points == 0)
                .collect();
            for subset in 1..1u32 << window.len() {
                let mask = window
                    .iter()
                    .enumerate()
                    .filter(|&(bit, _)| subset >> bit & 1 == 1)
                    .fold(0u32, |acc, (_, &cell)| acc | 1 << cell);
                let sign = if subset.count_ones() % 2 == 1 { 1 } else { -1 };
                weights[self.state(mask)] += sign;
            }
        }
        weights
    }
    /// Returns the face counts of the level-fold Kronecker power, or none past a `u128`.
    pub fn at(&self, level: u32) -> Option<Skeleton> {
        let values = self.vector(level)?;
        let counts = (0..=self.dimension)
            .map(|k| {
                let total = self.functional(k).iter().zip(&values).try_fold(
                    0i128,
                    |sum, (&weight, &value)| {
                        sum.checked_add(weight.checked_mul(i128::try_from(value).ok()?)?)
                    },
                )?;
                u128::try_from(total).ok()
            })
            .collect::<Option<Vec<u128>>>()?;
        Skeleton::from_counts(counts)
    }
    /// Returns the roots of the recurrence the `k`-face count obeys, every face when `k` is none, each with the multiplicity the recurrence needs: the spanning counts of the patterns the count reaches, a root repeated once per pattern of that count on one carry chain.
    pub fn roots(&self, k: Option<usize>) -> Vec<(u128, u32)> {
        let mut live = vec![false; self.states.len()];
        for face in 0..=self.dimension {
            if k.is_some_and(|wanted| wanted != face) {
                continue;
            }
            for (at, &weight) in self.functional(face).iter().enumerate() {
                if weight != 0 {
                    live[at] = true;
                }
            }
        }
        let mut order: Vec<usize> = (0..self.states.len()).collect();
        order.sort_by_key(|&at| {
            std::cmp::Reverse(support(self.states[at], self.dimension).count_ones())
        });
        for &at in &order {
            if live[at] {
                for &(to, _) in &self.transitions[at] {
                    live[to] = true;
                }
            }
        }
        let diagonal = |at: usize| -> u128 {
            self.transitions[at]
                .iter()
                .find(|&&(to, _)| to == at)
                .map_or(0, |&(_, count)| count)
        };
        let mut distinct: Vec<u128> = order
            .iter()
            .filter(|&&at| live[at])
            .map(|&at| diagonal(at))
            .collect();
        distinct.sort_unstable();
        distinct.dedup();
        distinct
            .into_iter()
            .map(|root| {
                let mut height = vec![0u32; self.states.len()];
                for &at in order.iter().rev() {
                    if !live[at] {
                        continue;
                    }
                    let below = self.transitions[at]
                        .iter()
                        .filter(|&&(to, _)| to != at)
                        .map(|&(to, _)| height[to])
                        .max()
                        .unwrap_or(0);
                    height[at] = below + u32::from(diagonal(at) == root);
                }
                (root, height.into_iter().max().unwrap_or(0))
            })
            .filter(|&(_, multiplicity)| multiplicity > 0)
            .collect()
    }
    /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the `k`-face count obeys from `L = 0`, every face when `k` is none, or none past an `i128`.
    pub fn recurrence(&self, k: Option<usize>) -> Option<Vec<i128>> {
        let mut poly: Vec<i128> = vec![1];
        for (root, multiplicity) in self.roots(k) {
            for _ in 0..multiplicity {
                let mut next = vec![0i128; poly.len() + 1];
                for (power, &coefficient) in poly.iter().enumerate() {
                    next[power] = next[power].checked_add(coefficient)?;
                    next[power + 1] = next[power + 1]
                        .checked_sub(i128::try_from(root).ok()?.checked_mul(coefficient)?)?;
                }
                poly = next;
            }
        }
        Some(poly[1..].iter().map(|&coefficient| -coefficient).collect())
    }
}

/// Returns the face counts of the tile's level-fold Kronecker power, or none past a `u128`.
///
/// ```
/// use mrly_math::bang::Code;
/// let sponge = mrly_math::bang::factory::create(Code::from(23u64), 3, 3, 2, 1).unwrap();
/// let level_two = mrly_math::counts::skeleton_of_tile(&sponge, 2).unwrap().unwrap();
/// assert_eq!(level_two.counts, [896, 2304, 1728, 400]);
/// assert_eq!(level_two.euler, -80);
/// ```
///
/// # Errors
///
/// Errors when the tile has more than four dimensions.
pub fn skeleton_of_tile(tile: &Tensor, level: u32) -> Result<Option<Skeleton>> {
    Ok(Placements::of_tile(tile)?.at(level))
}

/// Returns the face counts of the code's design at the given level, from the tile's placement counts alone.
///
/// ```
/// use mrly_math::bang::Code;
/// let carpet = mrly_math::counts::skeleton(Code::from(7u64), 3, 2, 2, 3).unwrap();
/// assert_eq!((carpet.vertices(), carpet.edges(), carpet.cells(), carpet.euler), (688, 1272, 512, -72));
/// ```
///
/// # Errors
///
/// Errors when the code is out of range, the dimension passes four, or a count passes a `u128`.
pub fn skeleton(
    code: Code,
    number: usize,
    dimension: usize,
    base: usize,
    level: u32,
) -> Result<Skeleton> {
    let tile = factory::create(code, number, dimension, base, 1)?;
    match Placements::of_tile(&tile)?.at(level) {
        Some(skeleton) => Ok(skeleton),
        None => value_error("a face count passes a hundred and twenty-eight bits."),
    }
}

/// Returns the coefficients of the recurrence the code's `k`-face count obeys along the level, every face and the Euler characteristic when `k` is none.
///
/// ```
/// use mrly_math::bang::Code;
/// assert_eq!(mrly_math::counts::skeleton_recurrence(Code::from(7u64), 3, 2, 2, Some(0)).unwrap(), [12, -35, 24]);
/// ```
///
/// # Errors
///
/// Errors when the code is out of range, the dimension passes four, or a coefficient passes an `i128`.
pub fn skeleton_recurrence(
    code: Code,
    number: usize,
    dimension: usize,
    base: usize,
    k: Option<usize>,
) -> Result<Vec<i128>> {
    let tile = factory::create(code, number, dimension, base, 1)?;
    match Placements::of_tile(&tile)?.recurrence(k) {
        Some(coefficients) => Ok(coefficients),
        None => value_error("a recurrence coefficient passes a hundred and twenty-eight bits."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counts::exposure_of_tile;
    use crate::three;
    use crate::two;
    use std::collections::HashSet;

    fn brute(grid: &Tensor) -> Skeleton {
        let dimension = grid.shape.len();
        let mut seen: Vec<HashSet<(Vec<usize>, u32)>> = vec![HashSet::new(); dimension + 1];
        for flat in 0..grid.size() {
            if grid.at(flat) == 0 {
                continue;
            }
            let mut coords = vec![0usize; dimension];
            let mut rest = flat;
            for axis in (0..dimension).rev() {
                coords[axis] = rest % grid.shape[axis];
                rest /= grid.shape[axis];
            }
            for spans in 0..1u32 << dimension {
                for ends in 0..1u32 << dimension {
                    if ends & spans != 0 {
                        continue;
                    }
                    let corner: Vec<usize> = (0..dimension)
                        .map(|axis| coords[axis] + (ends >> axis & 1) as usize)
                        .collect();
                    seen[spans.count_ones() as usize].insert((corner, spans));
                }
            }
        }
        Skeleton::from_counts(seen.iter().map(|faces| faces.len() as u128).collect()).unwrap()
    }

    #[test]
    fn the_recurrence_matches_the_rendered_skeleton() {
        for (code, dimension, base, top) in [
            (7u128, 2usize, 2usize, 4u32),
            (6, 2, 2, 4),
            (13, 2, 2, 4),
            (100, 2, 3, 4),
            (23, 3, 2, 4),
            (254, 3, 2, 4),
            (105, 3, 2, 4),
            (123456789, 3, 3, 3),
            (23, 3, 3, 3),
        ] {
            let code = Code(code);
            let tile = factory::create(code, 3, dimension, base, 1).unwrap();
            let placements = Placements::of_tile(&tile).unwrap();
            for level in 1..=top {
                let grid = factory::create(code, 3, dimension, base, level as usize).unwrap();
                assert_eq!(
                    placements.at(level).unwrap(),
                    brute(&grid),
                    "code={code} d={dimension} q={base} l={level}"
                );
            }
        }
    }
    #[test]
    fn the_rendered_skeleton_is_the_census() {
        let carpet = two::census::census(&two::create(Code(7), 3, 2, 0, 2).unwrap()).unwrap();
        let level_two = skeleton(Code(7), 3, 2, 2, 2).unwrap();
        assert_eq!(
            (
                level_two.vertices(),
                level_two.edges(),
                level_two.cells(),
                level_two.euler
            ),
            (
                carpet.vertices as u128,
                carpet.edges as u128,
                64,
                carpet.euler as i128
            )
        );
        let euler: Vec<i128> = (1..4)
            .map(|l| skeleton(Code(7), 3, 2, 2, l).unwrap().euler)
            .collect();
        assert_eq!(euler, [0, -8, -72]);
        for level in 1..3 {
            let sponge = three::census(&three::create(Code(23), 3, level, 2).unwrap()).unwrap();
            let closed = skeleton(Code(23), 3, 3, 2, level as u32).unwrap();
            assert_eq!(
                closed.counts,
                [
                    sponge.vertices as u128,
                    sponge.edges as u128,
                    sponge.faces as u128,
                    sponge.fills as u128
                ]
            );
            assert_eq!(closed.euler, sponge.euler as i128);
        }
    }
    #[test]
    fn the_terms_obey_the_recurrence() {
        for (code, dimension, base) in [
            (7u128, 2usize, 2usize),
            (6, 2, 2),
            (100, 2, 3),
            (23, 3, 2),
            (254, 3, 2),
            (123456789, 3, 3),
        ] {
            let tile = factory::create(Code(code), 3, dimension, base, 1).unwrap();
            let placements = Placements::of_tile(&tile).unwrap();
            let skeletons: Vec<Skeleton> = (0..12).map(|l| placements.at(l).unwrap()).collect();
            for (level, skeleton) in skeletons.iter().enumerate().skip(1) {
                let exposed = exposure_of_tile(&tile, level as u32).unwrap();
                assert_eq!(
                    2 * skeleton.counts[dimension - 1],
                    2 * dimension as u128 * skeleton.cells() + exposed,
                    "code={code} l={level}"
                );
            }
            for k in 0..=dimension {
                let rule = placements.recurrence(Some(k)).unwrap();
                let terms: Vec<i128> = skeletons.iter().map(|s| s.counts[k] as i128).collect();
                for at in rule.len()..terms.len() {
                    let predicted: i128 = rule
                        .iter()
                        .enumerate()
                        .map(|(back, &c)| c * terms[at - back - 1])
                        .sum();
                    assert_eq!(
                        predicted, terms[at],
                        "code={code} k={k} at={at} rule={rule:?}"
                    );
                }
            }
            let rule = placements.recurrence(None).unwrap();
            let terms: Vec<i128> = skeletons.iter().map(|s| s.euler).collect();
            for at in rule.len()..terms.len() {
                let predicted: i128 = rule
                    .iter()
                    .enumerate()
                    .map(|(back, &c)| c * terms[at - back - 1])
                    .sum();
                assert_eq!(
                    predicted, terms[at],
                    "code={code} euler at={at} rule={rule:?}"
                );
            }
        }
    }
    #[test]
    fn the_classics_close() {
        let carpet = factory::create(Code(7), 3, 2, 2, 1).unwrap();
        let carpet = Placements::of_tile(&carpet).unwrap();
        assert_eq!(carpet.roots(Some(0)), [(1, 1), (3, 1), (8, 1)]);
        for level in 0..10u32 {
            let (three, eight) = (3u128.pow(level), 8u128.pow(level));
            let skeleton = carpet.at(level).unwrap();
            assert_eq!(35 * skeleton.vertices(), 40 + 56 * three + 44 * eight);
            assert_eq!(5 * skeleton.edges(), 8 * three + 12 * eight);
            assert_eq!(skeleton.cells(), eight);
            assert_eq!(7 * skeleton.euler, 8 - eight as i128);
        }
        let sponge = factory::create(Code(23), 3, 3, 2, 1).unwrap();
        let sponge = Placements::of_tile(&sponge).unwrap();
        assert_eq!(sponge.roots(None), [(1, 1), (3, 1), (8, 1), (20, 1)]);
        assert_eq!(sponge.recurrence(Some(0)).unwrap(), [32, -275, 724, -480]);
        for level in 0..10u32 {
            let (three, eight, twenty) = (3u128.pow(level), 8u128.pow(level), 20u128.pow(level));
            let skeleton = sponge.at(level).unwrap();
            assert_eq!(
                11305 * skeleton.vertices(),
                16320 + 25536 * three + 28424 * eight + 20160 * twenty
            );
            assert_eq!(
                85 * skeleton.edges(),
                192 * three + 408 * eight + 420 * twenty
            );
            assert_eq!(skeleton.faces(), 2 * eight + 4 * twenty);
            assert_eq!(skeleton.cells(), twenty);
            assert_eq!(
                133 * skeleton.euler,
                192 - 38 * eight as i128 - 21 * twenty as i128
            );
        }
        let vertices: Vec<u128> = (1..5).map(|l| sponge.at(l).unwrap().vertices()).collect();
        assert_eq!(vertices, [64, 896, 15616, 295808]);
        let euler: Vec<i128> = (1..5).map(|l| sponge.at(l).unwrap().euler).collect();
        assert_eq!(euler, [-4, -80, -1408, -26432]);
    }
    #[test]
    fn a_repeated_root_carries_a_linear_term() {
        let tile = factory::create(Code(123456789), 3, 3, 3, 1).unwrap();
        let placements = Placements::of_tile(&tile).unwrap();
        assert_eq!(
            placements.roots(Some(0)),
            [(0, 2), (1, 2), (2, 1), (3, 1), (5, 1), (16, 1)]
        );
        assert_eq!(
            placements.roots(Some(1)),
            [(0, 1), (1, 1), (2, 1), (3, 1), (5, 1), (16, 1)]
        );
        let vertices: Vec<u128> = (0..6)
            .map(|l| placements.at(l).unwrap().vertices())
            .collect();
        assert_eq!(vertices, [8, 57, 699, 10463, 164778, 2626182]);
        for level in 2..12u32 {
            let [one, two, three, five, sixteen] = [1i128, 2, 3, 5, 16].map(|root| root.pow(level));
            let powers = 57657600 * 193 / 225 * one + 11 * 1921920 * i128::from(level)
                - 57657600 * 19 / 28 * two
                + 57657600 * 667 / 156 * three
                + 57657600 * 941 / 1100 * five
                + 144201667 * sixteen;
            let vertices = placements.at(level).unwrap().vertices() as i128;
            assert_eq!(57657600 * vertices, powers, "l={level}");
        }
    }
    #[test]
    fn a_single_cell_is_a_cube() {
        let one = Placements::of_tile(&factory::create(Code(255), 2, 3, 2, 1).unwrap()).unwrap();
        assert_eq!(one.at(0).unwrap().counts, [8, 12, 6, 1]);
        assert_eq!(one.at(0).unwrap().euler, 1);
        assert_eq!(
            one.at(3).unwrap(),
            brute(&factory::create(Code(255), 2, 3, 2, 3).unwrap())
        );
        assert!(Placements::of_tile(&Tensor::new(vec![2; 5])).is_err());
    }
}
