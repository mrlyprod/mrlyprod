use crate::ink;
use mrlycore::codec;
use mrlycore::errors::Result;
use mrlycore::Color;

// FRAME

/// A rectangle of board space: the area a figure lays itself out in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// The left edge in pixels.
    pub x: f64,
    /// The top edge in pixels.
    pub y: f64,
    /// The width in pixels.
    pub w: f64,
    /// The height in pixels.
    pub h: f64,
}

impl Frame {
    /// Builds a frame from its corner and its size.
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Frame {
        Frame { x, y, w, h }
    }
    /// Shrinks the frame by the same number of pixels on every side.
    pub fn inset(&self, px: f64) -> Frame {
        Frame::new(
            self.x + px,
            self.y + px,
            self.w - 2.0 * px,
            self.h - 2.0 * px,
        )
    }
    /// Returns the width of one of n columns.
    pub fn cell(&self, n: usize) -> f64 {
        self.w / n as f64
    }
    /// Maps unit coordinates, zero at the top left and one at the bottom right, to pixels.
    pub fn at(&self, u: f64, v: f64) -> (f64, f64) {
        (self.x + u * self.w, self.y + v * self.h)
    }
    /// Returns the middle of the frame.
    pub fn center(&self) -> (f64, f64) {
        self.at(0.5, 0.5)
    }
    /// Returns the largest square centred inside the frame.
    pub fn square(&self) -> Frame {
        let side = self.w.min(self.h);
        Frame::new(
            self.x + (self.w - side) / 2.0,
            self.y + (self.h - side) / 2.0,
            side,
            side,
        )
    }
    /// Returns the shorter half-side, the radius a centred disc fills the frame with.
    pub fn radius(&self) -> f64 {
        self.w.min(self.h) / 2.0
    }
    /// Splits the frame into n columns, left to right.
    pub fn cols(&self, n: usize) -> Vec<Frame> {
        let w = self.w / n as f64;
        (0..n)
            .map(|i| Frame::new(self.x + i as f64 * w, self.y, w, self.h))
            .collect()
    }
    /// Splits the frame into n rows, top to bottom.
    pub fn rows(&self, n: usize) -> Vec<Frame> {
        let h = self.h / n as f64;
        (0..n)
            .map(|i| Frame::new(self.x, self.y + i as f64 * h, self.w, h))
            .collect()
    }
}

// GEOMETRY

fn box_sdf(px: f64, py: f64, cx: f64, cy: f64, hw: f64, hh: f64) -> f64 {
    let qx = (px - cx).abs() - hw;
    let qy = (py - cy).abs() - hh;
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    outside + qx.max(qy).min(0.0)
}

fn segment_sdf(px: f64, py: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    let (vx, vy) = (b.0 - a.0, b.1 - a.1);
    let (wx, wy) = (px - a.0, py - a.1);
    let len = vx * vx + vy * vy;
    let t = if len <= f64::EPSILON {
        0.0
    } else {
        ((wx * vx + wy * vy) / len).clamp(0.0, 1.0)
    };
    ((wx - t * vx).powi(2) + (wy - t * vy).powi(2)).sqrt()
}

fn polygon_sdf(px: f64, py: f64, pts: &[(f64, f64)]) -> f64 {
    let mut dist = f64::MAX;
    let mut inside = false;
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        dist = dist.min(segment_sdf(px, py, a, b));
        if (a.1 > py) != (b.1 > py) && px < a.0 + (py - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            inside = !inside;
        }
    }
    if inside {
        -dist
    } else {
        dist
    }
}

fn bounds(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let mut b = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for p in pts {
        b.0 = b.0.min(p.0);
        b.1 = b.1.min(p.1);
        b.2 = b.2.max(p.0);
        b.3 = b.3.max(p.1);
    }
    b
}

// BOARD

/// The rgba canvas a figure is drawn on, row-major from the top left.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    /// The width in pixels.
    pub width: usize,
    /// The height in pixels.
    pub height: usize,
    /// The rgba pixels, one per point of the raster.
    pub pixels: Vec<[u8; 4]>,
}

