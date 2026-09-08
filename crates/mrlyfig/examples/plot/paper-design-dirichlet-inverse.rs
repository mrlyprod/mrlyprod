use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, plot, save, Board, Frame};

const RE_WIDTH: f64 = 1.00;
const IM_REACH: f64 = 60.0;
const RHO: (f64, f64) = (0.720790, 28.605680);
const BOX_RE: (f64, f64) = (0.72074, 0.72084);
const BOX_IM: (f64, f64) = (28.60563, 28.60573);
const BANDS: [(f64, f64); 2] = [(22.01, 24.01), (56.00, 58.00)];
const BAND_RE: f64 = 3.02;
const OVER: f64 = 22.0;

fn at(rect: Frame, abscissa: f64, re: f64, im: f64) -> (f64, f64) {
    (
        rect.x + rect.w * (re - abscissa) / RE_WIDTH,
        rect.y + rect.h * (1.0 - im / IM_REACH),
    )
}

fn stroke_box(board: &mut Board, rect: Frame, thick: f64, color: Color) {
    let pts = [
        (rect.x, rect.y),
        (rect.x + rect.w, rect.y),
        (rect.x + rect.w, rect.y + rect.h),
        (rect.x, rect.y + rect.h),
        (rect.x, rect.y),
    ];
    board.polyline(&pts, thick, color);
}

fn half_plane(board: &mut Board, rect: Frame) {
    board.rect(rect.x, rect.y, rect.w, rect.h, ink::panel());
    board.segment(
        (rect.x - OVER, rect.y + rect.h),
        (rect.x + rect.w, rect.y + rect.h),
        1.6,
        ink::line(),
    );
    board.segment(
        (rect.x, rect.y - OVER),
        (rect.x, rect.y + rect.h + OVER),
        2.8,
        ink::fade(ink::indigo(), 0.5),
    );
}

fn band(board: &mut Board, rect: Frame, abscissa: f64, span: (f64, f64)) {
    let (_, low) = at(rect, abscissa, abscissa, span.0);
    let (_, high) = at(rect, abscissa, abscissa, span.1);
    let run = rect.w + OVER;
    board.rect(rect.x, high, run, low - high, ink::fade(ink::yellow(), 0.16));
    board.segment(
        (rect.x, high),
        (rect.x + run, high),
        1.8,
        ink::fade(ink::yellow(), 0.5),
    );
    board.segment(
        (rect.x, low),
        (rect.x + run, low),
        1.8,
        ink::fade(ink::yellow(), 0.5),
    );
}

fn comb(abscissa: f64, period: f64, reach: f64) -> Vec<(f64, f64)> {
    let mut poles = Vec::new();
    let mut j = 0;
    while j as f64 * period <= reach {
        poles.push((abscissa, j as f64 * period));
        j += 1;
    }
    poles
}

fn main() -> Result<()> {
    let alpha = 2f64.ln() / 3f64.ln();
    let period = 2.0 * std::f64::consts::PI / 3f64.ln();
    let poles = comb(alpha, period, IM_REACH);
    let integers = 1.0f64;

    assert_eq!((IM_REACH / period).floor() as usize, 10);
    assert_eq!(poles.len(), 11);
    assert_eq!(BANDS.len(), 2);
    assert!(poles.iter().all(|p| p.0 == alpha));
    assert!(RHO.0 > alpha && RHO.0 < alpha + RE_WIDTH);
    assert!(RHO.0 > BOX_RE.0 && RHO.0 < BOX_RE.1);
    assert!(RHO.1 > BOX_IM.0 && RHO.1 < BOX_IM.1);
    assert!(BANDS
        .iter()
        .all(|b| b.1 > b.0 && b.0 > 0.0 && b.1 < IM_REACH));
    const { assert!(BAND_RE > RE_WIDTH) };
    assert!((alpha - 0.6309297536).abs() < 1e-9);
    assert!((period - 5.7192017348).abs() < 1e-9);

    let mut board = Board::square();
    let work = board.frame(0.08).inset(OVER + 2.0);
    let pw = work.w * 0.455;
    let ph = work.h * 0.865;
    let design = Frame::new(work.x, work.y, pw, ph);
    let control = Frame::new(work.x + work.w - pw, work.y + work.h - ph, pw, ph);

    half_plane(&mut board, control);
    let (ix, iy) = at(control, integers, integers, 0.0);
    plot::dots(&mut board, &[(ix, iy)], 6.0, ink::fade(ink::indigo(), 0.92));

    half_plane(&mut board, design);
    for span in &BANDS {
        band(&mut board, design, alpha, *span);
    }
    let marks: Vec<(f64, f64)> = poles.iter().map(|p| at(design, alpha, p.0, p.1)).collect();
    plot::dots(&mut board, &marks, 6.0, ink::fade(ink::indigo(), 0.92));

    let emblem = at(design, alpha, RHO.0, RHO.1);
    let half = design.w * 0.057;
    stroke_box(
        &mut board,
        Frame::new(emblem.0 - half, emblem.1 - half, 2.0 * half, 2.0 * half),
        2.0,
        ink::fade(ink::yellow(), 0.85),
    );
    plot::dots(&mut board, &[emblem], 12.0, ink::yellow());

    save("paper-design-dirichlet-inverse", &board)?;
    Ok(())
}
