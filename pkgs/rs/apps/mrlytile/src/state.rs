use super::helpers::{closest_nesting, default_paint, int, nearest, remap, source_label, work};
use super::render::{blank, cells, two_tone};
use super::{Tile, BUDGETS, MIN, THUMBS};
use mrlycore::colors::ink;
use mrlycore::json::Map;
use mrlycore::paint::{self, Edition, Ink, Paint, Scheme, Target};
use mrlycore::tile::{
    generals, nestings, powers, products, Catalog, Group, Parity, Source, Tile as Model,
};
use mrlycore::{json, Json};
use mrlymath::bang;
use mrlymath::two::tile as tile2d;

const BLANK_CODE: u128 = 0;
const SOLID_CODE: u128 = 15;

fn vocabulary(list: &Json) -> Option<String> {
    let items: Vec<String> = list
        .as_array()?
        .iter()
        .filter_map(|item| match item {
            Json::Str(word) => Some(word.clone()),
            other => Some(other.as_i64()?.to_string()),
        })
        .collect();
    match items.as_slice() {
        [] => None,
        [only] => Some(match only.parse::<i64>() {
            Ok(n) => format!("int {n}..{n}"),
            Err(_) => format!("{only} | {only}"),
        }),
        many => Some(many.join(" | ")),
    }
}

