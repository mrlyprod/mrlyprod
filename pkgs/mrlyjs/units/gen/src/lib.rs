#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe
#[wasm_bindgen]
pub fn background(seed: JsValue, width: usize, height: usize) -> Result<Vec<u8>, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrlyrs::gen::background(seed, width, height).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the flat cell the tile describes.
#[wasm_bindgen]
pub fn build_build_2d(tile: &Tile) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::build_2d(&tile.inner).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the cube the tile describes.
#[wasm_bindgen]
pub fn build_build_3d(tile: &Tile) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::build_3d(&tile.inner).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the tile's cube and flattens it through its projection.
#[wasm_bindgen]
pub fn build_build_6d(hex: JsValue) -> Result<JsValue, JsValue> {
    let hex = hand::from_js::<mrlyrs::gen::build::HexTile>(&hex)?;
    let value = mrlyrs::gen::build::build_6d(&hex).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Draws a random flat tile from the stream, rotations from the four quarter-turns.
#[wasm_bindgen]
pub fn build_create_2d(config: JsValue, rng: &mut hand::Rng) -> Result<Tile, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<2>>(&config)?;
    let value = mrlyrs::gen::build::create_2d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(Tile { inner: value })
}

/// Draws a cube tile from the config with cube orientations drawn from the stream.
#[wasm_bindgen]
pub fn build_create_3d(config: JsValue, rng: &mut hand::Rng) -> Result<Tile, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<3>>(&config)?;
    let value = mrlyrs::gen::build::create_3d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(Tile { inner: value })
}