impl Board {
    /// Builds a board of the given size flooded with the ground color.
    pub fn new(width: usize, height: usize, ground: Color) -> Board {
        Board {
            width,
            height,
            pixels: vec![[ground.r, ground.g, ground.b, ground.a]; width * height],
        }
    }
    /// The house figure: 1024 by 1024 on the dark ground.
    pub fn square() -> Board {
        Board::new(1024, 1024, ink::GROUND)
    }
    /// The social card: 1200 by 630 on the dark ground.
    pub fn og() -> Board {
        Board::new(1200, 630, ink::GROUND)
    }
    /// Returns the largest centred square left after a margin of the given fraction of the short side.
    pub fn frame(&self, margin: f64) -> Frame {
        let side = self.width.min(self.height) as f64 * (1.0 - 2.0 * margin);
        Frame::new(
            (self.width as f64 - side) / 2.0,
            (self.height as f64 - side) / 2.0,
            side,
            side,
        )
    }
    /// Returns the whole board inset by a margin of the given fraction of the short side.
    pub fn area(&self, margin: f64) -> Frame {
        let pad = self.width.min(self.height) as f64 * margin;
        Frame::new(0.0, 0.0, self.width as f64, self.height as f64).inset(pad)
    }
    /// Composites one color over one pixel at the given coverage.
    pub fn blend(&mut self, x: usize, y: usize, c: Color, cover: f64) {
        if x >= self.width || y >= self.height {
            return;
        }
        let a = (c.a as f64 / 255.0) * cover.clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let i = y * self.width + x;
        let d = self.pixels[i];
        let over = |s: u8, under: u8| (s as f64 * a + under as f64 * (1.0 - a)).round() as u8;
        let alpha = a + (d[3] as f64 / 255.0) * (1.0 - a);
        self.pixels[i] = [
            over(c.r, d[0]),
            over(c.g, d[1]),
            over(c.b, d[2]),
            (alpha * 255.0).round() as u8,
        ];
    }

    fn shade(&mut self, area: (f64, f64, f64, f64), c: Color, sdf: impl Fn(f64, f64) -> f64) {
        let x0 = (area.0 - 1.0).floor().max(0.0) as usize;
        let y0 = (area.1 - 1.0).floor().max(0.0) as usize;
        let x1 = (area.2 + 1.0).ceil().max(0.0) as usize;
        let y1 = (area.3 + 1.0).ceil().max(0.0) as usize;
        for py in y0..y1.min(self.height) {
            for px in x0..x1.min(self.width) {
                let d = sdf(px as f64 + 0.5, py as f64 + 0.5);
                self.blend(px, py, c, 0.5 - d);
            }
        }
    }

