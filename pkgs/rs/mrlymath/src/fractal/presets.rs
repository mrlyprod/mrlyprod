/// A named Julia constant in femto ticks.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    /// The display label.
    pub label: &'static str,
    /// The real part.
    pub re: i64,
    /// The imaginary part.
    pub im: i64,
}

/// The six house Julia constants.
pub const JULIA_PRESETS: [Preset; 6] = [
    Preset {
        label: "-0.4+0.6i",
        re: -400_000_000_000_000,
        im: 600_000_000_000_000,
    },
    Preset {
        label: "-0.8+0.156i",
        re: -800_000_000_000_000,
        im: 156_000_000_000_000,
    },
    Preset {
        label: "0.285+0.01i",
        re: 285_000_000_000_000,
        im: 10_000_000_000_000,
    },
    Preset {
        label: "-0.727+0.189i",
        re: -726_900_000_000_000,
        im: 188_900_000_000_000,
    },
    Preset {
        label: "-0.1+0.651i",
        re: -100_000_000_000_000,
        im: 651_000_000_000_000,
    },
    Preset {
        label: "0.355+0.355i",
        re: 355_000_000_000_000,
        im: 355_000_000_000_000,
    },
];

/// Returns the preset with the given label, or None.
pub fn preset(label: &str) -> Option<Preset> {
    JULIA_PRESETS.iter().copied().find(|p| p.label == label)
}