/// Draws a cube tile from the config under a projection drawn from the stream.
#[wasm_bindgen]
pub fn build_create_6d(config: JsValue, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::draw::ConfigNd<3>>(&config)?;
    let value = mrlyrs::gen::build::create_6d(&config, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Draws a random flat tile up to the given size under the default config.
#[wasm_bindgen]
pub fn build_random_tile_2d(max_size: usize, rng: &mut hand::Rng) -> Result<Tile, JsValue> {
    let value = mrlyrs::gen::build::random_tile_2d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(Tile { inner: value })
}

/// Draws a random cube tile up to the given size.
#[wasm_bindgen]
pub fn build_random_tile_3d(max_size: usize, rng: &mut hand::Rng) -> Result<Tile, JsValue> {
    let value = mrlyrs::gen::build::random_tile_3d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(Tile { inner: value })
}

/// Draws a random cube tile up to the given size under a random projection.
#[wasm_bindgen]
pub fn build_random_tile_6d(max_size: usize, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::build::random_tile_6d(max_size, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the plane's bang code of a classic design, or None for one outside the plane.
#[wasm_bindgen]
pub fn classic_code(design: JsValue) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::classic_code(design);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the bang code of a named design in a dimension, or None where it has no design.
#[wasm_bindgen]
pub fn classic_code_nd(design: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::classic_code_nd(design, dimension);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Draws a hex key of the given length from the stream.
#[wasm_bindgen]
pub fn hex_key(length: usize, rng: &mut hand::Rng) -> Result<String, JsValue> {
    let value = mrlyrs::gen::hex_key(length, rng.stream());
    Ok(value)
}

/// Draws one named design from the stream.
#[wasm_bindgen]
pub fn random_design(rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::random_design(rng.stream());
    hand::to_js(&value)
}

/// Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.
#[wasm_bindgen]
pub fn random_rotation(design: JsValue, rng: &mut hand::Rng) -> Result<u8, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::gen::random_rotation(design, rng.stream());
    Ok(value)
}

/// Returns the classic designs for a dimension.
#[wasm_bindgen]
pub fn recipe_classics(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::classics(dimension);
    hand::to_js(&value)
}

/// Returns every flat size in the range that passes the parity filter.
#[wasm_bindgen]
pub fn recipe_generals(min_size: usize, max_size: usize, parity: JsValue) -> Result<Vec<usize>, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::generals(min_size, max_size, parity);
    Ok(value)
}

/// Returns every factor list of depth two and beyond whose product lands in the size range.
#[wasm_bindgen]
pub fn recipe_nestings(min_size: usize, max_size: usize, parity: JsValue) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::nestings(min_size, max_size, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns every factor and level whose power lands in the size range.
#[wasm_bindgen]
pub fn recipe_powers(min_size: usize, max_size: usize, parity: JsValue) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::powers(min_size, max_size, parity);
    hand::to_js(&value)
}

/// Returns every count-long factor list whose product lands in the size range.
#[wasm_bindgen]
pub fn recipe_products(min_size: usize, max_size: usize, count: usize, parity: JsValue) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrlyrs::gen::Parity>(&parity)?;
    let value = mrlyrs::gen::recipe::products(min_size, max_size, count, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the side a factor raised to a level makes, or None when no usize holds it.
#[wasm_bindgen]
pub fn recipe_size(number: JsValue, level: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::i64_from_js(&number)?;
    let level = hand::i64_from_js(&level)?;
    let value = mrlyrs::gen::recipe::size(number, level);
    hand::to_js(&value)
}

/// Builds the mask of a mosaic tile: the two trees of the side, two where they cross.
#[wasm_bindgen]
pub fn tree_mask(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::tree_mask(n).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Draws a variation's seed from the stream, then the variation itself on that seed, with a
#[wasm_bindgen]
pub fn variation_create(config: JsValue, rng: &mut hand::Rng) -> Result<variation_Variation, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::variation::Config>(&config)?;
    let value = mrlyrs::gen::variation::create(&config, rng.stream()).map_err(hand::throw)?;
    Ok(variation_Variation { inner: value })
}

/// Builds the variation's base cell and draws its paint from the stream, painting the base
#[wasm_bindgen]
pub fn variation_generate(variation: &variation_Variation, config: JsValue, rng: &mut hand::Rng) -> Result<variation_Variation, JsValue> {
    let config = hand::from_js::<mrlyrs::gen::variation::Config>(&config)?;
    let value = mrlyrs::gen::variation::generate(variation.inner.clone(), &config, rng.stream()).map_err(hand::throw)?;
    Ok(variation_Variation { inner: value })
}

/// Renders every file of the variation to PNG at the given scale, scattering a Random edition
#[wasm_bindgen]
pub fn variation_render(variation: &variation_Variation, scale: usize, rng: &mut hand::Rng) -> Result<variation_Variation, JsValue> {
    let value = mrlyrs::gen::variation::render(variation.inner.clone(), scale, rng.stream()).map_err(hand::throw)?;
    Ok(variation_Variation { inner: value })
}

/// The five classic designs of the plane.
#[wasm_bindgen]
pub fn recipe_CLASSICS_2D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::CLASSICS_2D;
    hand::to_js(&value)
}

/// The six classic designs of the cube.
#[wasm_bindgen]
pub fn recipe_CLASSICS_3D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::gen::recipe::CLASSICS_3D;
    hand::to_js(&value)
}

/// The deepest fractal level a tile may take.
#[wasm_bindgen]
pub fn recipe_MAX_LEVEL() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_LEVEL;
    Ok(value)
}

/// The largest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn recipe_MAX_SIDE() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_SIDE;
    Ok(value)
}

/// The most slots a magic tile may take.
#[wasm_bindgen]
pub fn recipe_MAX_SLOTS() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MAX_SLOTS;
    Ok(value)
}

/// The smallest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn recipe_MIN_SIDE() -> Result<usize, JsValue> {
    let value = mrlyrs::gen::recipe::MIN_SIDE;
    Ok(value)
}

/// A complete recipe for one tile.
#[wasm_bindgen]
pub struct Tile {
    inner: mrlyrs::gen::Tile,
}

