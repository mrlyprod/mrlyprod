use figures::{ink, iso, save, Board, Color, Frame};
use mrlyrs::core::colors::{shades, GREEN};
use mrlyrs::core::error::Result;
use mrlyrs::math::bang::Code;
use mrlyrs::math::three::designs;

const CODE: u128 = 127;
const NUMBER: usize = 2;
const LEVEL: usize = 4;

fn shade() -> [Color; 3] {
    shades(GREEN)
}

fn corner(frame: Frame, size: f64, right: bool) -> Frame {
    let side = frame.w * size;
    let (x, y) = if right {
        (frame.x + frame.w - side, frame.y + frame.h - side)
    } else {
        (frame.x, frame.y)
    };
    Frame::new(x, y, side, side)
}

fn main() -> Result<()> {
    let seed = designs::create(Code::from(CODE), NUMBER, 1, 2)?;
    let solid = designs::create(Code::from(CODE), NUMBER, LEVEL, 2)?;
    assert_eq!(seed.types().sum(), 7);
    assert_eq!(solid.types().sum(), 7u64.pow(LEVEL as u32));
    assert_eq!(solid.types().shape, vec![16, 16, 16]);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    iso::draw(
        &mut board,
        corner(frame, 0.4, false),
        &seed,
        shade(),
        Some(ink::ground()),
    );
    iso::draw(&mut board, corner(frame, 0.64, true), &solid, shade(), None);
    save("demo-sponge", &board)?;
    Ok(())
}
