use mrlycore::trig;
use mrlymath::fractal;
use std::f64::consts::TAU;

pub(crate) fn paint(program: &str, u: &[f32], width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
    match program {
        "mandelbrot" => Some(mandelbrot(u, width, height)),
        "julia" => Some(julia(u, width, height)),
        "moire" => Some(moire(u, width, height)),
        "sleep" => Some(sleep(u, width, height)),
        _ => None,
    }
}

// PIXELS

fn at(u: &[f32], i: usize) -> f64 {
    u.get(i).copied().unwrap_or(0.0) as f64
}

fn hue(u: &[f32], i: usize) -> [f64; 3] {
    [at(u, i), at(u, i + 1), at(u, i + 2)]
}

fn shade(color: [f64; 3]) -> [u8; 4] {
    let byte = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    [byte(color[0]), byte(color[1]), byte(color[2]), 255]
}

fn mix(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn turn(angle: f64) -> (f64, f64) {
    let n = trig::N as i64;
    let step = ((angle * n as f64 / TAU).round() as i64).rem_euclid(n) as usize;
    let (ca, sa) = trig::unit(step);
    (ca as f64, sa as f64)
}

fn each(width: usize, height: usize, mut ink: impl FnMut(f64, f64) -> [u8; 4]) -> Vec<[u8; 4]> {
    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            pixels.push(ink(x as f64 + 0.5, y as f64 + 0.5));
        }
    }
    pixels
}

// MANDELBROT

fn mandelbrot(u: &[f32], width: usize, height: usize) -> Vec<[u8; 4]> {
    let (rx, ry) = (at(u, 0), at(u, 1));
    let (xmin, xmax, ymin, ymax) = (at(u, 12), at(u, 13), at(u, 14), at(u, 15));
    let center = ((xmin + xmax) * 0.5, (ymin + ymax) * 0.5);
    let (ca, sa) = turn(at(u, 11));
    let (time, max, band, fade) = (at(u, 2), at(u, 16), at(u, 17), at(u, 18));
    let primary = shade(hue(u, 4));
    let accent = shade(hue(u, 8));
    each(width, height, |fx, fy| {
        let cr = mix(xmin, xmax, fx / rx);
        let ci = mix(ymax, ymin, fy / ry);
        let (cr, ci) = fractal::rotate(cr, ci, center, ca, sa);
        let level = fractal::level(0.0, 0.0, cr, ci, max);
        fractal::tint(level, time, band, fade, primary, accent)
    })
}

// JULIA

fn julia(u: &[f32], width: usize, height: usize) -> Vec<[u8; 4]> {
    let (rx, ry) = (at(u, 0), at(u, 1));
    let (xmin, xmax, ymin, ymax) = (at(u, 12), at(u, 13), at(u, 14), at(u, 15));
    let center = ((xmin + xmax) * 0.5, (ymin + ymax) * 0.5);
    let (ca, sa) = turn(at(u, 11));
    let (cr, ci) = (at(u, 16), at(u, 17));
    let (time, max, band, fade) = (at(u, 2), at(u, 18), at(u, 19), at(u, 20));
    let primary = shade(hue(u, 4));
    let accent = shade(hue(u, 8));
    each(width, height, |fx, fy| {
        let zr = mix(xmin, xmax, fx / rx);
        let zi = mix(ymax, ymin, fy / ry);
        let (zr, zi) = fractal::rotate(zr, zi, center, ca, sa);
        let level = fractal::level(zr, zi, cr, ci, max);
        fractal::tint(level, time, band, fade, primary, accent)
    })
}

// MOIRE

fn parity(v: f64) -> f64 {
    let n = v.floor();
    n - 2.0 * (n * 0.5).floor()
}

fn hit(table: [f64; 4], n: f64, a: f64, b: f64) -> f64 {
    let ra = parity(n * a);
    let rb = parity(n * b);
    mix(mix(table[0], table[1], rb), mix(table[2], table[3], rb), ra)
}

fn moire(u: &[f32], width: usize, height: usize) -> Vec<[u8; 4]> {
    let (rx, ry) = (at(u, 0), at(u, 1));
    let side = rx.min(ry);
    let (ox, oy) = (0.5 * (rx - side), 0.5 * (ry - side));
    let table_a = [at(u, 12), at(u, 13), at(u, 14), at(u, 15)];
    let table_b = [at(u, 16), at(u, 17), at(u, 18), at(u, 19)];
    let (na, nb) = (at(u, 20), at(u, 21));
    let hex = at(u, 22) > 0.5;
    let root = 3.0f64.sqrt();
    let primary = hue(u, 4);
    let accent = hue(u, 8);
    each(width, height, |fx, fy| {
        let (ux, uy) = ((fx - ox) / side, (fy - oy) / side);
        let (a, b) = if hex {
            (ux - uy / root, 2.0 * uy / root)
        } else {
            (ux, uy)
        };
        let t = 0.5 * (hit(table_a, na, a, b) + hit(table_b, nb, a, b));
        shade([
            mix(primary[0], accent[0], t),
            mix(primary[1], accent[1], t),
            mix(primary[2], accent[2], t),
        ])
    })
}

// SLEEP