#[wasm_bindgen]
impl Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Tile, JsValue> {
        Ok(Tile { inner: hand::from_js(&data)? })
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
    /// The base factor of the construction.
    #[wasm_bindgen(getter)]
    pub fn factor(&self) -> Result<usize, JsValue> {
        let value = self.inner.factor;
        Ok(value)
    }
    /// The origin of each layer.
    #[wasm_bindgen(getter)]
    pub fn sources(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.sources.clone();
        hand::to_js(&value)
    }
    /// The grid size of each source.
    #[wasm_bindgen(getter)]
    pub fn numbers(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.numbers.clone();
        Ok(value)
    }
    /// The fractal level of each source.
    #[wasm_bindgen(getter)]
    pub fn levels(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.levels.clone();
        Ok(value)
    }
    /// The quarter-turn rotation of each source.
    #[wasm_bindgen(getter)]
    pub fn rotations(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.rotations.clone();
        Ok(value)
    }
    /// Whether the finished tile inverts.
    #[wasm_bindgen(getter)]
    pub fn invert(&self) -> Result<bool, JsValue> {
        let value = self.inner.invert;
        Ok(value)
    }
    /// Whether the finished tile flips.
    #[wasm_bindgen(getter)]
    pub fn flip(&self) -> Result<bool, JsValue> {
        let value = self.inner.flip;
        Ok(value)
    }
    /// The tile's width in cells.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> Result<usize, JsValue> {
        let value = self.inner.width;
        Ok(value)
    }
    /// The tile's height in cells.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height;
        Ok(value)
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
    pub fn new(group: JsValue) -> Result<Tile, JsValue> {
        let group = hand::from_js::<mrlyrs::gen::Group>(&group)?;
        let value = mrlyrs::gen::Tile::new(group);
        Ok(Tile { inner: value })
    }
    /// Recomputes the factor and side length the group and numbers imply, zero when they overflow.
    pub fn resize(&mut self) -> Result<(), JsValue> {
        self.inner.resize();
        Ok(())
    }
    /// Sets the tile's width and height.
    pub fn size(&self, width: usize, height: usize) -> Result<Tile, JsValue> {
        let value = self.inner.clone().size(width, height);
        Ok(Tile { inner: value })
    }
}

/// A tile recipe folded to its one canonical object.
#[wasm_bindgen]
pub struct name_Tile {
    inner: mrlyrs::gen::name::Tile,
}

