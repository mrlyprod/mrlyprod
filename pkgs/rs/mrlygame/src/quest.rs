use crate::config::Config;
use crate::frames::secondaries;
use crate::music::{score, Score};
use crate::sequence::{rulebook, Rulebook};
use crate::variations::{board, mask, Board};
use crate::{Path, Way};
use mrlycore::errors::{value_error, Result};
use mrlycore::paint::Ink;
use mrlycore::state::{choice, randint, seed};
use mrlymath::life::{crop, moore, Boundary, Config as LifeConfig, Counts, Fate, Story};
use mrlymath::two::Cell2d;

const MIN_PIVOT: usize = 8;

/// Returns the multiples of four a run of the count may pivot at, shortest first.
pub fn pivot_options(count: usize) -> Vec<usize> {
    let lo = count.min(MIN_PIVOT).div_ceil(4) * 4;
    let hi = (count / 4) * 4;
    if hi < lo {
        return Vec::new();
    }
    (lo..=hi).step_by(4).collect()
}

/// One chapter's editorial record: identity, way, mask, rule and score.
#[derive(Clone, Debug)]
pub struct Segment {
    /// The random hex identifier.
    pub key: String,
    /// The rulebook family.
    pub way: Way,
    /// The mask path.
    pub path: Path,
    /// The accent ink woven through the palettes.
    pub accent: Ink,
    /// The popped neighborhood mask.
    pub mask: Cell2d,
    /// The drawn rule, or None under Conway.
    pub rulebook: Option<Rulebook>,
    /// The musical styling.
    pub score: Score,
}

/// One finished quest: the story, its segments and the shared stage.
#[derive(Clone, Debug)]
pub struct Quest {
    /// The random hex identifier.
    pub key: String,
    /// The seed the attempt ran under.
    pub seed: u64,
    /// The edge policy every chapter runs under.
    pub boundary: Boundary,
    /// The primary ink.
    pub primary: Ink,
    /// The canvas side in cells before cropping.
    pub canvas: usize,
    /// The chaptered engine record.
    pub story: Story,
    /// The editorial record, one per chapter.
    pub segments: Vec<Segment>,
    /// Every frame, cropped to the lived-in square.
    pub grids: Vec<Cell2d>,
}

fn hex_key(length: usize) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    (0..length)
        .map(|_| DIGITS[randint(0, 15) as usize] as char)
        .collect()
}

fn setup_segment(prev: Option<(&Cell2d, Way)>, config: &Config) -> Result<(Board, Segment)> {
    let key = hex_key(8);
    let accent = choice(&secondaries());
    let (way, stage) = match prev {
        None => {
            let way = choice(&[Way::Conway, Way::Mrly]);
            (way, board(config)?)
        }
        Some((grid, prev_way)) => (prev_way.flip(), Board::of(grid.clone())),
    };
    let (path, mask_cell, rule) = match way {
        Way::Conway => (Path::Simple, moore(), None),
        Way::Mrly => {
            let (path, cell) = mask(&stage, config)?;
            let rule = rulebook(path == Path::Simple);
            (path, cell, Some(rule))
        }
    };
    let segment = Segment {
        key,
        way,
        path,
        accent,
        mask: mask_cell,
        rulebook: rule,
        score: score(way),
    };
    Ok((stage, segment))
}

fn chapter_config(segment: &Segment, boundary: Boundary, stage: &Board, cap: usize) -> LifeConfig {
    let (birth, survive) = match &segment.rulebook {
        Some(rule) => (rule.birth(), rule.survive()),
        None => (Counts::from(vec![3]), Counts::from(vec![2, 3])),
    };
    LifeConfig {
        boundary,
        max_generations: cap,
        grid_size: stage.grid,
        padding: stage.padding(),
        ..LifeConfig::new(segment.mask.clone(), birth, survive)
    }
}

fn attempt(config: &Config) -> Result<Option<Quest>> {
    let s = randint(0, i64::MAX) as u64;
    seed(s);
    let key = hex_key(8);
    let boundary = choice(&[Boundary::Constant, Boundary::Wrap]);
    let primary = Ink::Black;
    let mut story = Story::new();
    let mut segments: Vec<Segment> = Vec::new();
    let mut canvas = 0;
    let mut prev: Option<Cell2d> = None;
    for index in 0..config.max_segments {
        let chained = match (&prev, segments.last()) {
            (Some(grid), Some(last)) => Some((grid, last.way)),
            _ => None,
        };
        let (stage, segment) = setup_segment(chained, config)?;
        if index == 0 {
            canvas = stage.canvas_unit();
        }
        let chapter = chapter_config(&segment, boundary, &stage, config.max_generations);
        story.add(&stage.cell, &chapter)?;
        segments.push(segment);
        if story.fate()? == Fate::Alive {
            let grids = crop(&story.grids());
            return Ok(Some(Quest {
                key,
                seed: s,
                boundary,
                primary,
                canvas,
                story,
                segments,
                grids,
            }));
        }
        let count = story.chapters.last().map_or(0, |c| c.life.count);
        let options = pivot_options(count);
        let length = if options.is_empty() {
            count
        } else {
            choice(&options)
        };
        prev = Some(story.pivot(length)?);
    }
    Ok(None)
}

