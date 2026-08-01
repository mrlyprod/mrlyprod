use super::colors::{gradient, Color, ALPHA, BLACK, WHITE};
use super::errors::{value_error, Result};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Colorizer {
    TwoTone {
        background: Color,
        foreground: Color,
    },
    Wrap {
        background: Color,
        palette: Vec<Color>,
    },
    Bins {
        background: Color,
        ramp: Vec<Color>,
    },
    Exact {
        table: HashMap<usize, Color>,
        unlisted: Color,
    },
}

impl Colorizer {
    pub fn heat() -> Colorizer {
        let ramp = dedup(gradient(&[WHITE, BLACK], 128).unwrap_or_else(|_| vec![BLACK]));
        Colorizer::Bins {
            background: WHITE,
            ramp,
        }
    }
    pub fn fire() -> Colorizer {
        let stops = [
            Color::rgb(0, 0, 0),
            Color::rgb(180, 30, 0),
            Color::rgb(255, 140, 0),
            Color::rgb(255, 255, 220),
        ];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![BLACK]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    pub fn diverge() -> Colorizer {
        let stops = [
            Color::rgb(0, 90, 220),
            Color::rgb(245, 245, 245),
            Color::rgb(220, 40, 40),
        ];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![WHITE]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    pub fn two_tone(background: Color, foreground: Color) -> Colorizer {
        Colorizer::TwoTone {
            background,
            foreground,
        }
    }
    pub fn wrap(background: Color, palette: Vec<Color>) -> Colorizer {
        Colorizer::Wrap {
            background,
            palette,
        }
    }
    pub fn gradient_bins(background: Color, colors: &[Color], shades: usize) -> Result<Colorizer> {
        let ramp = dedup(gradient(colors, shades.max(1))?);
        Ok(Colorizer::Bins { background, ramp })
    }
    pub fn exact(entries: &[(Color, Vec<usize>)]) -> Result<Colorizer> {
        Colorizer::exact_with(entries, ALPHA)
    }
    pub fn exact_with(entries: &[(Color, Vec<usize>)], unlisted: Color) -> Result<Colorizer> {
        let mut seen_colors: Vec<Color> = Vec::with_capacity(entries.len());
        let mut table: HashMap<usize, Color> = HashMap::new();
        for (color, tags) in entries {
            if seen_colors.contains(color) {
                return value_error(format!(
                    "duplicate colour rgba({},{},{},{}) in ramp; list each colour once with all its tags.",
                    color.r, color.g, color.b, color.a
                ));
            }
            seen_colors.push(*color);
            for &tag in tags {
                if table.insert(tag, *color).is_some() {
                    return value_error(format!("tag {tag} assigned to more than one colour."));
                }
            }
        }
        Ok(Colorizer::Exact { table, unlisted })
    }
    pub fn color(&self, value: usize, max: usize) -> Color {
        match self {
            Colorizer::TwoTone {
                background,
                foreground,
            } => {
                if value == 0 {
                    *background
                } else {
                    *foreground
                }
            }
            Colorizer::Wrap {
                background,
                palette,
            } => {
                if value == 0 || palette.is_empty() {
                    *background
                } else {
                    palette[(value - 1) % palette.len()]
                }
            }
            Colorizer::Bins { background, ramp } => {
                if value == 0 || ramp.is_empty() {
                    return *background;
                }
                if max <= 1 {
                    return ramp[ramp.len() - 1];
                }
                let idx = (value - 1) * (ramp.len() - 1) / (max - 1).max(1);
                ramp[idx.min(ramp.len() - 1)]
            }
            Colorizer::Exact { table, unlisted } => *table.get(&value).unwrap_or(unlisted),
        }
    }
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
    use crate::colors::{BLUE, RED, WHITE};
    #[test]
    fn two_tone_ignores_magnitude() {
        let r = Colorizer::two_tone(WHITE, BLACK);
        assert_eq!(r.color(0, 9), WHITE);
        assert_eq!(r.color(1, 9), BLACK);
        assert_eq!(r.color(7, 9), BLACK);
    }
    #[test]
    fn wrap_cycles_palette() {
        let r = Colorizer::wrap(WHITE, vec![RED, BLACK]);
        assert_eq!(r.color(0, 9), WHITE);
        assert_eq!(r.color(1, 9), RED);
        assert_eq!(r.color(2, 9), BLACK);
        assert_eq!(r.color(3, 9), RED);
    }
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
    fn exact_maps_listed_tags_rest_transparent() {
        let r = Colorizer::exact(&[(RED, vec![2, 3, 5, 7]), (BLUE, vec![4, 6])]).unwrap();
        assert_eq!(r.color(3, 9), RED);
        assert_eq!(r.color(7, 9), RED);
        assert_eq!(r.color(4, 9), BLUE);
        assert_eq!(r.color(9, 9), ALPHA);
        assert_eq!(r.color(0, 9), ALPHA);
    }
    #[test]
    fn exact_with_paints_the_rest() {
        let r = Colorizer::exact_with(&[(RED, vec![2, 3])], BLUE).unwrap();
        assert_eq!(r.color(2, 9), RED);
        assert_eq!(r.color(8, 9), BLUE);
    }
    #[test]
    fn exact_rejects_overlapping_tags() {
        let err = Colorizer::exact(&[(RED, vec![2, 3]), (BLUE, vec![3, 4])]);
        assert!(err.is_err());
    }
    #[test]
    fn exact_rejects_duplicate_colors() {
        let err = Colorizer::exact(&[(RED, vec![2]), (RED, vec![5])]);
        assert!(err.is_err());
    }
    #[test]
    fn dedup_collapses_repeats() {
        assert_eq!(
            dedup(vec![BLACK, BLACK, WHITE, WHITE, BLACK]),
            vec![BLACK, WHITE, BLACK]
        );
    }
}
