mod helpers;
mod render;
mod rules;
mod state;

use crate::core::paint::{self, Paint};
use crate::core::state::seed;
use crate::core::tile::{Catalog, Design, Group, Parity, Tile as Model};
use crate::math::two::tile as tile2d;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::ui::frame;
use helpers::{catalog_name, source_label, work};
use render::{blank, two_tone};
use rules::{carpet, starter, validate_saved};
use serde_json::{json, Value as Json};

const BUDGETS: [usize; 3] = [16, 32, 64];
const MIN: usize = 2;
const CEILING: usize = 64;
const THUMBS: usize = 6;
const SHELF: usize = 12;
const STARTERS: [Design; 4] = [Design::Carpet, Design::Net, Design::Htree, Design::Vtree];

struct Entry {
    id: u64,
    name: String,
    tile: Model,
    paint: Option<Paint>,
}

fn seed_library() -> Vec<Entry> {
    STARTERS
        .iter()
        .enumerate()
        .map(|(i, &design)| Entry {
            id: (i + 1) as u64,
            name: design.name().to_lowercase(),
            tile: starter(design),
            paint: None,
        })
        .collect()
}

pub struct Tile {
    tile: Model,
    paint: Option<Paint>,
    catalog: Catalog,
    parity: Parity,
    budget: usize,
    frame: Json,
    dark: bool,
    library: Vec<Entry>,
    next: u64,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::new()
    }
}

impl Tile {
    pub fn new() -> Tile {
        let mut app = Tile {
            tile: carpet(),
            paint: None,
            catalog: Catalog::Classics,
            parity: Parity::Odds,
            budget: 64,
            frame: Json::Null,
            dark: false,
            library: seed_library(),
            next: STARTERS.len() as u64 + 1,
        };
        app.repaint();
        app
    }
    fn repaint(&mut self) {
        let board = crate::ui::frame::board(self.dark);
        let fill = crate::ui::frame::ink(self.dark);
        self.frame = match tile2d::build(&self.tile) {
            Ok(mut cell) => {
                let (w, h) = (cell.width(), cell.height());
                let colors = match &self.paint {
                    Some(coating) if paint::coat(&mut cell.cell, coating, None).is_ok() => (0
                        ..cell.cell.size())
                        .map(|i| {
                            let c = cell.cell.color_at(i);
                            if c[3] == 0 {
                                board
                            } else {
                                c
                            }
                        })
                        .collect(),
                    _ => two_tone(&cell, board, fill),
                };
                frame::field(w, h, colors, board).fact()
            }
            Err(_) => blank(board),
        };
    }
}

