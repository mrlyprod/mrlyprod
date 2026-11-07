use crate::collection::Collection;
use mrly_error::{Result, value_error};
use mrly_json::{Map, json};
use mrly_life::Catalog;
use mrly_life::recipe;
use mrly_life::{Reel, Run, Stop};
use mrly_rand::Rng;
use mrly_saga::Deck;

const DELAY: u16 = 10;

pub(crate) struct Pressed {
    pub(crate) meta: Map,
    pub(crate) files: Vec<(&'static str, Vec<u8>)>,
}

pub(crate) trait Source: Sync {
    fn draw(&self, rng: &mut Rng) -> Result<String>;
    fn count(&self) -> Option<usize> {
        None
    }
    fn key(&self, index: usize) -> Result<String> {
        value_error(format!("this source has no entry {index}: it only draws."))
    }
    fn press(&self, key: &str, files: bool) -> Result<Pressed>;
}

pub(crate) fn source(collection: &Collection) -> Result<Box<dyn Source>> {
    let catalog = || Catalog::standard();
    let whole = |key: &str, or: usize| collection.number(key).map_or(or, |n| n as usize);
    let lasts = |or: u32| collection.number("lasts").map_or(or, |n| n as u32);
    match collection.name.as_str() {
        "life" => Ok(Box::new(Life {
            catalog: catalog()?,
            lasts: lasts(mrly_life::catalog::LASTS),
        })),
        "saga" => {
            let catalog = catalog()?;
            let mut deck = Deck::preset(&catalog, collection.text("deck").unwrap_or("all"))?;
            deck.least = whole("least", deck.least);
            deck.most = whole("most", deck.most);
            deck.lasts = lasts(deck.lasts);
            if let Some(lengths) = collection.numbers("lengths")? {
                deck.lengths = lengths.into_iter().map(|n| n as u32).collect();
            }
            Ok(Box::new(Saga { catalog, deck }))
        }
        other => value_error(format!("the press has no source for {other} yet.")),
    }
}

pub(crate) fn known(name: &str) -> bool {
    matches!(name, "life" | "saga")
}

fn reel(meta: &mut Map, reel: &Reel) {
    let populations = reel.populations();
    let [height, width] = reel.shape();
    meta.insert("frames".into(), json!(reel.len()));
    meta.insert("height".into(), json!(height));
    meta.insert("width".into(), json!(width));
    meta.insert(
        "first".into(),
        json!(populations.first().copied().unwrap_or(0)),
    );
    meta.insert(
        "last".into(),
        json!(populations.last().copied().unwrap_or(0)),
    );
    meta.insert(
        "peak".into(),
        json!(populations.iter().copied().max().unwrap_or(0)),
    );
}

fn gif(reel: &Reel, files: bool) -> Result<Vec<(&'static str, Vec<u8>)>> {
    if !files {
        return Ok(Vec::new());
    }
    Ok(vec![("gif", mrly_life::gif(&reel.frames()?, 1, DELAY)?)])
}

struct Life {
    catalog: Catalog,
    lasts: u32,
}

impl Source for Life {
    fn draw(&self, rng: &mut Rng) -> Result<String> {
        Ok(recipe::name(&self.catalog.draw(rng, self.lasts)?))
    }

    fn count(&self) -> Option<usize> {
        Some(self.catalog.count())
    }

    fn key(&self, index: usize) -> Result<String> {
        Ok(recipe::name(&self.catalog.recipe(index)?))
    }

    fn press(&self, key: &str, files: bool) -> Result<Pressed> {
        let recipe = recipe::parse(key)?;
        let (shot, stop) = recipe::film(&recipe)?;
        let Stop {
            kind,
            generation,
            period,
            dr,
            dc,
            wall_by,
        } = stop;
        let mut meta = Map::new();
        meta.insert("stop".into(), json!(kind));
        meta.insert("generation".into(), json!(generation));
        meta.insert("period".into(), json!(period));
        meta.insert("dr".into(), json!(dr));
        meta.insert("dc".into(), json!(dc));
        meta.insert("wall_by".into(), json!(wall_by));
        meta.insert("flips".into(), json!(Run::new(&recipe)?.flips()));
        reel(&mut meta, &shot);
        Ok(Pressed {
            meta,
            files: gif(&shot, files)?,
        })
    }
}

struct Saga {
    catalog: Catalog,
    deck: Deck,
}

impl Source for Saga {
    fn draw(&self, rng: &mut Rng) -> Result<String> {
        Ok(mrly_saga::name(&mrly_saga::draw(
            &self.catalog,
            &self.deck,
            rng,
        )?))
    }

    fn press(&self, key: &str, files: bool) -> Result<Pressed> {
        let (shot, stops) = mrly_saga::film(&mrly_saga::parse(key)?)?;
        let mut meta = Map::new();
        meta.insert("chapters".into(), json!(stops.len()));
        meta.insert(
            "stops".into(),
            json!(stops.iter().map(|s| s.kind.clone()).collect::<Vec<_>>()),
        );
        meta.insert(
            "generations".into(),
            json!(stops.iter().map(|s| s.generation).collect::<Vec<_>>()),
        );
        reel(&mut meta, &shot);
        Ok(Pressed {
            meta,
            files: gif(&shot, files)?,
        })
    }
}
