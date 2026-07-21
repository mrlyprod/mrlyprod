use super::helpers::{closest_nesting, default_paint, int, nearest, source_label, work};
use super::render::{blank, two_tone};
use super::rules::resize;
use super::{Tile, BUDGETS, MIN, THUMBS};
use crate::core::paint::{self, Edition, Ink, Paint, Scheme, Target};
use crate::core::tile::{
    generals, nestings, powers, products, Catalog, Group, Parity, Source, Tile as Model,
};
use crate::math::bang;
use crate::math::two::tile as tile2d;
use crate::ui::frame;
use serde_json::{json, Value as Json};

impl Tile {
    pub fn sources(&self) -> Vec<Source> {
        bang::sources(&self.catalog, 2)
    }
    pub fn generals_of(&self) -> Vec<usize> {
        generals(MIN, self.budget, self.parity)
    }
    pub fn powers_of(&self) -> Vec<(usize, usize)> {
        powers(MIN, self.budget, self.parity)
    }
    pub fn nestings_of(&self) -> Vec<Vec<usize>> {
        nestings(MIN, self.budget, self.parity)
    }
    pub fn pairs_of(&self) -> Vec<Vec<usize>> {
        products(MIN, self.budget, 2, self.parity)
    }
    pub fn levels_of(&self, n: usize) -> Vec<usize> {
        self.powers_of()
            .iter()
            .filter(|&&(m, _)| m == n)
            .map(|&(_, level)| level)
            .collect()
    }
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
    pub fn preview(&self, model: &Model) -> Json {
        let board = crate::ui::frame::board(self.dark);
        let fill = crate::ui::frame::ink(self.dark);
        match tile2d::build(model) {
            Ok(cell) => frame::field(
                cell.width(),
                cell.height(),
                two_tone(&cell, board, fill),
                board,
            )
            .fact(),
            Err(_) => blank(board),
        }
    }
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
                resize(&mut probe);
                json!({ "level": level, "frame": self.preview(&probe) })
            })
            .collect()
    }
    pub fn shelf(&self) -> Vec<Json> {
        self.library
            .iter()
            .map(|entry| {
                json!({
                    "id": entry.id,
                    "name": entry.name,
                    "value": work(&entry.tile, &entry.paint),
                    "frame": self.preview(&entry.tile),
                })
            })
            .collect()
    }
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
    pub fn options(&self) -> Json {
        let groups: Vec<&str> = self.feasible().iter().map(|g| g.name()).collect();
        let sources: Vec<Json> = self
            .sources()
            .iter()
            .map(|s| {
                let label = source_label(s);
                json!({ "label": label, "value": label })
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
            "budgets": BUDGETS,
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
        tile.anti = vec![false; slots];
        resize(&mut tile);
        self.tile = tile;
        Ok(())
    }
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
        resize(&mut self.tile);
    }
    pub fn resize_slots(&mut self, numbers: Vec<usize>) {
        let count = numbers.len();
        let filler = self.tile.sources[0];
        self.tile.numbers = numbers;
        self.tile.sources.resize(count, filler);
        self.tile.levels.resize(count, 1);
        self.tile.rotations.resize(count, 0);
        self.tile.anti.resize(count, false);
        resize(&mut self.tile);
    }
    pub fn slot(&self, call_slot: &Json) -> Result<usize, &'static str> {
        let slot = call_slot.as_u64().unwrap_or(0) as usize;
        if slot >= self.tile.sources.len() {
            return Err("no such slot");
        }
        Ok(slot)
    }
    pub fn coating(&mut self) -> &mut Paint {
        if self.paint.is_none() {
            self.paint = Some(default_paint());
        }
        self.paint.as_mut().unwrap()
    }
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
                let old = self.sources();
                let new = bang::sources(&next, 2);
                self.tile.sources = self
                    .tile
                    .sources
                    .iter()
                    .map(|s| {
                        let idx = old.iter().position(|o| o == s).unwrap_or(0);
                        new[idx.min(new.len() - 1)]
                    })
                    .collect();
                self.catalog = next;
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
                resize(&mut self.tile);
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
                resize(&mut self.tile);
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
                resize(&mut self.tile);
                Ok(json!(factor))
            }
            "rotation" => {
                let slot = self.slot(slot)?;
                let rotation = value
                    .as_u64()
                    .or_else(|| value.as_str().and_then(|s| s.parse::<u64>().ok()))
                    .unwrap_or(9) as usize;
                if rotation > 3 {
                    return Err("rotation is 0 to 3");
                }
                self.tile.rotations[slot] = rotation;
                Ok(json!(rotation))
            }
            "anti" => {
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
