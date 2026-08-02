use super::layout::Op;
use super::text;
use crate::tokens::LINE;

pub(crate) fn paint_into(buf: &mut [[u8; 4]], w: usize, h: usize, ops: &[Op]) {
    band(buf, w, h, ops, 0);
}

pub(crate) fn band(buf: &mut [[u8; 4]], w: usize, h: usize, ops: &[Op], off: usize) {
    let off = off as i64;
    for op in ops {
        match op {
            Op::Rect {
                x,
                y,
                w: rw,
                h: rh,
                color,
                round,
            } => {
                let ty = *y as i64 - off;
                if outside(ty, *rh, h) {
                    continue;
                }
                rounded(buf, w, h, *x, ty, *rw, *rh, *color, *round);
            }
            Op::Text {
                x,
                y,
                text,
                scale,
                color,
                under,
            } => {
                let ty = *y as i64 - off;
                if outside(ty, LINE * scale + 2 * scale, h) {
                    continue;
                }
                text::draw(buf, w, h, text, *x, ty, *scale, *color);
                if *under {
                    let rule = text::width(text, *scale);
                    let base = ty + (LINE * scale + scale) as i64;
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
            } => {
                let ty = *y as i64 - off;
                if outside(ty, ih * scale, h) {
                    continue;
                }
                image(buf, w, h, *x, ty, *iw, *ih, *scale, pixels);
            }
        }
    }
}

fn outside(y: i64, tall: usize, h: usize) -> bool {
    y + tall as i64 <= 0 || y >= h as i64
}

#[allow(clippy::too_many_arguments)]
fn rounded(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x: usize,
    y: i64,
    rw: usize,
    rh: usize,
    color: [u8; 4],
    round: usize,
) {
    if round == 0 {
        if color[3] == 255 {
            crate::draw::fill_rect(buf, w, h, x, y, rw, rh, color);
        } else {
            blend_rect(buf, w, h, x, y, rw, rh, color);
        }
        return;
    }
    for dy in crate::draw::band(y, rh, h) {
        let cut = corner(dy, rh, round);
        let span = rw.saturating_sub(2 * cut);
        if span == 0 {
            continue;
        }
        let row = y + dy as i64;
        if color[3] == 255 {
            crate::draw::fill_rect(buf, w, h, x + cut, row, span, 1, color);
        } else {
            blend_rect(buf, w, h, x + cut, row, span, 1, color);
        }
    }
}

fn corner(row: usize, rh: usize, round: usize) -> usize {
    let r = round.min(rh / 2);
    if r == 0 {
        return 0;
    }
    let reach = r as f64 - 0.5;
    let off = if row < r {
        (r - row) as f64 - 0.5
    } else if row + r >= rh {
        (row + r + 1 - rh) as f64 - 0.5
    } else {
        return 0;
    };
    let inside = (reach * reach - off * off).max(0.0).sqrt();
    (reach - inside).round() as usize
}

#[allow(clippy::too_many_arguments)]
fn blend_rect(
    buf: &mut [[u8; 4]],
    w: usize,
    h: usize,
    x: usize,
    y: i64,
    rw: usize,
    rh: usize,
    color: [u8; 4],
) {
    for dy in crate::draw::band(y, rh, h) {
        let py = (y + dy as i64) as usize;
        for dx in 0..rw {
            let px = x + dx;
            if px < w {
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
    y: i64,
    iw: usize,
    ih: usize,
    scale: usize,
    pixels: &[[u8; 4]],
) {
    if pixels.len() != iw * ih {
        return;
    }
    for sy in crate::draw::band(y, ih * scale, h) {
        let py = (y + sy as i64) as usize;
        for sx in 0..iw * scale {
            let px = x + sx;
            if px >= w {
                continue;
            }
            let src = pixels[(sy / scale) * iw + sx / scale];
            crate::frame::over(&mut buf[py * w + px], src);
        }
    }
}
