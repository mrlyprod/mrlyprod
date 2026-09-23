#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Flips every type to one minus itself, same as invert.
#[wasm_bindgen]
pub fn core_cell_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.anti();
    hand::cell_to_js(&value)
}

/// Maps every type to one at or above the threshold and zero below, dropping colors.
#[wasm_bindgen]
pub fn core_cell_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell_to_js(&value)
}

/// Binarizes the types at Otsu's threshold, dropping colors.
#[wasm_bindgen]
pub fn core_cell_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell_to_js(&value)
}

/// Replaces every type with the rounded mean of its masked neighborhood, dropping colors.
#[wasm_bindgen]
pub fn core_cell_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the painted color at a flat index, or transparent while unpainted or past the end.
#[wasm_bindgen]
pub fn core_cell_color_at(cell: JsValue, flat: usize) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.color_at(flat);
    Ok(value.to_vec())
}

/// Builds the Kronecker product of the two cells' types.
#[wasm_bindgen]
pub fn core_cell_combine(cell: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let other = hand::cell_from_js(&other)?;
    let value = cell.combine(&other);
    hand::cell_to_js(&value)
}

/// Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.
#[wasm_bindgen]
pub fn core_cell_fractal(cell: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.fractal(level).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Flips every type to one minus itself.
#[wasm_bindgen]
pub fn core_cell_invert(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.invert();
    hand::cell_to_js(&value)
}

/// Tags every cell with its concentric shell distance from the center.
#[wasm_bindgen]
pub fn core_cell_layers(cell: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = cell.layers(dtype);
    hand::cell_to_js(&value)
}

/// Folds at least two cells into one by chained Kronecker products.
#[wasm_bindgen]
pub fn core_cell_magic(cells: JsValue) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::magic(&cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.
#[wasm_bindgen]
pub fn core_cell_mapping() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::cell::mapping();
    hand::map_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::color_to_js(*x2)))
    })
}

/// Stitches same-shaped cells into one grid of reps blocks per axis.
#[wasm_bindgen]
pub fn core_cell_merge(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::merge(&cells, reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Builds the 3-wide Moore mask of the dimension, every site on but the center.
#[wasm_bindgen]
pub fn core_cell_moore(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::cell::moore(dimension);
    hand::tensor_to_js(&value)
}

/// Lays the cell each mask entry indexes into that entry's place and merges the lot.
#[wasm_bindgen]
pub fn core_cell_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cells = hand::list_from_js(&cells, hand::cell_from_js)?;
    let value = mrlyrs::core::cell::mosaic(&mask, &cells).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Tags every cell with its count of target-valued neighbors under the mask.
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
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = cell
        .neighbors(&mask, target, wrap, dtype)
        .map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Wraps a tensor of types in a bare cell, colorless and tagless.
#[wasm_bindgen]
pub fn core_cell_new(types: JsValue) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let value = mrlyrs::core::Cell::new(types);
    hand::cell_to_js(&value)
}

/// Wraps the cell in count layers of value on every side, dropping colors.
#[wasm_bindgen]
pub fn core_cell_pad(cell: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.pad(count, value);
    hand::cell_to_js(&value)
}

/// Colors every mapped cell, picking within each type's palette by the mode.
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
    let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = cell
        .paint(&mapping, mode, rng_stream.as_mut())
        .map_err(hand::throw)?;
    if let Some(stream) = &rng_stream {
        hand::stream_to_js(&rng, stream)?;
    }
    hand::cell_to_js(&value)
}

/// Stamps value wherever the tiled mask is on, dropping colors.
#[wasm_bindgen]
pub fn core_cell_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map.
#[wasm_bindgen]
pub fn core_cell_remap(cell: JsValue, map: &[usize], shape: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = mrlyrs::core::cell::remap(&cell, map, shape).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else.
#[wasm_bindgen]
pub fn core_cell_rgba(cell: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.rgba();
    Ok(value)
}

/// Builds the flat source index of every destination cell after k quarter turns in the plane of the axes.
#[wasm_bindgen]
pub fn core_cell_rot90_map(
    shape: &[usize],
    k: usize,
    axes: JsValue,
) -> Result<Vec<usize>, JsValue> {
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = mrlyrs::core::cell::rot90_map(shape, k, axes).map_err(hand::throw)?;
    Ok(value)
}

/// Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along.
#[wasm_bindgen]
pub fn core_cell_rotate(cell: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = cell.rotate(k, axes).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Returns the shape of the type tensor.
#[wasm_bindgen]
pub fn core_cell_shape(cell: JsValue) -> Result<Vec<usize>, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.shape();
    Ok(value.to_vec())
}

/// Returns the number of cells.
#[wasm_bindgen]
pub fn core_cell_size(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.size();
    Ok(value)
}

/// Repeats the cell reps times along each axis, carrying colors and tags along.
#[wasm_bindgen]
pub fn core_cell_tile(cell: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let cell = hand::cell_from_js(&cell)?;
    let value = cell.tile(reps).map_err(hand::throw)?;
    hand::cell_to_js(&value)
}

/// Builds the flat source index of every destination cell after tiling reps copies per axis.
#[wasm_bindgen]
pub fn core_cell_tile_map(shape: &[usize], reps: &[usize]) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::core::cell::tile_map(shape, reps).map_err(hand::throw)?;
    Ok(value)
}

/// Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block.
#[wasm_bindgen]
pub fn core_codec_gif(
    frames: JsValue,
    palette: JsValue,
    width: usize,
    height: usize,
    scale: usize,
    delay: usize,
) -> Result<Vec<u8>, JsValue> {
    let frames = hand::from_js::<Vec<Vec<u8>>>(&frames)?;
    let frames_view: Vec<&[u8]> = frames.iter().map(Vec::as_slice).collect();
    let palette = hand::from_js::<Vec<[u8; 4]>>(&palette)?;
    let value = mrlyrs::core::codec::gif(&frames_view, &palette, width, height, scale, delay)
        .map_err(hand::throw)?;
    Ok(value)
}

/// Encodes rgba colors as a png, drawing each source pixel as a scale by scale block.
#[wasm_bindgen]
pub fn core_codec_png(
    colors: JsValue,
    width: usize,
    height: usize,
    scale: usize,
) -> Result<Vec<u8>, JsValue> {
    let colors = hand::from_js::<Vec<[u8; 4]>>(&colors)?;
    let value = mrlyrs::core::codec::png(&colors, width, height, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the color with its alpha set to level.
#[wasm_bindgen]
pub fn core_colors_alpha(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.alpha(level);
    Ok(hand::color_to_js(value))
}

/// Returns the ground rgba of the dark or the light theme.
#[wasm_bindgen]
pub fn core_colors_board(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::colors::board(dark);
    Ok(value.to_vec())
}

/// Formats the color as a css rgb or rgba call.
#[wasm_bindgen]
pub fn core_colors_css(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.css();
    Ok(value)
}

/// Parses a #RRGGBB or #RRGGBBAA code, hash optional.
#[wasm_bindgen]
pub fn core_colors_from_hex(hex: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::from_hex(hex).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Builds a gradient of steps colors sweeping evenly through the given stops.
#[wasm_bindgen]
pub fn core_colors_gradient(colors: JsValue, steps: usize) -> Result<JsValue, JsValue> {
    let colors = hand::list_from_js(&colors, hand::color_from_js)?;
    let value = mrlyrs::core::colors::gradient(&colors, steps).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// Returns the foreground rgba of the dark or the light theme.
#[wasm_bindgen]
pub fn core_colors_ink(dark: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::colors::ink(dark);
    Ok(value.to_vec())
}

/// Returns the color with every channel flipped and the alpha kept.
#[wasm_bindgen]
pub fn core_colors_invert(color: JsValue) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.invert();
    Ok(hand::color_to_js(value))
}

/// Returns the color scaled toward black below level 50 and toward white above.
#[wasm_bindgen]
pub fn core_colors_lightness(color: JsValue, level: u8) -> Result<JsValue, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.lightness(level).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level.
#[wasm_bindgen]
pub fn core_colors_luma_types(
    pixels: JsValue,
    width: usize,
    height: usize,
    level: u8,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrlyrs::core::colors::luma_types(&pixels, width, height, level);
    hand::tensor_to_js(&value)
}

/// Blends two colors linearly by ratio.
#[wasm_bindgen]
pub fn core_colors_mix(color_1: JsValue, color_2: JsValue, ratio: f64) -> Result<JsValue, JsValue> {
    let color_1 = hand::color_from_js(&color_1)?;
    let color_2 = hand::color_from_js(&color_2)?;
    let value = mrlyrs::core::colors::mix(color_1, color_2, ratio).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Returns the palette color a name spells.
#[wasm_bindgen]
pub fn core_colors_named(name: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::named(name).map_err(hand::throw)?;
    Ok(hand::color_to_js(value))
}

/// Draws a color from the stream, opaque unless alpha is asked for.
#[wasm_bindgen]
pub fn core_colors_random(alpha: bool, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::random(alpha, rng.stream());
    Ok(hand::color_to_js(value))
}

/// Builds an opaque color.
#[wasm_bindgen]
pub fn core_colors_rgb(r: u8, g: u8, b: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::rgb(r, g, b);
    Ok(hand::color_to_js(value))
}

/// Builds a color with an explicit alpha.
#[wasm_bindgen]
pub fn core_colors_rgba(r: u8, g: u8, b: u8, a: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Color::rgba(r, g, b, a);
    Ok(hand::color_to_js(value))
}

/// Returns a hue one shade lighter, itself, and one shade darker.
#[wasm_bindgen]
pub fn core_colors_shades(hue: JsValue) -> Result<JsValue, JsValue> {
    let hue = hand::color_from_js(&hue)?;
    let value = mrlyrs::core::colors::shades(hue);
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie.
#[wasm_bindgen]
pub fn core_colors_snap(pixels: JsValue, palette: JsValue) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let palette = hand::list_from_js(&palette, hand::color_from_js)?;
    let value = mrlyrs::core::colors::snap(&pixels, &palette);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Formats the color as lowercase hex, appending the alpha pair only when not opaque.
#[wasm_bindgen]
pub fn core_colors_to_hex(color: JsValue) -> Result<String, JsValue> {
    let color = hand::color_from_js(&color)?;
    let value = color.to_hex();
    Ok(value)
}

/// Parses JSON text into a value.
#[wasm_bindgen]
pub fn core_error_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::error::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.
#[wasm_bindgen]
pub fn core_hex_fit(
    pixels: JsValue,
    width: usize,
    height: usize,
    vertical: bool,
    filter: JsValue,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
    let value =
        mrlyrs::core::hex_fit(&pixels, width, height, vertical, filter).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::to_js(&value.1)?,
        hand::list_to_js(&value.2, |x2| Ok(hand::typed(&(*x2)[..])))?,
    ]))
}

/// Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.
#[wasm_bindgen]
pub fn core_hex_size(width: usize, height: usize, vertical: bool) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::hex_size(width, height, vertical);
    hand::to_js(&value)
}

/// Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.
#[wasm_bindgen]
pub fn core_image_blur(
    pixels: JsValue,
    width: usize,
    height: usize,
    radius: usize,
) -> Result<JsValue, JsValue> {
    let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
    let value = mrlyrs::core::image::blur(&pixels, width, height, radius);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.
#[wasm_bindgen]
pub fn core_paint_apply(
    paint: &core_paint_Paint,
    cell: JsValue,
    rng: &mut hand::Rng,
) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    mrlyrs::core::paint::apply(&paint.inner, &mut cell_value, rng.stream()).map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

/// Replays a stored paint onto a cell, tagging first and applying it from the stream.
#[wasm_bindgen]
pub fn core_paint_coat(
    cell: JsValue,
    paint: &core_paint_Paint,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<(), JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    mrlyrs::core::paint::coat(&mut cell_value, &paint.inner, mask.as_ref(), rng.stream())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(())
}

/// Draws a random paint under the config, applies it to the cell, and returns the recipe.
#[wasm_bindgen]
pub fn core_paint_paint(
    cell: JsValue,
    config: JsValue,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let config = hand::from_js::<mrlyrs::core::paint::Config>(&config)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::paint(&mut cell_value, &config, mask.as_ref(), rng.stream())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(core_paint_Paint { inner: value })
}

/// Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.
#[wasm_bindgen]
pub fn core_paint_prime(
    paint: &core_paint_Paint,
    cell: JsValue,
    mask: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::prime(
        paint.inner.clone(),
        &mut cell_value,
        mask.as_ref(),
        rng.stream(),
    )
    .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(core_paint_Paint { inner: value })
}

/// Draws a random edition from the allowed list, or from all seven.
#[wasm_bindgen]
pub fn core_paint_random_edition(
    editions: JsValue,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let editions = hand::from_js::<Option<Vec<mrlyrs::core::paint::Edition>>>(&editions)?;
    let value = mrlyrs::core::paint::random_edition(editions.as_deref(), rng.stream());
    hand::to_js(&value)
}

/// Redraws the paint's secondary inks and shades under its scheme.
#[wasm_bindgen]
pub fn core_paint_reroll(
    paint: &core_paint_Paint,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let value = mrlyrs::core::paint::reroll(paint.inner.clone(), rng.stream());
    Ok(core_paint_Paint { inner: value })
}

/// Draws the paint's scheme, target and primary under the config, then rerolls the rest.
#[wasm_bindgen]
pub fn core_paint_setup(
    paint: &core_paint_Paint,
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<core_paint_Paint, JsValue> {
    let config = hand::from_js::<mrlyrs::core::paint::Config>(&config)?;
    let value = mrlyrs::core::paint::setup(paint.inner.clone(), &config, rng.stream());
    Ok(core_paint_Paint { inner: value })
}

/// Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.
#[wasm_bindgen]
pub fn core_paint_tag(
    cell: JsValue,
    edition: JsValue,
    target: JsValue,
    mask: JsValue,
) -> Result<usize, JsValue> {
    let mut cell_value = hand::cell_from_js(&cell)?;
    let edition = hand::from_js::<mrlyrs::core::paint::Edition>(&edition)?;
    let target = hand::from_js::<mrlyrs::core::paint::Target>(&target)?;
    let mask = hand::option_from_js(&mask, hand::tensor_from_js)?;
    let value = mrlyrs::core::paint::tag(&mut cell_value, edition, target, mask.as_ref())
        .map_err(hand::throw)?;
    hand::cell_into_js(&cell, &cell_value)?;
    Ok(value)
}

/// Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.
#[wasm_bindgen]
pub fn core_ramp_color(colorizer: JsValue, value: usize, max: usize) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value = mrlyrs::core::ramp::color(&colorizer, value, max);
    Ok(hand::color_to_js(value))
}

/// Maps a slice of values to rgba pixels against the range maximum.
#[wasm_bindgen]
pub fn core_ramp_colors(
    colorizer: JsValue,
    values: &[usize],
    max: usize,
) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value = mrlyrs::core::ramp::colors(&colorizer, values, max);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Resamples rgba pixels to a new size.
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
    let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
    let value = mrlyrs::core::resample(&pixels, width, height, out_w, out_h, filter)
        .map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Draws an integer below n, or zero when n is zero.
#[wasm_bindgen]
pub fn core_rng_below(rng: &mut hand::Rng, n: usize) -> Result<usize, JsValue> {
    let value = rng.stream().below(n);
    Ok(value)
}

/// Draws a fair coin flip.
#[wasm_bindgen]
pub fn core_rng_boolean(rng: &mut hand::Rng) -> Result<bool, JsValue> {
    let value = rng.stream().boolean();
    Ok(value)
}

/// Returns true with probability p.
#[wasm_bindgen]
pub fn core_rng_chance(rng: &mut hand::Rng, p: f64) -> Result<bool, JsValue> {
    let value = rng.stream().chance(p);
    Ok(value)
}

/// Builds the stream from a seed.
#[wasm_bindgen]
pub fn core_rng_new(seed: JsValue) -> Result<hand::Rng, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrlyrs::core::Rng::new(seed);
    Ok(hand::Rng::wrap(value))
}

/// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
#[wasm_bindgen]
pub fn core_rng_range(rng: &mut hand::Rng, lo: JsValue, hi: JsValue) -> Result<i64, JsValue> {
    let lo = hand::i64_from_js(&lo)?;
    let hi = hand::i64_from_js(&hi)?;
    let value = rng.stream().range(lo, hi);
    Ok(value)
}

/// Draws amount distinct indices below length, or every index when amount is larger.
#[wasm_bindgen]
pub fn core_rng_sample_indices(
    rng: &mut hand::Rng,
    length: usize,
    amount: usize,
) -> Result<Vec<usize>, JsValue> {
    let value = rng.stream().sample_indices(length, amount);
    Ok(value)
}

/// Draws a float at or above zero and below one.
#[wasm_bindgen]
pub fn core_rng_unit(rng: &mut hand::Rng) -> Result<f64, JsValue> {
    let value = rng.stream().unit();
    Ok(value)
}

/// Returns the element at a flat index, which must be below the size like a slice index.
#[wasm_bindgen]
pub fn core_tensor_at(tensor: JsValue, flat: usize) -> Result<i64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.at(flat);
    Ok(value)
}

/// Maps every element to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn core_tensor_binarize(tensor: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize(threshold);
    hand::tensor_to_js(&value)
}

/// Binarizes at one above the Otsu threshold.
#[wasm_bindgen]
pub fn core_tensor_binarize_otsu(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.binarize_otsu();
    hand::tensor_to_js(&value)
}

/// Averages every position over its masked neighborhood, rounded.
#[wasm_bindgen]
pub fn core_tensor_blur(tensor: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = tensor.blur(&mask, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as bytes.
#[wasm_bindgen]
pub fn core_tensor_bytes(tensor: JsValue) -> Result<Vec<u8>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.bytes().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Counts the cells holding one value.
#[wasm_bindgen]
pub fn core_tensor_count(tensor: JsValue, value: u8) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.count(value);
    Ok(value)
}

/// Returns the element width.
#[wasm_bindgen]
pub fn core_tensor_dtype(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.dtype();
    hand::to_js(&value)
}

/// Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.
#[wasm_bindgen]
pub fn core_tensor_exposed(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.exposed();
    Ok(JsValue::from_str(&value.to_string()))
}

/// Builds a tensor of the shape and width filled with one value.
#[wasm_bindgen]
pub fn core_tensor_filled(
    shape: Vec<usize>,
    value: JsValue,
    dtype: JsValue,
) -> Result<JsValue, JsValue> {
    let value = hand::i64_from_js(&value)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = mrlyrs::core::Tensor::filled(shape, value, dtype);
    hand::tensor_to_js(&value)
}

/// Reverses the tensor along one axis.
#[wasm_bindgen]
pub fn core_tensor_flip(tensor: JsValue, axis: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.flip(axis).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Folds the tensor into its level-fold Kronecker power.
#[wasm_bindgen]
pub fn core_tensor_fractal(tensor: JsValue, level: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.fractal(level);
    hand::tensor_to_js(&value)
}

/// Builds a u8 tensor filled with one value.
#[wasm_bindgen]
pub fn core_tensor_full(shape: Vec<usize>, value: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::full(shape, value);
    hand::tensor_to_js(&value)
}

/// Returns the byte at a multi-index.
#[wasm_bindgen]
pub fn core_tensor_get(tensor: JsValue, multi: &[usize]) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.get(multi).map_err(hand::throw)?;
    Ok(value)
}

/// Wraps an i32 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn core_tensor_i32(data: Vec<i32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::i32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as i32s.
#[wasm_bindgen]
pub fn core_tensor_i32s(tensor: JsValue) -> Result<Vec<i32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.i32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Folds a multi-index into its flat index.
#[wasm_bindgen]
pub fn core_tensor_index(tensor: JsValue, multi: &[usize]) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.index(multi);
    Ok(value)
}

/// Flips every element between zero and one.
#[wasm_bindgen]
pub fn core_tensor_invert(tensor: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.invert();
    hand::tensor_to_js(&value)
}

/// Builds the Kronecker product of the two tensors.
#[wasm_bindgen]
pub fn core_tensor_kron(tensor: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let other = hand::tensor_from_js(&other)?;
    let value = tensor.kron(&other);
    hand::tensor_to_js(&value)
}

/// Numbers every position by its concentric ring out from the center.
#[wasm_bindgen]
pub fn core_tensor_layers(tensor: JsValue, dtype: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = tensor.layers(dtype);
    hand::tensor_to_js(&value)
}

/// Counts each position's masked neighbors holding the target bit.
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
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = tensor
        .neighbors(&mask, target, wrap, dtype)
        .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds a zeroed u8 tensor of the shape.
#[wasm_bindgen]
pub fn core_tensor_new(shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::new(shape);
    hand::tensor_to_js(&value)
}

/// Wraps a byte vector as a tensor of the shape.
#[wasm_bindgen]
pub fn core_tensor_of(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::of(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the Otsu threshold splitting the histogram at greatest variance.
#[wasm_bindgen]
pub fn core_tensor_otsu_threshold(tensor: JsValue) -> Result<u8, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.otsu_threshold();
    Ok(value)
}

/// Wraps the tensor in a count-thick border of one value.
#[wasm_bindgen]
pub fn core_tensor_pad(tensor: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.pad(count, value);
    hand::tensor_to_js(&value)
}

/// Stamps the value wherever the tiled mask is nonzero.
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

/// Writes the element at a flat index, which must be below the size like a slice index.
#[wasm_bindgen]
pub fn core_tensor_put(tensor: JsValue, flat: usize, value: JsValue) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    let value = hand::i64_from_js(&value)?;
    tensor_value.put(flat, value);
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

/// Rotates the tensor k quarter turns in the plane of two axes.
#[wasm_bindgen]
pub fn core_tensor_rot90(tensor: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let axes = hand::from_js::<(usize, usize)>(&axes)?;
    let value = tensor.rot90(k, axes).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Writes the byte at a multi-index.
#[wasm_bindgen]
pub fn core_tensor_set(tensor: JsValue, multi: &[usize], value: u8) -> Result<(), JsValue> {
    let mut tensor_value = hand::tensor_from_js(&tensor)?;
    tensor_value.set(multi, value).map_err(hand::throw)?;
    hand::tensor_into_js(&tensor, &tensor_value)?;
    Ok(())
}

/// Returns the number of elements.
#[wasm_bindgen]
pub fn core_tensor_size(tensor: JsValue) -> Result<usize, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.size();
    Ok(value)
}

/// Drops one axis by fixing it at an index.
#[wasm_bindgen]
pub fn core_tensor_slice(tensor: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.slice(axis, index).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the sum of all elements.
#[wasm_bindgen]
pub fn core_tensor_sum(tensor: JsValue) -> Result<u64, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.sum();
    Ok(value)
}

/// Repeats the tensor the given number of times along each axis.
#[wasm_bindgen]
pub fn core_tensor_tile(tensor: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.tile(reps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Swaps two axes.
#[wasm_bindgen]
pub fn core_tensor_transpose(tensor: JsValue, a: usize, b: usize) -> Result<JsValue, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.transpose(a, b).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds a zeroed tensor of the shape and width.
#[wasm_bindgen]
pub fn core_tensor_typed(shape: Vec<usize>, dtype: JsValue) -> Result<JsValue, JsValue> {
    let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
    let value = mrlyrs::core::Tensor::typed(shape, dtype);
    hand::tensor_to_js(&value)
}

/// Wraps a u16 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn core_tensor_u16(data: Vec<u16>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u16(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as u16s.
#[wasm_bindgen]
pub fn core_tensor_u16s(tensor: JsValue) -> Result<Vec<u16>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u16s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Wraps a u32 vector as a tensor of the shape.
#[wasm_bindgen]
pub fn core_tensor_u32(data: Vec<u32>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u32(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the elements as u32s.
#[wasm_bindgen]
pub fn core_tensor_u32s(tensor: JsValue) -> Result<Vec<u32>, JsValue> {
    let tensor = hand::tensor_from_js(&tensor)?;
    let value = tensor.u32s().map_err(hand::throw)?;
    Ok(value.to_vec())
}

/// Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`].
#[wasm_bindgen]
pub fn core_tensor_u8(data: Vec<u8>, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::Tensor::u8(data, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Decodes a png to its width, height, and rgba colors.
#[wasm_bindgen]
pub fn core_unpng(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::unpng(bytes).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::to_js(&value.1)?,
        hand::list_to_js(&value.2, |x2| Ok(hand::typed(&(*x2)[..])))?,
    ]))
}

/// Builds every glyph in font order: uppers, lowers, digits, extras, specials.
#[wasm_bindgen]
pub fn font_all() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::all();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.
#[wasm_bindgen]
pub fn font_animate(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::animate(text, pad);
    hand::to_js(&value)
}

/// Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any.
#[wasm_bindgen]
pub fn font_cycle(write: JsValue, merge: JsValue, hold: usize) -> Result<JsValue, JsValue> {
    let write = hand::from_js::<mrlyrs::font::Anim>(&write)?;
    let merge = hand::from_js::<Vec<Vec<usize>>>(&merge)?;
    let value = mrlyrs::font::cycle(&write, &merge, hold);
    hand::to_js(&value)
}

/// Builds the ten digit glyphs.
#[wasm_bindgen]
pub fn font_digits() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::digits();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck.
#[wasm_bindgen]
pub fn font_draft(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::draft(&rows);
    hand::to_js(&value)
}

/// Builds the punctuation, symbol and arrow glyphs.
#[wasm_bindgen]
pub fn font_extras() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::extras();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank.
#[wasm_bindgen]
pub fn font_floor(rows: JsValue) -> Result<usize, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::floor(&rows);
    Ok(value)
}

/// Returns an owned copy of the character's glyph, or None outside the font.
#[wasm_bindgen]
pub fn font_glyph(c: char) -> Result<Option<font_Glyph>, JsValue> {
    let value = mrlyrs::font::glyph(c);
    Ok(value.map(|inner| font_Glyph { inner }))
}

/// Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.
#[wasm_bindgen]
pub fn font_lower(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let rows_view: Vec<&str> = rows.iter().map(String::as_str).collect();
    let value = mrlyrs::font::lower(&rows_view);
    hand::to_js(&value)
}

/// Builds the twenty-six lowercase glyphs by rounding the uppers' corners.
#[wasm_bindgen]
pub fn font_lowers() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::lowers();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Returns the whole font as a map from character to bitmap rows.
#[wasm_bindgen]
pub fn font_map() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::map();
    hand::map_to_js(&value, hand::to_js)
}

/// Folds the written text's glyphs, frame by frame, into one centered stack.
#[wasm_bindgen]
pub fn font_merge(text: &str, pad: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::merge(text, pad);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the character's Unicode name, or a U+ code point label for a character outside the font.
#[wasm_bindgen]
pub fn font_name_of(c: char) -> Result<String, JsValue> {
    let value = mrlyrs::font::name_of(c);
    Ok(value)
}

/// Flattens the character's strokes into one cell-by-cell drawing order.
#[wasm_bindgen]
pub fn font_path(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::path(c);
    hand::to_js(&value)
}

/// Returns the character's hand-penned strokes from the pen tables, or None outside the font.
#[wasm_bindgen]
pub fn font_paths_penned(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::paths::penned(c);
    hand::to_js(&value)
}

/// Returns every pen in font order: uppers, lowers, digits, extras, specials.
#[wasm_bindgen]
pub fn font_pens_all() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::pens::all();
    hand::to_js(&value)
}

/// Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.
#[wasm_bindgen]
pub fn font_raster(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::raster(text);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Builds the four seven-row glyphs: dollar, at, copyright and registered.
#[wasm_bindgen]
pub fn font_specials() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::specials();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.
#[wasm_bindgen]
pub fn font_strokes(c: char) -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::strokes(c);
    hand::to_js(&value)
}

/// Returns every character in the font, in font order.
#[wasm_bindgen]
pub fn font_supported() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::supported();
    hand::to_js(&value)
}

/// Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has.
#[wasm_bindgen]
pub fn font_trim(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::font::trim(&rows);
    hand::to_js(&value)
}

/// Builds the twenty-six uppercase glyphs.
#[wasm_bindgen]
pub fn font_uppers() -> Result<JsValue, JsValue> {
    let value = mrlyrs::font::uppers();
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(font_Glyph { inner: x1.clone() }))
    })
}

/// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
#[wasm_bindgen]
pub fn gen_background(seed: JsValue, width: usize, height: usize) -> Result<Vec<u8>, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrlyrs::gen::background(seed, width, height).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the flat cell the tile describes.
#[wasm_bindgen]
pub fn gen_build_build_2d(tile: &gen_Tile) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::build_2d(&tile.inner).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the cube the tile describes.
#[wasm_bindgen]
pub fn gen_build_build_3d(tile: &gen_Tile) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::build_3d(&tile.inner).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the tile's cube and flattens it through its projection.
#[wasm_bindgen]
pub fn gen_build_build_6d(hex: JsValue) -> Result<JsValue, JsValue> {
    let hex = hand::from_js::<mrlyrs::gen::build::HexTile>(&hex)?;
    let value = mrlyrs::gen::build::build_6d(&hex).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Draws a random flat tile from the stream, rotations from the four quarter-turns.
#[wasm_bindgen]
pub fn gen_build_create_2d(config: JsValue, rng: &mut hand::Rng) -> Result<gen_Tile, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<2>>(&config)?;
    let value = mrlyrs::gen::build::create_2d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(gen_Tile { inner: value })
}

/// Draws a cube tile from the config with cube orientations drawn from the stream.
#[wasm_bindgen]
pub fn gen_build_create_3d(config: JsValue, rng: &mut hand::Rng) -> Result<gen_Tile, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<3>>(&config)?;
    let value = mrlyrs::gen::build::create_3d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(gen_Tile { inner: value })
}

