use crate::paint::{bake, motif_tile, solid_tile, Atlas, Glyph, Raster};
use mrlycore::colors::{board, hex, hex_of, ink};
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymath::two::Cell2d;
use std::collections::HashSet;
use std::sync::OnceLock;

fn lettered(text: &str) -> bool {
    static SET: OnceLock<HashSet<char>> = OnceLock::new();
    let set = SET.get_or_init(|| mrlyfont::supported().into_iter().collect());
    !text.is_empty() && text.chars().all(|c| set.contains(&c))
}

/// The ground of a tile: a literal color or a pen resolved at paint time.
#[derive(Clone, Debug, PartialEq)]
pub enum Ink {
    /// A literal RGBA color.
    Hex([u8; 4]),
    /// An index into the caller's pens.
    Pen(usize),
}

/// The weave woven into a tile's ground.
#[derive(Clone, Debug, PartialEq)]
pub enum Motif {
    /// A named weave: carpet, net, htree or vtree.
    Name(String),
    /// The board's own design, chosen at paint time.
    Design,
}

/// The color a face is drawn in.
#[derive(Clone, Debug, PartialEq)]
pub enum Tint {
    /// The board ink.
    Ink,
    /// An index into the caller's pens.
    Pen(usize),
}

/// The figure drawn over a tile's ground.
#[derive(Clone, Debug, PartialEq)]
pub enum Face {
    /// A text face, baked into pixels when the font covers it.
    Glyph(String),
    /// An emoji face, carried untinted as a glyph.
    Emoji(String),
    /// An on/off row face, stamped into the tile's pixels.
    Sprite(Vec<Vec<u8>>),
}

/// The dress of one role: ground, weave, face and tint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Visual {
    /// The ground ink.
    pub bg: Option<Ink>,
    /// The weave woven into the ground.
    pub motif: Option<Motif>,
    /// The figure drawn over the ground.
    pub face: Option<Face>,
    /// The color of the face.
    pub tint: Option<Tint>,
}

impl Visual {
    /// Builds the bare visual with no parts.
    pub fn none() -> Visual {
        Visual::default()
    }
    /// Builds a visual grounded in one literal color.
    pub fn solid(color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(Ink::Hex(color)),
            ..Visual::default()
        }
    }
    /// Builds a visual grounded in a caller's pen.
    pub fn pen(n: usize) -> Visual {
        Visual {
            bg: Some(Ink::Pen(n)),
            ..Visual::default()
        }
    }
    /// Builds a visual weaving the named motif into one literal color.
    pub fn motif(name: &str, color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(Ink::Hex(color)),
            motif: Some(Motif::Name(name.to_string())),
            ..Visual::default()
        }
    }
    /// Reweaves the visual with the board's own design.
    pub fn design(self) -> Visual {
        Visual {
            motif: Some(Motif::Design),
            ..self
        }
    }
    /// Dresses the visual with a text face.
    pub fn glyph(self, text: impl Into<String>) -> Visual {
        Visual {
            face: Some(Face::Glyph(text.into())),
            ..self
        }
    }
    /// Dresses the visual with an emoji face.
    pub fn emoji(self, value: impl Into<String>) -> Visual {
        Visual {
            face: Some(Face::Emoji(value.into())),
            ..self
        }
    }
    /// Dresses the visual with an on/off row face.
    pub fn sprite(self, rows: Vec<Vec<u8>>) -> Visual {
        Visual {
            face: Some(Face::Sprite(rows)),
            ..self
        }
    }
    /// Colors the visual's face with the given tint.
    pub fn tinted(self, tint: Tint) -> Visual {
        Visual {
            tint: Some(tint),
            ..self
        }
    }
    fn to_json(&self) -> Json {
        let mut out = json!({});
        match &self.bg {
            Some(Ink::Hex(color)) => out["bg"] = json!(hex(*color)),
            Some(Ink::Pen(n)) => out["bg"] = json!({ "pen": n }),
            None => {}
        }
        match &self.motif {
            Some(Motif::Name(name)) => out["motif"] = json!(name),
            Some(Motif::Design) => out["motif"] = json!("design"),
            None => {}
        }
        match &self.face {
            Some(Face::Glyph(text)) => out["face"] = json!({ "as": "glyph", "value": text }),
            Some(Face::Emoji(value)) => out["face"] = json!({ "as": "emoji", "value": value }),
            Some(Face::Sprite(rows)) => {
                out["face"] = json!({ "as": "sprite", "rows": rows.clone() })
            }
            None => {}
        }
        if self.face.is_some() {
            match &self.tint {
                Some(Tint::Ink) => out["face"]["tint"] = json!("ink"),
                Some(Tint::Pen(n)) => out["face"]["tint"] = json!({ "pen": n }),
                None => {}
            }
        }
        out
    }
}

