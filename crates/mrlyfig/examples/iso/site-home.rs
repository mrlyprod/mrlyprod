use mrlycore::errors::Result;
use mrlyfig::{ink, iso, save, Board};
use mrlymath::three::designs;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sponge = designs::create(23, 3, 3, 2)?;
    assert_eq!(sponge.types().sum(), 8000);
    iso::draw(
        &mut board,
        frame,
        &sponge,
        [ink::FG, ink::BLUE, ink::DIM],
        None,
    );
    save("site-home", &board)?;
    Ok(())
}
