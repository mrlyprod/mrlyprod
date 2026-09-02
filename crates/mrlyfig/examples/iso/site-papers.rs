use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;
use mrlyfig::{ink, iso, save, Board};
use mrlymath::three::Cell3d;
use mrlymath::two::designs;

const CODES: [u128; 12] = [15, 11, 10, 5, 1, 9, 7, 13, 12, 3, 6, 14];
const SIDE: usize = 9;
const PILES: usize = 2;
const LIFT: usize = 2;
const APART: usize = 18;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let deep = CODES.len() / PILES;
    let last = deep - 1;
    let wide = (PILES - 1) * APART + SIDE;
    let mut sheets = Vec::new();
    for code in CODES {
        let sheet = designs::create(code, 3, 2, 0, 2)?;
        let seed = designs::create(code, 3, 1, 0, 2)?;
        assert_eq!(sheet.types().shape, vec![SIDE, SIDE]);
        assert_eq!(sheet.types().sum(), seed.types().sum().pow(2));
        sheets.push((sheet.types().sum(), sheet));
    }
    assert_eq!(sheets.len(), PILES * deep);
    sheets.sort_by_key(|sheet| std::cmp::Reverse(sheet.0));
    let mut grid = Tensor::new(vec![wide, SIDE, last * LIFT + 1]);
    let mut want = 0u64;
    for (i, (count, sheet)) in sheets.iter().enumerate() {
        want += count;
        let cells = sheet.types();
        let (pile, step) = (i % PILES, i / PILES);
        let ox = pile * APART;
        let oz = step * LIFT;
        for a in 0..SIDE {
            for b in 0..SIDE {
                if cells.get(&[a, b]) != 0 {
                    grid.set(&[ox + a, b, oz], 1);
                }
            }
        }
    }
    assert_eq!(grid.sum(), want);
    let shelf = Cell3d::new(grid);
    iso::draw(
        &mut board,
        frame,
        &shelf,
        [ink::FG, ink::BLUE, ink::BLUE],
        None,
    );
    save("site-papers", &board)?;
    Ok(())
}
