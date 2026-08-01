use super::layout::Op;
use super::text;
use crate::tokens::LINE;

pub(crate) fn paint_into(buf: &mut [[u8; 4]], w: usize, h: usize, ops: &[Op]) {
    for op in ops {
        match op {
            Op::Rect {
                x,
                y,
                w: rw,
                h: rh,
                color,
            } => {
                if color[3] == 255 {
                    crate::draw::fill_rect(buf, w, h, *x, *y, *rw, *rh, *color);
                } else {
                    blend_rect(buf, w, h, *x, *y, *rw, *rh, *color);
                }
            }
            Op::Text {
                x,
                y,
                text,
                scale,
                color,
                under,
            } => {
                text::draw(buf, w, h, text, *x, *y, *scale, *color);
                if *under {
                    let rule = text::width(text, *scale);
                    let base = y + LINE * scale + scale;
                    crate::draw::fill_rect(buf, w, h, *x, base, rule, *scale, *color);
                }
            }
            Op::Image {
                x,
                y,
                w: iw,
                h: ih,
                scale,
                pixels,
            } => image(buf, w, h, *x, *y, *iw, *ih, *scale, pixels),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn blend_rect(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    rw: usize,
    rh: usize,
    color: [u8; 4],
) {
    for dy in 0..rh {
        for dx in 0..rw {
            let px = x + dx;
            let py = y + dy;
            if px < w && py < h {
                crate::frame::over(&mut buf[py * w + px], color);
            }
        }
    }
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
