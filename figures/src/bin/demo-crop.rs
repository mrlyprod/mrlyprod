use figures::{iso, save, Board};
use mrlyrs::core::colors::{shades, ORANGE};
use mrlyrs::core::error::Result;
use mrlyrs::math::bang::Code;
use mrlyrs::math::shape::{self, Frac, Region};
use mrlyrs::math::three::{designs, Cell3d};

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sponge = designs::create(Code::from(23u128), 3, 3, 2)?;
    assert_eq!(sponge.width(), 27);
    assert_eq!(sponge.types().sum(), 8000);
    let cut = shape::named("octahedron", 3, Frac::new(1, 2)?)?;
    let tally = shape::census(&cut, sponge.types())?;
    let kept = Cell3d::new(shape::crop(sponge.types(), &cut, true)?)?;
    assert_eq!(
        kept.types().sum() as usize,
        tally.filled[Region::In as usize] + tally.filled[Region::Cut as usize]
    );
    assert!(kept.types().sum() < 8000);
    iso::draw(&mut board, frame, &kept, shades(ORANGE), None);
    save("demo-crop", &board)?;
    Ok(())
}
