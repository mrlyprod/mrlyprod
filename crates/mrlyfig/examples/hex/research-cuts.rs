use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::{ink, plot, save};
use mrlymath::three;

const CODE: u128 = 126;
const LEVEL: usize = 7;
const HEIGHTS: [usize; 2] = [190, 191];
const HALF: usize = 6;

fn place(point: [u32; 3]) -> (f64, f64) {
    let (u, v) = three::shadow(point);
    (u as f64 / 2f64.sqrt(), v as f64 / 6f64.sqrt())
}

fn pieces(points: &[[u32; 3]]) -> Vec<usize> {
    let mut tally = vec![0usize; 8];
    for p in points {
        let key = ((p[0] >> HALF) * 4 + (p[1] >> HALF) * 2 + (p[2] >> HALF)) as usize;
        tally[key] += 1;
    }
    tally.retain(|&n| n > 0);
    tally
}

fn main() -> Result<()> {
    let profile = three::profile(CODE, 2, LEVEL, 2)?;
    let mut cuts = Vec::new();
    for height in HEIGHTS {
        assert_eq!(profile[height], 2187);
        let points = three::diagonal_slice(CODE, 2, LEVEL, 2, height)?;
        assert_eq!(points.len(), 2187);
        assert_eq!(pieces(&points), vec![729, 729, 729]);
        cuts.push(points);
    }

    let flat: Vec<(f64, f64)> = cuts.iter().flatten().copied().map(place).collect();
    assert_eq!(flat.len(), 4374);
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for (u, v) in &flat {
        lo.0 = lo.0.min(*u);
        lo.1 = lo.1.min(*v);
        hi.0 = hi.0.max(*u);
        hi.1 = hi.1.max(*v);
    }

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let span = (hi.0 - lo.0, hi.1 - lo.1);
    let scale = (frame.w / span.0).min(frame.h / span.1);
    let (cx, cy) = frame.center();
    let screen = |(u, v): (f64, f64)| {
        (
            cx + (u - (lo.0 + hi.0) / 2.0) * scale,
            cy - (v - (lo.1 + hi.1) / 2.0) * scale,
        )
    };
    for (cut, color) in cuts.iter().zip([ink::BLUE, ink::ORANGE]) {
        let dots: Vec<(f64, f64)> = cut.iter().copied().map(place).map(screen).collect();
        plot::dots(&mut board, &dots, 2.2, color);
    }
    save("research-cuts", &board)?;
    Ok(())
}
