const VERTEX: &str = include_str!("vertex.wgsl");

const PROGRAMS: [(&str, &str, usize); 6] = [
    ("mandelbrot", include_str!("mandelbrot.wgsl"), 20),
    ("julia", include_str!("julia.wgsl"), 24),
    ("waves", include_str!("waves.wgsl"), 16),
    ("billiards", include_str!("billiards.wgsl"), 16),
    ("lasers", include_str!("lasers.wgsl"), 16),
    ("mesh", include_str!("mesh.wgsl"), 24),
];

fn assemble(name: &str, fragment: &str) -> String {
    if name == "mesh" {
        fragment.to_string()
    } else {
        format!("{VERTEX}\n{fragment}")
    }
}

pub fn names() -> Vec<&'static str> {
    PROGRAMS.iter().map(|(name, _, _)| *name).collect()
}

pub fn floats(name: &str) -> Option<usize> {
    PROGRAMS
        .iter()
        .find(|(found, _, _)| *found == name)
        .map(|(_, _, floats)| *floats)
}

pub fn source(name: &str) -> Option<String> {
    PROGRAMS
        .iter()
        .find(|(found, _, _)| *found == name)
        .map(|(found, fragment, _)| assemble(found, fragment))
}

pub fn all() -> Vec<(&'static str, String)> {
    PROGRAMS
        .iter()
        .map(|(name, fragment, _)| (*name, assemble(name, fragment)))
        .collect()
}

pub fn linear(color: [u8; 4]) -> [f64; 3] {
    let decode = |v: u8| {
        let f = v as f64 / 255.0;
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    };
    [decode(color[0]), decode(color[1]), decode(color[2])]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_program_is_complete() {
        assert!(!names().is_empty());
        for (name, source) in all() {
            assert!(source.contains("fn vs_main"), "{name} misses vs_main");
            assert!(source.contains("fn fs_main"), "{name} misses fs_main");
            assert!(source.contains("var<uniform>"), "{name} misses uniforms");
        }
    }
    #[test]
    fn uniform_sizes_hold_the_std_layout() {
        for name in names() {
            let floats = floats(name).unwrap();
            assert!(floats >= 12, "{name} misses the shared head");
            assert_eq!(floats % 4, 0, "{name} is not 16 byte aligned");
        }
    }
    #[test]
    fn lookups_agree() {
        assert!(source("mandelbrot").is_some());
        assert!(source("nothing").is_none());
        assert!(floats("nothing").is_none());
        assert_eq!(all().len(), names().len());
    }
    #[test]
    fn linear_decodes_the_srgb_endpoints() {
        assert_eq!(linear([0, 0, 0, 255]), [0.0, 0.0, 0.0]);
        assert_eq!(linear([255, 255, 255, 255]), [1.0, 1.0, 1.0]);
        let mid = linear([128, 128, 128, 255])[0];
        assert!(mid > 0.21 && mid < 0.22);
    }
}
