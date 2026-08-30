use mrlylab::moire::{stack, Combine, Field, Lattice, Spec};
use mrlymath::bang::corners_to_code;
use mrlynum::spin::{profile, reach};

fn pearson(a: &[f64], b: &[f64], w: &[f64]) -> f64 {
    let total: f64 = w.iter().sum();
    let mean = |x: &[f64]| x.iter().zip(w).map(|(v, w)| v * w).sum::<f64>() / total;
    let (ma, mb) = (mean(a), mean(b));
    let cov: f64 = a
        .iter()
        .zip(b)
        .zip(w)
        .map(|((x, y), w)| w * (x - ma) * (y - mb))
        .sum();
    let var = |x: &[f64], m: f64| {
        x.iter()
            .zip(w)
            .map(|(v, w)| w * (v - m) * (v - m))
            .sum::<f64>()
    };
    cov / (var(a, ma) * var(b, mb)).sqrt()
}

#[test]
fn the_coprime_law_dies_under_the_spin() {
    let (size, steps) = (135, 400);
    let spec = Spec::new(corners_to_code(&[vec![0, 0]], 2, 2), 2, 2);
    let layer = |n: usize| stack(spec, &[n], Combine::Sum, 1, Lattice::Square, size, &[]).unwrap();
    let far = reach(size);
    let disc: Vec<f64> = (0..steps)
        .map(|k| {
            let r = k as f64 * far / (steps - 1) as f64;
            if r <= size as f64 / 2.0 {
                r
            } else {
                0.0
            }
        })
        .collect();
    let ones = vec![1.0; size * size];
    let spin = |f: &Field| {
        profile(&f.data, size, steps)
            .iter()
            .map(|&v| v as f64)
            .collect::<Vec<f64>>()
    };
    let mut flat_peak: f64 = 0.0;
    let mut spun_peak: f64 = 0.0;
    for (m, n) in [(3, 5), (5, 7), (3, 7), (9, 13)] {
        let (a, b) = (layer(m), layer(n));
        flat_peak = flat_peak.max(pearson(&a.as_f64(), &b.as_f64(), &ones).abs());
        spun_peak = spun_peak.max(pearson(&spin(&a), &spin(&b), &disc).abs());
    }
    assert!(flat_peak < 0.03, "flat {flat_peak}");
    assert!(spun_peak > 0.2, "spun {spun_peak}");
}
