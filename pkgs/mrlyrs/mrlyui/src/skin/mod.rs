use crate::frame::{
    bake, board, hex, hex_of, ink, motif_tile, solid_tile, Atlas, Frame, Glyph, Layer,
};
use mrlycore::json::Map;
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

#[derive(Clone, Debug, PartialEq)]
pub enum Ink {
    Hex([u8; 4]),
    Pen(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Motif {
    Name(String),
    Design,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tint {
    Ink,
    Pen(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Face {
    Glyph(String),
    Emoji(String),
    Sprite(Vec<Vec<u8>>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Visual {
    pub bg: Option<Ink>,
    pub motif: Option<Motif>,
    pub face: Option<Face>,
    pub tint: Option<Tint>,
}

impl Visual {
    pub fn none() -> Visual {
        Visual::default()
    }
    pub fn solid(color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(Ink::Hex(color)),
            ..Visual::default()
        }
    }
    pub fn pen(n: usize) -> Visual {
        Visual {
            bg: Some(Ink::Pen(n)),
            ..Visual::default()
        }
    }
    pub fn motif(name: &str, color: [u8; 4]) -> Visual {
        Visual {
            bg: Some(Ink::Hex(color)),
            motif: Some(Motif::Name(name.to_string())),
            ..Visual::default()
        }
    }
    pub fn design(self) -> Visual {
        Visual {
            motif: Some(Motif::Design),
            ..self
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skin {
    pub visuals: Vec<Visual>,
}

pub const SYMBOL: usize = 16;

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
    pub fn new(visuals: Vec<Visual>) -> Skin {
        Skin { visuals }
    }
    pub fn to_json(&self) -> Json {
        json!(self.visuals.iter().map(Visual::to_json).collect::<Vec<_>>())
    }
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

const APPS: [&str; 12] = [
    "captcha", "chess", "crush", "dice", "escape", "memory", "mines", "pixel", "quiz", "snake",
    "ttt", "twenty48",
];

fn wardrobe(app: &str) -> Vec<(&'static str, Skin)> {
    match app {
        "captcha" => captcha::corpus(),
        "chess" => chess::corpus(),
        "crush" => crush::corpus(),
        "dice" => dice::corpus(),
        "escape" => escape::corpus(),
        "memory" => memory::corpus(),
        "mines" => mines::corpus(),
        "pixel" => pixel::corpus(),
        "quiz" => quiz::corpus(),
        "snake" => snake::corpus(),
        "ttt" => ttt::corpus(),
        "twenty48" => twenty48::corpus(),
        _ => Vec::new(),
    }
}

pub fn corpus() -> Json {
    let mut out = Map::new();
    for app in APPS {
        let mut entry = Map::new();
        for (variant, skin) in wardrobe(app) {
            entry.insert(variant.to_string(), skin.to_json());
        }
        out.insert(app.to_string(), Json::Obj(entry));
    }
    Json::Obj(out)
}

pub fn dressed(app: &str, variant: &str) -> Option<Skin> {
    wardrobe(app)
        .into_iter()
        .find(|(name, _)| *name == variant)
        .map(|(_, skin)| skin)
}

pub fn raster(app: &str, cells: &Json, tile: usize, dark: bool) -> Option<Frame> {
    let variant = cells["skin"].as_str()?;
    let skin = dressed(app, variant)?;
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
    let mut frame = Frame::new(cols * k, rows * k, board(dark));
    frame.push(Layer::Tiles { ids, set });
    Some(frame)
}

pub mod captcha;
pub mod chess;
pub mod crush;
pub mod dice;
pub mod escape;
pub mod memory;
pub mod mines;
pub mod pixel;
pub mod quiz;
pub mod snake;
pub mod ttt;
pub mod twenty48;
pub mod two;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_tiles_carry_faces_not_pixels() {
        assert_eq!(pixels(3, "emojis"), SYMBOL);
        assert_eq!(pixels(40, "emojis"), 40);
        assert_eq!(pixels(3, "digits"), 3);
        let k = pixels(3, "emojis");
        let set = twenty48::skin("emojis").atlas(k, [0, 0, 0, 255], &[], "carpet");
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
    fn atlas_serves_chess_symbols_tinted() {
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
    fn mines_variants_dress_the_roles() {
        let tiles = mines::skin("tiles");
        assert_eq!(tiles.visuals.len(), 12);
        assert_eq!(tiles.visuals[0], Visual::pen(0));
        assert_eq!(tiles.visuals[2], Visual::pen(2).design());
        assert_eq!(tiles.visuals[10], Visual::pen(10));
        let digits = mines::skin("digits");
        assert_eq!(digits.visuals[2].face, Some(Face::Glyph("1".into())));
        assert_eq!(digits.visuals[1].face, None);
        assert_eq!(digits.visuals[10].face, Some(Face::Glyph("X".into())));
        let emojis = mines::skin("emojis");
        assert_eq!(emojis.visuals[1], Visual::none());
        assert_eq!(emojis.visuals[3].face, Some(Face::Glyph("2".into())));
        assert_eq!(emojis.visuals[3].bg, None);
        assert_eq!(emojis.visuals[10].face, Some(Face::Emoji("💣".into())));
        assert_eq!(
            emojis.visuals[mines::FLAG].face,
            Some(Face::Emoji("⛳".into()))
        );
    }

    #[test]
    fn corpus_names_every_tile_app() {
        let all = corpus();
        for app in APPS {
            let wardrobe = all[app].as_object().unwrap_or_else(|| {
                panic!("{app} misses its corpus entry");
            });
            assert!(!wardrobe.is_empty(), "{app} is undressed");
            for (variant, skin) in wardrobe {
                assert!(
                    !skin.as_array().unwrap().is_empty(),
                    "{app} {variant} has no roles"
                );
            }
        }
        assert_eq!(all.as_object().unwrap().len(), APPS.len());
        assert!(all.get("two").is_none());
        assert_eq!(all["ttt"]["tiles"][1]["bg"], json!({ "pen": 0 }));
        assert_eq!(all["ttt"]["tiles"][1]["motif"], json!("design"));
        assert_eq!(
            all["twenty48"]["digits"][3]["face"],
            json!({ "as": "glyph", "value": "8" })
        );
        assert_eq!(all["pixel"]["tiles"][7]["bg"], json!({ "pen": 7 }));
    }
    #[test]
    fn raster_paints_cells_like_the_old_frames() {
        let cells = json!({
            "ids": [[0, 1], [2, 0]],
            "skin": "tiles",
            "pens": ["#ff0000", "#0000ff"],
            "design": "carpet",
        });
        let frame = raster("ttt", &cells, 4, false).expect("a frame");
        assert_eq!(frame.width, 8);
        assert_eq!(frame.height, 8);
        let colors = frame.composite().cell.colors.unwrap();
        let x = motif_tile("carpet", 4, [255, 0, 0, 255], [0, 0, 0, 0]);
        assert_eq!(colors[4], x.cell.color_at(0));
        assert!(raster("ttt", &json!({ "skin": "wax", "ids": [[0]] }), 4, false).is_none());
        assert!(raster("two", &cells, 4, false).is_none());
        let painting = raster("pixel", &cells, 4, false).expect("a pixel frame");
        assert_eq!(painting.width, 8);
        let colors = painting.composite().cell.colors.unwrap();
        assert_eq!(colors[4], [0, 0, 255, 255]);
    }
}