/// A list of visuals, one per role.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skin {
    /// The visuals, indexed by role.
    pub visuals: Vec<Visual>,
}

/// The floor tile side for emoji skins.
pub const SYMBOL: usize = 16;

/// Returns the tile side in pixels, floored at SYMBOL for the emojis variant.
///
/// ```
/// assert_eq!(mrlyskin::pixels(3, "emojis"), 16);
/// assert_eq!(mrlyskin::pixels(3, "digits"), 3);
/// ```
pub fn pixels(tile: usize, variant: &str) -> usize {
    match variant {
        "emojis" => tile.max(SYMBOL),
        _ => tile,
    }
}

fn stamp(tile: &mut Cell2d, rows: &[Vec<u8>], k: usize, tint: [u8; 4]) {
    let h = rows.len();
    let w = rows.first().map(Vec::len).unwrap_or(0);
    if h == 0 || w == 0 {
        return;
    }
    if let Some(colors) = tile.cell.colors.as_mut() {
        for y in 0..k {
            for x in 0..k {
                if rows[y * h / k][x * w / k] != 0 {
                    colors[y * k + x] = tint;
                }
            }
        }
    }
}

impl Skin {
    /// Builds a skin from visuals indexed by role.
    pub fn new(visuals: Vec<Visual>) -> Skin {
        Skin { visuals }
    }
    /// Returns the skin as a JSON list of visuals, one per role.
    pub fn to_json(&self) -> Json {
        json!(self.visuals.iter().map(Visual::to_json).collect::<Vec<_>>())
    }
    /// Bakes every visual at one tile side into an atlas, carrying the faces that will not bake.
    pub fn atlas(&self, k: usize, ink: [u8; 4], pens: &[[u8; 4]], design: &str) -> Atlas {
        let clear = [0, 0, 0, 0];
        let pen = |n: usize| pens.get(n).copied().unwrap_or(clear);
        let mut set = Atlas::new(k, Vec::new());
        for v in &self.visuals {
            let color = match &v.bg {
                Some(Ink::Hex(c)) => *c,
                Some(Ink::Pen(n)) => pen(*n),
                None => clear,
            };
            let weave = match &v.motif {
                Some(Motif::Name(name)) => Some(name.as_str()),
                Some(Motif::Design) => Some(design),
                None => None,
            };
            let mut tile = match weave {
                Some(name) => motif_tile(name, k, color, clear),
                None => solid_tile(k, color),
            };
            let tint = match &v.tint {
                Some(Tint::Pen(n)) => pen(*n),
                _ => ink,
            };
            let mut face = None;
            match &v.face {
                Some(Face::Glyph(text)) if lettered(text) => bake(&mut tile, text, k, tint),
                Some(Face::Glyph(value)) => {
                    face = Some(Glyph {
                        ch: value.clone(),
                        tint: Some(tint),
                    })
                }
                Some(Face::Emoji(value)) => {
                    face = Some(Glyph {
                        ch: value.clone(),
                        tint: None,
                    })
                }
                Some(Face::Sprite(rows)) => stamp(&mut tile, rows, k, tint),
                None => {}
            }
            set.tiles.push(tile);
            set.faces.push(face);
        }
        set
    }
}

