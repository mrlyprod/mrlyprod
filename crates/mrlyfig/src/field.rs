use crate::board::{Board, Frame};
use crate::ink::Ramp;

// FIELDS

fn range(values: &[f64]) -> (f64, f64) {
    let lo = values.iter().copied().fold(f64::MAX, f64::min);
    let hi = values.iter().copied().fold(f64::MIN, f64::max);
    if hi - lo < 1e-12 {
        (lo, lo + 1.0)
    } else {
        (lo, hi)
    }
}

/// Paints a scalar field into the frame, nearest sampled, normalised to its own range.
pub fn draw(
    board: &mut Board,
    frame: Frame,
    width: usize,
    height: usize,
    values: &[f64],
    ramp: &Ramp,
) {
    draw_range(board, frame, width, height, values, range(values), ramp);
}

/// Paints a scalar field into the frame, nearest sampled, normalised to the given range.
pub fn draw_range(
    board: &mut Board,
    frame: Frame,
    width: usize,
    height: usize,
    values: &[f64],
    span: (f64, f64),
    ramp: &Ramp,
) {
    if width == 0 || height == 0 || values.len() < width * height {
        return;
    }
    let (lo, hi) = span;
    let reach = if (hi - lo).abs() < 1e-12 {
        1.0
    } else {
        hi - lo
    };
    let x0 = frame.x.ceil().max(0.0) as usize;
    let y0 = frame.y.ceil().max(0.0) as usize;
    let x1 = ((frame.x + frame.w).floor().max(0.0) as usize).min(board.width);
    let y1 = ((frame.y + frame.h).floor().max(0.0) as usize).min(board.height);
    for py in y0..y1 {
        for px in x0..x1 {
            let u = (px as f64 + 0.5 - frame.x) / frame.w;
            let v = (py as f64 + 0.5 - frame.y) / frame.h;
            let col = ((u * width as f64) as usize).min(width - 1);
            let row = ((v * height as f64) as usize).min(height - 1);
            let t = (values[row * width + col] - lo) / reach;
            board.blend(px, py, ramp.at(t), 1.0);
        }
    }
}

/// Paints a function over the unit square into the frame, sampled on a resolution by resolution grid.
pub fn sample(
    board: &mut Board,
    frame: Frame,
    resolution: usize,
    f: impl Fn(f64, f64) -> f64,
    ramp: &Ramp,
) {
    if resolution == 0 {
        return;
    }
    let mut values = Vec::with_capacity(resolution * resolution);
    for row in 0..resolution {
        for col in 0..resolution {
            let u = (col as f64 + 0.5) / resolution as f64;
            let v = (row as f64 + 0.5) / resolution as f64;
            values.push(f(u, v));
        }
    }
    draw(board, frame, resolution, resolution, &values, ramp);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink;
    #[test]
    fn a_flat_field_paints_the_ramps_low_end_everywhere() {
        let mut board = Board::new(64, 64, ink::GROUND);
        let frame = Frame::new(0.0, 0.0, 64.0, 64.0);
        sample(
            &mut board,
            frame,
            8,
            |_, _| 1.0,
            &Ramp::tone(ink::BLUE, ink::GOLD),
        );
        assert_eq!(
            board.pixels[0],
            [ink::BLUE.r, ink::BLUE.g, ink::BLUE.b, 255]
        );
    }
}
