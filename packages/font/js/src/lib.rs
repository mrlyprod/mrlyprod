#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_core::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn font_all() -> Result<JsValue, JsValue> {
    let value = mrly_font::all();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_animate(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrly_font::animate(text, pad);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_cycle(write: JsValue, merge: JsValue, hold: usize) -> Result<JsValue, JsValue> {
    let write = hand::from_js::<mrly_font::Anim>(&write)?;
    let merge = hand::from_js::<Vec<Vec<usize>>>(&merge)?;
    let value = mrly_font::cycle(&write, &merge, hold);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_digits() -> Result<JsValue, JsValue> {
    let value = mrly_font::digits();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_draft(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrly_font::draft(&rows);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_extras() -> Result<JsValue, JsValue> {
    let value = mrly_font::extras();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_floor(rows: JsValue) -> Result<usize, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrly_font::floor(&rows);
    Ok(value)
}

#[wasm_bindgen]
pub fn font_glyph(c: char) -> Result<Option<font_Glyph>, JsValue> {
    let value = mrly_font::glyph(c);
    Ok(value.map(|inner| font_Glyph { inner }))
}

#[wasm_bindgen]
pub fn font_lower(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let rows_view: Vec<&str> = rows.iter().map(String::as_str).collect();
    let value = mrly_font::lower(&rows_view);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_lowers() -> Result<JsValue, JsValue> {
    let value = mrly_font::lowers();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_map() -> Result<JsValue, JsValue> {
    let value = mrly_font::map();
    hand::map_to_js(&value, hand::to_js)
}

#[wasm_bindgen]
pub fn font_merge(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrly_font::merge(text, pad);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn font_name_of(c: char) -> Result<String, JsValue> {
    let value = mrly_font::name_of(c);
    Ok(value)
}

#[wasm_bindgen]
pub fn font_path(c: char) -> Result<JsValue, JsValue> {
    let value = mrly_font::path(c);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_paths_penned(c: char) -> Result<JsValue, JsValue> {
    let value = mrly_font::paths::penned(c);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_pens_all() -> Result<JsValue, JsValue> {
    let value = mrly_font::pens::all();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_raster(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_font::raster(text);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn font_specials() -> Result<JsValue, JsValue> {
    let value = mrly_font::specials();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_strokes(c: char) -> Result<JsValue, JsValue> {
    let value = mrly_font::strokes(c);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_supported() -> Result<JsValue, JsValue> {
    let value = mrly_font::supported();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_trim(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrly_font::trim(&rows);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn font_uppers() -> Result<JsValue, JsValue> {
    let value = mrly_font::uppers();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn font_FPS() -> Result<usize, JsValue> {
    let value = mrly_font::FPS;
    Ok(value)
}

#[wasm_bindgen]
pub fn font_HOLD() -> Result<usize, JsValue> {
    let value = mrly_font::HOLD;
    Ok(value)
}

#[wasm_bindgen]
pub struct font_Glyph {
    inner: mrly_font::Glyph,
}

#[wasm_bindgen]
impl font_Glyph {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<font_Glyph, JsValue> {
        Ok(font_Glyph {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn char(&self) -> Result<char, JsValue> {
        let value = self.inner.char;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_char(&mut self, value: char) -> Result<(), JsValue> {
        self.inner.char = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.rows.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_rows(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<String>>(&value)?;
        self.inner.rows = value;
        Ok(())
    }
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(char: char, rows: JsValue) -> Result<font_Glyph, JsValue> {
        let rows = hand::from_js::<Vec<String>>(&rows)?;
        let value = mrly_font::Glyph::new(char, rows);
        Ok(font_Glyph { inner: value })
    }
    pub fn width(&self) -> Result<usize, JsValue> {
        let value = self.inner.width();
        Ok(value)
    }
}