/// Draws a cube tile from the config under a projection drawn from the stream.
#[wasm_bindgen]
pub fn gen_build_create_6d(config: JsValue, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<3>>(&config)?;
    let value = mrlyrs::gen::build::create_6d(&config, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Draws a random flat tile up to the given size under the default config.
#[wasm_bindgen]
pub fn gen_build_random_tile_2d(max_size: usize, rng: &mut hand::Rng) -> Result<gen_Tile, JsValue> {
    let value = mrlyrs::gen::build::random_tile_2d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(gen_Tile { inner: value })
}

/// Draws a random cube tile up to the given size.
#[wasm_bindgen]
pub fn gen_build_random_tile_3d(max_size: usize, rng: &mut hand::Rng) -> Result<gen_Tile, JsValue> {
    let value = mrlyrs::gen::build::random_tile_3d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(gen_Tile { inner: value })
}

/// Draws a random cube tile up to the given size under a random projection.
#[wasm_bindgen]
pub fn gen_build_random_tile_6d(max_size: usize, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::random_tile_6d(max_size, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the plane's bang code of a classic design, or None for one outside the plane.
#[wasm_bindgen]
pub fn gen_classic_code(design: JsValue) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::classic_code(design);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the bang code of a named design in a dimension, or None where it has no design.
#[wasm_bindgen]
pub fn gen_classic_code_nd(design: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::classic_code_nd(design, dimension);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Draws a hex key of the given length from the stream.
#[wasm_bindgen]
pub fn gen_hex_key(length: usize, rng: &mut hand::Rng) -> Result<String, JsValue> {
    let value = mrlyrs::gen::hex_key(length, rng.stream());
    Ok(value)
}

/// Draws one of the four flat classics from the stream: carpet, net, vertical tree or void.
#[wasm_bindgen]
pub fn gen_random_design(rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::random_design(rng.stream());
    hand::to_js(&value)
}

/// Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.
#[wasm_bindgen]
pub fn gen_random_rotation(design: JsValue, rng: &mut hand::Rng) -> Result<u8, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::random_rotation(design, rng.stream());
    Ok(value)
}

/// Returns the classic designs for a dimension.
#[wasm_bindgen]
pub fn gen_recipe_classics(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::classics(dimension);
    hand::to_js(&value)
}

/// Returns every flat size in the range that passes the parity filter.
#[wasm_bindgen]
pub fn gen_recipe_generals(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<Vec<usize>, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::generals(min_size, max_size, parity);
    Ok(value)
}

/// Returns every factor list of depth two and beyond whose product lands in the size range.
#[wasm_bindgen]
pub fn gen_recipe_nestings(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::nestings(min_size, max_size, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns every factor and level whose power lands in the size range.
#[wasm_bindgen]
pub fn gen_recipe_powers(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::powers(min_size, max_size, parity);
    hand::to_js(&value)
}

/// Returns every count-long factor list whose product lands in the size range.
#[wasm_bindgen]
pub fn gen_recipe_products(
    min_size: usize,
    max_size: usize,
    count: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::products(min_size, max_size, count, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the side a factor raised to a level makes, or None when no usize holds it.
#[wasm_bindgen]
pub fn gen_recipe_size(number: JsValue, level: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::i64_from_js(&number)?;
    let level = hand::i64_from_js(&level)?;
    let value = mrlyrs::gen::recipe::size(number, level);
    hand::to_js(&value)
}

/// Builds the mask of a mosaic tile: the two trees of the side, two where they cross.
#[wasm_bindgen]
pub fn gen_tree_mask(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::tree_mask(n).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Draws a variation's seed from the stream, then the variation itself on that seed, with a
#[wasm_bindgen]
pub fn gen_variation_create(
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<gen_variation_Variation, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::variation::Config>(&config)?;
    let value = mrlyrs::gen::variation::create(&config, rng.stream()).map_err(hand::throw)?;
    Ok(gen_variation_Variation { inner: value })
}

/// Builds the variation's base cell and draws its paint from the stream, painting the base
#[wasm_bindgen]
pub fn gen_variation_generate(
    variation: &gen_variation_Variation,
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<gen_variation_Variation, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::variation::Config>(&config)?;
    let value = mrlyrs::gen::variation::generate(variation.inner.clone(), &config, rng.stream())
        .map_err(hand::throw)?;
    Ok(gen_variation_Variation { inner: value })
}

/// Renders every file of the variation to PNG at the given scale, scattering a Random edition
#[wasm_bindgen]
pub fn gen_variation_render(
    variation: &gen_variation_Variation,
    scale: usize,
    rng: &mut hand::Rng,
) -> Result<gen_variation_Variation, JsValue> {
    let value = mrlyrs::gen::variation::render(variation.inner.clone(), scale, rng.stream())
        .map_err(hand::throw)?;
    Ok(gen_variation_Variation { inner: value })
}

/// Returns whether a rule is affine, its algebraic degree at most one.
#[wasm_bindgen]
pub fn life_affine(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::affine(rule);
    Ok(value)
}

/// Runs a seed under a config until it fixes, loops or times out, recording every generation.
#[wasm_bindgen]
pub fn life_animate(seed: JsValue, config: &life_Config) -> Result<life_Life, JsValue> {
    let seed = hand::cell2d_from_js(&seed)?;
    let value = mrlyrs::life::animate(&seed, &config.inner).map_err(hand::throw)?;
    Ok(life_Life { inner: value })
}

/// Returns the mean fraction of sites changed between consecutive grids.
#[wasm_bindgen]
pub fn life_churn(grids: JsValue) -> Result<f64, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::churn(&grids);
    Ok(value)
}

/// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
#[wasm_bindgen]
pub fn life_corner_bits(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::corner_bits(rule);
    Ok(value)
}

/// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
#[wasm_bindgen]
pub fn life_counts(
    seq: &life_Source,
    max_neighbors: usize,
    include_zeros: bool,
    include_ones: bool,
) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::life::counts(seq.inner, max_neighbors, include_zeros, include_ones)
        .map_err(hand::throw)?;
    Ok(value)
}

/// Crops a frame sequence to the centred square bounding every cell ever alive.
#[wasm_bindgen]
pub fn life_crop(grids: JsValue) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::crop(&grids).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
#[wasm_bindgen]
pub fn life_cube_orbit(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::cube_orbit(rule);
    Ok(value)
}

/// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker
#[wasm_bindgen]
pub fn life_design_mask(
    dimension: usize,
    code: JsValue,
    number: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::life::design_mask(dimension, code, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.
#[wasm_bindgen]
pub fn life_elementary_output(rule: u8, l: u8, c: u8, r: u8) -> Result<u8, JsValue> {
    let value = mrlyrs::life::elementary::output(rule, l, c, r);
    Ok(value)
}

/// Returns the grid's binary Shannon entropy in millibits.
#[wasm_bindgen]
pub fn life_entropy(grid: JsValue) -> Result<i64, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrlyrs::life::entropy(&grid);
    Ok(value)
}

/// Renders grids to white-on-black PNG bytes at a pixel scale.
#[wasm_bindgen]
pub fn life_frames(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::frames(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
#[wasm_bindgen]
pub fn life_gasket(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::gasket(rule);
    hand::to_js(&value)
}

/// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
#[wasm_bindgen]
pub fn life_genus(rule: u8) -> Result<String, JsValue> {
    let value = mrlyrs::life::genus(rule);
    Ok(value)
}

/// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
#[wasm_bindgen]
pub fn life_heatmap(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::heatmap(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.
#[wasm_bindgen]
pub fn life_history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::history(row, rule, steps, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns Langton's lambda, the popcount over eight.
#[wasm_bindgen]
pub fn life_lambda(rule: u8) -> Result<f64, JsValue> {
    let value = mrlyrs::life::lambda(rule);
    Ok(value)
}

/// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
#[wasm_bindgen]
pub fn life_lattice_index(mask: JsValue) -> Result<usize, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::life::lattice_index(&mask);
    Ok(value)
}

/// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
#[wasm_bindgen]
pub fn life_mask_offsets(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::life::mask_offsets(&mask);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Builds the 3 by 3 Moore mask, every site on but the center.
#[wasm_bindgen]
pub fn life_moore() -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::moore().map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Renders grids into one looping black-on-white gif, the delay in hundredths of a second.
#[wasm_bindgen]
pub fn life_movie(grids: JsValue, scale: usize, delay: usize) -> Result<Vec<u8>, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::movie(&grids, scale, delay).map_err(hand::throw)?;
    Ok(value)
}

/// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
#[wasm_bindgen]
pub fn life_next_grid(
    cell: JsValue,
    birth: &[usize],
    survive: &[usize],
    mask: JsValue,
    boundary: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let boundary = hand::from_js::<mrlyrs::life::Boundary>(&boundary)?;
    let value =
        mrlyrs::life::next_grid(&cell, birth, survive, &mask, boundary).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
#[wasm_bindgen]
pub fn life_npn_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::npn_class(rule);
    Ok(value)
}

/// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
#[wasm_bindgen]
pub fn life_outer_totalistic(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::outer_totalistic(rule);
    hand::option_to_js(value.as_ref(), |x1| {
        Ok(hand::tuple_to_js(&[
            hand::typed(&(x1.0)[..]),
            hand::typed(&(x1.1)[..]),
        ]))
    })
}

/// Returns the count of neighbourhoods a rule sends to one.
#[wasm_bindgen]
pub fn life_popcount(rule: u8) -> Result<u32, JsValue> {
    let value = mrlyrs::life::popcount(rule);
    Ok(value)
}

/// Renders one grid to white-on-black PNG bytes at a pixel scale.
#[wasm_bindgen]
pub fn life_render_frame(grid: JsValue, scale: usize) -> Result<Vec<u8>, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrlyrs::life::render::frame(&grid, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
#[wasm_bindgen]
pub fn life_reversible(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::reversible(rule);
    Ok(value)
}

/// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
#[wasm_bindgen]
pub fn life_rule_degree(rule: u8) -> Result<i32, JsValue> {
    let value = mrlyrs::life::rule_degree(rule);
    Ok(value)
}

/// Returns the design name a rule carries, `bang dim 3, code <rule>`.
#[wasm_bindgen]
pub fn life_rule_name(rule: u8) -> Result<String, JsValue> {
    let value = mrlyrs::life::rule_name(rule).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
#[wasm_bindgen]
pub fn life_single_seed(rule: u8, steps: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::single_seed(rule, steps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Generates the sequence's values up to the limit.
#[wasm_bindgen]
pub fn life_source_sequence(seq: &life_Source, limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::life::source::sequence(seq.inner, limit).map_err(hand::throw)?;
    Ok(value)
}

/// Advances one row one generation, a constant-0 boundary unless the edges wrap.
#[wasm_bindgen]
pub fn life_step(row: &[u8], rule: u8, wrap: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::step(row, rule, wrap).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
#[wasm_bindgen]
pub fn life_surjective(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::surjective(rule);
    Ok(value)
}

/// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
#[wasm_bindgen]
pub fn life_tessellate(grids: JsValue, min_canvas: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::tessellate(&grids, min_canvas).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
#[wasm_bindgen]
pub fn life_wolfram_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::wolfram_class(rule);
    Ok(value)
}

/// Builds an n by n carpet, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_carpet_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n carpet, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_carpet_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a carpet of the given side at any rank, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_carpet_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n dust, on where both coordinates are even.
#[wasm_bindgen]
pub fn math_atoms_dust_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n dust, on where all three coordinates are even.
#[wasm_bindgen]
pub fn math_atoms_dust_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a dust of the given side at any rank, on where every coordinate is even.
#[wasm_bindgen]
pub fn math_atoms_dust_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n line, free on axis 1, on along the odd rows.
#[wasm_bindgen]
pub fn math_atoms_hline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::hline_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tree, free on axis 1, on along the even rows.
#[wasm_bindgen]
pub fn math_atoms_htree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::htree_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.
#[wasm_bindgen]
pub fn math_atoms_line_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::line_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

/// Builds an n by n net, on where at least one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_net_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n net, on where at least two coordinates are odd.
#[wasm_bindgen]
pub fn math_atoms_net_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.
#[wasm_bindgen]
pub fn math_atoms_net_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.
#[wasm_bindgen]
pub fn math_atoms_noise_2d(
    n: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::noise_2d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.
#[wasm_bindgen]
pub fn math_atoms_noise_3d(
    n: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::noise_3d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor of ones.
#[wasm_bindgen]
pub fn math_atoms_ones_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ones_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor of ones.
#[wasm_bindgen]
pub fn math_atoms_ones_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ones_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n point, on where both coordinates are odd.
#[wasm_bindgen]
pub fn math_atoms_point_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n point, on where all three coordinates are odd.
#[wasm_bindgen]
pub fn math_atoms_point_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a point of the given side at any rank, on where every coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_point_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n star, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_star_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n star, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_star_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a star of the given side at any rank, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn math_atoms_star_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.
#[wasm_bindgen]
pub fn math_atoms_tree_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::tree_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

/// Builds an n by n line, free on axis 0, on along the odd columns.
#[wasm_bindgen]
pub fn math_atoms_vline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::vline_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n void, on where both coordinates share one parity.
#[wasm_bindgen]
pub fn math_atoms_void_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n void, on where all three coordinates share one parity.
#[wasm_bindgen]
pub fn math_atoms_void_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a void of the given side at any rank, on where every coordinate shares one parity.
#[wasm_bindgen]
pub fn math_atoms_void_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tree, free on axis 0, on along the even columns.
#[wasm_bindgen]
pub fn math_atoms_vtree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::vtree_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 0, its rods running along x.
#[wasm_bindgen]
pub fn math_atoms_xline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::xline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 0, its beams running along x.
#[wasm_bindgen]
pub fn math_atoms_xtree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::xtree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 1, its rods running along y.
#[wasm_bindgen]
pub fn math_atoms_yline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::yline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 1, its beams running along y.
#[wasm_bindgen]
pub fn math_atoms_ytree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ytree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor of zeros.
#[wasm_bindgen]
pub fn math_atoms_zeros_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zeros_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor of zeros.
#[wasm_bindgen]
pub fn math_atoms_zeros_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zeros_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 2, its rods running along z.
#[wasm_bindgen]
pub fn math_atoms_zline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 2, its beams running along z.
#[wasm_bindgen]
pub fn math_atoms_ztree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ztree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds the universe of a dimension.
#[wasm_bindgen]
pub fn math_bang_bang(dimension: usize) -> Result<math_bang_Universe, JsValue> {
    let value = mrlyrs::math::bang::bang(dimension).map_err(hand::throw)?;
    Ok(math_bang_Universe { inner: value })
}

/// Returns the distinct rotation and reflection maps of a base-q axis.
#[wasm_bindgen]
pub fn math_bang_baseq_axis_maps(base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::axis_maps(base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the distinct one-dimensional design counts for bases 1 through max_base.
#[wasm_bindgen]
pub fn math_bang_baseq_bracelets(max_base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::bracelets(max_base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the least code of the design's orbit.
#[wasm_bindgen]
pub fn math_bang_baseq_canonical(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::canonical(&group, code).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Carries a code through one group element.
#[wasm_bindgen]
pub fn math_bang_baseq_carry(element: &[usize], code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::carry(element, code);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Returns the fill-class counts for dimensions 1 through max_dimension.
#[wasm_bindgen]
pub fn math_bang_baseq_class_sequence(max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::class_sequence(max_dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.
#[wasm_bindgen]
pub fn math_bang_baseq_classes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::classes(dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Counts base-q designs distinct under symmetry.
#[wasm_bindgen]
pub fn math_bang_baseq_distinct_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::bang::baseq::distinct_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the collapsed fill count at an even side number.
#[wasm_bindgen]
pub fn math_bang_baseq_even_fill_is_balanced(
    number: usize,
    dimension: usize,
    popcount: JsValue,
) -> Result<JsValue, JsValue> {
    let popcount = hand::u128_from_js(&popcount)?;
    let value = mrlyrs::math::bang::baseq::even_fill_is_balanced(number, dimension, popcount)
        .map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled-cell count of a binary design at a side number, folded from its filled corners.
#[wasm_bindgen]
pub fn math_bang_baseq_fill_from_corners(
    filled: JsValue,
    number: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::bang::baseq::fill_from_corners(&filled, number, dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.
#[wasm_bindgen]
pub fn math_bang_baseq_group(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::group(base, dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the symmetry group order counted from the enumerated axis maps.
#[wasm_bindgen]
pub fn math_bang_baseq_group_order(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns every code a design reaches under the group.
#[wasm_bindgen]
pub fn math_bang_baseq_orbit(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::orbit(&group, code);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

/// Returns the closed-form group order the axis-map count must match.
#[wasm_bindgen]
pub fn math_bang_baseq_predicted_group_order(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::bang::baseq::predicted_group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.
#[wasm_bindgen]
pub fn math_bang_baseq_representatives(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::representatives(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&hand::code_to_js(x1.0)),
            hand::to_js(&x1.1)?,
        ]))
    })
}

/// Returns the distinct-design counts for dimensions 1 through max_dimension.
#[wasm_bindgen]
pub fn math_bang_baseq_sequence(base: usize, max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::sequence(base, max_dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the raw design count before symmetry, two to the number of cells.
#[wasm_bindgen]
pub fn math_bang_baseq_total_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::total_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the anti designs for a dimension.
#[wasm_bindgen]
pub fn math_bang_catalog_antis(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::antis(dimension);
    hand::to_js(&value)
}

/// Returns the bitmask the code carries.
#[wasm_bindgen]
pub fn math_bang_code_get(code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = code.get();
    Ok(JsValue::from_str(&value.to_string()))
}

/// Unpacks a code into its filled residue corners.
#[wasm_bindgen]
pub fn math_bang_code_to_corners(
    code: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::code_to_corners(code, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the binary corners of a dimension in code order.
#[wasm_bindgen]
pub fn math_bang_corners(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::corners(dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Packs filled residue corners back into their code.
#[wasm_bindgen]
pub fn math_bang_corners_to_code(
    filled: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::bang::corners_to_code(&filled, dimension, base);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
#[wasm_bindgen]
pub fn math_bang_factory_create(
    code: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::factory::create(code, number, dimension, base, level)
        .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Renders a design straight from its filled residue corners.
#[wasm_bindgen]
pub fn math_bang_factory_create_from_corners(
    filled: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value =
        mrlyrs::math::bang::factory::create_from_corners(&filled, number, dimension, base, level)
            .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Renders a design from its canonical JSON name.
#[wasm_bindgen]
pub fn math_bang_factory_create_named(
    spec: &str,
    number: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::bang::factory::create_named(spec, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns every base-q residue corner of a dimension in row-major order.
#[wasm_bindgen]
pub fn math_bang_factory_residue_corners(
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::factory::residue_corners(dimension, base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the code count of a dimension and base, two to the number of corners.
#[wasm_bindgen]
pub fn math_bang_factory_total_codes(dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::factory::total_codes(dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Returns the code of the design filled wherever a corner's residue sum lands in the levels.
#[wasm_bindgen]
pub fn math_bang_levels_code(
    dimension: usize,
    base: usize,
    levels: &[usize],
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::levels_code(dimension, base, levels);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.
#[wasm_bindgen]
pub fn math_bang_magic(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::magic(&layers).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Composes JSON-named layers in order.
#[wasm_bindgen]
pub fn math_bang_magic_named(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<(String, usize)>>(&layers)?;
    let layers_view: Vec<(&str, usize)> =
        layers.iter().map(|(a0, a1)| (a0.as_str(), *a1)).collect();
    let value = mrlyrs::math::bang::magic_named(&layers_view).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds the tile sources a catalog names at a dimension.
#[wasm_bindgen]
pub fn math_bang_sources(catalog: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let catalog = hand::from_js::<mrlyrs::gen::recipe::Catalog>(&catalog)?;
    let value = mrlyrs::math::bang::sources(&catalog, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the full symmetry group as axis permutations paired with flip patterns.
#[wasm_bindgen]
pub fn math_bang_symmetries(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::symmetries(dimension);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            hand::typed(&(x1.0)[..]),
            hand::typed(&(x1.1)[..]),
        ]))
    })
}

/// Returns whether no two filled corners of a code sit at Hamming distance one.
#[wasm_bindgen]
pub fn math_bang_total_exposure(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::total_exposure(code, dimension);
    Ok(value)
}

/// Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.
#[wasm_bindgen]
pub fn math_bang_touches_every_corner(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::touches_every_corner(code, dimension);
    Ok(value)
}

/// Returns the algebraic normal form coefficients of a code, one per corner.
#[wasm_bindgen]
pub fn math_bang_universe_anf(code: JsValue, dimension: usize) -> Result<Vec<u8>, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::anf(code, dimension);
    Ok(value)
}

/// Formats the algebraic normal form of a code as a sum of monomials.
#[wasm_bindgen]
pub fn math_bang_universe_anf_string(code: JsValue, dimension: usize) -> Result<String, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::anf_string(code, dimension);
    Ok(value)
}

/// Applies a symmetry element to a corner.
#[wasm_bindgen]
pub fn math_bang_universe_apply(element: JsValue, corner: &[u8]) -> Result<Vec<u8>, JsValue> {
    let element = hand::from_js::<(Vec<usize>, Vec<u8>)>(&element)?;
    let value = mrlyrs::math::bang::universe::apply(&element, corner);
    Ok(value)
}

/// Returns the bit position a binary corner occupies in a code.
#[wasm_bindgen]
pub fn math_bang_universe_corner_index(corner: &[u8]) -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::universe::corner_index(corner);
    Ok(value)
}

/// Returns the algebraic degree of a code, or -1 for the zero design.
#[wasm_bindgen]
pub fn math_bang_universe_degree(code: JsValue, dimension: usize) -> Result<i32, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::degree(code, dimension);
    Ok(value)
}

/// Returns every code a design reaches under the full symmetry group.
#[wasm_bindgen]
pub fn math_bang_universe_orbit(code: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::orbit(code, dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

/// Returns every permutation of 0..n in sorted order.
#[wasm_bindgen]
pub fn math_bang_universe_permutations(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::universe::permutations(n);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the canonical design codes of a dimension, computed once and cached for the process.
#[wasm_bindgen]
pub fn math_bang_universe_codes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::universe_codes(dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the 4-connected components of a plane word without drawing it.
#[wasm_bindgen]
pub fn math_bang_word_components(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::components(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the constant-word component functional of a plane word's letter frequencies,
#[wasm_bindgen]
pub fn math_bang_word_constant_functional(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::constant_functional(&layers).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.
#[wasm_bindgen]
pub fn math_bang_word_dimension(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::dimension(&layers).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the filled cells of a word, the product of its letter fills.
#[wasm_bindgen]
pub fn math_bang_word_fill(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::fill(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Lists the filled cells of every letter, the product of which is the word's fill.
#[wasm_bindgen]
pub fn math_bang_word_fills(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::fills(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
#[wasm_bindgen]
pub fn math_bang_word_letter(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrlyrs::math::bang::MagicLayer>(&layer)?;
    let value = mrlyrs::math::bang::word::letter(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns whether every letter renders at its own residue base, the native case where a
#[wasm_bindgen]
pub fn math_bang_word_native(layers: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::native(&layers);
    Ok(value)
}

/// Returns the shortest whole period of the letter list, its own length when no shorter block repeats.
#[wasm_bindgen]
pub fn math_bang_word_period(layers: JsValue) -> Result<usize, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::period(&layers);
    Ok(value)
}

/// Folds a plane word letter by letter and returns the counts at every prefix.
#[wasm_bindgen]
pub fn math_bang_word_prefixes(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::prefixes(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the prefix rates of a plane word in log two units, the component rate
#[wasm_bindgen]
pub fn math_bang_word_rates(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::rates(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the side of a word, the product of its letter sides.
#[wasm_bindgen]
pub fn math_bang_word_side(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::side(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Spells the first letters of a schedule over an ordered pair of letters.
#[wasm_bindgen]
pub fn math_bang_word_spell(
    schedule: JsValue,
    pair: JsValue,
    length: usize,
) -> Result<JsValue, JsValue> {
    let schedule = hand::from_js::<mrlyrs::math::bang::word::Schedule>(&schedule)?;
    let pair = hand::from_js::<(
        mrlyrs::math::bang::MagicLayer,
        mrlyrs::math::bang::MagicLayer,
    )>(&pair)?;
    let value = mrlyrs::math::bang::word::spell(schedule, pair, length);
    hand::to_js(&value)
}

/// Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
#[wasm_bindgen]
pub fn math_bang_word_staircase(depth: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::word::staircase(depth).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
#[wasm_bindgen]
pub fn math_bang_word_thue_morse(index: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::word::thue_morse(index);
    Ok(value)
}

/// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
#[wasm_bindgen]
pub fn math_cell_census_edges(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::edges::<2>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::edges::<3>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Counts the faces of filled sites open to emptiness or the border.
#[wasm_bindgen]
pub fn math_cell_census_exposure(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::exposure::<2>(&cell);
            Ok(JsValue::from_str(&value.to_string()))
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::exposure::<3>(&cell);
            Ok(JsValue::from_str(&value.to_string()))
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Counts the filled sites of the cell.
#[wasm_bindgen]
pub fn math_cell_census_fills(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::fills::<2>(&cell);
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::fills::<3>(&cell);
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Counts the distinct corners the filled sites touch, the edge graph's nodes.
#[wasm_bindgen]
pub fn math_cell_census_vertices(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::vertices::<2>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::vertices::<3>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Counts the empty sites of the cell.
#[wasm_bindgen]
pub fn math_cell_census_voids(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::voids::<2>(&cell);
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::census::voids::<3>(&cell);
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Merges same-shaped cells into one block laid out by the per-axis repetition counts.
#[wasm_bindgen]
pub fn math_cell_geometry_merge_reps(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value =
                mrlyrs::math::cell::geometry::merge_reps::<2>(&cells, reps).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value =
                mrlyrs::math::cell::geometry::merge_reps::<3>(&cells, reps).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Writes the value into the cell wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn math_cell_geometry_perforate(
    mask: JsValue,
    cell: JsValue,
    value: u8,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::geometry::perforate::<2>(&mask, &cell, value)
                .map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::geometry::perforate::<3>(&mask, &cell, value)
                .map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Grows a seed pattern into a cell, deepened to its fractal past level one.
#[wasm_bindgen]
pub fn math_cell_grow(pattern: JsValue, level: usize) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&pattern)? {
        2 => {
            let pattern = hand::tensor_from_js(&pattern)?;
            let value = mrlyrs::math::cell::grow::<2>(pattern, level).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let pattern = hand::tensor_from_js(&pattern)?;
            let value = mrlyrs::math::cell::grow::<3>(pattern, level).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Inverts the cell.
#[wasm_bindgen]
pub fn math_cell_models_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.anti();
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.anti();
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Maps each site to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn math_cell_models_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.binarize(threshold);
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.binarize(threshold);
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Binarizes the cell at the threshold Otsu's method picks.
#[wasm_bindgen]
pub fn math_cell_models_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.binarize_otsu();
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.binarize_otsu();
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Rounds each site to the mean of its masked neighborhood, wrapping on request.
#[wasm_bindgen]
pub fn math_cell_models_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the Kronecker product of the two cells.
#[wasm_bindgen]
pub fn math_cell_models_combine(cell: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let other = hand::cell2d_from_js(&other)?;
            let value = cell.combine(&other);
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let other = hand::cell3d_from_js(&other)?;
            let value = cell.combine(&other);
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the narrowest count dtype that fits the mask's popcount.
#[wasm_bindgen]
pub fn math_cell_models_counting_dtype(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::cell::models::counting_dtype(&mask);
    hand::to_js(&value)
}

/// Returns the size of axis 0, the cube's leading axis.
#[wasm_bindgen]
pub fn math_cell_models_depth(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.depth();
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the narrowest unsigned dtype that holds the peak value.
#[wasm_bindgen]
pub fn math_cell_models_dtype_for(peak: JsValue) -> Result<JsValue, JsValue> {
    let peak = hand::i64_from_js(&peak)?;
    let value = mrlyrs::math::cell::models::dtype_for(peak);
    hand::to_js(&value)
}

/// Deepens the cell into its level-fold fractal.
#[wasm_bindgen]
pub fn math_cell_models_fractal(cell: JsValue, level: usize) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.fractal(level).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.fractal(level).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the size of the axis before the last.
#[wasm_bindgen]
pub fn math_cell_models_height(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.height();
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.height();
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Swaps filled and empty sites.
#[wasm_bindgen]
pub fn math_cell_models_invert(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.invert();
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.invert();
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Tags each site with its ring distance from the center.
#[wasm_bindgen]
pub fn math_cell_models_layers(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.layers();
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.layers();
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Tags each site with its count of masked neighbors matching the target, wrapping on request.
#[wasm_bindgen]
pub fn math_cell_models_neighbors(
    cell: JsValue,
    mask: JsValue,
    target: u8,
    wrap: bool,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.neighbors(&mask, target, wrap).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.neighbors(&mask, target, wrap).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Builds a cell from an N-dimensional tensor of types.
#[wasm_bindgen]
pub fn math_cell_models_new(types: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&types)? {
        2 => {
            let types = hand::tensor_from_js(&types)?;
            let value = mrlyrs::math::cell::models::CellNd::<2>::new(types).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let types = hand::tensor_from_js(&types)?;
            let value = mrlyrs::math::cell::models::CellNd::<3>::new(types).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Turns the cell into one of the 24 cube orientations.
#[wasm_bindgen]
pub fn math_cell_models_orient(cell: JsValue, index: usize) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.orient(index).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Wraps the cell in count layers of the given value on every side.
#[wasm_bindgen]
pub fn math_cell_models_pad(cell: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.pad(count, value);
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.pad(count, value);
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Colors each site by its type through the mapping in the given mode.
#[wasm_bindgen]
pub fn math_cell_models_paint(
    cell: JsValue,
    mapping: JsValue,
    mode: JsValue,
    rng: JsValue,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let mapping = hand::map_from_js::<u8, _>(&mapping, |x1| {
                hand::list_from_js(x1, hand::color_from_js)
            })?;
            let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = cell
                .paint(&mapping, mode, rng_stream.as_mut())
                .map_err(hand::throw)?;
            if let Some(stream) = &rng_stream {
                hand::stream_to_js(&rng, stream)?;
            }
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let mapping = hand::map_from_js::<u8, _>(&mapping, |x1| {
                hand::list_from_js(x1, hand::color_from_js)
            })?;
            let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = cell
                .paint(&mapping, mode, rng_stream.as_mut())
                .map_err(hand::throw)?;
            if let Some(stream) = &rng_stream {
                hand::stream_to_js(&rng, stream)?;
            }
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Writes the value wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn math_cell_models_perforate(
    cell: JsValue,
    mask: JsValue,
    value: u8,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.perforate(&mask, value).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let mask = hand::tensor_from_js(&mask)?;
            let value = cell.perforate(&mask, value).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Rotates the cell k quarter turns in the plane.
#[wasm_bindgen]
pub fn math_cell_models_rotate(cell: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.rotate(k).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let axes = if axes.is_undefined() {
                return Err(hand::refuse("a 3d cell wants axes."));
            } else {
                hand::from_js::<(usize, usize)>(&axes)?
            };
            let value = cell.rotate(k, axes).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Repeats the cell into a width-by-height array of copies.
#[wasm_bindgen]
pub fn math_cell_models_tile(
    cell: JsValue,
    width: usize,
    height: usize,
    depth: JsValue,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.tile(width, height).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let depth = if depth.is_undefined() {
                return Err(hand::refuse("a 3d cell wants depth."));
            } else {
                hand::from_js::<usize>(&depth)?
            };
            let value = cell.tile(width, height, depth).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the tensor of types.
#[wasm_bindgen]
pub fn math_cell_models_types(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.types();
            hand::tensor_to_js(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.types();
            hand::tensor_to_js(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the size of the last axis.
#[wasm_bindgen]
pub fn math_cell_models_width(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.width();
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.width();
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Colors the cell through the given mapping and mode, defaulting to the standard palette by type.
#[wasm_bindgen]
pub fn math_cell_paint(
    cell: JsValue,
    custom: JsValue,
    mode: JsValue,
    rng: JsValue,
) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let custom = hand::option_from_js(&custom, |x1| {
                hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js))
            })?;
            let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value =
                mrlyrs::math::cell::paint::<2>(cell, custom.as_ref(), mode, rng_stream.as_mut())
                    .map_err(hand::throw)?;
            if let Some(stream) = &rng_stream {
                hand::stream_to_js(&rng, stream)?;
            }
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let custom = hand::option_from_js(&custom, |x1| {
                hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js))
            })?;
            let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value =
                mrlyrs::math::cell::paint::<3>(cell, custom.as_ref(), mode, rng_stream.as_mut())
                    .map_err(hand::throw)?;
            if let Some(stream) = &rng_stream {
                hand::stream_to_js(&rng, stream)?;
            }
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Reads a triply nested JSON array into layers of byte rows.
#[wasm_bindgen]
pub fn math_cell_serializer_byte_cube(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::byte_cube(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..])))
    })
}

/// Reads a nested JSON array into rows of bytes.
#[wasm_bindgen]
pub fn math_cell_serializer_byte_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::byte_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Reads a nested JSON array into rows of four-channel colors.
#[wasm_bindgen]
pub fn math_cell_serializer_color_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::color_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..])))
    })
}

/// Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
#[wasm_bindgen]
pub fn math_cell_serializer_count_cube(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::count_cube(&value).map_err(hand::throw)?;
    Ok(value)
}

/// Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
#[wasm_bindgen]
pub fn math_cell_serializer_count_grid(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::count_grid(&value).map_err(hand::throw)?;
    Ok(value)
}

/// Parses JSON text into a value tree.
#[wasm_bindgen]
pub fn math_cell_serializer_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::cell::serializer::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.
#[wasm_bindgen]
pub fn math_cell_serializer_tag_layer(
    counts: JsValue,
    shape: Vec<usize>,
) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::i64_from_js)?;
    let value = mrlyrs::math::cell::serializer::tag_layer(&counts, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the types field of the data.
#[wasm_bindgen]
pub fn math_cell_serializer_types_field(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<mrlyrs::core::Json>(&data)?;
    let value = mrlyrs::math::cell::serializer::types_field(&data).map_err(hand::throw)?;
    hand::to_js(value)
}

/// Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.
#[wasm_bindgen]
pub fn math_counts_centered_hexagonal(m: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::centered_hexagonal(m);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled triangle count of the code's cut section at the given level, without rendering it.
#[wasm_bindgen]
pub fn math_counts_cut_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::cut_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty triangle count of the code's cut section at the given level.
#[wasm_bindgen]
pub fn math_counts_cut_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::cut_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the code's fractal dimension, the log of its one-level fill over the log of number.
#[wasm_bindgen]
pub fn math_counts_dimension(
    code: JsValue,
    number: usize,
    base_dimension: usize,
    base: usize,
) -> Result<f64, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrlyrs::math::counts::dimension(code, number, base_dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term
#[wasm_bindgen]
pub fn math_counts_edges_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::edges_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.
#[wasm_bindgen]
pub fn math_counts_exposure(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::exposure(code, number, dimension, level, base)
        .map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.
#[wasm_bindgen]
pub fn math_counts_exposure_of_tile_(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::exposure_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the coefficients of the recurrence the tile's exposure obeys.
#[wasm_bindgen]
pub fn math_counts_exposure_recurrence_(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::exposure_recurrence(&tile);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the filled cell count of the code's fractal at the given level, without rendering it.
#[wasm_bindgen]
pub fn math_counts_fill(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrlyrs::math::counts::fill(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Sums each corner's position products into a base fill and raises it to the level.
#[wasm_bindgen]
pub fn math_counts_fill_from_corners(
    filled: JsValue,
    number: usize,
    _dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::counts::fill_from_corners(&filled, number, _dimension, level, base);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the total cells of the grid, number to the dimension, to the level.
#[wasm_bindgen]
pub fn math_counts_grid(number: usize, dimension: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::grid(number, dimension, level);
    Ok(JsValue::from_str(&value.to_string()))
}

/// The widest dimension the exact carry arithmetic reaches at the base.
#[wasm_bindgen]
pub fn math_counts_ladder_cap(base: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::counts::ladder::cap(base).map_err(hand::throw)?;
    Ok(value)
}

/// The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.
#[wasm_bindgen]
pub fn math_counts_ladder_carry_matrix(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::carry_matrix(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string())))
    })
}

/// The monic characteristic polynomial of a square integer matrix, highest power first.
#[wasm_bindgen]
pub fn math_counts_ladder_characteristic(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::characteristic(&rows).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The determinant of a square integer matrix, read off its characteristic polynomial.
#[wasm_bindgen]
pub fn math_counts_ladder_determinant(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::determinant(&rows).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.
#[wasm_bindgen]
pub fn math_counts_ladder_digit_polynomial(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::counts::ladder::digit_polynomial(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The reflection-even block of the carry matrix, of size `ceil(D/2)`.
#[wasm_bindgen]
pub fn math_counts_ladder_even_block(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::even_block(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string())))
    })
}

/// The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.
#[wasm_bindgen]
pub fn math_counts_ladder_fill(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::fill(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.
#[wasm_bindgen]
pub fn math_counts_ladder_ladder(
    base: usize,
    dimension: usize,
    levels: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::counts::ladder::ladder(base, dimension, levels).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The Perron root of a nonnegative square integer matrix.
#[wasm_bindgen]
pub fn math_counts_ladder_perron(rows: JsValue) -> Result<f64, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::perron(&rows).map_err(hand::throw)?;
    Ok(value)
}

/// The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.
#[wasm_bindgen]
pub fn math_counts_ladder_sign(base: usize, dimension: usize) -> Result<i32, JsValue> {
    let value = mrlyrs::math::counts::ladder::sign(base, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.
#[wasm_bindgen]
pub fn math_counts_ladder_spectral_ratio(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::counts::ladder::spectral_ratio(base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The trace of a square integer matrix.
#[wasm_bindgen]
pub fn math_counts_ladder_trace(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::trace(&rows);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the fill ratio the code walks toward as the side number grows, reduced.
#[wasm_bindgen]
pub fn math_counts_limit(
    code: JsValue,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::limit(code, dimension, level, base).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

/// Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.
#[wasm_bindgen]
pub fn math_counts_pairs(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::pairs(&tile);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

/// Counts the indices below number that equal residue modulo base.
#[wasm_bindgen]
pub fn math_counts_positions(
    residue: usize,
    number: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::positions(residue, number, base);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled triangle count of the code's pro projection at the given level, without rendering it.
#[wasm_bindgen]
pub fn math_counts_pro_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::pro_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty triangle count of the code's pro projection at the given level.
#[wasm_bindgen]
pub fn math_counts_pro_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::pro_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.
#[wasm_bindgen]
pub fn math_counts_profile_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::profile_of_tile(&tile, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the filled fraction of the grid, or 0.0 for an empty grid.
#[wasm_bindgen]
pub fn math_counts_ratio(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<f64, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrlyrs::math::counts::ratio(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the exact filled fraction as a fraction of fill over grid, reduced.
#[wasm_bindgen]
pub fn math_counts_rational(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::rational(code, number, dimension, level, base)
        .map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

/// Returns the triangles of the full hexagon with side number to the level.
#[wasm_bindgen]
pub fn math_counts_six_grid_triangles(number: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::grid_triangles(number, level);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the boundary edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_boundary(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_boundary(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the core edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_core_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_core_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the core node count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_core_nodes(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_core_nodes(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the distinct triangle-edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the interior edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_interior(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_interior(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the triangle count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_triangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_triangles(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the vertex count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn math_counts_six_solid_slice_vertices(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_vertices(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the exposed face count of the code's 3D fractal at the given level.
#[wasm_bindgen]
pub fn math_counts_surface(
    code: JsValue,
    number: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::surface(code, number, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty cell count, grid minus fill.
#[wasm_bindgen]
pub fn math_counts_void(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrlyrs::math::counts::void(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Takes the full census of a network.
#[wasm_bindgen]
pub fn math_graph_census(network: &math_graph_Network) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::graph::census(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts the connected components of the network.
#[wasm_bindgen]
pub fn math_graph_components(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::components(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Extracts the network of filled sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn math_graph_core_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::core_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Extracts the network of corners and edges outlining every filled site.
#[wasm_bindgen]
pub fn math_graph_edge_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::edge_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.
#[wasm_bindgen]
pub fn math_graph_fractal_dimension(
    network: &math_graph_Network,
    samples: usize,
) -> Result<f64, JsValue> {
    let value = mrlyrs::math::graph::fractal_dimension(&network.inner, samples);
    Ok(value)
}

/// Counts the nodes of degree three or more.
#[wasm_bindgen]
pub fn math_graph_junctions(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::junctions(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Extracts the largest connected piece as a network of its own, branches re-indexed.
#[wasm_bindgen]
pub fn math_graph_largest_component(
    network: &math_graph_Network,
) -> Result<math_graph_Network, JsValue> {
    let value = mrlyrs::math::graph::largest_component(&network.inner).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Tags every node by its degree, indexed like the node list.
#[wasm_bindgen]
pub fn math_graph_roles(network: &math_graph_Network) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::graph::roles(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts the nodes of degree one.
#[wasm_bindgen]
pub fn math_graph_tips(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::tips(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Sums the straight-line lengths of every branch.
#[wasm_bindgen]
pub fn math_graph_total_length(network: &math_graph_Network) -> Result<f64, JsValue> {
    let value = mrlyrs::math::graph::total_length(&network.inner);
    Ok(value)
}

/// Extracts the core graph of the inverted grid, joining empty sites instead.
#[wasm_bindgen]
pub fn math_graph_tunnel_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::tunnel_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Returns every preset stacked up to the given scale.
#[wasm_bindgen]
pub fn math_moire_all(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::moire::all(limit);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(math_moire_Preset { inner: x1.clone() }))
    })
}

/// Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.
#[wasm_bindgen]
pub fn math_moire_frame(normal: JsValue, offset: f64) -> Result<JsValue, JsValue> {
    let normal = hand::from_js::<[f64; 3]>(&normal)?;
    let value = mrlyrs::math::moire::frame(normal, offset).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Samples a design over the pixel grid into a boolean mask.
#[wasm_bindgen]
pub fn math_moire_layer(params: JsValue) -> Result<JsValue, JsValue> {
    let params = hand::from_js::<mrlyrs::math::moire::Layer>(&params)?;
    let value = mrlyrs::math::moire::layer(&params).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the preset the name picks.
#[wasm_bindgen]
pub fn math_moire_named(name: &str, limit: usize) -> Result<math_moire_Preset, JsValue> {
    let value = mrlyrs::math::moire::named(name, limit).map_err(hand::throw)?;
    Ok(math_moire_Preset { inner: value })
}

/// Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.
#[wasm_bindgen]
pub fn math_moire_pairs_correlation(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::moire::pairs::correlation(m, n);
    Ok(value)
}

/// Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.
#[wasm_bindgen]
pub fn math_moire_pairs_sampled(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::moire::pairs::sampled(m, n).map_err(hand::throw)?;
    Ok(value)
}

/// Puts an odd scale of three or more on trial against every earlier odd scale.
#[wasm_bindgen]
pub fn math_moire_pairs_witness(scale: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::moire::pairs::witness(scale).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Quantizes a field into colored levels and encodes PNG bytes.
#[wasm_bindgen]
pub fn math_moire_render(
    field: &math_moire_Field,
    colorizer: JsValue,
    levels: usize,
    symmetric: bool,
    invert: bool,
    scale: usize,
) -> Result<Vec<u8>, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value =
        mrlyrs::math::moire::render(&field.inner, &colorizer, levels, symmetric, invert, scale)
            .map_err(hand::throw)?;
    Ok(value)
}

/// Returns the two lattice coordinates of each pixel centre along a row.
#[wasm_bindgen]
pub fn math_moire_sample_axes(
    size: usize,
    lattice: JsValue,
    row: usize,
) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::sample::axes(size, lattice, row);
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Unpacks a code into its residue-corner truth table.
#[wasm_bindgen]
pub fn math_moire_sample_membership(
    code: JsValue,
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value =
        mrlyrs::math::moire::sample::membership(code, base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Folds residues into a base-q index of the truth table.
#[wasm_bindgen]
pub fn math_moire_sample_pack(residues: &[usize], base: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::moire::sample::pack(residues, base);
    Ok(value)
}

/// Layers one design at several side numbers into a field under the chosen combine.
#[wasm_bindgen]
pub fn math_moire_stack(
    spec: JsValue,
    numbers: &[usize],
    combine: JsValue,
    level: usize,
    lattice: JsValue,
    size: usize,
    slices: &[f64],
) -> Result<math_moire_Field, JsValue> {
    let spec = hand::from_js::<mrlyrs::math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrlyrs::math::moire::Combine>(&combine)?;
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::stack(spec, numbers, combine, level, lattice, size, slices)
        .map_err(hand::throw)?;
    Ok(math_moire_Field { inner: value })
}

/// Sums layers of several designs at one side number into a field.
#[wasm_bindgen]
pub fn math_moire_stack_codes(
    specs: JsValue,
    number: usize,
    level: usize,
    lattice: JsValue,
    size: usize,
    slices: &[f64],
) -> Result<math_moire_Field, JsValue> {
    let specs = hand::from_js::<Vec<mrlyrs::math::moire::Spec>>(&specs)?;
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::stack_codes(&specs, number, level, lattice, size, slices)
        .map_err(hand::throw)?;
    Ok(math_moire_Field { inner: value })
}

/// Layers one cube design at several side numbers into a volume under the chosen combine.
#[wasm_bindgen]
pub fn math_moire_volume(
    spec: JsValue,
    numbers: &[usize],
    combine: JsValue,
    level: usize,
    size: usize,
) -> Result<math_moire_Volume, JsValue> {
    let spec = hand::from_js::<mrlyrs::math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrlyrs::math::moire::Combine>(&combine)?;
    let value =
        mrlyrs::math::moire::volume(spec, numbers, combine, level, size).map_err(hand::throw)?;
    Ok(math_moire_Volume { inner: value })
}

/// Returns the number of designs of the dimension and base that contain the number.
#[wasm_bindgen]
pub fn math_press_containing(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::containing(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.
#[wasm_bindgen]
pub fn math_press_coordinates(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::coordinates(number, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the members of a design below the limit.
#[wasm_bindgen]
pub fn math_press_count_below(
    code: JsValue,
    dimension: usize,
    base: usize,
    limit: JsValue,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let limit = hand::u128_from_js(&limit)?;
    let value =
        mrlyrs::math::press::count_below(code, dimension, base, limit).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the count of distinct digit vectors the number uses.
#[wasm_bindgen]
pub fn math_press_distinct(number: JsValue, dimension: usize, base: usize) -> Result<u32, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::distinct(number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Weaves dimension coordinates back into their single interleaved number.
#[wasm_bindgen]
pub fn math_press_interleave(coords: JsValue, base: usize) -> Result<JsValue, JsValue> {
    let coords = hand::list_from_js(&coords, hand::u128_from_js)?;
    let value = mrlyrs::math::press::interleave(&coords, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the allowed digit table of one magic layer, one flag per cell of its tile.
#[wasm_bindgen]
pub fn math_press_layer_table(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrlyrs::math::bang::MagicLayer>(&layer)?;
    let value = mrlyrs::math::press::layer_table(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns whether every digit vector of the number lies in the design.
#[wasm_bindgen]
pub fn math_press_member(
    code: JsValue,
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::member(code, number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the first members of a design in ascending order.
#[wasm_bindgen]
pub fn math_press_members(
    code: JsValue,
    dimension: usize,
    base: usize,
    count: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::press::members(code, dimension, base, count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the diagonal slice profile of one design pressed to a fractal level.
#[wasm_bindgen]
pub fn math_press_profile(
    code: JsValue,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::press::profile(code, dimension, base, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.
#[wasm_bindgen]
pub fn math_press_usage(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::usage(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Counts the members of a magic word from its layer fills, without enumeration.
#[wasm_bindgen]
pub fn math_press_word_count(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_count(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether the number lies in the magic word's composed design.
#[wasm_bindgen]
pub fn math_press_word_member(layers: JsValue, number: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::word_member(&layers, number).map_err(hand::throw)?;
    Ok(value)
}

/// Enumerates every member of the magic word in ascending order.
#[wasm_bindgen]
pub fn math_press_word_members(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_members(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the diagonal slice profile of a magic word by the substitution product.
#[wasm_bindgen]
pub fn math_press_word_profile(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_profile(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.
#[wasm_bindgen]
pub fn math_roulette_nodes(
    track: JsValue,
    pencils: JsValue,
    samples: usize,
    tol: f64,
) -> Result<math_roulette_Nodes, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value =
        mrlyrs::math::roulette::nodes(&track, &pencils, samples, tol).map_err(hand::throw)?;
    Ok(math_roulette_Nodes { inner: value })
}

/// Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.
#[wasm_bindgen]
pub fn math_roulette_side(a: JsValue, b: JsValue, c: JsValue) -> Result<i32, JsValue> {
    let a = hand::from_js::<[f64; 2]>(&a)?;
    let b = hand::from_js::<[f64; 2]>(&b)?;
    let c = hand::from_js::<[f64; 2]>(&c)?;
    let value = mrlyrs::math::roulette::side(a, b, c);
    Ok(value)
}

/// One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.
#[wasm_bindgen]
pub fn math_roulette_spread(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::roulette::spread(&track, &pencils, exact);
    hand::to_js(&value)
}

/// Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.
#[wasm_bindgen]
pub fn math_rules_render(
    filled: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value =
        mrlyrs::math::rules::render(&filled, number, dimension, base).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns every axis but the free one.
#[wasm_bindgen]
pub fn math_rules_tree_axes(dimension: usize, free_axis: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::math::rules::tree_axes(dimension, free_axis);
    Ok(value)
}

/// Tallies the design's cells and filled cells per region of the shape.
#[wasm_bindgen]
pub fn math_shape_census(shape: JsValue, types: JsValue) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let types = hand::tensor_from_js(&types)?;
    let value = mrlyrs::math::shape::census(&shape, &types).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Places one lattice cell relative to the shape, exactly, with no floats.
#[wasm_bindgen]
pub fn math_shape_classify(
    shape: JsValue,
    side: usize,
    index: &[usize],
) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::classify(&shape, side, index).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.
#[wasm_bindgen]
pub fn math_shape_crop(types: JsValue, shape: JsValue, keep_cut: bool) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::crop(&types, &shape, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.
#[wasm_bindgen]
pub fn math_shape_crossing_shell(
    radius: JsValue,
    number: JsValue,
    level: u32,
) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::math::shape::crossing_shell(radius, number, level);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Builds the whole crossing tree of one radius, pruned by the seats the design keeps.
#[wasm_bindgen]
pub fn math_shape_crossing_tree(
    radius: JsValue,
    number: JsValue,
    keep: JsValue,
) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let keep = hand::from_js::<Vec<bool>>(&keep)?;
    let value = mrlyrs::math::shape::crossing_tree(radius, number, &keep);
    hand::to_js(&value)
}

/// Builds a named shape of the dimension, centered at one half on every axis.
#[wasm_bindgen]
pub fn math_shape_named(
    name: &str,
    dimension: usize,
    radius: &math_shape_Frac,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::shape::named(name, dimension, radius.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.
#[wasm_bindgen]
pub fn math_shape_radial_census(
    types: JsValue,
    centre: JsValue,
    r_max: JsValue,
) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let centre = hand::list_from_js(&centre, hand::i64_from_js)?;
    let r_max = hand::u64_from_js(&r_max)?;
    let value = mrlyrs::math::shape::radial_census(&types, &centre, r_max);
    hand::to_js(&value)
}

/// Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.
#[wasm_bindgen]
pub fn math_shape_refine(
    types: JsValue,
    shape: JsValue,
    base: usize,
    extra: usize,
    keep_cut: bool,
) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value =
        mrlyrs::math::shape::refine(&types, &shape, base, extra, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.
#[wasm_bindgen]
pub fn math_shape_regions(shape: JsValue, dims: &[usize]) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::regions(&shape, dims).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Lists the named shapes of a dimension.
#[wasm_bindgen]
pub fn math_shape_shapes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::shape::shapes(dimension);
    hand::to_js(&value)
}

/// Swaps every fill triangle for a void and back.
#[wasm_bindgen]
pub fn math_six_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.anti();
    hand::cell6d_to_js(&value)
}

/// Maps each triangle to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn math_six_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell6d_to_js(&value)
}

/// Binarizes the triangles at the threshold Otsu's method picks.
#[wasm_bindgen]
pub fn math_six_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell6d_to_js(&value)
}

/// Builds a hexagon of the given radius, fill inside and void outside.
#[wasm_bindgen]
pub fn math_six_blank(
    radius: usize,
    orient: JsValue,
    fill: u8,
    void_: u8,
) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrlyrs::math::six::Orientation>(&orient)?;
    let value = mrlyrs::math::six::blank(radius, orient, fill, void_).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Rounds each triangle to the mean of its masked neighborhood, wrapping on request.
#[wasm_bindgen]
pub fn math_six_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Tallies a cell's triangles, corners and edges, counting the backdrop only on request.
#[wasm_bindgen]
pub fn math_six_census(cell: JsValue, include_grid: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::census(&cell, include_grid);
    hand::to_js(&value)
}

/// Counts the connected pieces of the fill, triangles joined across shared edges.
#[wasm_bindgen]
pub fn math_six_components(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::components(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Slices a cube through its center across the main diagonal into a hexagon.
#[wasm_bindgen]
pub fn math_six_cut(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::cut(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and slices its central hexagon.
#[wasm_bindgen]
pub fn math_six_cut_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::cut_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// The three corners of the east-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn math_six_east(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::east(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.
#[wasm_bindgen]
pub fn math_six_euler(cell: JsValue, include_grid: bool) -> Result<i64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::euler(&cell, include_grid);
    Ok(value)
}

/// Counts the filled triangles of the cell.
#[wasm_bindgen]
pub fn math_six_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::fills(&cell);
    Ok(value)
}

/// Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.
#[wasm_bindgen]
pub fn math_six_fills_only(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::fills_only(&cell);
    hand::to_js(&value)
}

/// Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.
#[wasm_bindgen]
pub fn math_six_framed(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::framed(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Parses a cell from JSON, defaulting any missing projection metadata.
#[wasm_bindgen]
pub fn math_six_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::from_json(text).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Returns the triangle count of the fill's largest connected piece.
#[wasm_bindgen]
pub fn math_six_giant(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::giant(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the largest connected piece of the filled-triangle network as a network of its own.
#[wasm_bindgen]
pub fn math_six_giant_network(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::giant_network(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Returns the grid height in triangles.
#[wasm_bindgen]
pub fn math_six_height(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.height();
    Ok(value)
}

/// Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.
#[wasm_bindgen]
pub fn math_six_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether the cell's three sides are equal.
#[wasm_bindgen]
pub fn math_six_is_cube(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::is_cube(&cell);
    Ok(value)
}

/// Returns whether the cell's width, height and parity frame a hexagon.
#[wasm_bindgen]
pub fn math_six_is_hex(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::six::is_hex(&cell);
    Ok(value)
}

/// Projects a cube into the isometric hexagon of top, left and right faces.
#[wasm_bindgen]
pub fn math_six_iso(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::iso(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and projects it isometrically.
#[wasm_bindgen]
pub fn math_six_iso_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::iso_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds a cell from its four parts.
#[wasm_bindgen]
pub fn math_six_new(
    cell: JsValue,
    projection: JsValue,
    orientation: JsValue,
    start: u8,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let projection = hand::from_js::<mrlyrs::math::six::Projection>(&projection)?;
    let orientation = hand::from_js::<mrlyrs::math::six::Orientation>(&orientation)?;
    let value = mrlyrs::math::six::Cell6d::new(cell, projection, orientation, start);
    hand::cell6d_to_js(&value)
}

/// The three corners of the north-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn math_six_north(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::north(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Returns the orientation a hexagon's width and height imply.
#[wasm_bindgen]
pub fn math_six_orientation(width: usize, height: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::orientation(width, height).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.
#[wasm_bindgen]
pub fn math_six_pad(cell: JsValue, k: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::pad(&cell, k, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Colors each triangle by its type through the custom or default mapping in the given or type mode.
#[wasm_bindgen]
pub fn math_six_paint(
    cell: JsValue,
    custom: JsValue,
    mode: JsValue,
    rng: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let custom = hand::option_from_js(&custom, |x1| {
        hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js))
    })?;
    let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = mrlyrs::math::six::paint(cell, custom.as_ref(), mode, rng_stream.as_mut())
        .map_err(hand::throw)?;
    if let Some(stream) = &rng_stream {
        hand::stream_to_js(&rng, stream)?;
    }
    hand::cell6d_to_js(&value)
}

/// Writes the value wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn math_six_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn math_six_png(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let value = mrlyrs::math::six::png(&cell, scale, outline, width).map_err(hand::throw)?;
    Ok(value)
}

/// Projects a cube's three facing sides into a hexagon of fills and voids.
#[wasm_bindgen]
pub fn math_six_pro(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::pro(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and projects its facing sides.
#[wasm_bindgen]
pub fn math_six_pro_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::pro_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Tessellates a hexagonal cell over the disc mask of the given radius.
#[wasm_bindgen]
pub fn math_six_radial(cell: JsValue, radius: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::radial(&cell, radius).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Crops the interlocking overhang off a disc tiled at the given radius and tile size.
#[wasm_bindgen]
pub fn math_six_radial_crop(
    cell: JsValue,
    radius: usize,
    size: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::radial_crop(&cell, radius, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the disc mask of cells within hex distance radius of the center.
#[wasm_bindgen]
pub fn math_six_radial_mask(radius: usize, orient: JsValue) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrlyrs::math::six::Orientation>(&orient)?;
    let value = mrlyrs::math::six::radial_mask(radius, orient);
    hand::tensor_to_js(&value)
}

/// Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.
#[wasm_bindgen]
pub fn math_six_raster(cell: JsValue, size: usize) -> Result<Vec<f32>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::raster(&cell, size).map_err(hand::throw)?;
    Ok(value)
}

/// Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes.
#[wasm_bindgen]
pub fn math_six_rect_png(cell: JsValue, scale: usize, start: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::rect_png(&cell, scale, start).map_err(hand::throw)?;
    Ok(value)
}

/// Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.
#[wasm_bindgen]
pub fn math_six_rect_svg(cell: JsValue, scale: usize, start: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::rect_svg(&cell, scale, start).map_err(hand::throw)?;
    Ok(value)
}

/// Counts the void regions the rim never reaches, the second route to the hole count.
#[wasm_bindgen]
pub fn math_six_rim_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::rim_holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.
#[wasm_bindgen]
pub fn math_six_skin(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::skin(&cell);
    hand::cell6d_to_js(&value)
}

/// Builds the network of filled triangles joined by shared edges.
#[wasm_bindgen]
pub fn math_six_slice_core_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_core_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Builds the network of fill and void triangles joined by shared edges.
#[wasm_bindgen]
pub fn math_six_slice_dual_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_dual_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.
#[wasm_bindgen]
pub fn math_six_slice_edge_graph(
    cell: JsValue,
    value: JsValue,
) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = hand::from_js::<Option<u8>>(&value)?;
    let value = mrlyrs::math::six::slice_edge_graph(&cell, value).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// Builds the network of void triangles joined by shared edges, the pore network of the slice.
#[wasm_bindgen]
pub fn math_six_slice_tunnel_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_tunnel_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

/// The three corners of the south-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn math_six_south(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::south(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.
#[wasm_bindgen]
pub fn math_six_spectral_exponent(cell: JsValue, window: f64) -> Result<f64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::spectral_exponent(&cell, window).map_err(hand::throw)?;
    Ok(value)
}

/// The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.
#[wasm_bindgen]
pub fn math_six_star_arm_law(number: usize) -> Result<math_six_star_Share, JsValue> {
    let value = mrlyrs::math::six::star::arm_law(number).map_err(hand::throw)?;
    Ok(math_six_star_Share { inner: value })
}

/// The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.
#[wasm_bindgen]
pub fn math_six_star_chi8(number: usize) -> Result<i64, JsValue> {
    let value = mrlyrs::math::six::star::chi8(number);
    Ok(value)
}

/// The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.
#[wasm_bindgen]
pub fn math_six_star_constant() -> Result<f64, JsValue> {
    let value = mrlyrs::math::six::star::constant();
    Ok(value)
}

/// The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.
#[wasm_bindgen]
pub fn math_six_star_decay(excesses: &[f64], layers: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::star::decay(excesses, layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.
#[wasm_bindgen]
pub fn math_six_star_width_law(half: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::six::star::width_law(half);
    Ok(value)
}

/// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn math_six_svg(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
    start: JsValue,
) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::svg(&cell, scale, outline, width, start).map_err(hand::throw)?;
    Ok(value)
}

/// Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.
#[wasm_bindgen]
pub fn math_six_tessellate(cell: JsValue, mask: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::six::tessellate(&cell, &mask).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tessellates a hexagonal cell over a full width-by-height mask.
#[wasm_bindgen]
pub fn math_six_tile(cell: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::tile(&cell, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.
#[wasm_bindgen]
pub fn math_six_tile_cell(
    cell: JsValue,
    width: usize,
    height: usize,
    crop: bool,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::tile_cell(&cell, width, height, crop).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Crops one interlocking step off each side of a sheet tiled at the given size.
#[wasm_bindgen]
pub fn math_six_tile_crop(cell: JsValue, size: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::tile_crop(&cell, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.
#[wasm_bindgen]
pub fn math_six_tile_step(size: JsValue) -> Result<JsValue, JsValue> {
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::tile_step(size).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Serializes a cell and its projection metadata to JSON.
#[wasm_bindgen]
pub fn math_six_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::to_json(&cell);
    Ok(value)
}

/// Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.
#[wasm_bindgen]
pub fn math_six_triangles(cell: JsValue, start: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::triangles(&cell, start).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            hand::list_to_js(&x1.0, |x3| {
                Ok(hand::tuple_to_js(&[
                    JsValue::from(x3.0),
                    JsValue::from(x3.1),
                ]))
            })?,
            hand::typed(&(x1.1)[..]),
        ]))
    })
}

/// The three corners of the west-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn math_six_west(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::west(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Returns the grid width in triangles.
#[wasm_bindgen]
pub fn math_six_width(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.width();
    Ok(value)
}

/// Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.
#[wasm_bindgen]
pub fn math_spectrum_clusters(eigenvalues: &[f64], tolerance: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::clusters(eigenvalues, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.
#[wasm_bindgen]
pub fn math_spectrum_laplacian(
    network: &math_graph_Network,
    normalised: bool,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::spectrum::laplacian(&network.inner, normalised).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.
#[wasm_bindgen]
pub fn math_spectrum_laplacian_spectrum(
    network: &math_graph_Network,
    normalised: bool,
) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::math::spectrum::laplacian_spectrum(&network.inner, normalised)
        .map_err(hand::throw)?;
    Ok(value)
}

/// Counts the eigenvalues within the tolerance of a value.
#[wasm_bindgen]
pub fn math_spectrum_multiplicity(
    eigenvalues: &[f64],
    value: f64,
    tolerance: f64,
) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spectrum::multiplicity(eigenvalues, value, tolerance);
    Ok(value)
}

/// Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.
#[wasm_bindgen]
pub fn math_spectrum_spectral_exponent(
    eigenvalues: &[f64],
    window: f64,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_exponent(eigenvalues, window);
    hand::to_js(&value)
}

/// Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.
#[wasm_bindgen]
pub fn math_spectrum_spectral_fit(eigenvalues: &[f64], window: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_fit(eigenvalues, window);
    hand::to_js(&value)
}

/// Builds the integrated density of states as points, each an eigenvalue and its rank fraction.
#[wasm_bindgen]
pub fn math_spectrum_spectral_points(eigenvalues: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_points(eigenvalues);
    hand::to_js(&value)
}

/// Returns the eigenvalues of a dense real symmetric matrix in ascending order.
#[wasm_bindgen]
pub fn math_spectrum_symmetric_eigenvalues(matrix: JsValue) -> Result<Vec<f64>, JsValue> {
    let matrix = hand::from_js::<Vec<Vec<f64>>>(&matrix)?;
    let value = mrlyrs::math::spectrum::symmetric_eigenvalues(&matrix).map_err(hand::throw)?;
    Ok(value)
}

/// The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.
#[wasm_bindgen]
pub fn math_spin_arcs(data: &[f32], size: usize, radius: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spin::arcs(data, size, radius).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.
#[wasm_bindgen]
pub fn math_spin_harmonics(
    data: &[f32],
    size: usize,
    rings: usize,
    orders: usize,
) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::math::spin::harmonics(data, size, rings, orders).map_err(hand::throw)?;
    Ok(value)
}

/// The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.
#[wasm_bindgen]
pub fn math_spin_mass(profile: &[f32], size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::mass(profile, size);
    Ok(value)
}

/// The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.
#[wasm_bindgen]
pub fn math_spin_mass_within(profile: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::mass_within(profile, size, radius);
    Ok(value)
}

/// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
#[wasm_bindgen]
pub fn math_spin_petals(copies: usize, order: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spin::petals(copies, order);
    Ok(value)
}

/// The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.
#[wasm_bindgen]
pub fn math_spin_profile(data: &[f32], size: usize, steps: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::spin::profile(data, size, steps).map_err(hand::throw)?;
    Ok(value)
}

/// Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.
#[wasm_bindgen]
pub fn math_spin_radial(
    data: &[f32],
    size: usize,
    out: usize,
    copies: usize,
    step: f64,
    blend: JsValue,
    samples: usize,
) -> Result<Vec<f32>, JsValue> {
    let blend = hand::from_js::<mrlyrs::math::spin::Blend>(&blend)?;
    let value = mrlyrs::math::spin::radial(data, size, out, copies, step, blend, samples)
        .map_err(hand::throw)?;
    Ok(value)
}

/// The radius of the corner circle of a square raster of the side, the last radius a profile reads.
#[wasm_bindgen]
pub fn math_spin_reach(size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::reach(size);
    Ok(value)
}

/// The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.
#[wasm_bindgen]
pub fn math_spin_ring(data: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::ring(data, size, radius).map_err(hand::throw)?;
    Ok(value)
}

/// The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.
#[wasm_bindgen]
pub fn math_spin_turns(power: &[f64]) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spin::turns(power);
    Ok(value)
}

/// The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.
#[wasm_bindgen]
pub fn math_spin_wheel(profile: &[f32], size: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::spin::wheel(profile, size);
    Ok(value)
}

/// The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.
#[wasm_bindgen]
pub fn math_spirograph_cell(width: usize, height: usize, reach: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spirograph::cell(width, height, reach);
    Ok(value)
}

/// The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.
#[wasm_bindgen]
pub fn math_spirograph_cover(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
    samples: usize,
    side: usize,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::cover(&track, &pencils, exact, samples, side)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.
#[wasm_bindgen]
pub fn math_spirograph_disc(track: JsValue, pencils: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::disc(&track, &pencils).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.
#[wasm_bindgen]
pub fn math_spirograph_distinct(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<usize, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::distinct(&track, &pencils, exact);
    Ok(value)
}

/// The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.
#[wasm_bindgen]
pub fn math_spirograph_frame(track: JsValue, pencils: JsValue) -> Result<Vec<f64>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::frame(&track, &pencils);
    Ok(value.to_vec())
}

/// The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.
#[wasm_bindgen]
pub fn math_spirograph_nodes(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::nodes(&track, &pencils, exact);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
}

/// Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.
#[wasm_bindgen]
pub fn math_spirograph_pencils(
    types: &[u8],
    width: usize,
    height: usize,
    mode: &str,
    reach: f64,
    jitter: f64,
    seed: u32,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spirograph::pencils(types, width, height, mode, reach, jitter, seed)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.
#[wasm_bindgen]
pub fn math_spirograph_point(track: JsValue, pencil: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrlyrs::math::spirograph::Pencil>(&pencil)?;
    let value = mrlyrs::math::spirograph::point(&track, &pencil, s);
    hand::to_js(&value)
}

/// The wheel's centre after `s` of path length.
#[wasm_bindgen]
pub fn math_spirograph_pose(track: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let value = mrlyrs::math::spirograph::pose(&track, s);
    hand::to_js(&value)
}

/// One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.
#[wasm_bindgen]
pub fn math_spirograph_representatives(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<Vec<usize>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::representatives(&track, &pencils, exact);
    Ok(value)
}

/// Counts the pencils by kind.
#[wasm_bindgen]
pub fn math_spirograph_seats(pencils: JsValue) -> Result<JsValue, JsValue> {
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::seats(&pencils);
    hand::to_js(&value)
}

/// The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.
#[wasm_bindgen]
pub fn math_spirograph_signed_area(track: JsValue, pencil: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrlyrs::math::spirograph::Pencil>(&pencil)?;
    let value = mrlyrs::math::spirograph::signed_area(&track, &pencil);
    hand::to_js(&value)
}

/// Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.
#[wasm_bindgen]
pub fn math_spirograph_trace(
    track: JsValue,
    pencils: JsValue,
    samples: usize,
) -> Result<Vec<f32>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::trace(&track, &pencils, samples).map_err(hand::throw)?;
    Ok(value)
}

/// Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.
#[wasm_bindgen]
pub fn math_spirograph_track(
    kind: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::spirograph::track(kind, ring, wheel, sides, laps).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.
#[wasm_bindgen]
pub fn math_spirograph_turn(track: JsValue, s: f64) -> Result<f64, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let value = mrlyrs::math::spirograph::turn(&track, s);
    Ok(value)
}

/// Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn math_three_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::carpet(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.
#[wasm_bindgen]
pub fn math_three_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Extracts the network of filled sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn math_three_core_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::core_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::core_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Builds the cube the universe code names, deepened to the given fractal level.
#[wasm_bindgen]
pub fn math_three_create(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::create(code, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.
#[wasm_bindgen]
pub fn math_three_diagonal_slice(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
    height: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::diagonal_slice(code, number, level, base, height)
        .map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.
#[wasm_bindgen]
pub fn math_three_diagonal_svg(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
    heights: &[usize],
    scale: usize,
) -> Result<String, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::diagonal_svg(code, number, level, base, heights, scale)
        .map_err(hand::throw)?;
    Ok(value)
}

/// Builds the dust cube, filled where every coordinate is even, at the given level.
#[wasm_bindgen]
pub fn math_three_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::dust(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Extracts the network of corners and edges outlining every filled site.
#[wasm_bindgen]
pub fn math_three_edge_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::edge_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::edge_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.
#[wasm_bindgen]
pub fn math_three_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.
#[wasm_bindgen]
pub fn math_three_extrude(cell: JsValue, axis: usize, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::three::extrude(&cell, axis, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Repeats every plane of a cube depth times along its leading axis, colors and tags with it.
#[wasm_bindgen]
pub fn math_three_extrude_cube(cell: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::extrude_cube(&cell, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.
#[wasm_bindgen]
pub fn math_three_faces(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::faces(&cell);
    Ok(value)
}

/// Returns the count of filled sites.
#[wasm_bindgen]
pub fn math_three_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::fills(&cell);
    Ok(value)
}

/// Builds a cube from its corner patterns, deepened to the given fractal level.
#[wasm_bindgen]
pub fn math_three_from_corners(
    corners: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value =
        mrlyrs::math::three::from_corners(&corners, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Parses a cell from its JSON, colors and tags included.
#[wasm_bindgen]
pub fn math_three_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::from_json(text).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cube from one string of digits per row, grouped plane by plane.
#[wasm_bindgen]
pub fn math_three_from_strings(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<Vec<Vec<String>>>(&data)?;
    let value = mrlyrs::math::three::from_strings(&data).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of faces buried between two filled sites, six per site less the exposed surface.
#[wasm_bindgen]
pub fn math_three_hidden(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::hidden(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
#[wasm_bindgen]
pub fn math_three_level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::level_set(number, levels, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Folds two or more cells into one by chained Kronecker combination.
#[wasm_bindgen]
pub fn math_three_magic(cells: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value = mrlyrs::math::three::magic::<2>(&cells).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value = mrlyrs::math::three::magic::<3>(&cells).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Tags every site with its Manhattan distance from the cube's center, the diamond shells.
#[wasm_bindgen]
pub fn math_three_manhattan_layers(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::manhattan_layers(cell);
    hand::cell3d_to_js(&value)
}

/// Merges the cells into one cube arranged width by height by depth.
#[wasm_bindgen]
pub fn math_three_merge(
    cells: JsValue,
    width: usize,
    height: usize,
    depth: usize,
) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
    let value = mrlyrs::math::three::merge(&cells, width, height, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cell by placing at each mask site the cell its value indexes.
#[wasm_bindgen]
pub fn math_three_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value = mrlyrs::math::three::mosaic::<2>(&mask, &cells).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value = mrlyrs::math::three::mosaic::<3>(&mask, &cells).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Builds the cube the name picks, deepened to the given fractal level.
#[wasm_bindgen]
pub fn math_three_named(design: JsValue, number: usize, level: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::math::three::named(design, number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the net cube, filled where at least two coordinates are odd, at the given level.
#[wasm_bindgen]
pub fn math_three_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::net(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cube whose every site turns on with probability density, at the given level.
#[wasm_bindgen]
pub fn math_three_noise(
    number: usize,
    level: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::three::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the solid cube at the given size and level.
#[wasm_bindgen]
pub fn math_three_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ones(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the 24 rotation triples that reach each distinct cube orientation.
#[wasm_bindgen]
pub fn math_three_orientations() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::orientations();
    hand::to_js(&value)
}

/// Builds the point cube, filled where every coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn math_three_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::point(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.
#[wasm_bindgen]
pub fn math_three_profile(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::profile(code, number, level, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.
#[wasm_bindgen]
pub fn math_three_project(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrlyrs::math::three::project(point);
    hand::to_js(&value)
}

/// Returns one outward quad per exposed face, scaled into the unit box.
#[wasm_bindgen]
pub fn math_three_quads(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::quads(&cell);
    hand::to_js(&value)
}

/// Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.
#[wasm_bindgen]
pub fn math_three_shadow(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrlyrs::math::three::shadow(point);
    Ok(hand::tuple_to_js(&[
        JsValue::from(value.0),
        JsValue::from(value.1),
    ]))
}

/// Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.
#[wasm_bindgen]
pub fn math_three_slice(cell: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::slice(&cell, axis, index).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Orients a copy of the cell by each mask value and merges them in the mask's shape.
#[wasm_bindgen]
pub fn math_three_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the star cube, filled where exactly one coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn math_three_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::star(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the first and last height a profile fills, or none when the design is empty.
#[wasm_bindgen]
pub fn math_three_support(counts: JsValue) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::u128_from_js)?;
    let value = mrlyrs::math::three::support(&counts);
    hand::to_js(&value)
}

/// Returns the count of filled faces exposed to void or the outside.
#[wasm_bindgen]
pub fn math_three_surface(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::surface(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.
#[wasm_bindgen]
pub fn math_three_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| {
        hand::map_from_js::<u8, _>(x1, hand::from_js::<String>)
    })?;
    let value = mrlyrs::math::three::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

/// Serializes the cell's shape and types to JSON, with colors and tags when present.
#[wasm_bindgen]
pub fn math_three_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_json(&cell);
    Ok(value)
}

/// Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.
#[wasm_bindgen]
pub fn math_three_to_obj(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_obj(&cell);
    Ok(value)
}

/// Unrolls the cube into one string of digits per row, grouped plane by plane.
#[wasm_bindgen]
pub fn math_three_to_strings(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_strings(&cell);
    hand::to_js(&value)
}

/// Extracts the network of empty sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn math_three_tunnel_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::tunnel_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::tunnel_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

/// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
#[wasm_bindgen]
pub fn math_three_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::void(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of empty sites.
#[wasm_bindgen]
pub fn math_three_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::voids(&cell);
    Ok(value)
}

/// Returns the filled-site count, the cube's volume.
#[wasm_bindgen]
pub fn math_three_volume(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::volume(&cell);
    Ok(value)
}

/// Returns the cell's edge-graph segments, scaled into the unit box.
#[wasm_bindgen]
pub fn math_three_wires(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::wires(&cell);
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from(math_three_Vec3 { inner: *x2 })))
    })
}

/// Builds the cube of rods along the x axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_xline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::xline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the x axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_xtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::xtree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of rods along the y axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_yline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::yline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the y axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_ytree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ytree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the all-void cube at the given size and level.
#[wasm_bindgen]
pub fn math_three_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::zeros(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of rods along the z axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_zline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::zline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the z axis at the given size and level.
#[wasm_bindgen]
pub fn math_three_ztree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ztree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.
#[wasm_bindgen]
pub fn math_tourbillon_eyes(qmax: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::eyes(qmax);
    hand::to_js(&value)
}

/// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
#[wasm_bindgen]
pub fn math_tourbillon_field(
    top: usize,
    size: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    mode: &str,
    blend: &str,
    seed: u32,
) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::tourbillon::field(
        top, size, schedule, increment, set, weights, mode, blend, seed,
    )
    .map_err(hand::throw)?;
    Ok(value)
}

/// The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.
#[wasm_bindgen]
pub fn math_tourbillon_layers(
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    seed: u32,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::layers(top, schedule, increment, set, weights, seed)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The least whole number of increments that closes a quarter turn, none once the count passes the cap.
#[wasm_bindgen]
pub fn math_tourbillon_period(increment: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::period(increment);
    hand::to_js(&value)
}

/// The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.
#[wasm_bindgen]
pub fn math_tourbillon_sharing(list: JsValue) -> Result<JsValue, JsValue> {
    let list = hand::from_js::<Vec<mrlyrs::math::tourbillon::Layer>>(&list)?;
    let value = mrlyrs::math::tourbillon::sharing(&list);
    hand::to_js(&value)
}

/// Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.
#[wasm_bindgen]
pub fn math_tourbillon_stack(
    list: JsValue,
    size: usize,
    mode: &str,
    blend: JsValue,
) -> Result<Vec<f32>, JsValue> {
    let list = hand::from_js::<Vec<mrlyrs::math::tourbillon::Layer>>(&list)?;
    let blend = hand::from_js::<mrlyrs::math::spin::Blend>(&blend)?;
    let value = mrlyrs::math::tourbillon::stack(&list, size, mode, blend).map_err(hand::throw)?;
    Ok(value)
}

/// Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.
#[wasm_bindgen]
pub fn math_tourbillon_stats(
    field: &[f32],
    size: usize,
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    blend: &str,
    seed: u32,
) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::stats(
        field, size, top, schedule, increment, set, weights, blend, seed,
    )
    .map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the payload bytes the cell's filled sites can hold, its length header paid for.
#[wasm_bindgen]
pub fn math_two_capacity(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::capacity(&cell);
    Ok(value)
}

/// Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.
#[wasm_bindgen]
pub fn math_two_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::carpet(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Takes the cell's full census in one reading.
#[wasm_bindgen]
pub fn math_two_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Builds the design a universe code names, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn math_two_create(
    code: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrlyrs::math::two::create(code, number, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the dust fractal, its seed on at every even-even site, deepened to the level.
#[wasm_bindgen]
pub fn math_two_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::dust(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Writes the payload over the cell's filled sites, repeating it until every site is spoken for.
#[wasm_bindgen]
pub fn math_two_embed(cell: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::embed(&cell, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the Euler characteristic of the filled sites, vertices less edges plus faces.
#[wasm_bindgen]
pub fn math_two_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Reads the payload back, the plain cell naming the sites the carried one wrote over.
#[wasm_bindgen]
pub fn math_two_extract(carrier: JsValue, carried: JsValue) -> Result<Vec<u8>, JsValue> {
    let carrier = hand::cell2d_from_js(&carrier)?;
    let carried = hand::cell2d_from_js(&carried)?;
    let value = mrlyrs::math::two::extract(&carrier, &carried).map_err(hand::throw)?;
    Ok(value)
}

/// Counts the filled sites of the cell.
#[wasm_bindgen]
pub fn math_two_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::fills(&cell);
    Ok(value)
}

/// Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn math_two_from_corners(
    corners: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value = mrlyrs::math::two::from_corners(&corners, number, level, rotation, base)
        .map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Restores a cell from its JSON string, colors and tags included.
#[wasm_bindgen]
pub fn math_two_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::from_json(text).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds a cell from rows of digits, the inverse of the text rendering.
#[wasm_bindgen]
pub fn math_two_from_strings(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::math::two::from_strings(&rows).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the hline fractal, its seed striped along odd rows, deepened to the level.
#[wasm_bindgen]
pub fn math_two_hline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::hline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the htree fractal, its seed striped along even rows, deepened to the level.
#[wasm_bindgen]
pub fn math_two_htree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::htree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the level-set design, filling every residue corner whose digits sum to a named level.
#[wasm_bindgen]
pub fn math_two_level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::two::level_set(number, levels, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tiles the mask over the shape and crops it, the perforation pattern itself.
#[wasm_bindgen]
pub fn math_two_mask(mask: JsValue, shape: &[usize]) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::two::mask(&mask, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.
#[wasm_bindgen]
pub fn math_two_merge(cells: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
    let value = mrlyrs::math::two::merge(&cells, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the design the name picks, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn math_two_named(
    design: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::math::two::named(design, number, level, rotation).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.
#[wasm_bindgen]
pub fn math_two_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::net(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds a random cell, each seed site drawn on with probability density, deepened to the level.
#[wasm_bindgen]
pub fn math_two_noise(
    number: usize,
    level: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::math::two::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds an all-filled cell of the given size and level.
#[wasm_bindgen]
pub fn math_two_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::ones(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// The five by five mask a carried mosaic lays its four tiles out under.
#[wasm_bindgen]
pub fn math_two_payload_frame() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::payload::frame();
    hand::tensor_to_js(&value)
}

/// Counts the faces of filled sites open to emptiness or the border.
#[wasm_bindgen]
pub fn math_two_perimeter(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::perimeter(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn math_two_png(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
    shape: JsValue,
) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let shape = hand::from_js::<mrlyrs::math::two::Shape>(&shape)?;
    let value = mrlyrs::math::two::png(&cell, scale, outline, width, shape).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the point fractal, its seed on at every odd-odd site, deepened to the level.
#[wasm_bindgen]
pub fn math_two_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::point(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Reads the payload back from a framed sheet, the plain fourth cell naming the sites.
#[wasm_bindgen]
pub fn math_two_read(sheet: JsValue, carrier: JsValue) -> Result<Vec<u8>, JsValue> {
    let sheet = hand::cell2d_from_js(&sheet)?;
    let carrier = hand::cell2d_from_js(&carrier)?;
    let value = mrlyrs::math::two::read(&sheet, &carrier).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the framed sheet of four same-sized cells, the fourth carrying the payload.
#[wasm_bindgen]
pub fn math_two_sheet(cells: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cells = hand::array_from_js::<_, 4>(&cells, hand::cell2d_from_js)?;
    let value = mrlyrs::math::two::sheet(&cells, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tiles quarter-turned copies of the cell as the 2d mask directs.
#[wasm_bindgen]
pub fn math_two_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.
#[wasm_bindgen]
pub fn math_two_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::star(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn math_two_svg(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
    shape: JsValue,
) -> Result<String, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let shape = hand::from_js::<mrlyrs::math::two::Shape>(&shape)?;
    let value = mrlyrs::math::two::svg(&cell, scale, outline, width, shape);
    Ok(value)
}

/// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
#[wasm_bindgen]
pub fn math_two_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| {
        hand::map_from_js::<u8, _>(x1, hand::from_js::<String>)
    })?;
    let value = mrlyrs::math::two::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

/// Lifts the flat cell into a cube one site deep, colors and tags with it.
#[wasm_bindgen]
pub fn math_two_to_3d(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::to_3d(&cell);
    hand::cell3d_to_js(&value)
}

/// Serializes the cell to a JSON string of its types, with colors and tags when present.
#[wasm_bindgen]
pub fn math_two_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::to_json(&cell);
    Ok(value)
}

/// Builds the vline fractal, its seed striped along odd columns, deepened to the level.
#[wasm_bindgen]
pub fn math_two_vline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::vline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.
#[wasm_bindgen]
pub fn math_two_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::void(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Counts the empty sites of the cell.
#[wasm_bindgen]
pub fn math_two_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::voids(&cell);
    Ok(value)
}

/// Builds the vtree fractal, its seed striped along even columns, deepened to the level.
#[wasm_bindgen]
pub fn math_two_vtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::vtree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds an all-empty cell of the given size and level.
#[wasm_bindgen]
pub fn math_two_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::zeros(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
#[wasm_bindgen]
pub fn num_apollonian_form(u: JsValue, v: JsValue) -> Result<JsValue, JsValue> {
    let u = hand::array_from_js::<_, 4>(&u, hand::i64_from_js)?;
    let v = hand::array_from_js::<_, 4>(&v, hand::i64_from_js)?;
    let value = mrlyrs::num::apollonian::form(u, v);
    Ok(JsValue::from_str(&value.to_string()))
}

/// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
#[wasm_bindgen]
pub fn num_apollonian_frame(p: JsValue) -> Result<Vec<f64>, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::frame(&p);
    Ok(value.to_vec())
}

/// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
#[wasm_bindgen]
pub fn num_apollonian_grow(name: &str, cap: JsValue) -> Result<JsValue, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrlyrs::num::apollonian::grow(name, cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
#[wasm_bindgen]
pub fn num_apollonian_is_ford(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrlyrs::num::apollonian::is_ford(c.inner);
    Ok(value)
}

/// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
#[wasm_bindgen]
pub fn num_apollonian_on_line(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrlyrs::num::apollonian::on_line(c.inner);
    Ok(value)
}

/// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
#[wasm_bindgen]
pub fn num_apollonian_reflect(q: JsValue, at: usize) -> Result<num_apollonian_Circle, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::reflect(&q, at);
    Ok(num_apollonian_Circle { inner: value })
}

/// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
#[wasm_bindgen]
pub fn num_apollonian_root(name: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::apollonian::root(name).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

/// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
#[wasm_bindgen]
pub fn num_apollonian_shadow(p: JsValue, order: usize) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::shadow(&p, order).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
#[wasm_bindgen]
pub fn num_apollonian_sound(q: JsValue) -> Result<bool, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::sound(&q);
    Ok(value)
}

/// The quadruple with the circle at the seat replaced by its reflection.
#[wasm_bindgen]
pub fn num_apollonian_swap(q: JsValue, at: usize) -> Result<JsValue, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::swap(&q, at);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

/// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
#[wasm_bindgen]
pub fn num_apollonian_touches(p: JsValue) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::touches(&p);
    hand::to_js(&value)
}

/// Adds two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn num_blend_add(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::add(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Convolves two sequences, keeping the exact prefix their shared length affords.
#[wasm_bindgen]
pub fn num_blend_cauchy(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::cauchy(&a, &b).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the monic characteristic polynomial of a recurrence, highest power first.
#[wasm_bindgen]
pub fn num_blend_characteristic(coefficients: JsValue) -> Result<JsValue, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrlyrs::num::blend::characteristic(&coefficients);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

/// Keeps every step-th term from the offset onward.
#[wasm_bindgen]
pub fn num_blend_decimate(a: JsValue, step: usize, offset: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::decimate(&a, step, offset).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the first differences of a sequence, one term shorter.
#[wasm_bindgen]
pub fn num_blend_delta(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::delta(&a);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.
#[wasm_bindgen]
pub fn num_blend_growth(coefficients: JsValue) -> Result<f64, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrlyrs::num::blend::growth(&coefficients);
    Ok(value)
}

/// Multiplies two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn num_blend_hadamard(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::hadamard(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
#[wasm_bindgen]
pub fn num_blend_recurrence(terms: JsValue) -> Result<JsValue, JsValue> {
    let terms = hand::list_from_js(&terms, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::recurrence(&terms);
    hand::option_to_js(value.as_ref(), |x1| {
        hand::list_to_js(x1, |x2| {
            Ok(hand::tuple_to_js(&[
                JsValue::from_str(&x2.0.to_string()),
                JsValue::from_str(&x2.1.to_string()),
            ]))
        })
    })
}

/// Multiplies every term of a sequence by the factor.
#[wasm_bindgen]
pub fn num_blend_scale(a: JsValue, factor: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let factor = hand::i128_from_js(&factor)?;
    let value = mrlyrs::num::blend::scale(&a, factor);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Drops the first terms of a sequence.
#[wasm_bindgen]
pub fn num_blend_shift(a: JsValue, count: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::shift(&a, count);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the partial sums of a sequence.
#[wasm_bindgen]
pub fn num_blend_sigma(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::sigma(&a).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Subtracts the second sequence from the first over their shared length.
#[wasm_bindgen]
pub fn num_blend_sub(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::sub(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Reports whether the packed function outputs one on exactly half of its inputs.
#[wasm_bindgen]
pub fn num_boolean_is_balanced(code: JsValue, n: usize) -> Result<bool, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::is_balanced(code, n);
    Ok(value)
}

/// Returns how far the packed function sits from every affine function, zero when it is one.
#[wasm_bindgen]
pub fn num_boolean_nonlinearity(code: JsValue, n: usize) -> Result<i64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::nonlinearity(code, n);
    Ok(value)
}

/// Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.
#[wasm_bindgen]
pub fn num_boolean_sac(code: JsValue, n: usize) -> Result<f64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::sac(code, n);
    Ok(value)
}

/// Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.
#[wasm_bindgen]
pub fn num_boolean_walsh_spectrum(code: JsValue, n: usize) -> Result<Vec<i64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::walsh_spectrum(code, n);
    Ok(value)
}

/// Returns the digits a bitmask names inside the base, ascending.
#[wasm_bindgen]
pub fn num_design_digits_of(mask: u32, base: JsValue) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::design::digits_of(mask, base);
    Ok(value)
}

/// Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.
#[wasm_bindgen]
pub fn num_design_echo_series(
    values: JsValue,
    log_x: &[f64],
    exponent: f64,
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrlyrs::num::design::echo_series(&values, log_x, exponent);
    Ok(value)
}

/// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
#[wasm_bindgen]
pub fn num_design_elements(
    base: JsValue,
    digits: JsValue,
    depth: usize,
) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrlyrs::num::design::elements(base, &digits, depth);
    Ok(value)
}

/// Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.
#[wasm_bindgen]
pub fn num_design_log_grid(values: JsValue, samples: usize) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrlyrs::num::design::log_grid(&values, samples);
    Ok(value)
}

/// Returns the running median of the power over a window of the given width, the window clamped at the ends.
#[wasm_bindgen]
pub fn num_design_median_floor(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::median_floor(power, width);
    Ok(value)
}

/// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
#[wasm_bindgen]
pub fn num_design_meter(mu: &[i8]) -> Result<Vec<i64>, JsValue> {
    let value = mrlyrs::num::design::meter(mu);
    Ok(value)
}

/// Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.
#[wasm_bindgen]
pub fn num_design_nearest(value: f64, list: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::design::nearest(value, list);
    Ok(value)
}

/// Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.
#[wasm_bindgen]
pub fn num_design_peaks(
    gamma: &[f64],
    score: &[f64],
    band: JsValue,
    threshold: f64,
) -> Result<Vec<usize>, JsValue> {
    let band = hand::from_js::<(f64, f64)>(&band)?;
    let value = mrlyrs::num::design::peaks(gamma, score, band, threshold).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
#[wasm_bindgen]
pub fn num_design_pole_lattice(base: JsValue, top: f64) -> Result<Vec<f64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::design::pole_lattice(base, top);
    Ok(value)
}

/// Reads the running meter at every point of the log grid and divides by x to the exponent.
#[wasm_bindgen]
pub fn num_design_resample(
    values: JsValue,
    running: JsValue,
    exponent: f64,
    log_x: &[f64],
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let running = hand::list_from_js(&running, hand::i64_from_js)?;
    let value = mrlyrs::num::design::resample(&values, &running, exponent, log_x);
    Ok(value)
}

/// Returns the power over its local median floor, the score a peak is read against.
#[wasm_bindgen]
pub fn num_design_score(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::score(power, width);
    Ok(value)
}

/// Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them.
#[wasm_bindgen]
pub fn num_design_size(digits: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrlyrs::num::design::size(&digits, depth);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.
#[wasm_bindgen]
pub fn num_design_spectrum(log_x: &[f64], series: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::design::spectrum(log_x, series).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.
#[wasm_bindgen]
pub fn num_design_upper_rms(series: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::design::upper_rms(series);
    Ok(value)
}

/// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
#[wasm_bindgen]
pub fn num_factor_aliquot(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::aliquot(number);
    Ok(value)
}

/// Returns whether two numbers share no divisor above one.
#[wasm_bindgen]
pub fn num_factor_coprime(a: usize, b: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::factor::coprime(a, b);
    Ok(value)
}

/// Builds every divisor of a wide number from its factorization, ascending, empty for zero.
#[wasm_bindgen]
pub fn num_factor_divisors(number: JsValue) -> Result<Vec<u64>, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::factor::divisors(number);
    Ok(value)
}

/// Returns the factorial of the number, the product of one through it, erring past thirty-four.
#[wasm_bindgen]
pub fn num_factor_factorial(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::factorial(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_factor_factorize(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::factorize(number);
    hand::to_js(&value)
}

/// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_factor_factorize_wide(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::factor::factorize_wide(number);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            hand::to_js(&x1.1)?,
        ]))
    })
}

/// Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.
#[wasm_bindgen]
pub fn num_factor_gcd(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::u128_from_js(&a)?;
    let b = hand::u128_from_js(&b)?;
    let value = mrlyrs::num::factor::gcd(a, b);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the least common multiple of two numbers, zero when either side is zero.
#[wasm_bindgen]
pub fn num_factor_lcm(a: usize, b: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::lcm(a, b);
    Ok(value)
}

/// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
#[wasm_bindgen]
pub fn num_factor_mobius(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::factor::mobius(number);
    Ok(value)
}

/// Sieves the Mobius values of zero through the limit in one pass.
#[wasm_bindgen]
pub fn num_factor_mobius_sieve(limit: usize) -> Result<Vec<i8>, JsValue> {
    let value = mrlyrs::num::factor::mobius_sieve(limit);
    Ok(value)
}

/// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
#[wasm_bindgen]
pub fn num_factor_radical(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::radical(number);
    Ok(value)
}

/// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
#[wasm_bindgen]
pub fn num_factor_reduce(numerator: JsValue, denominator: JsValue) -> Result<JsValue, JsValue> {
    let numerator = hand::u128_from_js(&numerator)?;
    let denominator = hand::u128_from_js(&denominator)?;
    let value = mrlyrs::num::factor::reduce(numerator, denominator);
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

/// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
#[wasm_bindgen]
pub fn num_factor_sigma(number: usize, power: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::sigma(number, power);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether no prime squares into the number, true for one and false for zero.
#[wasm_bindgen]
pub fn num_factor_squarefree(number: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::factor::squarefree(number);
    Ok(value)
}

/// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
#[wasm_bindgen]
pub fn num_factor_totient(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::totient(number);
    Ok(value)
}

/// Sieves the Euler totients of zero through n in one pass, the run beside the single value.
#[wasm_bindgen]
pub fn num_factor_totients(n: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::factor::totients(n);
    Ok(value)
}

/// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
#[wasm_bindgen]
pub fn num_factor_twisted(number: usize, rhythm: &[i8]) -> Result<i64, JsValue> {
    let value = mrlyrs::num::factor::twisted(number, rhythm);
    Ok(value)
}

/// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
#[wasm_bindgen]
pub fn num_fft_convolve(field: &[f64], kernel: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::convolve(field, kernel, size).map_err(hand::throw)?;
    Ok(value)
}

/// Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.
#[wasm_bindgen]
pub fn num_fft_convolve_with(
    field: &[f64],
    kernel_re: &[f64],
    kernel_im: &[f64],
    size: usize,
) -> Result<Vec<f64>, JsValue> {
    let value =
        mrlyrs::num::fft::convolve_with(field, kernel_re, kernel_im, size).map_err(hand::throw)?;
    Ok(value)
}

/// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
#[wasm_bindgen]
pub fn num_fft_embed_kernel(mask: &[u8], side: usize, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::embed_kernel(mask, side, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
#[wasm_bindgen]
pub fn num_fft_log_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::log_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
#[wasm_bindgen]
pub fn num_fft_magnitude_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::magnitude_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn num_fft_peak_ring(profile: &[f64]) -> Result<usize, JsValue> {
    let value = mrlyrs::num::fft::peak_ring(profile);
    Ok(value)
}

/// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn num_fft_peak_wavelength(profile: &[f64], size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::fft::peak_wavelength(profile, size);
    Ok(value)
}

/// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
#[wasm_bindgen]
pub fn num_fft_radial_profile(spectrum: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::radial_profile(spectrum, size).map_err(hand::throw)?;
    Ok(value)
}

/// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
#[wasm_bindgen]
pub fn num_fft_transform(field: &[f64], size: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::fft::transform(field, size).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.
#[wasm_bindgen]
pub fn num_gauss_classes(ring: JsValue, bound: JsValue) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let bound = hand::u64_from_js(&bound)?;
    let value = mrlyrs::num::gauss::classes(ring, bound);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Returns the norm from one through the limit with the most points and that count, the earliest on a tie.
#[wasm_bindgen]
pub fn num_gauss_peak(ring: JsValue, limit: usize) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let value = mrlyrs::num::gauss::peak(ring, limit);
    hand::to_js(&value)
}

/// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
#[wasm_bindgen]
pub fn num_gauss_shells(ring: JsValue, limit: usize) -> Result<Vec<u32>, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let value = mrlyrs::num::gauss::shells(ring, limit);
    Ok(value)
}

/// Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.
#[wasm_bindgen]
pub fn num_ladder_cofactor(
    design: &num_ladder_Design,
    s: &num_zeta_Complex,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::num::ladder::cofactor(&design.inner, s.inner, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(num_zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.
#[wasm_bindgen]
pub fn num_ladder_residue(
    design: &num_ladder_Design,
    m: usize,
    j: JsValue,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let j = hand::i64_from_js(&j)?;
    let value =
        mrlyrs::num::ladder::residue(&design.inner, m, j, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(num_zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Returns `zeta_F(s)` and the bound it is known to.
#[wasm_bindgen]
pub fn num_ladder_zeta(
    design: &num_ladder_Design,
    s: &num_zeta_Complex,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::num::ladder::zeta(&design.inner, s.inner, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(num_zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
#[wasm_bindgen]
pub fn num_lattice_coprime_pairs(n: usize) -> Result<u64, JsValue> {
    let value = mrlyrs::num::lattice::coprime_pairs(n);
    Ok(value)
}

/// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.
#[wasm_bindgen]
pub fn num_lattice_farey(order: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::farey(order);
    hand::to_js(&value)
}

/// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
#[wasm_bindgen]
pub fn num_lattice_grid(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::grid(n);
    hand::to_js(&value)
}

/// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
#[wasm_bindgen]
pub fn num_lattice_new_nodes(n: usize) -> Result<u64, JsValue> {
    let value = mrlyrs::num::lattice::new_nodes(n);
    Ok(value)
}

/// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
#[wasm_bindgen]
pub fn num_lattice_pi_estimate(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::pi_estimate(n);
    Ok(value)
}

/// Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.
#[wasm_bindgen]
pub fn num_lattice_recovered(n: usize, dimension: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::recovered(n, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The density the visible count of a window in the dimension walks to, one over zeta of the dimension.
#[wasm_bindgen]
pub fn num_lattice_visible_density(dimension: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::visible_density(dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.
#[wasm_bindgen]
pub fn num_lattice_zeta_factor(dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::zeta_factor(dimension);
    hand::to_js(&value)
}

/// The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.
#[wasm_bindgen]
pub fn num_lattice_zeta_whole(s: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::zeta_whole(s).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the count of allowed windows `card W`, the bits the code sets inside its window range.
#[wasm_bindgen]
pub fn num_memory_allowed_windows(rule: &num_memory_Rule) -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::allowed_windows(&rule.inner);
    Ok(value)
}

/// Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.
#[wasm_bindgen]
pub fn num_memory_cells(rule: &num_memory_Rule, level: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::memory::cells(&rule.inner, level);
    Ok(value)
}

/// Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.
#[wasm_bindgen]
pub fn num_memory_counts(rule: &num_memory_Rule, levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::memory::counts(&rule.inner, levels);
    Ok(value)
}

/// Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.
#[wasm_bindgen]
pub fn num_memory_exponent(rule: &num_memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::exponent(&rule.inner);
    Ok(value)
}

/// Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.
#[wasm_bindgen]
pub fn num_memory_kappa(rule: &num_memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::kappa(&rule.inner);
    Ok(value)
}

/// Returns the Perron root of the transfer matrix, the count's growth per level.
#[wasm_bindgen]
pub fn num_memory_perron(rule: &num_memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::perron(&rule.inner);
    Ok(value)
}

/// Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.
#[wasm_bindgen]
pub fn num_memory_transfer(rule: &num_memory_Rule) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::memory::transfer(&rule.inner);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the run-boundary word, one wherever a letter differs from the next.
#[wasm_bindgen]
pub fn num_morse_boundary(word: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::boundary(word);
    Ok(value)
}

/// Exclusive-ors two grids of the same length, site by site.
#[wasm_bindgen]
pub fn num_morse_difference(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::difference(a, b);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the digit rule.
#[wasm_bindgen]
pub fn num_morse_digits(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::digits(length);
    Ok(value)
}

/// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
#[wasm_bindgen]
pub fn num_morse_doubling(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::doubling(length);
    Ok(value)
}

/// Counts the sites where two grids of the same length differ.
#[wasm_bindgen]
pub fn num_morse_faults(a: &[u8], b: &[u8]) -> Result<usize, JsValue> {
    let value = mrlyrs::num::morse::faults(a, b);
    Ok(value)
}

/// Tests a grid against the Kronecker power of its own corner tile.
#[wasm_bindgen]
pub fn num_morse_fold(grid: &[u8], side: usize, number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::morse::fold(grid, side, number).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
#[wasm_bindgen]
pub fn num_morse_letter(place: JsValue) -> Result<u8, JsValue> {
    let place = hand::u64_from_js(&place)?;
    let value = mrlyrs::num::morse::letter(place);
    Ok(value)
}

/// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
#[wasm_bindgen]
pub fn num_morse_lift(kind: JsValue, side: usize) -> Result<Vec<u8>, JsValue> {
    let kind = hand::from_js::<mrlyrs::num::morse::Lift>(&kind)?;
    let value = mrlyrs::num::morse::lift(kind, side);
    Ok(value)
}

/// Folds a tile of the side into its Kronecker power at the level, one bit per site.
#[wasm_bindgen]
pub fn num_morse_power(tile: &[u8], number: usize, level: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::power(tile, number, level).map_err(hand::throw)?;
    Ok(value)
}

/// Repeats a tile until it fills a grid of the side.
#[wasm_bindgen]
pub fn num_morse_repeat(tile: &[u8], number: usize, side: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::repeat(tile, number, side);
    Ok(value)
}

/// Returns the lengths of the maximal blocks of one repeated letter, in order.
#[wasm_bindgen]
pub fn num_morse_runs(word: &[u8]) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::morse::runs(word);
    Ok(value)
}

/// Returns the substitution stage after the rounds, a word of length two to the rounds.
#[wasm_bindgen]
pub fn num_morse_stage(rounds: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::stage(rounds);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
#[wasm_bindgen]
pub fn num_morse_substitution(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::substitution(length);
    Ok(value)
}

/// Blows a grid up by the scale, every site becoming a scale-by-scale block.
#[wasm_bindgen]
pub fn num_morse_upsample(grid: &[u8], side: usize, scale: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::upsample(grid, side, scale);
    Ok(value)
}

/// Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.
#[wasm_bindgen]
pub fn num_prime_chart(top: usize, bins: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::chart(top, bins);
    hand::to_js(&value)
}

/// Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.
#[wasm_bindgen]
pub fn num_prime_flags(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::flags(limit);
    hand::to_js(&value)
}

/// Returns the count of unordered pairs of primes summing to the number, zero below four.
#[wasm_bindgen]
pub fn num_prime_goldbach(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::goldbach(number);
    Ok(value)
}

/// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
#[wasm_bindgen]
pub fn num_prime_goldbach_record(top: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::prime::goldbach_record(top);
    Ok(value)
}

/// Returns whether the number is prime, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_prime_is_prime(number: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::prime::is_prime(number);
    Ok(value)
}

/// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
#[wasm_bindgen]
pub fn num_prime_pile(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::prime::pile(number);
    hand::to_js(&value)
}

/// Returns the count of primes at or below n.
#[wasm_bindgen]
pub fn num_prime_prime_count(n: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::prime_count(n);
    Ok(value)
}

/// Returns the smallest prime at or above the number.
#[wasm_bindgen]
pub fn num_prime_prime_from(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::prime_from(number);
    Ok(value)
}

/// Returns the primes up to the limit, the finished sieve read as a list.
#[wasm_bindgen]
pub fn num_prime_primes(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::prime::primes(limit);
    Ok(value)
}

/// Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.
#[wasm_bindgen]
pub fn num_prime_rectangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::rectangles(number);
    hand::to_js(&value)
}

/// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
#[wasm_bindgen]
pub fn num_prime_splits(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::splits(number);
    hand::to_js(&value)
}

/// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
#[wasm_bindgen]
pub fn num_prime_squares(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::squares(number);
    hand::to_js(&value)
}

/// Returns one prime object for every prime up to and including the limit.
#[wasm_bindgen]
pub fn num_prime_study(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::study(limit);
    hand::to_js(&value)
}

/// Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.
#[wasm_bindgen]
pub fn num_radix_flowsnake() -> Result<num_radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::flowsnake().map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.
#[wasm_bindgen]
pub fn num_radix_gasket() -> Result<num_radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::gasket().map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.
#[wasm_bindgen]
pub fn num_radix_koch() -> Result<num_radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::koch().map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.
#[wasm_bindgen]
pub fn num_radix_terdragon() -> Result<num_radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::terdragon().map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.
#[wasm_bindgen]
pub fn num_radix_tile(m: JsValue, code: JsValue) -> Result<num_radix_Radix, JsValue> {
    let m = hand::u64_from_js(&m)?;
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::radix::tile(m, code).map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.
#[wasm_bindgen]
pub fn num_radix_twindragon() -> Result<num_radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::twindragon().map_err(hand::throw)?;
    Ok(num_radix_Radix { inner: value })
}

/// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
#[wasm_bindgen]
pub fn num_series_basel(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::basel(n);
    Ok(value)
}

/// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
#[wasm_bindgen]
pub fn num_series_bernoulli(count: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::series::bernoulli(count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

/// Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.
#[wasm_bindgen]
pub fn num_series_beta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::beta(s, terms);
    Ok(value)
}

/// Returns the powers of two up to the limit.
#[wasm_bindgen]
pub fn num_series_binary(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::binary(limit);
    Ok(value)
}

/// Returns the distinct Catalan numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_catalan(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::catalan(limit);
    Ok(value)
}

/// Returns the mod-three rhythm of the number: zero, one, minus one.
#[wasm_bindgen]
pub fn num_series_chi3(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi3(number);
    Ok(value)
}

/// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
#[wasm_bindgen]
pub fn num_series_chi4(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi4(number);
    Ok(value)
}

/// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
#[wasm_bindgen]
pub fn num_series_chi8(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi8(number);
    Ok(value)
}

/// Returns the L-series partial sum with a periodic rhythm painted on the terms.
#[wasm_bindgen]
pub fn num_series_dirichlet(s: f64, rhythm: &[i8], terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::dirichlet(s, rhythm, terms);
    Ok(value)
}

/// Returns one plus one over n raised to the n, walking to the natural base.
#[wasm_bindgen]
pub fn num_series_e_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::e_partial(n);
    Ok(value)
}

/// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
#[wasm_bindgen]
pub fn num_series_euler_gamma_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::euler_gamma_partial(n);
    Ok(value)
}

/// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
#[wasm_bindgen]
pub fn num_series_euler_product(s: f64, limit: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::euler_product(s, limit);
    Ok(value)
}

/// Returns the even numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_evens(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::evens(limit);
    Ok(value)
}

/// Returns the distinct Fibonacci numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_fibonacci(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::fibonacci(limit);
    Ok(value)
}

/// Returns the partial harmonic sum, the reciprocals of one through the term count.
#[wasm_bindgen]
pub fn num_series_harmonic(terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::harmonic(terms);
    Ok(value)
}

/// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
#[wasm_bindgen]
pub fn num_series_lambda(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::lambda(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
#[wasm_bindgen]
pub fn num_series_leibniz(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::leibniz(n);
    Ok(value)
}

/// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
#[wasm_bindgen]
pub fn num_series_li(x: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::li(x);
    Ok(value)
}

/// Returns the Mertens function at n, the Mobius values of one through n summed.
#[wasm_bindgen]
pub fn num_series_mertens(n: usize) -> Result<i64, JsValue> {
    let value = mrlyrs::num::series::mertens(n);
    Ok(value)
}

/// Returns the odd numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_odds(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::odds(limit);
    Ok(value)
}

/// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
#[wasm_bindgen]
pub fn num_series_visible(limit: usize, dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::series::visible(limit, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
#[wasm_bindgen]
pub fn num_series_wallis_half_pi(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::wallis_half_pi(n);
    Ok(value)
}

/// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
#[wasm_bindgen]
pub fn num_series_wallis_quarter_pi(factors: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::wallis_quarter_pi(factors);
    Ok(value)
}

/// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
#[wasm_bindgen]
pub fn num_series_zeta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::zeta(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
#[wasm_bindgen]
pub fn num_sieve_cells(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::cells(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
#[wasm_bindgen]
pub fn num_sieve_exponent(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::exponent(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
#[wasm_bindgen]
pub fn num_sieve_flat_word(side: JsValue, levels: usize) -> Result<Vec<u64>, JsValue> {
    let side = hand::u64_from_js(&side)?;
    let value = mrlyrs::num::sieve::flat_word(side, levels);
    Ok(value)
}

/// Returns the punctures the word makes, one per surviving cell at every level.
#[wasm_bindgen]
pub fn num_sieve_holes(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::holes(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
#[wasm_bindgen]
pub fn num_sieve_limit(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::limit(&word, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
#[wasm_bindgen]
pub fn num_sieve_odd_word(levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::sieve::odd_word(levels);
    Ok(value)
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
#[wasm_bindgen]
pub fn num_sieve_punctures(word: JsValue, dimension: u32) -> Result<Vec<u64>, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::punctures(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
#[wasm_bindgen]
pub fn num_sieve_raster(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::raster(&word).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::typed(&(value.1)[..]),
    ]))
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
#[wasm_bindgen]
pub fn num_sieve_ratio(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::ratio(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the side of the word, the product of its letters' sides.
#[wasm_bindgen]
pub fn num_sieve_side(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::side(&word).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
#[wasm_bindgen]
pub fn num_sieve_solid_limit() -> Result<f64, JsValue> {
    let value = mrlyrs::num::sieve::solid_limit();
    Ok(value)
}

/// Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.
#[wasm_bindgen]
pub fn num_spiral_diagonal(
    lattice: JsValue,
    side: usize,
    a: JsValue,
    b: JsValue,
    c: JsValue,
) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
    let a = hand::i64_from_js(&a)?;
    let b = hand::i64_from_js(&b)?;
    let c = hand::i64_from_js(&c)?;
    let value = mrlyrs::num::spiral::diagonal(lattice, side, a, b, c);
    hand::to_js(&value)
}

/// Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.
#[wasm_bindgen]
pub fn num_spiral_level_of(n: JsValue, base: JsValue) -> Result<u32, JsValue> {
    let n = hand::u64_from_js(&n)?;
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::spiral::level_of(n, base);
    Ok(value)
}

/// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
#[wasm_bindgen]
pub fn num_spiral_marks(mark: JsValue, limit: usize) -> Result<Vec<i8>, JsValue> {
    let mark = hand::from_js::<mrlyrs::num::spiral::Mark>(&mark)?;
    let value = mrlyrs::num::spiral::marks(mark, limit);
    Ok(value)
}

/// Winds one to the top on the square spiral and lays a square tile on every cell, the snail.
#[wasm_bindgen]
pub fn num_spiral_snail(base: JsValue, top: JsValue, growth: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let top = hand::u64_from_js(&top)?;
    let growth = hand::from_js::<mrlyrs::num::spiral::Growth>(&growth)?;
    let value = mrlyrs::num::spiral::snail(base, top, growth);
    hand::to_js(&value)
}

/// The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.
#[wasm_bindgen]
pub fn num_zeta_bump(u: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::bump(u);
    Ok(value)
}

/// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
#[wasm_bindgen]
pub fn num_zeta_corrections(p: f64) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::zeta::corrections(p);
    Ok(value.to_vec())
}

/// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
#[wasm_bindgen]
pub fn num_zeta_kernel(p: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::kernel(p);
    Ok(value)
}

/// Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.
#[wasm_bindgen]
pub fn num_zeta_mellin(s: &num_zeta_Complex) -> Result<num_zeta_Complex, JsValue> {
    let value = mrlyrs::num::zeta::mellin(s.inner);
    Ok(num_zeta_Complex { inner: value })
}

/// Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.
#[wasm_bindgen]
pub fn num_zeta_novelty_main() -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::novelty_main();
    Ok(value)
}

/// Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.
#[wasm_bindgen]
pub fn num_zeta_novelty_wave(gammas: &[f64], coef: JsValue, log_y: f64) -> Result<f64, JsValue> {
    let coef = hand::list_from_js(&coef, |x1| {
        hand::from_js::<mrlyrs::num::zeta::Complex>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::zeta::novelty_wave(gammas, &coef, log_y);
    Ok(value)
}

/// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
#[wasm_bindgen]
pub fn num_zeta_psi_formula(x: f64, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::psi_formula(x, gammas);
    Ok(value)
}

/// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
#[wasm_bindgen]
pub fn num_zeta_psi_stair(x: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::zeta::psi_stair(x);
    Ok(value)
}

/// Returns a positive real base raised to a complex exponent.
#[wasm_bindgen]
pub fn num_zeta_raise(base: f64, exponent: &num_zeta_Complex) -> Result<num_zeta_Complex, JsValue> {
    let value = mrlyrs::num::zeta::raise(base, exponent.inner);
    Ok(num_zeta_Complex { inner: value })
}

/// Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.
#[wasm_bindgen]
pub fn num_zeta_sharp_novelty(prefix: JsValue, y: f64) -> Result<f64, JsValue> {
    let prefix = hand::list_from_js(&prefix, hand::u64_from_js)?;
    let value = mrlyrs::num::zeta::sharp_novelty(&prefix, y);
    Ok(value)
}

/// Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.
#[wasm_bindgen]
pub fn num_zeta_smoothed_novelty(phi: JsValue, y: f64, main: f64) -> Result<f64, JsValue> {
    let phi = hand::list_from_js(&phi, hand::u64_from_js)?;
    let value = mrlyrs::num::zeta::smoothed_novelty(&phi, y, main);
    Ok(value)
}

/// The height of an equilateral triangle over its side, the squash a hex rendering wears.
#[wasm_bindgen]
pub fn core_HEX_RATIO() -> Result<f64, JsValue> {
    let value = mrlyrs::core::HEX_RATIO;
    Ok(value)
}

/// The eight bytes every png file starts with.
#[wasm_bindgen]
pub fn core_PNG_MAGIC() -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::core::PNG_MAGIC;
    Ok(value.to_vec())
}

/// The fully transparent color.
#[wasm_bindgen]
pub fn core_colors_ALPHA() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::ALPHA;
    Ok(hand::color_to_js(value))
}

/// The palette black, #000000.
#[wasm_bindgen]
pub fn core_colors_BLACK() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BLACK;
    Ok(hand::color_to_js(value))
}

/// The palette blue, #008cff.
#[wasm_bindgen]
pub fn core_colors_BLUE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BLUE;
    Ok(hand::color_to_js(value))
}

/// The palette brown, #b18462.
#[wasm_bindgen]
pub fn core_colors_BROWN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::BROWN;
    Ok(hand::color_to_js(value))
}

/// The palette cyan, #1ec9f3.
#[wasm_bindgen]
pub fn core_colors_CYAN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::CYAN;
    Ok(hand::color_to_js(value))
}

/// The dark theme.
#[wasm_bindgen]
pub fn core_colors_DARK() -> Result<core_colors_Theme, JsValue> {
    let value = mrlyrs::core::colors::DARK;
    Ok(core_colors_Theme { inner: value })
}

/// The palette gray, #8e8e93.
#[wasm_bindgen]
pub fn core_colors_GRAY() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::GRAY;
    Ok(hand::color_to_js(value))
}

/// The palette green, #32cc58.
#[wasm_bindgen]
pub fn core_colors_GREEN() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::GREEN;
    Ok(hand::color_to_js(value))
}

/// The palette indigo, #6768fa.
#[wasm_bindgen]
pub fn core_colors_INDIGO() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::INDIGO;
    Ok(hand::color_to_js(value))
}

/// The light theme.
#[wasm_bindgen]
pub fn core_colors_LIGHT() -> Result<core_colors_Theme, JsValue> {
    let value = mrlyrs::core::colors::LIGHT;
    Ok(core_colors_Theme { inner: value })
}

/// The palette mint, #00d1bb.
#[wasm_bindgen]
pub fn core_colors_MINT() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::MINT;
    Ok(hand::color_to_js(value))
}

/// The fifteen names, in palette order.
#[wasm_bindgen]
pub fn core_colors_NAMES() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::NAMES;
    hand::to_js(&value)
}

/// The palette orange, #ff8f2c.
#[wasm_bindgen]
pub fn core_colors_ORANGE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::ORANGE;
    Ok(hand::color_to_js(value))
}

/// The fifteen colors, in name order.
#[wasm_bindgen]
pub fn core_colors_PALETTE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PALETTE;
    hand::list_to_js(&value, |x1| Ok(hand::color_to_js(*x1)))
}

/// The palette pink, #ff325a.
#[wasm_bindgen]
pub fn core_colors_PINK() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PINK;
    Ok(hand::color_to_js(value))
}

/// The palette purple, #d332e9.
#[wasm_bindgen]
pub fn core_colors_PURPLE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::PURPLE;
    Ok(hand::color_to_js(value))
}

/// The palette red, #ff3d40.
#[wasm_bindgen]
pub fn core_colors_RED() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::RED;
    Ok(hand::color_to_js(value))
}

/// The palette teal, #00cad8.
#[wasm_bindgen]
pub fn core_colors_TEAL() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::TEAL;
    Ok(hand::color_to_js(value))
}

/// The palette white, #ffffff.
#[wasm_bindgen]
pub fn core_colors_WHITE() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::WHITE;
    Ok(hand::color_to_js(value))
}

/// The palette yellow, #ffd100.
#[wasm_bindgen]
pub fn core_colors_YELLOW() -> Result<JsValue, JsValue> {
    let value = mrlyrs::core::colors::YELLOW;
    Ok(hand::color_to_js(value))
}

/// The playback rate of every animation, in frames per second.
#[wasm_bindgen]
pub fn font_FPS() -> Result<usize, JsValue> {
    let value = mrlyrs::font::FPS;
    Ok(value)
}

/// The default number of frames a cycle rests between movements.
#[wasm_bindgen]
pub fn font_HOLD() -> Result<usize, JsValue> {
    let value = mrlyrs::font::HOLD;
    Ok(value)
}

/// The five classic designs of the plane.
#[wasm_bindgen]
pub fn gen_recipe_CLASSICS_2D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::CLASSICS_2D;
    hand::to_js(&value)
}

/// The six classic designs of the cube.
#[wasm_bindgen]
pub fn gen_recipe_CLASSICS_3D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::CLASSICS_3D;
    hand::to_js(&value)
}

/// The deepest fractal level a tile may take.
#[wasm_bindgen]
pub fn gen_recipe_MAX_LEVEL() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_LEVEL;
    Ok(value)
}

/// The largest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn gen_recipe_MAX_SIDE() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_SIDE;
    Ok(value)
}

/// The most slots a magic tile may take.
#[wasm_bindgen]
pub fn gen_recipe_MAX_SLOTS() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_SLOTS;
    Ok(value)
}

/// The smallest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn gen_recipe_MIN_SIDE() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MIN_SIDE;
    Ok(value)
}

/// The most cells a code walk visits, so that the walk stays within `2^20` codes.
#[wasm_bindgen]
pub fn math_bang_baseq_WALK_LIMIT() -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::baseq::WALK_LIMIT;
    Ok(value)
}

/// The five antis of the plane, the complements of the five classics in order.
#[wasm_bindgen]
pub fn math_bang_catalog_ANTIS_2D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::ANTIS_2D;
    hand::to_js(&value)
}

/// The six antis of the cube: point, dust, the three lines and the star.
#[wasm_bindgen]
pub fn math_bang_catalog_ANTIS_3D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::ANTIS_3D;
    hand::to_js(&value)
}

/// The widest side a profile spans.
#[wasm_bindgen]
pub fn math_counts_diagonal_WIDEST() -> Result<usize, JsValue> {
    let value = mrlyrs::math::counts::diagonal::WIDEST;
    Ok(value)
}

/// The largest corner count the tally press accepts, keeping its table a million rows.
#[wasm_bindgen]
pub fn math_press_CORNERS() -> Result<usize, JsValue> {
    let value = mrlyrs::math::press::CORNERS;
    Ok(value)
}

/// The default residue base.
#[wasm_bindgen]
pub fn math_rules_BASE() -> Result<usize, JsValue> {
    let value = mrlyrs::math::rules::BASE;
    Ok(value)
}

/// The refine output ceiling in cells.
#[wasm_bindgen]
pub fn math_shape_REFINE_LIMIT() -> Result<usize, JsValue> {
    let value = mrlyrs::math::shape::REFINE_LIMIT;
    Ok(value)
}

/// The triangle code for a filled site.
#[wasm_bindgen]
pub fn math_six_FILL() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::FILL;
    Ok(value)
}

/// The triangle code for the backdrop outside the figure.
#[wasm_bindgen]
pub fn math_six_GRID() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::GRID;
    Ok(value)
}

/// The triangle code for a cube's left face in the iso view.
#[wasm_bindgen]
pub fn math_six_LEFT() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::LEFT;
    Ok(value)
}

/// The triangle code for a cube's right face in the iso view.
#[wasm_bindgen]
pub fn math_six_RIGHT() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::RIGHT;
    Ok(value)
}

/// The triangle code for a cube's top face in the iso view.
#[wasm_bindgen]
pub fn math_six_UP() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::UP;
    Ok(value)
}

/// The triangle code for an empty site.
#[wasm_bindgen]
pub fn math_six_VOID() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::VOID;
    Ok(value)
}

/// The most laps of a line or a polygon.
#[wasm_bindgen]
pub fn math_spirograph_LAPS_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::LAPS_CAP;
    Ok(value)
}

/// The most pencils a wheel seats.
#[wasm_bindgen]
pub fn math_spirograph_PENCIL_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::PENCIL_CAP;
    Ok(value)
}

/// The most points one trace returns.
#[wasm_bindgen]
pub fn math_spirograph_POINT_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::POINT_CAP;
    Ok(value)
}

/// The largest radius of a ring or a wheel.
#[wasm_bindgen]
pub fn math_spirograph_RADIUS_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::RADIUS_CAP;
    Ok(value)
}

/// The largest raster side a cover rasters.
#[wasm_bindgen]
pub fn math_spirograph_RASTER_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::RASTER_CAP;
    Ok(value)
}

/// The fewest and the most sides of a polygon track.
#[wasm_bindgen]
pub fn math_spirograph_SIDES() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spirograph::SIDES;
    hand::to_js(&value)
}

/// The most circles one growth makes before it gives up.
#[wasm_bindgen]
pub fn num_apollonian_CIRCLE_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::num::apollonian::CIRCLE_CAP;
    Ok(value)
}

/// The largest curvature a packing is grown to.
#[wasm_bindgen]
pub fn num_apollonian_CURVATURE_CAP() -> Result<i64, JsValue> {
    let value = mrlyrs::num::apollonian::CURVATURE_CAP;
    Ok(value)
}

/// The deepest the Farey stack is read against a packing.
#[wasm_bindgen]
pub fn num_apollonian_ORDER_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::num::apollonian::ORDER_CAP;
    Ok(value)
}

/// The root quadruples on offer: the strip first, then the bounded packings named by their curvatures.
#[wasm_bindgen]
pub fn num_apollonian_ROOTS() -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::apollonian::ROOTS;
    hand::to_js(&value)
}

/// The relative rounding allowance the double-precision matrix ladder charges against the scale it carries.
#[wasm_bindgen]
pub fn num_automaton_ROUNDING() -> Result<f64, JsValue> {
    let value = mrlyrs::num::automaton::ROUNDING;
    Ok(value)
}

/// The ordinates of the first fourteen nontrivial zeros of the Riemann zeta function, the imaginary parts of the zeros on the critical line in ascending order.
#[wasm_bindgen]
pub fn num_design_ZETA_ORDINATES() -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::ZETA_ORDINATES;
    Ok(value.to_vec())
}

/// The relative rounding allowance the double-precision ladder charges against the scale it carries.
#[wasm_bindgen]
pub fn num_ladder_ROUNDING() -> Result<f64, JsValue> {
    let value = mrlyrs::num::ladder::ROUNDING;
    Ok(value)
}

/// The largest digit span a rule may read, so its code fits a `u64`.
#[wasm_bindgen]
pub fn num_memory_SPAN() -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::SPAN;
    Ok(value)
}

/// The sweep cap of the power iteration.
#[wasm_bindgen]
pub fn num_memory_SWEEPS() -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::SWEEPS;
    Ok(value)
}

/// The absolute `l^1` move of the normalised iterate that stops the power iteration, counted only when it holds over three consecutive sweeps.
#[wasm_bindgen]
pub fn num_memory_TOLERANCE() -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::TOLERANCE;
    Ok(value)
}

/// Lists the lifts in the order the gallery draws them.
#[wasm_bindgen]
pub fn num_morse_LIFTS() -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::morse::LIFTS;
    hand::to_js(&value)
}

/// The Apery constant, the value zeta takes at three.
#[wasm_bindgen]
pub fn num_series_APERY() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::APERY;
    Ok(value)
}

/// The Basel constant, pi squared over six, the value zeta takes at two.
#[wasm_bindgen]
pub fn num_series_BASEL() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::BASEL;
    Ok(value)
}

/// The Catalan constant, the value the Dirichlet beta function takes at two.
#[wasm_bindgen]
pub fn num_series_CATALAN() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::CATALAN;
    Ok(value)
}

/// The Euler constant, the limit of the harmonic sum less the logarithm.
#[wasm_bindgen]
pub fn num_series_EULER() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::EULER;
    Ok(value)
}

/// The visible density, six over pi squared, the share of lattice pairs that are coprime.
#[wasm_bindgen]
pub fn num_series_VISIBLE() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::VISIBLE;
    Ok(value)
}

/// The limit of the plane Wallis sieve's surviving area, pi over four.
#[wasm_bindgen]
pub fn num_sieve_PLANE_LIMIT() -> Result<f64, JsValue> {
    let value = mrlyrs::num::sieve::PLANE_LIMIT;
    Ok(value)
}

/// The t where the walk hands over from Euler-Maclaurin to Riemann-Siegel.
#[wasm_bindgen]
pub fn num_zeta_JOIN() -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::JOIN;
    Ok(value)
}

/// A paletted image: rows of palette indices and the palette they point into, hex strings in json.
#[wasm_bindgen]
pub struct core_Image {
    inner: mrlyrs::core::Image,
}

#[wasm_bindgen]
impl core_Image {
    /// Reads the Image from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<core_Image, JsValue> {
        Ok(core_Image {
            inner: hand::from_js(&data)?,
        })
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
    pub fn from_pixels(
        width: usize,
        height: usize,
        pixels: JsValue,
    ) -> Result<core_Image, JsValue> {
        let pixels = hand::from_js::<Vec<[u8; 4]>>(&pixels)?;
        let value = mrlyrs::core::Image::from_pixels(width, height, &pixels);
        Ok(core_Image { inner: value })
    }
    /// Builds an image from its four parts.
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: usize,
        height: usize,
        rows: JsValue,
        palette: JsValue,
    ) -> Result<core_Image, JsValue> {
        let rows = hand::from_js::<Vec<Vec<usize>>>(&rows)?;
        let palette = hand::list_from_js(&palette, hand::color_from_js)?;
        let value = mrlyrs::core::Image::new(width, height, rows, palette);
        Ok(core_Image { inner: value })
    }
    /// Encodes the image as a png at the given scale.
    pub fn png(&self, scale: usize) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.png(scale).map_err(hand::throw)?;
        Ok(value)
    }
    /// Resamples the image to a new size, its palette rebuilt from the blended pixels.
    pub fn resample(
        &self,
        width: usize,
        height: usize,
        filter: JsValue,
    ) -> Result<core_Image, JsValue> {
        let filter = hand::from_js::<mrlyrs::core::Filter>(&filter)?;
        let value = self
            .inner
            .resample(width, height, filter)
            .map_err(hand::throw)?;
        Ok(core_Image { inner: value })
    }
}

/// One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both.
#[wasm_bindgen]
pub struct core_colors_Theme {
    inner: mrlyrs::core::colors::Theme,
}

#[wasm_bindgen]
impl core_colors_Theme {
    /// Reads the Theme from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<core_colors_Theme, JsValue> {
        Ok(core_colors_Theme {
            inner: hand::from_js(&data)?,
        })
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
pub struct core_paint_Paint {
    inner: mrlyrs::core::paint::Paint,
}

#[wasm_bindgen]
impl core_paint_Paint {
    /// Reads the Paint from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<core_paint_Paint, JsValue> {
        Ok(core_paint_Paint {
            inner: hand::from_js(&data)?,
        })
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
    pub fn new(edition: JsValue) -> Result<core_paint_Paint, JsValue> {
        let edition = hand::from_js::<mrlyrs::core::paint::Edition>(&edition)?;
        let value = mrlyrs::core::paint::Paint::new(edition);
        Ok(core_paint_Paint { inner: value })
    }
}

/// One character's pixel bitmap.
#[wasm_bindgen]
pub struct font_Glyph {
    inner: mrlyrs::font::Glyph,
}

#[wasm_bindgen]
impl font_Glyph {
    /// Reads the Glyph from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<font_Glyph, JsValue> {
        Ok(font_Glyph {
            inner: hand::from_js(&data)?,
        })
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
    #[wasm_bindgen(setter)]
    pub fn set_char(&mut self, value: char) -> Result<(), JsValue> {
        self.inner.char = value;
        Ok(())
    }
    /// The bitmap rows of '0' and '1' characters.
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
    /// Returns the number of rows.
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height();
        Ok(value)
    }
    /// Builds a glyph from its character and rows.
    #[wasm_bindgen(constructor)]
    pub fn new(char: char, rows: JsValue) -> Result<font_Glyph, JsValue> {
        let rows = hand::from_js::<Vec<String>>(&rows)?;
        let value = mrlyrs::font::Glyph::new(char, rows);
        Ok(font_Glyph { inner: value })
    }
    /// Returns the cell width of the first row, or 0 for an empty glyph.
    pub fn width(&self) -> Result<usize, JsValue> {
        let value = self.inner.width();
        Ok(value)
    }
}

/// A complete recipe for one tile.
#[wasm_bindgen]
pub struct gen_Tile {
    inner: mrlyrs::gen::Tile,
}

#[wasm_bindgen]
impl gen_Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<gen_Tile, JsValue> {
        Ok(gen_Tile {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Tile as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The construction family.
    #[wasm_bindgen(getter)]
    pub fn group(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.group;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::gen::Group>(&value)?;
        self.inner.group = value;
        Ok(())
    }
    /// The base factor of the construction.
    #[wasm_bindgen(getter)]
    pub fn factor(&self) -> Result<usize, JsValue> {
        let value = self.inner.factor;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_factor(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.factor = value;
        Ok(())
    }
    /// The origin of each layer.
    #[wasm_bindgen(getter)]
    pub fn sources(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.sources.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_sources(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrlyrs::gen::recipe::Source>>(&value)?;
        self.inner.sources = value;
        Ok(())
    }
    /// The grid size of each source.
    #[wasm_bindgen(getter)]
    pub fn numbers(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.numbers.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_numbers(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.numbers = value;
        Ok(())
    }
    /// The fractal level of each source.
    #[wasm_bindgen(getter)]
    pub fn levels(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.levels.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_levels(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.levels = value;
        Ok(())
    }
    /// The quarter-turn rotation of each source.
    #[wasm_bindgen(getter)]
    pub fn rotations(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.rotations.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_rotations(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.rotations = value;
        Ok(())
    }
    /// Whether the finished tile inverts.
    #[wasm_bindgen(getter)]
    pub fn invert(&self) -> Result<bool, JsValue> {
        let value = self.inner.invert;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_invert(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.invert = value;
        Ok(())
    }
    /// Whether the finished tile flips.
    #[wasm_bindgen(getter)]
    pub fn flip(&self) -> Result<bool, JsValue> {
        let value = self.inner.flip;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_flip(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.flip = value;
        Ok(())
    }
    /// The tile's width in cells.
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
    /// The tile's height in cells.
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
    /// Checks that the slots, numbers and sizes agree.
    pub fn check(&self) -> Result<(), JsValue> {
        self.inner.check().map_err(hand::throw)?;
        Ok(())
    }
    /// Returns whether the recipe is a magic tile of one repeated source at one repeated number,
    pub fn degenerate(&self) -> Result<bool, JsValue> {
        let value = self.inner.degenerate();
        Ok(value)
    }
    /// Returns the larger of width and height.
    pub fn max_size(&self) -> Result<usize, JsValue> {
        let value = self.inner.max_size();
        Ok(value)
    }
    /// Builds an empty tile in a group.
    #[wasm_bindgen(constructor)]
    pub fn new(group: JsValue) -> Result<gen_Tile, JsValue> {
        let group = hand::from_js::<mrlyrs::gen::Group>(&group)?;
        let value = mrlyrs::gen::Tile::new(group);
        Ok(gen_Tile { inner: value })
    }
    /// Recomputes the factor and side length the group and numbers imply, zero when they overflow.
    pub fn resize(&mut self) -> Result<(), JsValue> {
        self.inner.resize();
        Ok(())
    }
    /// Sets the tile's width and height.
    pub fn size(&self, width: usize, height: usize) -> Result<gen_Tile, JsValue> {
        let value = self.inner.clone().size(width, height);
        Ok(gen_Tile { inner: value })
    }
}

/// A tile recipe folded to its one canonical object.
#[wasm_bindgen]
pub struct gen_name_Tile {
    inner: mrlyrs::gen::name::Tile,
}

#[wasm_bindgen]
impl gen_name_Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<gen_name_Tile, JsValue> {
        Ok(gen_name_Tile {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Tile as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The one design of a flat or fractal tile.
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code;
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_code(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, hand::u128_from_js)?;
        self.inner.code = value;
        Ok(())
    }
    /// The mask code of a special tile.
    #[wasm_bindgen(getter)]
    pub fn special(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.special;
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_special(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, hand::u128_from_js)?;
        self.inner.special = value;
        Ok(())
    }
    /// The letters of a magic tile, first letter outermost.
    #[wasm_bindgen(getter)]
    pub fn magic(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.magic.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_magic(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::u128_from_js)?;
        self.inner.magic = value;
        Ok(())
    }
    /// The three codes of a mosaic tile.
    #[wasm_bindgen(getter)]
    pub fn mosaic(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mosaic.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_mosaic(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::u128_from_js)?;
        self.inner.mosaic = value;
        Ok(())
    }
    /// The side of the mask of a special or mosaic tile.
    #[wasm_bindgen(getter)]
    pub fn factor(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.factor;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_factor(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<usize>>(&value)?;
        self.inner.factor = value;
        Ok(())
    }
    /// The side each slot renders at, one per letter for a magic tile.
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.side.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_side(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::gen::name::Slots>(&value)?;
        self.inner.side = value;
        Ok(())
    }
    /// The power a fractal tile is raised to, absent at one.
    #[wasm_bindgen(getter)]
    pub fn level(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.level;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_level(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<usize>>(&value)?;
        self.inner.level = value;
        Ok(())
    }
    /// The quarter turns of each slot, absent when nothing turns.
    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.turn.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_turn(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::gen::name::Slots>(&value)?;
        self.inner.turn = value;
        Ok(())
    }
    /// Whether a special tile flips its mask.
    #[wasm_bindgen(getter)]
    pub fn flip(&self) -> Result<bool, JsValue> {
        let value = self.inner.flip;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_flip(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.flip = value;
        Ok(())
    }
    /// Whether the finished tile inverts.
    #[wasm_bindgen(getter)]
    pub fn invert(&self) -> Result<bool, JsValue> {
        let value = self.inner.invert;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_invert(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.invert = value;
        Ok(())
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<gen_name_Tile, JsValue> {
        let value =
            <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::checked(self.inner.clone())
                .map_err(hand::throw)?;
        Ok(gen_name_Tile { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<gen_name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(gen_name_Tile { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<gen_name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(gen_name_Tile { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<gen_name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(gen_name_Tile { inner: value })
    }
    /// Folds a recipe to its name.
    pub fn of(recipe: &gen_Tile) -> Result<gen_name_Tile, JsValue> {
        let value = mrlyrs::gen::name::Tile::of(&recipe.inner).map_err(hand::throw)?;
        Ok(gen_name_Tile { inner: value })
    }
    /// Builds the recipe the name folds, resized and checked.
    pub fn recipe(&self) -> Result<gen_Tile, JsValue> {
        let value = self.inner.recipe().map_err(hand::throw)?;
        Ok(gen_Tile { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_mrly(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// One rendering of an artwork, sized in tile repetitions.
#[wasm_bindgen]
pub struct gen_variation_File {
    inner: mrlyrs::gen::variation::File,
}

#[wasm_bindgen]
impl gen_variation_File {
    /// Reads the File from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<gen_variation_File, JsValue> {
        Ok(gen_variation_File {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the File as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The count of tile repetitions across.
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
    /// The count of tile repetitions down.
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
    /// The encoded PNG bytes, empty until rendered and left out of the json.
    #[wasm_bindgen(getter)]
    pub fn png(&self) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.png.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_png(&mut self, value: Vec<u8>) -> Result<(), JsValue> {
        self.inner.png = value;
        Ok(())
    }
    /// Builds a file of the given repetition counts with no PNG bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Result<gen_variation_File, JsValue> {
        let value = mrlyrs::gen::variation::File::new(width, height);
        Ok(gen_variation_File { inner: value })
    }
}

/// One seeded artwork, from tile recipe to rendered files.
#[wasm_bindgen]
pub struct gen_variation_Variation {
    inner: mrlyrs::gen::variation::Variation,
}

#[wasm_bindgen]
impl gen_variation_Variation {
    /// Reads the Variation from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<gen_variation_Variation, JsValue> {
        Ok(gen_variation_Variation {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Variation as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The random hex identifier.
    #[wasm_bindgen(getter)]
    pub fn key(&self) -> Result<String, JsValue> {
        let value = self.inner.key.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_key(&mut self, value: String) -> Result<(), JsValue> {
        self.inner.key = value;
        Ok(())
    }
    /// The seed the variation is drawn under.
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> Result<u64, JsValue> {
        let value = self.inner.seed;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_seed(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u64_from_js(&value)?;
        self.inner.seed = value;
        Ok(())
    }
    /// The paint edition.
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
    /// The primary inks, when the config fixes them.
    #[wasm_bindgen(getter)]
    pub fn primaries(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.primaries.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_primaries(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<Vec<mrlyrs::core::paint::Ink>>>(&value)?;
        self.inner.primaries = value;
        Ok(())
    }
    /// The tile recipe.
    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> Result<gen_Tile, JsValue> {
        let value = self.inner.tile.clone();
        Ok(gen_Tile { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, value: &gen_Tile) -> Result<(), JsValue> {
        self.inner.tile = value.inner.clone();
        Ok(())
    }
    /// The mask tile, present only under the Neighbors edition.
    #[wasm_bindgen(getter)]
    pub fn mask(&self) -> Result<Option<gen_Tile>, JsValue> {
        let value = self.inner.mask.clone();
        Ok(value.map(|inner| gen_Tile { inner }))
    }
    #[wasm_bindgen(setter)]
    pub fn set_mask(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, |x1| {
            hand::from_js::<mrlyrs::gen::Tile>(&hand::plain(x1)?)
        })?;
        self.inner.mask = value;
        Ok(())
    }
    /// The paint, set by generate.
    #[wasm_bindgen(getter)]
    pub fn paint(&self) -> Result<Option<core_paint_Paint>, JsValue> {
        let value = self.inner.paint.clone();
        Ok(value.map(|inner| core_paint_Paint { inner }))
    }
    #[wasm_bindgen(setter)]
    pub fn set_paint(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, |x1| {
            hand::from_js::<mrlyrs::core::paint::Paint>(&hand::plain(x1)?)
        })?;
        self.inner.paint = value;
        Ok(())
    }
    /// The built base cell, set by generate and left out of the json.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.base.clone();
        hand::option_to_js(value.as_ref(), hand::cell2d_to_js)
    }
    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, hand::cell2d_from_js)?;
        self.inner.base = value;
        Ok(())
    }
    /// The renderings, filled by render.
    #[wasm_bindgen(getter)]
    pub fn files(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.files.clone();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(gen_variation_File { inner: x1.clone() }))
        })
    }
    #[wasm_bindgen(setter)]
    pub fn set_files(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, |x1| {
            hand::from_js::<mrlyrs::gen::variation::File>(&hand::plain(x1)?)
        })?;
        self.inner.files = value;
        Ok(())
    }
    /// Returns whether the edition paints the whole tiled canvas.
    pub fn is_cover(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_cover();
        Ok(value)
    }
    /// Returns whether the edition paints the base cell before tiling.
    pub fn is_prime(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_prime();
        Ok(value)
    }
}

/// The rulebook of a life run.
#[wasm_bindgen]
pub struct life_Config {
    inner: mrlyrs::life::Config,
}

#[wasm_bindgen]
impl life_Config {
    /// Reads the Config from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Config, JsValue> {
        Ok(life_Config {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Config as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The neighborhood mask.
    #[wasm_bindgen(getter)]
    pub fn mask(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mask.clone();
        hand::cell2d_to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_mask(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::cell2d_from_js(&value)?;
        self.inner.mask = value;
        Ok(())
    }
    /// The neighbor counts that create a cell.
    #[wasm_bindgen(getter)]
    pub fn birth(&self) -> Result<life_Counts, JsValue> {
        let value = self.inner.birth.clone();
        Ok(life_Counts { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_birth(&mut self, value: &life_Counts) -> Result<(), JsValue> {
        self.inner.birth = value.inner.clone();
        Ok(())
    }
    /// The neighbor counts that keep a cell.
    #[wasm_bindgen(getter)]
    pub fn survive(&self) -> Result<life_Counts, JsValue> {
        let value = self.inner.survive.clone();
        Ok(life_Counts { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_survive(&mut self, value: &life_Counts) -> Result<(), JsValue> {
        self.inner.survive = value.inner.clone();
        Ok(())
    }
    /// The edge policy.
    #[wasm_bindgen(getter)]
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_boundary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::life::Boundary>(&value)?;
        self.inner.boundary = value;
        Ok(())
    }
    /// The generation cap.
    #[wasm_bindgen(getter)]
    pub fn max_generations(&self) -> Result<usize, JsValue> {
        let value = self.inner.max_generations;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_max_generations(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.max_generations = value;
        Ok(())
    }
    /// The tiling factor applied to the seed.
    #[wasm_bindgen(getter)]
    pub fn grid_size(&self) -> Result<usize, JsValue> {
        let value = self.inner.grid_size;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_grid_size(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.grid_size = value;
        Ok(())
    }
    /// The dead border added around the seed.
    #[wasm_bindgen(getter)]
    pub fn padding(&self) -> Result<usize, JsValue> {
        let value = self.inner.padding;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_padding(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.padding = value;
        Ok(())
    }
    /// Returns the largest neighbor count the mask can reach.
    pub fn budget(&self) -> Result<usize, JsValue> {
        let value = self.inner.budget();
        Ok(value)
    }
    /// Resolves the birth and survive counts against the mask's budget.
    pub fn counts(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.counts().map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            hand::typed(&(value.0)[..]),
            hand::typed(&(value.1)[..]),
        ]))
    }
    /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
    #[wasm_bindgen(constructor)]
    pub fn new(
        mask: JsValue,
        birth: &life_Counts,
        survive: &life_Counts,
    ) -> Result<life_Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = mrlyrs::life::Config::new(mask, birth.inner.clone(), survive.inner.clone());
        Ok(life_Config { inner: value })
    }
}

/// The neighbor counts one side of a rule fires on.
#[wasm_bindgen]
pub struct life_Counts {
    inner: mrlyrs::life::Counts,
}

#[wasm_bindgen]
impl life_Counts {
    /// Reads the Counts from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Counts, JsValue> {
        Ok(life_Counts {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Counts as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Builds the counts a sequence lays down, keeping zeros and ones on request.
    pub fn drawn(seq: &life_Source, zeros: bool, ones: bool) -> Result<life_Counts, JsValue> {
        let value = mrlyrs::life::Counts::drawn(seq.inner, zeros, ones);
        Ok(life_Counts { inner: value })
    }
    /// Spells the counts outright.
    pub fn list(counts: Vec<usize>) -> Result<life_Counts, JsValue> {
        let value = mrlyrs::life::Counts::list(counts);
        Ok(life_Counts { inner: value })
    }
    /// Returns the counts, a drawn side resolved against the mask's neighbor budget.
    pub fn values(&self, budget: usize) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.values(budget).map_err(hand::throw)?;
        Ok(value)
    }
}

/// The recorded run of one seed.
#[wasm_bindgen]
pub struct life_Life {
    inner: mrlyrs::life::Life,
}

#[wasm_bindgen]
impl life_Life {
    /// Reads the Life from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Life, JsValue> {
        Ok(life_Life {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Life as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Every generation in order.
    #[wasm_bindgen(getter)]
    pub fn grids(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.grids.clone();
        hand::list_to_js(&value, hand::cell2d_to_js)
    }
    #[wasm_bindgen(setter)]
    pub fn set_grids(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::cell2d_from_js)?;
        self.inner.grids = value;
        Ok(())
    }
    /// The run's ending.
    #[wasm_bindgen(getter)]
    pub fn fate(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.fate;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_fate(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::life::Fate>(&value)?;
        self.inner.fate = value;
        Ok(())
    }
    /// The number of recorded generations.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> Result<usize, JsValue> {
        let value = self.inner.count;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_count(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.count = value;
        Ok(())
    }
    /// The cycle length when the fate is a loop, else zero.
    #[wasm_bindgen(getter)]
    pub fn loop_length(&self) -> Result<usize, JsValue> {
        let value = self.inner.loop_length;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_loop_length(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.loop_length = value;
        Ok(())
    }
    /// Returns the final grid, or None when the run is empty.
    pub fn last(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.last();
        hand::option_to_js(value, hand::cell2d_to_js)
    }
}

/// A life rule: the birth and survival counts and whether the edge wraps.
#[wasm_bindgen]
pub struct life_Rule {
    inner: mrlyrs::life::Rule,
}

#[wasm_bindgen]
impl life_Rule {
    /// Reads the Rule from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Rule, JsValue> {
        Ok(life_Rule {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Rule as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The neighbor counts that create a cell, listed or drawn from a sequence.
    #[wasm_bindgen(getter)]
    pub fn birth(&self) -> Result<life_Counts, JsValue> {
        let value = self.inner.birth.clone();
        Ok(life_Counts { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_birth(&mut self, value: &life_Counts) -> Result<(), JsValue> {
        self.inner.birth = value.inner.clone();
        Ok(())
    }
    /// The neighbor counts that keep a cell, listed or drawn from a sequence.
    #[wasm_bindgen(getter)]
    pub fn survive(&self) -> Result<life_Counts, JsValue> {
        let value = self.inner.survive.clone();
        Ok(life_Counts { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_survive(&mut self, value: &life_Counts) -> Result<(), JsValue> {
        self.inner.survive = value.inner.clone();
        Ok(())
    }
    /// Whether the edge wraps, false unless said.
    #[wasm_bindgen(getter)]
    pub fn wrap(&self) -> Result<bool, JsValue> {
        let value = self.inner.wrap;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_wrap(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.wrap = value;
        Ok(())
    }
    /// Returns the edge policy the rule runs under.
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary();
        hand::to_js(&value)
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<life_Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::checked(self.inner.clone())
            .map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Builds a life config running this rule over a neighborhood mask.
    pub fn config(&self, mask: JsValue) -> Result<life_Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = self.inner.config(mask);
        Ok(life_Config { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<life_Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<life_Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<life_Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
    #[wasm_bindgen(constructor)]
    pub fn new(
        birth: &life_Counts,
        survive: &life_Counts,
        wrap: bool,
    ) -> Result<life_Rule, JsValue> {
        let value = mrlyrs::life::Rule::new(birth.inner.clone(), survive.inner.clone(), wrap);
        Ok(life_Rule { inner: value })
    }
    /// Reads the rule out of a life config.
    pub fn of(config: &life_Config) -> Result<life_Rule, JsValue> {
        let value = mrlyrs::life::Rule::of(&config.inner);
        Ok(life_Rule { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_mrly(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// A named source of neighbor-count values.
#[wasm_bindgen]
pub struct life_Source {
    inner: mrlyrs::life::Source,
}

#[wasm_bindgen]
impl life_Source {
    /// Reads the Source from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Source, JsValue> {
        Ok(life_Source {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Source as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns every fixed sequence, the seeded and coded families excluded.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::all();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
    pub fn designs() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::designs();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    /// Returns whether the sequence is a seeded random draw.
    pub fn is_random(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_random();
        Ok(value)
    }
    /// Returns the sequence's parseable name, the one string that regenerates it.
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name();
        Ok(value)
    }
    /// Returns the six number sequences, the random one listed under seed zero.
    pub fn numbers() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::numbers();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    /// Returns the sequence's OEIS id, or None off the encyclopedia.
    pub fn oeis(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.oeis();
        hand::to_js(&value)
    }
    /// Parses a sequence name back to its source.
    pub fn parse(name: &str) -> Result<life_Source, JsValue> {
        let value = mrlyrs::life::Source::parse(name).map_err(hand::throw)?;
        Ok(life_Source { inner: value })
    }
    /// Reads a canonical name off the front of the text, returning the tail left over.
    pub fn read(text: &str) -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::read(text);
        hand::option_to_js(value.as_ref(), |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(life_Source { inner: x1.0 }),
                hand::to_js(&x1.1)?,
            ]))
        })
    }
}

/// A single design with its place in the orbit structure.
#[wasm_bindgen]
pub struct math_bang_Design {
    inner: mrlyrs::math::bang::Design,
}

#[wasm_bindgen]
impl math_bang_Design {
    /// Reads the Design from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_bang_Design, JsValue> {
        Ok(math_bang_Design {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Design as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The design's code.
    #[wasm_bindgen(getter)]
    pub fn i(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.i;
        Ok(JsValue::from_str(&hand::code_to_js(value)))
    }
    #[wasm_bindgen(setter)]
    pub fn set_i(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::code_from_js(&value)?;
        self.inner.i = value;
        Ok(())
    }
    /// The design's dimension.
    #[wasm_bindgen(getter)]
    pub fn dimension(&self) -> Result<usize, JsValue> {
        let value = self.inner.dimension;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dimension(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dimension = value;
        Ok(())
    }
    /// Whether this code is the smallest in its orbit.
    #[wasm_bindgen(getter)]
    pub fn canonical(&self) -> Result<bool, JsValue> {
        let value = self.inner.canonical;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_canonical(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.canonical = value;
        Ok(())
    }
    /// The smallest code in the orbit.
    #[wasm_bindgen(getter)]
    pub fn class_rep(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.class_rep;
        Ok(JsValue::from_str(&hand::code_to_js(value)))
    }
    #[wasm_bindgen(setter)]
    pub fn set_class_rep(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::code_from_js(&value)?;
        self.inner.class_rep = value;
        Ok(())
    }
    /// The number of codes in the orbit.
    #[wasm_bindgen(getter)]
    pub fn orbit_size(&self) -> Result<usize, JsValue> {
        let value = self.inner.orbit_size;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_orbit_size(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.orbit_size = value;
        Ok(())
    }
    /// Returns the design's algebraic normal form as a string.
    pub fn anf(&self) -> Result<String, JsValue> {
        let value = self.inner.anf();
        Ok(value)
    }
    /// Returns the design's algebraic degree, or -1 for the zero design.
    pub fn degree(&self) -> Result<i32, JsValue> {
        let value = self.inner.degree();
        Ok(value)
    }
    /// Returns the design's name as a line of prose, `bang dim 2, code 7`.
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name().map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns the design's filled corners in sorted order.
    pub fn rule(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.rule();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
}

/// The complete enumeration of one dimension's designs and orbits.
#[wasm_bindgen]
pub struct math_bang_Universe {
    inner: mrlyrs::math::bang::Universe,
}

#[wasm_bindgen]
impl math_bang_Universe {
    /// Reads the Universe from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_bang_Universe, JsValue> {
        Ok(math_bang_Universe {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Universe as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The universe's dimension.
    #[wasm_bindgen(getter)]
    pub fn dimension(&self) -> Result<usize, JsValue> {
        let value = self.inner.dimension;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dimension(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dimension = value;
        Ok(())
    }
    /// The number of codes in the universe.
    #[wasm_bindgen(getter)]
    pub fn total(&self) -> Result<usize, JsValue> {
        let value = self.inner.total;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_total(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.total = value;
        Ok(())
    }
    /// Returns every design in code order.
    pub fn all(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.all();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_bang_Design { inner: x1.clone() }))
        })
    }
    /// Returns the designs whose codes lead their orbits.
    pub fn canonical(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.canonical();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_bang_Design { inner: x1.clone() }))
        })
    }
    /// Returns the design at a code with its precomputed orbit facts.
    pub fn design(&self, code: JsValue) -> Result<math_bang_Design, JsValue> {
        let code = hand::code_from_js(&code)?;
        let value = self.inner.design(code);
        Ok(math_bang_Design { inner: value })
    }
    /// Returns the number of distinct orbits.
    pub fn distinct(&self) -> Result<usize, JsValue> {
        let value = self.inner.distinct();
        Ok(value)
    }
    /// Enumerates every orbit of a dimension from 1 to 4.
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize) -> Result<math_bang_Universe, JsValue> {
        let value = mrlyrs::math::bang::Universe::new(dimension).map_err(hand::throw)?;
        Ok(math_bang_Universe { inner: value })
    }
}

/// The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions.
#[wasm_bindgen]
pub struct math_counts_Exposure {
    inner: mrlyrs::math::counts::Exposure,
}

#[wasm_bindgen]
impl math_counts_Exposure {
    /// Reads the Exposure from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_counts_Exposure, JsValue> {
        Ok(math_counts_Exposure {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Exposure as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The filled cells of the tile.
    #[wasm_bindgen(getter)]
    pub fn occupancy(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.occupancy;
        Ok(JsValue::from_str(&value.to_string()))
    }
    #[wasm_bindgen(setter)]
    pub fn set_occupancy(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u128_from_js(&value)?;
        self.inner.occupancy = value;
        Ok(())
    }
    /// The exposed faces of the tile.
    #[wasm_bindgen(getter)]
    pub fn exposed(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.exposed;
        Ok(JsValue::from_str(&value.to_string()))
    }
    #[wasm_bindgen(setter)]
    pub fn set_exposed(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u128_from_js(&value)?;
        self.inner.exposed = value;
        Ok(())
    }
    /// Per axis, the adjacent filled pairs and the spanning positions.
    #[wasm_bindgen(getter)]
    pub fn axes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.axes.clone();
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from_str(&x1.0.to_string()),
                JsValue::from_str(&x1.1.to_string()),
            ]))
        })
    }
    #[wasm_bindgen(setter)]
    pub fn set_axes(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, |x1| {
            Ok((
                hand::u128_from_js(&hand::item(x1, 0)?)?,
                hand::u128_from_js(&hand::item(x1, 1)?)?,
            ))
        })?;
        self.inner.axes = value;
        Ok(())
    }
    /// Returns the exposed faces of the level-fold Kronecker power, or none past a u128.
    pub fn at(&self, level: u32) -> Result<JsValue, JsValue> {
        let value = self.inner.at(level);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    /// Folds the counts from the filled residue corners at a side number, without rendering the tile.
    pub fn from_corners(
        filled: JsValue,
        number: usize,
        dimension: usize,
        base: usize,
    ) -> Result<math_counts_Exposure, JsValue> {
        let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
        let value = mrlyrs::math::counts::Exposure::from_corners(&filled, number, dimension, base);
        Ok(math_counts_Exposure { inner: value })
    }
    /// Reads the counts off a rendered tile.
    pub fn of_tile(tile: JsValue) -> Result<math_counts_Exposure, JsValue> {
        let tile = hand::tensor_from_js(&tile)?;
        let value = mrlyrs::math::counts::Exposure::of_tile(&tile);
        Ok(math_counts_Exposure { inner: value })
    }
    /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.
    pub fn recurrence(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.recurrence();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
}

/// A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle.
#[wasm_bindgen]
pub struct math_graph_Layout {
    inner: mrlyrs::math::graph::Layout,
}

#[wasm_bindgen]
impl math_graph_Layout {
    /// Returns the mean net force per node in units of `k` after the last tick.
    pub fn energy(&self) -> Result<f64, JsValue> {
        let value = self.inner.energy();
        Ok(value)
    }
    /// Starts a layout from a network's own positions and branches.
    pub fn from_network(
        network: &math_graph_Network,
        seed: JsValue,
    ) -> Result<math_graph_Layout, JsValue> {
        let seed = hand::u64_from_js(&seed)?;
        let value =
            mrlyrs::math::graph::Layout::from_network(&network.inner, seed).map_err(hand::throw)?;
        Ok(math_graph_Layout { inner: value })
    }
    /// Returns the ideal branch length `k`.
    pub fn ideal(&self) -> Result<f64, JsValue> {
        let value = self.inner.ideal();
        Ok(value)
    }
    /// Returns the mean distance a node moved in the last tick.
    pub fn moved(&self) -> Result<f64, JsValue> {
        let value = self.inner.moved();
        Ok(value)
    }
    /// Starts a layout from flat positions, `dim` floats per node, and the branch pairs.
    #[wasm_bindgen(constructor)]
    pub fn new(
        positions: &[f64],
        branches: JsValue,
        dim: usize,
        seed: JsValue,
    ) -> Result<math_graph_Layout, JsValue> {
        let branches = hand::from_js::<Vec<(usize, usize)>>(&branches)?;
        let seed = hand::u64_from_js(&seed)?;
        let value = mrlyrs::math::graph::Layout::new(positions, &branches, dim, seed)
            .map_err(hand::throw)?;
        Ok(math_graph_Layout { inner: value })
    }
    /// Returns the node count.
    pub fn nodes(&self) -> Result<usize, JsValue> {
        let value = self.inner.nodes();
        Ok(value)
    }
    /// Returns the positions, `dim` floats per node.
    pub fn positions(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.positions();
        Ok(value.to_vec())
    }
    /// Runs the ticks and returns the energy left: the mean net force per node in units of `k`.
    pub fn step(&mut self, ticks: usize) -> Result<f64, JsValue> {
        let value = self.inner.step(ticks);
        Ok(value)
    }
    /// Returns the cap on one node's move in the next tick.
    pub fn temperature(&self) -> Result<f64, JsValue> {
        let value = self.inner.temperature();
        Ok(value)
    }
    /// Returns the ticks stepped so far.
    pub fn ticks(&self) -> Result<usize, JsValue> {
        let value = self.inner.ticks();
        Ok(value)
    }
}

/// A spatial graph of nodes and branches.
#[wasm_bindgen]
pub struct math_graph_Network {
    inner: mrlyrs::math::graph::Network,
}

#[wasm_bindgen]
impl math_graph_Network {
    /// Reads the Network from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_graph_Network, JsValue> {
        Ok(math_graph_Network {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Network as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The dimension every position must match.
    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Result<usize, JsValue> {
        let value = self.inner.dim;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dim(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dim = value;
        Ok(())
    }
    /// The nodes in insertion order.
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.nodes.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_nodes(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrlyrs::math::graph::Node>>(&value)?;
        self.inner.nodes = value;
        Ok(())
    }
    /// The branches in insertion order.
    #[wasm_bindgen(getter)]
    pub fn branches(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.branches.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_branches(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrlyrs::math::graph::Branch>>(&value)?;
        self.inner.branches = value;
        Ok(())
    }
    /// Appends a branch between two node indices.
    pub fn add_branch(&mut self, parent: usize, child: usize, radius: f64) -> Result<(), JsValue> {
        self.inner
            .add_branch(parent, child, radius)
            .map_err(hand::throw)?;
        Ok(())
    }
    /// Appends a node at the position and returns its index.
    pub fn add_node(&mut self, position: Vec<f64>) -> Result<usize, JsValue> {
        let value = self.inner.add_node(position).map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns the undirected neighbor lists of every node.
    pub fn adjacency(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.adjacency().map_err(hand::throw)?;
        hand::map_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Returns each node's branch count, indexed like the node list.
    pub fn degree(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.degree().map_err(hand::throw)?;
        Ok(value)
    }
    /// Builds an empty network of the given dimension.
    #[wasm_bindgen(constructor)]
    pub fn new(dim: usize) -> Result<math_graph_Network, JsValue> {
        let value = mrlyrs::math::graph::Network::new(dim);
        Ok(math_graph_Network { inner: value })
    }
}

/// A square grid of f32 samples.
#[wasm_bindgen]
pub struct math_moire_Field {
    inner: mrlyrs::math::moire::Field,
}

#[wasm_bindgen]
impl math_moire_Field {
    /// Reads the Field from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_moire_Field, JsValue> {
        Ok(math_moire_Field {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Field as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The samples in row-major order.
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<Vec<f32>, JsValue> {
        let value = self.inner.data.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Vec<f32>) -> Result<(), JsValue> {
        self.inner.data = value;
        Ok(())
    }
    /// The side length in samples.
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Result<usize, JsValue> {
        let value = self.inner.size;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.size = value;
        Ok(())
    }
    /// Returns the samples widened to f64.
    pub fn as_f64(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.as_f64();
        Ok(value)
    }
    /// Wraps row-major samples of the given side.
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<math_moire_Field, JsValue> {
        let value = mrlyrs::math::moire::Field::from_data(data, size).map_err(hand::throw)?;
        Ok(math_moire_Field { inner: value })
    }
    /// Returns the largest sample.
    pub fn max(&self) -> Result<f32, JsValue> {
        let value = self.inner.max();
        Ok(value)
    }
    /// Returns the mean sample, or zero for an empty field.
    pub fn mean(&self) -> Result<f64, JsValue> {
        let value = self.inner.mean();
        Ok(value)
    }
    /// Returns the smallest sample.
    pub fn min(&self) -> Result<f32, JsValue> {
        let value = self.inner.min();
        Ok(value)
    }
    /// Builds a zeroed field of the given side.
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Result<math_moire_Field, JsValue> {
        let value = mrlyrs::math::moire::Field::new(size);
        Ok(math_moire_Field { inner: value })
    }
    /// Returns the samples scaled into 0..1, symmetric about zero on request.
    pub fn normalized(&self, symmetric: bool) -> Result<Vec<f32>, JsValue> {
        let value = self.inner.normalized(symmetric);
        Ok(value)
    }
}

/// One named moire recipe: the design, the scales it stacks and the lattice it samples.
#[wasm_bindgen]
pub struct math_moire_Preset {
    inner: mrlyrs::math::moire::Preset,
}

#[wasm_bindgen]
impl math_moire_Preset {
    /// Writes the Preset as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The name the recipe answers to.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name;
        Ok(value.to_string())
    }
    /// The design sampled at every scale.
    #[wasm_bindgen(getter)]
    pub fn spec(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.spec;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_spec(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::math::moire::Spec>(&value)?;
        self.inner.spec = value;
        Ok(())
    }
    /// The side numbers stacked.
    #[wasm_bindgen(getter)]
    pub fn numbers(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.numbers.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_numbers(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.numbers = value;
        Ok(())
    }
    /// The way the layers merge.
    #[wasm_bindgen(getter)]
    pub fn combine(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.combine;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_combine(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::math::moire::Combine>(&value)?;
        self.inner.combine = value;
        Ok(())
    }
    /// The fractal depth of each layer.
    #[wasm_bindgen(getter)]
    pub fn level(&self) -> Result<usize, JsValue> {
        let value = self.inner.level;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_level(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.level = value;
        Ok(())
    }
    /// The lattice the layers are sampled on.
    #[wasm_bindgen(getter)]
    pub fn lattice(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.lattice;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_lattice(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::math::moire::Lattice>(&value)?;
        self.inner.lattice = value;
        Ok(())
    }
    /// The carpet stack: every base-three corner but the centre, summed over odd scales.
    pub fn carpet(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::carpet(limit);
        Ok(math_moire_Preset { inner: value })
    }
    /// Samples the preset into a square field of the given side.
    pub fn field(&self, size: usize) -> Result<math_moire_Field, JsValue> {
        let value = self.inner.field(size).map_err(hand::throw)?;
        Ok(math_moire_Field { inner: value })
    }
    /// The parity heatmap: odd scales of the low corner summed on the square lattice.
    pub fn heatmap(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::heatmap(limit);
        Ok(math_moire_Preset { inner: value })
    }
    /// The hive: the parity heatmap sampled on the hexagonal lattice.
    pub fn hive(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::hive(limit);
        Ok(math_moire_Preset { inner: value })
    }
    /// The parity weave: the same odd scales folded to their parity instead of summed.
    pub fn weave(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::weave(limit);
        Ok(math_moire_Preset { inner: value })
    }
}

/// A cubic grid of f32 samples, x-major.
#[wasm_bindgen]
pub struct math_moire_Volume {
    inner: mrlyrs::math::moire::Volume,
}

#[wasm_bindgen]
impl math_moire_Volume {
    /// Reads the Volume from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_moire_Volume, JsValue> {
        Ok(math_moire_Volume {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Volume as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The samples, x-major, then y, then z.
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<Vec<f32>, JsValue> {
        let value = self.inner.data.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Vec<f32>) -> Result<(), JsValue> {
        self.inner.data = value;
        Ok(())
    }
    /// The side in samples.
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Result<usize, JsValue> {
        let value = self.inner.size;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.size = value;
        Ok(())
    }
    /// Reads the sample at a voxel.
    pub fn at(&self, x: usize, y: usize, z: usize) -> Result<f32, JsValue> {
        let value = self.inner.at(x, y, z);
        Ok(value)
    }
    /// Counts the samples at or above the level.
    pub fn count(&self, level: f32) -> Result<usize, JsValue> {
        let value = self.inner.count(level);
        Ok(value)
    }
    /// Wraps x-major samples of the side.
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<math_moire_Volume, JsValue> {
        let value = mrlyrs::math::moire::Volume::from_data(data, size).map_err(hand::throw)?;
        Ok(math_moire_Volume { inner: value })
    }
    /// Returns the largest sample.
    pub fn max(&self) -> Result<f32, JsValue> {
        let value = self.inner.max();
        Ok(value)
    }
    /// Returns the smallest sample.
    pub fn min(&self) -> Result<f32, JsValue> {
        let value = self.inner.min();
        Ok(value)
    }
    /// Builds a zeroed volume of the side.
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Result<math_moire_Volume, JsValue> {
        let value = mrlyrs::math::moire::Volume::new(size);
        Ok(math_moire_Volume { inner: value })
    }
    /// Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.
    pub fn plane(&self, frame: JsValue, out: usize) -> Result<JsValue, JsValue> {
        let frame = hand::from_js::<mrlyrs::math::moire::Frame>(&frame)?;
        let value = self.inner.plane(&frame, out);
        Ok(hand::tuple_to_js(&[
            hand::typed(&(value.0)[..]),
            hand::typed(&(value.1)[..]),
        ]))
    }
    /// Reads the voxel a point of the unit cube falls in, or zero outside it.
    pub fn sample(&self, p: JsValue) -> Result<JsValue, JsValue> {
        let p = hand::from_js::<[f64; 3]>(&p)?;
        let value = self.inner.sample(p);
        hand::to_js(&value)
    }
    /// Thresholds into a byte tensor: one where a sample reaches the level, zero below.
    pub fn solid(&self, level: f32) -> Result<JsValue, JsValue> {
        let value = self.inner.solid(level);
        hand::tensor_to_js(&value)
    }
}

/// A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists.
#[wasm_bindgen]
pub struct math_name_Bang {
    inner: mrlyrs::math::name::Bang,
}

#[wasm_bindgen]
impl math_name_Bang {
    /// Reads the Bang from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Bang, JsValue> {
        Ok(math_name_Bang {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Bang as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The number of axes.
    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Result<usize, JsValue> {
        let value = self.inner.dim;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dim(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dim = value;
        Ok(())
    }
    /// The lattice, square unless said.
    #[wasm_bindgen(getter)]
    pub fn lattice(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.lattice;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_lattice(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrlyrs::math::name::Lattice>(&value)?;
        self.inner.lattice = value;
        Ok(())
    }
    /// The digits per axis, 2 unless said.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<usize, JsValue> {
        let value = self.inner.base;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.base = value;
        Ok(())
    }
    /// The design as a number.
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code;
        Ok(JsValue::from_str(&value.to_string()))
    }
    #[wasm_bindgen(setter)]
    pub fn set_code(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u128_from_js(&value)?;
        self.inner.code = value;
        Ok(())
    }
    /// One unit index per filled digit, absent when nothing turns.
    #[wasm_bindgen(getter)]
    pub fn twist(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.twist.clone();
        hand::option_to_js(value.as_ref(), |x1| Ok(hand::typed(&(*x1)[..])))
    }
    #[wasm_bindgen(setter)]
    pub fn set_twist(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<Vec<usize>>>(&value)?;
        self.inner.twist = value;
        Ok(())
    }
    /// Returns the number of digits the code addresses.
    pub fn cells(&self) -> Result<u32, JsValue> {
        let value = self.inner.cells().map_err(hand::throw)?;
        Ok(value)
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<math_name_Bang, JsValue> {
        let value =
            <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::checked(self.inner.clone())
                .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    /// Pins a code to its dimension and base on the square lattice.
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue, dim: usize, base: usize) -> Result<math_name_Bang, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::name::Bang::new(code, dim, base);
        Ok(math_name_Bang { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_mrly(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// A design sequence's address: the design, the reading taken off it and the index it runs along.
#[wasm_bindgen]
pub struct math_name_Sequence {
    inner: mrlyrs::math::name::Sequence,
}

#[wasm_bindgen]
impl math_name_Sequence {
    /// Reads the Sequence from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Sequence, JsValue> {
        Ok(math_name_Sequence {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Sequence as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The number of axes.
    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Result<usize, JsValue> {
        let value = self.inner.dim;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dim(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dim = value;
        Ok(())
    }
    /// The digits per axis, 2 unless said.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<usize, JsValue> {
        let value = self.inner.base;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.base = value;
        Ok(())
    }
    /// The design as a number.
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code;
        Ok(JsValue::from_str(&value.to_string()))
    }
    #[wasm_bindgen(setter)]
    pub fn set_code(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u128_from_js(&value)?;
        self.inner.code = value;
        Ok(())
    }
    /// The reading taken.
    #[wasm_bindgen(getter)]
    pub fn measure(&self) -> Result<String, JsValue> {
        let value = self.inner.measure.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_measure(&mut self, value: String) -> Result<(), JsValue> {
        self.inner.measure = value;
        Ok(())
    }
    /// The index the reading runs along.
    #[wasm_bindgen(getter)]
    pub fn axis(&self) -> Result<String, JsValue> {
        let value = self.inner.axis.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_axis(&mut self, value: String) -> Result<(), JsValue> {
        self.inner.axis = value;
        Ok(())
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<math_name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::checked(
            self.inner.clone(),
        )
        .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    /// Returns the design pinned to its dimension and base.
    pub fn design(&self) -> Result<math_name_Bang, JsValue> {
        let value = self.inner.design();
        Ok(math_name_Bang { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    /// Pins a design's reading to its measure and axis.
    #[wasm_bindgen(constructor)]
    pub fn new(
        code: JsValue,
        dim: usize,
        base: usize,
        measure: &str,
        axis: &str,
    ) -> Result<math_name_Sequence, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::name::Sequence::new(code, dim, base, measure, axis);
        Ok(math_name_Sequence { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value =
            <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_file(&self.inner)
                .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value =
            <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value =
            <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_mrly(&self.inner)
                .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value =
            <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_url(&self.inner)
                .map_err(hand::throw)?;
        Ok(value)
    }
}

/// A magic word: an ordered list of design letters, first letter outermost, each at its own side.
#[wasm_bindgen]
pub struct math_name_Word {
    inner: mrlyrs::math::name::Word,
}

#[wasm_bindgen]
impl math_name_Word {
    /// Reads the Word from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Word, JsValue> {
        Ok(math_name_Word {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Word as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The number of axes every letter shares.
    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Result<usize, JsValue> {
        let value = self.inner.dim;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dim(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dim = value;
        Ok(())
    }
    /// The codes of the letters in order.
    #[wasm_bindgen(getter)]
    pub fn magic(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.magic.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_magic(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::u128_from_js)?;
        self.inner.magic = value;
        Ok(())
    }
    /// The side each letter renders at.
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.side.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_side(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.side = value;
        Ok(())
    }
    /// The base of each letter, absent when every letter is base 2.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.base.clone();
        hand::option_to_js(value.as_ref(), |x1| Ok(hand::typed(&(*x1)[..])))
    }
    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<Vec<usize>>>(&value)?;
        self.inner.base = value;
        Ok(())
    }
    /// Returns the base of every letter, 2 where the name says nothing.
    pub fn bases(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.bases();
        Ok(value)
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<math_name_Word, JsValue> {
        let value =
            <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::checked(self.inner.clone())
                .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    /// Returns every letter as a design pinned to the word's dimension and its own base.
    pub fn letters(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.letters();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_name_Bang { inner: x1.clone() }))
        })
    }
    /// Pins an ordered letter list at base 2.
    #[wasm_bindgen(constructor)]
    pub fn new(dim: usize, letters: JsValue) -> Result<math_name_Word, JsValue> {
        let letters = hand::list_from_js(&letters, |x1| {
            Ok((
                hand::u128_from_js(&hand::item(x1, 0)?)?,
                hand::from_js::<usize>(&hand::item(x1, 1)?)?,
            ))
        })?;
        let value = mrlyrs::math::name::Word::new(dim, &letters).map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_mrly(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// The tally press: one pass over the integers weighs every design of a universe at once.
#[wasm_bindgen]
pub struct math_press_Press {
    inner: mrlyrs::math::press::Press,
}

#[wasm_bindgen]
impl math_press_Press {
    /// Reads the Press from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_press_Press, JsValue> {
        Ok(math_press_Press {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Press as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The design dimension of the universe.
    #[wasm_bindgen(getter)]
    pub fn dimension(&self) -> Result<usize, JsValue> {
        let value = self.inner.dimension;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dimension(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dimension = value;
        Ok(())
    }
    /// The numeral base of the universe.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<usize, JsValue> {
        let value = self.inner.base;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_base(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.base = value;
        Ok(())
    }
    /// Adds a weighted number to its usage bucket.
    pub fn add(&mut self, number: JsValue, weight: JsValue) -> Result<(), JsValue> {
        let number = hand::u128_from_js(&number)?;
        let weight = hand::i128_from_js(&weight)?;
        self.inner.add(number, weight);
        Ok(())
    }
    /// Builds an empty press over every design of the dimension and base.
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize, base: usize) -> Result<math_press_Press, JsValue> {
        let value = mrlyrs::math::press::Press::new(dimension, base).map_err(hand::throw)?;
        Ok(math_press_Press { inner: value })
    }
    /// Returns the total weight the design at a code has collected.
    pub fn total(&self, code: JsValue) -> Result<JsValue, JsValue> {
        let code = hand::code_from_js(&code)?;
        let value = self.inner.total(code);
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Returns every design's total in code order by one subset-sum transform.
    pub fn totals(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.totals();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
}

/// Every crossing of a traced roulette: the curves against themselves, the curves against one another, and how crowded the worst node is.
#[wasm_bindgen]
pub struct math_roulette_Nodes {
    inner: mrlyrs::math::roulette::Nodes,
}

#[wasm_bindgen]
impl math_roulette_Nodes {
    /// Reads the Nodes from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_roulette_Nodes, JsValue> {
        Ok(math_roulette_Nodes {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Nodes as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The curves counted, in the order the pencils came in.
    #[wasm_bindgen(getter)]
    pub fn curves(&self) -> Result<usize, JsValue> {
        let value = self.inner.curves;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_curves(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.curves = value;
        Ok(())
    }
    /// How often each curve crosses itself, curve by curve.
    #[wasm_bindgen(getter)]
    pub fn selves(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.selves.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_selves(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.selves = value;
        Ok(())
    }
    /// How often each pair of curves crosses, the lower curve first, in lexicographic order.
    #[wasm_bindgen(getter)]
    pub fn pairs(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.pairs.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_pairs(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.pairs = value;
        Ok(())
    }
    /// The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet.
    #[wasm_bindgen(getter)]
    pub fn most(&self) -> Result<usize, JsValue> {
        let value = self.inner.most;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_most(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.most = value;
        Ok(())
    }
    /// The nodes more than one crossing clusters at.
    #[wasm_bindgen(getter)]
    pub fn crowded(&self) -> Result<usize, JsValue> {
        let value = self.inner.crowded;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_crowded(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.crowded = value;
        Ok(())
    }
    /// The distinct points the crossings sit at, one for every cluster.
    #[wasm_bindgen(getter)]
    pub fn points(&self) -> Result<usize, JsValue> {
        let value = self.inner.points;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_points(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.points = value;
        Ok(())
    }
    /// The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded.
    #[wasm_bindgen(getter)]
    pub fn branches(&self) -> Result<usize, JsValue> {
        let value = self.inner.branches;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_branches(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.branches = value;
        Ok(())
    }
    /// The segment pairs that meet without crossing: collinear or end to end.
    #[wasm_bindgen(getter)]
    pub fn touches(&self) -> Result<usize, JsValue> {
        let value = self.inner.touches;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_touches(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.touches = value;
        Ok(())
    }
    /// Returns the default Nodes.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<math_roulette_Nodes, JsValue> {
        let value = mrlyrs::math::roulette::Nodes::default();
        Ok(math_roulette_Nodes { inner: value })
    }
    /// How often the curves `i` and `j` cross, either order, and zero when they are one curve.
    pub fn pair(&self, i: usize, j: usize) -> Result<usize, JsValue> {
        let value = self.inner.pair(i, j);
        Ok(value)
    }
    /// Every crossing of two curves.
    pub fn paired(&self) -> Result<usize, JsValue> {
        let value = self.inner.paired();
        Ok(value)
    }
    /// Every self crossing.
    pub fn selved(&self) -> Result<usize, JsValue> {
        let value = self.inner.selved();
        Ok(value)
    }
    /// Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero.
    pub fn total(&self) -> Result<usize, JsValue> {
        let value = self.inner.total();
        Ok(value)
    }
}

/// An exact rational number with a positive, reduced denominator.
#[wasm_bindgen]
pub struct math_shape_Frac {
    inner: mrlyrs::math::shape::Frac,
}

#[wasm_bindgen]
impl math_shape_Frac {
    /// Reads the Frac from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_shape_Frac, JsValue> {
        Ok(math_shape_Frac {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Frac as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The numerator, carrying the sign.
    #[wasm_bindgen(getter)]
    pub fn num(&self) -> Result<i64, JsValue> {
        let value = self.inner.num;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_num(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.num = value;
        Ok(())
    }
    /// The denominator, always positive.
    #[wasm_bindgen(getter)]
    pub fn den(&self) -> Result<i64, JsValue> {
        let value = self.inner.den;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_den(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.den = value;
        Ok(())
    }
    /// Returns the exact difference.
    pub fn minus(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.minus(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    /// Builds the reduced fraction num over den.
    #[wasm_bindgen(constructor)]
    pub fn new(num: JsValue, den: JsValue) -> Result<math_shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let den = hand::i64_from_js(&den)?;
        let value = mrlyrs::math::shape::Frac::new(num, den).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    /// Returns the exact sum.
    pub fn plus(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.plus(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    /// Returns the exact product.
    pub fn times(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.times(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    /// Wraps an integer as a fraction over one.
    pub fn whole(num: JsValue) -> Result<math_shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let value = mrlyrs::math::shape::Frac::whole(num);
        Ok(math_shape_Frac { inner: value })
    }
}

/// The exact reading of a cut layer: how many cells were inked out of how many were read.
#[wasm_bindgen]
pub struct math_six_star_Share {
    inner: mrlyrs::math::six::star::Share,
}

#[wasm_bindgen]
impl math_six_star_Share {
    /// Reads the Share from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_six_star_Share, JsValue> {
        Ok(math_six_star_Share {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Share as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The count of inked cells.
    #[wasm_bindgen(getter)]
    pub fn inked(&self) -> Result<i64, JsValue> {
        let value = self.inner.inked;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_inked(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.inked = value;
        Ok(())
    }
    /// The count of cells read.
    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> Result<i64, JsValue> {
        let value = self.inner.cells;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_cells(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.cells = value;
        Ok(())
    }
    /// The share in lowest terms, numerator then denominator.
    pub fn reduced(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.reduced();
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// The share as a real number.
    pub fn value(&self) -> Result<f64, JsValue> {
        let value = self.inner.value();
        Ok(value)
    }
}

/// The ghost star of a coded cube's hexagonal cut stack, read in the cell frame.
#[wasm_bindgen]
pub struct math_six_star_Star {
    inner: mrlyrs::math::six::star::Star,
}

#[wasm_bindgen]
impl math_six_star_Star {
    /// Reads the Star from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_six_star_Star, JsValue> {
        Ok(math_six_star_Star {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Star as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.
    pub fn arm(&self, number: usize, half: usize) -> Result<math_six_star_Share, JsValue> {
        let value = self.inner.arm(number, half).map_err(hand::throw)?;
        Ok(math_six_star_Share { inner: value })
    }
    /// The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.
    pub fn cell(&self, number: usize, x: JsValue, z: JsValue) -> Result<JsValue, JsValue> {
        let x = hand::i64_from_js(&x)?;
        let z = hand::i64_from_js(&z)?;
        let value = self.inner.cell(number, x, z);
        hand::to_js(&value)
    }
    /// The per-layer excess of the star band over the hexagon across the first `L` odd layers.
    pub fn excesses(&self, layers: usize, half: usize) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.excesses(layers, half).map_err(hand::throw)?;
        Ok(value)
    }
    /// The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.
    pub fn hexagon(&self, number: usize) -> Result<math_six_star_Share, JsValue> {
        let value = self.inner.hexagon(number).map_err(hand::throw)?;
        Ok(math_six_star_Share { inner: value })
    }
    /// Reads the star of a base-2 space code, the carpet being `23`.
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue) -> Result<math_six_star_Star, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::six::star::Star::new(code).map_err(hand::throw)?;
        Ok(math_six_star_Star { inner: value })
    }
}

/// A three-component vector of f32.
#[wasm_bindgen]
pub struct math_three_Vec3 {
    inner: mrlyrs::math::three::Vec3,
}

#[wasm_bindgen]
impl math_three_Vec3 {
    /// Reads the Vec3 from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_three_Vec3, JsValue> {
        Ok(math_three_Vec3 {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Vec3 as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The x component.
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> Result<f32, JsValue> {
        let value = self.inner.x;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, value: f32) -> Result<(), JsValue> {
        self.inner.x = value;
        Ok(())
    }
    /// The y component.
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Result<f32, JsValue> {
        let value = self.inner.y;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, value: f32) -> Result<(), JsValue> {
        self.inner.y = value;
        Ok(())
    }
    /// The z component.
    #[wasm_bindgen(getter)]
    pub fn z(&self) -> Result<f32, JsValue> {
        let value = self.inner.z;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_z(&mut self, value: f32) -> Result<(), JsValue> {
        self.inner.z = value;
        Ok(())
    }
    /// Returns the cross product, perpendicular to both vectors.
    pub fn cross(&self, o: &math_three_Vec3) -> Result<math_three_Vec3, JsValue> {
        let value = self.inner.cross(o.inner);
        Ok(math_three_Vec3 { inner: value })
    }
    /// Returns the dot product of the two vectors.
    pub fn dot(&self, o: &math_three_Vec3) -> Result<f32, JsValue> {
        let value = self.inner.dot(o.inner);
        Ok(value)
    }
    /// Builds a vector from its components.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Result<math_three_Vec3, JsValue> {
        let value = mrlyrs::math::three::Vec3::new(x, y, z);
        Ok(math_three_Vec3 { inner: value })
    }
    /// Multiplies every component by the scalar.
    pub fn scale(&self, s: f32) -> Result<math_three_Vec3, JsValue> {
        let value = self.inner.scale(s);
        Ok(math_three_Vec3 { inner: value })
    }
}

/// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
#[wasm_bindgen]
pub struct num_apollonian_Circle {
    inner: mrlyrs::num::apollonian::Circle,
}

#[wasm_bindgen]
impl num_apollonian_Circle {
    /// Reads the Circle from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_apollonian_Circle, JsValue> {
        Ok(num_apollonian_Circle {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Circle as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The curvature.
    #[wasm_bindgen(getter)]
    pub fn k(&self) -> Result<i64, JsValue> {
        let value = self.inner.k;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_k(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.k = value;
        Ok(())
    }
    /// The curvature times the centre's abscissa.
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> Result<i64, JsValue> {
        let value = self.inner.x;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.x = value;
        Ok(())
    }
    /// The curvature times the centre's ordinate.
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Result<i64, JsValue> {
        let value = self.inner.y;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i64_from_js(&value)?;
        self.inner.y = value;
        Ok(())
    }
    /// The centre, none on a line.
    pub fn centre(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.centre();
        hand::to_js(&value)
    }
    /// Whether the circle is a line.
    pub fn is_line(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_line();
        Ok(value)
    }
    /// The radius, none on a line.
    pub fn radius(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.radius();
        hand::to_js(&value)
    }
}

/// A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from.
#[wasm_bindgen]
pub struct num_automaton_Automaton {
    inner: mrlyrs::num::automaton::Automaton,
}

#[wasm_bindgen]
impl num_automaton_Automaton {
    /// Reads the Automaton from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_automaton_Automaton, JsValue> {
        Ok(num_automaton_Automaton {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Automaton as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`].
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    /// Returns the base `q = 2^D`.
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    /// Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.
    pub fn cofactor(&self, s: &num_zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self
            .inner
            .cofactor(s.inner, tolerance)
            .map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(num_zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
    /// Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.
    pub fn denominator(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.denominator();
        Ok(value)
    }
    /// Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`.
    pub fn matrix(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.matrix();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Builds the ladder of a rule, choosing the peel depth.
    #[wasm_bindgen(constructor)]
    pub fn new(rule: &num_memory_Rule) -> Result<num_automaton_Automaton, JsValue> {
        let value = mrlyrs::num::automaton::Automaton::new(&rule.inner).map_err(hand::throw)?;
        Ok(num_automaton_Automaton { inner: value })
    }
    /// Returns the peel depth `P`.
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    /// Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.
    pub fn perron(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.perron();
        hand::to_js(&value)
    }
    /// Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.
    pub fn residue(&self, w0: &num_zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self
            .inner
            .residue(w0.inner, tolerance)
            .map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(num_zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
    /// Returns the rule.
    pub fn rule(&self) -> Result<num_memory_Rule, JsValue> {
        let value = self.inner.rule();
        Ok(num_memory_Rule { inner: value })
    }
    /// Returns the state count `q^(k-1)`.
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    /// Builds the ladder at an explicit peel depth, at least the rule width and at least two.
    pub fn with_peel(
        rule: &num_memory_Rule,
        peel: usize,
    ) -> Result<num_automaton_Automaton, JsValue> {
        let value =
            mrlyrs::num::automaton::Automaton::with_peel(&rule.inner, peel).map_err(hand::throw)?;
        Ok(num_automaton_Automaton { inner: value })
    }
    /// Returns `zeta_W(s)` and the bound it is known to.
    pub fn zeta(&self, s: &num_zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.zeta(s.inner, tolerance).map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(num_zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
}

/// The symmetric window of one ring: every point within a reach, with the norms sieved once.
#[wasm_bindgen]
pub struct num_gauss_Window {
    inner: mrlyrs::num::gauss::Window,
}

#[wasm_bindgen]
impl num_gauss_Window {
    /// Reads the Window from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_gauss_Window, JsValue> {
        Ok(num_gauss_Window {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Window as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Counts every class inside.
    pub fn census(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.census();
        hand::to_js(&value)
    }
    /// Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.
    pub fn class(&self, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = self.inner.class(a, b);
        hand::to_js(&value)
    }
    /// Returns whether a point lies inside.
    pub fn holds(&self, a: JsValue, b: JsValue) -> Result<bool, JsValue> {
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = self.inner.holds(a, b);
        Ok(value)
    }
    /// Opens the window of a ring out to a reach, sieving every norm inside it.
    #[wasm_bindgen(constructor)]
    pub fn new(ring: JsValue, radius: JsValue) -> Result<num_gauss_Window, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = mrlyrs::num::gauss::Window::new(ring, radius);
        Ok(num_gauss_Window { inner: value })
    }
    /// Lists every point inside, row by row from the bottom left of the bounding square.
    pub fn points(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.points();
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the reach.
    pub fn radius(&self) -> Result<u64, JsValue> {
        let value = self.inner.radius();
        Ok(value)
    }
    /// Returns the ring.
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
}

/// A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at.
#[wasm_bindgen]
pub struct num_ladder_Design {
    inner: mrlyrs::num::ladder::Design,
}

#[wasm_bindgen]
impl num_ladder_Design {
    /// Reads the Design from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_ladder_Design, JsValue> {
        Ok(num_ladder_Design {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Design as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the abscissa `alpha = log_q k`.
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    /// Returns the base.
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    /// Returns the digit set, ascending.
    pub fn digits(&self) -> Result<Vec<u64>, JsValue> {
        let value = self.inner.digits();
        Ok(value.to_vec())
    }
    /// Builds a design on the base and the digit set, choosing the peel depth.
    #[wasm_bindgen(constructor)]
    pub fn new(base: JsValue, digits: JsValue) -> Result<num_ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value = mrlyrs::num::ladder::Design::new(base, &digits).map_err(hand::throw)?;
        Ok(num_ladder_Design { inner: value })
    }
    /// Returns the peel depth.
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    /// Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.
    pub fn pole(&self, m: usize, j: JsValue) -> Result<num_zeta_Complex, JsValue> {
        let j = hand::i64_from_js(&j)?;
        let value = self.inner.pole(m, j);
        Ok(num_zeta_Complex { inner: value })
    }
    /// Builds a design at an explicit peel depth, at least two.
    pub fn with_peel(
        base: JsValue,
        digits: JsValue,
        peel: usize,
    ) -> Result<num_ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value =
            mrlyrs::num::ladder::Design::with_peel(base, &digits, peel).map_err(hand::throw)?;
        Ok(num_ladder_Design { inner: value })
    }
}

/// A rule on `k` consecutive digits of a design word.
#[wasm_bindgen]
pub struct num_memory_Rule {
    inner: mrlyrs::num::memory::Rule,
}

#[wasm_bindgen]
impl num_memory_Rule {
    /// Reads the Rule from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_memory_Rule, JsValue> {
        Ok(num_memory_Rule {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Rule as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The dimension `D`, one to three.
    #[wasm_bindgen(getter)]
    pub fn dimension(&self) -> Result<usize, JsValue> {
        let value = self.inner.dimension;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_dimension(&mut self, value: usize) -> Result<(), JsValue> {
        self.inner.dimension = value;
        Ok(())
    }
    /// The window width `k`, at least one.
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
    /// The window code, bit `w` set when window `w` is allowed.
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> Result<u64, JsValue> {
        let value = self.inner.code;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_code(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::u64_from_js(&value)?;
        self.inner.code = value;
        Ok(())
    }
    /// Returns whether a word, coarsest digit first, is accepted.
    pub fn accepts(&self, word: &[usize]) -> Result<bool, JsValue> {
        let value = self.inner.accepts(word);
        Ok(value)
    }
    /// Returns whether the window is allowed, and false for any window out of range.
    pub fn allowed(&self, window: usize) -> Result<bool, JsValue> {
        let value = self.inner.allowed(window);
        Ok(value)
    }
    /// Returns the letters that stand in at least one allowed window.
    pub fn alphabet(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.alphabet();
        Ok(value)
    }
    /// Returns the count of rules of this shape, `2^(2^(k D))`.
    pub fn codes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.codes();
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Returns the rule that allows every window.
    pub fn full(dimension: usize, width: usize) -> Result<num_memory_Rule, JsValue> {
        let value = mrlyrs::num::memory::Rule::full(dimension, width).map_err(hand::throw)?;
        Ok(num_memory_Rule { inner: value })
    }
    /// Returns the letter count `2^D`, the digit vectors of the cube's corners.
    pub fn letters(&self) -> Result<usize, JsValue> {
        let value = self.inner.letters();
        Ok(value)
    }
    /// Builds a rule from its dimension, its width and its code.
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize, width: usize, code: JsValue) -> Result<num_memory_Rule, JsValue> {
        let code = hand::u64_from_js(&code)?;
        let value = mrlyrs::num::memory::Rule::new(dimension, width, code).map_err(hand::throw)?;
        Ok(num_memory_Rule { inner: value })
    }
    /// Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    /// Returns the window count `2^(k D)`.
    pub fn windows(&self) -> Result<usize, JsValue> {
        let value = self.inner.windows();
        Ok(value)
    }
}

/// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
#[wasm_bindgen]
pub struct num_prime_Sieve {
    inner: mrlyrs::num::prime::Sieve,
}

#[wasm_bindgen]
impl num_prime_Sieve {
    /// Reads the Sieve from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_prime_Sieve, JsValue> {
        Ok(num_prime_Sieve {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Sieve as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the count of numbers marked prime so far.
    pub fn count(&self) -> Result<usize, JsValue> {
        let value = self.inner.count();
        Ok(value)
    }
    /// Returns whether every number is settled.
    pub fn done(&self) -> Result<bool, JsValue> {
        let value = self.inner.done();
        Ok(value)
    }
    /// Runs the sieve to the end.
    pub fn finish(&mut self) -> Result<(), JsValue> {
        self.inner.finish();
        Ok(())
    }
    /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
    #[wasm_bindgen(constructor)]
    pub fn new(limit: usize) -> Result<num_prime_Sieve, JsValue> {
        let value = mrlyrs::num::prime::Sieve::new(limit);
        Ok(num_prime_Sieve { inner: value })
    }
    /// Returns the count of primes used so far.
    pub fn rank(&self) -> Result<usize, JsValue> {
        let value = self.inner.rank();
        Ok(value)
    }
    /// Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.
    pub fn step(&mut self) -> Result<usize, JsValue> {
        let value = self.inner.step();
        Ok(value)
    }
    /// Returns the count of numbers the last step struck.
    pub fn struck(&self) -> Result<usize, JsValue> {
        let value = self.inner.struck();
        Ok(value)
    }
    /// Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.
    pub fn types(&self) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.types();
        Ok(value.to_vec())
    }
}

/// The base of a radix design: a ring and an element of norm at least two, the scale every word is read against.
#[wasm_bindgen]
pub struct num_radix_Base {
    inner: mrlyrs::num::radix::Base,
}

#[wasm_bindgen]
impl num_radix_Base {
    /// Reads the Base from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_radix_Base, JsValue> {
        Ok(num_radix_Base {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Base as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the index in the canonical residue system of the class of a point.
    pub fn class(&self, z: JsValue) -> Result<usize, JsValue> {
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let value = self.inner.class(z).map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns whether two points are congruent modulo the base.
    pub fn congruent(&self, z: JsValue, w: JsValue) -> Result<bool, JsValue> {
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let w = (
            hand::i64_from_js(&hand::item(&w, 0)?)?,
            hand::i64_from_js(&hand::item(&w, 1)?)?,
        );
        let value = self.inner.congruent(z, w);
        Ok(value)
    }
    /// Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.
    pub fn group(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.group().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.
    pub fn mirrored(&self) -> Result<bool, JsValue> {
        let value = self.inner.mirrored();
        Ok(value)
    }
    /// Fixes a base in a ring.
    #[wasm_bindgen(constructor)]
    pub fn new(ring: JsValue, value: JsValue) -> Result<num_radix_Base, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = (
            hand::i64_from_js(&hand::item(&value, 0)?)?,
            hand::i64_from_js(&hand::item(&value, 1)?)?,
        );
        let value = mrlyrs::num::radix::Base::new(ring, value).map_err(hand::throw)?;
        Ok(num_radix_Base { inner: value })
    }
    /// Returns the norm `q` of the base: the count of residue classes and the square of the scale.
    pub fn norm(&self) -> Result<u64, JsValue> {
        let value = self.inner.norm();
        Ok(value)
    }
    /// Returns the base raised to a level.
    pub fn power(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.power(level);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.
    pub fn residues(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.residues().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the ring.
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    /// Returns the base element.
    pub fn value(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.value();
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
}

/// A radix design: a digit set inside one ring, placed by a base with a unit twist per digit.
#[wasm_bindgen]
pub struct num_radix_Radix {
    inner: mrlyrs::num::radix::Radix,
}

#[wasm_bindgen]
impl num_radix_Radix {
    /// Reads the Radix from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_radix_Radix, JsValue> {
        Ok(num_radix_Radix {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Radix as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the base.
    pub fn base(&self) -> Result<num_radix_Base, JsValue> {
        let value = self.inner.base();
        Ok(num_radix_Base { inner: value })
    }
    /// Returns whether every digit is the canonical representative of its class.
    pub fn canonical(&self) -> Result<bool, JsValue> {
        let value = self.inner.canonical().map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code().map_err(hand::throw)?;
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Returns the digits.
    pub fn digits(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.digits();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.
    pub fn dimension(&self) -> Result<f64, JsValue> {
        let value = self.inner.dimension();
        Ok(value)
    }
    /// Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.
    pub fn distinct(&self, level: usize) -> Result<usize, JsValue> {
        let value = self.inner.distinct(level);
        Ok(value)
    }
    /// Returns the count of words of a level, `|F|^L`.
    pub fn fill(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.fill(level);
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.
    pub fn from_code(base: &num_radix_Base, code: JsValue) -> Result<num_radix_Radix, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::num::radix::Radix::from_code(base.inner, code).map_err(hand::throw)?;
        Ok(num_radix_Radix { inner: value })
    }
    /// Builds a design from a base, a digit list and a unit twist per digit.
    #[wasm_bindgen(constructor)]
    pub fn new(
        base: &num_radix_Base,
        digits: JsValue,
        twists: JsValue,
    ) -> Result<num_radix_Radix, JsValue> {
        let digits = hand::list_from_js(&digits, |x1| {
            Ok((
                hand::i64_from_js(&hand::item(x1, 0)?)?,
                hand::i64_from_js(&hand::item(x1, 1)?)?,
            ))
        })?;
        let twists = hand::list_from_js(&twists, |x1| {
            Ok((
                hand::i64_from_js(&hand::item(x1, 0)?)?,
                hand::i64_from_js(&hand::item(x1, 1)?)?,
            ))
        })?;
        let value =
            mrlyrs::num::radix::Radix::new(base.inner, digits, twists).map_err(hand::throw)?;
        Ok(num_radix_Radix { inner: value })
    }
    /// Returns the level-`L` points in the plane, the scaled words divided by `b^L`.
    pub fn plane(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.plane(level);
        hand::to_js(&value)
    }
    /// Returns the ring.
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    /// Returns the digit count `|F|`.
    pub fn size(&self) -> Result<usize, JsValue> {
        let value = self.inner.size();
        Ok(value)
    }
    /// Returns the twists.
    pub fn twists(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.twists();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the design with the twists named by their index in the unit list, the units in turning order from one.
    pub fn with_twists(&self, units: &[usize]) -> Result<num_radix_Radix, JsValue> {
        let value = self.inner.clone().with_twists(units).map_err(hand::throw)?;
        Ok(num_radix_Radix { inner: value })
    }
    /// Returns the level-`L` points in exact ring coordinates scaled by `b^L`.
    pub fn words(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.words(level);
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
}

/// A complex number: a real and an imaginary part.
#[wasm_bindgen]
pub struct num_zeta_Complex {
    inner: mrlyrs::num::zeta::Complex,
}

#[wasm_bindgen]
impl num_zeta_Complex {
    /// Reads the Complex from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_zeta_Complex, JsValue> {
        Ok(num_zeta_Complex {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Complex as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The real part.
    #[wasm_bindgen(getter)]
    pub fn re(&self) -> Result<f64, JsValue> {
        let value = self.inner.re;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_re(&mut self, value: f64) -> Result<(), JsValue> {
        self.inner.re = value;
        Ok(())
    }
    /// The imaginary part.
    #[wasm_bindgen(getter)]
    pub fn im(&self) -> Result<f64, JsValue> {
        let value = self.inner.im;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_im(&mut self, value: f64) -> Result<(), JsValue> {
        self.inner.im = value;
        Ok(())
    }
    /// Returns the modulus.
    pub fn abs(&self) -> Result<f64, JsValue> {
        let value = self.inner.abs();
        Ok(value)
    }
    /// Returns the principal argument.
    pub fn arg(&self) -> Result<f64, JsValue> {
        let value = self.inner.arg();
        Ok(value)
    }
    /// Returns the default Complex.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<num_zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::default();
        Ok(num_zeta_Complex { inner: value })
    }
    /// Returns the exponential.
    pub fn exp(&self) -> Result<num_zeta_Complex, JsValue> {
        let value = self.inner.exp();
        Ok(num_zeta_Complex { inner: value })
    }
    /// Returns the principal logarithm.
    pub fn ln(&self) -> Result<num_zeta_Complex, JsValue> {
        let value = self.inner.ln();
        Ok(num_zeta_Complex { inner: value })
    }
    /// Builds a complex number from its parts.
    #[wasm_bindgen(constructor)]
    pub fn new(re: f64, im: f64) -> Result<num_zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::new(re, im);
        Ok(num_zeta_Complex { inner: value })
    }
    /// Returns a unit complex number at the given angle.
    pub fn turn(angle: f64) -> Result<num_zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::turn(angle);
        Ok(num_zeta_Complex { inner: value })
    }
}

/// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
#[wasm_bindgen]
pub struct num_zeta_Line {
    inner: mrlyrs::num::zeta::Line,
}

#[wasm_bindgen]
impl num_zeta_Line {
    /// Reads the Line from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_zeta_Line, JsValue> {
        Ok(num_zeta_Line {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Line as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Counts the zeros on the line below t.
    pub fn count(&self, t: f64) -> Result<usize, JsValue> {
        let value = self.inner.count(t);
        Ok(value)
    }
    /// Returns the default Line.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<num_zeta_Line, JsValue> {
        let value = mrlyrs::num::zeta::Line::default();
        Ok(num_zeta_Line { inner: value })
    }
    /// Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.
    pub fn exact(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.exact(t);
        Ok(value)
    }
    /// Returns the n-th Gram point, where theta is n pi, by Newton from the right.
    pub fn gram(&self, n: JsValue) -> Result<f64, JsValue> {
        let n = hand::i64_from_js(&n)?;
        let value = self.inner.gram(n);
        Ok(value)
    }
    /// Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.
    pub fn maclaurin(&self, t: f64) -> Result<num_zeta_Complex, JsValue> {
        let value = self.inner.maclaurin(t);
        Ok(num_zeta_Complex { inner: value })
    }
    /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<num_zeta_Line, JsValue> {
        let value = mrlyrs::num::zeta::Line::new();
        Ok(num_zeta_Line { inner: value })
    }
    /// Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.
    pub fn novelty_coefficients(&self, gammas: &[f64]) -> Result<JsValue, JsValue> {
        let value = self.inner.novelty_coefficients(gammas);
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(num_zeta_Complex { inner: *x1 }))
        })
    }
    /// Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.
    pub fn pair(&self, s: &num_zeta_Complex) -> Result<JsValue, JsValue> {
        let value = self.inner.pair(s.inner);
        Ok(hand::tuple_to_js(&[
            JsValue::from(num_zeta_Complex { inner: value.0 }),
            JsValue::from(num_zeta_Complex { inner: value.1 }),
        ]))
    }
    /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
    pub fn point(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.point(t);
        Ok(hand::tuple_to_js(&[
            JsValue::from(num_zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
    /// Returns the largest gap between the two engines over the t range on a grid.
    pub fn seam(&self, t0: f64, t1: f64, steps: usize) -> Result<f64, JsValue> {
        let value = self.inner.seam(t0, t1, steps);
        Ok(value)
    }
    /// Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.
    pub fn siegel(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.siegel(t);
        Ok(value)
    }
    /// Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.
    pub fn theta(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.theta(t);
        Ok(value)
    }
    /// Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.
    pub fn z(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.z(t);
        Ok(value)
    }
    /// Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.
    pub fn zeros(&self, count: usize) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.zeros(count);
        Ok(value)
    }
}

/// A rule that turns counter values into colors.
#[wasm_bindgen]
pub struct core_Colorizer {}

#[wasm_bindgen]
impl core_Colorizer {
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
    pub fn gradient_bins(
        background: JsValue,
        colors: JsValue,
        shades: usize,
    ) -> Result<JsValue, JsValue> {
        let background = hand::color_from_js(&background)?;
        let colors = hand::list_from_js(&colors, hand::color_from_js)?;
        let value = mrlyrs::core::Colorizer::gradient_bins(background, &colors, shades)
            .map_err(hand::throw)?;
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
pub struct core_Dtype {}

#[wasm_bindgen]
impl core_Dtype {
    /// Returns the largest value the width can hold.
    pub fn max(dtype: JsValue) -> Result<i64, JsValue> {
        let dtype = hand::from_js::<mrlyrs::core::Dtype>(&dtype)?;
        let value = dtype.max();
        Ok(value)
    }
}

/// The constraints a caller may put on a random paint.
#[wasm_bindgen]
pub struct core_paint_Config {}

#[wasm_bindgen]
impl core_paint_Config {
    /// Returns the default Config.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Config::default();
        hand::to_js(&value)
    }
}

/// The seven ways a paint distributes its colors over a cell.
#[wasm_bindgen]
pub struct core_paint_Edition {}

#[wasm_bindgen]
impl core_paint_Edition {
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
pub struct core_paint_Ink {}

#[wasm_bindgen]
impl core_paint_Ink {
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
pub struct core_paint_Scheme {}

#[wasm_bindgen]
impl core_paint_Scheme {
    /// Returns every Scheme in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Scheme::all();
        hand::to_js(&value)
    }
}

/// The side of the figure the primary ink lands on.
#[wasm_bindgen]
pub struct core_paint_Target {}

#[wasm_bindgen]
impl core_paint_Target {
    /// Returns every Target in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::core::paint::Target::all();
        hand::to_js(&value)
    }
}

/// The five construction families a tile can belong to.
#[wasm_bindgen]
pub struct gen_Group {}

#[wasm_bindgen]
impl gen_Group {
    /// Returns every Group in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::Group::all();
        hand::to_js(&value)
    }
}

/// The parity filter over candidate sizes.
#[wasm_bindgen]
pub struct gen_Parity {}

#[wasm_bindgen]
impl gen_Parity {
    /// Returns every Parity in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::Parity::all();
        hand::to_js(&value)
    }
    /// Returns true when the number passes the filter.
    pub fn keep(parity: JsValue, n: usize) -> Result<bool, JsValue> {
        let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
        let value = parity.keep(n);
        Ok(value)
    }
}

/// The constraints a random flat tile is drawn under.
#[wasm_bindgen]
pub struct gen_build_Config2d {}

#[wasm_bindgen]
impl gen_build_Config2d {
    /// Returns the default Config2d.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::build::Config2d::default();
        hand::to_js(&value)
    }
}

/// The constraints a random cube tile is drawn under, shared by the hex pipeline.
#[wasm_bindgen]
pub struct gen_build_Config3d {}

#[wasm_bindgen]
impl gen_build_Config3d {
    /// Returns the default Config3d.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::build::Config3d::default();
        hand::to_js(&value)
    }
}

/// One value for every slot of a tile, or one value per slot.
#[wasm_bindgen]
pub struct gen_name_Slots {}

#[wasm_bindgen]
impl gen_name_Slots {
    /// Returns the default Slots.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::name::Slots::default();
        hand::to_js(&value)
    }
}

/// The named designs a source can point at: the four classics and their four antis.
#[wasm_bindgen]
pub struct gen_recipe_Design {}

#[wasm_bindgen]
impl gen_recipe_Design {
    /// Returns every Design in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::recipe::Design::all();
        hand::to_js(&value)
    }
}

/// The settings an artwork is drawn under.
#[wasm_bindgen]
pub struct gen_variation_Config {}

#[wasm_bindgen]
impl gen_variation_Config {
    /// Returns the default Config.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::variation::Config::default();
        hand::to_js(&value)
    }
}

/// The edge policy of a life grid.
#[wasm_bindgen]
pub struct life_Boundary {}

#[wasm_bindgen]
impl life_Boundary {
    /// Returns every Boundary in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Boundary::all();
        hand::to_js(&value)
    }
    /// Returns whether the edges wrap.
    pub fn wrap(boundary: JsValue) -> Result<bool, JsValue> {
        let boundary = hand::from_js::<mrlyrs::life::Boundary>(&boundary)?;
        let value = boundary.wrap();
        Ok(value)
    }
}

/// The ending of a life run.
#[wasm_bindgen]
pub struct life_Fate {}

#[wasm_bindgen]
impl life_Fate {
    /// Returns every Fate in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Fate::all();
        hand::to_js(&value)
    }
}

/// One ordered layer of a magic composition: a coded design at its own side number.
#[wasm_bindgen]
pub struct math_bang_MagicLayer {}

#[wasm_bindgen]
impl math_bang_MagicLayer {
    /// Pins a design to the side number it renders at.
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(design: &math_name_Bang, number: usize) -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::bang::MagicLayer::new(design.inner.clone(), number);
        hand::to_js(&value)
    }
}

/// The named infinite schedules over an ordered pair of letters.
#[wasm_bindgen]
pub struct math_bang_word_Schedule {}

#[wasm_bindgen]
impl math_bang_word_Schedule {
    /// Returns every Schedule in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::bang::word::Schedule::all();
        hand::to_js(&value)
    }
    /// Returns the letter frequencies the schedule tends to.
    pub fn frequencies(schedule: JsValue) -> Result<JsValue, JsValue> {
        let schedule = hand::from_js::<mrlyrs::math::bang::word::Schedule>(&schedule)?;
        let value = schedule.frequencies();
        hand::to_js(&value)
    }
    /// Returns the letter the schedule takes at the place, zero or one.
    pub fn place(schedule: JsValue, index: usize) -> Result<usize, JsValue> {
        let schedule = hand::from_js::<mrlyrs::math::bang::word::Schedule>(&schedule)?;
        let value = schedule.place(index);
        Ok(value)
    }
}

/// The recipe for one moire layer.
#[wasm_bindgen]
pub struct math_moire_Layer {}

#[wasm_bindgen]
impl math_moire_Layer {
    /// Builds a layer at level 1 on a 512-pixel square lattice.
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(spec: JsValue, number: usize) -> Result<JsValue, JsValue> {
        let spec = hand::from_js::<mrlyrs::math::moire::Spec>(&spec)?;
        let value = mrlyrs::math::moire::Layer::new(spec, number);
        hand::to_js(&value)
    }
}

/// The identity of a design: its code, base and dimension.
#[wasm_bindgen]
pub struct math_moire_Spec {}

#[wasm_bindgen]
impl math_moire_Spec {
    /// Builds a spec from a code, base and dimension.
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(code: JsValue, base: usize, dimension: usize) -> Result<JsValue, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::moire::Spec::new(code, base, dimension);
        hand::to_js(&value)
    }
}

/// The lattice the cells sit on.
#[wasm_bindgen]
pub struct math_name_Lattice {}

#[wasm_bindgen]
impl math_name_Lattice {
    /// Returns the default Lattice.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::name::Lattice::default();
        hand::to_js(&value)
    }
    /// Returns whether this is the square lattice.
    pub fn is_square(lattice: JsValue) -> Result<bool, JsValue> {
        let lattice = hand::from_js::<mrlyrs::math::name::Lattice>(&lattice)?;
        let value = lattice.is_square();
        Ok(value)
    }
    /// Returns the number of unit directions a twist may pick from.
    pub fn units(lattice: JsValue) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrlyrs::math::name::Lattice>(&lattice)?;
        let value = lattice.units();
        Ok(value)
    }
}

/// Where one lattice cell sits relative to a shape.
#[wasm_bindgen]
pub struct math_shape_Region {}

#[wasm_bindgen]
impl math_shape_Region {
    /// Swaps In and Out, keeping Cut.
    pub fn flip(region: JsValue) -> Result<JsValue, JsValue> {
        let region = hand::from_js::<mrlyrs::math::shape::Region>(&region)?;
        let value = region.flip();
        hand::to_js(&value)
    }
}

/// The three classes of layer count the `1/L^2` term of the decay reads.
#[wasm_bindgen]
pub struct math_six_star_Branch {}

#[wasm_bindgen]
impl math_six_star_Branch {
    /// The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.
    pub fn constant(branch: JsValue) -> Result<f64, JsValue> {
        let branch = hand::from_js::<mrlyrs::math::six::star::Branch>(&branch)?;
        let value = branch.constant();
        Ok(value)
    }
    /// The name of the branch.
    pub fn name(branch: JsValue) -> Result<String, JsValue> {
        let branch = hand::from_js::<mrlyrs::math::six::star::Branch>(&branch)?;
        let value = branch.name();
        Ok(value)
    }
    /// The branch of a layer count.
    pub fn of(layers: usize) -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::six::star::Branch::of(layers);
        hand::to_js(&value)
    }
    /// The exact `1/L^2` coefficient at even `L`, absent at odd `L`.
    pub fn residual(branch: JsValue) -> Result<JsValue, JsValue> {
        let branch = hand::from_js::<mrlyrs::math::six::star::Branch>(&branch)?;
        let value = branch.residual();
        hand::to_js(&value)
    }
}

/// The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has.
#[wasm_bindgen]
pub struct math_spin_Blend {}

#[wasm_bindgen]
impl math_spin_Blend {
    /// Merges one site's copies into the blended value.
    pub fn fold(blend: JsValue, values: &[f32]) -> Result<f32, JsValue> {
        let blend = hand::from_js::<mrlyrs::math::spin::Blend>(&blend)?;
        let value = blend.fold(values);
        Ok(value)
    }
    /// Reads a blend by name: mean, sum, union, meet, parity or difference.
    pub fn named(name: &str) -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::spin::Blend::named(name);
        hand::to_js(&value)
    }
}

/// The mass of a byte grid taken as a wheel: how many pencils of each kind it seats.
#[wasm_bindgen]
pub struct math_spirograph_Seats {}

#[wasm_bindgen]
impl math_spirograph_Seats {
    /// Returns the default Seats.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::spirograph::Seats::default();
        hand::to_js(&value)
    }
}

/// What a point of the ring is.
#[wasm_bindgen]
pub struct num_gauss_Class {}

#[wasm_bindgen]
impl num_gauss_Class {
    /// Returns whether the class is prime.
    pub fn prime(class_: JsValue) -> Result<bool, JsValue> {
        let class_ = hand::from_js::<mrlyrs::num::gauss::Class>(&class_)?;
        let value = class_.prime();
        Ok(value)
    }
    /// Returns the class as a word.
    pub fn word(class_: JsValue) -> Result<String, JsValue> {
        let class_ = hand::from_js::<mrlyrs::num::gauss::Class>(&class_)?;
        let value = class_.word();
        Ok(value)
    }
}

/// The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice.
#[wasm_bindgen]
pub struct num_gauss_Ring {}

#[wasm_bindgen]
impl num_gauss_Ring {
    /// Returns the unit multiples of a point, the point first, turning anticlockwise.
    pub fn associates(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.associates(a, b);
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin.
    pub fn canon(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.canon(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the conjugate: the mirror image in the real axis.
    pub fn conjugate(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.conjugate(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the count of points within the reach: the square or the hexagon.
    pub fn count(ring: JsValue, radius: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.count(radius);
        Ok(value)
    }
    /// Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.
    pub fn div_rem(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let w = (
            hand::i64_from_js(&hand::item(&w, 0)?)?,
            hand::i64_from_js(&hand::item(&w, 1)?)?,
        );
        let value = ring.div_rem(z, w);
        Ok(hand::tuple_to_js(&[
            hand::tuple_to_js(&[JsValue::from(value.0 .0), JsValue::from(value.0 .1)]),
            hand::tuple_to_js(&[JsValue::from(value.1 .0), JsValue::from(value.1 .1)]),
        ]))
    }
    /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
    pub fn fate(ring: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let n = hand::u64_from_js(&n)?;
        let value = ring.fate(n);
        hand::to_js(&value)
    }
    /// Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.
    pub fn gaussian_gcd(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let w = (
            hand::i64_from_js(&hand::item(&w, 0)?)?,
            hand::i64_from_js(&hand::item(&w, 1)?)?,
        );
        let value = ring.gaussian_gcd(z, w);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.
    pub fn inert(ring: JsValue, p: JsValue) -> Result<bool, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let p = hand::u64_from_js(&p)?;
        let value = ring.inert(p);
        Ok(value)
    }
    /// Returns the product of two points.
    pub fn mul(ring: JsValue, arg1: JsValue, arg2: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let arg1 = (
            hand::i64_from_js(&hand::item(&arg1, 0)?)?,
            hand::i64_from_js(&hand::item(&arg1, 1)?)?,
        );
        let arg2 = (
            hand::i64_from_js(&hand::item(&arg2, 0)?)?,
            hand::i64_from_js(&hand::item(&arg2, 1)?)?,
        );
        let value = ring.mul(arg1, arg2);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Reads a ring from its name.
    pub fn named(name: &str) -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::gauss::Ring::named(name);
        hand::to_js(&value)
    }
    /// Returns the point nearest a place in the plane.
    pub fn nearest(ring: JsValue, x: f64, y: f64) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.nearest(x, y);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the norm of a point: its squared length.
    pub fn norm(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.norm(a, b);
        Ok(value)
    }
    /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
    pub fn place(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.place(a, b);
        hand::to_js(&value)
    }
    /// Returns the one rational prime that ramifies: 2 or 3.
    pub fn ramified(ring: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.ramified();
        Ok(value)
    }
    /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
    pub fn reach(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.reach(a, b);
        Ok(value)
    }
    /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
    pub fn symmetry(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.symmetry();
        Ok(value)
    }
    /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
    pub fn top(ring: JsValue, radius: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.top(radius);
        Ok(value)
    }
    /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
    pub fn turn(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.turn(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the count of units: 4 or 6.
    pub fn units(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.units();
        Ok(value)
    }
    /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
    pub fn whole(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.whole(a, b);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
    }
}

/// The four ways the word lifts from a line to the plane, one sign at every site.
#[wasm_bindgen]
pub struct num_morse_Lift {}

#[wasm_bindgen]
impl num_morse_Lift {
    /// Returns every Lift in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::morse::Lift::all();
        hand::to_js(&value)
    }
    /// Returns the sign at a site, zero for plus one and one for minus one.
    pub fn at(lift: JsValue, i: JsValue, j: JsValue) -> Result<u8, JsValue> {
        let lift = hand::from_js::<mrlyrs::num::morse::Lift>(&lift)?;
        let i = hand::u64_from_js(&i)?;
        let j = hand::u64_from_js(&j)?;
        let value = lift.at(i, j);
        Ok(value)
    }
    /// Returns the lift's formula, written the way the page prints it.
    pub fn formula(lift: JsValue) -> Result<String, JsValue> {
        let lift = hand::from_js::<mrlyrs::num::morse::Lift>(&lift)?;
        let value = lift.formula();
        Ok(value)
    }
}

/// Which cells of the square winding grow into a tile.
#[wasm_bindgen]
pub struct num_spiral_Growth {}

#[wasm_bindgen]
impl num_spiral_Growth {
    /// Returns every Growth in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Growth::all();
        hand::to_js(&value)
    }
}

/// The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right.
#[wasm_bindgen]
pub struct num_spiral_Lattice {}

#[wasm_bindgen]
impl num_spiral_Lattice {
    /// Returns every Lattice in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Lattice::all();
        hand::to_js(&value)
    }
    /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
    pub fn count(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let value = lattice.count(side);
        Ok(value)
    }
    /// Returns the number at a cell, one at the origin.
    pub fn n(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.n(x, y);
        Ok(value)
    }
    /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
    pub fn radius(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let value = lattice.radius(side);
        Ok(value)
    }
    /// Returns the ring a number sits on, zero for one.
    pub fn ring(lattice: JsValue, n: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.ring(n);
        Ok(value)
    }
    /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
    pub fn ring_of(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.ring_of(x, y);
        Ok(value)
    }
    /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
    pub fn xy(lattice: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.xy(n);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
}

/// What a cell is painted for.
#[wasm_bindgen]
pub struct num_spiral_Mark {}

#[wasm_bindgen]
impl num_spiral_Mark {
    /// Returns every Mark in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Mark::all();
        hand::to_js(&value)
    }
}
