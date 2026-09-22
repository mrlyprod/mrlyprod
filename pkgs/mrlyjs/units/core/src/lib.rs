#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Flips every type to one minus itself, same as invert.
#[wasm_bindgen]
pub fn cell_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.anti();
    hand::cell_to_js(&value)
}

/// Maps every type to one at or above the threshold and zero below, dropping colors.
#[wasm_bindgen]
pub fn cell_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell_to_js(&value)
}

/// Binarizes the types at Otsu's threshold, dropping colors.
#[wasm_bindgen]
pub fn cell_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell_to_js(&value)
}

/// Replaces every type with the rounded mean of its masked neighborhood, dropping colors.
#[wasm_bindgen]
pub fn cell_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the painted color at a flat index, or transparent while unpainted or past the end.
#[wasm_bindgen]
pub fn cell_color_at(cell: JsValue, flat: usize) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.color_at(flat);
    Ok(value.to_vec())
}

/// Builds the Kronecker product of the two cells' types.
#[wasm_bindgen]
pub fn cell_combine(cell: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let other = hand::cell_from_js(&other)?;
    let value = cell.combine(&other);
    hand::cell_to_js(&value)
}

/// Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.
#[wasm_bindgen]
pub fn cell_fractal(cell: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.fractal(level).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Flips every type to one minus itself.
#[wasm_bindgen]
pub fn cell_invert(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.invert();
    hand::cell_to_js(&value)
}

/// Tags every cell with its concentric shell distance from the center.
#[wasm_bindgen]
pub fn cell_layers(cell: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = cell.layers(dtype);
    hand::cell_to_js(&value)
}

/// Folds at least two cells into one by chained Kronecker products.
#[wasm_bindgen]
pub fn cell_magic(cells: JsValue) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::magic(&cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.
#[wasm_bindgen]
pub fn cell_mapping() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::cell::mapping();
    hand::map_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(hand::color_to_js(*x2))))
}

