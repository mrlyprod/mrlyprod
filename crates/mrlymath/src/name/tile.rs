use super::text;
use super::Named;
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::tile::{Design, Group, Source, Tile};

const CODES_2D: [(Design, u128); 5] = [
    (Design::Carpet, 7),
    (Design::Net, 14),
    (Design::Htree, 3),
    (Design::Vtree, 5),
    (Design::Void, 9),
];

const TOTAL_2D: u128 = 16;

/// Returns the plane's bang code of a classic design, or None for one outside the plane.
pub fn classic_code(design: Design) -> Option<u128> {
    CODES_2D
        .iter()
        .find(|&&(d, _)| d == design)
        .map(|&(_, code)| code)
}

fn code_of(source: Source) -> u128 {
    match source {
        Source::Classic(design) => {
            classic_code(design).expect("a 3d-only design has no code in the plane")
        }
        Source::Code(code) => code,
    }
}

fn word_of(group: Group) -> &'static str {
    match group {
        Group::General => "general",
        Group::Fractal => "fractal",
        Group::Magic => "magic",
        Group::Special => "special",
        Group::Mosaic => "mosaic",
    }
}

fn group_of(word: &str) -> Result<Group> {
    match word {
        "general" => Ok(Group::General),
        "fractal" => Ok(Group::Fractal),
        "magic" => Ok(Group::Magic),
        "special" => Ok(Group::Special),
        "mosaic" => Ok(Group::Mosaic),
        other => value_error(format!("unknown tile group {other:?}.")),
    }
}

fn slot_run(code: u128, number: Option<usize>, rotation: usize, anti: bool) -> String {
    let mut tags = vec![('c', Some(code))];
    if let Some(number) = number {
        tags.push(('n', Some(number as u128)));
    }
    if !rotation.is_multiple_of(4) {
        tags.push(('r', Some((rotation % 4) as u128)));
    }
    if anti {
        tags.push(('a', None));
    }
    text::run(&tags)
}

struct Take<'a> {
    fields: &'a [&'a str],
    at: usize,
}

impl<'a> Take<'a> {
    fn new(fields: &'a [&'a str]) -> Take<'a> {
        Take { fields, at: 0 }
    }
    fn peek(&self) -> Option<&'a str> {
        self.fields.get(self.at).copied()
    }
    fn next(&mut self) -> Option<&'a str> {
        let field = self.peek()?;
        self.at += 1;
        Some(field)
    }
    fn need(&mut self, tag: char) -> Result<u128> {
        match self.next().map(text::tags).transpose()?.as_deref() {
            Some([(t, Some(value))]) if *t == tag => Ok(*value),
            _ => value_error(format!("tile name misses its {tag:?} field.")),
        }
    }
    fn want(&mut self, tag: char) -> Result<Option<u128>> {
        let Some(field) = self.peek() else {
            return Ok(None);
        };
        if !field.starts_with(tag) || field.len() < 2 {
            return Ok(None);
        }
        Ok(Some(self.need(tag)?))
    }
    fn flag(&mut self, tag: char) -> bool {
        if self.peek() == Some(&tag.to_string()[..]) {
            self.at += 1;
            return true;
        }
        false
    }
    fn done(&self) -> Result<()> {
        match self.peek() {
            None => Ok(()),
            Some(extra) => value_error(format!("tile name holds a stray field {extra:?}.")),
        }
    }
}

fn code(value: u128) -> Result<u128> {
    if value >= TOTAL_2D {
        return value_error(format!("code {value} is not in the plane (0..15)."));
    }
    Ok(value)
}

fn rotation(value: Option<u128>) -> Result<usize> {
    match value {
        None => Ok(0),
        Some(0) => value_error("rotation 0 elides."),
        Some(r @ 1..=3) => Ok(r as usize),
        Some(_) => value_error("rotation is 0 to 3."),
    }
}

fn slot(take: &mut Take) -> Result<(u128, Option<usize>, usize, bool)> {
    let Some(field) = take.next() else {
        return value_error("tile name misses a slot field.");
    };
    let tags = text::tags(field)?;
    let mut rest = tags.as_slice();
    let [('c', Some(source)), tail @ ..] = rest else {
        return value_error(format!("slot field {field:?} does not open with a code."));
    };
    let source = code(*source)?;
    rest = tail;
    let number = if let [('n', Some(n)), tail @ ..] = rest {
        rest = tail;
        Some(text::small(*n)?)
    } else {
        None
    };
    let turn = if let [('r', Some(r)), tail @ ..] = rest {
        rest = tail;
        rotation(Some(*r))?
    } else {
        0
    };
    let anti = if let [('a', None), tail @ ..] = rest {
        rest = tail;
        true
    } else {
        false
    };
    if !rest.is_empty() {
        return value_error(format!("slot field {field:?} holds a stray tag."));
    }
    Ok((source, number, turn, anti))
}

