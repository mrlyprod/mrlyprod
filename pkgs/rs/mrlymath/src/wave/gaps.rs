/// The transmission threshold below which a band gap opens.
pub const GAP_THRESH: f64 = 0.12;

/// A band gap: one contiguous run of transmission below the threshold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gap {
    /// The run's lowest frequency.
    pub lo: f64,
    /// The run's highest frequency.
    pub hi: f64,
    /// The frequency of the run's first deepest point.
    pub centre: f64,
    /// The transmission at the deepest point.
    pub depth: f64,
    /// Whether the run touches an end of the read band.
    pub band_edge: bool,
}

/// Extracts the contiguous sub-threshold runs of a transmission curve.
pub fn gaps(freqs: &[f64], values: &[f64], threshold: f64) -> Vec<Gap> {
    let n = values.len().min(freqs.len());
    let mut out = Vec::new();
    let mut i = 0;
    while i < n {
        if values[i] < threshold {
            let mut j = i;
            while j < n && values[j] < threshold {
                j += 1;
            }
            let mut deepest = i;
            for k in i..j {
                if values[k] < values[deepest] {
                    deepest = k;
                }
            }
            out.push(Gap {
                lo: freqs[i],
                hi: freqs[j - 1],
                centre: freqs[deepest],
                depth: values[deepest],
                band_edge: i == 0 || j == n,
            });
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_split_centre_and_flag_edges() {
        let freqs: Vec<f64> = (1..=10).map(|k| k as f64 / 100.0).collect();
        let values = [1.0, 0.05, 0.03, 0.08, 1.0, 1.0, 0.11, 1.0, 1.0, 0.02];
        let found = gaps(&freqs, &values, GAP_THRESH);
        assert_eq!(found.len(), 3);
        assert_eq!((found[0].lo, found[0].hi), (0.02, 0.04));
        assert_eq!((found[0].centre, found[0].depth), (0.03, 0.03));
        assert!(!found[0].band_edge);
        assert_eq!(
            (found[1].lo, found[1].hi, found[1].centre),
            (0.07, 0.07, 0.07)
        );
        assert!(found[2].band_edge);
        assert_eq!(found[2].depth, 0.02);
    }

    #[test]
    fn the_threshold_is_strict_and_nan_stays_out() {
        let freqs = [0.01, 0.02, 0.03];
        let clear = [0.12, f64::NAN, 0.5];
        assert!(gaps(&freqs, &clear, GAP_THRESH).is_empty());
    }

    #[test]
    fn a_leading_run_is_a_band_edge() {
        let freqs = [0.01, 0.02, 0.03];
        let values = [0.01, 0.5, 0.5];
        let found = gaps(&freqs, &values, GAP_THRESH);
        assert_eq!(found.len(), 1);
        assert!(found[0].band_edge);
        assert_eq!((found[0].lo, found[0].hi), (0.01, 0.01));
    }
}
