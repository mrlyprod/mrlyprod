#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub label: &'static str,
    pub re: f64,
    pub im: f64,
}

pub const JULIA_PRESETS: [Preset; 6] = [
    Preset {
        label: "-0.4+0.6i",
        re: -0.4,
        im: 0.6,
    },
    Preset {
        label: "-0.8+0.156i",
        re: -0.8,
        im: 0.156,
    },
    Preset {
        label: "0.285+0.01i",
        re: 0.285,
        im: 0.01,
    },
    Preset {
        label: "-0.727+0.189i",
        re: -0.7269,
        im: 0.1889,
    },
    Preset {
        label: "-0.1+0.651i",
        re: -0.1,
        im: 0.651,
    },
    Preset {
        label: "0.355+0.355i",
        re: 0.355,
        im: 0.355,
    },
];

pub fn preset(label: &str) -> Option<Preset> {
    JULIA_PRESETS.iter().copied().find(|p| p.label == label)
}