impl App for Tile {
    fn route(&self) -> &str {
        "tile"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("tile").emoji("🀄").category("math")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
        self.repaint();
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "tile": self.tile.to_json(),
            "paint": self.paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
            "catalog": catalog_name(&self.catalog),
            "parity": self.parity.name(),
            "budget": self.budget,
            "options": self.options(),
            "thumbs": self.thumbs(),
            "library": self.shelf(),
            "frame": self.frame,
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new(
                "tile.set",
                json!({ "key": "string", "value": "any", "slot": "int" }),
            ),
            Verb::new("tile.roll", json!({ "seed": "int" })),
            Verb::new("tile.paint", json!({ "seed": "int" })),
            Verb::new("tile.strip", json!({})),
            Verb::new("tile.reset", json!({})),
            Verb::new("tile.save", json!({ "name": "string" })),
            Verb::new("tile.name", json!({ "id": "int", "name": "string" })),
            Verb::new("tile.drop", json!({ "id": "int" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "tile.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value"), call.arg("slot")) {
                    Ok(value) => {
                        self.repaint();
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            "tile.roll" => {
                let s = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                seed(s);
                let config = tile2d::Config {
                    groups: Group::all().to_vec(),
                    catalog: self.catalog.clone(),
                    min_size: MIN,
                    max_size: self.budget,
                    parity: self.parity,
                    invert: None,
                    anti: None,
                };
                match tile2d::create(&config) {
                    Ok(tile) => {
                        self.tile = tile;
                        if self.paint.is_some() {
                            self.roll_paint();
                        }
                        self.repaint();
                        Outcome::ok(json!({ "seed": s }))
                    }
                    Err(_) => Outcome::fail("could not roll a tile"),
                }
            }
            "tile.paint" => {
                let s = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                seed(s);
                self.roll_paint();
                self.repaint();
                Outcome::ok(json!({ "seed": s }))
            }
            "tile.strip" => {
                self.paint = None;
                self.repaint();
                Outcome::ok(json!({ "stripped": true }))
            }
            "tile.reset" => {
                *self = Tile::new();
                Outcome::ok(json!({}))
            }
            "tile.save" => {
                if self.library.len() >= SHELF {
                    return Outcome::fail("library is full");
                }
                let bundle = work(&self.tile, &self.paint);
                if self
                    .library
                    .iter()
                    .any(|e| work(&e.tile, &e.paint) == bundle)
                {
                    return Outcome::fail("already saved");
                }
                let id = self.next;
                let provided = call.arg("name").as_str().unwrap_or("").trim().to_string();
                let name = if provided.is_empty() {
                    self.tile
                        .sources
                        .first()
                        .map(source_label)
                        .unwrap_or_else(|| format!("tile {id}"))
                } else {
                    provided
                };
                self.library.push(Entry {
                    id,
                    name: name.clone(),
                    tile: self.tile.clone(),
                    paint: self.paint.clone(),
                });
                self.next += 1;
                Outcome::ok(json!({ "id": id, "name": name }))
            }
            "tile.name" => {
                let id = call.arg("id").as_u64().unwrap_or(0);
                let name = call.arg("name").as_str().unwrap_or("").trim().to_string();
                if name.is_empty() {
                    return Outcome::fail("empty name");
                }
                match self.library.iter_mut().find(|e| e.id == id) {
                    Some(entry) => {
                        entry.name = name.clone();
                        Outcome::ok(json!({ "id": id, "name": name }))
                    }
                    None => Outcome::fail("unknown tile"),
                }
            }
            "tile.drop" => {
                let id = call.arg("id").as_u64().unwrap_or(0);
                match self.library.iter().position(|e| e.id == id) {
                    Some(index) => {
                        self.library.remove(index);
                        Outcome::ok(json!({ "id": id }))
                    }
                    None => Outcome::fail("unknown tile"),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        json!({
            "tile": self.tile.to_json(),
            "paint": self.paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
            "catalog": catalog_name(&self.catalog),
            "parity": self.parity.name(),
            "budget": self.budget,
            "library": self
                .library
                .iter()
                .map(|e| {
                    json!({
                        "id": e.id,
                        "name": e.name,
                        "tile": e.tile.to_json(),
                        "paint": e.paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
                    })
                })
                .collect::<Vec<_>>(),
            "next": self.next,
        })
    }
    fn load(&mut self, state: &Json) {
        *self = Tile::new();
        if let Some(budget) = state["budget"].as_u64() {
            if BUDGETS.contains(&(budget as usize)) {
                self.budget = budget as usize;
            }
        }
        if let Ok(parity) = Parity::parse(state["parity"].as_str().unwrap_or("")) {
            self.parity = parity;
        }
        if state["catalog"].as_str() == Some("Universe") {
            self.catalog = Catalog::Universe;
        }
        if let Ok((model, coating)) = validate_saved(state) {
            self.tile = model;
            self.paint = coating;
        }
        if let Some(entries) = state["library"].as_array() {
            let mut library = Vec::new();
            let mut top: u64 = 0;
            for entry in entries {
                if let Ok((tile, paint)) = validate_saved(entry) {
                    let id = entry["id"].as_u64().unwrap_or(0);
                    let trimmed = entry["name"].as_str().unwrap_or("").trim();
                    let name = if trimmed.is_empty() {
                        tile.sources
                            .first()
                            .map(source_label)
                            .unwrap_or_else(|| format!("tile {id}"))
                    } else {
                        trimmed.to_string()
                    };
                    top = top.max(id);
                    library.push(Entry {
                        id,
                        name,
                        tile,
                        paint,
                    });
                }
            }
            self.library = library;
            self.next = state["next"].as_u64().unwrap_or(0).max(top + 1);
        }
        self.snap();
        self.repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::rules::{carpet, check_model, resize};
    use super::*;
    use crate::core::paint::{Edition, Target};
    use crate::core::state::guard;
    use crate::core::tile::{Design, Source};
    use crate::os::kernel::testkit::{iden, send};

    fn app() -> Tile {
        Tile::new()
    }
    fn set(t: &mut Tile, key: &str, value: Json) -> Outcome {
        send(t, "tile.set", json!({ "key": key, "value": value }))
    }

    #[test]
    fn boot_touches_no_rng() {
        let _g = guard();
        seed(1);
        let a = app();
        seed(99);
        let b = app();
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.tile.group, Group::Fractal);
        assert_eq!(a.tile.numbers, vec![3]);
        assert_eq!(a.tile.levels, vec![2]);
        assert_eq!(a.tile.max_size(), 9);
        assert!(a.paint.is_none());
    }
    #[test]
    fn group_switches_stay_legal() {
        let mut t = app();
        for group in ["General", "Magic", "Special", "Mosaic", "Fractal"] {
            let out = set(&mut t, "group", json!(group));
            assert!(out.ok, "group {group}");
            assert!(t.tile.max_size() <= 64, "group {group}");
            assert!(check_model(&t.tile).is_ok(), "group {group}");
        }
        assert!(!set(&mut t, "group", json!("Sparkle")).ok);
    }
    #[test]
    fn group_switch_carries_the_lead_source() {
        let mut t = app();
        assert!(set(&mut t, "source", json!("Net")).ok);
        assert!(set(&mut t, "group", json!("Magic")).ok);
        assert_eq!(t.tile.sources[0], Source::Classic(Design::Net));
    }
    #[test]
    fn fractal_number_snaps_level() {
        let mut t = app();
        assert!(set(&mut t, "number", json!(7)).ok);
        assert_eq!(t.tile.levels, vec![2]);
        assert_eq!(t.tile.max_size(), 49);
        assert!(set(&mut t, "number", json!(3)).ok);
        assert!(set(&mut t, "level", json!(3)).ok);
        assert_eq!(t.tile.max_size(), 27);
        assert!(set(&mut t, "number", json!(7)).ok);
        assert_eq!(t.tile.levels, vec![2]);
        assert!(!set(&mut t, "number", json!(6)).ok);
        assert!(!set(&mut t, "level", json!(9)).ok);
    }
    #[test]
    fn magic_snap_lands_in_nestings() {
        let mut t = app();
        assert!(set(&mut t, "group", json!("Magic")).ok);
        assert!(set(&mut t, "count", json!(3)).ok);
        assert_eq!(t.tile.numbers.len(), 3);
        assert!(t.nestings_of().contains(&t.tile.numbers));
        let out = send(
            &mut t,
            "tile.set",
            json!({ "key": "number", "value": 5, "slot": 1 }),
        );
        assert!(out.ok);
        assert_eq!(t.tile.numbers[1], 5);
        assert!(t.nestings_of().contains(&t.tile.numbers));
        assert!(t.tile.max_size() <= 64);
        assert_eq!(t.tile.sources.len(), 3);
        assert_eq!(t.tile.anti.len(), 3);
    }
    #[test]
    fn special_pair_stays_in_products() {
        let mut t = app();
        assert!(set(&mut t, "group", json!("Special")).ok);
        assert!(set(&mut t, "factor", json!(5)).ok);
        let pair = vec![t.tile.factor, t.tile.numbers[0]];
        assert!(t.pairs_of().contains(&pair));
        assert!(set(&mut t, "flip", json!(true)).ok);
        assert!(t.tile.flip);
        assert!(set(&mut t, "group", json!("General")).ok);
        assert!(!set(&mut t, "flip", json!(true)).ok);
    }
    #[test]
    fn parity_and_budget_snap_the_staged_tile() {
        let mut t = app();
        assert!(set(&mut t, "level", json!(3)).ok);
        assert_eq!(t.tile.max_size(), 27);
        assert!(set(&mut t, "budget", json!(16)).ok);
        assert!(t.tile.max_size() <= 16);
        assert!(check_model(&t.tile).is_ok());
        assert!(set(&mut t, "parity", json!("Evens")).ok);
        assert_eq!(t.tile.numbers[0] % 2, 0);
        assert!(check_model(&t.tile).is_ok());
        assert!(!set(&mut t, "budget", json!(100)).ok);
    }
    #[test]
    fn catalog_remaps_sources_by_index() {
        let mut t = app();
        assert!(set(&mut t, "catalog", json!("Universe")).ok);
        assert!(matches!(t.tile.sources[0], Source::Code(_)));
        assert!(check_model(&t.tile).is_ok());
        let label = source_label(&t.tile.sources[0]);
        assert!(label.starts_with("mrly_"));
        assert!(set(&mut t, "catalog", json!("Classics")).ok);
        assert!(matches!(t.tile.sources[0], Source::Classic(_)));
    }
    #[test]
    fn roll_is_seeded() {
        let _g = guard();
        let mut a = app();
        let mut b = app();
        for t in [&mut a, &mut b] {
            assert!(send(t, "tile.roll", json!({ "seed": 7 })).ok);
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert!(a.tile.max_size() <= 64);
        assert!(check_model(&a.tile).is_ok());
    }
    #[test]
    fn paint_dice_is_seeded_and_respects_staged_knobs() {
        let _g = guard();
        let mut a = app();
        let mut b = app();
        for t in [&mut a, &mut b] {
            assert!(set(t, "edition", json!("Layers")).ok);
            assert!(set(t, "target", json!("Void")).ok);
            assert!(send(t, "tile.paint", json!({ "seed": 7 })).ok);
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        let coating = a.paint.as_ref().unwrap();
        assert_eq!(coating.edition, Edition::Layers);
        assert_eq!(coating.target, Target::Void);
        assert!(!coating.secondary.is_empty());
    }
    #[test]
    fn coat_is_deterministic() {
        let _g = guard();
        let mut t = app();
        assert!(send(&mut t, "tile.paint", json!({ "seed": 11 })).ok);
        let frame = t.frame.clone();
        let mut back = app();
        back.load(&t.save());
        assert_eq!(back.frame, frame);
        seed(555);
        back.repaint();
        assert_eq!(back.frame, frame);
    }
    #[test]
    fn load_rejects_bad_bundles() {
        let mut t = app();
        let before = t.state(&iden());
        t.load(&json!({ "tile": { "group": "General" }, "paint": Json::Null }));
        assert_eq!(t.state(&iden()), before);
        let mut oversize = carpet();
        oversize.levels = vec![5];
        resize(&mut oversize);
        t.load(&json!({ "tile": oversize.to_json(), "paint": Json::Null }));
        assert_eq!(t.state(&iden()), before);
        let mut flipped = carpet();
        flipped.flip = true;
        t.load(&json!({ "tile": flipped.to_json(), "paint": Json::Null }));
        assert_eq!(t.state(&iden()), before);
        let sane = carpet();
        t.load(&json!({ "tile": sane.to_json(), "paint": { "edition": "Sparkle" } }));
        assert_eq!(t.state(&iden()), before);
    }
    #[test]
    fn strip_clears_the_paint() {
        let _g = guard();
        let mut t = app();
        assert!(send(&mut t, "tile.paint", json!({ "seed": 7 })).ok);
        assert!(t.paint.is_some());
        let bare = app().frame.clone();
        assert!(send(&mut t, "tile.strip", json!({})).ok);
        assert!(t.paint.is_none());
        assert_eq!(t.frame, bare);
    }
    #[test]
    fn paint_knobs_stage_a_default_paint() {
        let mut t = app();
        assert!(t.paint.is_none());
        assert!(set(&mut t, "edition", json!("Rows")).ok);
        let coating = t.paint.as_ref().unwrap();
        assert_eq!(coating.edition, Edition::Rows);
        assert!(set(&mut t, "scheme", json!("Multitone")).ok);
        let coating = t.paint.as_ref().unwrap();
        assert_eq!(coating.secondary.len(), 1);
        assert!(!coating.shades.is_empty());
        assert!(set(&mut t, "scheme", json!("Multicolor")).ok);
        assert!(t.paint.as_ref().unwrap().shades.is_empty());
        assert!(set(&mut t, "primary", json!("White")).ok);
        assert!(set(&mut t, "primary", json!("Teal")).ok);
        assert!(!set(&mut t, "primary", json!("Neon")).ok);
        assert!(set(&mut t, "target", json!("Void")).ok);
        assert!(!set(&mut t, "edition", json!("Sparkle")).ok);
    }
    #[test]
    fn save_load_round_trips() {
        let _g = guard();
        let mut a = app();
        assert!(send(&mut a, "tile.roll", json!({ "seed": 9 })).ok);
        assert!(send(&mut a, "tile.paint", json!({ "seed": 2 })).ok);
        assert!(set(&mut a, "budget", json!(32)).ok);
        let mut b = app();
        b.load(&a.save());
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn load_survives_garbage() {
        let mut t = app();
        t.load(&json!({ "tile": 7, "budget": "soup", "parity": [1] }));
        assert_eq!(t.state(&iden()), app().state(&iden()));
    }
    #[test]
    fn reset_restores_the_defaults() {
        let _g = guard();
        let mut t = app();
        assert!(send(&mut t, "tile.roll", json!({ "seed": 5 })).ok);
        assert!(send(&mut t, "tile.reset", json!({})).ok);
        assert_eq!(t.state(&iden()), app().state(&iden()));
    }
    #[test]
    fn thumbs_ride_the_fractal_levels() {
        let mut t = app();
        let thumbs = t.thumbs();
        assert_eq!(thumbs.len(), 2);
        assert_eq!(thumbs[0]["level"], json!(2));
        assert_eq!(thumbs[1]["level"], json!(3));
        assert!(set(&mut t, "group", json!("General")).ok);
        assert!(t.thumbs().is_empty());
    }
    #[test]
    fn state_carries_the_studio() {
        let t = app();
        let state = t.state(&iden());
        assert_eq!(state["paint"], Json::Null);
        assert_eq!(state["catalog"], json!("Classics"));
        assert_eq!(state["parity"], json!("Odds"));
        assert_eq!(state["budget"], json!(64));
        assert_eq!(state["options"]["budgets"], json!([16, 32, 64]));
        assert_eq!(state["options"]["groups"].as_array().unwrap().len(), 5);
        assert!(state["options"]["sources"][0]["label"].is_string());
        assert!(state["frame"]["rows"].is_array());
        let back = Model::from_json(&state["tile"]).unwrap();
        assert_eq!(back, t.tile);
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let t = app();
        let names: Vec<String> = t.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "tile.set",
                "tile.roll",
                "tile.paint",
                "tile.strip",
                "tile.reset",
                "tile.save",
                "tile.name",
                "tile.drop"
            ]
        );
    }
    #[test]
    fn library_seeds_four_buildable_starters() {
        let _g = guard();
        seed(1);
        let a = app();
        seed(99);
        let b = app();
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.library.len(), 4);
        let names: Vec<&str> = a.library.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["carpet", "net", "htree", "vtree"]);
        for (i, entry) in a.library.iter().enumerate() {
            assert_eq!(entry.id, (i + 1) as u64);
            assert!(entry.paint.is_none());
            assert!(tile2d::build(&entry.tile).is_ok());
        }
        assert_eq!(a.next, 5);
    }
    #[test]
    fn library_value_parses_back_to_a_model() {
        let t = app();
        let cards = t.state(&iden())["library"].as_array().unwrap().clone();
        assert_eq!(cards.len(), 4);
        assert_eq!(cards[0]["id"], json!(1));
        assert_eq!(cards[0]["name"], json!("carpet"));
        for card in &cards {
            assert_eq!(card["value"]["v"], json!(1));
            assert_eq!(card["value"]["paint"], Json::Null);
            let tile = Model::from_json(&card["value"]["tile"]).unwrap();
            assert!(tile2d::build(&tile).is_ok());
            assert!(card["frame"]["rows"].is_array());
        }
    }
    #[test]
    fn save_dedupes_and_names() {
        let mut t = app();
        let out = send(&mut t, "tile.save", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("already saved"));
        assert!(set(&mut t, "group", json!("General")).ok);
        let out = send(&mut t, "tile.save", json!({ "name": "  keeper  " }));
        assert!(out.ok);
        assert_eq!(out.data["name"], json!("keeper"));
        assert_eq!(out.data["id"], json!(5));
        assert_eq!(t.library.len(), 5);
        assert_eq!(t.next, 6);
        assert_eq!(
            send(&mut t, "tile.save", json!({})).note.as_deref(),
            Some("already saved")
        );
    }
    #[test]
    fn save_auto_names_from_the_source() {
        let mut t = app();
        assert!(set(&mut t, "group", json!("General")).ok);
        let out = send(&mut t, "tile.save", json!({}));
        assert!(out.ok);
        let label = source_label(&t.tile.sources[0]);
        assert_eq!(out.data["name"], json!(label));
    }
    #[test]
    fn save_caps_the_library() {
        let mut t = app();
        while t.library.len() < SHELF {
            let id = t.next;
            t.library.push(Entry {
                id,
                name: format!("x{id}"),
                tile: carpet(),
                paint: None,
            });
            t.next += 1;
        }
        let out = send(&mut t, "tile.save", json!({}));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("library is full"));
    }
    #[test]
    fn name_and_drop_edit_the_library() {
        let mut t = app();
        let out = send(&mut t, "tile.name", json!({ "id": 2, "name": "  webby  " }));
        assert!(out.ok);
        assert_eq!(out.data["name"], json!("webby"));
        assert_eq!(t.library[1].name, "webby");
        assert_eq!(
            send(&mut t, "tile.name", json!({ "id": 2, "name": "   " }))
                .note
                .as_deref(),
            Some("empty name")
        );
        assert_eq!(
            send(&mut t, "tile.name", json!({ "id": 99, "name": "x" }))
                .note
                .as_deref(),
            Some("unknown tile")
        );
        let out = send(&mut t, "tile.drop", json!({ "id": 1 }));
        assert!(out.ok);
        assert_eq!(t.library.len(), 3);
        assert!(!t.library.iter().any(|e| e.id == 1));
        assert_eq!(
            send(&mut t, "tile.drop", json!({ "id": 1 }))
                .note
                .as_deref(),
            Some("unknown tile")
        );
    }
    #[test]
    fn save_load_carries_the_library() {
        let _g = guard();
        let mut a = app();
        assert!(send(&mut a, "tile.roll", json!({ "seed": 9 })).ok);
        assert!(send(&mut a, "tile.paint", json!({ "seed": 2 })).ok);
        assert!(send(&mut a, "tile.save", json!({ "name": "keeper" })).ok);
        let mut b = app();
        b.load(&a.save());
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
        assert_eq!(b.library.len(), 5);
        assert_eq!(b.next, a.next);
    }
    #[test]
    fn load_reseeds_starters_when_library_missing() {
        let mut t = app();
        send(&mut t, "tile.drop", json!({ "id": 1 }));
        t.load(&json!({ "tile": carpet().to_json(), "paint": Json::Null }));
        assert_eq!(t.library.len(), 4);
        assert_eq!(t.library[0].name, "carpet");
        assert_eq!(t.next, 5);
    }
    #[test]
    fn load_drops_invalid_library_entries() {
        let mut t = app();
        let mut oversize = carpet();
        oversize.levels = vec![5];
        resize(&mut oversize);
        t.load(&json!({
            "tile": carpet().to_json(),
            "paint": Json::Null,
            "library": [
                { "id": 10, "name": "keep", "tile": carpet().to_json(), "paint": Json::Null },
                { "id": 11, "name": "toobig", "tile": oversize.to_json(), "paint": Json::Null },
                { "id": 12, "name": "junk", "tile": 7, "paint": Json::Null },
            ],
            "next": 3,
        }));
        assert_eq!(t.library.len(), 1);
        assert_eq!(t.library[0].id, 10);
        assert_eq!(t.library[0].name, "keep");
        assert_eq!(t.next, 11);
    }
}
