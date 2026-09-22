#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Returns whether a rule is affine, its algebraic degree at most one.
#[wasm_bindgen]
pub fn affine(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::affine(rule);
    Ok(value)
}

/// Runs a seed under a config until it fixes, loops or times out, recording every generation.
#[wasm_bindgen]
pub fn animate(seed: JsValue, config: &Config) -> Result<Life, JsValue> {
    let seed = hand::cell2d_from_js(&seed)?;
    let value = mrlyrs::life::animate(&seed, &config.inner).map_err(hand::throw)?;
    Ok(Life { inner: value })
}

/// Returns the mean fraction of sites changed between consecutive grids.
#[wasm_bindgen]
pub fn churn(grids: JsValue) -> Result<f64, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::churn(&grids);
    Ok(value)
}

/// Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.
#[wasm_bindgen]
pub fn corner_bits(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::corner_bits(rule);
    Ok(value)
}

/// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
#[wasm_bindgen]
pub fn counts(seq: &Source, max_neighbors: usize, include_zeros: bool, include_ones: bool) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::life::counts(seq.inner, max_neighbors, include_zeros, include_ones).map_err(hand::throw)?;
    Ok(value)
}

/// Crops a frame sequence to the centred square bounding every cell ever alive.
#[wasm_bindgen]
pub fn crop(grids: JsValue) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::crop(&grids).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.
#[wasm_bindgen]
pub fn cube_orbit(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::cube_orbit(rule);
    Ok(value)
}

/// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker
#[wasm_bindgen]
pub fn design_mask(dimension: usize, code: JsValue, number: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::life::design_mask(dimension, code, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.
#[wasm_bindgen]
pub fn elementary_output(rule: u8, l: u8, c: u8, r: u8) -> Result<u8, JsValue> {
    let value = mrlyrs::life::elementary::output(rule, l, c, r);
    Ok(value)
}

/// Returns the grid's binary Shannon entropy in millibits.
#[wasm_bindgen]
pub fn entropy(grid: JsValue) -> Result<i64, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrlyrs::life::entropy(&grid);
    Ok(value)
}

/// Renders grids to white-on-black PNG bytes at a pixel scale.
#[wasm_bindgen]
pub fn frames(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::frames(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the base-2 plane design a rule's single seed draws, or None when it draws none.
#[wasm_bindgen]
pub fn gasket(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::gasket(rule);
    hand::to_js(&value)
}

/// Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.
#[wasm_bindgen]
pub fn genus(rule: u8) -> Result<String, JsValue> {
    let value = mrlyrs::life::genus(rule);
    Ok(value)
}

/// Renders a whole run's cumulative-visit heatmap frames with the heat ramp.
#[wasm_bindgen]
pub fn heatmap(grids: JsValue, scale: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::heatmap(&grids, scale).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.
#[wasm_bindgen]
pub fn history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::history(row, rule, steps, wrap).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns Langton's lambda, the popcount over eight.
#[wasm_bindgen]
pub fn lambda(rule: u8) -> Result<f64, JsValue> {
    let value = mrlyrs::life::lambda(rule);
    Ok(value)
}

/// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
#[wasm_bindgen]
pub fn lattice_index(mask: JsValue) -> Result<usize, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::life::lattice_index(&mask);
    Ok(value)
}

/// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
#[wasm_bindgen]
pub fn mask_offsets(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::life::mask_offsets(&mask);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Builds the 3 by 3 Moore mask, every site on but the center.
#[wasm_bindgen]
pub fn moore() -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::moore().map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Renders grids into one looping black-on-white gif, the delay in hundredths of a second.
#[wasm_bindgen]
pub fn movie(grids: JsValue, scale: usize, delay: usize) -> Result<Vec<u8>, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::movie(&grids, scale, delay).map_err(hand::throw)?;
    Ok(value)
}

/// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
#[wasm_bindgen]
pub fn next_grid(cell: JsValue, birth: &[usize], survive: &[usize], mask: JsValue, boundary: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let boundary = hand::from_js::<mrlyrs::life::Boundary>(&boundary)?;
    let value = mrlyrs::life::next_grid(&cell, birth, survive, &mask, boundary).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.
#[wasm_bindgen]
pub fn npn_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::npn_class(rule);
    Ok(value)
}

/// Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.
#[wasm_bindgen]
pub fn outer_totalistic(rule: u8) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::outer_totalistic(rule);
    hand::option_to_js(value.as_ref(), |x1| Ok(hand::tuple_to_js(&[hand::typed(&(x1.0)[..]), hand::typed(&(x1.1)[..])])))
}

/// Returns the count of neighbourhoods a rule sends to one.
#[wasm_bindgen]
pub fn popcount(rule: u8) -> Result<u32, JsValue> {
    let value = mrlyrs::life::popcount(rule);
    Ok(value)
}

/// Renders one grid to white-on-black PNG bytes at a pixel scale.
#[wasm_bindgen]
pub fn render_frame(grid: JsValue, scale: usize) -> Result<Vec<u8>, JsValue> {
    let grid = hand::cell2d_from_js(&grid)?;
    let value = mrlyrs::life::render::frame(&grid, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.
#[wasm_bindgen]
pub fn reversible(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::reversible(rule);
    Ok(value)
}

/// Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.
#[wasm_bindgen]
pub fn rule_degree(rule: u8) -> Result<i32, JsValue> {
    let value = mrlyrs::life::rule_degree(rule);
    Ok(value)
}

/// Returns the design name a rule carries, `bang dim 3, code <rule>`.
#[wasm_bindgen]
pub fn rule_name(rule: u8) -> Result<String, JsValue> {
    let value = mrlyrs::life::rule_name(rule).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.
#[wasm_bindgen]
pub fn single_seed(rule: u8, steps: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::life::single_seed(rule, steps).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Generates the sequence's values up to the limit.
#[wasm_bindgen]
pub fn source_sequence(seq: &Source, limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::life::source::sequence(seq.inner, limit).map_err(hand::throw)?;
    Ok(value)
}

/// Advances one row one generation, a constant-0 boundary unless the edges wrap.
#[wasm_bindgen]
pub fn step(row: &[u8], rule: u8, wrap: bool) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::step(row, rule, wrap).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.
#[wasm_bindgen]
pub fn surjective(rule: u8) -> Result<bool, JsValue> {
    let value = mrlyrs::life::surjective(rule);
    Ok(value)
}

/// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
#[wasm_bindgen]
pub fn tessellate(grids: JsValue, min_canvas: usize) -> Result<JsValue, JsValue> {
    let grids = hand::list_from_js(&grids, hand::cell2d_from_js)?;
    let value = mrlyrs::life::tessellate(&grids, min_canvas).map_err(hand::throw)?;
    hand::list_to_js(&value, hand::cell2d_to_js)
}

/// Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.
#[wasm_bindgen]
pub fn wolfram_class(rule: u8) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::life::wolfram_class(rule);
    Ok(value)
}

/// The rulebook of a life run.
#[wasm_bindgen]
pub struct Config {
    inner: mrlyrs::life::Config,
}

#[wasm_bindgen]
impl Config {
    /// Reads the Config from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Config, JsValue> {
        Ok(Config { inner: hand::from_js(&data)? })
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
    /// The neighbor counts that create a cell.
    #[wasm_bindgen(getter)]
    pub fn birth(&self) -> Result<Counts, JsValue> {
        let value = self.inner.birth.clone();
        Ok(Counts { inner: value })
    }
    /// The neighbor counts that keep a cell.
    #[wasm_bindgen(getter)]
    pub fn survive(&self) -> Result<Counts, JsValue> {
        let value = self.inner.survive.clone();
        Ok(Counts { inner: value })
    }
    /// The edge policy.
    #[wasm_bindgen(getter)]
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary;
        hand::to_js(&value)
    }
    /// The generation cap.
    #[wasm_bindgen(getter)]
    pub fn max_generations(&self) -> Result<usize, JsValue> {
        let value = self.inner.max_generations;
        Ok(value)
    }
    /// The tiling factor applied to the seed.
    #[wasm_bindgen(getter)]
    pub fn grid_size(&self) -> Result<usize, JsValue> {
        let value = self.inner.grid_size;
        Ok(value)
    }
    /// The dead border added around the seed.
    #[wasm_bindgen(getter)]
    pub fn padding(&self) -> Result<usize, JsValue> {
        let value = self.inner.padding;
        Ok(value)
    }
    /// Returns the largest neighbor count the mask can reach.
    pub fn budget(&self) -> Result<usize, JsValue> {
        let value = self.inner.budget();
        Ok(value)
    }
    /// Resolves the birth and survive counts against the mask's budget.
    pub fn counts(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.counts().map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[hand::typed(&(value.0)[..]), hand::typed(&(value.1)[..])]))
    }
    /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
    #[wasm_bindgen(constructor)]
    pub fn new(mask: JsValue, birth: &Counts, survive: &Counts) -> Result<Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = mrlyrs::life::Config::new(mask, birth.inner.clone(), survive.inner.clone());
        Ok(Config { inner: value })
    }
}

/// The neighbor counts one side of a rule fires on.
#[wasm_bindgen]
pub struct Counts {
    inner: mrlyrs::life::Counts,
}

#[wasm_bindgen]
impl Counts {
    /// Reads the Counts from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Counts, JsValue> {
        Ok(Counts { inner: hand::from_js(&data)? })
    }
    /// Writes the Counts as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Builds the counts a sequence lays down, keeping zeros and ones on request.
    pub fn drawn(seq: &Source, zeros: bool, ones: bool) -> Result<Counts, JsValue> {
        let value = mrlyrs::life::Counts::drawn(seq.inner, zeros, ones);
        Ok(Counts { inner: value })
    }
    /// Spells the counts outright.
    pub fn list(counts: Vec<usize>) -> Result<Counts, JsValue> {
        let value = mrlyrs::life::Counts::list(counts);
        Ok(Counts { inner: value })
    }
    /// Returns the counts, a drawn side resolved against the mask's neighbor budget.
    pub fn values(&self, budget: usize) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.values(budget).map_err(hand::throw)?;
        Ok(value)
    }
}

