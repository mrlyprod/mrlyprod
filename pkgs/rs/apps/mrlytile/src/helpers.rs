use mrlycore::paint::{Edition, Ink, Paint};
use mrlycore::tile::{Catalog, Source, Tile as Model};
use mrlycore::{json, Json};

/// Names a source, either its classic design or its base-2 catalog code.
pub fn source_label(source: &Source) -> String {
    match source {
        Source::Classic(design) => design.name().to_string(),
        Source::Code(code) => format!("mrly_{code:02}"),
    }
}

/// Bundles a tile with its paint into the versioned value the library compares by.
pub fn work(tile: &Model, paint: &Option<Paint>) -> Json {
    json!({
        "v": 1,
        "tile": tile.to_json(),
        "paint": paint.as_ref().map(|p| p.to_json()).unwrap_or(Json::Null),
    })
}

/// Reads a whole number from a number or a numeric string, and zero from anything else.
pub fn int(value: &Json) -> usize {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse::<u64>().ok()))
        .unwrap_or(0) as usize
}

/// Picks the option lying closest to a value, taking the smaller one on a tie.
pub fn nearest(options: &[usize], value: usize) -> usize {
    *options
        .iter()
        .min_by_key(|&&n| (n.abs_diff(value), n))
        .unwrap()
}

/// Picks the nesting closest to the current one, preferring one of the same length.
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

/// Builds the simple coat, white as its second ink, that a bare tile is first given.
pub fn default_paint() -> Paint {
    let mut coating = Paint::new(Edition::Simple);
    coating.secondary = vec![Ink::White];
    coating
}

/// Names a catalog the way state and saves spell it.
pub fn catalog_name(catalog: &Catalog) -> &'static str {
    match catalog {
        Catalog::Universe => "Universe",
        _ => "Classics",
    }
}
