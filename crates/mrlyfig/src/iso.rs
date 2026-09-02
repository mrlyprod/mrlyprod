use crate::board::{Board, Frame};
use mrlycore::Color;
use mrlymath::three::faces::quads;
use mrlymath::three::Cell3d;

// PROJECTION

const RATIO: f64 = 0.866_025_403_784_438_6;

/// Projects a point of cube space to the isometric plane: x runs right-down, y left-down, z up.
pub fn project(x: f64, y: f64, z: f64) -> (f64, f64) {
    ((x - y) * RATIO, (x + y) * 0.5 - z)
}

type Face = ([(f64, f64); 4], usize, f64);

fn fit(faces: &[Face], frame: Frame) -> impl Fn((f64, f64)) -> (f64, f64) {
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for (quad, _, _) in faces {
        for p in quad {
            lo.0 = lo.0.min(p.0);
            lo.1 = lo.1.min(p.1);
            hi.0 = hi.0.max(p.0);
            hi.1 = hi.1.max(p.1);
        }
    }
    let (span_x, span_y) = ((hi.0 - lo.0).max(1e-9), (hi.1 - lo.1).max(1e-9));
    let scale = (frame.w / span_x).min(frame.h / span_y);
    let (ox, oy) = (
        frame.x + (frame.w - span_x * scale) / 2.0,
        frame.y + (frame.h - span_y * scale) / 2.0,
    );
    move |p: (f64, f64)| (ox + (p.0 - lo.0) * scale, oy + (p.1 - lo.1) * scale)
}

// DRAWING

/// Draws a cube's exposed faces in isometric, fitted to the frame and painted back to front.
///
/// The shades are the tones of the top, the left and the right face in that order, and the
/// edge, when given, is the hairline stroked around every face. Faces turned away from the
/// viewer are dropped before the painter's sort.
pub fn draw(
    board: &mut Board,
    frame: Frame,
    cell: &Cell3d,
    shade: [Color; 3],
    edge: Option<Color>,
) {
    let mut faces = Vec::new();
    for quad in quads(cell) {
        let (nx, ny, nz) = (
            quad.normal.x as f64,
            quad.normal.y as f64,
            quad.normal.z as f64,
        );
        if nx + ny + nz <= 0.0 {
            continue;
        }
        let tone = if nz > 0.0 {
            0
        } else if ny > 0.0 {
            1
        } else {
            2
        };
        let corner = |i: usize| {
            let v = quad.verts[i];
            project(v.x as f64, v.y as f64, v.z as f64)
        };
        let depth = quad
            .verts
            .iter()
            .map(|v| (v.x + v.y + v.z) as f64)
            .sum::<f64>()
            / 4.0;
        faces.push(([corner(0), corner(1), corner(2), corner(3)], tone, depth));
    }
    if faces.is_empty() {
        return;
    }
    faces.sort_by(|a, b| a.2.total_cmp(&b.2));
    let place = fit(&faces, frame);
    let thick = (frame.w.min(frame.h) / 400.0).max(0.6);
    for (quad, tone, _) in &faces {
        let pts: Vec<(f64, f64)> = quad.iter().map(|p| place(*p)).collect();
        board.polygon(&pts, shade[*tone]);
        if let Some(color) = edge {
            let mut loop_pts = pts.clone();
            loop_pts.push(pts[0]);
            board.polyline(&loop_pts, thick, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_projection_lifts_z_straight_up_the_screen() {
        let ground = project(0.0, 0.0, 0.0);
        let above = project(0.0, 0.0, 1.0);
        assert!((above.0 - ground.0).abs() < 1e-12);
        assert!(above.1 < ground.1);
    }
}
