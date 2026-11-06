use crate::ink::hsl;
use crate::sponge::digest;
use mrly_color::Color;
use mrly_error::{Result, value_error};
use mrly_image::Image;
use mrly_image::sample::Filter;
use serde::{Deserialize, Serialize};

const SIDE: usize = 7;
const HALF: usize = SIDE.div_ceil(2);
const MARGIN: usize = 1;
const CANVAS: usize = SIDE + 2 * MARGIN;
const LEAST: usize = 12;
const MOST: usize = 37;
const TRIES: usize = 64;
const SATURATION: [usize; 3] = [55, 65, 75];
const LIGHTNESS: [usize; 3] = [52, 58, 64];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Picture {
    pub cells: Vec<bool>,
    pub ink: Color,
    pub ground: Color,
}

fn cells(digest: &[u8]) -> Vec<bool> {
    let mut cells = vec![false; SIDE * SIDE];
    for row in 0..SIDE {
        for col in 0..HALF {
            let k = row * HALF + col;
            let on = (digest[k / 8] >> (7 - k % 8)) & 1 == 1;
            cells[row * SIDE + col] = on;
            cells[row * SIDE + SIDE - 1 - col] = on;
        }
    }
    cells
}

pub fn picture(bytes: &[u8]) -> Picture {
    let mut hashed = digest(bytes);
    let mut cells = self::cells(&hashed);
    for _ in 0..TRIES {
        if (LEAST..=MOST).contains(&cells.iter().filter(|&&on| on).count()) {
            break;
        }
        hashed = digest(&hashed);
        cells = self::cells(&hashed);
    }
    let hue = usize::from(u16::from_be_bytes([hashed[4], hashed[5]])) % 360;
    let tone = usize::from(hashed[6]);
    Picture {
        cells,
        ink: hsl(hue, SATURATION[tone % 3], LIGHTNESS[tone / 3 % 3]),
        ground: hsl(hue, 25, 10),
    }
}

pub fn image(name: &str, size: usize) -> Result<Image> {
    if !(1..=4096).contains(&size) {
        return value_error("size must be from 1 to 4096.");
    }
    let picture = picture(name.as_bytes());
    let paint = |color: Color| [color.r, color.g, color.b, color.a];
    let mut colors = vec![paint(picture.ground); CANVAS * CANVAS];
    for (flat, _) in picture.cells.iter().enumerate().filter(|(_, on)| **on) {
        let (row, col) = (flat / SIDE + MARGIN, flat % SIDE + MARGIN);
        colors[row * CANVAS + col] = paint(picture.ink);
    }
    Image::new(CANVAS, CANVAS, colors)?.resample(size, size, Filter::Nearest)
}

pub fn svg(name: &str, size: usize) -> Result<String> {
    if size < 1 {
        return value_error("size must be at least 1.");
    }
    let picture = picture(name.as_bytes());
    let mut path = String::new();
    for row in 0..SIDE {
        let mut col = 0;
        while col < SIDE {
            if !picture.cells[row * SIDE + col] {
                col += 1;
                continue;
            }
            let start = col;
            while col < SIDE && picture.cells[row * SIDE + col] {
                col += 1;
            }
            path.push_str(&format!(
                "M{} {}h{}v1h-{}z",
                start + MARGIN,
                row + MARGIN,
                col - start,
                col - start
            ));
        }
    }
    Ok(format!(
        "<svg width=\"{size}\" height=\"{size}\" viewBox=\"0 0 {CANVAS} {CANVAS}\" shape-rendering=\"crispEdges\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"{CANVAS}\" height=\"{CANVAS}\" fill=\"{}\"/>\n<path fill=\"{}\" d=\"{path}\"/>\n</svg>\n",
        picture.ground.to_hex(),
        picture.ink.to_hex()
    ))
}
