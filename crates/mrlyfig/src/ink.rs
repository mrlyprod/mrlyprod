use mrlycore::Color;

// PALETTE

/// The dark ground every figure is painted on.
pub const GROUND: Color = Color::rgb(0x07, 0x09, 0x0b);
/// The raised panel, one step above the ground.
pub const PANEL: Color = Color::rgb(0x12, 0x16, 0x1b);
/// The hairline that separates one thing from the next.
pub const LINE: Color = Color::rgb(0x1f, 0x26, 0x2e);
/// The foreground, the brightest tone in the set.
pub const FG: Color = Color::rgb(0xe8, 0xec, 0xf1);
/// The dimmed foreground, for anything secondary.
pub const DIM: Color = Color::rgb(0x7f, 0x8a, 0x97);
/// The blue ink.
pub const BLUE: Color = Color::rgb(0x5c, 0xc8, 0xff);
/// The orange ink.
pub const ORANGE: Color = Color::rgb(0xff, 0x8a, 0x5c);
/// The gold ink.
pub const GOLD: Color = Color::rgb(0xff, 0xd1, 0x66);
/// The green ink.
pub const GREEN: Color = Color::rgb(0x6e, 0xe7, 0xa8);
/// The pink ink.
pub const PINK: Color = Color::rgb(0xff, 0x7a, 0xb6);
/// The violet ink.
pub const VIOLET: Color = Color::rgb(0xa9, 0x9c, 0xff);

/// The six inks in their fixed order, the wheel a figure cycles through.
pub const INKS: [Color; 6] = [BLUE, ORANGE, GOLD, GREEN, PINK, VIOLET];

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
    /// Reads the ramp at t, clamped to the unit interval.
    pub fn at(&self, t: f64) -> Color {
        if self.stops.is_empty() {
            return GROUND;
        }
        if self.stops.len() == 1 {
            return self.stops[0];
        }
        let t = t.clamp(0.0, 1.0) * (self.stops.len() - 1) as f64;
        let i = (t.floor() as usize).min(self.stops.len() - 2);
        mix(self.stops[i], self.stops[i + 1], t - i as f64)
    }
    /// The heat ramp: ground, blue, gold, foreground.
    pub fn heat() -> Ramp {
        Ramp::new(vec![GROUND, BLUE, GOLD, FG])
    }
    /// The fire ramp: ground, orange, gold, foreground.
    pub fn fire() -> Ramp {
        Ramp::new(vec![GROUND, ORANGE, GOLD, FG])
    }
    /// The diverging ramp: blue through the ground to orange.
    pub fn diverge() -> Ramp {
        Ramp::new(vec![BLUE, GROUND, ORANGE])
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
        assert_eq!(ramp.at(0.0), GROUND);
        assert_eq!(ramp.at(1.0), FG);
    }
    #[test]
    fn mix_halfway_sits_between_the_two() {
        assert_eq!(
            mix(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255), 0.5).r,
            128
        );
    }
}
