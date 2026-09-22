mod hand;

use mrlyrs::core::colors::Color;
use mrlyrs::gen;
use mrlyrs::math::two;
use mrlyrs::num::factor;
use wasm_bindgen::prelude::*;

// PROOF

/// Turns the tensor a quarter turn k times in the plane of two axes.
#[wasm_bindgen]
pub fn tensor_rot90(tensor: JsValue, k: usize, a: usize, b: usize) -> Result<JsValue, JsValue> {
    let turned = hand::tensor_from_js(&tensor)?
        .rot90(k, (a, b))
        .map_err(hand::throw)?;
    hand::tensor_to_js(&turned)
}

/// Builds the carpet fractal of the number, deepened to the level.
#[wasm_bindgen]
pub fn two_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let cell = two::carpet(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&cell)
}

/// Takes the cell's full census in one reading.
#[wasm_bindgen]
pub fn two_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let read = two::census(&hand::cell2d_from_js(&cell)?).map_err(hand::throw)?;
    hand::to_js(&read)
}

/// Renders one seeded background of the size as png bytes.
#[wasm_bindgen]
pub fn gen_background(seed: JsValue, width: usize, height: usize) -> Result<Vec<u8>, JsValue> {
    gen::background(hand::u64_from_js(&seed)?, width, height).map_err(hand::throw)
}

/// Returns the greatest common divisor of two decimal strings.
#[wasm_bindgen]
pub fn num_factor_gcd(a: JsValue, b: JsValue) -> Result<String, JsValue> {
    let (a, b) = (hand::u128_from_js(&a)?, hand::u128_from_js(&b)?);
    Ok(hand::u128_to_js(factor::gcd(a, b)))
}

/// Parses a #RRGGBB or #RRGGBBAA code into an rgba array.
#[wasm_bindgen]
pub fn color_from_hex(hex: &str) -> Result<JsValue, JsValue> {
    Ok(hand::color_to_js(
        Color::from_hex(hex).map_err(hand::throw)?,
    ))
}

/// Draws one color from the stream, opaque unless alpha is asked for.
#[wasm_bindgen]
pub fn color_random(alpha: bool, rng: &mut hand::Rng) -> JsValue {
    hand::color_to_js(Color::random(alpha, rng.stream()))
}
