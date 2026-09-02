use crate::board::{Board, Frame};
use mrlycore::Color;

// RANGES

fn span(values: &[f64]) -> (f64, f64) {
    let lo = values.iter().copied().fold(f64::MAX, f64::min);
    let hi = values.iter().copied().fold(f64::MIN, f64::max);
    if hi - lo < 1e-12 {
        (lo - 0.5, hi + 0.5)
    } else {
        (lo, hi)
    }
}

// MARKS

/// Draws one bar per value from the foot of the frame, the tallest filling its height.
pub fn bars(board: &mut Board, frame: Frame, values: &[f64], gap: f64, color: Color) {
    if values.is_empty() {
        return;
    }
    let peak = values.iter().copied().fold(0.0f64, |a, b| a.max(b.abs()));
    if peak <= 0.0 {
        return;
    }
    let slot = frame.w / values.len() as f64;
    let pad = gap * slot / 2.0;
    for (i, value) in values.iter().enumerate() {
        let h = frame.h * (value.abs() / peak);
        board.rect(
            frame.x + i as f64 * slot + pad,
            frame.y + frame.h - h,
            slot - 2.0 * pad,
            h,
            color,
        );
    }
}

/// Draws one disc at each point, in board pixels.
pub fn dots(board: &mut Board, pts: &[(f64, f64)], r: f64, color: Color) {
    for (x, y) in pts {
        board.disc(*x, *y, r, color);
    }
}

/// Strokes one circle per radius about the same centre, in board pixels.
pub fn rings(board: &mut Board, center: (f64, f64), radii: &[f64], thick: f64, color: Color) {
    for r in radii {
        board.ring(center.0, center.1, *r, thick, color);
    }
}

/// Strokes the values as a staircase across the frame, one tread per value.
pub fn staircase(board: &mut Board, frame: Frame, values: &[f64], thick: f64, color: Color) {
    if values.is_empty() {
        return;
    }
    let (lo, hi) = span(values);
    let slot = frame.w / values.len() as f64;
    let mut pts = Vec::with_capacity(values.len() * 2);
    for (i, value) in values.iter().enumerate() {
        let y = frame.y + frame.h * (1.0 - (value - lo) / (hi - lo));
        pts.push((frame.x + i as f64 * slot, y));
        pts.push((frame.x + (i + 1) as f64 * slot, y));
    }
    board.polyline(&pts, thick, color);
}

/// Strokes a curve through the paired data, both axes mapped from their own range into the frame.
pub fn curve(board: &mut Board, frame: Frame, xs: &[f64], ys: &[f64], thick: f64, color: Color) {
    let n = xs.len().min(ys.len());
    if n < 2 {
        return;
    }
    let (x_lo, x_hi) = span(&xs[..n]);
    let (y_lo, y_hi) = span(&ys[..n]);
    let pts: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            (
                frame.x + frame.w * (xs[i] - x_lo) / (x_hi - x_lo),
                frame.y + frame.h * (1.0 - (ys[i] - y_lo) / (y_hi - y_lo)),
            )
        })
        .collect();
    board.polyline(&pts, thick, color);
}

/// Strokes the bare hairline box of the frame, without a tick or a label.
pub fn axis(board: &mut Board, frame: Frame, color: Color) {
    let thick = (frame.w.min(frame.h) / 512.0).max(1.0);
    let corners = [
        (frame.x, frame.y),
        (frame.x + frame.w, frame.y),
        (frame.x + frame.w, frame.y + frame.h),
        (frame.x, frame.y + frame.h),
        (frame.x, frame.y),
    ];
    board.polyline(&corners, thick, color);
}

/// Strokes the hairline foot of the frame alone.
pub fn baseline(board: &mut Board, frame: Frame, color: Color) {
    let thick = (frame.w.min(frame.h) / 512.0).max(1.0);
    board.segment(
        (frame.x, frame.y + frame.h),
        (frame.x + frame.w, frame.y + frame.h),
        thick,
        color,
    );
}
