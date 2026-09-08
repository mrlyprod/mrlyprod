use mrlycore::colors::{INDIGO, INDIGO_DARK, INDIGO_LIGHT};
use mrlycore::errors::Result;
use mrlyfig::{iso, save, Board};
use mrlylab::moire::volume::volume;
use mrlylab::moire::{Combine, Spec};
use mrlymath::three::Cell3d;

const CODE: u128 = 23;
const SIZE: usize = 64;
const SCALES: [usize; 6] = [1, 3, 5, 7, 9, 11];

fn main() -> Result<()> {
    let field = volume(
        Spec::new(CODE, 2, 3),
        &SCALES,
        Combine::Sum,
        1,
        SIZE,
    )?;
    let mark = SCALES.len() as f32;
    assert_eq!(field.data.len(), SIZE * SIZE * SIZE);
    assert_eq!(field.max(), mark);
    assert_eq!(field.count(mark), 34416);
    let shell = Cell3d::new(field.solid(mark));
    assert_eq!(shell.types().sum(), 34416);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    iso::draw(
        &mut board,
        frame,
        &shell,
        [INDIGO_LIGHT, INDIGO, INDIGO_DARK],
        None,
    );
    save("demo-volume", &board)?;
    Ok(())
}