#[wasm_bindgen]
impl name_Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<name_Tile, JsValue> {
        Ok(name_Tile { inner: hand::from_js(&data)? })
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
    /// The mask code of a special tile.
    #[wasm_bindgen(getter)]
    pub fn special(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.special;
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    /// The letters of a magic tile, first letter outermost.
    #[wasm_bindgen(getter)]
    pub fn magic(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.magic.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    /// The three codes of a mosaic tile.
    #[wasm_bindgen(getter)]
    pub fn mosaic(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mosaic.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    /// The side of the mask of a special or mosaic tile.
    #[wasm_bindgen(getter)]
    pub fn factor(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.factor;
        hand::to_js(&value)
    }
    /// The side each slot renders at, one per letter for a magic tile.
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.side.clone();
        hand::to_js(&value)
    }
    /// The power a fractal tile is raised to, absent at one.
    #[wasm_bindgen(getter)]
    pub fn level(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.level;
        hand::to_js(&value)
    }
    /// The quarter turns of each slot, absent when nothing turns.
    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.turn.clone();
        hand::to_js(&value)
    }
    /// Whether a special tile flips its mask.
    #[wasm_bindgen(getter)]
    pub fn flip(&self) -> Result<bool, JsValue> {
        let value = self.inner.flip;
        Ok(value)
    }
    /// Whether the finished tile inverts.
    #[wasm_bindgen(getter)]
    pub fn invert(&self) -> Result<bool, JsValue> {
        let value = self.inner.invert;
        Ok(value)
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::checked(self.inner.clone()).map_err(hand::throw)?;
        Ok(name_Tile { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(name_Tile { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(name_Tile { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<name_Tile, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(name_Tile { inner: value })
    }
    /// Folds a recipe to its name.
    pub fn of(recipe: &Tile) -> Result<name_Tile, JsValue> {
        let value = mrlyrs::gen::name::Tile::of(&recipe.inner).map_err(hand::throw)?;
        Ok(name_Tile { inner: value })
    }
    /// Builds the recipe the name folds, resized and checked.
    pub fn recipe(&self) -> Result<Tile, JsValue> {
        let value = self.inner.recipe().map_err(hand::throw)?;
        Ok(Tile { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_file(&self.inner).map_err(hand::throw)?;
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
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_mrly(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_url(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
}

/// One rendering of an artwork, sized in tile repetitions.
#[wasm_bindgen]
pub struct variation_File {
    inner: mrlyrs::gen::variation::File,
}

#[wasm_bindgen]
impl variation_File {
    /// Reads the File from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<variation_File, JsValue> {
        Ok(variation_File { inner: hand::from_js(&data)? })
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
    /// The count of tile repetitions down.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Result<usize, JsValue> {
        let value = self.inner.height;
        Ok(value)
    }
    /// The encoded PNG bytes, empty until rendered and left out of the json.
    #[wasm_bindgen(getter)]
    pub fn png(&self) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.png.clone();
        Ok(value)
    }
    /// Builds a file of the given repetition counts with no PNG bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Result<variation_File, JsValue> {
        let value = mrlyrs::gen::variation::File::new(width, height);
        Ok(variation_File { inner: value })
    }
}

/// One seeded artwork, from tile recipe to rendered files.
#[wasm_bindgen]
pub struct variation_Variation {
    inner: mrlyrs::gen::variation::Variation,
}

#[wasm_bindgen]
impl variation_Variation {
    /// Reads the Variation from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<variation_Variation, JsValue> {
        Ok(variation_Variation { inner: hand::from_js(&data)? })
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
    /// The seed the variation is drawn under.
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> Result<u64, JsValue> {
        let value = self.inner.seed;
        Ok(value)
    }
    /// The paint edition.
    #[wasm_bindgen(getter)]
    pub fn edition(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.edition;
        hand::to_js(&value)
    }
    /// The primary inks, when the config fixes them.
    #[wasm_bindgen(getter)]
    pub fn primaries(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.primaries.clone();
        hand::to_js(&value)
    }
    /// The tile recipe.
    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> Result<Tile, JsValue> {
        let value = self.inner.tile.clone();
        Ok(Tile { inner: value })
    }
    /// The mask tile, present only under the Neighbors edition.
    #[wasm_bindgen(getter)]
    pub fn mask(&self) -> Result<Option<Tile>, JsValue> {
        let value = self.inner.mask.clone();
        Ok(value.map(|inner| Tile { inner }))
    }
    /// The built base cell, set by generate and left out of the json.
    #[wasm_bindgen(getter)]
    pub fn base(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.base.clone();
        hand::option_to_js(value.as_ref(), hand::cell2d_to_js)
    }
    /// The renderings, filled by render.
    #[wasm_bindgen(getter)]
    pub fn files(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.files.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(variation_File { inner: x1.clone() })))
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

/// The five construction families a tile can belong to.
#[wasm_bindgen]
pub struct Group {}

#[wasm_bindgen]
impl Group {
    /// Returns every Group in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::Group::all();
        hand::to_js(&value)
    }
}

/// The parity filter over candidate sizes.
#[wasm_bindgen]
pub struct Parity {}

#[wasm_bindgen]
impl Parity {
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

/// The named designs a source can point at: the four classics and their four antis.
#[wasm_bindgen]
pub struct recipe_Design {}

#[wasm_bindgen]
impl recipe_Design {
    /// Returns every Design in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::gen::recipe::Design::all();
        hand::to_js(&value)
    }
}