/// Returns the wardrobe skin wearing the variant name, or None when none does.
///
/// ```
/// use mrlyskin::{dressed, Skin, Visual};
/// let wardrobe = vec![("plain", Skin::new(vec![Visual::solid([1, 2, 3, 255])]))];
/// assert_eq!(dressed(wardrobe, "plain").unwrap().visuals.len(), 1);
/// ```
pub fn dressed(wardrobe: Vec<(&'static str, Skin)>, variant: &str) -> Option<Skin> {
    wardrobe
        .into_iter()
        .find(|(name, _)| *name == variant)
        .map(|(_, skin)| skin)
}

/// Dresses a cells fact's ids in the wardrobe skin it names, or None when the fact is missing a piece.
pub fn raster(
    wardrobe: Vec<(&'static str, Skin)>,
    cells: &Json,
    tile: usize,
    dark: bool,
) -> Option<Raster> {
    let variant = cells["skin"].as_str()?;
    let skin = dressed(wardrobe, variant)?;
    let facts = cells["ids"].as_array()?;
    let rows = facts.len();
    let cols = facts.first()?.as_array()?.len();
    if cols == 0 {
        return None;
    }
    let mut ids = Tensor::new(vec![rows, cols]);
    for (r, row) in facts.iter().enumerate() {
        for (c, id) in row.as_array()?.iter().enumerate() {
            ids.set(&[r, c], id.as_u64()? as u8);
        }
    }
    let pens: Vec<[u8; 4]> = cells["pens"]
        .as_array()
        .map(|list| list.iter().filter_map(Json::as_str).map(hex_of).collect())
        .unwrap_or_default();
    let design = cells["design"].as_str().unwrap_or("");
    let k = pixels(tile, variant);
    let set = skin.atlas(k, ink(dark), &pens, design);
    Some(Raster::new(ids, set, board(dark)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Vec<(&'static str, Skin)> {
        vec![
            (
                "tiles",
                Skin::new(vec![
                    Visual::pen(0),
                    Visual::pen(1).design(),
                    Visual::solid([0, 255, 0, 255]),
                ]),
            ),
            ("plain", Skin::new(vec![Visual::solid([1, 2, 3, 255])])),
        ]
    }

    #[test]
    fn emoji_tiles_carry_faces_not_pixels() {
        assert_eq!(pixels(3, "emojis"), SYMBOL);
        assert_eq!(pixels(40, "emojis"), 40);
        assert_eq!(pixels(3, "digits"), 3);
        let k = pixels(3, "emojis");
        let skin = Skin::new(vec![Visual::none(), Visual::none().emoji("🍒")]);
        let set = skin.atlas(k, [0, 0, 0, 255], &[], "carpet");
        let glyph = set.faces[1].as_ref().expect("an emoji face");
        assert!(!glyph.ch.is_empty());
        assert_eq!(glyph.tint, None);
        let tile = set.tiles[1].cell.colors.clone().unwrap();
        assert!(
            tile.iter().all(|px| px[3] == 0),
            "an emoji tile baked pixels"
        );
    }

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
        let sprite = Visual::none()
            .sprite(vec![vec![1, 0], vec![0, 1]])
            .to_json();
        assert_eq!(
            sprite["face"],
            json!({ "as": "sprite", "rows": [[1, 0], [0, 1]] })
        );
    }
    #[test]
    fn slot_json_speaks_pens_and_design() {
        let slot = Visual::pen(1).design().glyph("X").tinted(Tint::Ink);
        let out = slot.to_json();
        assert_eq!(out["bg"], json!({ "pen": 1 }));
        assert_eq!(out["motif"], json!("design"));
        assert_eq!(out["face"]["tint"], json!("ink"));
        let penned = Visual::none().glyph("Q").tinted(Tint::Pen(0)).to_json();
        assert_eq!(penned["face"]["tint"], json!({ "pen": 0 }));
        assert!(Visual::pen(0)
            .tinted(Tint::Ink)
            .to_json()
            .get("face")
            .is_none());
    }
    #[test]
    fn skin_json_is_indexed_by_role() {
        let skin = Skin::new(vec![Visual::none(), Visual::solid([0, 0, 255, 255])]);
        let out = skin.to_json();
        assert_eq!(out.as_array().unwrap().len(), 2);
        assert_eq!(out[1]["bg"], json!("#0000ff"));
    }
    #[test]
    fn atlas_builds_solid_motif_and_baked() {
        let red = [255, 0, 0, 255];
        let ink = [0, 0, 0, 255];
        let skin = Skin::new(vec![
            Visual::solid(red),
            Visual::motif("carpet", red),
            Visual::solid(red).glyph("8"),
            Visual::none().emoji("💣"),
        ]);
        let set = skin.atlas(8, ink, &[], "");
        assert_eq!(set.size, 8);
        assert_eq!(set.tiles.len(), 4);
        assert_eq!(set.tiles[0].cell.colors, solid_tile(8, red).cell.colors);
        assert_eq!(
            set.tiles[1].cell.colors,
            motif_tile("carpet", 8, red, [0, 0, 0, 0]).cell.colors
        );
        assert!(set.tiles[2].cell.colors.as_ref().unwrap().contains(&ink));
        assert_eq!(
            set.faces[3],
            Some(Glyph {
                ch: "💣".into(),
                tint: None
            })
        );
    }
    #[test]
    fn atlas_resolves_pens_design_and_tints() {
        let blue = [0, 0, 255, 255];
        let gold = [255, 200, 0, 255];
        let ink = [0, 0, 0, 255];
        let skin = Skin::new(vec![
            Visual::pen(0).design(),
            Visual::none().glyph("♞").tinted(Tint::Pen(1)),
            Visual::pen(9),
        ]);
        let set = skin.atlas(8, ink, &[blue, gold], "net");
        assert_eq!(
            set.tiles[0].cell.colors,
            motif_tile("net", 8, blue, [0, 0, 0, 0]).cell.colors
        );
        assert_eq!(
            set.faces[1],
            Some(Glyph {
                ch: "♞".into(),
                tint: Some(gold)
            })
        );
        assert_eq!(
            set.tiles[2].cell.colors,
            solid_tile(8, [0, 0, 0, 0]).cell.colors
        );
    }
    #[test]
    fn atlas_stamps_sprite_faces() {
        let ink = [9, 9, 9, 255];
        let rows = vec![vec![1, 0], vec![0, 1]];
        let skin = Skin::new(vec![Visual::none().sprite(rows)]);
        let set = skin.atlas(4, ink, &[], "");
        let colors = set.tiles[0].cell.colors.clone().unwrap();
        assert_eq!(colors[0], ink);
        assert_eq!(colors[3], [0, 0, 0, 0]);
        assert_eq!(colors[15], ink);
        assert_eq!(set.faces[0], None);
    }
    #[test]
    fn atlas_serves_symbols_tinted() {
        let ink = [220, 10, 10, 255];
        let skin = Skin::new(vec![
            Visual::none().emoji("♔"),
            Visual::none().glyph("♞"),
            Visual::none().glyph("A"),
        ]);
        let set = skin.atlas(16, ink, &[], "");
        assert_eq!(
            set.faces[0],
            Some(Glyph {
                ch: "♔".into(),
                tint: None
            })
        );
        assert_eq!(
            set.faces[1],
            Some(Glyph {
                ch: "♞".into(),
                tint: Some(ink)
            })
        );
        assert_eq!(set.faces[2], None);
        assert!(set.tiles[2]
            .cell
            .colors
            .as_ref()
            .unwrap()
            .iter()
            .any(|c| c[3] > 0));
    }

    #[test]
    fn dressed_finds_the_named_variant() {
        let skin = dressed(table(), "plain").expect("a skin");
        assert_eq!(skin.visuals, vec![Visual::solid([1, 2, 3, 255])]);
        assert_eq!(dressed(table(), "tiles").unwrap().visuals.len(), 3);
        assert!(dressed(table(), "wax").is_none());
        assert!(dressed(Vec::new(), "tiles").is_none());
    }
    #[test]
    fn raster_paints_the_cells_fact() {
        let cells = json!({
            "ids": [[0, 1], [2, 0]],
            "skin": "tiles",
            "pens": ["#ff0000", "#0000ff"],
            "design": "carpet",
        });
        let shot = raster(table(), &cells, 4, false).expect("a raster");
        assert_eq!(shot.width(), 8);
        assert_eq!(shot.height(), 8);
        let colors = shot.composite().cell.colors.unwrap();
        assert_eq!(colors[0], [255, 0, 0, 255]);
        let woven = motif_tile("carpet", 4, [0, 0, 255, 255], [0, 0, 0, 0]);
        assert_eq!(colors[4], woven.cell.color_at(0));
        assert_eq!(colors[32], [0, 255, 0, 255]);
        assert!(raster(table(), &json!({ "skin": "wax", "ids": [[0]] }), 4, false).is_none());
        assert!(raster(Vec::new(), &cells, 4, false).is_none());
    }
}
