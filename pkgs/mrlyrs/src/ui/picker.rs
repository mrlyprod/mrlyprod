use crate::core::tile::{
    generals, nestings, powers, products, Catalog, Group, Parity, Source, Tile as Model,
};
use crate::math::two::tile as tile2d;
use crate::ui::frame;
use serde_json::{json, Value as Json};

const MIN: usize = 2;
const BUDGET: usize = 16;
const PAGE: usize = 12;

pub fn source_label(source: &Source) -> String {
    match source {
        Source::Classic(design) => design.name().to_string(),
        Source::Code(code) => format!("mrly_{code:02}"),
    }
}

pub fn glyphs(set: &str) -> Json {
    match set {
        "emoji" => {
            let entries: Vec<Json> = crate::core::emoji::names()
                .iter()
                .map(|name| json!({ "name": name, "glyphs": crate::core::emoji::grid(name) }))
                .collect();
            json!(entries)
        }
        "font" => {
            let glyphs: Vec<String> = crate::font::supported()
                .iter()
                .map(|c| c.to_string())
                .collect();
            json!([{ "name": "font", "glyphs": glyphs }])
        }
        _ => json!([]),
    }
}

fn finish(mut tile: Model) -> Model {
    let slots = tile.sources.len();
    if tile.levels.len() != slots {
        tile.levels = vec![1; slots];
    }
    tile.rotations = vec![0; slots];
    tile.anti = vec![false; slots];
    if !matches!(tile.group, Group::Special | Group::Mosaic) {
        tile.factor = tile.numbers[0];
    }
    let size = match tile.group {
        Group::General => tile.numbers[0],
        Group::Fractal => tile.numbers[0].pow(tile.levels[0] as u32),
        Group::Magic => tile.numbers.iter().product(),
        Group::Special | Group::Mosaic => tile.factor * tile.numbers[0],
    };
    tile.width = size;
    tile.height = size;
    tile
}

fn catalogue(group: Group, catalog: &Catalog) -> Vec<(String, Model)> {
    let sources = crate::math::bang::sources(catalog, 2);
    let mut out = Vec::new();
    match group {
        Group::General => {
            for n in generals(MIN, BUDGET, Parity::Both) {
                for &source in &sources {
                    let mut tile = Model::new(Group::General);
                    tile.numbers = vec![n];
                    tile.sources = vec![source];
                    out.push((format!("{} {}", source_label(&source), n), finish(tile)));
                }
            }
        }
        Group::Fractal => {
            for (n, level) in powers(MIN, BUDGET, Parity::Both) {
                for &source in &sources {
                    let mut tile = Model::new(Group::Fractal);
                    tile.numbers = vec![n];
                    tile.levels = vec![level];
                    tile.sources = vec![source];
                    let name = format!("{} {}^{}", source_label(&source), n, level);
                    out.push((name, finish(tile)));
                }
            }
        }
        Group::Magic => {
            for numbers in nestings(MIN, BUDGET, Parity::Both) {
                for &source in &sources {
                    let dims = numbers
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join("x");
                    let mut tile = Model::new(Group::Magic);
                    tile.sources = vec![source; numbers.len()];
                    tile.numbers = numbers.clone();
                    out.push((format!("{} {}", source_label(&source), dims), finish(tile)));
                }
            }
        }
        Group::Special => {
            for pair in products(MIN, BUDGET, 2, Parity::Both) {
                for &source in &sources {
                    let mut tile = Model::new(Group::Special);
                    tile.factor = pair[0];
                    tile.numbers = vec![pair[1]];
                    tile.sources = vec![source];
                    let name = format!("{} {}x{}", source_label(&source), pair[0], pair[1]);
                    out.push((name, finish(tile)));
                }
            }
        }
        Group::Mosaic => {
            let m = sources.len();
            for (i, pair) in products(MIN, BUDGET, 2, Parity::Both)
                .into_iter()
                .enumerate()
            {
                let mut tile = Model::new(Group::Mosaic);
                tile.factor = pair[0];
                tile.numbers = vec![pair[1]; 3];
                tile.sources = vec![sources[i % m], sources[(i + 1) % m], sources[(i + 2) % m]];
                let name = format!("{} {}x{}", source_label(&sources[i % m]), pair[0], pair[1]);
                out.push((name, finish(tile)));
            }
        }
    }
    out
}

fn thumb(tile: &Model, board: [u8; 4], fill: [u8; 4]) -> Json {
    match tile2d::build(tile) {
        Ok(cell) => {
            let colors = cell
                .types()
                .bytes()
                .iter()
                .map(|&v| if v != 0 { fill } else { board })
                .collect();
            frame::field(cell.width(), cell.height(), colors, board).fact()
        }
        Err(_) => frame::field(1, 1, vec![board], board).fact(),
    }
}

