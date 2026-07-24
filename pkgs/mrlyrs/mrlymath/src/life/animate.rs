use super::models::{Config, Life};
use super::step::next_grid;
use super::Fate;
use crate::two::Cell2d;
use mrlycore::errors::Result;
use std::collections::HashMap;

fn prepare(seed: &Cell2d, config: &Config) -> Cell2d {
    let mut grid = seed.clone();
    if config.grid_size > 1 {
        grid = grid.tile(config.grid_size, config.grid_size);
    }
    if config.padding > 0 {
        grid = grid.pad(config.padding, 0);
    }
    grid
}

pub fn animate(seed: &Cell2d, config: &Config) -> Result<Life> {
    let mask = config.mask.types().clone();
    let mut current = prepare(seed, config);
    let mut grids = vec![current.clone()];
    let mut history: HashMap<Vec<u8>, usize> = HashMap::new();
    history.insert(current.types().bytes().to_vec(), 0);
    let mut fate = Fate::Timeout;
    let mut loop_length = 0;
    for i in 1..config.max_generations {
        let next = next_grid(
            &current,
            &config.birth,
            &config.survive,
            &mask,
            config.boundary,
        )?;
        if next.types() == current.types() {
            fate = if next.types().sum() == 0 {
                Fate::Dead
            } else {
                Fate::Alive
            };
            break;
        }
        let key = next.types().bytes().to_vec();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::Boundary;
    use mrlycore::atoms;
    use mrlycore::tensor::Tensor;
    fn moore_mask() -> Cell2d {
        let mut m = atoms::carpet_2d(3);
        m.set(&[1, 1], 0);
        Cell2d::new(m)
    }
    fn conway(mask: Cell2d) -> Config {
        Config {
            boundary: Boundary::Constant,
            max_generations: 16,
            ..Config::new(mask, vec![3], vec![2, 3])
        }
    }
    #[test]
    fn blinker_is_a_loop_of_two() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        let life = animate(&Cell2d::new(t), &conway(moore_mask())).unwrap();
        assert_eq!(life.fate, Fate::Loop);
        assert_eq!(life.loop_length, 2);
    }
    #[test]
    fn block_is_alive_still_life() {
        let mut t = Tensor::new(vec![4, 4]);
        for (y, x) in [(1, 1), (1, 2), (2, 1), (2, 2)] {
            t.set(&[y, x], 1);
        }
        let life = animate(&Cell2d::new(t), &conway(moore_mask())).unwrap();
        assert_eq!(life.fate, Fate::Alive);
        assert_eq!(life.count, 1);
    }
    #[test]
    fn binarize_on_life_grids_stays_pointwise() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        let life = animate(&Cell2d::new(t), &conway(moore_mask())).unwrap();
        for grid in &life.grids {
            let binarized = grid.clone().binarize(1);
            assert_eq!(binarized.types(), grid.types());
            let twice = binarized.clone().binarize(1);
            assert_eq!(twice.types(), binarized.types());
        }
    }
}
