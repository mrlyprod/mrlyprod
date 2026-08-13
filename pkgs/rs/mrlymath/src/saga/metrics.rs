use mrlycore::tensor::Tensor;

/// Estimates a grid's disorder as run-coded bits over raw bits:
/// each run of equal cells costs the entropy of run values plus a gamma
/// length code, so flat grids land near zero and noise climbs toward one half.
pub fn chaos(grid: &Tensor) -> f64 {
    let n = grid.size();
    if n == 0 {
        return 0.0;
    }
    let mut runs: Vec<(i64, usize)> = Vec::new();
    for i in 0..n {
        let value = grid.at(i);
        match runs.last_mut() {
            Some((last, len)) if *last == value => *len += 1,
            _ => runs.push((value, 1)),
        }
    }
    let mut tallies: Vec<(i64, usize)> = Vec::new();
    for (value, _) in &runs {
        match tallies.iter_mut().find(|(v, _)| v == value) {
            Some((_, count)) => *count += 1,
            None => tallies.push((*value, 1)),
        }
    }
    let total = runs.len() as f64;
    let entropy: f64 = tallies
        .iter()
        .map(|(_, count)| {
            let p = *count as f64 / total;
            -p * p.log2()
        })
        .sum();
    let bits: f64 = runs
        .iter()
        .map(|(_, len)| entropy + 2.0 * (*len as f64).log2().floor() + 1.0)
        .sum();
    bits / (8.0 * n as f64)
}

/// Measures the cell-diff rate between two grids: the differing fraction,
/// or one whole when the shapes disagree.
pub fn spread(a: &Tensor, b: &Tensor) -> f64 {
    if a.shape != b.shape || a.size() == 0 {
        return 1.0;
    }
    let differing = (0..a.size()).filter(|&i| a.at(i) != b.at(i)).count();
    differing as f64 / a.size() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::rng::Rng;

    #[test]
    fn chaos_ranks_flat_below_stripes_below_noise() {
        let flat = Tensor::new(vec![10, 10]);
        let mut stripes = Tensor::new(vec![10, 10]);
        for r in (0..10).step_by(2) {
            for c in 0..10 {
                stripes.set(&[r, c], 1);
            }
        }
        let mut rng = Rng::new(3);
        let mut noise = Tensor::new(vec![10, 10]);
        for i in 0..100 {
            noise.put(i, rng.range(0, 9));
        }
        assert!(chaos(&flat) < chaos(&stripes));
        assert!(chaos(&stripes) < chaos(&noise));
        assert!(chaos(&flat) < 0.05);
        assert!(chaos(&noise) > 0.25);
    }
    #[test]
    fn spread_counts_the_differing_fraction() {
        let a = Tensor::new(vec![2, 2]);
        let mut b = a.clone();
        assert_eq!(spread(&a, &b), 0.0);
        b.set(&[0, 0], 5);
        assert_eq!(spread(&a, &b), 0.25);
        assert_eq!(spread(&a, &Tensor::new(vec![3, 3])), 1.0);
    }
}
