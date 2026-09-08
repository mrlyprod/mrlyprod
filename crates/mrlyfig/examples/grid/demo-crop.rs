use mrlycore::errors::Result;
use mrlycore::palette::{ORANGE, ORANGE_DARK, ORANGE_LIGHT};
use mrlyfig::{iso, save, Board};
use mrlymath::shape::{self, Frac, Region};
use mrlymath::three::{designs, Cell3d};

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sponge = designs::create(23, 3, 3, 2)?;
    assert_eq!(sponge.width(), 27);
    assert_eq!(sponge.types().sum(), 8000);
    let cut = shape::named("octahedron", 3, Frac::new(1, 2))?;
    let tally = shape::census(&cut, sponge.types());
    let kept = Cell3d::new(shape::crop(sponge.types(), &cut, true));
    assert_eq!(
        kept.types().sum() as usize,
        tally.filled[Region::In as usize] + tally.filled[Region::Cut as usize]
    );
    assert!(kept.types().sum() < 8000);
    iso::draw(
        &mut board,
        frame,
        &kept,
        [ORANGE_LIGHT, ORANGE, ORANGE_DARK],
        None,
    );
    save("demo-crop", &board)?;
    Ok(())
}
