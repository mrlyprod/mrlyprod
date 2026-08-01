use super::universe::bang;
use mrlycore::tile::{classics, Catalog, Source};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

pub fn universe_codes(dimension: usize) -> &'static [u128] {
    static CACHE: OnceLock<Mutex<BTreeMap<usize, &'static [u128]>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut guard = cache.lock().unwrap();
    if let Some(codes) = guard.get(&dimension) {
        return codes;
    }
    let codes: Vec<u128> = bang(dimension)
        .canonical()
        .into_iter()
        .map(|design| design.i)
        .collect();
    let leaked: &'static [u128] = Box::leak(codes.into_boxed_slice());
    guard.insert(dimension, leaked);
    leaked
}

pub fn sources(catalog: &Catalog, dimension: usize) -> Vec<Source> {
    match catalog {
        Catalog::Classics => classics(dimension)
            .into_iter()
            .map(Source::Classic)
            .collect(),
        Catalog::Universe => universe_codes(dimension)
            .iter()
            .map(|&code| Source::Code(code))
            .collect(),
        Catalog::Codes(list) => list.iter().map(|&code| Source::Code(code)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::tile::{CLASSICS_2D, CLASSICS_3D};
    #[test]
    fn catalog_classics_are_named_designs() {
        assert_eq!(
            sources(&Catalog::Classics, 2),
            CLASSICS_2D
                .into_iter()
                .map(Source::Classic)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            sources(&Catalog::Classics, 3),
            CLASSICS_3D
                .into_iter()
                .map(Source::Classic)
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn catalog_universe_has_full_orbit_counts() {
        assert_eq!(sources(&Catalog::Universe, 2).len(), 6);
        assert_eq!(sources(&Catalog::Universe, 3).len(), 22);
    }
}
