use mrlycore::colors::Color;
use mrlycore::errors::Result;
use mrlycore::paint::Ink;
use mrlycore::ramp::Colorizer;
use mrlycore::state::{randint, sample, shuffle};
use mrlymath::two::Cell2d;

/// Returns the thirteen secondary inks, black and white excluded.
pub fn secondaries() -> Vec<Ink> {
    Ink::all()
        .into_iter()
        .filter(|ink| !matches!(ink, Ink::Black | Ink::White))
        .collect()
}

/// Draws a segment's neighbor palette: the accent kept, the draw cycled to the wanted length.
pub fn frame_palette(accent: Ink, simple: bool, wanted: usize) -> Vec<Color> {
    let pool = secondaries();
    let k = if simple {
        wanted.min(pool.len())
    } else {
        randint(2, pool.len() as i64) as usize
    };
    let mut inks = sample(&pool, k.max(1));
    if !inks.contains(&accent) {
        inks.pop();
        inks.push(accent);
        shuffle(&mut inks);
    }
    (0..wanted).map(|i| inks[i % inks.len()].color()).collect()
}

/// Builds a segment's heatmap colorizer: the primary at zero, the accent fading to its opposite pole.
pub fn heat_colorizer(primary: Ink, accent: Ink, peak: usize) -> Result<Colorizer> {
    let pole = if primary == Ink::White {
        Ink::Black
    } else {
        Ink::White
    };
    Colorizer::gradient_bins(primary.color(), &[accent.color(), pole.color()], peak)
}

/// Returns the highest cumulative visit count across a span of frames, at least one.
pub fn span_peak(grids: &[Cell2d]) -> usize {
    let Some(first) = grids.first() else {
        return 1;
    };
    let mut total = vec![0usize; first.types().size()];
    for grid in grids {
        for (i, &v) in grid.types().bytes().iter().enumerate() {
            total[i] += v as usize;
        }
    }
    total.into_iter().max().unwrap_or(0).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard, seed};
    use mrlycore::tensor::Tensor;
    #[test]
    fn thirteen_secondaries_skip_black_and_white() {
        let pool = secondaries();
        assert_eq!(pool.len(), 13);
        assert!(!pool.contains(&Ink::Black));
        assert!(!pool.contains(&Ink::White));
    }
    #[test]
    fn the_palette_keeps_the_accent_and_the_length() {
        let _g = guard();
        for s in 0..50 {
            seed(s);
            let palette = frame_palette(Ink::Teal, true, 9);
            assert_eq!(palette.len(), 9);
            assert!(palette.contains(&Ink::Teal.color()), "seed {s}");
        }
    }
    #[test]
    fn a_long_palette_cycles_its_draw() {
        let _g = guard();
        seed(3);
        let palette = frame_palette(Ink::Red, false, 40);
        assert_eq!(palette.len(), 40);
        let mut distinct = palette.clone();
        distinct.sort_by_key(|c| (c.r, c.g, c.b));
        distinct.dedup();
        assert!(distinct.len() <= 13);
        for (i, color) in palette.iter().enumerate() {
            assert_eq!(*color, palette[i % distinct.len().max(1)]);
        }
    }
    #[test]
    fn the_heat_ramp_grounds_on_the_primary() {
        let colorizer = heat_colorizer(Ink::Black, Ink::Teal, 8).unwrap();
        assert_eq!(colorizer.color(0, 8), Ink::Black.color());
        assert_eq!(colorizer.color(8, 8), Ink::White.color());
        let inverted = heat_colorizer(Ink::White, Ink::Teal, 8).unwrap();
        assert_eq!(inverted.color(0, 8), Ink::White.color());
        assert_eq!(inverted.color(8, 8), Ink::Black.color());
    }
    #[test]
    fn the_span_peak_counts_cumulative_visits() {
        let mut a = Tensor::new(vec![2, 2]);
        a.set(&[0, 0], 1);
        a.set(&[1, 1], 1);
        let mut b = Tensor::new(vec![2, 2]);
        b.set(&[0, 0], 1);
        let grids = vec![Cell2d::new(a), Cell2d::new(b)];
        assert_eq!(span_peak(&grids), 2);
        assert_eq!(span_peak(&[]), 1);
    }
}
