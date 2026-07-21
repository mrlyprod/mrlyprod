use crate::core::paint::{Edition, Ink, Paint};
use crate::core::tile::{Catalog, Source, Tile as Model};
use serde_json::{json, Value as Json};

pub fn source_label(source: &Source) -> String {
    match source {
        Source::Classic(design) => design.name().to_string(),
        Source::Code(code) => format!("mrly_{code:02}"),
    }
}

pub fn work(tile: &Model, paint: &Option<Paint>) -> Json {
    json!({
        "v": 1,
        "tile": tile.to_json(),
        "paint": paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
    })
}

pub fn int(value: &Json) -> usize {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse::<u64>().ok()))
        .unwrap_or(0) as usize
}

pub fn nearest(options: &[usize], value: usize) -> usize {
    *options
        .iter()
        .min_by_key(|&&n| (n.abs_diff(value), n))
        .unwrap()
}

pub fn closest_nesting(options: &[Vec<usize>], current: &[usize]) -> Vec<usize> {
    let same: Vec<&Vec<usize>> = options
        .iter()
        .filter(|o| o.len() == current.len())
        .collect();
    let pool: Vec<&Vec<usize>> = if same.is_empty() {
        options.iter().collect()
    } else {
        same
    };
    let cost = |option: &Vec<usize>| {
        let changed = option.iter().zip(current).filter(|(a, b)| a != b).count();
        (
            changed + option.len().abs_diff(current.len()),
            option.clone(),
        )
    };
    (*pool.iter().min_by_key(|o| cost(o)).unwrap()).clone()
}

pub fn default_paint() -> Paint {
    let mut coating = Paint::new(Edition::Simple);
    coating.secondary = vec![Ink::White];
    coating
}

pub fn catalog_name(catalog: &Catalog) -> &'static str {
    match catalog {
        Catalog::Universe => "Universe",
        _ => "Classics",
    }
}
