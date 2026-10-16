#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Draws one seeded artwork and returns its image: a random flat tile under the default recipe
#[wasm_bindgen]
pub fn tile_background(seed: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrly::tile::background(seed, width, height).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

/// Builds the flat cell the tile describes.
#[wasm_bindgen]
pub fn tile_build_build_2d(tile: &tile_Tile) -> Result<JsValue, JsValue> {
    let value = mrly::tile::build::build_2d(&tile.inner).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the cube the tile describes.
#[wasm_bindgen]
pub fn tile_build_build_3d(tile: &tile_Tile) -> Result<JsValue, JsValue> {
    let value = mrly::tile::build::build_3d(&tile.inner).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the tile's cube and flattens it through its projection.
#[wasm_bindgen]
pub fn tile_build_build_6d(hex: JsValue) -> Result<JsValue, JsValue> {
    let hex = hand::from_js::<mrly::tile::build::HexTile>(&hex)?;
    let value = mrly::tile::build::build_6d(&hex).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Draws a random flat tile from the stream, rotations from the four quarter-turns.
#[wasm_bindgen]
pub fn tile_build_create_2d(config: JsValue, rng: &mut hand::Rng) -> Result<tile_Tile, JsValue> {
    let config = hand::from_js::<mrly::tile::draw::ConfigNd<2>>(&config)?;
    let value = mrly::tile::build::create_2d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

/// Draws a cube tile from the config with cube orientations drawn from the stream.
#[wasm_bindgen]
pub fn tile_build_create_3d(config: JsValue, rng: &mut hand::Rng) -> Result<tile_Tile, JsValue> {
    let config = hand::from_js::<mrly::tile::draw::ConfigNd<3>>(&config)?;
    let value = mrly::tile::build::create_3d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

/// Draws a cube tile from the config under a projection drawn from the stream.
#[wasm_bindgen]
pub fn tile_build_create_6d(config: JsValue, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let config = hand::from_js::<mrly::tile::draw::ConfigNd<3>>(&config)?;
    let value = mrly::tile::build::create_6d(&config, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Draws a random flat tile up to the given size under the default config.
#[wasm_bindgen]
pub fn tile_build_random_tile_2d(
    max_size: usize,
    rng: &mut hand::Rng,
) -> Result<tile_Tile, JsValue> {
    let value = mrly::tile::build::random_tile_2d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

/// Draws a random cube tile up to the given size.
#[wasm_bindgen]
pub fn tile_build_random_tile_3d(
    max_size: usize,
    rng: &mut hand::Rng,
) -> Result<tile_Tile, JsValue> {
    let value = mrly::tile::build::random_tile_3d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

/// Draws a random cube tile up to the given size under a random projection.
#[wasm_bindgen]
pub fn tile_build_random_tile_6d(max_size: usize, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrly::tile::build::random_tile_6d(max_size, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the plane's bang code of a classic design, or None for one outside the plane.
#[wasm_bindgen]
pub fn tile_classic_code(design: JsValue) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly::tile::recipe::Design>(&design)?;
    let value = mrly::tile::classic_code(design);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the bang code of a named design in a dimension, or None where it has no design.
#[wasm_bindgen]
pub fn tile_classic_code_nd(design: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly::tile::recipe::Design>(&design)?;
    let value = mrly::tile::classic_code_nd(design, dimension);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Draws a hex key of the given length from the stream.
#[wasm_bindgen]
pub fn tile_hex_key(length: usize, rng: &mut hand::Rng) -> Result<String, JsValue> {
    let value = mrly::tile::hex_key(length, rng.stream());
    Ok(value)
}

/// Draws one of the four flat classics from the stream: carpet, net, vertical tree or void.
#[wasm_bindgen]
pub fn tile_random_design(rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrly::tile::random_design(rng.stream());
    hand::to_js(&value)
}

/// Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.
#[wasm_bindgen]
pub fn tile_random_rotation(design: JsValue, rng: &mut hand::Rng) -> Result<u8, JsValue> {
    let design = hand::from_js::<mrly::tile::recipe::Design>(&design)?;
    let value = mrly::tile::random_rotation(design, rng.stream());
    Ok(value)
}

/// Returns the classic designs for a dimension.
#[wasm_bindgen]
pub fn tile_recipe_classics(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly::tile::recipe::classics(dimension);
    hand::to_js(&value)
}

/// Returns every flat size in the range that passes the parity filter.
#[wasm_bindgen]
pub fn tile_recipe_generals(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<Vec<usize>, JsValue> {
    let parity = hand::from_js::<mrly::tile::Parity>(&parity)?;
    let value = mrly::tile::recipe::generals(min_size, max_size, parity);
    Ok(value)
}

/// Returns every factor list of depth two and beyond whose product lands in the size range.
#[wasm_bindgen]
pub fn tile_recipe_nestings(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly::tile::Parity>(&parity)?;
    let value = mrly::tile::recipe::nestings(min_size, max_size, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns every factor and level whose power lands in the size range.
#[wasm_bindgen]
pub fn tile_recipe_powers(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly::tile::Parity>(&parity)?;
    let value = mrly::tile::recipe::powers(min_size, max_size, parity);
    hand::to_js(&value)
}

/// Returns every count-long factor list whose product lands in the size range.
#[wasm_bindgen]
pub fn tile_recipe_products(
    min_size: usize,
    max_size: usize,
    count: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly::tile::Parity>(&parity)?;
    let value = mrly::tile::recipe::products(min_size, max_size, count, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the side a factor raised to a level makes, or None when no usize holds it.
#[wasm_bindgen]
pub fn tile_recipe_size(number: JsValue, level: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::i64_from_js(&number)?;
    let level = hand::i64_from_js(&level)?;
    let value = mrly::tile::recipe::size(number, level);
    hand::to_js(&value)
}

/// Builds the mask of a mosaic tile: the two trees of the side, two where they cross.
#[wasm_bindgen]
pub fn tile_tree_mask(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly::tile::tree_mask(n).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Draws a variation's seed from the stream, then the variation itself on that seed, with a
#[wasm_bindgen]
pub fn tile_variation_create(
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let config = hand::from_js::<mrly::tile::variation::Config>(&config)?;
    let value = mrly::tile::variation::create(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

/// Builds the variation's base cell and draws its paint from the stream, painting the base
#[wasm_bindgen]
pub fn tile_variation_generate(
    variation: &tile_variation_Variation,
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let config = hand::from_js::<mrly::tile::variation::Config>(&config)?;
    let value = mrly::tile::variation::generate(variation.inner.clone(), &config, rng.stream())
        .map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

/// Renders every file of the variation to an image at the given scale, scattering a Random edition
#[wasm_bindgen]
pub fn tile_variation_render(
    variation: &tile_variation_Variation,
    scale: usize,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let value = mrly::tile::variation::render(variation.inner.clone(), scale, rng.stream())
        .map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

/// The five classic designs of the plane.
#[wasm_bindgen]
pub fn tile_recipe_CLASSICS_2D() -> Result<JsValue, JsValue> {
    let value = mrly::tile::recipe::CLASSICS_2D;
    hand::to_js(&value)
}

/// The six classic designs of the cube.
#[wasm_bindgen]
pub fn tile_recipe_CLASSICS_3D() -> Result<JsValue, JsValue> {
    let value = mrly::tile::recipe::CLASSICS_3D;
    hand::to_js(&value)
}

/// The deepest fractal level a tile may take.
#[wasm_bindgen]
pub fn tile_recipe_MAX_LEVEL() -> Result<usize, JsValue> {
    let value = mrly::tile::recipe::MAX_LEVEL;
    Ok(value)
}

/// The largest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn tile_recipe_MAX_SIDE() -> Result<usize, JsValue> {
    let value = mrly::tile::recipe::MAX_SIDE;
    Ok(value)
}

/// The most slots a magic tile may take.
#[wasm_bindgen]
pub fn tile_recipe_MAX_SLOTS() -> Result<usize, JsValue> {
    let value = mrly::tile::recipe::MAX_SLOTS;
    Ok(value)
}

/// The smallest side, number or factor a tile may take.
#[wasm_bindgen]
pub fn tile_recipe_MIN_SIDE() -> Result<usize, JsValue> {
    let value = mrly::tile::recipe::MIN_SIDE;
    Ok(value)
}

/// A complete recipe for one tile.
#[wasm_bindgen]
pub struct tile_Tile {
    inner: mrly::tile::Tile,
}

#[wasm_bindgen]
impl tile_Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_Tile, JsValue> {
        Ok(tile_Tile {
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
        let value = hand::from_js::<mrly::tile::Group>(&value)?;
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
        let value = hand::from_js::<Vec<mrly::tile::recipe::Source>>(&value)?;
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
    pub fn new(group: JsValue) -> Result<tile_Tile, JsValue> {
        let group = hand::from_js::<mrly::tile::Group>(&group)?;
        let value = mrly::tile::Tile::new(group);
        Ok(tile_Tile { inner: value })
    }
    /// Recomputes the factor and side length the group and numbers imply, zero when they overflow.
    pub fn resize(&mut self) -> Result<(), JsValue> {
        self.inner.resize();
        Ok(())
    }
    /// Sets the tile's width and height.
    pub fn size(&self, width: usize, height: usize) -> Result<tile_Tile, JsValue> {
        let value = self.inner.clone().size(width, height);
        Ok(tile_Tile { inner: value })
    }
}

/// A tile recipe folded to its one canonical object.
#[wasm_bindgen]
pub struct tile_name_Tile {
    inner: mrly::tile::name::Tile,
}

#[wasm_bindgen]
impl tile_name_Tile {
    /// Reads the Tile from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_name_Tile, JsValue> {
        Ok(tile_name_Tile {
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
        let value = hand::from_js::<mrly::tile::name::Slots>(&value)?;
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
        let value = hand::from_js::<mrly::tile::name::Slots>(&value)?;
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
    pub fn checked(&self) -> Result<tile_name_Tile, JsValue> {
        let value =
            <mrly::tile::name::Tile as mrly::math::name::Named>::checked(self.inner.clone())
                .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    /// Folds a recipe to its name.
    pub fn of(recipe: &tile_Tile) -> Result<tile_name_Tile, JsValue> {
        let value = mrly::tile::name::Tile::of(&recipe.inner).map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    /// Builds the recipe the name folds, resized and checked.
    pub fn recipe(&self) -> Result<tile_Tile, JsValue> {
        let value = self.inner.recipe().map_err(hand::throw)?;
        Ok(tile_Tile { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly::tile::name::Tile as mrly::math::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// One rendering of an artwork, sized in tile repetitions.
#[wasm_bindgen]
pub struct tile_variation_File {
    inner: mrly::tile::variation::File,
}

#[wasm_bindgen]
impl tile_variation_File {
    /// Reads the File from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_variation_File, JsValue> {
        Ok(tile_variation_File {
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
    /// The rendered image, absent until rendered and left out of the json.
    #[wasm_bindgen(getter)]
    pub fn image(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.image.clone();
        hand::option_to_js(value.as_ref(), hand::image_to_js)
    }
    #[wasm_bindgen(setter)]
    pub fn set_image(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, hand::image_from_js)?;
        self.inner.image = value;
        Ok(())
    }
    /// Builds a file of the given repetition counts with no image.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Result<tile_variation_File, JsValue> {
        let value = mrly::tile::variation::File::new(width, height);
        Ok(tile_variation_File { inner: value })
    }
}

/// One seeded artwork, from tile recipe to rendered files.
#[wasm_bindgen]
pub struct tile_variation_Variation {
    inner: mrly::tile::variation::Variation,
}

#[wasm_bindgen]
impl tile_variation_Variation {
    /// Reads the Variation from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_variation_Variation, JsValue> {
        Ok(tile_variation_Variation {
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
        let value = hand::from_js::<mrly::core::paint::Edition>(&value)?;
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
        let value = hand::from_js::<Option<Vec<mrly::core::paint::Ink>>>(&value)?;
        self.inner.primaries = value;
        Ok(())
    }
    /// The tile recipe.
    #[wasm_bindgen(getter)]
    pub fn tile(&self) -> Result<tile_Tile, JsValue> {
        let value = self.inner.tile.clone();
        Ok(tile_Tile { inner: value })
    }
    #[wasm_bindgen(setter)]
    pub fn set_tile(&mut self, value: &tile_Tile) -> Result<(), JsValue> {
        self.inner.tile = value.inner.clone();
        Ok(())
    }
    /// The mask tile, present only under the Neighbors edition.
    #[wasm_bindgen(getter)]
    pub fn mask(&self) -> Result<Option<tile_Tile>, JsValue> {
        let value = self.inner.mask.clone();
        Ok(value.map(|inner| tile_Tile { inner }))
    }
    #[wasm_bindgen(setter)]
    pub fn set_mask(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, |x1| {
            hand::from_js::<mrly::tile::Tile>(&hand::plain(x1)?)
        })?;
        self.inner.mask = value;
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
            Ok(JsValue::from(tile_variation_File { inner: x1.clone() }))
        })
    }
    #[wasm_bindgen(setter)]
    pub fn set_files(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, |x1| {
            hand::from_js::<mrly::tile::variation::File>(&hand::plain(x1)?)
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

/// The five construction families a tile can belong to.
#[wasm_bindgen]
pub struct tile_Group {}

#[wasm_bindgen]
impl tile_Group {
    /// Returns every Group in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly::tile::Group::all();
        hand::to_js(&value)
    }
}

/// The parity filter over candidate sizes.
#[wasm_bindgen]
pub struct tile_Parity {}

#[wasm_bindgen]
impl tile_Parity {
    /// Returns every Parity in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly::tile::Parity::all();
        hand::to_js(&value)
    }
    /// Returns true when the number passes the filter.
    pub fn keep(parity: JsValue, n: usize) -> Result<bool, JsValue> {
        let parity = hand::from_js::<mrly::tile::Parity>(&parity)?;
        let value = parity.keep(n);
        Ok(value)
    }
}

/// The constraints a random flat tile is drawn under.
#[wasm_bindgen]
pub struct tile_build_Config2d {}

#[wasm_bindgen]
impl tile_build_Config2d {
    /// Returns the default Config2d.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly::tile::build::Config2d::default();
        hand::to_js(&value)
    }
}

/// The constraints a random cube tile is drawn under, shared by the hex pipeline.
#[wasm_bindgen]
pub struct tile_build_Config3d {}

#[wasm_bindgen]
impl tile_build_Config3d {
    /// Returns the default Config3d.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly::tile::build::Config3d::default();
        hand::to_js(&value)
    }
}

/// One value for every slot of a tile, or one value per slot.
#[wasm_bindgen]
pub struct tile_name_Slots {}

#[wasm_bindgen]
impl tile_name_Slots {
    /// Returns the default Slots.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly::tile::name::Slots::default();
        hand::to_js(&value)
    }
}

/// The named designs a source can point at: the four classics and their four antis.
#[wasm_bindgen]
pub struct tile_recipe_Design {}

#[wasm_bindgen]
impl tile_recipe_Design {
    /// Returns every Design in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly::tile::recipe::Design::all();
        hand::to_js(&value)
    }
}

/// The settings an artwork is drawn under.
#[wasm_bindgen]
pub struct tile_variation_Config {}

#[wasm_bindgen]
impl tile_variation_Config {
    /// Returns the default Config.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly::tile::variation::Config::default();
        hand::to_js(&value)
    }
}
