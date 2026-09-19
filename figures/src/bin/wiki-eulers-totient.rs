use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlynum::lattice::totients;
use mrlynum::prime::is_prime;

const TOP: usize = 60;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let phi = totients(TOP);
    assert_eq!(phi[59], 58);
    let ceiling = (TOP - 1) as f64;
    let slot = frame.w / TOP as f64;
    let pad = slot * 0.14;
    let mut touching = 0usize;
    for n in 1..=TOP {
        let hit = n > 1 && phi[n] as usize == n - 1;
        assert_eq!(hit, is_prime(n));
        touching += hit as usize;
        let height = frame.h * phi[n] as f64 / ceiling;
        board.rect(
            frame.x + (n - 1) as f64 * slot + pad,
            frame.y + frame.h - height,
            slot - 2.0 * pad,
            height,
            if hit { ink::yellow() } else { ink::blue() },
        );
    }
    assert_eq!(touching, 17);
    let rise = |n: usize| {
        (
            frame.x + (n as f64 - 0.5) * slot,
            frame.y + frame.h * (1.0 - (n - 1) as f64 / ceiling),
        )
    };
    board.segment(rise(1), rise(TOP), 2.0, ink::line());
    plot::baseline(&mut board, frame, ink::line());
    save("wiki-eulers-totient", &board)?;
    Ok(())
}
