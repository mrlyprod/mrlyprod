use super::{Skin, Visual};
use mrlycore::colors::named;

pub const MODES: [&str; 2] = ["solid", "glyphs"];

const SOLID_FILL: &str = "red";
const SOLID_VOID: &str = "black";
const GLYPH_FILL: &str = "🍎";
const GLYPH_VOID: &str = "🍏";

pub fn glyphs(mode: &str) -> bool {
    mode == MODES[1]
}

pub fn defaults(mode: &str) -> (&'static str, &'static str) {
    match glyphs(mode) {
        true => (GLYPH_FILL, GLYPH_VOID),
        false => (SOLID_FILL, SOLID_VOID),
    }
}

pub fn accepts(mode: &str, value: &str) -> Result<(), &'static str> {
    match glyphs(mode) {
        true => match value.chars().count() == 1 {
            true => Ok(()),
            false => Err("one character"),
        },
        false => named(value).map(|_| ()).map_err(|_| "unknown color"),
    }
}

fn ink(name: &str, spare: &str) -> [u8; 4] {
    let color = named(name).or_else(|_| named(spare)).unwrap();
    [color.r, color.g, color.b, 255]
}

fn wear(value: &str) -> Visual {
    match value.chars().all(|c| c.is_ascii()) {
        true => Visual::none().glyph(value),
        false => Visual::none().emoji(value),
    }
}

pub fn skin(mode: &str, fill: &str, void: &str) -> Skin {
    match glyphs(mode) {
        true => Skin::new(vec![wear(void), wear(fill)]),
        false => Skin::new(vec![
            Visual::solid(ink(void, SOLID_VOID)),
            Visual::solid(ink(fill, SOLID_FILL)),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skin::Face;

    #[test]
    fn modes_carry_their_own_defaults() {
        assert_eq!(defaults("solid"), (SOLID_FILL, SOLID_VOID));
        assert_eq!(defaults("glyphs"), (GLYPH_FILL, GLYPH_VOID));
        assert!(!glyphs("solid"));
        assert!(glyphs("glyphs"));
    }
    #[test]
    fn each_mode_accepts_its_own_alphabet() {
        assert!(accepts("solid", "red").is_ok());
        assert_eq!(accepts("solid", "beige"), Err("unknown color"));
        assert!(accepts("glyphs", "🍎").is_ok());
        assert!(accepts("glyphs", "#").is_ok());
        assert_eq!(accepts("glyphs", "xy"), Err("one character"));
    }
    #[test]
    fn roles_dress_void_then_fill() {
        let solid = skin("solid", "red", "black");
        assert_eq!(solid.visuals.len(), 2);
        assert_eq!(solid.visuals[0], Visual::solid([0, 0, 0, 255]));
        assert_eq!(solid.visuals[1].bg, Some(ink("red", SOLID_FILL)));
        assert_eq!(solid.visuals[1].face, None);
        let dressed = skin("glyphs", "🍎", "#");
        assert_eq!(dressed.visuals[0].face, Some(Face::Glyph("#".into())));
        assert_eq!(dressed.visuals[1].face, Some(Face::Emoji("🍎".into())));
        assert_eq!(dressed.visuals[1].bg, None);
    }
}
