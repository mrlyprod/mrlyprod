use mrlycore::errors::{value_error, Result};

/// The cost class of a measure along the level axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cost {
    /// A closed form: microseconds a term.
    Closed,
    /// A convolution over the diagonal profile: milliseconds a term.
    Convolved,
    /// A rendered grid of `n^(D L)` cells.
    Grid,
}

/// One integer reading of a design.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Measure {
    /// The filled cells.
    Fills,
    /// The empty cells.
    Voids,
    /// The exposed faces: the perimeter in the plane, the surface in space.
    Surface,
    /// The most filled cells on one diagonal plane.
    Peak,
    /// The diagonal planes holding a filled cell.
    Heights,
    /// The distinct corners the filled cells touch.
    Vertices,
    /// The distinct unit edges the filled cells carry.
    Edges,
    /// The distinct unit faces the filled cells carry.
    Faces,
    /// The Euler characteristic of the filled complex.
    Euler,
    /// The filled triangles of the diagonal slice of a cube.
    Triangles,
    /// The holes of the diagonal slice of a cube.
    Holes,
    /// The connected pieces of the diagonal slice of a cube.
    Pieces,
}

impl Measure {
    /// Every measure, in ledger order.
    pub const ALL: [Measure; 12] = [
        Measure::Fills,
        Measure::Voids,
        Measure::Surface,
        Measure::Peak,
        Measure::Heights,
        Measure::Vertices,
        Measure::Edges,
        Measure::Faces,
        Measure::Euler,
        Measure::Triangles,
        Measure::Holes,
        Measure::Pieces,
    ];
    /// Returns the measure's one-word name.
    pub fn slug(self) -> &'static str {
        match self {
            Measure::Fills => "fills",
            Measure::Voids => "voids",
            Measure::Surface => "surface",
            Measure::Peak => "peak",
            Measure::Heights => "heights",
            Measure::Vertices => "vertices",
            Measure::Edges => "edges",
            Measure::Faces => "faces",
            Measure::Euler => "euler",
            Measure::Triangles => "triangles",
            Measure::Holes => "holes",
            Measure::Pieces => "pieces",
        }
    }
    /// Parses a one-word name back into its measure, or an error for any other word.
    pub fn parse(slug: &str) -> Result<Measure> {
        Measure::ALL
            .into_iter()
            .find(|measure| measure.slug() == slug)
            .map_or_else(|| value_error(format!("unknown measure {slug:?}.")), Ok)
    }
    /// Returns the measure's cost class along the level axis.
    pub fn cost(self) -> Cost {
        match self {
            Measure::Fills | Measure::Voids | Measure::Surface => Cost::Closed,
            Measure::Peak | Measure::Heights => Cost::Convolved,
            _ => Cost::Grid,
        }
    }
    /// Returns whether the measure reads a design of the dimension and base.
    pub fn applies(self, dimension: usize, base: usize) -> bool {
        match self {
            Measure::Fills | Measure::Voids | Measure::Surface => dimension >= 1,
            Measure::Peak | Measure::Heights => dimension >= 2,
            Measure::Vertices | Measure::Edges | Measure::Euler => dimension == 2 || dimension == 3,
            Measure::Faces => dimension == 3,
            Measure::Triangles | Measure::Holes | Measure::Pieces => dimension == 3 && base == 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_slug_parses_back() {
        for measure in Measure::ALL {
            assert_eq!(Measure::parse(measure.slug()).unwrap(), measure);
        }
        assert!(Measure::parse("area").is_err());
    }
    #[test]
    fn the_plane_and_the_cube_read_their_measures() {
        let plane: Vec<Measure> = Measure::ALL
            .into_iter()
            .filter(|m| m.applies(2, 2))
            .collect();
        assert_eq!(plane.len(), 8);
        let cube: Vec<Measure> = Measure::ALL
            .into_iter()
            .filter(|m| m.applies(3, 2))
            .collect();
        assert_eq!(cube.len(), 12);
        assert_eq!(Measure::ALL.iter().filter(|m| m.applies(1, 5)).count(), 3);
        assert!(!Measure::Triangles.applies(3, 3));
        assert_eq!(Measure::Euler.cost(), Cost::Grid);
    }
}