    /// Fills an axis-aligned rectangle.
    pub fn rect(&mut self, x: f64, y: f64, w: f64, h: f64, c: Color) {
        let (cx, cy) = (x + w / 2.0, y + h / 2.0);
        let (hw, hh) = (w / 2.0, h / 2.0);
        self.shade((x, y, x + w, y + h), c, |px, py| {
            box_sdf(px, py, cx, cy, hw, hh)
        });
    }
    /// Fills a rectangle with rounded corners of the given radius.
    pub fn round_rect(&mut self, x: f64, y: f64, w: f64, h: f64, r: f64, c: Color) {
        let (cx, cy) = (x + w / 2.0, y + h / 2.0);
        let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
        let (hw, hh) = (w / 2.0 - r, h / 2.0 - r);
        self.shade((x, y, x + w, y + h), c, |px, py| {
            box_sdf(px, py, cx, cy, hw, hh) - r
        });
    }
    /// Fills a disc.
    pub fn disc(&mut self, cx: f64, cy: f64, r: f64, c: Color) {
        self.shade((cx - r, cy - r, cx + r, cy + r), c, |px, py| {
            ((px - cx).powi(2) + (py - cy).powi(2)).sqrt() - r
        });
    }
    /// Strokes a circle of the given radius, the stroke centred on it.
    pub fn ring(&mut self, cx: f64, cy: f64, r: f64, thick: f64, c: Color) {
        let outer = r + thick / 2.0;
        self.shade(
            (cx - outer, cy - outer, cx + outer, cy + outer),
            c,
            |px, py| (((px - cx).powi(2) + (py - cy).powi(2)).sqrt() - r).abs() - thick / 2.0,
        );
    }
    /// Strokes a straight run between two points, with round caps.
    pub fn segment(&mut self, a: (f64, f64), b: (f64, f64), thick: f64, c: Color) {
        let half = thick / 2.0;
        let (bx0, by0, bx1, by1) = bounds(&[a, b]);
        self.shade(
            (bx0 - half, by0 - half, bx1 + half, by1 + half),
            c,
            |px, py| segment_sdf(px, py, a, b) - half,
        );
    }
    /// Strokes a chain of points as one stroke, with round caps and joints.
    pub fn polyline(&mut self, pts: &[(f64, f64)], thick: f64, c: Color) {
        if pts.len() < 2 {
            return;
        }
        let half = thick / 2.0;
        let (bx0, by0, bx1, by1) = bounds(pts);
        self.shade(
            (bx0 - half, by0 - half, bx1 + half, by1 + half),
            c,
            |px, py| {
                let mut d = f64::MAX;
                for pair in pts.windows(2) {
                    d = d.min(segment_sdf(px, py, pair[0], pair[1]));
                }
                d - half
            },
        );
    }
    /// Fills a triangle.
    pub fn triangle(&mut self, a: (f64, f64), b: (f64, f64), c: (f64, f64), color: Color) {
        self.polygon(&[a, b, c], color);
    }
    /// Fills any simple polygon, its inside decided by the even-odd rule.
    pub fn polygon(&mut self, pts: &[(f64, f64)], c: Color) {
        if pts.len() < 3 {
            return;
        }
        let (bx0, by0, bx1, by1) = bounds(pts);
        self.shade((bx0, by0, bx1, by1), c, |px, py| polygon_sdf(px, py, pts));
    }
    /// Strokes the arc of a circle about a centre between two angles in radians, clockwise on the screen.
    pub fn arc(&mut self, center: (f64, f64), r: f64, angles: (f64, f64), thick: f64, c: Color) {
        let (cx, cy) = center;
        let (from, to) = angles;
        let half = thick / 2.0;
        let outer = r + half;
        let span = (to - from).abs();
        let (lo, hi) = if to >= from { (from, to) } else { (to, from) };
        let ends = [
            (cx + r * lo.cos(), cy + r * lo.sin()),
            (cx + r * hi.cos(), cy + r * hi.sin()),
        ];
        self.shade(
            (cx - outer, cy - outer, cx + outer, cy + outer),
            c,
            |px, py| {
                let angle = (py - cy).atan2(px - cx);
                let mut turn = angle - lo;
                while turn < 0.0 {
                    turn += std::f64::consts::TAU;
                }
                if turn <= span.min(std::f64::consts::TAU) {
                    (((px - cx).powi(2) + (py - cy).powi(2)).sqrt() - r).abs() - half
                } else {
                    let d = ends
                        .iter()
                        .map(|e| ((px - e.0).powi(2) + (py - e.1).powi(2)).sqrt())
                        .fold(f64::MAX, f64::min);
                    d - half
                }
            },
        );
    }
    /// Encodes the board as a png at one pixel per point.
    pub fn png(&self) -> Result<Vec<u8>> {
        codec::png(&self.pixels, self.width, self.height, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_cells_tile_the_frame_exactly() {
        let frame = Board::square().frame(0.08);
        assert!((frame.cell(81) * 81.0 - frame.w).abs() < 1e-9);
    }
    #[test]
    fn a_disc_covers_its_own_area() {
        let mut board = Board::new(256, 256, ink::GROUND);
        board.disc(128.0, 128.0, 90.0, ink::FG);
        let lit: f64 = board
            .pixels
            .iter()
            .map(|p| (p[0] as f64 - 7.0) / (232.0 - 7.0))
            .sum();
        let want = std::f64::consts::PI * 90.0 * 90.0;
        assert!(
            (lit - want).abs() / want < 0.02,
            "covered {lit}, want {want}"
        );
    }
    #[test]
    fn the_og_board_is_the_social_card_size() {
        let board = Board::og();
        assert_eq!((board.width, board.height), (1200, 630));
    }
}
