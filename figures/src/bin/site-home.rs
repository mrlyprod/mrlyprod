use mrlyfig::{ink, iso, save, Board};
use mrlyrs::core::errors::Result;
use mrlyrs::math::three::designs;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sponge = designs::create(23, 3, 3, 2)?;
    assert_eq!(sponge.types().sum(), 8000);
    iso::draw(
        &mut board,
        frame,
        &sponge,
        [ink::fg(), ink::blue(), ink::dim()],
        None,
    );
    save("site-home", &board)?;
    Ok(())
}
