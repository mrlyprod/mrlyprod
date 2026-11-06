use super::Fate;
use super::models::{Config, Life};
use super::step::next_grid;
use mrly_error::Result;
use mrly_two::Cell2d;
use std::collections::HashMap;

fn prepare(seed: &Cell2d, config: &Config) -> Result<Cell2d> {
    let mut grid = seed.clone();
    if config.grid_size > 1 {
        grid = grid.tile(config.grid_size, config.grid_size)?;
    }
    if config.padding > 0 {
        grid = grid.pad(config.padding, 0);
    }
    Ok(grid)
}

pub fn animate(seed: &Cell2d, config: &Config) -> Result<Life> {
    let mask = config.mask.types().clone();
    let (birth, survive) = config.counts()?;
    let mut current = prepare(seed, config)?;
    let mut grids = vec![current.clone()];
    let mut history: HashMap<Vec<u8>, usize> = HashMap::new();
    history.insert(current.types().bytes()?.to_vec(), 0);
    let mut fate = Fate::Timeout;
    let mut loop_length = 0;
    for i in 1..config.max_generations {
        let next = next_grid(&current, &birth, &survive, &mask, config.boundary)?;
        if next.types() == current.types() {
            fate = if next.types().sum() == 0 {
                Fate::Dead
            } else {
                Fate::Alive
            };
            break;
        }
        let key = next.types().bytes()?.to_vec();
        if let Some(&seen_at) = history.get(&key) {
            loop_length = i - seen_at;
            fate = Fate::Loop;
            break;
        }
        history.insert(key, i);
        current = next;
        grids.push(current.clone());
    }
    let count = grids.len();
    Ok(Life {
        grids,
        fate,
        count,
        loop_length,
    })
}
