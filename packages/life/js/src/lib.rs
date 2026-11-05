#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_math::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn life_affine(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::affine(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_animate(seed: JsValue, config: &life_Config) -> Result<life_Life, JsValue> {
    let seed = hand::cell2d_from_js(&seed)?;
    let value = mrly_life::animate(&seed, &config.inner).map_err(hand::throw)?;
    Ok(life_Life { inner: value })
}

#[wasm_bindgen]
pub fn life_churn(grids: JsValue) -> Result<f64, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::churn(&grids);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_corner_bits(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::corner_bits(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_counts(
    seq: &life_Source,
    max_neighbors: usize,
    include_zeros: bool,
    include_ones: bool,
) -> Result<Vec<usize>, JsValue> {
    let value = mrly_life::counts(seq.inner, max_neighbors, include_zeros, include_ones)
        .map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn life_crop(grids: JsValue) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::crop(&grids).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

#[wasm_bindgen]
pub fn life_cube_orbit(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::cube_orbit(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_design_mask(
    dimension: usize,
    code: JsValue,
    number: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_life::design_mask(dimension, code, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn life_elementary_output(rule: u8, l: u8, c: u8, r: u8) -> Result<u8, JsValue> {
    let value = mrly_life::elementary::output(rule, l, c, r);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_entropy(grid: JsValue) -> Result<i64, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrly_life::entropy(&grid);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_frames(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::frames(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::image_to_js)
}

#[wasm_bindgen]
pub fn life_gasket(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrly_life::gasket(rule);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn life_genus(rule: u8) -> Result<String, JsValue> {
    let value = mrly_life::genus(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_heatmap(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::heatmap(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::image_to_js)
}

#[wasm_bindgen]
pub fn life_history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Result<JsValue, JsValue> {
    let value = mrly_life::history(row, rule, steps, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn life_lambda(rule: u8) -> Result<f64, JsValue> {
    let value = mrly_life::lambda(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_lattice_index(mask: JsValue) -> Result<usize, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_life::lattice_index(&mask);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_mask_offsets(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_life::mask_offsets(&mask);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn life_moore() -> Result<JsValue, JsValue> {
    let value = mrly_life::moore().map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

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
    let boundary = hand::from_js::<mrly_life::Boundary>(&boundary)?;
    let value =
        mrly_life::next_grid(&cell, birth, survive, &mask, boundary).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn life_npn_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::npn_class(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_outer_totalistic(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrly_life::outer_totalistic(rule);
    hand::option_to_js(value.as_ref(), |x1| {
        Ok(hand::tuple_to_js(&[
            hand::typed(&(x1.0)[..]),
            hand::typed(&(x1.1)[..]),
        ]))
    })
}

#[wasm_bindgen]
pub fn life_popcount(rule: u8) -> Result<u32, JsValue> {
    let value = mrly_life::popcount(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_recipe_grammar() -> Result<JsValue, JsValue> {
    let value = mrly_life::recipe::grammar();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn life_recipe_name(recipe: JsValue) -> Result<String, JsValue> {
    let recipe = hand::from_js::<mrly_life::recipe::Recipe>(&recipe)?;
    let value = mrly_life::recipe::name(&recipe);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_recipe_parse(name: &str) -> Result<JsValue, JsValue> {
    let value = mrly_life::recipe::parse(name).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn life_render_frame(grid: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrly_life::render::frame(&grid, scale).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn life_reversible(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::reversible(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_rule_degree(rule: u8) -> Result<i32, JsValue> {
    let value = mrly_life::rule_degree(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_rule_name(rule: u8) -> Result<String, JsValue> {
    let value = mrly_life::rule_name(rule).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn life_single_seed(rule: u8, steps: usize) -> Result<JsValue, JsValue> {
    let value = mrly_life::single_seed(rule, steps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn life_source_sequence(seq: &life_Source, limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_life::source::sequence(seq.inner, limit).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn life_step(row: &[u8], rule: u8, wrap: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::step(row, rule, wrap).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn life_surjective(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::surjective(rule);
    Ok(value)
}

#[wasm_bindgen]
pub fn life_tessellate(grids: JsValue, min_canvas: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::tessellate(&grids, min_canvas).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

#[wasm_bindgen]
pub fn life_wolfram_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::wolfram_class(rule);
    Ok(value)
}

#[wasm_bindgen]
pub struct life_Config {
    inner: mrly_life::Config,
}

#[wasm_bindgen]
impl life_Config {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Config, JsValue> {
        Ok(life_Config {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(getter)]
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_boundary(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_life::Boundary>(&value)?;
        self.inner.boundary = value;
        Ok(())
    }
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
    pub fn budget(&self) -> Result<usize, JsValue> {
        let value = self.inner.budget();
        Ok(value)
    }
    pub fn counts(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.counts().map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            hand::typed(&(value.0)[..]),
            hand::typed(&(value.1)[..]),
        ]))
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        mask: JsValue,
        birth: &life_Counts,
        survive: &life_Counts,
    ) -> Result<life_Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = mrly_life::Config::new(mask, birth.inner.clone(), survive.inner.clone());
        Ok(life_Config { inner: value })
    }
}

#[wasm_bindgen]
pub struct life_Counts {
    inner: mrly_life::Counts,
}

#[wasm_bindgen]
impl life_Counts {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Counts, JsValue> {
        Ok(life_Counts {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn drawn(seq: &life_Source, zeros: bool, ones: bool) -> Result<life_Counts, JsValue> {
        let value = mrly_life::Counts::drawn(seq.inner, zeros, ones);
        Ok(life_Counts { inner: value })
    }
    pub fn list(counts: Vec<usize>) -> Result<life_Counts, JsValue> {
        let value = mrly_life::Counts::list(counts);
        Ok(life_Counts { inner: value })
    }
    pub fn values(&self, budget: usize) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.values(budget).map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct life_Life {
    inner: mrly_life::Life,
}

#[wasm_bindgen]
impl life_Life {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Life, JsValue> {
        Ok(life_Life {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(getter)]
    pub fn fate(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.fate;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_fate(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_life::Fate>(&value)?;
        self.inner.fate = value;
        Ok(())
    }
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
    pub fn last(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.last();
        hand::option_to_js(value, hand::cell2d_to_js)
    }
}

#[wasm_bindgen]
pub struct life_Rule {
    inner: mrly_life::Rule,
}

#[wasm_bindgen]
impl life_Rule {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Rule, JsValue> {
        Ok(life_Rule {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary();
        hand::to_js(&value)
    }
    pub fn checked(&self) -> Result<life_Rule, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::checked(self.inner.clone())
            .map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    pub fn config(&self, mask: JsValue) -> Result<life_Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = self.inner.config(mask);
        Ok(life_Config { inner: value })
    }
    pub fn from_file(text: &str) -> Result<life_Rule, JsValue> {
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    pub fn from_json(text: &str) -> Result<life_Rule, JsValue> {
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    pub fn from_url(text: &str) -> Result<life_Rule, JsValue> {
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        birth: &life_Counts,
        survive: &life_Counts,
        wrap: bool,
    ) -> Result<life_Rule, JsValue> {
        let value = mrly_life::Rule::new(birth.inner.clone(), survive.inner.clone(), wrap);
        Ok(life_Rule { inner: value })
    }
    pub fn of(config: &life_Config) -> Result<life_Rule, JsValue> {
        let value = mrly_life::Rule::of(&config.inner);
        Ok(life_Rule { inner: value })
    }
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct life_Source {
    inner: mrly_life::Source,
}

#[wasm_bindgen]
impl life_Source {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<life_Source, JsValue> {
        Ok(life_Source {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::Source::all();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    pub fn designs() -> Result<JsValue, JsValue> {
        let value = mrly_life::Source::designs();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    pub fn is_random(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_random();
        Ok(value)
    }
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name();
        Ok(value)
    }
    pub fn numbers() -> Result<JsValue, JsValue> {
        let value = mrly_life::Source::numbers();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    pub fn oeis(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.oeis();
        hand::to_js(&value)
    }
    pub fn parse(name: &str) -> Result<life_Source, JsValue> {
        let value = mrly_life::Source::parse(name).map_err(hand::throw)?;
        Ok(life_Source { inner: value })
    }
    pub fn read(text: &str) -> Result<JsValue, JsValue> {
        let value = mrly_life::Source::read(text);
        hand::option_to_js(value.as_ref(), |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(life_Source { inner: x1.0 }),
                hand::to_js(&x1.1)?,
            ]))
        })
    }
}

#[wasm_bindgen]
pub struct life_recipe_Run {
    inner: mrly_life::recipe::Run,
}

#[wasm_bindgen]
impl life_recipe_Run {
    pub fn background(&self) -> Result<u8, JsValue> {
        let value = self.inner.background();
        Ok(value)
    }
    pub fn birth(&self) -> Result<Vec<u32>, JsValue> {
        let value = self.inner.birth();
        Ok(value)
    }
    pub fn budget(&self) -> Result<u32, JsValue> {
        let value = self.inner.budget();
        Ok(value)
    }
    pub fn cap(&self) -> Result<u32, JsValue> {
        let value = self.inner.cap();
        Ok(value)
    }
    pub fn flips(&self) -> Result<bool, JsValue> {
        let value = self.inner.flips();
        Ok(value)
    }
    pub fn frame(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.frame();
        hand::cell_to_js(&value)
    }
    pub fn generation(&self) -> Result<u32, JsValue> {
        let value = self.inner.generation();
        Ok(value)
    }
    pub fn mask(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mask();
        hand::tensor_to_js(&value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(recipe: JsValue) -> Result<life_recipe_Run, JsValue> {
        let recipe = hand::from_js::<mrly_life::recipe::Recipe>(&recipe)?;
        let value = mrly_life::recipe::Run::new(&recipe).map_err(hand::throw)?;
        Ok(life_recipe_Run { inner: value })
    }
    pub fn origin(&self) -> Result<Vec<i64>, JsValue> {
        let value = self.inner.origin();
        Ok(value.to_vec())
    }
    pub fn population(&self) -> Result<u32, JsValue> {
        let value = self.inner.population();
        Ok(value)
    }
    pub fn reach(&self) -> Result<u32, JsValue> {
        let value = self.inner.reach();
        Ok(value)
    }
    pub fn side(&self) -> Result<u32, JsValue> {
        let value = self.inner.side();
        Ok(value)
    }
    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        let value = self.inner.step();
        hand::to_js(&value)
    }
    pub fn stop(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.stop();
        hand::to_js(&value)
    }
    pub fn survive(&self) -> Result<Vec<u32>, JsValue> {
        let value = self.inner.survive();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct life_Boundary {}

#[wasm_bindgen]
impl life_Boundary {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::Boundary::all();
        hand::to_js(&value)
    }
    pub fn wrap(boundary: JsValue) -> Result<bool, JsValue> {
        let boundary = hand::from_js::<mrly_life::Boundary>(&boundary)?;
        let value = boundary.wrap();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct life_Fate {}

#[wasm_bindgen]
impl life_Fate {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::Fate::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct life_recipe_Mode {}

#[wasm_bindgen]
impl life_recipe_Mode {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::recipe::Mode::all();
        hand::to_js(&value)
    }
}
