use crate::frame::{bake, hex, motif_tile, solid_tile, TileSet};
use mrlycore::{json, Json};

#[derive(Clone, Debug, PartialEq)]
pub enum Face {
    Glyph(String),
    Emoji(String),
    Sprite(Vec<Vec<u8>>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Visual {
    pub bg: Option<[u8; 4]>,
    pub motif: Option<String>,
    pub face: Option<Face>,
}

impl Visual {
    pub fn none() -> Visual {
        Visual::default()
    }
    pub fn solid(color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(color),
            ..Visual::default()
        }
    }
    pub fn motif(name: &str, color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(color),
            motif: Some(name.to_string()),
            ..Visual::default()
        }
    }
    pub fn glyph(self, text: impl Into<String>) -> Visual {
        Visual {
            face: Some(Face::Glyph(text.into())),
            ..self
        }
    }
    pub fn emoji(self, value: impl Into<String>) -> Visual {
        Visual {
            face: Some(Face::Emoji(value.into())),
            ..self
        }
    }
    pub fn sprite(self, rows: Vec<Vec<u8>>) -> Visual {
        Visual {
            face: Some(Face::Sprite(rows)),
            ..self
        }
    }
    fn to_json(&self) -> Json {
        let mut out = json!({});
        if let Some(bg) = self.bg {
            out["bg"] = json!(hex(bg));
        }
        if let Some(motif) = &self.motif {
            out["motif"] = json!(motif);
        }
        match &self.face {
            Some(Face::Glyph(text)) => out["face"] = json!({ "as": "glyph", "value": text }),
            Some(Face::Emoji(value)) => out["face"] = json!({ "as": "emoji", "value": value }),
            Some(Face::Sprite(rows)) => {
                out["face"] = json!({ "as": "sprite", "rows": rows.clone() })
            }
            None => {}
        }
        out
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skin {
    pub visuals: Vec<Visual>,
}

impl Skin {
    pub fn new(visuals: Vec<Visual>) -> Skin {
        Skin { visuals }
    }
    pub fn to_json(&self) -> Json {
        json!(self.visuals.iter().map(Visual::to_json).collect::<Vec<_>>())
    }
    pub fn tileset(&self, k: usize, ink: [u8; 4]) -> TileSet {
        let clear = [0, 0, 0, 0];
        let tiles = self
            .visuals
            .iter()
            .map(|v| {
                let color = v.bg.unwrap_or(clear);
                let mut tile = match &v.motif {
                    Some(name) => motif_tile(name, k, color, clear),
                    None => solid_tile(k, color),
                };
                match &v.face {
                    Some(Face::Glyph(text)) => bake(&mut tile, text, k, ink),
                    Some(Face::Emoji(value)) => crate::emoji::bake(&mut tile, value, k),
                    _ => {}
                }
                tile
            })
            .collect();
        TileSet::new(k, tiles)
    }
}

pub mod chess;
pub mod memory;
pub mod mines;
pub mod twenty48;
pub mod two;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_json_omits_absent_parts() {
        let plain = Visual::none().to_json();
        assert_eq!(plain, json!({}));
        let full = Visual::motif("carpet", [255, 0, 0, 255])
            .glyph("3")
            .to_json();
        assert_eq!(full["bg"], json!("#ff0000"));
        assert_eq!(full["motif"], json!("carpet"));
        assert_eq!(full["face"], json!({ "as": "glyph", "value": "3" }));
        let emoji = Visual::none().emoji("💣").to_json();
        assert_eq!(emoji["face"]["as"], json!("emoji"));
        assert!(emoji.get("bg").is_none());
    }
    #[test]
    fn skin_json_is_indexed_by_role() {
        let skin = Skin::new(vec![Visual::none(), Visual::solid([0, 0, 255, 255])]);
        let out = skin.to_json();
        assert_eq!(out.as_array().unwrap().len(), 2);
        assert_eq!(out[1]["bg"], json!("#0000ff"));
    }
    #[test]
    fn tileset_builds_solid_motif_and_baked() {
        let red = [255, 0, 0, 255];
        let ink = [0, 0, 0, 255];
        let skin = Skin::new(vec![
            Visual::solid(red),
            Visual::motif("carpet", red),
            Visual::solid(red).glyph("8"),
            Visual::none().emoji("💣"),
        ]);
        let set = skin.tileset(8, ink);
        assert_eq!(set.size, 8);
        assert_eq!(set.tiles.len(), 4);
        assert_eq!(set.tiles[0].cell.colors, solid_tile(8, red).cell.colors);
        assert_eq!(
            set.tiles[1].cell.colors,
            motif_tile("carpet", 8, red, [0, 0, 0, 0]).cell.colors
        );
        assert!(set.tiles[2].cell.colors.as_ref().unwrap().contains(&ink));
        assert!(set.tiles[3]
            .cell
            .colors
            .as_ref()
            .unwrap()
            .iter()
            .any(|c| c[3] > 0));
    }
    #[test]
    fn mines_variants_dress_the_roles() {
        let colors = [[10, 10, 10, 255]; 9];
        let hidden = [70, 70, 78, 255];
        let mine = [220, 40, 40, 255];
        let tiles = mines::skin("tiles", "carpet", &colors, hidden, mine);
        assert_eq!(tiles.visuals.len(), 12);
        assert_eq!(tiles.visuals[0], Visual::solid(hidden));
        assert_eq!(tiles.visuals[2].face, None);
        assert_eq!(tiles.visuals[10], Visual::solid(mine));
        let digits = mines::skin("digits", "carpet", &colors, hidden, mine);
        assert_eq!(digits.visuals[2].face, Some(Face::Glyph("1".into())));
        assert_eq!(digits.visuals[1].face, None);
        assert_eq!(digits.visuals[10].face, Some(Face::Glyph("X".into())));
        let emojis = mines::skin("emojis", "carpet", &colors, hidden, mine);
        assert_eq!(emojis.visuals[1], Visual::none());
        assert_eq!(emojis.visuals[3].face, Some(Face::Glyph("2".into())));
        assert_eq!(emojis.visuals[3].bg, None);
        assert_eq!(emojis.visuals[10].face, Some(Face::Emoji("💣".into())));
        assert_eq!(
            emojis.visuals[mines::FLAG].face,
            Some(Face::Emoji("⛳".into()))
        );
    }
}
