use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::bang::factory;
use mrlymath::shape::crossing_tree;

const RADIUS: u64 = 242;
const LEVEL: usize = 5;
const SIDE: usize = 243;

fn outline(board: &mut Board, grid: &Grid, x: u64, y: u64, step: usize, thick: f64, color: Color) {
    let (left, top, _, _) = grid.cell(y as usize * step, SIDE - (x as usize + 1) * step);
    let (right, foot, w, h) = grid.cell((y as usize + 1) * step - 1, SIDE - 1 - x as usize * step);
    let (wide, tall) = (right + w - left, foot + h - top);
    board.rect(left, top, wide, thick, color);
    board.rect(left, top + tall - thick, wide, thick, color);
    board.rect(left, top, thick, tall, color);
    board.rect(left + wide - thick, top, thick, tall, color);
}

fn main() -> Result<()> {
    let tile = factory::create(7, 3, 2, 2, 1)?;
    let keep: Vec<bool> = tile.bytes().iter().map(|&byte| byte != 0).collect();
    let tree = crossing_tree(RADIUS, 3, &keep);
    assert_eq!(tree.orphans, 0);
    assert_eq!(tree.levels.len(), LEVEL + 1);
    for (level, boxes) in tree.levels.iter().enumerate() {
        assert_eq!(
            boxes.len() as u64,
            2 * (RADIUS / 3u64.pow(level as u32)) + 1
        );
    }
    assert_eq!(tree.levels[0].len(), 485);

    let design = factory::create(7, 3, 2, 2, LEVEL)?;
    assert_eq!(design.shape, vec![SIDE, SIDE]);
    assert_eq!(design.sum(), 32768);

    let mut board = Board::square();
    let grid = Grid::new(board.frame(0.08), SIDE, SIDE, 0.0);
    for x in 0..SIDE {
        for y in 0..SIDE {
            if design.bytes()[x * SIDE + y] != 0 {
                grid.fill(&mut board, y, SIDE - 1 - x, ink::line());
            }
        }
    }
    let unit = grid.cell(0, 0).2;
    for (level, thick, color) in [
        (2usize, 0.5 * unit, ink::dim()),
        (3, unit, ink::dim()),
        (4, 2.0 * unit, ink::dim()),
    ] {
        let step = 3usize.pow(level as u32);
        for cell in &tree.levels[level] {
            outline(&mut board, &grid, cell.x, cell.y, step, thick, color);
        }
    }
    let mut kept = 0u64;
    for cell in &tree.levels[0] {
        let color = if cell.live {
            kept += 1;
            ink::yellow()
        } else {
            ink::blue()
        };
        grid.fill(
            &mut board,
            cell.y as usize,
            SIDE - 1 - cell.x as usize,
            color,
        );
    }
    assert_eq!(kept, 296);
    save("demo-shell", &board)?;
    Ok(())
}
