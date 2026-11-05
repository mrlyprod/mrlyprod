#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_math::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tile_background(seed: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let seed = hand::u64_from_js(&seed)?;
    let value = mrly_tile::background(seed, width, height).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn tile_build_build_2d(tile: &tile_Tile) -> Result<JsValue, JsValue> {
    let value = mrly_tile::build::build_2d(&tile.inner).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn tile_build_build_3d(tile: &tile_Tile) -> Result<JsValue, JsValue> {
    let value = mrly_tile::build::build_3d(&tile.inner).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn tile_build_build_6d(hex: JsValue) -> Result<JsValue, JsValue> {
    let hex = hand::from_js::<mrly_tile::build::HexTile>(&hex)?;
    let value = mrly_tile::build::build_6d(&hex).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn tile_build_create_2d(config: JsValue, rng: &mut hand::Rng) -> Result<tile_Tile, JsValue> {
    let config = hand::from_js::<mrly_tile::draw::ConfigNd<2>>(&config)?;
    let value = mrly_tile::build::create_2d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

#[wasm_bindgen]
pub fn tile_build_create_3d(config: JsValue, rng: &mut hand::Rng) -> Result<tile_Tile, JsValue> {
    let config = hand::from_js::<mrly_tile::draw::ConfigNd<3>>(&config)?;
    let value = mrly_tile::build::create_3d(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

#[wasm_bindgen]
pub fn tile_build_create_6d(config: JsValue, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let config = hand::from_js::<mrly_tile::draw::ConfigNd<3>>(&config)?;
    let value = mrly_tile::build::create_6d(&config, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_build_random_tile_2d(
    max_size: usize,
    rng: &mut hand::Rng,
) -> Result<tile_Tile, JsValue> {
    let value = mrly_tile::build::random_tile_2d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

#[wasm_bindgen]
pub fn tile_build_random_tile_3d(
    max_size: usize,
    rng: &mut hand::Rng,
) -> Result<tile_Tile, JsValue> {
    let value = mrly_tile::build::random_tile_3d(max_size, rng.stream()).map_err(hand::throw)?;
    Ok(tile_Tile { inner: value })
}

#[wasm_bindgen]
pub fn tile_build_random_tile_6d(max_size: usize, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrly_tile::build::random_tile_6d(max_size, rng.stream()).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_classic_code(design: JsValue) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_tile::recipe::Design>(&design)?;
    let value = mrly_tile::classic_code(design);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn tile_classic_code_nd(design: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_tile::recipe::Design>(&design)?;
    let value = mrly_tile::classic_code_nd(design, dimension);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn tile_hex_key(length: usize, rng: &mut hand::Rng) -> Result<String, JsValue> {
    let value = mrly_tile::hex_key(length, rng.stream());
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_random_design(rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrly_tile::random_design(rng.stream());
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_random_rotation(design: JsValue, rng: &mut hand::Rng) -> Result<u8, JsValue> {
    let design = hand::from_js::<mrly_tile::recipe::Design>(&design)?;
    let value = mrly_tile::random_rotation(design, rng.stream());
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_recipe_classics(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_tile::recipe::classics(dimension);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_recipe_generals(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<Vec<usize>, JsValue> {
    let parity = hand::from_js::<mrly_tile::Parity>(&parity)?;
    let value = mrly_tile::recipe::generals(min_size, max_size, parity);
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_recipe_nestings(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly_tile::Parity>(&parity)?;
    let value = mrly_tile::recipe::nestings(min_size, max_size, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn tile_recipe_powers(
    min_size: usize,
    max_size: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly_tile::Parity>(&parity)?;
    let value = mrly_tile::recipe::powers(min_size, max_size, parity);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_recipe_products(
    min_size: usize,
    max_size: usize,
    count: usize,
    parity: JsValue,
) -> Result<JsValue, JsValue> {
    let parity = hand::from_js::<mrly_tile::Parity>(&parity)?;
    let value = mrly_tile::recipe::products(min_size, max_size, count, parity);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn tile_recipe_size(number: JsValue, level: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::i64_from_js(&number)?;
    let level = hand::i64_from_js(&level)?;
    let value = mrly_tile::recipe::size(number, level);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_tree_mask(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_tile::tree_mask(n).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn tile_variation_create(
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let config = hand::from_js::<mrly_tile::variation::Config>(&config)?;
    let value = mrly_tile::variation::create(&config, rng.stream()).map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

#[wasm_bindgen]
pub fn tile_variation_generate(
    variation: &tile_variation_Variation,
    config: JsValue,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let config = hand::from_js::<mrly_tile::variation::Config>(&config)?;
    let value = mrly_tile::variation::generate(variation.inner.clone(), &config, rng.stream())
        .map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

#[wasm_bindgen]
pub fn tile_variation_render(
    variation: &tile_variation_Variation,
    scale: usize,
    rng: &mut hand::Rng,
) -> Result<tile_variation_Variation, JsValue> {
    let value = mrly_tile::variation::render(variation.inner.clone(), scale, rng.stream())
        .map_err(hand::throw)?;
    Ok(tile_variation_Variation { inner: value })
}

#[wasm_bindgen]
pub fn tile_recipe_CLASSICS_2D() -> Result<JsValue, JsValue> {
    let value = mrly_tile::recipe::CLASSICS_2D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_recipe_CLASSICS_3D() -> Result<JsValue, JsValue> {
    let value = mrly_tile::recipe::CLASSICS_3D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn tile_recipe_MAX_LEVEL() -> Result<usize, JsValue> {
    let value = mrly_tile::recipe::MAX_LEVEL;
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_recipe_MAX_SIDE() -> Result<usize, JsValue> {
    let value = mrly_tile::recipe::MAX_SIDE;
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_recipe_MAX_SLOTS() -> Result<usize, JsValue> {
    let value = mrly_tile::recipe::MAX_SLOTS;
    Ok(value)
}

#[wasm_bindgen]
pub fn tile_recipe_MIN_SIDE() -> Result<usize, JsValue> {
    let value = mrly_tile::recipe::MIN_SIDE;
    Ok(value)
}

#[wasm_bindgen]
pub struct tile_Tile {
    inner: mrly_tile::Tile,
}

#[wasm_bindgen]
impl tile_Tile {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_Tile, JsValue> {
        Ok(tile_Tile {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn group(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.group;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_tile::Group>(&value)?;
        self.inner.group = value;
        Ok(())
    }
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
    #[wasm_bindgen(getter)]
    pub fn sources(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.sources.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_sources(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrly_tile::recipe::Source>>(&value)?;
        self.inner.sources = value;
        Ok(())
    }
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
    pub fn check(&self) -> Result<(), JsValue> {
        self.inner.check().map_err(hand::throw)?;
        Ok(())
    }
    pub fn degenerate(&self) -> Result<bool, JsValue> {
        let value = self.inner.degenerate();
        Ok(value)
    }
    pub fn max_size(&self) -> Result<usize, JsValue> {
        let value = self.inner.max_size();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(group: JsValue) -> Result<tile_Tile, JsValue> {
        let group = hand::from_js::<mrly_tile::Group>(&group)?;
        let value = mrly_tile::Tile::new(group);
        Ok(tile_Tile { inner: value })
    }
    pub fn resize(&mut self) -> Result<(), JsValue> {
        self.inner.resize();
        Ok(())
    }
    pub fn size(&self, width: usize, height: usize) -> Result<tile_Tile, JsValue> {
        let value = self.inner.clone().size(width, height);
        Ok(tile_Tile { inner: value })
    }
}

#[wasm_bindgen]
pub struct tile_name_Tile {
    inner: mrly_tile::name::Tile,
}

#[wasm_bindgen]
impl tile_name_Tile {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_name_Tile, JsValue> {
        Ok(tile_name_Tile {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.side.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_side(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_tile::name::Slots>(&value)?;
        self.inner.side = value;
        Ok(())
    }
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
    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.turn.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_turn(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_tile::name::Slots>(&value)?;
        self.inner.turn = value;
        Ok(())
    }
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
    pub fn checked(&self) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::checked(self.inner.clone())
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    pub fn from_file(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    pub fn from_json(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    pub fn from_url(text: &str) -> Result<tile_name_Tile, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    pub fn of(recipe: &tile_Tile) -> Result<tile_name_Tile, JsValue> {
        let value = mrly_tile::name::Tile::of(&recipe.inner).map_err(hand::throw)?;
        Ok(tile_name_Tile { inner: value })
    }
    pub fn recipe(&self) -> Result<tile_Tile, JsValue> {
        let value = self.inner.recipe().map_err(hand::throw)?;
        Ok(tile_Tile { inner: value })
    }
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_tile::name::Tile as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct tile_variation_File {
    inner: mrly_tile::variation::File,
}

#[wasm_bindgen]
impl tile_variation_File {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_variation_File, JsValue> {
        Ok(tile_variation_File {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Result<tile_variation_File, JsValue> {
        let value = mrly_tile::variation::File::new(width, height);
        Ok(tile_variation_File { inner: value })
    }
}

#[wasm_bindgen]
pub struct tile_variation_Variation {
    inner: mrly_tile::variation::Variation,
}

#[wasm_bindgen]
impl tile_variation_Variation {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<tile_variation_Variation, JsValue> {
        Ok(tile_variation_Variation {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn primaries(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.primaries.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_primaries(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Option<Vec<mrly_core::paint::Ink>>>(&value)?;
        self.inner.primaries = value;
        Ok(())
    }
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
    #[wasm_bindgen(getter)]
    pub fn mask(&self) -> Result<Option<tile_Tile>, JsValue> {
        let value = self.inner.mask.clone();
        Ok(value.map(|inner| tile_Tile { inner }))
    }
    #[wasm_bindgen(setter)]
    pub fn set_mask(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::option_from_js(&value, |x1| {
            hand::from_js::<mrly_tile::Tile>(&hand::plain(x1)?)
        })?;
        self.inner.mask = value;
        Ok(())
    }
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
            hand::from_js::<mrly_tile::variation::File>(&hand::plain(x1)?)
        })?;
        self.inner.files = value;
        Ok(())
    }
    pub fn is_cover(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_cover();
        Ok(value)
    }
    pub fn is_prime(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_prime();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct tile_Group {}

#[wasm_bindgen]
impl tile_Group {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_tile::Group::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct tile_Parity {}

#[wasm_bindgen]
impl tile_Parity {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_tile::Parity::all();
        hand::to_js(&value)
    }
    pub fn keep(parity: JsValue, n: usize) -> Result<bool, JsValue> {
        let parity = hand::from_js::<mrly_tile::Parity>(&parity)?;
        let value = parity.keep(n);
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct tile_build_Config2d {}

#[wasm_bindgen]
impl tile_build_Config2d {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_tile::build::Config2d::default();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct tile_build_Config3d {}

#[wasm_bindgen]
impl tile_build_Config3d {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_tile::build::Config3d::default();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct tile_name_Slots {}

#[wasm_bindgen]
impl tile_name_Slots {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_tile::name::Slots::default();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct tile_recipe_Design {}

#[wasm_bindgen]
impl tile_recipe_Design {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_tile::recipe::Design::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct tile_variation_Config {}

#[wasm_bindgen]
impl tile_variation_Config {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_tile::variation::Config::default();
        hand::to_js(&value)
    }
}
