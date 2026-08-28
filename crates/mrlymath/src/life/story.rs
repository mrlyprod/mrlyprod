use super::animate::animate;
use super::models::{Config, Life};
use super::Fate;
use crate::name::{Named, Rule};
use crate::two::{self, Cell2d};
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::{json, Json};
use serde::Deserialize;

/// One config's run inside a story.
#[derive(Clone, Debug)]
pub struct Chapter {
    /// The chapter's rulebook.
    pub config: Config,
    /// The chapter's recorded run.
    pub life: Life,
}

impl Chapter {
    fn truncate(&mut self, length: usize) {
        if length > 0 && length < self.life.grids.len() {
            self.life.grids.truncate(length);
            self.life.count = self.life.grids.len();
        }
    }
    /// Encodes the chapter's rule, mask, seed, length and fate as a JSON object,
    /// or an error when the config carries no nameable rule.
    pub fn to_json(&self) -> Result<Json> {
        Ok(json!({
            "v": 1,
            "rule": Rule::of(&self.config)?.to_str(),
            "mask": two::to_strings(&self.config.mask),
            "seed": self.life.grids.first().map(two::to_strings),
            "length": self.life.grids.len(),
            "fate": self.life.fate.name(),
        }))
    }
    /// Decodes a chapter from its JSON object and replays it, or an error naming the broken field.
    pub fn from_json(value: &Json) -> Result<Chapter> {
        let parts = Parts::deserialize(value)?;
        let rule = Rule::from_str(&parts.rule)?;
        let mask = two::from_strings(&parts.mask)?;
        let seed = two::from_strings(&parts.seed)?;
        if parts.length == 0 {
            return value_error("field \"length\" must be positive.");
        }
        let fate = Fate::parse(&parts.fate)?;
        let mut config = rule.config(mask);
        config.max_generations = parts.length;
        let mut life = animate(&seed, &config)?;
        life.grids.truncate(parts.length);
        life.count = life.grids.len();
        life.fate = fate;
        Ok(Chapter { config, life })
    }
}

#[derive(Deserialize)]
struct Parts {
    rule: String,
    mask: Vec<String>,
    seed: Vec<String>,
    length: usize,
    fate: String,
}

/// A chain of life runs, each seeded by the last frame of the one before.
#[derive(Clone, Debug)]
pub struct Story {
    /// The chapters in order.
    pub chapters: Vec<Chapter>,
}

impl Story {
    /// Builds an empty story.
    pub fn new() -> Story {
        Story {
            chapters: Vec::new(),
        }
    }
    /// Runs a chapter from a seed and returns its final grid.
    pub fn add(&mut self, seed: &Cell2d, config: &Config) -> Result<Cell2d> {
        let life = animate(seed, config)?;
        let last = life
            .last()
            .cloned()
            .ok_or_else(|| mrlycore::MrlyError::Value("chapter produced no grids.".into()))?;
        self.chapters.push(Chapter {
            config: config.clone(),
            life,
        });
        Ok(last)
    }
    /// Truncates the last chapter to a length and returns its new final grid.
    pub fn pivot(&mut self, length: usize) -> Result<Cell2d> {
        let chapter = self
            .chapters
            .last_mut()
            .ok_or_else(|| mrlycore::MrlyError::Value("no chapter to pivot.".into()))?;
        chapter.truncate(length);
        chapter
            .life
            .last()
            .cloned()
            .ok_or_else(|| mrlycore::MrlyError::Value("chapter empty after pivot.".into()))
    }
    /// Returns every grid of every chapter in order.
    pub fn grids(&self) -> Vec<Cell2d> {
        self.chapters
            .iter()
            .flat_map(|c| c.life.grids.iter().cloned())
            .collect()
    }
    /// Returns the frame count of each chapter.
    pub fn chapter_lengths(&self) -> Vec<usize> {
        self.chapters.iter().map(|c| c.life.grids.len()).collect()
    }
    /// Returns the total frame count across chapters.
    pub fn count(&self) -> usize {
        self.chapters.iter().map(|c| c.life.grids.len()).sum()
    }
    /// Returns the full-run index of a chapter's first frame.
    pub fn chapter_start(&self, i: usize) -> usize {
        self.chapters
            .iter()
            .take(i)
            .map(|c| c.life.grids.len())
            .sum()
    }
    /// Returns the full-run index one past a chapter's last frame.
    pub fn chapter_end(&self, i: usize) -> usize {
        self.chapter_start(i) + self.chapters.get(i).map_or(0, |c| c.life.grids.len())
    }
    /// Returns the index of the first frame.
    pub fn first_frame_idx(&self) -> usize {
        0
    }
    /// Returns the index of the last frame.
    pub fn last_frame_idx(&self) -> usize {
        self.count().saturating_sub(1)
    }
    /// Returns the last chapter's fate, or an error on an empty story.
    pub fn fate(&self) -> Result<Fate> {
        self.chapters
            .last()
            .map(|c| c.life.fate)
            .ok_or_else(|| mrlycore::MrlyError::Value("empty story.".into()))
    }
    /// Encodes the story chapter by chapter as a JSON object,
    /// or an error when a chapter carries no nameable rule.
    pub fn to_json(&self) -> Result<Json> {
        let chapters: Vec<Json> = self
            .chapters
            .iter()
            .map(Chapter::to_json)
            .collect::<Result<Vec<Json>>>()?;
        Ok(json!({
            "v": 1,
            "chapters": chapters,
        }))
    }
    /// Decodes a story from its JSON object and replays every chapter, or an error naming the broken field.
    pub fn from_json(value: &Json) -> Result<Story> {
        let chapters = value
            .get("chapters")
            .and_then(Json::as_array)
            .ok_or_else(|| MrlyError::Value("field \"chapters\" must be a list.".into()))?;
        let mut story = Story::new();
        for chapter in chapters {
            story.chapters.push(Chapter::from_json(chapter)?);
        }
        Ok(story)
    }
}

