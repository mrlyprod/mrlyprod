use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{ink, plot, save};
use mrlymath::three;

const CODE: u128 = 126;
const LEVEL: usize = 4;
const WIDE: usize = 4;

fn span(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for (u, v) in points {
        lo.0 = lo.0.min(*u);
        lo.1 = lo.1.min(*v);
        hi.0 = hi.0.max(*u);
        hi.1 = hi.1.max(*v);
    }
    (lo, hi)
}

fn panels(frame: Frame, gap: f64) -> Vec<Frame> {
    frame
        .rows(WIDE)
        .iter()
        .flat_map(|row| row.cols(WIDE))
        .map(|cell| cell.inset(gap))
        .collect()
}

fn main() -> Result<()> {
    let counts = three::profile(CODE, 2, LEVEL, 2)?;
    let (low, high) = three::support(&counts).expect("the design fills some height");
    let cells = 3usize.pow(LEVEL as u32);
    assert_eq!((low, high), (15, 30));
    assert_eq!(high - low + 1, WIDE * WIDE);
    let mut cuts = Vec::new();
    for (height, count) in counts.iter().enumerate().take(high + 1).skip(low) {
        assert_eq!(*count, cells as u128);
        let points = three::diagonal_slice(CODE, 2, LEVEL, 2, height)?;
        assert_eq!(points.len(), cells);
        cuts.push(points.iter().copied().map(three::project).collect::<Vec<_>>());
    }
    let flat: Vec<(f64, f64)> = cuts.iter().flatten().copied().collect();
    assert_eq!(flat.len(), WIDE * WIDE * cells);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let stage = panels(frame, frame.w / WIDE as f64 * 0.05);
    let (lo, hi) = span(&flat);
    let mid = ((lo.0 + hi.0) / 2.0, (lo.1 + hi.1) / 2.0);
    let scale = (stage[0].w / (hi.0 - lo.0)).min(stage[0].h / (hi.1 - lo.1));
    let middle = [(low + high) / 2, (low + high).div_ceil(2)];
    for (i, cut) in cuts.iter().enumerate() {
        let (cx, cy) = stage[i].center();
        let dots: Vec<(f64, f64)> = cut
            .iter()
            .map(|(u, v)| (cx + (u - mid.0) * scale, cy - (v - mid.1) * scale))
            .collect();
        let color = if middle.contains(&(low + i)) {
            ink::yellow()
        } else {
            ink::blue()
        };
        plot::dots(&mut board, &dots, scale * 0.42, color);
    }
    save("demo-cuts", &board)?;
    Ok(())
}