/// The recorded run of one seed.
#[wasm_bindgen]
pub struct Life {
    inner: mrlyrs::life::Life,
}

#[wasm_bindgen]
impl Life {
    /// Reads the Life from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Life, JsValue> {
        Ok(Life { inner: hand::from_js(&data)? })
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
    /// The run's ending.
    #[wasm_bindgen(getter)]
    pub fn fate(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.fate;
        hand::to_js(&value)
    }
    /// The number of recorded generations.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> Result<usize, JsValue> {
        let value = self.inner.count;
        Ok(value)
    }
    /// The cycle length when the fate is a loop, else zero.
    #[wasm_bindgen(getter)]
    pub fn loop_length(&self) -> Result<usize, JsValue> {
        let value = self.inner.loop_length;
        Ok(value)
    }
    /// Returns the final grid, or None when the run is empty.
    pub fn last(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.last();
        hand::option_to_js(value, hand::cell2d_to_js)
    }
}

/// A life rule: the birth and survival counts and whether the edge wraps.
#[wasm_bindgen]
pub struct Rule {
    inner: mrlyrs::life::Rule,
}

#[wasm_bindgen]
impl Rule {
    /// Reads the Rule from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Rule, JsValue> {
        Ok(Rule { inner: hand::from_js(&data)? })
    }
    /// Writes the Rule as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The neighbor counts that create a cell, listed or drawn from a sequence.
    #[wasm_bindgen(getter)]
    pub fn birth(&self) -> Result<Counts, JsValue> {
        let value = self.inner.birth.clone();
        Ok(Counts { inner: value })
    }
    /// The neighbor counts that keep a cell, listed or drawn from a sequence.
    #[wasm_bindgen(getter)]
    pub fn survive(&self) -> Result<Counts, JsValue> {
        let value = self.inner.survive.clone();
        Ok(Counts { inner: value })
    }
    /// Whether the edge wraps, false unless said.
    #[wasm_bindgen(getter)]
    pub fn wrap(&self) -> Result<bool, JsValue> {
        let value = self.inner.wrap;
        Ok(value)
    }
    /// Returns the edge policy the rule runs under.
    pub fn boundary(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.boundary();
        hand::to_js(&value)
    }
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    pub fn checked(&self) -> Result<Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::checked(self.inner.clone()).map_err(hand::throw)?;
        Ok(Rule { inner: value })
    }
    /// Builds a life config running this rule over a neighborhood mask.
    pub fn config(&self, mask: JsValue) -> Result<Config, JsValue> {
        let mask = hand::cell2d_from_js(&mask)?;
        let value = self.inner.config(mask);
        Ok(Config { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(Rule { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(Rule { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<Rule, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(Rule { inner: value })
    }
    /// Builds a rule from its counts and edge policy, listed counts folded to a sorted set.
    #[wasm_bindgen(constructor)]
    pub fn new(birth: &Counts, survive: &Counts, wrap: bool) -> Result<Rule, JsValue> {
        let value = mrlyrs::life::Rule::new(birth.inner.clone(), survive.inner.clone(), wrap);
        Ok(Rule { inner: value })
    }
    /// Reads the rule out of a life config.
    pub fn of(config: &Config) -> Result<Rule, JsValue> {
        let value = mrlyrs::life::Rule::of(&config.inner);
        Ok(Rule { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_file(&self.inner).map_err(hand::throw)?;
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
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_mrly(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_url(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
}

/// A named source of neighbor-count values.
#[wasm_bindgen]
pub struct Source {
    inner: mrlyrs::life::Source,
}

#[wasm_bindgen]
impl Source {
    /// Reads the Source from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<Source, JsValue> {
        Ok(Source { inner: hand::from_js(&data)? })
    }
    /// Writes the Source as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns every fixed sequence, the seeded and coded families excluded.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::all();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(Source { inner: *x1 })))
    }
    /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
    pub fn designs() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::designs();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(Source { inner: *x1 })))
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
        hand::list_to_js(&value, |x1| Ok(JsValue::from(Source { inner: *x1 })))
    }
    /// Returns the sequence's OEIS id, or None off the encyclopedia.
    pub fn oeis(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.oeis();
        hand::to_js(&value)
    }
    /// Parses a sequence name back to its source.
    pub fn parse(name: &str) -> Result<Source, JsValue> {
        let value = mrlyrs::life::Source::parse(name).map_err(hand::throw)?;
        Ok(Source { inner: value })
    }
    /// Reads a canonical name off the front of the text, returning the tail left over.
    pub fn read(text: &str) -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Source::read(text);
        hand::option_to_js(value.as_ref(), |x1| Ok(hand::tuple_to_js(&[JsValue::from(Source { inner: x1.0 }), hand::to_js(&x1.1)?])))
    }
}

/// The edge policy of a life grid.
#[wasm_bindgen]
pub struct Boundary {}

#[wasm_bindgen]
impl Boundary {
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
pub struct Fate {}

#[wasm_bindgen]
impl Fate {
    /// Returns every Fate in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::life::Fate::all();
        hand::to_js(&value)
    }
}
