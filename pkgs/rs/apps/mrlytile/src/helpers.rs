use mrlycore::paint::{Edition, Ink, Paint};
use mrlycore::tile::{Catalog, Source, Tile as Model, CLASSICS_2D};
use mrlycore::{json, Json};
use mrlymath::bang::{bang, universe_codes};
use mrlymath::name::{classic_code, Bang, Named};

/// Names a source, either its classic design or its canonical bang name.
pub fn source_label(source: &Source) -> String {
    match source {
        Source::Classic(design) => design.name().to_string(),
        Source::Code(code) => Bang::new(*code, 2, 2).to_str(),
    }
}

/// Carries a source into a catalog by its code, falling back to the orbit lead or itself.
pub fn remap(source: Source, catalog: &Catalog) -> Source {
    match catalog {
        Catalog::Universe => {
            let code = match source {
                Source::Classic(design) => match classic_code(design) {
                    Some(code) => code,
                    None => return source,
                },
                Source::Code(code) => code,
            };
            if universe_codes(2).contains(&code) {
                Source::Code(code)
            } else if code < 16 {
                Source::Code(bang(2).design(code).class_rep)
            } else {
                source
            }
        }
        _ => match source {
            Source::Code(code) => CLASSICS_2D
                .into_iter()
                .find(|&design| classic_code(design) == Some(code))
                .map(Source::Classic)
                .unwrap_or(source),
            classic => classic,
        },
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
