use super::colors::{gradient, shades, Color, BLACK, BLUE, ORANGE, RED, WHITE, YELLOW};
use super::error::Result;

/// A rule that turns counter values into colors.
#[derive(Clone, Debug)]
pub enum Colorizer {
    /// The background at zero, then the ramp binned across the range.
    Bins {
        /// The color at zero.
        background: Color,
        /// The colors binned across the range.
        ramp: Vec<Color>,
    },
}

impl Colorizer {
    /// Builds the white-to-black heat ramp.
    pub fn heat() -> Colorizer {
        let ramp = dedup(gradient(&[WHITE, BLACK], 128).unwrap_or_else(|_| vec![BLACK]));
        Colorizer::Bins {
            background: WHITE,
            ramp,
        }
    }
    /// Builds the black-through-ember fire ramp: black, dark red, orange, light yellow.
    pub fn fire() -> Colorizer {
        let stops = [BLACK, shades(RED)[2], ORANGE, shades(YELLOW)[0]];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![BLACK]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    /// Builds the blue-to-red diverging ramp around a white middle.
    pub fn diverge() -> Colorizer {
        let stops = [BLUE, WHITE, RED];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![WHITE]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    /// Builds a binned colorizer from a gradient through the given stops.
    pub fn gradient_bins(background: Color, colors: &[Color], shades: usize) -> Result<Colorizer> {
        let ramp = dedup(gradient(colors, shades.max(1))?);
        Ok(Colorizer::Bins { background, ramp })
    }
    /// Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.
    pub fn color(&self, value: usize, max: usize) -> Color {
        match self {
            Colorizer::Bins { background, ramp } => {
                if value == 0 || ramp.is_empty() {
                    return *background;
                }
                if max <= 1 {
                    return ramp[ramp.len() - 1];
                }
                let idx = (value - 1).saturating_mul(ramp.len() - 1) / (max - 1);
                ramp[idx.min(ramp.len() - 1)]
            }
        }
    }
    /// Maps a slice of values to rgba pixels against the range maximum.
    pub fn colors(&self, values: &[usize], max: usize) -> Vec<[u8; 4]> {
        values
            .iter()
            .map(|&v| {
                let c = self.color(v, max);
                [c.r, c.g, c.b, c.a]
            })
            .collect()
    }
}

impl Default for Colorizer {
    fn default() -> Colorizer {
        Colorizer::heat()
    }
}

fn dedup(colors: Vec<Color>) -> Vec<Color> {
    let mut out: Vec<Color> = Vec::with_capacity(colors.len());
    for c in colors {
        if out.last() != Some(&c) {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heat_is_white_bg_dark_max() {
        let r = Colorizer::heat();
        assert_eq!(r.color(0, 10), WHITE);
        assert_eq!(r.color(10, 10), BLACK);
    }
    #[test]
    fn bins_spread_across_range() {
        let r = Colorizer::gradient_bins(WHITE, &[WHITE, BLACK], 4).unwrap();
        let low = r.color(1, 100);
        let high = r.color(100, 100);
        assert!(low.r > high.r);
    }
    #[test]
    fn dedup_collapses_repeats() {
        assert_eq!(
            dedup(vec![BLACK, BLACK, WHITE, WHITE, BLACK]),
            vec![BLACK, WHITE, BLACK]
        );
    }
}
