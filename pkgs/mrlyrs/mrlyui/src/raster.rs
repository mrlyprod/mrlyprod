use crate::frame::{field, over, Frame};

#[derive(Clone, Debug)]
pub struct Tri {
    pub x: [f32; 3],
    pub y: [f32; 3],
    pub z: [f32; 3],
    pub color: [u8; 4],
}

pub fn paint(width: usize, height: usize, tris: &[Tri], background: [u8; 4]) -> Frame {
    let mut buf = vec![background; width * height];
    let mut zbuf = vec![f32::MAX; width * height];
    for t in tris {
        fill(t, width, height, &mut buf, &mut zbuf, true);
    }
    field(width, height, buf, background)
}

fn edge(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
    (bx - ax) * (py - ay) - (by - ay) * (px - ax)
}

pub(crate) fn fill(
    t: &Tri,
    width: usize,
    height: usize,
    buf: &mut [[u8; 4]],
    zbuf: &mut [f32],
    write: bool,
) {
    let xi: Vec<i64> = t.x.iter().map(|&v| (v * 16.0).floor() as i64).collect();
    let yi: Vec<i64> = t.y.iter().map(|&v| (v * 16.0).floor() as i64).collect();
    let signed = edge(xi[0], yi[0], xi[1], yi[1], xi[2], yi[2]);
    if signed == 0 {
        return;
    }
    let (b, c) = if signed > 0 { (1, 2) } else { (2, 1) };
    let area = signed.abs();
    let min_x = xi.iter().min().unwrap() + 7;
    let max_x = xi.iter().max().unwrap() - 8;
    let min_y = yi.iter().min().unwrap() + 7;
    let max_y = yi.iter().max().unwrap() - 8;
    let px0 = min_x.div_euclid(16).max(0);
    let px1 = max_x.div_euclid(16).min(width as i64 - 1);
    let py0 = min_y.div_euclid(16).max(0);
    let py1 = max_y.div_euclid(16).min(height as i64 - 1);
    for py in py0..=py1 {
        for px in px0..=px1 {
            let sx = px * 16 + 8;
            let sy = py * 16 + 8;
            let wa = edge(xi[b], yi[b], xi[c], yi[c], sx, sy);
            let wb = edge(xi[c], yi[c], xi[0], yi[0], sx, sy);
            let wc = edge(xi[0], yi[0], xi[b], yi[b], sx, sy);
            if wa < 0 || wb < 0 || wc < 0 {
                continue;
            }
            let depth =
                (wa as f32 * t.z[0] + wb as f32 * t.z[b] + wc as f32 * t.z[c]) / area as f32;
            let i = py as usize * width + px as usize;
            if depth < zbuf[i] {
                if write {
                    zbuf[i] = depth;
                }
                if t.color[3] == 255 {
                    buf[i] = t.color;
                } else {
                    over(&mut buf[i], t.color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: [u8; 4] = [255, 0, 0, 255];
    const BLUE: [u8; 4] = [0, 0, 255, 255];
    const BG: [u8; 4] = [0, 0, 0, 255];

    fn tri(x: [f32; 3], y: [f32; 3], z: [f32; 3], color: [u8; 4]) -> Tri {
        Tri { x, y, z, color }
    }
    fn pixels(frame: &Frame) -> Vec<[u8; 4]> {
        frame.composite().cell.colors.unwrap()
    }

    #[test]
    fn a_triangle_paints_its_pixels() {
        let t = tri([0.0, 8.0, 0.0], [0.0, 0.0, 8.0], [1.0, 1.0, 1.0], RED);
        let px = pixels(&paint(8, 8, &[t], BG));
        assert_eq!(px[0], RED);
        assert_eq!(px[15], BG);
        assert_eq!(px[63], BG);
        assert!(px.iter().filter(|&&c| c == RED).count() > 20);
    }
    #[test]
    fn winding_does_not_matter() {
        let a = tri([0.0, 8.0, 0.0], [0.0, 0.0, 8.0], [1.0, 1.0, 1.0], RED);
        let b = tri([0.0, 0.0, 8.0], [0.0, 8.0, 0.0], [1.0, 1.0, 1.0], RED);
        assert_eq!(
            pixels(&paint(8, 8, &[a], BG)),
            pixels(&paint(8, 8, &[b], BG))
        );
    }
    #[test]
    fn the_nearer_triangle_wins() {
        let far = tri([0.0, 8.0, 0.0], [0.0, 0.0, 8.0], [5.0, 5.0, 5.0], RED);
        let near = tri([0.0, 8.0, 0.0], [0.0, 0.0, 8.0], [2.0, 2.0, 2.0], BLUE);
        for order in [[far.clone(), near.clone()], [near, far]] {
            assert_eq!(pixels(&paint(8, 8, &order, BG))[0], BLUE);
        }
    }
    #[test]
    fn degenerate_triangles_are_skipped() {
        let t = tri([1.0, 1.0, 1.0], [1.0, 4.0, 7.0], [1.0, 1.0, 1.0], RED);
        assert!(pixels(&paint(8, 8, &[t], BG)).iter().all(|&c| c == BG));
    }
    #[test]
    fn offscreen_extents_clip() {
        let t = tri(
            [-20.0, 20.0, 0.0],
            [-20.0, -20.0, 20.0],
            [1.0, 1.0, 1.0],
            RED,
        );
        let px = pixels(&paint(8, 8, &[t], BG));
        assert!(px.contains(&RED));
    }
    #[test]
    fn glass_blends_without_writing_depth() {
        let mut buf = vec![BG; 64];
        let mut zbuf = vec![f32::MAX; 64];
        let glass = tri(
            [0.0, 8.0, 0.0],
            [0.0, 0.0, 8.0],
            [2.0, 2.0, 2.0],
            [255, 0, 0, 128],
        );
        fill(&glass, 8, 8, &mut buf, &mut zbuf, false);
        assert!(buf[0][0] > 100 && buf[0][0] < 200);
        assert_eq!(zbuf[0], f32::MAX);
    }
}
