#[allow(clippy::too_many_arguments)]
pub fn fill_rect(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x0: usize,
    y0: usize,
    rw: usize,
    rh: usize,
    color: [u8; 4],
) {
    for dy in 0..rh {
        for dx in 0..rw {
            let px = x0 + dx;
            let py = y0 + dy;
            if px < w && py < h {
                buf[py * w + px] = color;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn blit(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    rows: &[Vec<u8>],
    x: usize,
    y: usize,
    scale: usize,
    color: [u8; 4],
) {
    for (ry, row) in rows.iter().enumerate() {
        for (rx, &bit) in row.iter().enumerate() {
            if bit & 1 == 0 {
                continue;
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x + rx * scale + dx;
                    let py = y + ry * scale + dy;
                    if px < w && py < h {
                        buf[py * w + px] = color;
                    }
                }
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