fn sleep(u: &[f32], width: usize, height: usize) -> Vec<[u8; 4]> {
    let (rx, ry) = (at(u, 0), at(u, 1));
    let (cols, rows) = (at(u, 12), at(u, 13));
    let scale = (rx / cols).min(ry / rows);
    let (ox, oy) = (0.5 * (rx - cols * scale), 0.5 * (ry - rows * scale));
    let (sx, sy) = (at(u, 14), at(u, 15));
    let span = at(u, 16);
    let primary = shade(hue(u, 4));
    let accent = shade(hue(u, 8));
    each(width, height, |fx, fy| {
        let (px, py) = ((fx - ox) / scale, (fy - oy) / scale);
        let (dx, dy) = (px - sx, py - sy);
        let inside = px >= 0.0 && py >= 0.0 && px < cols && py < rows;
        let covered = dx >= 0.0 && dy >= 0.0 && dx < span && dy < span;
        if inside && covered {
            accent
        } else {
            primary
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const WHITE: [u8; 4] = [255, 255, 255, 255];
    const BLACK: [u8; 4] = [0, 0, 0, 255];

    fn dive() -> Vec<f32> {
        let mut u = vec![0.0f32; 20];
        u[0] = 32.0;
        u[1] = 32.0;
        u[8..11].copy_from_slice(&[1.0, 1.0, 1.0]);
        u[12..16].copy_from_slice(&[-2.0, 1.0, -1.5, 1.5]);
        u[16] = 96.0;
        u[17] = 10.0;
        u[18] = 1.0;
        u
    }

    fn spiral() -> Vec<f32> {
        let mut u = vec![0.0f32; 24];
        u[0] = 32.0;
        u[1] = 32.0;
        u[8..11].copy_from_slice(&[1.0, 1.0, 1.0]);
        u[12..16].copy_from_slice(&[-1.5, 1.5, -1.5, 1.5]);
        u[16] = -0.4;
        u[17] = 0.6;
        u[18] = 96.0;
        u[19] = 10.0;
        u[20] = 1.0;
        u
    }

    fn weave() -> Vec<f32> {
        let mut u = vec![0.0f32; 24];
        u[0] = 32.0;
        u[1] = 32.0;
        u[8..11].copy_from_slice(&[1.0, 1.0, 1.0]);
        u[12..16].copy_from_slice(&[1.0, 1.0, 1.0, 0.0]);
        u[16..20].copy_from_slice(&[1.0, 1.0, 0.0, 1.0]);
        u[20] = 9.0;
        u[21] = 13.0;
        u
    }

    fn drift() -> Vec<f32> {
        let mut u = vec![0.0f32; 20];
        u[0] = 8.0;
        u[1] = 8.0;
        u[8..11].copy_from_slice(&[1.0, 1.0, 1.0]);
        u[12] = 4.0;
        u[13] = 4.0;
        u[16] = 1.0;
        u
    }

    #[test]
    fn the_dispatch_knows_four_programs() {
        for (name, u) in [
            ("mandelbrot", dive()),
            ("julia", spiral()),
            ("moire", weave()),
            ("sleep", drift()),
        ] {
            let pixels = paint(name, &u, 8, 6).unwrap();
            assert_eq!(pixels.len(), 48);
            assert!(pixels.iter().all(|p| p[3] == 255));
        }
        assert!(paint("nothing", &dive(), 8, 6).is_none());
    }
    #[test]
    fn sleep_paints_the_spot_in_the_top_left() {
        let pixels = paint("sleep", &drift(), 8, 8).unwrap();
        for i in [0, 1, 8, 9] {
            assert_eq!(pixels[i], WHITE);
        }
        assert_eq!(pixels[2], BLACK);
        assert_eq!(pixels[16], BLACK);
        assert_eq!(pixels[63], BLACK);
        assert_eq!(pixels.iter().filter(|p| **p == WHITE).count(), 4);
    }
    #[test]
    fn moire_stacks_two_lattices_on_the_ramp() {
        let pixels = paint("moire", &weave(), 32, 32).unwrap();
        assert!(pixels.iter().all(|p| [0, 128, 255].contains(&p[0])));
        for step in [0u8, 128, 255] {
            assert!(pixels.iter().any(|p| p[0] == step), "no pixel at {step}");
        }
    }
    #[test]
    fn mandelbrot_holds_the_set_against_the_bands() {
        let pixels = paint("mandelbrot", &dive(), 32, 32).unwrap();
        assert_eq!(pixels[16 * 32 + 16], BLACK);
        assert!(pixels.iter().any(|p| *p != pixels[0]));
        assert!(pixels.iter().filter(|p| p[0] > 0).count() > 32);
    }
    #[test]
    fn julia_fades_back_to_primary() {
        let mut u = spiral();
        assert!(paint("julia", &u, 32, 32).unwrap().iter().any(|p| p[0] > 0));
        u[20] = 0.0;
        assert!(paint("julia", &u, 32, 32)
            .unwrap()
            .iter()
            .all(|p| *p == BLACK));
    }
    #[test]
    fn a_negative_turn_lands_on_the_same_step() {
        let quarter = (TAU / 4.0) as f32;
        let mut back = dive();
        back[11] = -quarter;
        let mut forth = dive();
        forth[11] = 3.0 * quarter;
        let turned = paint("mandelbrot", &back, 24, 24);
        assert_eq!(turned, paint("mandelbrot", &forth, 24, 24));
        assert_ne!(turned, paint("mandelbrot", &dive(), 24, 24));
    }
}