fn finish(mut tile: Tile) -> Result<Tile> {
    tile.resize();
    tile.check()
        .map_err(|note| MrlyError::Value(format!("parsed tile fails its check: {note}.")))?;
    Ok(tile)
}

impl Named for Tile {
    fn to_str(&self) -> String {
        let mut fields = vec![word_of(self.group).to_string()];
        match self.group {
            Group::General | Group::Fractal => {
                fields.push(text::run(&[('c', Some(code_of(self.sources[0])))]));
                fields.push(text::run(&[('n', Some(self.numbers[0] as u128))]));
                if self.group == Group::Fractal && self.levels[0] != 1 {
                    fields.push(text::run(&[('l', Some(self.levels[0] as u128))]));
                }
                if !self.rotations[0].is_multiple_of(4) {
                    fields.push(text::run(&[('r', Some((self.rotations[0] % 4) as u128))]));
                }
                if self.anti[0] ^ self.invert {
                    fields.push("i".to_string());
                }
            }
            Group::Magic => {
                for i in 0..self.sources.len() {
                    fields.push(slot_run(
                        code_of(self.sources[i]),
                        Some(self.numbers[i]),
                        self.rotations[i],
                        self.anti[i],
                    ));
                }
                if self.invert {
                    fields.push("i".to_string());
                }
            }
            Group::Special => {
                fields.push(text::run(&[('c', Some(code_of(self.sources[0])))]));
                fields.push(text::run(&[('f', Some(self.factor as u128))]));
                fields.push(text::run(&[('n', Some(self.numbers[0] as u128))]));
                if !self.rotations[0].is_multiple_of(4) {
                    fields.push(text::run(&[('r', Some((self.rotations[0] % 4) as u128))]));
                }
                if self.flip {
                    fields.push("x".to_string());
                }
                if self.invert {
                    fields.push("i".to_string());
                }
            }
            Group::Mosaic => {
                fields.push(text::run(&[('f', Some(self.factor as u128))]));
                fields.push(text::run(&[('n', Some(self.numbers[0] as u128))]));
                for i in 0..self.sources.len() {
                    fields.push(slot_run(
                        code_of(self.sources[i]),
                        None,
                        self.rotations[i],
                        self.anti[i],
                    ));
                }
                if self.invert {
                    fields.push("i".to_string());
                }
            }
        }
        text::compose("tile", &fields)
    }
    fn from_str(text: &str) -> Result<Tile> {
        let fields = text::split(text, "tile")?;
        let group = group_of(fields[0])?;
        let mut take = Take::new(&fields[1..]);
        let mut tile = Tile::new(group);
        match group {
            Group::General | Group::Fractal => {
                let source = code(take.need('c')?)?;
                let number = text::small(take.need('n')?)?;
                let level = match group {
                    Group::Fractal => match take.want('l')? {
                        Some(1) => return value_error("level 1 elides."),
                        Some(level) => text::small(level)?,
                        None => 1,
                    },
                    _ => 1,
                };
                let turn = rotation(take.want('r')?)?;
                let invert = take.flag('i');
                take.done()?;
                tile.sources = vec![Source::Code(source)];
                tile.numbers = vec![number];
                tile.levels = vec![level];
                tile.rotations = vec![turn];
                tile.anti = vec![false];
                tile.invert = invert;
            }
            Group::Magic => {
                let mut sources = Vec::new();
                let mut numbers = Vec::new();
                let mut rotations = Vec::new();
                let mut anti = Vec::new();
                while take.peek().is_some_and(|field| field.starts_with('c')) {
                    let (source, number, turn, flag) = slot(&mut take)?;
                    let Some(number) = number else {
                        return value_error("magic slot misses its number.");
                    };
                    sources.push(Source::Code(source));
                    numbers.push(number);
                    rotations.push(turn);
                    anti.push(flag);
                }
                let invert = take.flag('i');
                take.done()?;
                let count = sources.len();
                tile.sources = sources;
                tile.numbers = numbers;
                tile.levels = vec![1; count];
                tile.rotations = rotations;
                tile.anti = anti;
                tile.invert = invert;
            }
            Group::Special => {
                let source = code(take.need('c')?)?;
                let factor = text::small(take.need('f')?)?;
                let number = text::small(take.need('n')?)?;
                let turn = rotation(take.want('r')?)?;
                let flip = take.flag('x');
                let invert = take.flag('i');
                take.done()?;
                tile.sources = vec![Source::Code(source)];
                tile.factor = factor;
                tile.numbers = vec![number];
                tile.levels = vec![1];
                tile.rotations = vec![turn];
                tile.anti = vec![false];
                tile.flip = flip;
                tile.invert = invert;
            }
            Group::Mosaic => {
                let factor = text::small(take.need('f')?)?;
                let number = text::small(take.need('n')?)?;
                let mut sources = Vec::new();
                let mut rotations = Vec::new();
                let mut anti = Vec::new();
                while take.peek().is_some_and(|field| field.starts_with('c')) {
                    let (source, extra, turn, flag) = slot(&mut take)?;
                    if extra.is_some() {
                        return value_error("mosaic slots share the one number.");
                    }
                    sources.push(Source::Code(source));
                    rotations.push(turn);
                    anti.push(flag);
                }
                let invert = take.flag('i');
                take.done()?;
                let count = sources.len();
                tile.sources = sources;
                tile.factor = factor;
                tile.numbers = vec![number; count];
                tile.levels = vec![1; count];
                tile.rotations = rotations;
                tile.anti = anti;
                tile.invert = invert;
            }
        }
        finish(tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    use crate::two::tile as tile2d;
    use mrlycore::state::{guard, seed};
    use mrlycore::tile::{Catalog, Parity};

    fn built(tile: &Tile) -> crate::two::Cell2d {
        tile2d::build(tile).unwrap()
    }
    fn fractal(design: Design) -> Tile {
        let mut tile = Tile::new(Group::Fractal);
        tile.sources = vec![Source::Classic(design)];
        tile.numbers = vec![3];
        tile.levels = vec![2];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.resize();
        tile
    }

    #[test]
    fn classic_codes_match_their_renders() {
        for (design, code) in CODES_2D {
            let by_name = designs::create(code, 3, 1, 0, 2).unwrap();
            let by_classic = match design {
                Design::Carpet => designs::carpet(3, 1).unwrap(),
                Design::Net => designs::net(3, 1).unwrap(),
                Design::Htree => designs::htree(3, 1).unwrap(),
                Design::Vtree => designs::vtree(3, 1).unwrap(),
                Design::Void => designs::void(3, 1).unwrap(),
                _ => unreachable!(),
            };
            assert_eq!(by_name, by_classic, "{}", design.name());
        }
    }
    #[test]
    fn example_names_hold_verbatim() {
        let carpet = fractal(Design::Carpet);
        assert_eq!(carpet.to_str(), "mrly_tile_fractal_c7_n3_l2");
        let mut general = Tile::new(Group::General);
        general.sources = vec![Source::Code(3)];
        general.numbers = vec![5];
        general.levels = vec![1];
        general.rotations = vec![1];
        general.anti = vec![false];
        general.invert = true;
        general.resize();
        assert_eq!(general.to_str(), "mrly_tile_general_c3_n5_r1_i");
        let mut magic = Tile::new(Group::Magic);
        magic.sources = vec![Source::Classic(Design::Carpet), Source::Code(14)];
        magic.numbers = vec![3, 5];
        magic.levels = vec![1, 1];
        magic.rotations = vec![0, 2];
        magic.anti = vec![false, true];
        magic.invert = true;
        magic.resize();
        assert_eq!(magic.to_str(), "mrly_tile_magic_c7n3_c14n5r2a_i");
        let mut special = Tile::new(Group::Special);
        special.sources = vec![Source::Classic(Design::Vtree)];
        special.factor = 3;
        special.numbers = vec![5];
        special.levels = vec![1];
        special.rotations = vec![0];
        special.anti = vec![true];
        special.flip = true;
        special.resize();
        assert_eq!(special.to_str(), "mrly_tile_special_c5_f3_n5_x");
        let mut mosaic = Tile::new(Group::Mosaic);
        mosaic.sources = vec![Source::Code(7), Source::Code(14), Source::Code(5)];
        mosaic.factor = 3;
        mosaic.numbers = vec![3, 3, 3];
        mosaic.levels = vec![1, 1, 1];
        mosaic.rotations = vec![0, 1, 0];
        mosaic.anti = vec![false, false, true];
        mosaic.invert = true;
        mosaic.resize();
        assert_eq!(mosaic.to_str(), "mrly_tile_mosaic_f3_n3_c7_c14r1_c5a_i");
    }
    #[test]
    fn parsed_tiles_pass_check_and_build() {
        for name in [
            "mrly_tile_fractal_c7_n3_l2",
            "mrly_tile_general_c3_n5_r1_i",
            "mrly_tile_magic_c7n3_c14n5r2a_i",
            "mrly_tile_special_c5_f3_n5_x",
            "mrly_tile_mosaic_f3_n3_c7_c14r1_c5a_i",
        ] {
            let tile = Tile::from_str(name).unwrap();
            assert_eq!(tile.check(), Ok(()));
            assert_eq!(tile.to_str(), name);
            let cell = built(&tile);
            assert_eq!(cell.width(), tile.width);
        }
    }
    #[test]
    fn anti_invert_pairs_share_one_name_and_one_picture() {
        let mut plain = fractal(Design::Carpet);
        let mut folded = plain.clone();
        folded.anti = vec![true];
        folded.invert = true;
        assert_eq!(plain.to_str(), folded.to_str());
        assert_eq!(built(&plain), built(&folded));
        plain.invert = true;
        let mut alias = plain.clone();
        alias.anti = vec![true];
        alias.invert = false;
        assert_eq!(plain.to_str(), alias.to_str());
        assert_eq!(built(&plain), built(&alias));
        assert_eq!(plain.to_str(), "mrly_tile_fractal_c7_n3_l2_i");
    }
    #[test]
    fn classics_and_codes_share_one_name() {
        let by_classic = fractal(Design::Net);
        let mut by_code = by_classic.clone();
        by_code.sources = vec![Source::Code(14)];
        assert_eq!(by_classic.to_str(), by_code.to_str());
        assert_eq!(built(&by_classic), built(&by_code));
    }
    #[test]
    fn dead_special_anti_folds_away() {
        let mut special = Tile::new(Group::Special);
        special.sources = vec![Source::Code(5)];
        special.factor = 3;
        special.numbers = vec![3];
        special.levels = vec![1];
        special.rotations = vec![0];
        special.anti = vec![true];
        special.resize();
        let name = special.to_str();
        let parsed = Tile::from_str(&name).unwrap();
        assert_eq!(parsed.anti, vec![false]);
        assert_eq!(built(&special), built(&parsed));
    }
    #[test]
    fn only_the_canonical_form_parses() {
        for bad in [
            "mrly_tile_fractal_c7_n3_l1",
            "mrly_tile_general_c7_n3_r0",
            "mrly_tile_general_c7_n3_r4",
            "mrly_tile_general_c16_n3",
            "mrly_tile_general_c7_n03",
            "mrly_tile_general_n3_c7",
            "mrly_tile_general_c7_n3_i_i",
            "mrly_tile_sparkle_c7_n3",
            "mrly_tile_magic_c7n3_i",
            "mrly_tile_mosaic_f3_n3_c7n3_c14_c5",
            "mrly_tile_special_c5_f3_n5_i_x",
            "mrly_tile_general_c7_n99",
        ] {
            assert!(Tile::from_str(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn seeded_tiles_round_trip() {
        let _guard = guard();
        let config = tile2d::Config {
            catalog: Catalog::Universe,
            min_size: 2,
            max_size: 64,
            parity: Parity::Both,
            ..tile2d::Config::default()
        };
        for s in 0..300 {
            seed(s);
            let tile = tile2d::create(&config).unwrap();
            let name = tile.to_str();
            let parsed = Tile::from_str(&name).unwrap();
            assert_eq!(parsed.to_str(), name, "seed {s}");
            assert_eq!(parsed.check(), Ok(()), "seed {s}");
            assert_eq!(built(&parsed), built(&tile), "seed {s}");
        }
    }
    #[test]
    fn seeded_classic_tiles_round_trip() {
        let _guard = guard();
        let config = tile2d::Config {
            min_size: 2,
            max_size: 64,
            parity: Parity::Both,
            ..tile2d::Config::default()
        };
        for s in 0..300 {
            seed(s);
            let tile = tile2d::create(&config).unwrap();
            let name = tile.to_str();
            let parsed = Tile::from_str(&name).unwrap();
            assert_eq!(parsed.to_str(), name, "seed {s}");
            assert_eq!(built(&parsed), built(&tile), "seed {s}");
        }
    }
}
