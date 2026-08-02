#[allow(clippy::too_many_arguments)]
pub fn fill_rect(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x0: usize,
    y0: i64,
    rw: usize,
    rh: usize,
    color: [u8; 4],
) {
    for dy in band(y0, rh, h) {
        let py = (y0 + dy as i64) as usize;
        for dx in 0..rw {
            let px = x0 + dx;
            if px < w {
                buf[py * w + px] = color;
            }
        }
    }
}

pub fn band(y: i64, tall: usize, h: usize) -> std::ops::Range<usize> {
    let first = (-y).clamp(0, tall as i64) as usize;
    let last = (h as i64 - y).clamp(0, tall as i64) as usize;
    first..last.max(first)
}

#[allow(clippy::too_many_arguments)]
pub fn blit(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    rows: &[Vec<u8>],
    x: usize,
    y: i64,
    scale: usize,
    color: [u8; 4],
) {
    let seen = band(y, rows.len() * scale, h);
    for (ry, row) in rows.iter().enumerate() {
        for dy in 0..scale {
            let step = ry * scale + dy;
            if !seen.contains(&step) {
                continue;
            }
            let py = (y + step as i64) as usize;
            for (rx, &bit) in row.iter().enumerate() {
                if bit & 1 == 0 {
                    continue;
                }
                for dx in 0..scale {
                    let px = x + rx * scale + dx;
                    if px < w {
                        buf[py * w + px] = color;
                    }
                }
            }
        }
    }
}

pub fn sprite(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    pixels: &[[u8; 4]],
    k: usize,
    x: usize,
    y: i64,
) {
    if pixels.len() != k * k {
        return;
    }
    for sy in band(y, k, h) {
        let py = (y + sy as i64) as usize;
        for sx in 0..k {
            let px = x + sx;
            if px < w {
                crate::frame::over(&mut buf[py * w + px], pixels[sy * k + sx]);
            }
        }
    }
}

pub fn fit(text: &str, field: usize, scales: &[usize]) -> (Vec<Vec<u8>>, usize, String) {
    let rows = mrlyfont::raster(text);
    let w = rows.first().map(Vec::len).unwrap_or(0);
    for &scale in scales {
        if w * scale <= field {
            return (rows, scale, text.to_string());
        }
    }
    let scale = scales.last().copied().unwrap_or(1);
    let mut chars: Vec<char> = text.chars().collect();
    while !chars.is_empty() {
        chars.pop();
        let cut_text: String = chars.iter().collect();
        let cut = mrlyfont::raster(&cut_text);
        let cw = cut.first().map(Vec::len).unwrap_or(0);
        if cw * scale <= field {
            return (cut, scale, cut_text);
        }
    }
    (Vec::new(), scale, String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_rect_clips_to_the_buffer() {
        let mut buf = vec![[0, 0, 0, 0]; 4];
        let red = [255, 0, 0, 255];
        fill_rect(&mut buf, 2, 2, 1, 1, 3, 3, red);
        assert_eq!(buf, vec![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], red]);
    }

    #[test]
    fn blit_scales_and_clips() {
        let mut buf = vec![[0, 0, 0, 0]; 9];
        let ink = [9, 9, 9, 255];
        blit(&mut buf, 3, 3, &[vec![1, 0], vec![0, 1]], 0, 0, 2, ink);
        assert_eq!(buf[0], ink);
        assert_eq!(buf[2], [0, 0, 0, 0]);
        assert_eq!(buf[8], ink);
    }

    #[test]
    fn fit_prefers_the_largest_scale() {
        let (rows, scale, text) = fit("hi", 1000, &[3, 2]);
        assert_eq!(scale, 3);
        assert_eq!(text, "hi");
        assert!(!rows.is_empty());
    }

    #[test]
    fn fit_truncates_at_the_smallest_scale() {
        let (_, scale, text) = fit("abcdefghijklmnopqrstuvwxyz", 40, &[3, 2]);
        assert_eq!(scale, 2);
        assert!(text.chars().count() < 26);
    }
}
