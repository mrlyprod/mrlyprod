use crate::draw::{blit, fill_rect, fit};
use mrlycore::colors::{mix, Color, PALETTE};
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;
use mrlymath::crypto::hash::{digest, fingerprint_cell, Config, Digest};
use mrlymath::life::{self, animate};
use mrlymath::two::Cell2d;

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 126;
pub const SCALE: usize = 5;

const BACKGROUND: Color = Color::rgb(13, 17, 23);
const QUIET: f64 = 0.25;
const COLS: usize = 40;
const ROWS: usize = 21;
const CELL: usize = 6;
const MARGIN: usize = 16;
const FIELD_X: usize = 76;
const FIELD: usize = WIDTH - FIELD_X - MARGIN;
const WHITE: [u8; 4] = [255, 255, 255, 255];

pub fn card(route: &str, title: &str) -> Result<Vec<[u8; 4]>> {
    let d = digest(route.as_bytes(), &Config::default())?;
    let (texture_ink, accent_ink) = inks(&d);
    let accent = rgba(accent_ink);
    let mut buf = vec![rgba(BACKGROUND); WIDTH * HEIGHT];
    let grid = texture(&d);
    let live = rgba(mix(BACKGROUND, texture_ink, QUIET)?);
    for r in 0..ROWS {
        for c in 0..COLS {
            if grid[r * COLS + c] & 1 == 1 {
                fill_rect(
                    &mut buf,
                    WIDTH,
                    HEIGHT,
                    c * CELL,
                    r * CELL,
                    CELL,
                    CELL,
                    live,
                );
            }
        }
    }
    fill_rect(&mut buf, WIDTH, HEIGHT, 13, 36, 54, 54, rgba(BACKGROUND));
    let icon: Vec<Vec<u8>> = fingerprint_cell(&d, 12)
        .chunks(12)
        .map(|c| c.to_vec())
        .collect();
    blit(&mut buf, WIDTH, HEIGHT, &icon, MARGIN, 39, 4, accent);
    let (title_rows, title_scale, _) = fit(title, FIELD, &[3, 2]);
    let title_h = title_rows.len() * title_scale;
    let brand_rows = crate::font::raster("mrly.net");
    let brand_h = brand_rows.len();
    let total = title_h + 6 + brand_h;
    let mut y = HEIGHT.saturating_sub(total) / 2;
    blit(
        &mut buf,
        WIDTH,
        HEIGHT,
        &title_rows,
        FIELD_X,
        y,
        title_scale,
        WHITE,
    );
    y += title_h + 6;
    blit(&mut buf, WIDTH, HEIGHT, &brand_rows, FIELD_X, y, 1, accent);
    Ok(buf)
}

pub fn card_png(route: &str, title: &str) -> Result<Vec<u8>> {
    mrlycore::png(&card(route, title)?, WIDTH, HEIGHT, SCALE)
}

fn rgba(c: Color) -> [u8; 4] {
    [c.r, c.g, c.b, c.a]
}

fn inks(d: &Digest) -> (Color, Color) {
    let b = d.to_bytes();
    let b0 = *b.first().unwrap_or(&0) as usize;
    let b1 = *b.get(1).unwrap_or(&0) as usize;
    let ti = 2 + b0 % 13;
    let mut ai = 2 + b1 % 13;
    if ai == ti {
        ai = 2 + (b1 + 7) % 13;
    }
    (PALETTE[ti], PALETTE[ai])
}

fn seed_grid(d: &Digest) -> Vec<u8> {
    let fp = fingerprint_cell(d, ROWS);
    let mut seed = vec![0u8; ROWS * COLS];
    for r in 0..ROWS {
        for c in 0..COLS {
            seed[r * COLS + c] = fp[r * ROWS + (c % ROWS)];
        }
    }
    seed
}

fn texture(d: &Digest) -> Vec<u8> {
    let seed = seed_grid(d);
    let seed_cell = Cell2d::new(Tensor::of(seed.clone(), vec![ROWS, COLS]));
    let mut mask = mrlycore::atoms::carpet_2d(3);
    mask.set(&[1, 1], 0);
    let config = life::Config {
        max_generations: 6,
        ..life::Config::new(Cell2d::new(mask), vec![3], vec![2, 3])
    };
    match animate(&seed_cell, &config) {
        Ok(run) => match run.last() {
            Some(grid) if grid.types().sum() > 0 => grid.types().bytes().to_vec(),
            _ => seed,
        },
        Err(_) => seed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_has_card_dimensions() {
        let raster = card("clock", "clock").unwrap();
        assert_eq!(raster.len(), WIDTH * HEIGHT);
        let png = card_png("clock", "clock").unwrap();
        assert_eq!(&png[0..4], &[137, 80, 78, 71]);
        let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
        let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
        assert_eq!(w, 1200);
        assert_eq!(h, 630);
    }

    #[test]
    fn card_is_deterministic() {
        let a = card_png("moire", "moire").unwrap();
        let b = card_png("moire", "moire").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn distinct_routes_get_distinct_cards() {
        let a = card("clock", "clock").unwrap();
        let b = card("snake", "snake").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn long_title_still_fits() {
        let title = "abcdefghijklmnopqrstuvwxyz0123456789abcd";
        assert_eq!(title.chars().count(), 40);
        let raster = card("clock", title).unwrap();
        assert_eq!(raster.len(), WIDTH * HEIGHT);
        let (_, scale, text) = fit(title, FIELD, &[3, 2]);
        assert_eq!(scale, 2);
        assert!(text.chars().count() < 40);
    }

    #[test]
    fn dead_texture_falls_back_to_seed() {
        let d = Digest { bits: vec![0; 256] };
        assert_eq!(texture(&d), seed_grid(&d));
        assert!(texture(&d).iter().all(|&v| v == 0));
        assert_eq!(card("", "").unwrap().len(), WIDTH * HEIGHT);
    }
}