impl Default for Story {
    fn default() -> Story {
        Story::new()
    }
}

/// Runs a seed through each config in turn, pivoting between chapters when a length is given.
pub fn tell(seed: &Cell2d, configs: &[Config], pivot_at: Option<usize>) -> Result<Story> {
    if configs.is_empty() {
        return value_error("a story needs at least one chapter.");
    }
    let mut story = Story::new();
    let mut current = seed.clone();
    let last_index = configs.len() - 1;
    for (i, config) in configs.iter().enumerate() {
        current = story.add(&current, config)?;
        if i != last_index {
            if let Some(length) = pivot_at {
                current = story.pivot(length)?;
            }
        }
    }
    Ok(story)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{moore, Boundary, Config};
    use mrlycore::tensor::Tensor;
    fn conway() -> Config {
        Config {
            boundary: Boundary::Constant,
            max_generations: 12,
            padding: 2,
            ..Config::new(moore(), vec![3], vec![2, 3])
        }
    }
    fn blinker() -> Cell2d {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        Cell2d::new(t)
    }
    #[test]
    fn two_chapter_story_concatenates() {
        let story = tell(&blinker(), &[conway(), conway()], Some(2)).unwrap();
        assert_eq!(story.chapters.len(), 2);
        assert_eq!(story.chapter_lengths()[0], 2);
        assert_eq!(story.count(), story.grids().len());
    }
    #[test]
    fn story_json_replays_the_run() {
        let story = tell(&blinker(), &[conway(), conway()], Some(2)).unwrap();
        let back = Story::from_json(&story.to_json().unwrap()).unwrap();
        assert_eq!(back.chapter_lengths(), story.chapter_lengths());
        assert_eq!(back.fate().unwrap(), story.fate().unwrap());
        let (a, b) = (story.grids(), back.grids());
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x.types(), y.types());
        }
        for (c, d) in story.chapters.iter().zip(&back.chapters) {
            assert_eq!(Rule::of(&c.config).unwrap(), Rule::of(&d.config).unwrap());
            assert_eq!(c.life.fate, d.life.fate);
        }
    }
    #[test]
    fn chapter_json_rejects_broken_fields() {
        let story = tell(&blinker(), &[conway()], None).unwrap();
        let mut value = story.chapters[0].to_json().unwrap();
        value["fate"] = json!("sparkle");
        assert!(Chapter::from_json(&value).is_err());
        assert!(Chapter::from_json(&json!({})).is_err());
        assert!(Story::from_json(&json!({ "chapters": 3 })).is_err());
    }
}
