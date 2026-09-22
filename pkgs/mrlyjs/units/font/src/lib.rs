#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Builds every glyph in font order: uppers, lowers, digits, extras, specials.
#[wasm_bindgen]
pub fn all() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::all();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.
#[wasm_bindgen]
pub fn animate(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::animate(text, pad);
    hand::to_js(&value)
}

/// Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any.
#[wasm_bindgen]
pub fn cycle(write: JsValue, merge: JsValue, hold: usize) -> Result<JsValue, JsValue> {
    let write = hand::from_js::<mrlyrs::font::Anim>(&write)?;
    let merge = hand::from_js::<Vec<Vec<usize>>>(&merge)?;
    let value = mrlyrs::font::cycle(&write, &merge, hold);
    hand::to_js(&value)
}

/// Builds the ten digit glyphs.
#[wasm_bindgen]
pub fn digits() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::digits();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck.
#[wasm_bindgen]
pub fn draft(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::draft(&rows);
    hand::to_js(&value)
}

/// Builds the punctuation, symbol and arrow glyphs.
#[wasm_bindgen]
pub fn extras() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::extras();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank.
#[wasm_bindgen]
pub fn floor(rows: JsValue) -> Result<usize, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::floor(&rows);
    Ok(value)
}

/// Returns an owned copy of the character's glyph, or None outside the font.
#[wasm_bindgen]
pub fn glyph(c: char) -> Result<Option<Glyph>, JsValue> {
    let value = mrlyrs::font::glyph(c);
    Ok(value.map(|inner| Glyph { inner }))
}

/// Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.
#[wasm_bindgen]
pub fn lower(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let rows_view: Vec<&str> = rows.iter().map(String::as_str).collect();
    let value = mrlyrs::font::lower(&rows_view);
    hand::to_js(&value)
}

/// Builds the twenty-six lowercase glyphs by rounding the uppers' corners.
#[wasm_bindgen]
pub fn lowers() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::lowers();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// Returns the whole font as a map from character to bitmap rows.
#[wasm_bindgen]
pub fn map() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::map();
    hand::map_to_js(&value, hand::to_js)
}

/// Folds the written text's glyphs, frame by frame, into one centered stack.
#[wasm_bindgen]
pub fn merge(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::merge(text, pad);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the character's Unicode name, or a U+ code point label for a character outside the font.
#[wasm_bindgen]
pub fn name_of(c: char) -> Result<String, JsValue> {
    let value = mrlyrs::font::name_of(c);
    Ok(value)
}

/// Flattens the character's strokes into one cell-by-cell drawing order.
#[wasm_bindgen]
pub fn path(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::path(c);
    hand::to_js(&value)
}

/// Returns the character's hand-penned strokes from the pen tables, or None outside the font.
#[wasm_bindgen]
pub fn paths_penned(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::paths::penned(c);
    hand::to_js(&value)
}

/// Returns every pen in font order: uppers, lowers, digits, extras, specials.
#[wasm_bindgen]
pub fn pens_all() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::pens::all();
    hand::to_js(&value)
}

/// Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.
#[wasm_bindgen]
pub fn raster(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::raster(text);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Builds the four seven-row glyphs: dollar, at, copyright and registered.
#[wasm_bindgen]
pub fn specials() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::specials();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.
#[wasm_bindgen]
pub fn strokes(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::strokes(c);
    hand::to_js(&value)
}

/// Returns every character in the font, in font order.
#[wasm_bindgen]
pub fn supported() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::supported();
    hand::to_js(&value)
}

/// Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has.
#[wasm_bindgen]
pub fn trim(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::trim(&rows);
    hand::to_js(&value)
}

/// Builds the twenty-six uppercase glyphs.
#[wasm_bindgen]
pub fn uppers() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::uppers();
    hand::list_to_js(&value, |x1| Ok(JsValue::from(Glyph { inner: x1.clone() })))
}

/// The playback rate of every animation, in frames per second.
#[wasm_bindgen]
pub fn FPS() -> Result<usize, JsValue> {
    let value = mrlyrs::font::FPS;
    Ok(value)
}

/// The default number of frames a cycle rests between movements.
#[wasm_bindgen]
pub fn HOLD() -> Result<usize, JsValue> {
    let value = mrlyrs::font::HOLD;
    Ok(value)
}

/// One character's pixel bitmap.
#[wasm_bindgen]
pub struct Glyph {
    inner: mrlyrs::font::Glyph,
}

#[wasm_bindgen]
impl Glyph {
    /// Reads the Glyph from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Glyph, JsValue> {
        Ok(Glyph { inner: hand::from_js(&data)? })
    }
    /// Writes the Glyph as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The character the glyph draws.
    #[wasm_bindgen(getter)]
    pub fn char(&self) -> Result<char, JsValue> {
        let value = self.inner.char;
        Ok(value)
    }
    /// The bitmap rows of '0' and '1' characters.
    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.rows.clone();
        hand::to_js(&value)
    }
    /// Returns the number of rows.
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height();
        Ok(value)
    }
    /// Builds a glyph from its character and rows.
    #[wasm_bindgen(constructor)]
    pub fn new(char: char, rows: JsValue) -> Result<Glyph, JsValue> {
        let rows = hand::from_js::<Vec<String>>(&rows)?;
        let value = mrlyrs::font::Glyph::new(char, rows);
        Ok(Glyph { inner: value })
    }
    /// Returns the cell width of the first row, or 0 for an empty glyph.
    pub fn width(&self) -> Result<usize, JsValue> {
        let value = self.inner.width();
        Ok(value)
    }
}