pub fn designs(req: &Json) -> Json {
    let group = Group::parse(req["group"].as_str().unwrap_or("")).unwrap_or(Group::Fractal);
    let catalog = if req["catalog"].as_str() == Some("Universe") {
        Catalog::Universe
    } else {
        Catalog::Classics
    };
    let dark = req["dark"].as_bool().unwrap_or(false);
    let all = catalogue(group, &catalog);
    let pages = all.len().div_ceil(PAGE).max(1);
    let page = (req["page"].as_u64().unwrap_or(0) as usize).min(pages - 1);
    let board = frame::board(dark);
    let fill = frame::ink(dark);
    let designs: Vec<Json> = all
        .iter()
        .skip(page * PAGE)
        .take(PAGE)
        .map(|(name, tile)| {
            json!({
                "name": name,
                "frame": thumb(tile, board, fill),
                "value": { "v": 1, "tile": tile.to_json(), "paint": Json::Null },
            })
        })
        .collect();
    json!({
        "config": {
            "group": group.name(),
            "catalog": if matches!(catalog, Catalog::Universe) { "Universe" } else { "Classics" },
            "page": page,
            "dark": dark,
        },
        "vocab": {
            "groups": Group::all().iter().map(|g| g.name()).collect::<Vec<_>>(),
            "catalogs": ["Classics", "Universe"],
            "count": all.len(),
            "pages": pages,
            "size": PAGE,
        },
        "designs": designs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_sets_cover_emoji_and_font() {
        let emoji = glyphs("emoji");
        let entries = emoji.as_array().unwrap();
        assert_eq!(entries.len(), crate::core::emoji::names().len());
        assert_eq!(entries[0]["name"], json!("smileys"));
        assert!(!entries[0]["glyphs"].as_array().unwrap().is_empty());
        let font = glyphs("font");
        assert_eq!(font[0]["name"], json!("font"));
        assert_eq!(font[0]["glyphs"].as_array().unwrap().len(), 108);
        assert_eq!(glyphs("nope"), json!([]));
    }
    #[test]
    fn designs_page_is_deterministic_and_buildable() {
        let req = json!({ "group": "Fractal", "catalog": "Classics", "page": 0 });
        let a = designs(&req);
        assert_eq!(a, designs(&req));
        assert_eq!(a["config"]["group"], json!("Fractal"));
        assert_eq!(a["vocab"]["groups"].as_array().unwrap().len(), 5);
        assert_eq!(a["vocab"]["catalogs"], json!(["Classics", "Universe"]));
        let list = a["designs"].as_array().unwrap();
        assert!(!list.is_empty() && list.len() <= PAGE);
        for design in list {
            assert!(design["name"].as_str().is_some());
            assert_eq!(design["value"]["v"], json!(1));
            assert_eq!(design["value"]["paint"], Json::Null);
            let tile = Model::from_json(&design["value"]["tile"]).unwrap();
            assert!(tile.max_size() <= BUDGET);
            assert!(tile2d::build(&tile).is_ok());
            assert!(design["frame"]["rows"].is_array());
            assert!(design["frame"]["width"].as_u64().unwrap() <= BUDGET as u64);
        }
    }
    #[test]
    fn every_group_and_catalog_yields_buildable_designs() {
        for group in Group::all() {
            for catalog in [Catalog::Classics, Catalog::Universe] {
                let all = catalogue(group, &catalog);
                assert!(!all.is_empty());
                for (name, tile) in &all {
                    assert!(!name.is_empty());
                    let cell = tile2d::build(tile).unwrap();
                    assert_eq!(cell.width(), tile.width);
                    assert!(tile.max_size() <= BUDGET);
                }
            }
        }
    }
    #[test]
    fn page_clamps_to_the_last() {
        let out = designs(&json!({ "group": "General", "page": 999 }));
        let pages = out["vocab"]["pages"].as_u64().unwrap();
        assert_eq!(out["config"]["page"], json!(pages - 1));
        assert!(!out["designs"].as_array().unwrap().is_empty());
    }
    #[test]
    fn unknown_request_falls_back_to_defaults() {
        let out = designs(&json!({ "group": "Sparkle", "catalog": "soup" }));
        assert_eq!(out["config"]["group"], json!("Fractal"));
        assert_eq!(out["config"]["catalog"], json!("Classics"));
        assert_eq!(out["config"]["page"], json!(0));
    }
}
