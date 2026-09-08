use mrlycore::colors::{Theme, DARK, LIGHT};
use mrlycore::Color;
use std::sync::OnceLock;

// THEME

/// The theme every figure is painted in: light when MRLYFIG_THEME is "light", dark otherwise, read once.
pub fn theme() -> &'static Theme {
    static THEME: OnceLock<&'static Theme> = OnceLock::new();
    THEME.get_or_init(|| match std::env::var("MRLYFIG_THEME").as_deref() {
        Ok("light") => &LIGHT,
        _ => &DARK,
    })
}

/// The name of the theme in press, "dark" or "light".
pub fn name() -> &'static str {
    if *theme() == LIGHT {
        "light"
    } else {
        "dark"
    }
}

// SURFACES

/// The ground every figure is painted on.
pub fn ground() -> Color {
    theme().ground
}

/// The raised panel, one step off the ground.
pub fn panel() -> Color {
    theme().panel
}

/// The hairline that separates one thing from the next.
pub fn line() -> Color {
    theme().line
}

/// The foreground, the strongest tone on the ground.
pub fn fg() -> Color {
    theme().fg
}

/// The dimmed foreground, for anything secondary.
pub fn dim() -> Color {
    theme().dim
}

// HUES

/// The red ink.
pub fn red() -> Color {
    theme().red
}

/// The orange ink.
pub fn orange() -> Color {
    theme().orange
}

/// The yellow ink.
pub fn yellow() -> Color {
    theme().yellow
}

/// The green ink.
pub fn green() -> Color {
    theme().green
}

/// The mint ink.
pub fn mint() -> Color {
    theme().mint
}

/// The teal ink.
pub fn teal() -> Color {
    theme().teal
}

/// The cyan ink.
pub fn cyan() -> Color {
    theme().cyan
}

/// The blue ink.
pub fn blue() -> Color {
    theme().blue
}

/// The indigo ink.
pub fn indigo() -> Color {
    theme().indigo
}

/// The purple ink.
pub fn purple() -> Color {
    theme().purple
}

/// The pink ink.
pub fn pink() -> Color {
    theme().pink
}

/// The brown ink.
pub fn brown() -> Color {
    theme().brown
}

/// The gray ink.
pub fn gray() -> Color {
    theme().gray
}

/// The six inks in their fixed order, the wheel a figure cycles through: blue, orange, yellow, green, pink, indigo.
pub fn inks() -> [Color; 6] {
    theme().inks()
}

// MIXING

/// Blends two colors channel by channel, t clamped to the unit interval.
pub fn mix(a: Color, b: Color, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    Color::rgba(
        lerp(a.r, b.r),
        lerp(a.g, b.g),
        lerp(a.b, b.b),
        lerp(a.a, b.a),
    )
}

/// Returns the color at a fraction of its opacity, alpha clamped to the unit interval.
pub fn fade(c: Color, alpha: f64) -> Color {
    Color::rgba(c.r, c.g, c.b, (255.0 * alpha.clamp(0.0, 1.0)).round() as u8)
}

// RAMP

/// A color ramp: a line through its stops, read at any point of the unit interval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ramp {
    /// The stops, evenly spaced from zero to one.
    pub stops: Vec<Color>,
}

impl Ramp {
    /// Builds a ramp from its stops, which must not be empty.
    pub fn new(stops: Vec<Color>) -> Ramp {
        Ramp { stops }
    }
    /// Reads the ramp at t, clamped to the unit interval; the ground when there are no stops.
    pub fn at(&self, t: f64) -> Color {
        if self.stops.is_empty() {
            return ground();
        }
        if self.stops.len() == 1 {
            return self.stops[0];
        }
        let t = t.clamp(0.0, 1.0) * (self.stops.len() - 1) as f64;
        let i = (t.floor() as usize).min(self.stops.len() - 2);
        mix(self.stops[i], self.stops[i + 1], t - i as f64)
    }
    /// The heat ramp: ground, blue, yellow, foreground.
    pub fn heat() -> Ramp {
        Ramp::new(vec![ground(), blue(), yellow(), fg()])
    }
    /// The fire ramp: ground, orange, yellow, foreground.
    pub fn fire() -> Ramp {
        Ramp::new(vec![ground(), orange(), yellow(), fg()])
    }
    /// The diverging ramp: blue through the ground to orange.
    pub fn diverge() -> Ramp {
        Ramp::new(vec![blue(), ground(), orange()])
    }
    /// The two-tone ramp from one color straight to another.
    pub fn tone(a: Color, b: Color) -> Ramp {
        Ramp::new(vec![a, b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ramp_ends_are_its_end_stops() {
        let ramp = Ramp::heat();
        assert_eq!(ramp.at(0.0), ground());
        assert_eq!(ramp.at(1.0), fg());
    }
    #[test]
    fn an_empty_ramp_reads_the_ground() {
        assert_eq!(Ramp::new(vec![]).at(0.5), ground());
    }
    #[test]
    fn mix_halfway_sits_between_the_two() {
        assert_eq!(
            mix(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255), 0.5).r,
            128
        );
    }
    #[test]
    fn the_inks_are_the_themes_six() {
        assert_eq!(inks(), theme().inks());
        assert_eq!(inks()[0], blue());
        assert_eq!(inks()[5], indigo());
    }
}
