use mrlycore::json;
use wasm_bindgen::prelude::*;

/// Lays the text out as one 0/1 grid: its row count, its column count and its rows as strings of '0' and '1', as JSON.
#[wasm_bindgen]
pub fn font_raster(text: &str) -> String {
    let grid = mrlyfont::raster(text);
    let cols = grid.first().map_or(0, |row| row.len());
    let rows: Vec<String> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|bit| if *bit == 1 { '1' } else { '0' })
                .collect()
        })
        .collect();
    json!({ "rows": grid.len(), "cols": cols, "grid": rows }).to_string()
}

/// Writes the text cell by cell in stroke order on a board padded by pad: the board size, the rate and the lit cell indices of every frame, as JSON.
#[wasm_bindgen]
pub fn font_animate(text: &str, pad: usize) -> String {
    anim_json(&mrlyfont::animate(text, pad))
}

/// Loops the text's write-and-fold cycle, holding hold frames between the movements, in the same shape as font_animate.
#[wasm_bindgen]
pub fn font_cycle(text: &str, pad: usize, hold: usize) -> String {
    let write = mrlyfont::animate(text, pad);
    let merged = mrlyfont::merge(text, pad);
    anim_json(&mrlyfont::cycle(&write, &merged, hold))
}

/// Returns the least strokes that can write the character, the minimum cover of its cells by 4-adjacent paths, or 0 outside the font.
#[wasm_bindgen]
pub fn font_floor(c: &str) -> usize {
    c.chars()
        .next()
        .and_then(mrlyfont::glyph)
        .map_or(0, |glyph| mrlyfont::floor(&mrlyfont::trim(&glyph.rows)))
}

/// Returns every character the font supports, in font order, as one string.
#[wasm_bindgen]
pub fn font_chars() -> String {
    mrlyfont::supported().into_iter().collect()
}

fn anim_json(anim: &mrlyfont::Anim) -> String {
    json!({
        "rows": anim.rows,
        "cols": anim.cols,
        "fps": anim.fps,
        "frames": anim.frames,
    })
    .to_string()
}
