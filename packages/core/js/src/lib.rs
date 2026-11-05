#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_core::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn core_cell_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.anti();
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_color_at(cell: JsValue, flat: usize) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.color_at(flat);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_cell_combine(cell: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let other = hand::cell_from_js(&other)?;
    let value = cell.combine(&other);
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_fractal(cell: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.fractal(level).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_invert(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.invert();
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_layers(cell: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = cell.layers(dtype);
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_magic(cells: JsValue) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrly_core::cell::magic(&cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_mapping() -> Result<JsValue, JsValue> {
    let value = mrly_core::cell::mapping();
    hand::map_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::color_to_js(*x2)))
    })
}

#[wasm_bindgen]
pub fn core_cell_merge(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrly_core::cell::merge(&cells, reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_moore(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_core::cell::moore(dimension);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrly_core::cell::mosaic(&mask, &cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_neighbors(
    cell: JsValue,
    mask: JsValue,
    target: u8,
    wrap: bool,
    dtype: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = cell
        .neighbors(&mask, target, wrap, dtype)
        .map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_new(types: JsValue) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let value = mrly_core::Cell::new(types);
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_pad(cell: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.pad(count, value);
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_paint(
    cell: JsValue,
    mapping: JsValue,
    mode: JsValue,
    rng: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mapping =
        hand::map_from_js::<u8, _>(&mapping, |x1| hand::list_from_js(x1, hand::color_from_js))?;
    let mode = hand::from_js::<mrly_core::Mode>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = cell
        .paint(&mapping, mode, rng_stream.as_mut())
        .map_err(hand::throw)?;
    if let Some(stream) = &rng_stream {
        hand::stream_to_js(&rng, stream)?;
    }
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_remap(cell: JsValue, map: &[usize], shape: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = mrly_core::cell::remap(&cell, map, shape).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_rgba(cell: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.rgba();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_cell_rot90_map(
    shape: &[usize],
    k: usize,
    axes: JsValue,
) -> Result<Vec<usize>, JsValue> {
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = mrly_core::cell::rot90_map(shape, k, axes).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn core_cell_rotate(cell: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = cell.rotate(k, axes).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_shape(cell: JsValue) -> Result<Vec<usize>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.shape();
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_cell_size(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.size();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_cell_tile(cell: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.tile(reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

#[wasm_bindgen]
pub fn core_cell_tile_map(shape: &[usize], reps: &[usize]) -> Result<Vec<usize>, JsValue> {
    let value = mrly_core::cell::tile_map(shape, reps).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn core_colors_alpha(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.alpha(level);
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_board(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrly_core::colors::board(dark);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_colors_css(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.css();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_colors_from_hex(hex: &str) -> Result<JsValue, JsValue> {
    let value = mrly_core::Color::from_hex(hex).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_gradient(colors: JsValue, steps: usize) -> Result<JsValue, JsValue> {
    let colors = hand::list_from_js(&colors, hand::color_from_js)?;
    let value = mrly_core::colors::gradient(&colors, steps).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

#[wasm_bindgen]
pub fn core_colors_ink(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrly_core::colors::ink(dark);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_colors_invert(color: JsValue) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.invert();
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_lightness(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.lightness(level).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_luma_types(
    pixels: JsValue,
    width: usize,
    height: usize,
    level: u8,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrly_core::colors::luma_types(&pixels, width, height, level);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_colors_mix(color_1: JsValue, color_2: JsValue, ratio: f64) -> Result<JsValue, JsValue> {
    let color_1 = hand::color_from_js(&color_1)?;
    let color_2 = hand::color_from_js(&color_2)?;
    let value = mrly_core::colors::mix(color_1, color_2, ratio).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_named(name: &str) -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::named(name).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_random(alpha: bool, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrly_core::Color::random(alpha, rng.stream());
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_rgb(r: u8, g: u8, b: u8) -> Result<JsValue, JsValue> {
    let value = mrly_core::Color::rgb(r, g, b);
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_rgba(r: u8, g: u8, b: u8, a: u8) -> Result<JsValue, JsValue> {
    let value = mrly_core::Color::rgba(r, g, b, a);
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_shades(hue: JsValue) -> Result<JsValue, JsValue> {
    let hue = hand::color_from_js(&hue)?;
    let value = mrly_core::colors::shades(hue);
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

#[wasm_bindgen]
pub fn core_colors_snap(pixels: JsValue, palette: JsValue) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let palette = hand::list_from_js(&palette, hand::color_from_js)?;
    let value = mrly_core::colors::snap(&pixels, &palette);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn core_colors_to_hex(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.to_hex();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_error_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_core::error::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn core_hex_fit(
    pixels: JsValue,
    width: usize,
    height: usize,
    vertical: bool,
    filter: JsValue,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let filter = hand::from_js::<mrly_core::Filter>(&filter)?;
    let value =
        mrly_core::hex_fit(&pixels, width, height, vertical, filter).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::to_js(&value.1)?,
        hand::list_to_js(&value.2, |x2| Ok(hand::typed(&(*x2)[..])))?,
    ]))
}

#[wasm_bindgen]
pub fn core_hex_size(width: usize, height: usize, vertical: bool) -> Result<JsValue, JsValue> {
    let value = mrly_core::hex_size(width, height, vertical);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn core_image_blur(
    pixels: JsValue,
    width: usize,
    height: usize,
    radius: usize,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrly_core::image::blur(&pixels, width, height, radius);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn core_image_new(width: usize, height: usize, colors: JsValue) -> Result<JsValue, JsValue> {
    let colors = hand::from_js::<Vec<[u8; 4]>>(&colors)?;
    let value = mrly_core::Image::new(width, height, colors).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn core_image_resample(
    image: JsValue,
    width: usize,
    height: usize,
    filter: JsValue,
) -> Result<JsValue, JsValue> {
    let image = hand::image_from_js(&image)?;
    let filter = hand::from_js::<mrly_core::Filter>(&filter)?;
    let value = image.resample(width, height, filter).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn core_image_scale(image: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let image = hand::image_from_js(&image)?;
    let value = image.scale(scale).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn core_name_sha_hex(bytes: &[u8]) -> Result<String, JsValue> {
    let value = mrly_core::name::sha::hex(bytes);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_name_sha_sha256(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_core::name::sha::sha256(bytes);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_name_sha_short(bytes: &[u8]) -> Result<String, JsValue> {
    let value = mrly_core::name::sha::short(bytes);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_paint_apply(
    paint: &core_paint_Paint,
    cell: JsValue,
    rng: &mut hand::Rng,
) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    mrly_core::paint::apply(&paint.inner, &mut cell_value, rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

#[wasm_bindgen]
pub fn core_paint_coat(
    cell: JsValue,
    paint: &core_paint_Paint,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    mrly_core::paint::coat(&mut cell_value, &paint.inner, mask.as_ref(), rng.stream())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

#[wasm_bindgen]
pub fn core_paint_paint(
    cell: JsValue,
    config: JsValue,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let config = hand::from_js::<mrly_core::paint::Config>(&config)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrly_core::paint::paint(&mut cell_value, &config, mask.as_ref(), rng.stream())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(core_paint_Paint { inner: value })
}

#[wasm_bindgen]
pub fn core_paint_prime(
    paint: &core_paint_Paint,
    cell: JsValue,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrly_core::paint::prime(
        paint.inner.clone(),
        &mut cell_value,
        mask.as_ref(),
        rng.stream(),
    )
    .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(core_paint_Paint { inner: value })
}

#[wasm_bindgen]
pub fn core_paint_random_edition(
    editions: JsValue,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let editions = hand::from_js::<Option<Vec<mrly_core::paint::Edition>>>(&editions)?;
    let value = mrly_core::paint::random_edition(editions.as_deref(), rng.stream());
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn core_paint_reroll(
    paint: &core_paint_Paint,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let value = mrly_core::paint::reroll(paint.inner.clone(), rng.stream());
    Ok(core_paint_Paint { inner: value })
}

#[wasm_bindgen]
pub fn core_paint_setup(
    paint: &core_paint_Paint,
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let config = hand::from_js::<mrly_core::paint::Config>(&config)?;
    let value = mrly_core::paint::setup(paint.inner.clone(), &config, rng.stream());
    Ok(core_paint_Paint { inner: value })
}

#[wasm_bindgen]
pub fn core_paint_tag(
    cell: JsValue,
    edition: JsValue,
    target: JsValue,
    mask: JsValue,
) -> Result<usize, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let edition = hand::from_js::<mrly_core::paint::Edition>(&edition)?;
    let target = hand::from_js::<mrly_core::paint::Target>(&target)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrly_core::paint::tag(&mut cell_value, edition, target, mask.as_ref())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn core_ramp_color(colorizer: JsValue, value: usize, max: usize) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrly_core::Colorizer>(&colorizer)?;
    let value = mrly_core::ramp::color(&colorizer, value, max);
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_ramp_colors(
    colorizer: JsValue,
    values: &[usize],
    max: usize,
) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrly_core::Colorizer>(&colorizer)?;
    let value = mrly_core::ramp::colors(&colorizer, values, max);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn core_resample(
    pixels: JsValue,
    width: usize,
    height: usize,
    out_w: usize,
    out_h: usize,
    filter: JsValue,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let filter = hand::from_js::<mrly_core::Filter>(&filter)?;
    let value =
        mrly_core::resample(&pixels, width, height, out_w, out_h, filter).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn core_rng_below(rng: &mut hand::Rng, n: usize) -> Result<usize, JsValue> {
    let value = rng.stream().below(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_rng_boolean(rng: &mut hand::Rng) -> Result<bool, JsValue> {
    let value = rng.stream().boolean();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_rng_chance(rng: &mut hand::Rng, p: f64) -> Result<bool, JsValue> {
    let value = rng.stream().chance(p);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_rng_new(seed: JsValue) -> Result<hand::Rng, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrly_core::Rng::new(seed);
    Ok(hand::Rng::wrap(value))
}

#[wasm_bindgen]
pub fn core_rng_range(rng: &mut hand::Rng, lo: JsValue, hi: JsValue) -> Result<i64, JsValue> {
    let lo = hand::i64_from_js(&lo)?;
    let hi = hand::i64_from_js(&hi)?;
    let value = rng.stream().range(lo, hi);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_rng_sample_indices(
    rng: &mut hand::Rng,
    length: usize,
    amount: usize,
) -> Result<Vec<usize>, JsValue> {
    let value = rng.stream().sample_indices(length, amount);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_rng_unit(rng: &mut hand::Rng) -> Result<f64, JsValue> {
    let value = rng.stream().unit();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_at(tensor: JsValue, flat: usize) -> Result<i64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.at(flat);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_binarize(tensor: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize(threshold);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_binarize_otsu(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize_otsu();
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_blur(tensor: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = tensor.blur(&mask, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_bytes(tensor: JsValue) -> Result<Vec<u8>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.bytes().map_err(hand::throw)?;
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_tensor_count(tensor: JsValue, value: u8) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.count(value);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_dtype(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.dtype();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_exposed(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.exposed();
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn core_tensor_filled(
    shape: Vec<usize>,
    value: JsValue,
    dtype: JsValue,
) -> Result<JsValue, JsValue> {
    let value = hand::i64_from_js(&value)?;
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = mrly_core::Tensor::filled(shape, value, dtype);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_flip(tensor: JsValue, axis: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.flip(axis).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_fractal(tensor: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.fractal(level);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_full(shape: Vec<usize>, value: u8) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::full(shape, value);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_get(tensor: JsValue, multi: &[usize]) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.get(multi).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_i32(data: Vec<i32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::i32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_i32s(tensor: JsValue) -> Result<Vec<i32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.i32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_tensor_index(tensor: JsValue, multi: &[usize]) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.index(multi);
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_invert(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.invert();
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_kron(tensor: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let other = hand::tensor_from_js(&other)?;
    let value = tensor.kron(&other);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_layers(tensor: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = tensor.layers(dtype);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_neighbors(
    tensor: JsValue,
    mask: JsValue,
    target: u8,
    wrap: bool,
    dtype: JsValue,
) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = tensor
        .neighbors(&mask, target, wrap, dtype)
        .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_new(shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::new(shape);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_of(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::of(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_otsu_threshold(tensor: JsValue) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.otsu_threshold();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_pad(tensor: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.pad(count, value);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_perforate(
    tensor: JsValue,
    mask: JsValue,
    value: u8,
) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = tensor.perforate(&mask, value).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_put(tensor: JsValue, flat: usize, value: JsValue) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    let value = hand::i64_from_js(&value)?;
    tensor_value.put(flat, value);
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

#[wasm_bindgen]
pub fn core_tensor_rot90(tensor: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = tensor.rot90(k, axes).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_set(tensor: JsValue, multi: &[usize], value: u8) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    tensor_value.set(multi, value).map_err(hand::throw)?;
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

#[wasm_bindgen]
pub fn core_tensor_size(tensor: JsValue) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.size();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_slice(tensor: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.slice(axis, index).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_sum(tensor: JsValue) -> Result<u64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.sum();
    Ok(value)
}

#[wasm_bindgen]
pub fn core_tensor_tile(tensor: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.tile(reps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_transpose(tensor: JsValue, a: usize, b: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.transpose(a, b).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_typed(shape: Vec<usize>, dtype: JsValue) -> Result<JsValue, JsValue> {
    let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
    let value = mrly_core::Tensor::typed(shape, dtype);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_u16(data: Vec<u16>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::u16(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_u16s(tensor: JsValue) -> Result<Vec<u16>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u16s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_tensor_u32(data: Vec<u32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::u32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_tensor_u32s(tensor: JsValue) -> Result<Vec<u32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn core_tensor_u8(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrly_core::Tensor::u8(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn core_HEX_RATIO() -> Result<f64, JsValue> {
    let value = mrly_core::HEX_RATIO;
    Ok(value)
}

#[wasm_bindgen]
pub fn core_colors_ALPHA() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::ALPHA;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_BLACK() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::BLACK;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_BLUE() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::BLUE;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_BROWN() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::BROWN;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_CYAN() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::CYAN;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_DARK() -> Result<core_colors_Theme, JsValue> {
    let value = mrly_core::colors::DARK;
    Ok(core_colors_Theme { inner: value })
}

#[wasm_bindgen]
pub fn core_colors_GRAY() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::GRAY;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_GREEN() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::GREEN;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_INDIGO() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::INDIGO;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_LIGHT() -> Result<core_colors_Theme, JsValue> {
    let value = mrly_core::colors::LIGHT;
    Ok(core_colors_Theme { inner: value })
}

#[wasm_bindgen]
pub fn core_colors_MINT() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::MINT;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_NAMES() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::NAMES;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn core_colors_ORANGE() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::ORANGE;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_PALETTE() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::PALETTE;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

#[wasm_bindgen]
pub fn core_colors_PINK() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::PINK;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_PURPLE() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::PURPLE;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_RED() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::RED;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_TEAL() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::TEAL;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_WHITE() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::WHITE;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub fn core_colors_YELLOW() -> Result<JsValue, JsValue> {
    let value = mrly_core::colors::YELLOW;
    Ok(hand::color_to_js(value))
}

#[wasm_bindgen]
pub struct core_colors_Theme {
    inner: mrly_core::colors::Theme,
}

#[wasm_bindgen]
impl core_colors_Theme {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<core_colors_Theme, JsValue> {
        Ok(core_colors_Theme {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn ground(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ground;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_ground(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.ground = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn bg(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.bg;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_bg(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.bg = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn panel(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.panel;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_panel(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.panel = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn deep(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.deep;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_deep(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.deep = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn line(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.line;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_line(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.line = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn fg(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.fg;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_fg(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.fg = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.dim;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_dim(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.dim = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn accent(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.accent;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_accent(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.accent = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn on_accent(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.on_accent;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_on_accent(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.on_accent = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn red(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.red;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_red(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.red = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn orange(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.orange;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_orange(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.orange = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn yellow(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.yellow;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_yellow(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.yellow = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn green(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.green;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_green(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.green = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn mint(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mint;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_mint(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.mint = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn teal(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.teal;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_teal(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.teal = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn cyan(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.cyan;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_cyan(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.cyan = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn blue(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.blue;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_blue(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.blue = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn indigo(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.indigo;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_indigo(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.indigo = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn purple(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.purple;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_purple(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.purple = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn pink(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.pink;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_pink(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.pink = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn brown(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.brown;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_brown(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.brown = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn gray(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.gray;
        Ok(hand::color_to_js(value))
    }
    #[wasm_bindgen(setter)]
    pub fn set_gray(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::color_from_js(&value)?;
        self.inner.gray = value;
        Ok(())
    }
    pub fn hues(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.hues();
        hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
    }
    pub fn inks(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.inks();
        hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
    }
}

#[wasm_bindgen]
pub struct core_paint_Paint {
    inner: mrly_core::paint::Paint,
}

#[wasm_bindgen]
impl core_paint_Paint {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<core_paint_Paint, JsValue> {
        Ok(core_paint_Paint {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn edition(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.edition;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_edition(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_core::paint::Edition>(&value)?;
        self.inner.edition = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn scheme(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.scheme;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_scheme(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_core::paint::Scheme>(&value)?;
        self.inner.scheme = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn target(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.target;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_target(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_core::paint::Target>(&value)?;
        self.inner.target = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn primary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.primary;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_primary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_core::paint::Ink>(&value)?;
        self.inner.primary = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn secondary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.secondary.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_secondary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrly_core::paint::Ink>>(&value)?;
        self.inner.secondary = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn shades(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.shades.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_shades(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.shades = value;
        Ok(())
    }
    pub fn is_simple(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_simple();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(edition: JsValue) -> Result<core_paint_Paint, JsValue> {
        let edition = hand::from_js::<mrly_core::paint::Edition>(&edition)?;
        let value = mrly_core::paint::Paint::new(edition);
        Ok(core_paint_Paint { inner: value })
    }
}

#[wasm_bindgen]
pub struct core_Colorizer {}

#[wasm_bindgen]
impl core_Colorizer {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_core::Colorizer::default();
        hand::to_js(&value)
    }
    pub fn diverge() -> Result<JsValue, JsValue> {
        let value = mrly_core::Colorizer::diverge();
        hand::to_js(&value)
    }
    pub fn fire() -> Result<JsValue, JsValue> {
        let value = mrly_core::Colorizer::fire();
        hand::to_js(&value)
    }
    pub fn gradient_bins(
        background: JsValue,
        colors: JsValue,
        shades: usize,
    ) -> Result<JsValue, JsValue> {
        let background = hand::color_from_js(&background)?;
        let colors = hand::list_from_js(&colors, hand::color_from_js)?;
        let value = mrly_core::Colorizer::gradient_bins(background, &colors, shades)
            .map_err(hand::throw)?;
        hand::to_js(&value)
    }
    pub fn heat() -> Result<JsValue, JsValue> {
        let value = mrly_core::Colorizer::heat();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct core_Dtype {}

#[wasm_bindgen]
impl core_Dtype {
    pub fn max(dtype: JsValue) -> Result<i64, JsValue> {
        let dtype = hand::from_js::<mrly_core::Dtype>(&dtype)?;
        let value = dtype.max();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct core_paint_Config {}

#[wasm_bindgen]
impl core_paint_Config {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_core::paint::Config::default();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct core_paint_Edition {}

#[wasm_bindgen]
impl core_paint_Edition {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_core::paint::Edition::all();
        hand::to_js(&value)
    }
    pub fn mode(edition: JsValue) -> Result<JsValue, JsValue> {
        let edition = hand::from_js::<mrly_core::paint::Edition>(&edition)?;
        let value = edition.mode();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct core_paint_Ink {}

#[wasm_bindgen]
impl core_paint_Ink {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_core::paint::Ink::all();
        hand::to_js(&value)
    }
    pub fn color(ink: JsValue) -> Result<JsValue, JsValue> {
        let ink = hand::from_js::<mrly_core::paint::Ink>(&ink)?;
        let value = ink.color();
        Ok(hand::color_to_js(value))
    }
}

#[wasm_bindgen]
pub struct core_paint_Scheme {}

#[wasm_bindgen]
impl core_paint_Scheme {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_core::paint::Scheme::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct core_paint_Target {}

#[wasm_bindgen]
impl core_paint_Target {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_core::paint::Target::all();
        hand::to_js(&value)
    }
}
