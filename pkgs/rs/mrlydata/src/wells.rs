use crate::trails;
use mrlycore::data::Well;

/// Gathers every well: the math wells, the font wells, then the goose trails.
pub fn wells() -> Vec<Box<dyn Well>> {
    let mut out = mrlymath::data::wells();
    out.extend(mrlyfont::data::wells());
    out.extend(trails::wells());
    out
}

/// Finds a well by name, or None for a stranger.
pub fn find(name: &str) -> Option<Box<dyn Well>> {
    wells().into_iter().find(|well| well.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::guard;
    use mrlycore::Json;

    #[test]
    fn well_names_are_unique() {
        let mut names: Vec<String> = wells().iter().map(|w| w.name().to_string()).collect();
        let count = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), count);
    }

    #[test]
    fn three_crates_pour_into_the_press() {
        let names: Vec<String> = wells().iter().map(|w| w.name().to_string()).collect();
        assert!(names.contains(&"tiles".to_string()));
        assert!(names.contains(&"glyphs".to_string()));
        assert!(names.iter().any(|name| name.starts_with("trail_")));
    }

    #[test]
    fn every_well_pours_identical_bytes_twice() {
        let _g = guard();
        for well in wells() {
            let a: Vec<String> = well.pour(7, 2).iter().map(Json::to_string).collect();
            let b: Vec<String> = well.pour(7, 2).iter().map(Json::to_string).collect();
            assert_eq!(a, b, "{} drifted between pours", well.name());
            assert!(!a.is_empty(), "{} poured nothing", well.name());
        }
    }

    #[test]
    fn every_well_obeys_the_prefix_law() {
        let _g = guard();
        for well in wells() {
            let short = well.pour(11, 1);
            let long = well.pour(11, 3);
            assert_eq!(short[..], long[..1], "{} broke the prefix law", well.name());
        }
    }

    #[test]
    fn find_answers_by_name() {
        assert!(find("glyphs").is_some());
        assert!(find("nothing").is_none());
    }
}