impl Tile {
    /// Lists the sources the staged catalog offers, the blank and solid codes left out.
    pub fn sources(&self) -> Vec<Source> {
        bang::sources(&self.catalog, 2)
            .into_iter()
            .filter(|s| !matches!(s, Source::Code(BLANK_CODE) | Source::Code(SOLID_CODE)))
            .collect()
    }
    /// Lists the sizes a general tile may take under the budget and parity.
    pub fn generals_of(&self) -> Vec<usize> {
        generals(MIN, self.budget, self.parity)
    }
    /// Lists the number and level pairs a fractal can be built from.
    pub fn powers_of(&self) -> Vec<(usize, usize)> {
        powers(MIN, self.budget, self.parity)
    }
    /// Lists the number runs a magic tile can nest.
    pub fn nestings_of(&self) -> Vec<Vec<usize>> {
        nestings(MIN, self.budget, self.parity)
    }
    /// Lists the factor and number pairs a special or mosaic tile can use.
    pub fn pairs_of(&self) -> Vec<Vec<usize>> {
        products(MIN, self.budget, 2, self.parity)
    }
    /// Lists the fractal levels one number can reach inside the budget.
    pub fn levels_of(&self, n: usize) -> Vec<usize> {
        self.powers_of()
            .iter()
            .filter(|&&(m, _)| m == n)
            .map(|&(_, level)| level)
            .collect()
    }
    /// Lists the groups with at least one legal tile under the staged budget and parity.
    pub fn feasible(&self) -> Vec<Group> {
        Group::all()
            .into_iter()
            .filter(|group| match group {
                Group::General => !self.generals_of().is_empty(),
                Group::Fractal => !self.powers_of().is_empty(),
                Group::Magic => !self.nestings_of().is_empty(),
                Group::Special => !self.pairs_of().is_empty(),
                Group::Mosaic => !self.pairs_of().is_empty() && self.sources().len() >= 3,
            })
            .collect()
    }
    /// Draws a tile unpainted in the current ink, blank if it will not build.
    pub fn preview(&self, model: &Model) -> Json {
        match tile2d::build(model) {
            Ok(cell) => cells(cell.width(), cell.height(), two_tone(&cell, ink(self.dark))),
            Err(_) => blank(),
        }
    }
    /// Draws one thumbnail per level the staged fractal can reach, and none for other groups.
    pub fn thumbs(&self) -> Vec<Json> {
        if self.tile.group != Group::Fractal {
            return Vec::new();
        }
        let levels = self.levels_of(self.tile.numbers[0]);
        if levels.is_empty() || levels.len() > THUMBS {
            return Vec::new();
        }
        levels
            .iter()
            .map(|&level| {
                let mut probe = self.tile.clone();
                probe.levels = vec![level];
                probe.resize();
                json!({ "level": level, "cells": self.preview(&probe) })
            })
            .collect()
    }
    /// Draws every saved tile with its name, its bundled value and a preview.
    pub fn shelf(&self) -> Vec<Json> {
        self.library
            .iter()
            .map(|entry| {
                json!({
                    "id": entry.id,
                    "name": &entry.name,
                    "value": work(&entry.tile, &entry.paint),
                    "cells": self.preview(&entry.tile),
                })
            })
            .collect()
    }
    /// Lists the numbers each slot may take for a magic tile, and the one shared list otherwise.
    pub fn numbers_options(&self) -> Vec<Vec<usize>> {
        match self.tile.group {
            Group::General => vec![self.generals_of()],
            Group::Fractal => {
                let mut ns: Vec<usize> = self.powers_of().iter().map(|&(n, _)| n).collect();
                ns.dedup();
                vec![ns]
            }
            Group::Magic => {
                let count = self.tile.numbers.len();
                let options = self.nestings_of();
                (0..count)
                    .map(|i| {
                        let mut ns: Vec<usize> = options
                            .iter()
                            .filter(|o| o.len() == count)
                            .map(|o| o[i])
                            .collect();
                        ns.sort_unstable();
                        ns.dedup();
                        ns
                    })
                    .collect()
            }
            Group::Special | Group::Mosaic => {
                let factor = self.tile.factor;
                let mut ns: Vec<usize> = self
                    .pairs_of()
                    .iter()
                    .filter(|pair| pair[0] == factor)
                    .map(|pair| pair[1])
                    .collect();
                ns.sort_unstable();
                ns.dedup();
                vec![ns]
            }
        }
    }
    /// Lists every choice the studio's knobs currently offer.
    pub fn options(&self) -> Json {
        let groups: Vec<&str> = self.feasible().iter().map(|g| g.name()).collect();
        let sources: Vec<Json> = self
            .sources()
            .iter()
            .map(|s| {
                let label = source_label(s);
                json!({ "label": &label, "value": label })
            })
            .collect();
        let levels = match self.tile.group {
            Group::Fractal => self.levels_of(self.tile.numbers[0]),
            _ => Vec::new(),
        };
        let counts = match self.tile.group {
            Group::Magic => {
                let mut lengths: Vec<usize> = self.nestings_of().iter().map(|o| o.len()).collect();
                lengths.sort_unstable();
                lengths.dedup();
                lengths
            }
            _ => Vec::new(),
        };
        let factors = match self.tile.group {
            Group::Special | Group::Mosaic => {
                let mut fs: Vec<usize> = self.pairs_of().iter().map(|pair| pair[0]).collect();
                fs.sort_unstable();
                fs.dedup();
                fs
            }
            _ => Vec::new(),
        };
        json!({
            "groups": groups,
            "catalogs": ["Classics", "Universe"],
            "parities": ["Evens", "Odds", "Both"],
            "budgets": BUDGETS.to_vec(),
            "editions": Edition::all().iter().map(|e| e.name()).collect::<Vec<_>>(),
            "schemes": ["Multicolor", "Multitone"],
            "targets": ["Fill", "Void"],
            "primaries": Ink::all().iter().map(|i| i.name()).collect::<Vec<_>>(),
            "sources": sources,
            "rotations": [0, 1, 2, 3],
            "numbers": self.numbers_options(),
            "levels": levels,
            "counts": counts,
            "factors": factors,
        })
    }
    /// Advertises one hint per knob the staged tile can turn, dropping any the grammar cannot spell.
    pub fn menu(&self) -> Json {
        let options = self.options();
        let mut numbers: Vec<usize> = self.numbers_options().concat();
        numbers.sort_unstable();
        numbers.dedup();
        let sources: Vec<String> = self.sources().iter().map(source_label).collect();
        let knobs = [
            ("group", vocabulary(&options["groups"])),
            ("catalog", vocabulary(&options["catalogs"])),
            ("parity", vocabulary(&options["parities"])),
            ("budget", vocabulary(&options["budgets"])),
            ("source", vocabulary(&json!(sources))),
            ("number", vocabulary(&json!(numbers))),
            ("level", vocabulary(&options["levels"])),
            ("count", vocabulary(&options["counts"])),
            ("factor", vocabulary(&options["factors"])),
            ("rotation", vocabulary(&options["rotations"])),
            (
                "anti",
                (self.tile.group != Group::Special).then(|| "bool".to_string()),
            ),
            ("invert", Some("bool".to_string())),
            (
                "flip",
                (self.tile.group == Group::Special).then(|| "bool".to_string()),
            ),
            ("edition", vocabulary(&options["editions"])),
            ("scheme", vocabulary(&options["schemes"])),
            ("target", vocabulary(&options["targets"])),
            ("primary", vocabulary(&options["primaries"])),
        ];
        let mut menu = Map::new();
        for (key, hint) in knobs {
            if let Some(hint) = hint {
                menu.insert(key.to_string(), json!(hint));
            }
        }
        Json::Obj(menu)
    }
    /// Stages the first legal tile of a group, carrying the leading source and rotation, invert, and flip while the group stays special.
    pub fn rebuild(&mut self, group: Group) -> Result<(), &'static str> {
        let sources = self.sources();
        let first = sources[0];
        let lead = self
            .tile
            .sources
            .first()
            .copied()
            .filter(|s| sources.contains(s))
            .unwrap_or(first);
        let turn = self.tile.rotations.first().copied().unwrap_or(0);
        let invert = self.tile.invert;
        let flip = self.tile.flip && group == Group::Special;
        let mut tile = Model::new(group);
        match group {
            Group::General => {
                let numbers = self.generals_of();
                if numbers.is_empty() {
                    return Err("no legal option");
                }
                tile.numbers = vec![numbers[0]];
                tile.sources = vec![lead];
            }
            Group::Fractal => {
                let options = self.powers_of();
                if options.is_empty() {
                    return Err("no legal option");
                }
                let (n, level) = options[0];
                tile.numbers = vec![n];
                tile.levels = vec![level];
                tile.sources = vec![lead];
            }
            Group::Magic => {
                let options = self.nestings_of();
                if options.is_empty() {
                    return Err("no legal option");
                }
                let numbers = options[0].clone();
                let count = numbers.len();
                tile.numbers = numbers;
                tile.sources = vec![lead];
                tile.sources.extend(vec![first; count - 1]);
            }
            Group::Special | Group::Mosaic => {
                let pairs = self.pairs_of();
                if pairs.is_empty() || (group == Group::Mosaic && sources.len() < 3) {
                    return Err("no legal option");
                }
                let (factor, n) = (pairs[0][0], pairs[0][1]);
                tile.factor = factor;
                if group == Group::Mosaic {
                    tile.numbers = vec![n; 3];
                    tile.sources = vec![
                        lead,
                        sources.get(1).copied().unwrap_or(first),
                        sources.get(2).copied().unwrap_or(first),
                    ];
                } else {
                    tile.numbers = vec![n];
                    tile.sources = vec![lead];
                }
            }
        }
        let slots = tile.sources.len();
        if tile.levels.len() != slots {
            tile.levels = vec![1; slots];
        }
        tile.rotations = vec![0; slots];
        tile.rotations[0] = turn;
        tile.anti = vec![false; slots];
        tile.invert = invert;
        tile.flip = flip;
        tile.resize();
        self.tile = tile;
        Ok(())
    }
    /// Nudges the staged tile to the nearest legal combination, falling back to a general one.
    pub fn snap(&mut self) {
        if !self.feasible().contains(&self.tile.group) {
            let _ = self.rebuild(Group::General);
            return;
        }
        match self.tile.group {
            Group::General => {
                let options = self.generals_of();
                self.tile.numbers[0] = nearest(&options, self.tile.numbers[0]);
            }
            Group::Fractal => {
                let mut ns: Vec<usize> = self.powers_of().iter().map(|&(n, _)| n).collect();
                ns.dedup();
                let n = nearest(&ns, self.tile.numbers[0]);
                let levels = self.levels_of(n);
                let level = self.tile.levels[0];
                self.tile.numbers[0] = n;
                self.tile.levels[0] = if levels.contains(&level) {
                    level
                } else {
                    *levels
                        .iter()
                        .rev()
                        .find(|&&l| l <= level)
                        .unwrap_or(&levels[0])
                };
            }
            Group::Magic => {
                let options = self.nestings_of();
                let numbers = closest_nesting(&options, &self.tile.numbers);
                self.resize_slots(numbers);
            }
            Group::Special | Group::Mosaic => {
                let pairs = self.pairs_of();
                let (factor, n) = (self.tile.factor, self.tile.numbers[0]);
                let pick = pairs
                    .iter()
                    .min_by_key(|pair| {
                        (
                            pair[0].abs_diff(factor) + pair[1].abs_diff(n),
                            pair[0],
                            pair[1],
                        )
                    })
                    .unwrap()
                    .clone();
                self.tile.factor = pick[0];
                for number in self.tile.numbers.iter_mut() {
                    *number = pick[1];
                }
            }
        }
        self.tile.resize();
    }
    /// Restages the tile on a new run of numbers, padding or trimming its slots to match.
    pub fn resize_slots(&mut self, numbers: Vec<usize>) {
        let count = numbers.len();
        let filler = self.tile.sources[0];
        self.tile.numbers = numbers;
        self.tile.sources.resize(count, filler);
        self.tile.levels.resize(count, 1);
        self.tile.rotations.resize(count, 0);
        self.tile.anti.resize(count, false);
        self.tile.resize();
    }
    /// Reads a slot index, refusing one the staged tile does not have.
    pub fn slot(&self, call_slot: &Json) -> Result<usize, &'static str> {
        let slot = call_slot.as_u64().unwrap_or(0) as usize;
        if slot >= self.tile.sources.len() {
            return Err("no such slot");
        }
        Ok(slot)
    }
    /// Returns the staged paint, starting a default coat if the tile is still bare.
    pub fn coating(&mut self) -> &mut Paint {
        if self.paint.is_none() {
            self.paint = Some(default_paint());
        }
        self.paint.as_mut().unwrap()
    }
    /// Turns one knob, refusing any value that is not legal for the staged tile.
    pub fn apply(&mut self, key: &str, value: &Json, slot: &Json) -> Result<Json, &'static str> {
        match key {
            "group" => {
                let group =
                    Group::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                if !self.feasible().contains(&group) {
                    return Err("no such option");
                }
                self.rebuild(group)?;
                Ok(json!(group.name()))
            }
            "catalog" => {
                let next = match value.as_str().unwrap_or("") {
                    "Classics" => Catalog::Classics,
                    "Universe" => Catalog::Universe,
                    _ => return Err("no such option"),
                };
                self.tile.sources = self.tile.sources.iter().map(|&s| remap(s, &next)).collect();
                self.catalog = next;
                self.snap();
                if self.tile.check().is_err() {
                    let _ = self.rebuild(self.tile.group);
                }
                Ok(value.clone())
            }
            "parity" => {
                self.parity =
                    Parity::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                self.snap();
                Ok(value.clone())
            }
            "budget" => {
                let budget = int(value);
                if !BUDGETS.contains(&budget) {
                    return Err("no such option");
                }
                self.budget = budget;
                self.snap();
                Ok(json!(budget))
            }
            "source" => {
                let slot = self.slot(slot)?;
                let label = value.as_str().ok_or("value must be a string")?;
                let source = self
                    .sources()
                    .into_iter()
                    .find(|s| source_label(s) == label)
                    .ok_or("no such option")?;
                self.tile.sources[slot] = source;
                Ok(value.clone())
            }
            "number" => {
                let n = int(value);
                match self.tile.group {
                    Group::General => {
                        if !self.generals_of().contains(&n) {
                            return Err("no such option");
                        }
                        self.tile.numbers[0] = n;
                    }
                    Group::Fractal => {
                        let levels = self.levels_of(n);
                        if levels.is_empty() {
                            return Err("no such option");
                        }
                        let level = self.tile.levels[0];
                        self.tile.numbers[0] = n;
                        self.tile.levels[0] = if levels.contains(&level) {
                            level
                        } else {
                            *levels
                                .iter()
                                .rev()
                                .find(|&&l| l <= level)
                                .unwrap_or(&levels[0])
                        };
                    }
                    Group::Magic => {
                        let slot = self.slot(slot)?;
                        let count = self.tile.numbers.len();
                        let options: Vec<Vec<usize>> = self
                            .nestings_of()
                            .into_iter()
                            .filter(|o| o.len() == count && o[slot] == n)
                            .collect();
                        if options.is_empty() {
                            return Err("no such option");
                        }
                        self.tile.numbers = closest_nesting(&options, &self.tile.numbers);
                    }
                    Group::Special | Group::Mosaic => {
                        let factor = self.tile.factor;
                        if !self.pairs_of().iter().any(|p| p[0] == factor && p[1] == n) {
                            return Err("no such option");
                        }
                        for number in self.tile.numbers.iter_mut() {
                            *number = n;
                        }
                    }
                }
                self.tile.resize();
                Ok(json!(n))
            }
            "level" => {
                if self.tile.group != Group::Fractal {
                    return Err("level is fractal only");
                }
                let level = int(value);
                if !self.levels_of(self.tile.numbers[0]).contains(&level) {
                    return Err("no such option");
                }
                self.tile.levels[0] = level;
                self.tile.resize();
                Ok(json!(level))
            }
            "count" => {
                if self.tile.group != Group::Magic {
                    return Err("count is magic only");
                }
                let count = int(value);
                let options: Vec<Vec<usize>> = self
                    .nestings_of()
                    .into_iter()
                    .filter(|o| o.len() == count)
                    .collect();
                if options.is_empty() {
                    return Err("no such option");
                }
                let numbers = closest_nesting(&options, &self.tile.numbers);
                self.resize_slots(numbers);
                Ok(json!(count))
            }
            "factor" => {
                if !matches!(self.tile.group, Group::Special | Group::Mosaic) {
                    return Err("factor is special or mosaic only");
                }
                let factor = int(value);
                let numbers: Vec<usize> = self
                    .pairs_of()
                    .iter()
                    .filter(|p| p[0] == factor)
                    .map(|p| p[1])
                    .collect();
                if numbers.is_empty() {
                    return Err("no such option");
                }
                let n = nearest(&numbers, self.tile.numbers[0]);
                self.tile.factor = factor;
                for number in self.tile.numbers.iter_mut() {
                    *number = n;
                }
                self.tile.resize();
                Ok(json!(factor))
            }
            "rotation" => {
                let slot = self.slot(slot)?;
                let rotation = value
                    .as_u64()
                    .or_else(|| value.as_str().and_then(|s| s.parse::<u64>().ok()))
                    .ok_or("value must be a number")? as usize;
                if rotation > 3 {
                    return Err("rotation is 0 to 3");
                }
                self.tile.rotations[slot] = rotation;
                Ok(json!(rotation))
            }
            "anti" => {
                if self.tile.group == Group::Special {
                    return Err("anti does nothing for a special tile");
                }
                let slot = self.slot(slot)?;
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.tile.anti[slot] = on;
                Ok(json!(on))
            }
            "invert" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.tile.invert = on;
                Ok(json!(on))
            }
            "flip" => {
                if self.tile.group != Group::Special {
                    return Err("flip is special only");
                }
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.tile.flip = on;
                Ok(json!(on))
            }
            "edition" => {
                let edition =
                    Edition::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                self.coating().edition = edition;
                Ok(value.clone())
            }
            "scheme" => {
                let scheme =
                    Scheme::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                let coating = self.coating();
                coating.scheme = scheme;
                match scheme {
                    Scheme::Multitone => {
                        coating.secondary.truncate(1);
                        if coating.secondary.is_empty() {
                            coating.secondary = vec![Ink::Blue];
                        }
                        if coating.shades.is_empty() {
                            coating.shades = vec![0, 1];
                        }
                    }
                    Scheme::Multicolor => {
                        coating.shades.clear();
                        if coating.secondary.is_empty() {
                            coating.secondary = vec![if coating.primary == Ink::White {
                                Ink::Black
                            } else {
                                Ink::White
                            }];
                        }
                    }
                }
                Ok(value.clone())
            }
            "target" => {
                let target =
                    Target::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                self.coating().target = target;
                Ok(value.clone())
            }
            "primary" => {
                let primary =
                    Ink::parse(value.as_str().unwrap_or("")).map_err(|_| "no such option")?;
                self.coating().primary = primary;
                Ok(value.clone())
            }
            _ => Err("no such key"),
        }
    }
    /// Dices a coating for the staged tile, rerolling the current one if there is any.
    pub fn roll_paint(&mut self) {
        let rolled = match &self.paint {
            Some(staged) => paint::reroll(staged.clone()),
            None => paint::setup(
                Paint::new(paint::random_edition(None)),
                &paint::Config::default(),
            ),
        };
        self.paint = Some(match tile2d::build(&self.tile) {
            Ok(mut cell) => paint::prime(rolled.clone(), &mut cell.cell, None).unwrap_or(rolled),
            Err(_) => rolled,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocabulary_keeps_a_single_option() {
        assert_eq!(vocabulary(&json!([])), None);
        assert_eq!(vocabulary(&json!([5])), Some("int 5..5".to_string()));
        assert_eq!(
            vocabulary(&json!(["General"])),
            Some("General | General".to_string())
        );
        assert_eq!(vocabulary(&json!([3, 5])), Some("3 | 5".to_string()));
        assert_eq!(
            vocabulary(&json!(["Evens", "Odds"])),
            Some("Evens | Odds".to_string())
        );
    }
}