/// Stitches same-shaped cells into one grid of reps blocks per axis.
#[wasm_bindgen]
pub fn cell_merge(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::merge(&cells, reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Builds the 3-wide Moore mask of the dimension, every site on but the center.
#[wasm_bindgen]
pub fn cell_moore(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::cell::moore(dimension);
    hand::tensor_to_js(&value)
}

/// Lays the cell each mask entry indexes into that entry's place and merges the lot.
#[wasm_bindgen]
pub fn cell_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::mosaic(&mask, &cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Tags every cell with its count of target-valued neighbors under the mask.
#[wasm_bindgen]
pub fn cell_neighbors(cell: JsValue, mask: JsValue, target: u8, wrap: bool, dtype: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = cell.neighbors(&mask, target, wrap, dtype).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Wraps a tensor of types in a bare cell, colorless and tagless.
#[wasm_bindgen]
pub fn cell_new(types: JsValue) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let value = mrlyrs::core::Cell::new(types);
    hand::cell_to_js(&value)
}

/// Wraps the cell in count layers of value on every side, dropping colors.
#[wasm_bindgen]
pub fn cell_pad(cell: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.pad(count, value);
    hand::cell_to_js(&value)
}

/// Colors every mapped cell, picking within each type's palette by the mode.
#[wasm_bindgen]
pub fn cell_paint(cell: JsValue, mapping: JsValue, mode: JsValue, rng: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mapping = hand::map_from_js::<u8, _>(&mapping, |x1| hand::list_from_js(x1, hand::color_from_js))?;
    let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = cell.paint(&mapping, mode, rng_stream.as_mut()).map_err(hand::throw)?;
    if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
    hand::cell_to_js(&value)
}

/// Stamps value wherever the tiled mask is on, dropping colors.
#[wasm_bindgen]
pub fn cell_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map.
#[wasm_bindgen]
pub fn cell_remap(cell: JsValue, map: &[usize], shape: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = mrlyrs::core::cell::remap(&cell, map, shape).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else.
#[wasm_bindgen]
pub fn cell_rgba(cell: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.rgba();
    Ok(value)
}

/// Builds the flat source index of every destination cell after k quarter turns in the plane of the axes.
#[wasm_bindgen]
pub fn cell_rot90_map(shape: &[usize], k: usize, axes: JsValue) -> Result<Vec<usize>, JsValue> {
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = mrlyrs::core::cell::rot90_map(shape, k, axes).map_err(hand::throw)?;
    Ok(value)
}

/// Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along.
#[wasm_bindgen]
pub fn cell_rotate(cell: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = cell.rotate(k, axes).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the shape of the type tensor.
#[wasm_bindgen]
pub fn cell_shape(cell: JsValue) -> Result<Vec<usize>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.shape();
    Ok(value.to_vec())
}

/// Returns the number of cells.
#[wasm_bindgen]
pub fn cell_size(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.size();
    Ok(value)
}

/// Repeats the cell reps times along each axis, carrying colors and tags along.
#[wasm_bindgen]
pub fn cell_tile(cell: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.tile(reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Builds the flat source index of every destination cell after tiling reps copies per axis.
#[wasm_bindgen]
pub fn cell_tile_map(shape: &[usize], reps: &[usize]) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::core::cell::tile_map(shape, reps).map_err(hand::throw)?;
    Ok(value)
}

/// Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block.
#[wasm_bindgen]
pub fn codec_gif(frames: JsValue, palette: JsValue, width: usize, height: usize, scale: usize, delay: usize) -> Result<Vec<u8>, JsValue> {
    let frames = hand::from_js::<Vec<Vec<u8>>>(&frames)?;
    let frames_view: Vec<&[u8]> = frames.iter().map(Vec::as_slice).collect();
    let palette = hand::from_js::<Vec<[u8; 4]>>(&palette)?;
    let value = mrlyrs::core::codec::gif(&frames_view, &palette, width, height, scale, delay).map_err(hand::throw)?;
    Ok(value)
}

/// Encodes rgba colors as a png, drawing each source pixel as a scale by scale block.
#[wasm_bindgen]
pub fn codec_png(colors: JsValue, width: usize, height: usize, scale: usize) -> Result<Vec<u8>, JsValue> {
    let colors = hand::from_js::<Vec<[u8; 4]>>(&colors)?;
    let value = mrlyrs::core::codec::png(&colors, width, height, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the color with its alpha set to level.
#[wasm_bindgen]
pub fn colors_alpha(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.alpha(level);
    Ok(hand::color_to_js(value))
}

/// Returns the ground rgba of the dark or the light theme.
#[wasm_bindgen]
pub fn colors_board(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::colors::board(dark);
    Ok(value.to_vec())
}

/// Formats the color as a css rgb or rgba call.
#[wasm_bindgen]
pub fn colors_css(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.css();
    Ok(value)
}

/// Parses a #RRGGBB or #RRGGBBAA code, hash optional.
#[wasm_bindgen]
pub fn colors_from_hex(hex: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::from_hex(hex).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Builds a gradient of steps colors sweeping evenly through the given stops.
#[wasm_bindgen]
pub fn colors_gradient(colors: JsValue, steps: usize) -> Result<JsValue, JsValue> {
    let colors = hand::list_from_js(&colors, hand::color_from_js)?;
    let value = mrlyrs::core::colors::gradient(&colors, steps).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// Returns the foreground rgba of the dark or the light theme.
#[wasm_bindgen]
pub fn colors_ink(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::colors::ink(dark);
    Ok(value.to_vec())
}

/// Returns the color with every channel flipped and the alpha kept.
#[wasm_bindgen]
pub fn colors_invert(color: JsValue) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.invert();
    Ok(hand::color_to_js(value))
}

/// Returns the color scaled toward black below level 50 and toward white above.
#[wasm_bindgen]
pub fn colors_lightness(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.lightness(level).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level.
#[wasm_bindgen]
pub fn colors_luma_types(pixels: JsValue, width: usize, height: usize, level: u8) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrlyrs::core::colors::luma_types(&pixels, width, height, level);
    hand::tensor_to_js(&value)
}

/// Blends two colors linearly by ratio.
#[wasm_bindgen]
pub fn colors_mix(color_1: JsValue, color_2: JsValue, ratio: f64) -> Result<JsValue, JsValue> {
    let color_1 = hand::color_from_js(&color_1)?;
    let color_2 = hand::color_from_js(&color_2)?;
    let value = mrlyrs::core::colors::mix(color_1, color_2, ratio).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Returns the palette color a name spells.
#[wasm_bindgen]
pub fn colors_named(name: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::named(name).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Draws a color from the stream, opaque unless alpha is asked for.
#[wasm_bindgen]
pub fn colors_random(alpha: bool, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::random(alpha, rng.stream());
    Ok(hand::color_to_js(value))
}

/// Builds an opaque color.
#[wasm_bindgen]
pub fn colors_rgb(r: u8, g: u8, b: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::rgb(r, g, b);
    Ok(hand::color_to_js(value))
}

/// Builds a color with an explicit alpha.
#[wasm_bindgen]
pub fn colors_rgba(r: u8, g: u8, b: u8, a: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::rgba(r, g, b, a);
    Ok(hand::color_to_js(value))
}

/// Returns a hue one shade lighter, itself, and one shade darker.
#[wasm_bindgen]
pub fn colors_shades(hue: JsValue) -> Result<JsValue, JsValue> {
    let hue = hand::color_from_js(&hue)?;
    let value = mrlyrs::core::colors::shades(hue);
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie.
#[wasm_bindgen]
pub fn colors_snap(pixels: JsValue, palette: JsValue) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let palette = hand::list_from_js(&palette, hand::color_from_js)?;
    let value = mrlyrs::core::colors::snap(&pixels, &palette);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Formats the color as lowercase hex, appending the alpha pair only when not opaque.
#[wasm_bindgen]
pub fn colors_to_hex(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.to_hex();
    Ok(value)
}

/// Parses JSON text into a value.
#[wasm_bindgen]
pub fn error_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::error::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.
#[wasm_bindgen]
pub fn hex_fit(pixels: JsValue, width: usize, height: usize, vertical: bool, filter: JsValue) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
    let value = mrlyrs::core::hex_fit(&pixels, width, height, vertical, filter).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[hand::to_js(&value.0)?, hand::to_js(&value.1)?, hand::list_to_js(&value.2, |x2| Ok(hand::typed(&(*x2)[..])))?]))
}

/// Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.
#[wasm_bindgen]
pub fn hex_size(width: usize, height: usize, vertical: bool) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::hex_size(width, height, vertical);
    hand::to_js(&value)
}

/// Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.
#[wasm_bindgen]
pub fn image_blur(pixels: JsValue, width: usize, height: usize, radius: usize) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrlyrs::core::image::blur(&pixels, width, height, radius);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.
#[wasm_bindgen]
pub fn paint_apply(paint: &paint_Paint, cell: JsValue, rng: &mut hand::Rng) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    mrlyrs::core::paint::apply(&paint.inner, &mut cell_value, rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

/// Replays a stored paint onto a cell, tagging first and applying it from the stream.
#[wasm_bindgen]
pub fn paint_coat(cell: JsValue, paint: &paint_Paint, mask: JsValue, rng: &mut hand::Rng) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    mrlyrs::core::paint::coat(&mut cell_value, &paint.inner, mask.as_ref(), rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

/// Draws a random paint under the config, applies it to the cell, and returns the recipe.
#[wasm_bindgen]
pub fn paint_paint(cell: JsValue, config: JsValue, mask: JsValue, rng: &mut hand::Rng) -> Result<paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let config = hand::from_js::<mrlyrs::core::paint::Config>(&config)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::paint(&mut cell_value, &config, mask.as_ref(), rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(paint_Paint { inner: value })
}

/// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
#[wasm_bindgen]
pub fn paint_prime(paint: &paint_Paint, cell: JsValue, mask: JsValue, rng: &mut hand::Rng) -> Result<paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::prime(paint.inner.clone(), &mut cell_value, mask.as_ref(), rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(paint_Paint { inner: value })
}

/// Draws a random edition from the allowed list, or from all seven.
#[wasm_bindgen]
pub fn paint_random_edition(editions: JsValue, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let editions = hand::from_js::<Option<Vec<mrlyrs::core::paint::Edition>>>(&editions)?;
    let value = mrlyrs::core::paint::random_edition(editions.as_deref(), rng.stream());
    hand::to_js(&value)
}

/// Redraws the paint's secondary inks and shades under its scheme.
#[wasm_bindgen]
pub fn paint_reroll(paint: &paint_Paint, rng: &mut hand::Rng) -> Result<paint_Paint, JsValue> {
    let value = mrlyrs::core::paint::reroll(paint.inner.clone(), rng.stream());
    Ok(paint_Paint { inner: value })
}

/// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
#[wasm_bindgen]
pub fn paint_setup(paint: &paint_Paint, config: JsValue, rng: &mut hand::Rng) -> Result<paint_Paint, JsValue> {
    let config = hand::from_js::<mrlyrs::core::paint::Config>(&config)?;
    let value = mrlyrs::core::paint::setup(paint.inner.clone(), &config, rng.stream());
    Ok(paint_Paint { inner: value })
}

/// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
#[wasm_bindgen]
pub fn paint_tag(cell: JsValue, edition: JsValue, target: JsValue, mask: JsValue) -> Result<usize, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let edition = hand::from_js::<mrlyrs::core::paint::Edition>(&edition)?;
    let target = hand::from_js::<mrlyrs::core::paint::Target>(&target)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::tag(&mut cell_value, edition, target, mask.as_ref()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(value)
}

/// Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.
#[wasm_bindgen]
pub fn ramp_color(colorizer: JsValue, value: usize, max: usize) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value = mrlyrs::core::ramp::color(&colorizer, value, max);
    Ok(hand::color_to_js(value))
}

/// Maps a slice of values to rgba pixels against the range maximum.
#[wasm_bindgen]
pub fn ramp_colors(colorizer: JsValue, values: &[usize], max: usize) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value = mrlyrs::core::ramp::colors(&colorizer, values, max);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Resamples rgba pixels to a new size.
#[wasm_bindgen]
pub fn resample(pixels: JsValue, width: usize, height: usize, out_w: usize, out_h: usize, filter: JsValue) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
    let value = mrlyrs::core::resample(&pixels, width, height, out_w, out_h, filter).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Draws an integer below n, or zero when n is zero.
#[wasm_bindgen]
pub fn rng_below_(rng: &mut hand::Rng, n: usize) -> Result<usize, JsValue> {
    let value = rng.stream().below(n);
    Ok(value)
}

/// Draws a fair coin flip.
#[wasm_bindgen]
pub fn rng_boolean_(rng: &mut hand::Rng) -> Result<bool, JsValue> {
    let value = rng.stream().boolean();
    Ok(value)
}

/// Returns true with probability p.
#[wasm_bindgen]
pub fn rng_chance_(rng: &mut hand::Rng, p: f64) -> Result<bool, JsValue> {
    let value = rng.stream().chance(p);
    Ok(value)
}

/// Builds the stream from a seed.
#[wasm_bindgen]
pub fn rng_new_(seed: JsValue) -> Result<hand::Rng, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrlyrs::core::Rng::new(seed);
    Ok(hand::Rng::wrap(value))
}

/// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
#[wasm_bindgen]
pub fn rng_range_(rng: &mut hand::Rng, lo: JsValue, hi: JsValue) -> Result<i64, JsValue> {
    let lo = hand::i64_from_js(&lo)?;
    let hi = hand::i64_from_js(&hi)?;
    let value = rng.stream().range(lo, hi);
    Ok(value)
}

/// Draws amount distinct indices below length, or every index when amount is larger.
#[wasm_bindgen]
pub fn rng_sample_indices_(rng: &mut hand::Rng, length: usize, amount: usize) -> Result<Vec<usize>, JsValue> {
    let value = rng.stream().sample_indices(length, amount);
    Ok(value)
}

/// Draws a float at or above zero and below one.
#[wasm_bindgen]
pub fn rng_unit_(rng: &mut hand::Rng) -> Result<f64, JsValue> {
    let value = rng.stream().unit();
    Ok(value)
}

/// Returns the element at a flat index, which must be below the size like a slice index.
#[wasm_bindgen]
pub fn tensor_at(tensor: JsValue, flat: usize) -> Result<i64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.at(flat);
    Ok(value)
}

/// Maps every element to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn tensor_binarize(tensor: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize(threshold);
    hand::tensor_to_js(&value)
}

/// Binarizes at one above the Otsu threshold.
#[wasm_bindgen]
pub fn tensor_binarize_otsu(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize_otsu();
    hand::tensor_to_js(&value)
}

/// Averages every position over its masked neighborhood, rounded.
#[wasm_bindgen]
pub fn tensor_blur(tensor: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = tensor.blur(&mask, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as bytes.
#[wasm_bindgen]
pub fn tensor_bytes(tensor: JsValue) -> Result<Vec<u8>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.bytes().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Counts the cells holding one value.
#[wasm_bindgen]
pub fn tensor_count(tensor: JsValue, value: u8) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.count(value);
    Ok(value)
}

/// Returns the element width.
#[wasm_bindgen]
pub fn tensor_dtype(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.dtype();
    hand::to_js(&value)
}

/// Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.
#[wasm_bindgen]
pub fn tensor_exposed(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.exposed();
    Ok(JsValue::from_str(&value.to_string()))
}

/// Builds a tensor of the shape and width filled with one value.
#[wasm_bindgen]
pub fn tensor_filled(shape: Vec<usize>, value: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::i64_from_js(&value)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = mrlyrs::core::Tensor::filled(shape, value, dtype);
    hand::tensor_to_js(&value)
}

/// Reverses the tensor along one axis.
#[wasm_bindgen]
pub fn tensor_flip(tensor: JsValue, axis: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.flip(axis).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Folds the tensor into its level-fold Kronecker power.
#[wasm_bindgen]
pub fn tensor_fractal(tensor: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.fractal(level);
    hand::tensor_to_js(&value)
}

/// Builds a u8 tensor filled with one value.
#[wasm_bindgen]
pub fn tensor_full(shape: Vec<usize>, value: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::full(shape, value);
    hand::tensor_to_js(&value)
}

/// Returns the byte at a multi-index.
#[wasm_bindgen]
pub fn tensor_get(tensor: JsValue, multi: &[usize]) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.get(multi).map_err(hand::throw)?;
    Ok(value)
}

/// Wraps an i32 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn tensor_i32(data: Vec<i32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::i32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as i32s.
#[wasm_bindgen]
pub fn tensor_i32s(tensor: JsValue) -> Result<Vec<i32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.i32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Folds a multi-index into its flat index.
#[wasm_bindgen]
pub fn tensor_index(tensor: JsValue, multi: &[usize]) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.index(multi);
    Ok(value)
}

/// Flips every element between zero and one.
#[wasm_bindgen]
pub fn tensor_invert(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.invert();
    hand::tensor_to_js(&value)
}

/// Builds the Kronecker product of the two tensors.
#[wasm_bindgen]
pub fn tensor_kron(tensor: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let other = hand::tensor_from_js(&other)?;
    let value = tensor.kron(&other);
    hand::tensor_to_js(&value)
}

/// Numbers every position by its concentric ring out from the center.
#[wasm_bindgen]
pub fn tensor_layers(tensor: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = tensor.layers(dtype);
    hand::tensor_to_js(&value)
}

/// Counts each position's masked neighbors holding the target bit.
#[wasm_bindgen]
pub fn tensor_neighbors(tensor: JsValue, mask: JsValue, target: u8, wrap: bool, dtype: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = tensor.neighbors(&mask, target, wrap, dtype).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds a zeroed u8 tensor of the shape.
#[wasm_bindgen]
pub fn tensor_new(shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::new(shape);
    hand::tensor_to_js(&value)
}

/// Wraps a byte vector as a tensor of the shape.
#[wasm_bindgen]
pub fn tensor_of(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::of(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the Otsu threshold splitting the histogram at greatest variance.
#[wasm_bindgen]
pub fn tensor_otsu_threshold(tensor: JsValue) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.otsu_threshold();
    Ok(value)
}

/// Wraps the tensor in a count-thick border of one value.
#[wasm_bindgen]
pub fn tensor_pad(tensor: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.pad(count, value);
    hand::tensor_to_js(&value)
}

/// Stamps the value wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn tensor_perforate(tensor: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = tensor.perforate(&mask, value).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Writes the element at a flat index, which must be below the size like a slice index.
#[wasm_bindgen]
pub fn tensor_put(tensor: JsValue, flat: usize, value: JsValue) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    let value = hand::i64_from_js(&value)?;
    tensor_value.put(flat, value);
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

/// Rotates the tensor k quarter turns in the plane of two axes.
#[wasm_bindgen]
pub fn tensor_rot90(tensor: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = tensor.rot90(k, axes).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Writes the byte at a multi-index.
#[wasm_bindgen]
pub fn tensor_set(tensor: JsValue, multi: &[usize], value: u8) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    tensor_value.set(multi, value).map_err(hand::throw)?;
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

/// Returns the number of elements.
#[wasm_bindgen]
pub fn tensor_size(tensor: JsValue) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.size();
    Ok(value)
}

/// Drops one axis by fixing it at an index.
#[wasm_bindgen]
pub fn tensor_slice(tensor: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.slice(axis, index).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the sum of all elements.
#[wasm_bindgen]
pub fn tensor_sum(tensor: JsValue) -> Result<u64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.sum();
    Ok(value)
}

/// Repeats the tensor the given number of times along each axis.
#[wasm_bindgen]
pub fn tensor_tile(tensor: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.tile(reps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Swaps two axes.
#[wasm_bindgen]
pub fn tensor_transpose(tensor: JsValue, a: usize, b: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.transpose(a, b).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds a zeroed tensor of the shape and width.
#[wasm_bindgen]
pub fn tensor_typed(shape: Vec<usize>, dtype: JsValue) -> Result<JsValue, JsValue> {
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = mrlyrs::core::Tensor::typed(shape, dtype);
    hand::tensor_to_js(&value)
}

/// Wraps a u16 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn tensor_u16(data: Vec<u16>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u16(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as u16s.
#[wasm_bindgen]
pub fn tensor_u16s(tensor: JsValue) -> Result<Vec<u16>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u16s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Wraps a u32 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn tensor_u32(data: Vec<u32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as u32s.
#[wasm_bindgen]
pub fn tensor_u32s(tensor: JsValue) -> Result<Vec<u32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`].
#[wasm_bindgen]
pub fn tensor_u8(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u8(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Decodes a png to its width, height, and rgba colors.
#[wasm_bindgen]
pub fn unpng(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::unpng(bytes).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[hand::to_js(&value.0)?, hand::to_js(&value.1)?, hand::list_to_js(&value.2, |x2| Ok(hand::typed(&(*x2)[..])))?]))
}

/// The height of an equilateral triangle over its side, the squash a hex rendering wears.
#[wasm_bindgen]
pub fn HEX_RATIO() -> Result<f64, JsValue> {
    let value = mrlyrs::core::HEX_RATIO;
    Ok(value)
}

/// The eight bytes every png file starts with.
#[wasm_bindgen]
pub fn PNG_MAGIC() -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::PNG_MAGIC;
    Ok(value.to_vec())
}

/// The fully transparent color.
#[wasm_bindgen]
pub fn colors_ALPHA() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::ALPHA;
    Ok(hand::color_to_js(value))
}

/// The palette black, #000000.
#[wasm_bindgen]
pub fn colors_BLACK() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BLACK;
    Ok(hand::color_to_js(value))
}

/// The palette blue, #008cff.
#[wasm_bindgen]
pub fn colors_BLUE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BLUE;
    Ok(hand::color_to_js(value))
}

/// The palette brown, #b18462.
#[wasm_bindgen]
pub fn colors_BROWN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BROWN;
    Ok(hand::color_to_js(value))
}

/// The palette cyan, #1ec9f3.
#[wasm_bindgen]
pub fn colors_CYAN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::CYAN;
    Ok(hand::color_to_js(value))
}

/// The dark theme.
#[wasm_bindgen]
pub fn colors_DARK() -> Result<colors_Theme, JsValue> {
    let value = mrlyrs::core::colors::DARK;
    Ok(colors_Theme { inner: value })
}

/// The palette gray, #8e8e93.
#[wasm_bindgen]
pub fn colors_GRAY() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::GRAY;
    Ok(hand::color_to_js(value))
}

/// The palette green, #32cc58.
#[wasm_bindgen]
pub fn colors_GREEN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::GREEN;
    Ok(hand::color_to_js(value))
}

/// The palette indigo, #6768fa.
#[wasm_bindgen]
pub fn colors_INDIGO() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::INDIGO;
    Ok(hand::color_to_js(value))
}

/// The light theme.
#[wasm_bindgen]
pub fn colors_LIGHT() -> Result<colors_Theme, JsValue> {
    let value = mrlyrs::core::colors::LIGHT;
    Ok(colors_Theme { inner: value })
}

/// The palette mint, #00d1bb.
#[wasm_bindgen]
pub fn colors_MINT() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::MINT;
    Ok(hand::color_to_js(value))
}

/// The fifteen names, in palette order.
#[wasm_bindgen]
pub fn colors_NAMES() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::NAMES;
    hand::to_js(&value)
}

/// The palette orange, #ff8f2c.
#[wasm_bindgen]
pub fn colors_ORANGE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::ORANGE;
    Ok(hand::color_to_js(value))
}

/// The fifteen colors, in name order.
#[wasm_bindgen]
pub fn colors_PALETTE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PALETTE;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// The palette pink, #ff325a.
#[wasm_bindgen]
pub fn colors_PINK() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PINK;
    Ok(hand::color_to_js(value))
}

/// The palette purple, #d332e9.
#[wasm_bindgen]
pub fn colors_PURPLE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PURPLE;
    Ok(hand::color_to_js(value))
}

/// The palette red, #ff3d40.
#[wasm_bindgen]
pub fn colors_RED() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::RED;
    Ok(hand::color_to_js(value))
}

/// The palette teal, #00cad8.
#[wasm_bindgen]
pub fn colors_TEAL() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::TEAL;
    Ok(hand::color_to_js(value))
}

/// The palette white, #ffffff.
#[wasm_bindgen]
pub fn colors_WHITE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::WHITE;
    Ok(hand::color_to_js(value))
}

/// The palette yellow, #ffd100.
#[wasm_bindgen]
pub fn colors_YELLOW() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::YELLOW;
    Ok(hand::color_to_js(value))
}

/// A paletted image: rows of palette indices and the palette they point into, hex strings in json.
#[wasm_bindgen]
pub struct Image {
    inner: mrlyrs::core::Image,
}

#[wasm_bindgen]
impl Image {
    /// Reads the Image from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Image, JsValue> {
        Ok(Image { inner: hand::from_js(&data)? })
    }
    /// Writes the Image as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The width in pixels.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> Result<usize, JsValue> {
        let value = self.inner.width;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.width = value;
        Ok(())
    }
    /// The height in pixels.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.height = value;
        Ok(())
    }
    /// The palette index of every pixel, row by row.
    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.rows.clone();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    #[wasm_bindgen(setter)]
    pub fn set_rows(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<Vec<usize>>>(&value)?;
        self.inner.rows = value;
        Ok(())
    }
    /// The colors the rows index.
    #[wasm_bindgen(getter)]
    pub fn palette(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.palette.clone();
        hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
    }
    #[wasm_bindgen(setter)]
    pub fn set_palette(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::color_from_js)?;
        self.inner.palette = value;
        Ok(())
    }
    /// Returns the flat rgba pixels, transparent wherever an index misses the palette.
    pub fn colors(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.colors();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Builds a paletted image from raw rgba pixels, growing the palette as new colors appear.
    pub fn from_pixels(width: usize, height: usize, pixels: JsValue) -> Result<Image, JsValue> {
        let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
        let value = mrlyrs::core::Image::from_pixels(width, height, &pixels);
        Ok(Image { inner: value })
    }
    /// Builds an image from its four parts.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, rows: JsValue, palette: JsValue) -> Result<Image, JsValue> {
        let rows = hand::from_js::<Vec<Vec<usize>>>(&rows)?;
        let palette = hand::list_from_js(&palette, hand::color_from_js)?;
        let value = mrlyrs::core::Image::new(width, height, rows, palette);
        Ok(Image { inner: value })
    }
    /// Encodes the image as a png at the given scale.
    pub fn png(&self, scale: usize) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.png(scale).map_err(hand::throw)?;
        Ok(value)
    }
    /// Resamples the image to a new size, its palette rebuilt from the blended pixels.
    pub fn resample(&self, width: usize, height: usize, filter: JsValue) -> Result<Image, JsValue> {
        let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
        let value = self.inner.resample(width, height, filter).map_err(hand::throw)?;
        Ok(Image { inner: value })
    }
}

/// One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both.
#[wasm_bindgen]
pub struct colors_Theme {
    inner: mrlyrs::core::colors::Theme,
}

#[wasm_bindgen]
impl colors_Theme {
    /// Reads the Theme from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<colors_Theme, JsValue> {
        Ok(colors_Theme { inner: hand::from_js(&data)? })
    }
    /// Writes the Theme as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The ground every figure is painted on.
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
    /// The page background, one step off the ground.
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
    /// The raised panel.
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
    /// The sunken well.
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
    /// The hairline between things.
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
    /// The foreground, the strongest tone.
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
    /// The dimmed foreground, for anything secondary.
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
    /// The interactive accent.
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
    /// The tone written on the accent.
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
    /// The red ink.
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
    /// The orange ink.
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
    /// The yellow ink.
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
    /// The green ink.
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
    /// The mint ink.
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
    /// The teal ink.
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
    /// The cyan ink.
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
    /// The blue ink.
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
    /// The indigo ink.
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
    /// The purple ink.
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
    /// The pink ink.
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
    /// The brown ink.
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
    /// The gray ink.
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
    /// The thirteen inks in name order.
    pub fn hues(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.hues();
        hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
    }
    /// The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.
    pub fn inks(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.inks();
        hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
    }
}

/// A complete coloring recipe for one cell.
#[wasm_bindgen]
pub struct paint_Paint {
    inner: mrlyrs::core::paint::Paint,
}

#[wasm_bindgen]
impl paint_Paint {
    /// Reads the Paint from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<paint_Paint, JsValue> {
        Ok(paint_Paint { inner: hand::from_js(&data)? })
    }
    /// Writes the Paint as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The coloring edition.
    #[wasm_bindgen(getter)]
    pub fn edition(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.edition;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_edition(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::core::paint::Edition>(&value)?;
        self.inner.edition = value;
        Ok(())
    }
    /// The secondary color scheme.
    #[wasm_bindgen(getter)]
    pub fn scheme(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.scheme;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_scheme(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::core::paint::Scheme>(&value)?;
        self.inner.scheme = value;
        Ok(())
    }
    /// The side the primary ink lands on.
    #[wasm_bindgen(getter)]
    pub fn target(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.target;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_target(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::core::paint::Target>(&value)?;
        self.inner.target = value;
        Ok(())
    }
    /// The primary ink.
    #[wasm_bindgen(getter)]
    pub fn primary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.primary;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_primary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::core::paint::Ink>(&value)?;
        self.inner.primary = value;
        Ok(())
    }
    /// The secondary inks.
    #[wasm_bindgen(getter)]
    pub fn secondary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.secondary.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_secondary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrlyrs::core::paint::Ink>>(&value)?;
        self.inner.secondary = value;
        Ok(())
    }
    /// The shade indices of a multitone ramp.
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
    /// Returns true for the Simple edition.
    pub fn is_simple(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_simple();
        Ok(value)
    }
    /// Builds a black-primary, fill-target, multicolor paint for an edition.
    #[wasm_bindgen(constructor)]
    pub fn new(edition: JsValue) -> Result<paint_Paint, JsValue> {
        let edition = hand::from_js::<mrlyrs::core::paint::Edition>(&edition)?;
        let value = mrlyrs::core::paint::Paint::new(edition);
        Ok(paint_Paint { inner: value })
    }
}

/// A rule that turns counter values into colors.
#[wasm_bindgen]
pub struct Colorizer {}

#[wasm_bindgen]
impl Colorizer {
    /// Returns the default Colorizer.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::Colorizer::default();
        hand::to_js(&value)
    }
    /// Builds the blue-to-red diverging ramp around a white middle.
    pub fn diverge() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::Colorizer::diverge();
        hand::to_js(&value)
    }
    /// Builds the black-through-ember fire ramp: black, dark red, orange, light yellow.
    pub fn fire() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::Colorizer::fire();
        hand::to_js(&value)
    }
    /// Builds a binned colorizer from a gradient through the given stops.
    pub fn gradient_bins(background: JsValue, colors: JsValue, shades: usize) -> Result<JsValue, JsValue> {
        let background = hand::color_from_js(&background)?;
        let colors = hand::list_from_js(&colors, hand::color_from_js)?;
        let value = mrlyrs::core::Colorizer::gradient_bins(background, &colors, shades).map_err(hand::throw)?;
        hand::to_js(&value)
    }
    /// Builds the white-to-black heat ramp.
    pub fn heat() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::Colorizer::heat();
        hand::to_js(&value)
    }
}

/// The element widths a tensor can hold.
#[wasm_bindgen]
pub struct Dtype {}

#[wasm_bindgen]
impl Dtype {
    /// Returns the largest value the width can hold.
    pub fn max(dtype: JsValue) -> Result<i64, JsValue> {
        let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
        let value = dtype.max();
        Ok(value)
    }
}

/// The constraints a caller may put on a random paint.
#[wasm_bindgen]
pub struct paint_Config {}

#[wasm_bindgen]
impl paint_Config {
    /// Returns the default Config.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Config::default();
        hand::to_js(&value)
    }
}

/// The seven ways a paint distributes its colors over a cell.
#[wasm_bindgen]
pub struct paint_Edition {}

#[wasm_bindgen]
impl paint_Edition {
    /// Returns every Edition in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Edition::all();
        hand::to_js(&value)
    }
    /// Returns the cell-painting mode this edition renders with, or None for Random, which scatters.
    pub fn mode(edition: JsValue) -> Result<JsValue, JsValue> {
        let edition = hand::from_js::<mrlyrs::core::paint::Edition>(&edition)?;
        let value = edition.mode();
        hand::to_js(&value)
    }
}

/// The fifteen named inks a paint draws from.
#[wasm_bindgen]
pub struct paint_Ink {}

#[wasm_bindgen]
impl paint_Ink {
    /// Returns every Ink in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Ink::all();
        hand::to_js(&value)
    }
    /// Returns the ink's color.
    pub fn color(ink: JsValue) -> Result<JsValue, JsValue> {
        let ink = hand::from_js::<mrlyrs::core::paint::Ink>(&ink)?;
        let value = ink.color();
        Ok(hand::color_to_js(value))
    }
}

/// The two ways secondary colors are drawn.
#[wasm_bindgen]
pub struct paint_Scheme {}

#[wasm_bindgen]
impl paint_Scheme {
    /// Returns every Scheme in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Scheme::all();
        hand::to_js(&value)
    }
}

/// The side of the figure the primary ink lands on.
#[wasm_bindgen]
pub struct paint_Target {}

#[wasm_bindgen]
impl paint_Target {
    /// Returns every Target in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Target::all();
        hand::to_js(&value)
    }
}
