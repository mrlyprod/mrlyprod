use mrlyfig::{ink, iso, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::math::three::designs;

const CODE: u128 = 23;
const LEVEL: usize = 3;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sponge = designs::create(CODE, 3, LEVEL, 2)?;
    assert_eq!(sponge.types().sum(), 8000);
    iso::draw(
        &mut board,
        frame,
        &sponge,
        [ink::yellow(), ink::orange(), ink::dim()],
        None,
    );
    save("wiki-menger-sponge", &board)?;
    Ok(())
}
