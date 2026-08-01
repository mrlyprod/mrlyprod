use super::animate::animate;
use super::models::{Config, Life};
use super::Fate;
use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};

#[derive(Clone, Debug)]
pub struct Chapter {
    pub config: Config,
    pub life: Life,
}

impl Chapter {
    fn truncate(&mut self, length: usize) {
        if length > 0 && length < self.life.grids.len() {
            self.life.grids.truncate(length);
            self.life.count = self.life.grids.len();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Story {
    pub chapters: Vec<Chapter>,
}

impl Story {
    pub fn new() -> Story {
        Story {
            chapters: Vec::new(),
        }
    }
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
    pub fn grids(&self) -> Vec<Cell2d> {
        self.chapters
            .iter()
            .flat_map(|c| c.life.grids.iter().cloned())
            .collect()
    }
    pub fn chapter_lengths(&self) -> Vec<usize> {
        self.chapters.iter().map(|c| c.life.grids.len()).collect()
    }
    pub fn count(&self) -> usize {
        self.chapters.iter().map(|c| c.life.grids.len()).sum()
    }
    pub fn chapter_start(&self, i: usize) -> usize {
        self.chapters
            .iter()
            .take(i)
            .map(|c| c.life.grids.len())
            .sum()
    }
    pub fn chapter_end(&self, i: usize) -> usize {
        self.chapter_start(i) + self.chapters.get(i).map_or(0, |c| c.life.grids.len())
    }
    pub fn first_frame_idx(&self) -> usize {
        0
    }
    pub fn last_frame_idx(&self) -> usize {
        self.count().saturating_sub(1)
    }
    pub fn fate(&self) -> Result<Fate> {
        self.chapters
            .last()
            .map(|c| c.life.fate)
            .ok_or_else(|| mrlycore::MrlyError::Value("empty story.".into()))
    }
}

impl Default for Story {
    fn default() -> Story {
        Story::new()
    }
}

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
    use crate::life::{Boundary, Config};
    use mrlycore::atoms;
    use mrlycore::tensor::Tensor;
    fn moore() -> Cell2d {
        let mut m = atoms::carpet_2d(3);
        m.set(&[1, 1], 0);
        Cell2d::new(m)
    }
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
}
