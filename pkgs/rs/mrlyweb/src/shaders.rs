/// The shared WGSL vertex stage that fragment programs wear.
pub const VERTEX: &str = include_str!("vertex.wgsl");

/// A gpu program: a named WGSL source and the uniform size it expects.
pub struct Program {
    /// The name lookups go by.
    pub name: &'static str,
    /// The WGSL fragment source.
    pub fragment: &'static str,
    /// The uniform buffer size, counted in floats.
    pub floats: usize,
    /// Whether the source already carries its own vertex stage.
    pub whole: bool,
}

/// Prepends the shared vertex stage to the fragment, or passes a whole program through untouched.
pub fn assemble(program: &Program) -> String {
    if program.whole {
        program.fragment.to_string()
    } else {
        format!("{VERTEX}\n{}", program.fragment)
    }
}

/// Decodes an sRGB color to its linear RGB triple, alpha dropped.
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
    fn a_whole_program_stands_alone() {
        let mesh = Program {
            name: "mesh",
            fragment: mrlymath::space::MESH_WGSL,
            floats: 24,
            whole: true,
        };
        assert_eq!(assemble(&mesh), mrlymath::space::MESH_WGSL);
    }
    #[test]
    fn a_fragment_program_wears_the_vertex_head() {
        let one = Program {
            name: "one",
            fragment: "fn fs_main() {}",
            floats: 20,
            whole: false,
        };
        let source = assemble(&one);
        assert!(source.starts_with(VERTEX));
        assert!(source.ends_with("fn fs_main() {}"));
    }
    #[test]
    fn linear_decodes_the_srgb_endpoints() {
        assert_eq!(linear([0, 0, 0, 255]), [0.0, 0.0, 0.0]);
        assert_eq!(linear([255, 255, 255, 255]), [1.0, 1.0, 1.0]);
        let mid = linear([128, 128, 128, 255])[0];
        assert!(mid > 0.21 && mid < 0.22);
    }
}
