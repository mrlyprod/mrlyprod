use crate::config::{Config, LADDER};
use crate::Path;
use mrlycore::errors::Result;
use mrlycore::state::{boolean, choice};
use mrlycore::tile::{Design, Group, Source, Tile, CLASSICS_2D};
use mrlymath::two::tile as tile2d;
use mrlymath::two::{build, Cell2d};

/// A drawn board: the seed cell and its climb from tile to canvas.
#[derive(Clone, Debug)]
pub struct Board {
    /// The built seed cell.
    pub cell: Cell2d,
    /// The tile side in cells.
    pub unit: usize,
    /// The tile to grid multiplier.
    pub grid: usize,
    /// The grid to canvas multiplier.
    pub canvas: usize,
}

impl Board {
    /// Wraps a bare grid as a board with no climb.
    pub fn of(cell: Cell2d) -> Board {
        let unit = cell.width().max(cell.height());
        Board {
            cell,
            unit,
            grid: 1,
            canvas: 1,
        }
    }
    /// Returns the grid side in cells.
    pub fn grid_unit(&self) -> usize {
        self.unit * self.grid
    }
    /// Returns the canvas side in cells.
    pub fn canvas_unit(&self) -> usize {
        self.grid_unit() * self.canvas
    }
    /// Returns the dead border between grid and canvas.
    pub fn padding(&self) -> usize {
        (self.canvas_unit() - self.grid_unit()) / 2
    }
}

fn ladder_climb(base: usize, max_canvas: usize) -> usize {
    let options: Vec<usize> = LADDER
        .iter()
        .copied()
        .filter(|i| base * i <= max_canvas)
        .collect();
    if options.is_empty() {
        return 1;
    }
    choice(&options)
}

fn draw_config(min_size: usize, max_size: usize) -> tile2d::Config {
    tile2d::Config {
        groups: vec![Group::General, Group::Fractal, Group::Magic],
        min_size,
        max_size,
        ..tile2d::Config::default()
    }
}

/// Draws a seeded board: a random tile climbed up the grid and canvas ladders.
pub fn board(config: &Config) -> Result<Board> {
    let tile = tile2d::create(&tile2d::Config {
        min_size: config.min_tile,
        max_size: config.max_tile,
        ..tile2d::Config::default()
    })?;
    let cell = build(&tile)?;
    let unit = tile.max_size();
    let grid = ladder_climb(unit, config.max_canvas);
    let canvas = ladder_climb(unit * grid, config.max_canvas);
    Ok(Board {
        cell,
        unit,
        grid,
        canvas,
    })
}

fn pop_center(cell: &mut Cell2d) {
    let center = cell.width() / 2;
    cell.cell.types.set(&[center, center], 0);
}

fn simple_mask() -> Result<Cell2d> {
    let design = choice(&CLASSICS_2D);
    let mut tile = Tile::new(Group::General).size(3, 3);
    tile.sources = vec![Source::Classic(design)];
    tile.numbers = vec![3];
    tile.levels = vec![1];
    tile.rotations = vec![0];
    tile.anti = vec![false];
    tile.factor = 3;
    tile.invert = matches!(design, Design::Htree | Design::Vtree) && boolean();
    build(&tile)
}

fn basic_mask(config: &Config) -> Result<Cell2d> {
    let tile = tile2d::create(&draw_config(config.min_mask, config.max_mask))?;
    build(&tile)
}

fn copy_mask(board: &Board) -> Cell2d {
    let cell = board.cell.clone();
    if boolean() {
        cell.invert()
    } else {
        cell
    }
}

/// Draws a segment's mask down one of the three paths, center popped.
pub fn mask(board: &Board, config: &Config) -> Result<(Path, Cell2d)> {
    let mut paths = vec![Path::Simple, Path::Basic];
    if (config.min_mask..=config.max_mask).contains(&board.unit) {
        paths.push(Path::Copy);
    }
    let path = choice(&paths);
    let mut cell = match path {
        Path::Simple => simple_mask()?,
        Path::Basic => basic_mask(config)?,
        Path::Copy => copy_mask(board),
    };
    pop_center(&mut cell);
    Ok((path, cell))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::{guard, seed};
    fn tiny() -> Config {
        Config {
            max_canvas: 15,
            min_tile: 3,
            max_tile: 5,
            min_mask: 3,
            max_mask: 5,
            ..Config::default()
        }
    }
    #[test]
    fn boards_climb_within_the_canvas_cap() {
        let _g = guard();
        let config = tiny();
        for s in 0..50 {
            seed(s);
            let b = board(&config).unwrap();
            assert!(b.unit >= 3 && b.unit <= 5);
            assert!(LADDER.contains(&b.grid));
            assert!(LADDER.contains(&b.canvas));
            assert!(b.grid_unit() <= config.max_canvas);
            assert!(b.canvas_unit() <= config.max_canvas);
            assert_eq!(b.grid_unit() + 2 * b.padding(), b.canvas_unit());
            assert_eq!(b.cell.width(), b.unit);
        }
    }
    #[test]
    fn masks_come_odd_square_and_popped() {
        let _g = guard();
        let config = tiny();
        for s in 0..50 {
            seed(s);
            let b = board(&config).unwrap();
            let (path, cell) = mask(&b, &config).unwrap();
            let side = cell.width();
            assert_eq!(side, cell.height(), "path {path:?} seed {s}");
            assert_eq!(side % 2, 1, "path {path:?} seed {s}");
            assert_eq!(cell.types().get(&[side / 2, side / 2]), 0);
            if path == Path::Simple {
                assert_eq!(side, 3);
            }
        }
    }
    #[test]
    fn copy_masks_mirror_the_board() {
        let _g = guard();
        let config = tiny();
        for s in 0..200 {
            seed(s);
            let b = board(&config).unwrap();
            let (path, cell) = mask(&b, &config).unwrap();
            if path == Path::Copy {
                assert_eq!(cell.width(), b.cell.width());
                return;
            }
        }
        panic!("no seed took the copy path");
    }
    #[test]
    fn oversized_boards_lose_the_copy_path() {
        let _g = guard();
        let config = Config {
            max_mask: 3,
            ..tiny()
        };
        for s in 0..100 {
            seed(s);
            let b = board(&config).unwrap();
            if b.unit > 3 {
                let (path, _) = mask(&b, &config).unwrap();
                assert_ne!(path, Path::Copy, "seed {s}");
            }
        }
    }
    #[test]
    fn wrapped_boards_carry_no_climb() {
        let b = Board::of(mrlymath::life::moore());
        assert_eq!(b.unit, 3);
        assert_eq!(b.grid_unit(), 3);
        assert_eq!(b.canvas_unit(), 3);
        assert_eq!(b.padding(), 0);
    }
}
