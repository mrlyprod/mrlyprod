use super::colors::{BLACK, BLUE, Color, ORANGE, RED, WHITE, YELLOW, gradient, shades};
use super::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Colorizer {
    Bins { background: Color, ramp: Vec<Color> },
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
        let stops = [BLACK, shades(RED)[2], ORANGE, shades(YELLOW)[0]];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![BLACK]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    pub fn diverge() -> Colorizer {
        let stops = [BLUE, WHITE, RED];
        let ramp = dedup(gradient(&stops, 128).unwrap_or_else(|_| vec![WHITE]));
        Colorizer::Bins {
            background: ramp[0],
            ramp,
        }
    }
    pub fn gradient_bins(background: Color, colors: &[Color], shades: usize) -> Result<Colorizer> {
        let ramp = dedup(gradient(colors, shades.max(1))?);
        Ok(Colorizer::Bins { background, ramp })
    }
}

pub fn color(colorizer: &Colorizer, value: usize, max: usize) -> Color {
    match colorizer {
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

pub fn colors(colorizer: &Colorizer, values: &[usize], max: usize) -> Vec<[u8; 4]> {
    values
        .iter()
        .map(|&v| {
            let c = color(colorizer, v, max);
            [c.r, c.g, c.b, c.a]
        })
        .collect()
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
