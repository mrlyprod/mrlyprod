use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Color, Frame};
use mrlynum::apollonian::{grow, is_ford, on_line, shadow, touches, Circle};
use mrlynum::lattice::farey;

const TOP: i64 = 2048;
const DEEP: usize = 32;
const CIRCLES: usize = 2448;
const RESTING: usize = 323;
const BRIGHT: u128 = 528;
const SIDE: f64 = 640.0;
const BAND: f64 = 232.0;
const OVER: f64 = 26.0;
const TICK: f64 = 3.0;
const RULE: f64 = 2.0;

fn octave(k: i64) -> usize {
    k.ilog2() as usize % 6
}

fn ring(board: &mut Board, at: (f64, f64), r: f64, thick: f64, span: (f64, f64), color: Color) {
    let half = thick / 2.0;
    let reach = r + half + 1.0;
    let x0 = (at.0 - reach).max(span.0 - 1.0).max(0.0).floor() as usize;
    let x1 = (at.0 + reach).min(span.1 + 1.0).max(0.0).ceil() as usize;
    let y0 = (at.1 - reach).max(0.0).floor() as usize;
    let y1 = (at.1 + reach).max(0.0).ceil() as usize;
    for py in y0..y1.min(board.height) {
        for px in x0..x1.min(board.width) {
            let (fx, fy) = (px as f64 + 0.5, py as f64 + 0.5);
            let edge = (((fx - at.0).powi(2) + (fy - at.1).powi(2)).sqrt() - r).abs() - half;
            let cut = (span.0 - fx).max(fx - span.1);
            board.blend(px, py, color, 0.5 - edge.max(cut));
        }
    }
}

fn main() -> Result<()> {
    let p = grow("strip", TOP)?;
    let marks = touches(&p);
    let read = shadow(&p, DEEP)?;
    let stack: Vec<(i64, i64, u64)> = farey(DEEP)
        .iter()
        .filter(|n| n.num > 0 && n.num < n.den)
        .map(|n| (n.num as i64, n.den as i64, n.brightness))
        .collect();

    assert_eq!(p.circles.len(), CIRCLES);
    assert_eq!((p.broken, p.strayed), (0, 0));
    assert!(p.strip);
    assert_eq!(marks.len(), RESTING);
    assert!(p
        .circles
        .iter()
        .filter(|c| on_line(**c))
        .all(|&c| is_ford(c)));
    assert!(read.covered);
    assert_eq!(
        (read.nodes, read.touched, read.missed),
        (RESTING, RESTING, 0)
    );
    assert_eq!((read.bright, read.want), (BRIGHT, BRIGHT));
    assert_eq!(stack.len(), RESTING);
    let mut peak = 0u64;
    for (node, mark) in stack.iter().zip(marks.iter()) {
        assert_eq!((node.0, node.1), (mark.num, mark.den));
        assert_eq!(mark.k, 2 * mark.den * mark.den);
        assert_eq!(node.2, DEEP as u64 / mark.den as u64);
        peak = peak.max(node.2);
    }
    assert_eq!(peak, DEEP as u64 / 2);

    let mut board = Board::square();
    let left = (board.width as f64 - SIDE) / 2.0;
    let head = (board.height as f64 - (SIDE + BAND)) / 2.0;
    let frame = Frame::new(left, head, SIDE, SIDE);
    let span = (frame.x, frame.x + frame.w);
    let foot = frame.y + frame.h;
    let six = ink::inks();
    let place = |c: &Circle| {
        let (x, y) = c.centre().unwrap_or((0.0, 0.0));
        (
            (frame.x + frame.w * x, foot - frame.h * y),
            frame.w / c.k as f64,
        )
    };

    for rule in [frame.y, foot] {
        board.segment(
            (span.0 - OVER, rule),
            (span.1 + OVER, rule),
            RULE,
            ink::dim(),
        );
    }
    for c in p
        .circles
        .iter()
        .chain(p.root.iter())
        .filter(|c| !c.is_line())
    {
        let (at, r) = place(c);
        let hair = (r / 12.0).clamp(0.65, 1.8);
        let thick = if is_ford(*c) { hair * 1.5 } else { hair };
        ring(&mut board, at, r, thick, span, six[octave(c.k)]);
    }
    for (num, den, bright) in &stack {
        let at = frame.x + frame.w * *num as f64 / *den as f64;
        let lit = *bright as f64 / peak as f64;
        board.segment(
            (at, foot - TICK),
            (at, foot + BAND * lit),
            0.75 + 2.0 * lit,
            ink::fade(ink::blue(), 0.45 + 0.55 * lit),
        );
    }
    save("demo-apollonian", &board)?;
    Ok(())
}