/// Runs the quest: seeded attempts retried until a chapter settles alive.
pub fn quest(config: &Config) -> Result<Quest> {
    for _ in 0..config.attempts.max(1) {
        if let Some(found) = attempt(config)? {
            return Ok(found);
        }
    }
    value_error("no attempt settled alive within the budget.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::guard;
    use mrlycore::Json;

    pub fn tiny() -> Config {
        Config {
            max_generations: 16,
            max_segments: 4,
            max_canvas: 15,
            min_tile: 3,
            max_tile: 5,
            min_mask: 3,
            max_mask: 5,
            attempts: 64,
        }
    }

    const PINNED: u64 = 1;

    #[test]
    fn the_quest_settles_alive_on_the_pinned_seed() {
        let _g = guard();
        mrlycore::state::seed(PINNED);
        let found = quest(&tiny()).unwrap();
        assert_eq!(found.story.fate().unwrap(), Fate::Alive);
        assert!(!found.segments.is_empty());
        assert!(found.segments.len() <= 4);
        assert_eq!(found.segments.len(), found.story.chapters.len());
        assert_eq!(found.grids.len(), found.story.count());
        assert!(found.canvas <= 15);
        assert!(found.grids[0].width() <= found.canvas);
    }

    #[test]
    fn ways_flip_between_chained_segments() {
        let _g = guard();
        mrlycore::state::seed(PINNED);
        let found = quest(&tiny()).unwrap();
        for pair in found.segments.windows(2) {
            assert_eq!(pair[1].way, pair[0].way.flip());
        }
        for segment in &found.segments {
            match segment.way {
                Way::Conway => {
                    assert_eq!(segment.path, Path::Simple);
                    assert!(segment.rulebook.is_none());
                    assert_eq!(segment.mask.types(), moore().types());
                }
                Way::Mrly => assert!(segment.rulebook.is_some()),
            }
        }
    }

    #[test]
    fn pivot_options_step_by_four() {
        assert_eq!(pivot_options(20), vec![8, 12, 16, 20]);
        assert_eq!(pivot_options(12), vec![8, 12]);
        assert_eq!(pivot_options(9), vec![8]);
        assert_eq!(pivot_options(8), vec![8]);
        assert_eq!(pivot_options(4), vec![4]);
        assert_eq!(pivot_options(7), Vec::<usize>::new());
        assert_eq!(pivot_options(3), Vec::<usize>::new());
        assert_eq!(pivot_options(0), vec![0]);
    }

    #[test]
    fn every_pivot_lands_on_a_multiple_of_four() {
        let _g = guard();
        mrlycore::state::seed(PINNED);
        let found = quest(&tiny()).unwrap();
        let lengths = found.story.chapter_lengths();
        for &length in &lengths[..lengths.len() - 1] {
            assert!(length % 4 == 0 || pivot_options(length).is_empty());
        }
    }

    #[test]
    fn a_wide_mask_quest_replays_from_its_names() {
        let _g = guard();
        let mut wide = false;
        for s in 1..=8u64 {
            mrlycore::state::seed(s);
            let found = quest(&tiny()).unwrap();
            let back = Story::from_json(&found.story.to_json().unwrap()).unwrap();
            assert_eq!(back.chapter_lengths(), found.story.chapter_lengths());
            for (a, b) in found.story.grids().iter().zip(back.grids()) {
                assert_eq!(a.types(), b.types());
            }
            for (a, b) in found.story.chapters.iter().zip(&back.chapters) {
                let counts = a.config.counts().unwrap();
                assert_eq!(counts, b.config.counts().unwrap());
                wide |= counts.0.iter().chain(&counts.1).any(|&n| n > 9);
            }
        }
        assert!(wide, "no chapter counted past nine");
    }

    #[test]
    fn the_quest_replays_from_one_outer_seed() {
        let _g = guard();
        mrlycore::state::seed(PINNED);
        let a = quest(&tiny()).unwrap();
        mrlycore::state::seed(PINNED);
        let b = quest(&tiny()).unwrap();
        assert_eq!(a.key, b.key);
        assert_eq!(a.seed, b.seed);
        assert_eq!(
            Json::to_string(&a.story.to_json().unwrap()),
            Json::to_string(&b.story.to_json().unwrap())
        );
        for (x, y) in a.grids.iter().zip(&b.grids) {
            assert_eq!(x.types(), y.types());
        }
    }
}
