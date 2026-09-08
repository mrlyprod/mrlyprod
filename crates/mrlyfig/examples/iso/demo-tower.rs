use mrlycore::colors::{ORANGE, ORANGE_DARK, ORANGE_LIGHT};
use mrlycore::errors::Result;
use mrlyfig::board::Frame;
use mrlyfig::{iso, save, Board, Color};
use mrlymath::bang::{magic, MagicLayer};
use mrlymath::name::Bang;
use mrlymath::three::designs;
use mrlymath::three::Cell3d;

const NUMBER: usize = 3;
const SHARE: [f64; 3] = [0.26, 0.33, 0.4];
const WALK: [f64; 3] = [0.2, 0.5, 0.8];

fn word() -> [MagicLayer; 3] {
    [
        MagicLayer::new(Bang::new(23, 3, 2), NUMBER),
        MagicLayer::new(Bang::new(9, 3, 2), NUMBER),
        MagicLayer::new(Bang::new(23, 3, 2), NUMBER),
    ]
}

fn block(k: usize) -> Result<Cell3d> {
    let letters = word();
    if k == 1 {
        return designs::create(letters[0].design.code, NUMBER, 1, 2);
    }
    Ok(Cell3d::new(magic(&letters[..k])?))
}

fn shade() -> [Color; 3] {
    [ORANGE_LIGHT, ORANGE, ORANGE_DARK]
}

fn stack(frame: Frame) -> Vec<Frame> {
    let mut out = Vec::new();
    for (share, walk) in SHARE.iter().zip(WALK) {
        let side = frame.w * share;
        let (cx, cy) = (frame.x + frame.w * walk, frame.y + frame.h * walk);
        out.push(Frame::new(cx - side / 2.0, cy - side / 2.0, side, side));
    }
    out
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sides = [3usize, 9, 27];
    let counts = [20u64, 200, 4000];
    for (k, panel) in stack(frame).iter().enumerate() {
        let cell = block(k + 1)?;
        assert_eq!(cell.types().shape, vec![sides[k]; 3]);
        assert_eq!(cell.types().sum(), counts[k]);
        iso::draw(&mut board, *panel, &cell, shade(), None);
    }
    save("demo-tower", &board)?;
    Ok(())
}
