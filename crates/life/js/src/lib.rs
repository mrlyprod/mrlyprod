#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_math_js_hand as hand;
use wasm_bindgen::prelude::*;

/// Returns whether a rule is affine, its algebraic degree at most one.
#[wasm_bindgen]
pub fn life_affine(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::affine(rule);
    Ok(value)
}

/// Runs a seed under a config until it fixes, loops or times out, recording every generation.
#[wasm_bindgen]
pub fn life_animate(seed: JsValue, config: &life_Config) -> Result<life_Life, JsValue> {
    let seed = hand::cell2d_from_js(&seed)?;
    let value = mrly_life::animate(&seed, &config.inner).map_err(hand::throw)?;
    Ok(life_Life { inner: value })
}

/// Returns the mean fraction of sites changed between consecutive grids.
#[wasm_bindgen]
pub fn life_churn(grids: JsValue) -> Result<f64, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::churn(&grids);
    Ok(value)
}

/// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
#[wasm_bindgen]
pub fn life_corner_bits(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::corner_bits(rule);
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
    let value = mrly_life::counts(seq.inner, max_neighbors, include_zeros, include_ones)
        .map_err(hand::throw)?;
    Ok(value)
}

/// Crops a frame sequence to the centred square bounding every cell ever alive.
#[wasm_bindgen]
pub fn life_crop(grids: JsValue) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::crop(&grids).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
#[wasm_bindgen]
pub fn life_cube_orbit(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::cube_orbit(rule);
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
    let value = mrly_life::design_mask(dimension, code, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.
#[wasm_bindgen]
pub fn life_elementary_output(rule: u8, l: u8, c: u8, r: u8) -> Result<u8, JsValue> {
    let value = mrly_life::elementary::output(rule, l, c, r);
    Ok(value)
}

/// Returns the grid's binary Shannon entropy in millibits.
#[wasm_bindgen]
pub fn life_entropy(grid: JsValue) -> Result<i64, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrly_life::entropy(&grid);
    Ok(value)
}

/// Renders grids to white-on-black images at a pixel scale.
#[wasm_bindgen]
pub fn life_frames(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::frames(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::image_to_js)
}

/// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
#[wasm_bindgen]
pub fn life_gasket(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrly_life::gasket(rule);
    hand::to_js(&value)
}

/// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
#[wasm_bindgen]
pub fn life_genus(rule: u8) -> Result<String, JsValue> {
    let value = mrly_life::genus(rule);
    Ok(value)
}

/// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
#[wasm_bindgen]
pub fn life_heatmap(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::heatmap(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::image_to_js)
}

/// Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.
#[wasm_bindgen]
pub fn life_history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Result<JsValue, JsValue> {
    let value = mrly_life::history(row, rule, steps, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns Langton's lambda, the popcount over eight.
#[wasm_bindgen]
pub fn life_lambda(rule: u8) -> Result<f64, JsValue> {
    let value = mrly_life::lambda(rule);
    Ok(value)
}

/// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
#[wasm_bindgen]
pub fn life_lattice_index(mask: JsValue) -> Result<usize, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_life::lattice_index(&mask);
    Ok(value)
}

/// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
#[wasm_bindgen]
pub fn life_mask_offsets(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_life::mask_offsets(&mask);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Builds the 3 by 3 Moore mask, every site on but the center.
#[wasm_bindgen]
pub fn life_moore() -> Result<JsValue, JsValue> {
    let value = mrly_life::moore().map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
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
    let boundary = hand::from_js::<mrly_life::Boundary>(&boundary)?;
    let value =
        mrly_life::next_grid(&cell, birth, survive, &mask, boundary).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
#[wasm_bindgen]
pub fn life_npn_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::npn_class(rule);
    Ok(value)
}

/// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
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

/// Returns the count of neighbourhoods a rule sends to one.
#[wasm_bindgen]
pub fn life_popcount(rule: u8) -> Result<u32, JsValue> {
    let value = mrly_life::popcount(rule);
    Ok(value)
}

/// Lists what every knob of a recipe may offer, in grammar order.
#[wasm_bindgen]
pub fn life_recipe_grammar() -> Result<JsValue, JsValue> {
    let value = mrly_life::recipe::grammar();
    hand::to_js(&value)
}

/// Writes a recipe as its name, the one string that regenerates it.
#[wasm_bindgen]
pub fn life_recipe_name(recipe: JsValue) -> Result<String, JsValue> {
    let recipe = hand::from_js::<mrly_life::recipe::Recipe>(&recipe)?;
    let value = mrly_life::recipe::name(&recipe);
    Ok(value)
}

/// Reads a recipe out of its name.
#[wasm_bindgen]
pub fn life_recipe_parse(name: &str) -> Result<JsValue, JsValue> {
    let value = mrly_life::recipe::parse(name).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Renders one grid to a white-on-black image at a pixel scale.
#[wasm_bindgen]
pub fn life_render_frame(grid: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrly_life::render::frame(&grid, scale).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

/// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
#[wasm_bindgen]
pub fn life_reversible(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::reversible(rule);
    Ok(value)
}

/// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
#[wasm_bindgen]
pub fn life_rule_degree(rule: u8) -> Result<i32, JsValue> {
    let value = mrly_life::rule_degree(rule);
    Ok(value)
}

/// Returns the design name a rule carries, `bang dim 3, code <rule>`.
#[wasm_bindgen]
pub fn life_rule_name(rule: u8) -> Result<String, JsValue> {
    let value = mrly_life::rule_name(rule).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
#[wasm_bindgen]
pub fn life_single_seed(rule: u8, steps: usize) -> Result<JsValue, JsValue> {
    let value = mrly_life::single_seed(rule, steps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Generates the sequence's values up to the limit.
#[wasm_bindgen]
pub fn life_source_sequence(seq: &life_Source, limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_life::source::sequence(seq.inner, limit).map_err(hand::throw)?;
    Ok(value)
}

/// Advances one row one generation, a constant-0 boundary unless the edges wrap.
#[wasm_bindgen]
pub fn life_step(row: &[u8], rule: u8, wrap: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::step(row, rule, wrap).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
#[wasm_bindgen]
pub fn life_surjective(rule: u8) -> Result<bool, JsValue> {
    let value = mrly_life::surjective(rule);
    Ok(value)
}

/// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
#[wasm_bindgen]
pub fn life_tessellate(grids: JsValue, min_canvas: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrly_life::tessellate(&grids, min_canvas).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
#[wasm_bindgen]
pub fn life_wolfram_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrly_life::wolfram_class(rule);
    Ok(value)
}

/// The rulebook of a life run.
#[wasm_bindgen]
pub struct life_Config {
    inner: mrly_life::Config,
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
        let value = hand::from_js::<mrly_life::Boundary>(&value)?;
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
        let value = mrly_life::Config::new(mask, birth.inner.clone(), survive.inner.clone());
        Ok(life_Config { inner: value })
    }
}

/// The neighbor counts one side of a rule fires on.
#[wasm_bindgen]
pub struct life_Counts {
    inner: mrly_life::Counts,
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
        let value = mrly_life::Counts::drawn(seq.inner, zeros, ones);
        Ok(life_Counts { inner: value })
    }
    /// Spells the counts outright.
    pub fn list(counts: Vec<usize>) -> Result<life_Counts, JsValue> {
        let value = mrly_life::Counts::list(counts);
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
    inner: mrly_life::Life,
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
        let value = hand::from_js::<mrly_life::Fate>(&value)?;
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
    inner: mrly_life::Rule,
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
        let value = <mrly_life::Rule as mrly_core::name::Named>::checked(self.inner.clone())
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
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<life_Rule, JsValue> {
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<life_Rule, JsValue> {
        let value =
            <mrly_life::Rule as mrly_core::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(life_Rule { inner: value })
    }
    /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
    #[wasm_bindgen(constructor)]
    pub fn new(
        birth: &life_Counts,
        survive: &life_Counts,
        wrap: bool,
    ) -> Result<life_Rule, JsValue> {
        let value = mrly_life::Rule::new(birth.inner.clone(), survive.inner.clone(), wrap);
        Ok(life_Rule { inner: value })
    }
    /// Reads the rule out of a life config.
    pub fn of(config: &life_Config) -> Result<life_Rule, JsValue> {
        let value = mrly_life::Rule::of(&config.inner);
        Ok(life_Rule { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_life::Rule as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

/// A named source of neighbor-count values.
#[wasm_bindgen]
pub struct life_Source {
    inner: mrly_life::Source,
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
        let value = mrly_life::Source::all();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
    pub fn designs() -> Result<JsValue, JsValue> {
        let value = mrly_life::Source::designs();
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
        let value = mrly_life::Source::numbers();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(life_Source { inner: *x1 })))
    }
    /// Returns the sequence's OEIS id, or None off the encyclopedia.
    pub fn oeis(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.oeis();
        hand::to_js(&value)
    }
    /// Parses a sequence name back to its source.
    pub fn parse(name: &str) -> Result<life_Source, JsValue> {
        let value = mrly_life::Source::parse(name).map_err(hand::throw)?;
        Ok(life_Source { inner: value })
    }
    /// Reads a canonical name off the front of the text, returning the tail left over.
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

/// A recipe's run, stepped one generation at a time until its stop.
#[wasm_bindgen]
pub struct life_recipe_Run {
    inner: mrly_life::recipe::Run,
}

#[wasm_bindgen]
impl life_recipe_Run {
    /// Returns the background the frame sits on, 1 once a flipping rule has turned it.
    pub fn background(&self) -> Result<u8, JsValue> {
        let value = self.inner.background();
        Ok(value)
    }
    /// Returns the birth counts drawn against the budget.
    pub fn birth(&self) -> Result<Vec<u32>, JsValue> {
        let value = self.inner.birth();
        Ok(value)
    }
    /// Returns the mask's budget, the most neighbours a cell can count.
    pub fn budget(&self) -> Result<u32, JsValue> {
        let value = self.inner.budget();
        Ok(value)
    }
    /// Returns the canvas side: an island's cap, at least its seed board, or a wallpaper's torus.
    pub fn cap(&self) -> Result<u32, JsValue> {
        let value = self.inner.cap();
        Ok(value)
    }
    /// Returns whether the rule flips its background, birth holding zero.
    pub fn flips(&self) -> Result<bool, JsValue> {
        let value = self.inner.flips();
        Ok(value)
    }
    /// Returns the live frame: the cropped island pattern, or the whole torus.
    pub fn frame(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.frame();
        hand::cell_to_js(&value)
    }
    /// Returns the generation reached.
    pub fn generation(&self) -> Result<u32, JsValue> {
        let value = self.inner.generation();
        Ok(value)
    }
    /// Returns the mask as a square tensor of bytes, its centre popped.
    pub fn mask(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.mask();
        hand::tensor_to_js(&value)
    }
    /// Sets a recipe up at generation zero: the tiled seed, the mask's kernel, the rule drawn against its budget.
    #[wasm_bindgen(constructor)]
    pub fn new(recipe: JsValue) -> Result<life_recipe_Run, JsValue> {
        let recipe = hand::from_js::<mrly_life::recipe::Recipe>(&recipe)?;
        let value = mrly_life::recipe::Run::new(&recipe).map_err(hand::throw)?;
        Ok(life_recipe_Run { inner: value })
    }
    /// Returns the row and column of the frame's top-left corner, the seed's top-left corner being the origin.
    pub fn origin(&self) -> Result<Vec<i64>, JsValue> {
        let value = self.inner.origin();
        Ok(value.to_vec())
    }
    /// Returns the live cells of the frame.
    pub fn population(&self) -> Result<u32, JsValue> {
        let value = self.inner.population();
        Ok(value)
    }
    /// Returns the mask's reach, half its side.
    pub fn reach(&self) -> Result<u32, JsValue> {
        let value = self.inner.reach();
        Ok(value)
    }
    /// Returns the side of the seed board, the seed's side to its level times its tiling.
    pub fn side(&self) -> Result<u32, JsValue> {
        let value = self.inner.side();
        Ok(value)
    }
    /// Advances one generation and returns the stop the moment the run ends; after a stop it returns the same stop and changes nothing.
    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        let value = self.inner.step();
        hand::to_js(&value)
    }
    /// Returns the stop once the run has ended.
    pub fn stop(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.stop();
        hand::to_js(&value)
    }
    /// Returns the survive counts drawn against the budget.
    pub fn survive(&self) -> Result<Vec<u32>, JsValue> {
        let value = self.inner.survive();
        Ok(value)
    }
}

/// The edge policy of a life grid.
#[wasm_bindgen]
pub struct life_Boundary {}

#[wasm_bindgen]
impl life_Boundary {
    /// Returns every Boundary in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::Boundary::all();
        hand::to_js(&value)
    }
    /// Returns whether the edges wrap.
    pub fn wrap(boundary: JsValue) -> Result<bool, JsValue> {
        let boundary = hand::from_js::<mrly_life::Boundary>(&boundary)?;
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
        let value = mrly_life::Fate::all();
        hand::to_js(&value)
    }
}

/// The mode of a run.
#[wasm_bindgen]
pub struct life_recipe_Mode {}

#[wasm_bindgen]
impl life_recipe_Mode {
    /// Returns every Mode in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_life::recipe::Mode::all();
        hand::to_js(&value)
    }
}
