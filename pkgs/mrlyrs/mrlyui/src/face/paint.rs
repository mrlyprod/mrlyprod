use super::layout::Op;
use super::text;

pub(crate) fn paint(ops: &[Op], w: usize, h: usize, board: [u8; 4]) -> Vec<[u8; 4]> {
    let mut buf = vec![board; w * h];
    for op in ops {
        match op {
            Op::Rect {
                x,
                y,
                w: rw,
                h: rh,
                color,
            } => crate::draw::fill_rect(&mut buf, w, h, *x, *y, *rw, *rh, *color),
            Op::Text {
                x,
                y,
                text,
                scale,
                color,
            } => text::draw(&mut buf, w, h, text, *x, *y, *scale, *color),
            Op::Image {
                x,
                y,
                w: iw,
                h: ih,
                scale,
                pixels,
            } => image(&mut buf, w, h, *x, *y, *iw, *ih, *scale, pixels),
        }
    }
    buf
}

#[allow(clippy::too_many_arguments)]
fn image(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    iw: usize,
    ih: usize,
    scale: usize,
    pixels: &[[u8; 4]],
) {
    if pixels.len() != iw * ih {
        return;
    }
    for sy in 0..ih * scale {
        for sx in 0..iw * scale {
            let px = x + sx;
            let py = y + sy;
            if px >= w || py >= h {
                continue;
            }
            let src = pixels[(sy / scale) * iw + sx / scale];
            crate::frame::over(&mut buf[py * w + px], src);
        }
    }
}
