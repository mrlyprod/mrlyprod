use mrly_error::{Result, value_error};
use mrly_life::recipe::{self, Recipe};
use mrly_life::{Catalog, Counts};
use mrly_life::{Reel, Run, Stop};
use mrly_rand::Rng;
use serde::{Deserialize, Serialize};

pub const LASTS: u32 = 8;
pub const LENGTHS: [u32; 4] = [16, 32, 48, 64];
pub const LEAST: usize = 2;
pub const MOST: usize = 6;
pub const MOORE: &str = "code7.3.1";

const TRIES: usize = 64;
const BAR: &str = " | ";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    pub mask: String,
    pub birth: String,
    pub survive: String,
    pub x: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Saga {
    pub recipe: Recipe,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    pub masks: Vec<String>,
    pub rules: Vec<[String; 2]>,
    pub lengths: Vec<u32>,
    pub least: usize,
    pub most: usize,
    pub lasts: u32,
}

fn conway() -> [String; 2] {
    ["3".to_string(), "2,3".to_string()]
}

impl Deck {
    fn of(masks: Vec<String>, rules: Vec<[String; 2]>) -> Deck {
        Deck {
            masks,
            rules,
            lengths: LENGTHS.to_vec(),
            least: LEAST,
            most: MOST,
            lasts: LASTS,
        }
    }

    pub fn all(catalog: &Catalog) -> Deck {
        Deck::of(catalog.masks(), catalog.rules())
    }

    pub fn rules(catalog: &Catalog) -> Deck {
        Deck::of(vec![MOORE.to_string()], catalog.rules())
    }

    pub fn masks(catalog: &Catalog) -> Deck {
        Deck::of(catalog.masks(), vec![conway()])
    }

    pub fn preset(catalog: &Catalog, name: &str) -> Result<Deck> {
        match name {
            "all" => Ok(Deck::all(catalog)),
            "rules" => Ok(Deck::rules(catalog)),
            "masks" => Ok(Deck::masks(catalog)),
            other => value_error(format!(
                "no deck {other:?}; the decks are all, rules and masks."
            )),
        }
    }

    fn deal(&self, rng: &mut Rng) -> Result<Chapter> {
        let [birth, survive] = rng.choice(&self.rules)?.clone();
        Ok(Chapter {
            mask: rng.choice(&self.masks)?.clone(),
            birth,
            survive,
            x: *rng.choice(&self.lengths)?,
        })
    }
}

impl Chapter {
    fn name(&self) -> String {
        format!(
            "mask:{} birth:{} survive:{} x:{}",
            self.mask, self.birth, self.survive, self.x
        )
    }

    fn parse(text: &str) -> Result<Chapter> {
        let (mut mask, mut birth, mut survive, mut x) = (None, None, None, None);
        for word in text.split_whitespace() {
            let Some((key, value)) = word.split_once(':') else {
                return value_error(format!("word {word:?} wants key:value."));
            };
            match key {
                "mask" => mask = Some(value.to_string()),
                "birth" => birth = Some(Counts::parse(value)?.token()),
                "survive" => survive = Some(Counts::parse(value)?.token()),
                "x" => x = value.parse::<u32>().ok(),
                other => return value_error(format!("unknown key {other:?}.")),
            }
        }
        let (Some(mask), Some(birth), Some(survive), Some(x)) = (mask, birth, survive, x) else {
            return value_error(format!("chapter {text:?} misses a part."));
        };
        Ok(Chapter {
            mask,
            birth,
            survive,
            x,
        })
    }

    fn carry(&self, run: &Run) -> Result<Run> {
        run.carry(&self.mask, &self.birth, &self.survive, self.x)
    }
}

pub fn name(saga: &Saga) -> String {
    let mut out = recipe::name(&saga.recipe);
    for chapter in &saga.chapters {
        out.push_str(BAR);
        out.push_str(&chapter.name());
    }
    out
}

pub fn parse(text: &str) -> Result<Saga> {
    let mut parts = text.split(BAR);
    let recipe = recipe::parse(parts.next().unwrap_or_default())?;
    let chapters = parts.map(Chapter::parse).collect::<Result<Vec<_>>>()?;
    Ok(Saga { recipe, chapters })
}

fn turn(deck: &Deck, run: &Run, rng: &mut Rng) -> Result<Option<(Chapter, Run)>> {
    for _ in 0..TRIES {
        let chapter = deck.deal(rng)?;
        let mut next = chapter.carry(run)?;
        if next.finish().generation >= deck.lasts {
            return Ok(Some((chapter, next)));
        }
    }
    Ok(None)
}

pub fn draw(catalog: &Catalog, deck: &Deck, rng: &mut Rng) -> Result<Saga> {
    if deck.least == 0 || deck.most < deck.least {
        return value_error("a deck tells at least one chapter and no fewer than its least.");
    }
    for _ in 0..TRIES {
        let mut recipe = catalog.recipe(rng.below(catalog.count()))?;
        let want = deck.least + rng.below(deck.most - deck.least + 1);
        let first = deck.deal(rng)?;
        (recipe.mask, recipe.birth, recipe.survive, recipe.x) =
            (first.mask, first.birth, first.survive, first.x);
        let mut run = Run::new(&recipe)?;
        if run.finish().generation < deck.lasts {
            continue;
        }
        let mut chapters = Vec::new();
        while chapters.len() + 1 < want {
            let Some((chapter, next)) = turn(deck, &run, rng)? else {
                break;
            };
            chapters.push(chapter);
            run = next;
        }
        if chapters.len() + 1 >= deck.least {
            return Ok(Saga { recipe, chapters });
        }
    }
    value_error(format!(
        "no saga held {} chapters of {} generations in {TRIES} draws.",
        deck.least, deck.lasts
    ))
}

pub fn film(saga: &Saga) -> Result<(Reel, Vec<Stop>)> {
    let mut run = Run::new(&saga.recipe)?;
    let mut reel = Reel::new();
    reel.shoot(&run);
    let mut stops = vec![reel.roll(&mut run)];
    for chapter in &saga.chapters {
        run = chapter.carry(&run)?;
        stops.push(reel.roll(&mut run));
    }
    Ok((reel, stops))
}
