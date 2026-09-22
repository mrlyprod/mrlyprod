#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Builds an n by n carpet, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_carpet_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n carpet, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_carpet_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a carpet of the given side at any rank, on where at most one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_carpet_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::carpet_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n dust, on where both coordinates are even.
#[wasm_bindgen]
pub fn atoms_dust_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n dust, on where all three coordinates are even.
#[wasm_bindgen]
pub fn atoms_dust_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a dust of the given side at any rank, on where every coordinate is even.
#[wasm_bindgen]
pub fn atoms_dust_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::dust_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n line, free on axis 1, on along the odd rows.
#[wasm_bindgen]
pub fn atoms_hline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::hline_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tree, free on axis 1, on along the even rows.
#[wasm_bindgen]
pub fn atoms_htree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::htree_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.
#[wasm_bindgen]
pub fn atoms_line_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::line_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

/// Builds an n by n net, on where at least one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_net_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n net, on where at least two coordinates are odd.
#[wasm_bindgen]
pub fn atoms_net_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.
#[wasm_bindgen]
pub fn atoms_net_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::net_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.
#[wasm_bindgen]
pub fn atoms_noise_2d(n: usize, density: f64, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::noise_2d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.
#[wasm_bindgen]
pub fn atoms_noise_3d(n: usize, density: f64, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::noise_3d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor of ones.
#[wasm_bindgen]
pub fn atoms_ones_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ones_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor of ones.
#[wasm_bindgen]
pub fn atoms_ones_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ones_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n point, on where both coordinates are odd.
#[wasm_bindgen]
pub fn atoms_point_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n point, on where all three coordinates are odd.
#[wasm_bindgen]
pub fn atoms_point_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a point of the given side at any rank, on where every coordinate is odd.
#[wasm_bindgen]
pub fn atoms_point_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::point_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n star, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_star_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n star, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_star_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a star of the given side at any rank, on where exactly one coordinate is odd.
#[wasm_bindgen]
pub fn atoms_star_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::star_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.
#[wasm_bindgen]
pub fn atoms_tree_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::tree_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

/// Builds an n by n line, free on axis 0, on along the odd columns.
#[wasm_bindgen]
pub fn atoms_vline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::vline_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n void, on where both coordinates share one parity.
#[wasm_bindgen]
pub fn atoms_void_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n void, on where all three coordinates share one parity.
#[wasm_bindgen]
pub fn atoms_void_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds a void of the given side at any rank, on where every coordinate shares one parity.
#[wasm_bindgen]
pub fn atoms_void_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::void_nd(n, rank);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tree, free on axis 0, on along the even columns.
#[wasm_bindgen]
pub fn atoms_vtree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::vtree_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 0, its rods running along x.
#[wasm_bindgen]
pub fn atoms_xline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::xline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 0, its beams running along x.
#[wasm_bindgen]
pub fn atoms_xtree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::xtree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 1, its rods running along y.
#[wasm_bindgen]
pub fn atoms_yline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::yline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 1, its beams running along y.
#[wasm_bindgen]
pub fn atoms_ytree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ytree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n tensor of zeros.
#[wasm_bindgen]
pub fn atoms_zeros_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zeros_2d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tensor of zeros.
#[wasm_bindgen]
pub fn atoms_zeros_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zeros_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n line, free on axis 2, its rods running along z.
#[wasm_bindgen]
pub fn atoms_zline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::zline_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds an n by n by n tree, free on axis 2, its beams running along z.
#[wasm_bindgen]
pub fn atoms_ztree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::atoms::ztree_3d(n);
    hand::tensor_to_js(&value)
}

/// Builds the universe of a dimension.
#[wasm_bindgen]
pub fn bang_bang(dimension: usize) -> Result<bang_Universe, JsValue> {
    let value = mrlyrs::math::bang::bang(dimension).map_err(hand::throw)?;
    Ok(bang_Universe { inner: value })
}

/// Returns the distinct rotation and reflection maps of a base-q axis.
#[wasm_bindgen]
pub fn bang_baseq_axis_maps(base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::axis_maps(base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the distinct one-dimensional design counts for bases 1 through max_base.
#[wasm_bindgen]
pub fn bang_baseq_bracelets(max_base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::bracelets(max_base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the least code of the design's orbit.
#[wasm_bindgen]
pub fn bang_baseq_canonical(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::canonical(&group, code).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Carries a code through one group element.
#[wasm_bindgen]
pub fn bang_baseq_carry(element: &[usize], code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::carry(element, code);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Returns the fill-class counts for dimensions 1 through max_dimension.
#[wasm_bindgen]
pub fn bang_baseq_class_sequence(max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::class_sequence(max_dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.
#[wasm_bindgen]
pub fn bang_baseq_classes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::classes(dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Counts base-q designs distinct under symmetry.
#[wasm_bindgen]
pub fn bang_baseq_distinct_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::distinct_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the collapsed fill count at an even side number.
#[wasm_bindgen]
pub fn bang_baseq_even_fill_is_balanced(number: usize, dimension: usize, popcount: JsValue) -> Result<JsValue, JsValue> {
    let popcount = hand::u128_from_js(&popcount)?;
    let value = mrlyrs::math::bang::baseq::even_fill_is_balanced(number, dimension, popcount).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled-cell count of a binary design at a side number, folded from its filled corners.
#[wasm_bindgen]
pub fn bang_baseq_fill_from_corners(filled: JsValue, number: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::bang::baseq::fill_from_corners(&filled, number, dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.
#[wasm_bindgen]
pub fn bang_baseq_group(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::group(base, dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the symmetry group order counted from the enumerated axis maps.
#[wasm_bindgen]
pub fn bang_baseq_group_order(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns every code a design reaches under the group.
#[wasm_bindgen]
pub fn bang_baseq_orbit(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::baseq::orbit(&group, code);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

/// Returns the closed-form group order the axis-map count must match.
#[wasm_bindgen]
pub fn bang_baseq_predicted_group_order(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::predicted_group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.
#[wasm_bindgen]
pub fn bang_baseq_representatives(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::representatives(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from_str(&hand::code_to_js(x1.0)), hand::to_js(&x1.1)?])))
}

/// Returns the distinct-design counts for dimensions 1 through max_dimension.
#[wasm_bindgen]
pub fn bang_baseq_sequence(base: usize, max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::sequence(base, max_dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the raw design count before symmetry, two to the number of cells.
#[wasm_bindgen]
pub fn bang_baseq_total_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::baseq::total_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the anti designs for a dimension.
#[wasm_bindgen]
pub fn bang_catalog_antis(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::antis(dimension);
    hand::to_js(&value)
}

/// Returns the bitmask the code carries.
#[wasm_bindgen]
pub fn bang_code_get(code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = code.get();
    Ok(JsValue::from_str(&value.to_string()))
}

/// Unpacks a code into its filled residue corners.
#[wasm_bindgen]
pub fn bang_code_to_corners(code: JsValue, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::code_to_corners(code, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the binary corners of a dimension in code order.
#[wasm_bindgen]
pub fn bang_corners(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::corners(dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Packs filled residue corners back into their code.
#[wasm_bindgen]
pub fn bang_corners_to_code(filled: JsValue, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::bang::corners_to_code(&filled, dimension, base);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Renders a coded design to a tensor at its side number, dimension, base and fractal level.
#[wasm_bindgen]
pub fn bang_factory_create(code: JsValue, number: usize, dimension: usize, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::factory::create(code, number, dimension, base, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Renders a design straight from its filled residue corners.
#[wasm_bindgen]
pub fn bang_factory_create_from_corners(filled: JsValue, number: usize, dimension: usize, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::bang::factory::create_from_corners(&filled, number, dimension, base, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Renders a design from its canonical JSON name.
#[wasm_bindgen]
pub fn bang_factory_create_named(spec: &str, number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::factory::create_named(spec, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns every base-q residue corner of a dimension in row-major order.
#[wasm_bindgen]
pub fn bang_factory_residue_corners(dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::factory::residue_corners(dimension, base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the code count of a dimension and base, two to the number of corners.
#[wasm_bindgen]
pub fn bang_factory_total_codes(dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::factory::total_codes(dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Returns the code of the design filled wherever a corner's residue sum lands in the levels.
#[wasm_bindgen]
pub fn bang_levels_code(dimension: usize, base: usize, levels: &[usize]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::levels_code(dimension, base, levels);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.
#[wasm_bindgen]
pub fn bang_magic(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::magic(&layers).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Composes JSON-named layers in order.
#[wasm_bindgen]
pub fn bang_magic_named(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<(String, usize)>>(&layers)?;
    let layers_view: Vec<(&str, usize)> = layers.iter().map(|(a0, a1)| (a0.as_str(), *a1)).collect();
    let value = mrlyrs::math::bang::magic_named(&layers_view).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Builds the tile sources a catalog names at a dimension.
#[wasm_bindgen]
pub fn bang_sources(catalog: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let catalog = hand::from_js::<mrlyrs::gen::recipe::Catalog>(&catalog)?;
    let value = mrlyrs::math::bang::sources(&catalog, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the full symmetry group as axis permutations paired with flip patterns.
#[wasm_bindgen]
pub fn bang_symmetries(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::symmetries(dimension);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[hand::typed(&(x1.0)[..]), hand::typed(&(x1.1)[..])])))
}

/// Returns whether no two filled corners of a code sit at Hamming distance one.
#[wasm_bindgen]
pub fn bang_total_exposure(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::total_exposure(code, dimension);
    Ok(value)
}

/// Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.
#[wasm_bindgen]
pub fn bang_touches_every_corner(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::touches_every_corner(code, dimension);
    Ok(value)
}

/// Returns the algebraic normal form coefficients of a code, one per corner.
#[wasm_bindgen]
pub fn bang_universe_anf(code: JsValue, dimension: usize) -> Result<Vec<u8>, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::anf(code, dimension);
    Ok(value)
}

/// Formats the algebraic normal form of a code as a sum of monomials.
#[wasm_bindgen]
pub fn bang_universe_anf_string(code: JsValue, dimension: usize) -> Result<String, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::anf_string(code, dimension);
    Ok(value)
}

/// Applies a symmetry element to a corner.
#[wasm_bindgen]
pub fn bang_universe_apply(element: JsValue, corner: &[u8]) -> Result<Vec<u8>, JsValue> {
    let element = hand::from_js::<(Vec<usize>, Vec<u8>)>(&element)?;
    let value = mrlyrs::math::bang::universe::apply(&element, corner);
    Ok(value)
}

/// Returns the bit position a binary corner occupies in a code.
#[wasm_bindgen]
pub fn bang_universe_corner_index(corner: &[u8]) -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::universe::corner_index(corner);
    Ok(value)
}

/// Returns the algebraic degree of a code, or -1 for the zero design.
#[wasm_bindgen]
pub fn bang_universe_degree(code: JsValue, dimension: usize) -> Result<i32, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::degree(code, dimension);
    Ok(value)
}

/// Returns every code a design reaches under the full symmetry group.
#[wasm_bindgen]
pub fn bang_universe_orbit(code: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::bang::universe::orbit(code, dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

/// Returns every permutation of 0..n in sorted order.
#[wasm_bindgen]
pub fn bang_universe_permutations(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::universe::permutations(n);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the canonical design codes of a dimension, computed once and cached for the process.
#[wasm_bindgen]
pub fn bang_universe_codes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::universe_codes(dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the 4-connected components of a plane word without drawing it.
#[wasm_bindgen]
pub fn bang_word_components(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::components(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the constant-word component functional of a plane word's letter frequencies,
#[wasm_bindgen]
pub fn bang_word_constant_functional(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::constant_functional(&layers).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.
#[wasm_bindgen]
pub fn bang_word_dimension(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::dimension(&layers).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the filled cells of a word, the product of its letter fills.
#[wasm_bindgen]
pub fn bang_word_fill(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::fill(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Lists the filled cells of every letter, the product of which is the word's fill.
#[wasm_bindgen]
pub fn bang_word_fills(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::fills(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Reads one plane letter: its fill, its runs, the rows and columns that wrap into a
#[wasm_bindgen]
pub fn bang_word_letter(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrlyrs::math::bang::MagicLayer>(&layer)?;
    let value = mrlyrs::math::bang::word::letter(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns whether every letter renders at its own residue base, the native case where a
#[wasm_bindgen]
pub fn bang_word_native(layers: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::native(&layers);
    Ok(value)
}

/// Returns the shortest whole period of the letter list, its own length when no shorter block repeats.
#[wasm_bindgen]
pub fn bang_word_period(layers: JsValue) -> Result<usize, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::period(&layers);
    Ok(value)
}

/// Folds a plane word letter by letter and returns the counts at every prefix.
#[wasm_bindgen]
pub fn bang_word_prefixes(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::prefixes(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the prefix rates of a plane word in log two units, the component rate
#[wasm_bindgen]
pub fn bang_word_rates(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::rates(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the side of a word, the product of its letter sides.
#[wasm_bindgen]
pub fn bang_word_side(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::bang::word::side(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Spells the first letters of a schedule over an ordered pair of letters.
#[wasm_bindgen]
pub fn bang_word_spell(schedule: JsValue, pair: JsValue, length: usize) -> Result<JsValue, JsValue> {
    let schedule = hand::from_js::<mrlyrs::math::bang::word::Schedule>(&schedule)?;
    let pair = hand::from_js::<(mrlyrs::math::bang::MagicLayer, mrlyrs::math::bang::MagicLayer)>(&pair)?;
    let value = mrlyrs::math::bang::word::spell(schedule, pair, length);
    hand::to_js(&value)
}

/// Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,
#[wasm_bindgen]
pub fn bang_word_staircase(depth: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::word::staircase(depth).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
#[wasm_bindgen]
pub fn bang_word_thue_morse(index: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::word::thue_morse(index);
    Ok(value)
}

/// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
#[wasm_bindgen]
pub fn cell_census_edges(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Counts the faces of filled sites open to emptiness or the border.
#[wasm_bindgen]
pub fn cell_census_exposure(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Counts the filled sites of the cell.
#[wasm_bindgen]
pub fn cell_census_fills(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Counts the distinct corners the filled sites touch, the edge graph's nodes.
#[wasm_bindgen]
pub fn cell_census_vertices(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Counts the empty sites of the cell.
#[wasm_bindgen]
pub fn cell_census_voids(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Merges same-shaped cells into one block laid out by the per-axis repetition counts.
#[wasm_bindgen]
pub fn cell_geometry_merge_reps(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value = mrlyrs::math::cell::geometry::merge_reps::<2>(&cells, reps).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value = mrlyrs::math::cell::geometry::merge_reps::<3>(&cells, reps).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Writes the value into the cell wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn cell_geometry_perforate(mask: JsValue, cell: JsValue, value: u8) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::cell::geometry::perforate::<2>(&mask, &cell, value).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::cell::geometry::perforate::<3>(&mask, &cell, value).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Grows a seed pattern into a cell, deepened to its fractal past level one.
#[wasm_bindgen]
pub fn cell_grow(pattern: JsValue, level: usize) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Inverts the cell.
#[wasm_bindgen]
pub fn cell_models_anti(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Maps each site to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn cell_models_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Binarizes the cell at the threshold Otsu's method picks.
#[wasm_bindgen]
pub fn cell_models_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Rounds each site to the mean of its masked neighborhood, wrapping on request.
#[wasm_bindgen]
pub fn cell_models_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the Kronecker product of the two cells.
#[wasm_bindgen]
pub fn cell_models_combine(cell: JsValue, other: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the narrowest count dtype that fits the mask's popcount.
#[wasm_bindgen]
pub fn cell_models_counting_dtype(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::cell::models::counting_dtype(&mask);
    hand::to_js(&value)
}

/// Returns the size of axis 0, the cube's leading axis.
#[wasm_bindgen]
pub fn cell_models_depth(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.depth();
            Ok(value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the narrowest unsigned dtype that holds the peak value.
#[wasm_bindgen]
pub fn cell_models_dtype_for(peak: JsValue) -> Result<JsValue, JsValue> {
    let peak = hand::i64_from_js(&peak)?;
    let value = mrlyrs::math::cell::models::dtype_for(peak);
    hand::to_js(&value)
}

/// Deepens the cell into its level-fold fractal.
#[wasm_bindgen]
pub fn cell_models_fractal(cell: JsValue, level: usize) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the size of the axis before the last.
#[wasm_bindgen]
pub fn cell_models_height(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Swaps filled and empty sites.
#[wasm_bindgen]
pub fn cell_models_invert(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Tags each site with its ring distance from the center.
#[wasm_bindgen]
pub fn cell_models_layers(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Tags each site with its count of masked neighbors matching the target, wrapping on request.
#[wasm_bindgen]
pub fn cell_models_neighbors(cell: JsValue, mask: JsValue, target: u8, wrap: bool) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Builds a cell from an N-dimensional tensor of types.
#[wasm_bindgen]
pub fn cell_models_new(types: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Turns the cell into one of the 24 cube orientations.
#[wasm_bindgen]
pub fn cell_models_orient(cell: JsValue, index: usize) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = cell.orient(index).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Wraps the cell in count layers of the given value on every side.
#[wasm_bindgen]
pub fn cell_models_pad(cell: JsValue, count: usize, value: u8) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Colors each site by its type through the mapping in the given mode.
#[wasm_bindgen]
pub fn cell_models_paint(cell: JsValue, mapping: JsValue, mode: JsValue, rng: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let mapping = hand::map_from_js::<u8, _>(&mapping, |x1| hand::list_from_js(x1, hand::color_from_js))?;
            let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = cell.paint(&mapping, mode, rng_stream.as_mut()).map_err(hand::throw)?;
            if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let mapping = hand::map_from_js::<u8, _>(&mapping, |x1| hand::list_from_js(x1, hand::color_from_js))?;
            let mode = hand::from_js::<mrlyrs::core::Mode>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = cell.paint(&mapping, mode, rng_stream.as_mut()).map_err(hand::throw)?;
            if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Writes the value wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn cell_models_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Rotates the cell k quarter turns in the plane.
#[wasm_bindgen]
pub fn cell_models_rotate(cell: JsValue, k: usize, axes: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.rotate(k).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let axes = if axes.is_undefined() { return Err(hand::refuse("a 3d cell wants axes.")); } else { hand::from_js::<(usize, usize)>(&axes)? };
            let value = cell.rotate(k, axes).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Repeats the cell into a width-by-height array of copies.
#[wasm_bindgen]
pub fn cell_models_tile(cell: JsValue, width: usize, height: usize, depth: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = cell.tile(width, height).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let depth = if depth.is_undefined() { return Err(hand::refuse("a 3d cell wants depth.")); } else { hand::from_js::<usize>(&depth)? };
            let value = cell.tile(width, height, depth).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the tensor of types.
#[wasm_bindgen]
pub fn cell_models_types(cell: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the size of the last axis.
#[wasm_bindgen]
pub fn cell_models_width(cell: JsValue) -> Result<usize, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Colors the cell through the given mapping and mode, defaulting to the standard palette by type.
#[wasm_bindgen]
pub fn cell_paint(cell: JsValue, custom: JsValue, mode: JsValue, rng: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let custom = hand::option_from_js(&custom, |x1| hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js)))?;
            let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = mrlyrs::math::cell::paint::<2>(cell, custom.as_ref(), mode, rng_stream.as_mut()).map_err(hand::throw)?;
            if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let custom = hand::option_from_js(&custom, |x1| hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js)))?;
            let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value = mrlyrs::math::cell::paint::<3>(cell, custom.as_ref(), mode, rng_stream.as_mut()).map_err(hand::throw)?;
            if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Reads a triply nested JSON array into layers of byte rows.
#[wasm_bindgen]
pub fn cell_serializer_byte_cube(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::byte_cube(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..]))))
}

/// Reads a nested JSON array into rows of bytes.
#[wasm_bindgen]
pub fn cell_serializer_byte_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::byte_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Reads a nested JSON array into rows of four-channel colors.
#[wasm_bindgen]
pub fn cell_serializer_color_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::color_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..]))))
}

/// Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
#[wasm_bindgen]
pub fn cell_serializer_count_cube(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::count_cube(&value).map_err(hand::throw)?;
    Ok(value)
}

/// Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
#[wasm_bindgen]
pub fn cell_serializer_count_grid(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrlyrs::core::Json>(&value)?;
    let value = mrlyrs::math::cell::serializer::count_grid(&value).map_err(hand::throw)?;
    Ok(value)
}

/// Parses JSON text into a value tree.
#[wasm_bindgen]
pub fn cell_serializer_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::cell::serializer::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.
#[wasm_bindgen]
pub fn cell_serializer_tag_layer(counts: JsValue, shape: Vec<usize>) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::i64_from_js)?;
    let value = mrlyrs::math::cell::serializer::tag_layer(&counts, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns the types field of the data.
#[wasm_bindgen]
pub fn cell_serializer_types_field(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<mrlyrs::core::Json>(&data)?;
    let value = mrlyrs::math::cell::serializer::types_field(&data).map_err(hand::throw)?;
    hand::to_js(value)
}

/// Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.
#[wasm_bindgen]
pub fn counts_centered_hexagonal(m: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::centered_hexagonal(m);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled triangle count of the code's cut section at the given level, without rendering it.
#[wasm_bindgen]
pub fn counts_cut_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::cut_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty triangle count of the code's cut section at the given level.
#[wasm_bindgen]
pub fn counts_cut_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::cut_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the code's fractal dimension, the log of its one-level fill over the log of number.
#[wasm_bindgen]
pub fn counts_dimension(code: JsValue, number: usize, base_dimension: usize, base: usize) -> Result<f64, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::dimension(code, number, base_dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term
#[wasm_bindgen]
pub fn counts_edges_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::edges_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.
#[wasm_bindgen]
pub fn counts_exposure(code: JsValue, number: usize, dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::exposure(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.
#[wasm_bindgen]
pub fn counts_exposure_of_tile_(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::exposure_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the coefficients of the recurrence the tile's exposure obeys.
#[wasm_bindgen]
pub fn counts_exposure_recurrence_(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::exposure_recurrence(&tile);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the filled cell count of the code's fractal at the given level, without rendering it.
#[wasm_bindgen]
pub fn counts_fill(code: JsValue, number: usize, dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::fill(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Sums each corner's position products into a base fill and raises it to the level.
#[wasm_bindgen]
pub fn counts_fill_from_corners(filled: JsValue, number: usize, _dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::counts::fill_from_corners(&filled, number, _dimension, level, base);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the total cells of the grid, number to the dimension, to the level.
#[wasm_bindgen]
pub fn counts_grid(number: usize, dimension: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::grid(number, dimension, level);
    Ok(JsValue::from_str(&value.to_string()))
}

/// The widest dimension the exact carry arithmetic reaches at the base.
#[wasm_bindgen]
pub fn counts_ladder_cap(base: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::counts::ladder::cap(base).map_err(hand::throw)?;
    Ok(value)
}

/// The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.
#[wasm_bindgen]
pub fn counts_ladder_carry_matrix(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::carry_matrix(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string()))))
}

/// The monic characteristic polynomial of a square integer matrix, highest power first.
#[wasm_bindgen]
pub fn counts_ladder_characteristic(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::characteristic(&rows).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The determinant of a square integer matrix, read off its characteristic polynomial.
#[wasm_bindgen]
pub fn counts_ladder_determinant(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::determinant(&rows).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.
#[wasm_bindgen]
pub fn counts_ladder_digit_polynomial(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::digit_polynomial(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The reflection-even block of the carry matrix, of size `ceil(D/2)`.
#[wasm_bindgen]
pub fn counts_ladder_even_block(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::even_block(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string()))))
}

/// The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.
#[wasm_bindgen]
pub fn counts_ladder_fill(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::fill(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.
#[wasm_bindgen]
pub fn counts_ladder_ladder(base: usize, dimension: usize, levels: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::ladder(base, dimension, levels).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// The Perron root of a nonnegative square integer matrix.
#[wasm_bindgen]
pub fn counts_ladder_perron(rows: JsValue) -> Result<f64, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::perron(&rows).map_err(hand::throw)?;
    Ok(value)
}

/// The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.
#[wasm_bindgen]
pub fn counts_ladder_sign(base: usize, dimension: usize) -> Result<i32, JsValue> {
    let value = mrlyrs::math::counts::ladder::sign(base, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.
#[wasm_bindgen]
pub fn counts_ladder_spectral_ratio(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::ladder::spectral_ratio(base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The trace of a square integer matrix.
#[wasm_bindgen]
pub fn counts_ladder_trace(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrlyrs::math::counts::ladder::trace(&rows);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the fill ratio the code walks toward as the side number grows, reduced.
#[wasm_bindgen]
pub fn counts_limit(code: JsValue, dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::limit(code, dimension, level, base).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[JsValue::from_str(&value.0.to_string()), JsValue::from_str(&value.1.to_string())]))
}

/// Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.
#[wasm_bindgen]
pub fn counts_pairs(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::pairs(&tile);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from_str(&x1.0.to_string()), JsValue::from_str(&x1.1.to_string())])))
}

/// Counts the indices below number that equal residue modulo base.
#[wasm_bindgen]
pub fn counts_positions(residue: usize, number: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::positions(residue, number, base);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the filled triangle count of the code's pro projection at the given level, without rendering it.
#[wasm_bindgen]
pub fn counts_pro_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::pro_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty triangle count of the code's pro projection at the given level.
#[wasm_bindgen]
pub fn counts_pro_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::pro_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.
#[wasm_bindgen]
pub fn counts_profile_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrlyrs::math::counts::profile_of_tile(&tile, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the filled fraction of the grid, or 0.0 for an empty grid.
#[wasm_bindgen]
pub fn counts_ratio(code: JsValue, number: usize, dimension: usize, level: u32, base: usize) -> Result<f64, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::ratio(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the exact filled fraction as a fraction of fill over grid, reduced.
#[wasm_bindgen]
pub fn counts_rational(code: JsValue, number: usize, dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::rational(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[JsValue::from_str(&value.0.to_string()), JsValue::from_str(&value.1.to_string())]))
}

/// Returns the triangles of the full hexagon with side number to the level.
#[wasm_bindgen]
pub fn counts_six_grid_triangles(number: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::grid_triangles(number, level);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the boundary edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_boundary(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_boundary(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the core edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_core_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_core_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the core node count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_core_nodes(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_core_nodes(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the distinct triangle-edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the interior edge count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_interior(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_interior(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the triangle count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_triangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_triangles(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the vertex count of the solid slice, defined for odd number.
#[wasm_bindgen]
pub fn counts_six_solid_slice_vertices(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::counts::six::solid_slice_vertices(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the exposed face count of the code's 3D fractal at the given level.
#[wasm_bindgen]
pub fn counts_surface(code: JsValue, number: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::surface(code, number, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the empty cell count, grid minus fill.
#[wasm_bindgen]
pub fn counts_void(code: JsValue, number: usize, dimension: usize, level: u32, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::counts::void(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Takes the full census of a network.
#[wasm_bindgen]
pub fn graph_census(network: &graph_Network) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::graph::census(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts the connected components of the network.
#[wasm_bindgen]
pub fn graph_components(network: &graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::components(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Extracts the network of filled sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn graph_core_graph(grid: JsValue) -> Result<graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::core_graph(&grid).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Extracts the network of corners and edges outlining every filled site.
#[wasm_bindgen]
pub fn graph_edge_graph(grid: JsValue) -> Result<graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::edge_graph(&grid).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.
#[wasm_bindgen]
pub fn graph_fractal_dimension(network: &graph_Network, samples: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::graph::fractal_dimension(&network.inner, samples);
    Ok(value)
}

/// Counts the nodes of degree three or more.
#[wasm_bindgen]
pub fn graph_junctions(network: &graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::junctions(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Extracts the largest connected piece as a network of its own, branches re-indexed.
#[wasm_bindgen]
pub fn graph_largest_component(network: &graph_Network) -> Result<graph_Network, JsValue> {
    let value = mrlyrs::math::graph::largest_component(&network.inner).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Tags every node by its degree, indexed like the node list.
#[wasm_bindgen]
pub fn graph_roles(network: &graph_Network) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::graph::roles(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts the nodes of degree one.
#[wasm_bindgen]
pub fn graph_tips(network: &graph_Network) -> Result<usize, JsValue> {
    let value = mrlyrs::math::graph::tips(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

/// Sums the straight-line lengths of every branch.
#[wasm_bindgen]
pub fn graph_total_length(network: &graph_Network) -> Result<f64, JsValue> {
    let value = mrlyrs::math::graph::total_length(&network.inner);
    Ok(value)
}

/// Extracts the core graph of the inverted grid, joining empty sites instead.
#[wasm_bindgen]
pub fn graph_tunnel_graph(grid: JsValue) -> Result<graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrlyrs::math::graph::tunnel_graph(&grid).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Returns every preset stacked up to the given scale.
#[wasm_bindgen]
pub fn moire_all(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::moire::all(limit);
    hand::list_to_js(&value, |x1| Ok(JsValue::from(moire_Preset { inner: x1.clone() })))
}

/// Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.
#[wasm_bindgen]
pub fn moire_frame(normal: JsValue, offset: f64) -> Result<JsValue, JsValue> {
    let normal = hand::from_js::<[f64; 3]>(&normal)?;
    let value = mrlyrs::math::moire::frame(normal, offset).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Samples a design over the pixel grid into a boolean mask.
#[wasm_bindgen]
pub fn moire_layer(params: JsValue) -> Result<JsValue, JsValue> {
    let params = hand::from_js::<mrlyrs::math::moire::Layer>(&params)?;
    let value = mrlyrs::math::moire::layer(&params).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the preset the name picks.
#[wasm_bindgen]
pub fn moire_named(name: &str, limit: usize) -> Result<moire_Preset, JsValue> {
    let value = mrlyrs::math::moire::named(name, limit).map_err(hand::throw)?;
    Ok(moire_Preset { inner: value })
}

/// Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.
#[wasm_bindgen]
pub fn moire_pairs_correlation(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::moire::pairs::correlation(m, n);
    Ok(value)
}

/// Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.
#[wasm_bindgen]
pub fn moire_pairs_sampled(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::moire::pairs::sampled(m, n).map_err(hand::throw)?;
    Ok(value)
}

/// Puts an odd scale of three or more on trial against every earlier odd scale.
#[wasm_bindgen]
pub fn moire_pairs_witness(scale: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::moire::pairs::witness(scale).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Quantizes a field into colored levels and encodes PNG bytes.
#[wasm_bindgen]
pub fn moire_render(field: &moire_Field, colorizer: JsValue, levels: usize, symmetric: bool, invert: bool, scale: usize) -> Result<Vec<u8>, JsValue> {
    let colorizer = hand::from_js::<mrlyrs::core::Colorizer>(&colorizer)?;
    let value = mrlyrs::math::moire::render(&field.inner, &colorizer, levels, symmetric, invert, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the two lattice coordinates of each pixel centre along a row.
#[wasm_bindgen]
pub fn moire_sample_axes(size: usize, lattice: JsValue, row: usize) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::sample::axes(size, lattice, row);
    Ok(hand::tuple_to_js(&[hand::typed(&(value.0)[..]), hand::typed(&(value.1)[..])]))
}

/// Unpacks a code into its residue-corner truth table.
#[wasm_bindgen]
pub fn moire_sample_membership(code: JsValue, base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::math::moire::sample::membership(code, base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Folds residues into a base-q index of the truth table.
#[wasm_bindgen]
pub fn moire_sample_pack(residues: &[usize], base: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::moire::sample::pack(residues, base);
    Ok(value)
}

/// Layers one design at several side numbers into a field under the chosen combine.
#[wasm_bindgen]
pub fn moire_stack(spec: JsValue, numbers: &[usize], combine: JsValue, level: usize, lattice: JsValue, size: usize, slices: &[f64]) -> Result<moire_Field, JsValue> {
    let spec = hand::from_js::<mrlyrs::math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrlyrs::math::moire::Combine>(&combine)?;
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::stack(spec, numbers, combine, level, lattice, size, slices).map_err(hand::throw)?;
    Ok(moire_Field { inner: value })
}

/// Sums layers of several designs at one side number into a field.
#[wasm_bindgen]
pub fn moire_stack_codes(specs: JsValue, number: usize, level: usize, lattice: JsValue, size: usize, slices: &[f64]) -> Result<moire_Field, JsValue> {
    let specs = hand::from_js::<Vec<mrlyrs::math::moire::Spec>>(&specs)?;
    let lattice = hand::from_js::<mrlyrs::math::moire::Lattice>(&lattice)?;
    let value = mrlyrs::math::moire::stack_codes(&specs, number, level, lattice, size, slices).map_err(hand::throw)?;
    Ok(moire_Field { inner: value })
}

/// Layers one cube design at several side numbers into a volume under the chosen combine.
#[wasm_bindgen]
pub fn moire_volume(spec: JsValue, numbers: &[usize], combine: JsValue, level: usize, size: usize) -> Result<moire_Volume, JsValue> {
    let spec = hand::from_js::<mrlyrs::math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrlyrs::math::moire::Combine>(&combine)?;
    let value = mrlyrs::math::moire::volume(spec, numbers, combine, level, size).map_err(hand::throw)?;
    Ok(moire_Volume { inner: value })
}

/// Returns the number of designs of the dimension and base that contain the number.
#[wasm_bindgen]
pub fn press_containing(number: JsValue, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::containing(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.
#[wasm_bindgen]
pub fn press_coordinates(number: JsValue, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::coordinates(number, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the members of a design below the limit.
#[wasm_bindgen]
pub fn press_count_below(code: JsValue, dimension: usize, base: usize, limit: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let limit = hand::u128_from_js(&limit)?;
    let value = mrlyrs::math::press::count_below(code, dimension, base, limit).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the count of distinct digit vectors the number uses.
#[wasm_bindgen]
pub fn press_distinct(number: JsValue, dimension: usize, base: usize) -> Result<u32, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::distinct(number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Weaves dimension coordinates back into their single interleaved number.
#[wasm_bindgen]
pub fn press_interleave(coords: JsValue, base: usize) -> Result<JsValue, JsValue> {
    let coords = hand::list_from_js(&coords, hand::u128_from_js)?;
    let value = mrlyrs::math::press::interleave(&coords, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the allowed digit table of one magic layer, one flag per cell of its tile.
#[wasm_bindgen]
pub fn press_layer_table(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrlyrs::math::bang::MagicLayer>(&layer)?;
    let value = mrlyrs::math::press::layer_table(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns whether every digit vector of the number lies in the design.
#[wasm_bindgen]
pub fn press_member(code: JsValue, number: JsValue, dimension: usize, base: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::member(code, number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the first members of a design in ascending order.
#[wasm_bindgen]
pub fn press_members(code: JsValue, dimension: usize, base: usize, count: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::press::members(code, dimension, base, count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the diagonal slice profile of one design pressed to a fractal level.
#[wasm_bindgen]
pub fn press_profile(code: JsValue, dimension: usize, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::press::profile(code, dimension, base, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.
#[wasm_bindgen]
pub fn press_usage(number: JsValue, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::usage(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

/// Counts the members of a magic word from its layer fills, without enumeration.
#[wasm_bindgen]
pub fn press_word_count(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_count(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether the number lies in the magic word's composed design.
#[wasm_bindgen]
pub fn press_word_member(layers: JsValue, number: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrlyrs::math::press::word_member(&layers, number).map_err(hand::throw)?;
    Ok(value)
}

/// Enumerates every member of the magic word in ascending order.
#[wasm_bindgen]
pub fn press_word_members(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_members(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the diagonal slice profile of a magic word by the substitution product.
#[wasm_bindgen]
pub fn press_word_profile(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrlyrs::math::bang::MagicLayer>>(&layers)?;
    let value = mrlyrs::math::press::word_profile(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.
#[wasm_bindgen]
pub fn roulette_nodes(track: JsValue, pencils: JsValue, samples: usize, tol: f64) -> Result<roulette_Nodes, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::roulette::nodes(&track, &pencils, samples, tol).map_err(hand::throw)?;
    Ok(roulette_Nodes { inner: value })
}

/// Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.
#[wasm_bindgen]
pub fn roulette_side(a: JsValue, b: JsValue, c: JsValue) -> Result<i32, JsValue> {
    let a = hand::from_js::<[f64; 2]>(&a)?;
    let b = hand::from_js::<[f64; 2]>(&b)?;
    let c = hand::from_js::<[f64; 2]>(&c)?;
    let value = mrlyrs::math::roulette::side(a, b, c);
    Ok(value)
}

/// One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.
#[wasm_bindgen]
pub fn roulette_spread(track: JsValue, pencils: JsValue, exact: bool) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::roulette::spread(&track, &pencils, exact);
    hand::to_js(&value)
}

/// Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.
#[wasm_bindgen]
pub fn rules_render(filled: JsValue, number: usize, dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrlyrs::math::rules::render(&filled, number, dimension, base).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Returns every axis but the free one.
#[wasm_bindgen]
pub fn rules_tree_axes(dimension: usize, free_axis: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::math::rules::tree_axes(dimension, free_axis);
    Ok(value)
}

/// Tallies the design's cells and filled cells per region of the shape.
#[wasm_bindgen]
pub fn shape_census(shape: JsValue, types: JsValue) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let types = hand::tensor_from_js(&types)?;
    let value = mrlyrs::math::shape::census(&shape, &types).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Places one lattice cell relative to the shape, exactly, with no floats.
#[wasm_bindgen]
pub fn shape_classify(shape: JsValue, side: usize, index: &[usize]) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::classify(&shape, side, index).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.
#[wasm_bindgen]
pub fn shape_crop(types: JsValue, shape: JsValue, keep_cut: bool) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::crop(&types, &shape, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.
#[wasm_bindgen]
pub fn shape_crossing_shell(radius: JsValue, number: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::math::shape::crossing_shell(radius, number, level);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from(x1.0), JsValue::from(x1.1)])))
}

/// Builds the whole crossing tree of one radius, pruned by the seats the design keeps.
#[wasm_bindgen]
pub fn shape_crossing_tree(radius: JsValue, number: JsValue, keep: JsValue) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let keep = hand::from_js::<Vec<bool>>(&keep)?;
    let value = mrlyrs::math::shape::crossing_tree(radius, number, &keep);
    hand::to_js(&value)
}

/// Builds a named shape of the dimension, centered at one half on every axis.
#[wasm_bindgen]
pub fn shape_named(name: &str, dimension: usize, radius: &shape_Frac) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::shape::named(name, dimension, radius.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.
#[wasm_bindgen]
pub fn shape_radial_census(types: JsValue, centre: JsValue, r_max: JsValue) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let centre = hand::list_from_js(&centre, hand::i64_from_js)?;
    let r_max = hand::u64_from_js(&r_max)?;
    let value = mrlyrs::math::shape::radial_census(&types, &centre, r_max);
    hand::to_js(&value)
}

/// Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.
#[wasm_bindgen]
pub fn shape_refine(types: JsValue, shape: JsValue, base: usize, extra: usize, keep_cut: bool) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::refine(&types, &shape, base, extra, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.
#[wasm_bindgen]
pub fn shape_regions(shape: JsValue, dims: &[usize]) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrlyrs::math::shape::Shape>(&shape)?;
    let value = mrlyrs::math::shape::regions(&shape, dims).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Lists the named shapes of a dimension.
#[wasm_bindgen]
pub fn shape_shapes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::shape::shapes(dimension);
    hand::to_js(&value)
}

/// Swaps every fill triangle for a void and back.
#[wasm_bindgen]
pub fn six_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.anti();
    hand::cell6d_to_js(&value)
}

/// Maps each triangle to one at or above the threshold, zero below.
#[wasm_bindgen]
pub fn six_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell6d_to_js(&value)
}

/// Binarizes the triangles at the threshold Otsu's method picks.
#[wasm_bindgen]
pub fn six_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell6d_to_js(&value)
}

/// Builds a hexagon of the given radius, fill inside and void outside.
#[wasm_bindgen]
pub fn six_blank(radius: usize, orient: JsValue, fill: u8, void_: u8) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrlyrs::math::six::Orientation>(&orient)?;
    let value = mrlyrs::math::six::blank(radius, orient, fill, void_).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Rounds each triangle to the mean of its masked neighborhood, wrapping on request.
#[wasm_bindgen]
pub fn six_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Tallies a cell's triangles, corners and edges, counting the backdrop only on request.
#[wasm_bindgen]
pub fn six_census(cell: JsValue, include_grid: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::census(&cell, include_grid);
    hand::to_js(&value)
}

/// Counts the connected pieces of the fill, triangles joined across shared edges.
#[wasm_bindgen]
pub fn six_components(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::components(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Slices a cube through its center across the main diagonal into a hexagon.
#[wasm_bindgen]
pub fn six_cut(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::cut(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and slices its central hexagon.
#[wasm_bindgen]
pub fn six_cut_design(code: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::cut_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// The three corners of the east-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn six_east(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::east(x, y);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from(x1.0), JsValue::from(x1.1)])))
}

/// Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.
#[wasm_bindgen]
pub fn six_euler(cell: JsValue, include_grid: bool) -> Result<i64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::euler(&cell, include_grid);
    Ok(value)
}

/// Counts the filled triangles of the cell.
#[wasm_bindgen]
pub fn six_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::fills(&cell);
    Ok(value)
}

/// Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.
#[wasm_bindgen]
pub fn six_fills_only(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::fills_only(&cell);
    hand::to_js(&value)
}

/// Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.
#[wasm_bindgen]
pub fn six_framed(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::framed(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Parses a cell from JSON, defaulting any missing projection metadata.
#[wasm_bindgen]
pub fn six_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::from_json(text).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Returns the triangle count of the fill's largest connected piece.
#[wasm_bindgen]
pub fn six_giant(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::giant(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the largest connected piece of the filled-triangle network as a network of its own.
#[wasm_bindgen]
pub fn six_giant_network(cell: JsValue) -> Result<graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::giant_network(&cell).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Returns the grid height in triangles.
#[wasm_bindgen]
pub fn six_height(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.height();
    Ok(value)
}

/// Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.
#[wasm_bindgen]
pub fn six_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Returns whether the cell's three sides are equal.
#[wasm_bindgen]
pub fn six_is_cube(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::is_cube(&cell);
    Ok(value)
}

/// Returns whether the cell's width, height and parity frame a hexagon.
#[wasm_bindgen]
pub fn six_is_hex(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::six::is_hex(&cell);
    Ok(value)
}

/// Projects a cube into the isometric hexagon of top, left and right faces.
#[wasm_bindgen]
pub fn six_iso(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::iso(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and projects it isometrically.
#[wasm_bindgen]
pub fn six_iso_design(code: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::iso_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds a cell from its four parts.
#[wasm_bindgen]
pub fn six_new(cell: JsValue, projection: JsValue, orientation: JsValue, start: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let projection = hand::from_js::<mrlyrs::math::six::Projection>(&projection)?;
    let orientation = hand::from_js::<mrlyrs::math::six::Orientation>(&orientation)?;
    let value = mrlyrs::math::six::Cell6d::new(cell, projection, orientation, start);
    hand::cell6d_to_js(&value)
}

/// The three corners of the north-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn six_north(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::north(x, y);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from(x1.0), JsValue::from(x1.1)])))
}

/// Returns the orientation a hexagon's width and height imply.
#[wasm_bindgen]
pub fn six_orientation(width: usize, height: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::orientation(width, height).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.
#[wasm_bindgen]
pub fn six_pad(cell: JsValue, k: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::pad(&cell, k, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Colors each triangle by its type through the custom or default mapping in the given or type mode.
#[wasm_bindgen]
pub fn six_paint(cell: JsValue, custom: JsValue, mode: JsValue, rng: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let custom = hand::option_from_js(&custom, |x1| hand::map_from_js::<u8, _>(x1, |x2| hand::list_from_js(x2, hand::color_from_js)))?;
    let mode = hand::from_js::<Option<mrlyrs::core::Mode>>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = mrlyrs::math::six::paint(cell, custom.as_ref(), mode, rng_stream.as_mut()).map_err(hand::throw)?;
    if let Some(stream) = &rng_stream { hand::stream_to_js(&rng, stream)?; }
    hand::cell6d_to_js(&value)
}

/// Writes the value wherever the tiled mask is nonzero.
#[wasm_bindgen]
pub fn six_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn six_png(cell: JsValue, scale: usize, outline: JsValue, width: usize) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let value = mrlyrs::math::six::png(&cell, scale, outline, width).map_err(hand::throw)?;
    Ok(value)
}

/// Projects a cube's three facing sides into a hexagon of fills and voids.
#[wasm_bindgen]
pub fn six_pro(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::six::pro(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Builds the coded 3d design and projects its facing sides.
#[wasm_bindgen]
pub fn six_pro_design(code: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::six::pro_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Tessellates a hexagonal cell over the disc mask of the given radius.
#[wasm_bindgen]
pub fn six_radial(cell: JsValue, radius: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::radial(&cell, radius).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Crops the interlocking overhang off a disc tiled at the given radius and tile size.
#[wasm_bindgen]
pub fn six_radial_crop(cell: JsValue, radius: usize, size: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::radial_crop(&cell, radius, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the disc mask of cells within hex distance radius of the center.
#[wasm_bindgen]
pub fn six_radial_mask(radius: usize, orient: JsValue) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrlyrs::math::six::Orientation>(&orient)?;
    let value = mrlyrs::math::six::radial_mask(radius, orient);
    hand::tensor_to_js(&value)
}

/// Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.
#[wasm_bindgen]
pub fn six_raster(cell: JsValue, size: usize) -> Result<Vec<f32>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::raster(&cell, size).map_err(hand::throw)?;
    Ok(value)
}

/// Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes.
#[wasm_bindgen]
pub fn six_rect_png(cell: JsValue, scale: usize, start: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::rect_png(&cell, scale, start).map_err(hand::throw)?;
    Ok(value)
}

/// Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.
#[wasm_bindgen]
pub fn six_rect_svg(cell: JsValue, scale: usize, start: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::rect_svg(&cell, scale, start).map_err(hand::throw)?;
    Ok(value)
}

/// Counts the void regions the rim never reaches, the second route to the hole count.
#[wasm_bindgen]
pub fn six_rim_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::rim_holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.
#[wasm_bindgen]
pub fn six_skin(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::skin(&cell);
    hand::cell6d_to_js(&value)
}

/// Builds the network of filled triangles joined by shared edges.
#[wasm_bindgen]
pub fn six_slice_core_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_core_graph(&cell).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Builds the network of fill and void triangles joined by shared edges.
#[wasm_bindgen]
pub fn six_slice_dual_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_dual_graph(&cell).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.
#[wasm_bindgen]
pub fn six_slice_edge_graph(cell: JsValue, value: JsValue) -> Result<graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = hand::from_js::<Option<u8>>(&value)?;
    let value = mrlyrs::math::six::slice_edge_graph(&cell, value).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// Builds the network of void triangles joined by shared edges, the pore network of the slice.
#[wasm_bindgen]
pub fn six_slice_tunnel_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::slice_tunnel_graph(&cell).map_err(hand::throw)?;
    Ok(graph_Network { inner: value })
}

/// The three corners of the south-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn six_south(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::south(x, y);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from(x1.0), JsValue::from(x1.1)])))
}

/// Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.
#[wasm_bindgen]
pub fn six_spectral_exponent(cell: JsValue, window: f64) -> Result<f64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::spectral_exponent(&cell, window).map_err(hand::throw)?;
    Ok(value)
}

/// The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.
#[wasm_bindgen]
pub fn six_star_arm_law(number: usize) -> Result<six_star_Share, JsValue> {
    let value = mrlyrs::math::six::star::arm_law(number).map_err(hand::throw)?;
    Ok(six_star_Share { inner: value })
}

/// The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.
#[wasm_bindgen]
pub fn six_star_chi8(number: usize) -> Result<i64, JsValue> {
    let value = mrlyrs::math::six::star::chi8(number);
    Ok(value)
}

/// The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.
#[wasm_bindgen]
pub fn six_star_constant() -> Result<f64, JsValue> {
    let value = mrlyrs::math::six::star::constant();
    Ok(value)
}

/// The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.
#[wasm_bindgen]
pub fn six_star_decay(excesses: &[f64], layers: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::six::star::decay(excesses, layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.
#[wasm_bindgen]
pub fn six_star_width_law(half: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::six::star::width_law(half);
    Ok(value)
}

/// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn six_svg(cell: JsValue, scale: usize, outline: JsValue, width: usize, start: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::svg(&cell, scale, outline, width, start).map_err(hand::throw)?;
    Ok(value)
}

/// Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.
#[wasm_bindgen]
pub fn six_tessellate(cell: JsValue, mask: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::six::tessellate(&cell, &mask).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tessellates a hexagonal cell over a full width-by-height mask.
#[wasm_bindgen]
pub fn six_tile(cell: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::tile(&cell, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.
#[wasm_bindgen]
pub fn six_tile_cell(cell: JsValue, width: usize, height: usize, crop: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::tile_cell(&cell, width, height, crop).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

/// Crops one interlocking step off each side of a sheet tiled at the given size.
#[wasm_bindgen]
pub fn six_tile_crop(cell: JsValue, size: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::tile_crop(&cell, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.
#[wasm_bindgen]
pub fn six_tile_step(size: JsValue) -> Result<JsValue, JsValue> {
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrlyrs::math::six::tile_step(size).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Serializes a cell and its projection metadata to JSON.
#[wasm_bindgen]
pub fn six_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrlyrs::math::six::to_json(&cell);
    Ok(value)
}

/// Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.
#[wasm_bindgen]
pub fn six_triangles(cell: JsValue, start: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrlyrs::math::six::triangles(&cell, start).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[hand::list_to_js(&x1.0, |x3| Ok(hand::tuple_to_js(&[JsValue::from(x3.0), JsValue::from(x3.1)])))?, hand::typed(&(x1.1)[..])])))
}

/// The three corners of the west-pointing triangle at the grid column and row.
#[wasm_bindgen]
pub fn six_west(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrlyrs::math::six::west(x, y);
    hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from(x1.0), JsValue::from(x1.1)])))
}

/// Returns the grid width in triangles.
#[wasm_bindgen]
pub fn six_width(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.width();
    Ok(value)
}

/// Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.
#[wasm_bindgen]
pub fn spectrum_clusters(eigenvalues: &[f64], tolerance: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::clusters(eigenvalues, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.
#[wasm_bindgen]
pub fn spectrum_laplacian(network: &graph_Network, normalised: bool) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::laplacian(&network.inner, normalised).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.
#[wasm_bindgen]
pub fn spectrum_laplacian_spectrum(network: &graph_Network, normalised: bool) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::math::spectrum::laplacian_spectrum(&network.inner, normalised).map_err(hand::throw)?;
    Ok(value)
}

/// Counts the eigenvalues within the tolerance of a value.
#[wasm_bindgen]
pub fn spectrum_multiplicity(eigenvalues: &[f64], value: f64, tolerance: f64) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spectrum::multiplicity(eigenvalues, value, tolerance);
    Ok(value)
}

/// Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.
#[wasm_bindgen]
pub fn spectrum_spectral_exponent(eigenvalues: &[f64], window: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_exponent(eigenvalues, window);
    hand::to_js(&value)
}

/// Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.
#[wasm_bindgen]
pub fn spectrum_spectral_fit(eigenvalues: &[f64], window: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_fit(eigenvalues, window);
    hand::to_js(&value)
}

/// Builds the integrated density of states as points, each an eigenvalue and its rank fraction.
#[wasm_bindgen]
pub fn spectrum_spectral_points(eigenvalues: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spectrum::spectral_points(eigenvalues);
    hand::to_js(&value)
}

/// Returns the eigenvalues of a dense real symmetric matrix in ascending order.
#[wasm_bindgen]
pub fn spectrum_symmetric_eigenvalues(matrix: JsValue) -> Result<Vec<f64>, JsValue> {
    let matrix = hand::from_js::<Vec<Vec<f64>>>(&matrix)?;
    let value = mrlyrs::math::spectrum::symmetric_eigenvalues(&matrix).map_err(hand::throw)?;
    Ok(value)
}

/// The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.
#[wasm_bindgen]
pub fn spin_arcs(data: &[f32], size: usize, radius: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spin::arcs(data, size, radius).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.
#[wasm_bindgen]
pub fn spin_harmonics(data: &[f32], size: usize, rings: usize, orders: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::math::spin::harmonics(data, size, rings, orders).map_err(hand::throw)?;
    Ok(value)
}

/// The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.
#[wasm_bindgen]
pub fn spin_mass(profile: &[f32], size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::mass(profile, size);
    Ok(value)
}

/// The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.
#[wasm_bindgen]
pub fn spin_mass_within(profile: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::mass_within(profile, size, radius);
    Ok(value)
}

/// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
#[wasm_bindgen]
pub fn spin_petals(copies: usize, order: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spin::petals(copies, order);
    Ok(value)
}

/// The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.
#[wasm_bindgen]
pub fn spin_profile(data: &[f32], size: usize, steps: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::spin::profile(data, size, steps).map_err(hand::throw)?;
    Ok(value)
}

/// Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.
#[wasm_bindgen]
pub fn spin_radial(data: &[f32], size: usize, out: usize, copies: usize, step: f64, blend: JsValue, samples: usize) -> Result<Vec<f32>, JsValue> {
    let blend = hand::from_js::<mrlyrs::math::spin::Blend>(&blend)?;
    let value = mrlyrs::math::spin::radial(data, size, out, copies, step, blend, samples).map_err(hand::throw)?;
    Ok(value)
}

/// The radius of the corner circle of a square raster of the side, the last radius a profile reads.
#[wasm_bindgen]
pub fn spin_reach(size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::reach(size);
    Ok(value)
}

/// The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.
#[wasm_bindgen]
pub fn spin_ring(data: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spin::ring(data, size, radius).map_err(hand::throw)?;
    Ok(value)
}

/// The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.
#[wasm_bindgen]
pub fn spin_turns(power: &[f64]) -> Result<usize, JsValue> {
    let value = mrlyrs::math::spin::turns(power);
    Ok(value)
}

/// The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.
#[wasm_bindgen]
pub fn spin_wheel(profile: &[f32], size: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::spin::wheel(profile, size);
    Ok(value)
}

/// The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.
#[wasm_bindgen]
pub fn spirograph_cell(width: usize, height: usize, reach: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::math::spirograph::cell(width, height, reach);
    Ok(value)
}

/// The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.
#[wasm_bindgen]
pub fn spirograph_cover(track: JsValue, pencils: JsValue, exact: bool, samples: usize, side: usize) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::cover(&track, &pencils, exact, samples, side).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.
#[wasm_bindgen]
pub fn spirograph_disc(track: JsValue, pencils: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::disc(&track, &pencils).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.
#[wasm_bindgen]
pub fn spirograph_distinct(track: JsValue, pencils: JsValue, exact: bool) -> Result<usize, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::distinct(&track, &pencils, exact);
    Ok(value)
}

/// The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.
#[wasm_bindgen]
pub fn spirograph_frame(track: JsValue, pencils: JsValue) -> Result<Vec<f64>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::frame(&track, &pencils);
    Ok(value.to_vec())
}

/// The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.
#[wasm_bindgen]
pub fn spirograph_nodes(track: JsValue, pencils: JsValue, exact: bool) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::nodes(&track, &pencils, exact);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
}

/// Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.
#[wasm_bindgen]
pub fn spirograph_pencils(types: &[u8], width: usize, height: usize, mode: &str, reach: f64, jitter: f64, seed: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spirograph::pencils(types, width, height, mode, reach, jitter, seed).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.
#[wasm_bindgen]
pub fn spirograph_point(track: JsValue, pencil: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrlyrs::math::spirograph::Pencil>(&pencil)?;
    let value = mrlyrs::math::spirograph::point(&track, &pencil, s);
    hand::to_js(&value)
}

/// The wheel's centre after `s` of path length.
#[wasm_bindgen]
pub fn spirograph_pose(track: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let value = mrlyrs::math::spirograph::pose(&track, s);
    hand::to_js(&value)
}

/// One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.
#[wasm_bindgen]
pub fn spirograph_representatives(track: JsValue, pencils: JsValue, exact: bool) -> Result<Vec<usize>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::representatives(&track, &pencils, exact);
    Ok(value)
}

/// Counts the pencils by kind.
#[wasm_bindgen]
pub fn spirograph_seats(pencils: JsValue) -> Result<JsValue, JsValue> {
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::seats(&pencils);
    hand::to_js(&value)
}

/// The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.
#[wasm_bindgen]
pub fn spirograph_signed_area(track: JsValue, pencil: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrlyrs::math::spirograph::Pencil>(&pencil)?;
    let value = mrlyrs::math::spirograph::signed_area(&track, &pencil);
    hand::to_js(&value)
}

/// Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.
#[wasm_bindgen]
pub fn spirograph_trace(track: JsValue, pencils: JsValue, samples: usize) -> Result<Vec<f32>, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrlyrs::math::spirograph::Pencil>>(&pencils)?;
    let value = mrlyrs::math::spirograph::trace(&track, &pencils, samples).map_err(hand::throw)?;
    Ok(value)
}

/// Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.
#[wasm_bindgen]
pub fn spirograph_track(kind: &str, ring: usize, wheel: usize, sides: usize, laps: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spirograph::track(kind, ring, wheel, sides, laps).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.
#[wasm_bindgen]
pub fn spirograph_turn(track: JsValue, s: f64) -> Result<f64, JsValue> {
    let track = hand::from_js::<mrlyrs::math::spirograph::Track>(&track)?;
    let value = mrlyrs::math::spirograph::turn(&track, s);
    Ok(value)
}

/// Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn three_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::carpet(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.
#[wasm_bindgen]
pub fn three_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Extracts the network of filled sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn three_core_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::core_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::core_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Builds the cube the universe code names, deepened to the given fractal level.
#[wasm_bindgen]
pub fn three_create(code: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::create(code, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.
#[wasm_bindgen]
pub fn three_diagonal_slice(code: JsValue, number: usize, level: usize, base: usize, height: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::diagonal_slice(code, number, level, base, height).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.
#[wasm_bindgen]
pub fn three_diagonal_svg(code: JsValue, number: usize, level: usize, base: usize, heights: &[usize], scale: usize) -> Result<String, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::diagonal_svg(code, number, level, base, heights, scale).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the dust cube, filled where every coordinate is even, at the given level.
#[wasm_bindgen]
pub fn three_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::dust(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Extracts the network of corners and edges outlining every filled site.
#[wasm_bindgen]
pub fn three_edge_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::edge_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::edge_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.
#[wasm_bindgen]
pub fn three_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.
#[wasm_bindgen]
pub fn three_extrude(cell: JsValue, axis: usize, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::three::extrude(&cell, axis, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Repeats every plane of a cube depth times along its leading axis, colors and tags with it.
#[wasm_bindgen]
pub fn three_extrude_cube(cell: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::extrude_cube(&cell, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.
#[wasm_bindgen]
pub fn three_faces(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::faces(&cell);
    Ok(value)
}

/// Returns the count of filled sites.
#[wasm_bindgen]
pub fn three_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::fills(&cell);
    Ok(value)
}

/// Builds a cube from its corner patterns, deepened to the given fractal level.
#[wasm_bindgen]
pub fn three_from_corners(corners: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value = mrlyrs::math::three::from_corners(&corners, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Parses a cell from its JSON, colors and tags included.
#[wasm_bindgen]
pub fn three_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::from_json(text).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cube from one string of digits per row, grouped plane by plane.
#[wasm_bindgen]
pub fn three_from_strings(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<Vec<Vec<String>>>(&data)?;
    let value = mrlyrs::math::three::from_strings(&data).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of faces buried between two filled sites, six per site less the exposed surface.
#[wasm_bindgen]
pub fn three_hidden(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::hidden(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Builds the cube filled wherever the residue sum lands in the levels, at the given level.
#[wasm_bindgen]
pub fn three_level_set(number: usize, levels: &[usize], level: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::level_set(number, levels, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Folds two or more cells into one by chained Kronecker combination.
#[wasm_bindgen]
pub fn three_magic(cells: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Tags every site with its Manhattan distance from the cube's center, the diamond shells.
#[wasm_bindgen]
pub fn three_manhattan_layers(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::manhattan_layers(cell);
    hand::cell3d_to_js(&value)
}

/// Merges the cells into one cube arranged width by height by depth.
#[wasm_bindgen]
pub fn three_merge(cells: JsValue, width: usize, height: usize, depth: usize) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
    let value = mrlyrs::math::three::merge(&cells, width, height, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cell by placing at each mask site the cell its value indexes.
#[wasm_bindgen]
pub fn three_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
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
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Builds the cube the name picks, deepened to the given fractal level.
#[wasm_bindgen]
pub fn three_named(design: JsValue, number: usize, level: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::math::three::named(design, number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the net cube, filled where at least two coordinates are odd, at the given level.
#[wasm_bindgen]
pub fn three_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::net(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds a cube whose every site turns on with probability density, at the given level.
#[wasm_bindgen]
pub fn three_noise(number: usize, level: usize, density: f64, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the solid cube at the given size and level.
#[wasm_bindgen]
pub fn three_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ones(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the 24 rotation triples that reach each distinct cube orientation.
#[wasm_bindgen]
pub fn three_orientations() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::orientations();
    hand::to_js(&value)
}

/// Builds the point cube, filled where every coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn three_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::point(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.
#[wasm_bindgen]
pub fn three_profile(code: JsValue, number: usize, level: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::three::profile(code, number, level, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.
#[wasm_bindgen]
pub fn three_project(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrlyrs::math::three::project(point);
    hand::to_js(&value)
}

/// Returns one outward quad per exposed face, scaled into the unit box.
#[wasm_bindgen]
pub fn three_quads(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::quads(&cell);
    hand::to_js(&value)
}

/// Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.
#[wasm_bindgen]
pub fn three_shadow(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrlyrs::math::three::shadow(point);
    Ok(hand::tuple_to_js(&[JsValue::from(value.0), JsValue::from(value.1)]))
}

/// Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.
#[wasm_bindgen]
pub fn three_slice(cell: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::slice(&cell, axis, index).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Orients a copy of the cell by each mask value and merges them in the mask's shape.
#[wasm_bindgen]
pub fn three_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the star cube, filled where exactly one coordinate is odd, at the given level.
#[wasm_bindgen]
pub fn three_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::star(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the first and last height a profile fills, or none when the design is empty.
#[wasm_bindgen]
pub fn three_support(counts: JsValue) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::u128_from_js)?;
    let value = mrlyrs::math::three::support(&counts);
    hand::to_js(&value)
}

/// Returns the count of filled faces exposed to void or the outside.
#[wasm_bindgen]
pub fn three_surface(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::surface(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.
#[wasm_bindgen]
pub fn three_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| hand::map_from_js::<u8, _>(x1, hand::from_js::<String>))?;
    let value = mrlyrs::math::three::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

/// Serializes the cell's shape and types to JSON, with colors and tags when present.
#[wasm_bindgen]
pub fn three_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_json(&cell);
    Ok(value)
}

/// Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.
#[wasm_bindgen]
pub fn three_to_obj(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_obj(&cell);
    Ok(value)
}

/// Unrolls the cube into one string of digits per row, grouped plane by plane.
#[wasm_bindgen]
pub fn three_to_strings(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::to_strings(&cell);
    hand::to_js(&value)
}

/// Extracts the network of empty sites joined to their axis neighbors.
#[wasm_bindgen]
pub fn three_tunnel_graph(cell: JsValue) -> Result<graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrlyrs::math::three::tunnel_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrlyrs::math::three::tunnel_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!("a 2d or 3d cell was wanted, not {rank}d."))),
    }
}

/// Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.
#[wasm_bindgen]
pub fn three_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::void(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Returns the count of empty sites.
#[wasm_bindgen]
pub fn three_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::voids(&cell);
    Ok(value)
}

/// Returns the filled-site count, the cube's volume.
#[wasm_bindgen]
pub fn three_volume(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::volume(&cell);
    Ok(value)
}

/// Returns the cell's edge-graph segments, scaled into the unit box.
#[wasm_bindgen]
pub fn three_wires(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrlyrs::math::three::wires(&cell);
    hand::list_to_js(&value, |x1| hand::list_to_js(x1, |x2| Ok(JsValue::from(three_Vec3 { inner: *x2 }))))
}

/// Builds the cube of rods along the x axis at the given size and level.
#[wasm_bindgen]
pub fn three_xline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::xline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the x axis at the given size and level.
#[wasm_bindgen]
pub fn three_xtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::xtree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of rods along the y axis at the given size and level.
#[wasm_bindgen]
pub fn three_yline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::yline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the y axis at the given size and level.
#[wasm_bindgen]
pub fn three_ytree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ytree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the all-void cube at the given size and level.
#[wasm_bindgen]
pub fn three_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::zeros(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of rods along the z axis at the given size and level.
#[wasm_bindgen]
pub fn three_zline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::zline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// Builds the cube of beams along the z axis at the given size and level.
#[wasm_bindgen]
pub fn three_ztree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::three::ztree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

/// The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.
#[wasm_bindgen]
pub fn tourbillon_eyes(qmax: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::eyes(qmax);
    hand::to_js(&value)
}

/// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
#[wasm_bindgen]
pub fn tourbillon_field(top: usize, size: usize, schedule: &str, increment: f64, set: &str, weights: &str, mode: &str, blend: &str, seed: u32) -> Result<Vec<f32>, JsValue> {
    let value = mrlyrs::math::tourbillon::field(top, size, schedule, increment, set, weights, mode, blend, seed).map_err(hand::throw)?;
    Ok(value)
}

/// The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.
#[wasm_bindgen]
pub fn tourbillon_layers(top: usize, schedule: &str, increment: f64, set: &str, weights: &str, seed: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::layers(top, schedule, increment, set, weights, seed).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// The least whole number of increments that closes a quarter turn, none once the count passes the cap.
#[wasm_bindgen]
pub fn tourbillon_period(increment: f64) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::period(increment);
    hand::to_js(&value)
}

/// The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.
#[wasm_bindgen]
pub fn tourbillon_sharing(list: JsValue) -> Result<JsValue, JsValue> {
    let list = hand::from_js::<Vec<mrlyrs::math::tourbillon::Layer>>(&list)?;
    let value = mrlyrs::math::tourbillon::sharing(&list);
    hand::to_js(&value)
}

/// Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.
#[wasm_bindgen]
pub fn tourbillon_stack(list: JsValue, size: usize, mode: &str, blend: JsValue) -> Result<Vec<f32>, JsValue> {
    let list = hand::from_js::<Vec<mrlyrs::math::tourbillon::Layer>>(&list)?;
    let blend = hand::from_js::<mrlyrs::math::spin::Blend>(&blend)?;
    let value = mrlyrs::math::tourbillon::stack(&list, size, mode, blend).map_err(hand::throw)?;
    Ok(value)
}

/// Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.
#[wasm_bindgen]
pub fn tourbillon_stats(field: &[f32], size: usize, top: usize, schedule: &str, increment: f64, set: &str, weights: &str, blend: &str, seed: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::tourbillon::stats(field, size, top, schedule, increment, set, weights, blend, seed).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the payload bytes the cell's filled sites can hold, its length header paid for.
#[wasm_bindgen]
pub fn two_capacity(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::capacity(&cell);
    Ok(value)
}

/// Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.
#[wasm_bindgen]
pub fn two_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::carpet(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Takes the cell's full census in one reading.
#[wasm_bindgen]
pub fn two_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Builds the design a universe code names, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn two_create(code: JsValue, number: usize, level: usize, rotation: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrlyrs::math::two::create(code, number, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the dust fractal, its seed on at every even-even site, deepened to the level.
#[wasm_bindgen]
pub fn two_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::dust(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Writes the payload over the cell's filled sites, repeating it until every site is spoken for.
#[wasm_bindgen]
pub fn two_embed(cell: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::embed(&cell, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Returns the Euler characteristic of the filled sites, vertices less edges plus faces.
#[wasm_bindgen]
pub fn two_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

/// Reads the payload back, the plain cell naming the sites the carried one wrote over.
#[wasm_bindgen]
pub fn two_extract(carrier: JsValue, carried: JsValue) -> Result<Vec<u8>, JsValue> {
    let carrier = hand::cell2d_from_js(&carrier)?;
    let carried = hand::cell2d_from_js(&carried)?;
    let value = mrlyrs::math::two::extract(&carrier, &carried).map_err(hand::throw)?;
    Ok(value)
}

/// Counts the filled sites of the cell.
#[wasm_bindgen]
pub fn two_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::fills(&cell);
    Ok(value)
}

/// Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn two_from_corners(corners: JsValue, number: usize, level: usize, rotation: usize, base: usize) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value = mrlyrs::math::two::from_corners(&corners, number, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Restores a cell from its JSON string, colors and tags included.
#[wasm_bindgen]
pub fn two_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::from_json(text).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds a cell from rows of digits, the inverse of the text rendering.
#[wasm_bindgen]
pub fn two_from_strings(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrlyrs::math::two::from_strings(&rows).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the hline fractal, its seed striped along odd rows, deepened to the level.
#[wasm_bindgen]
pub fn two_hline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::hline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the htree fractal, its seed striped along even rows, deepened to the level.
#[wasm_bindgen]
pub fn two_htree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::htree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the level-set design, filling every residue corner whose digits sum to a named level.
#[wasm_bindgen]
pub fn two_level_set(number: usize, levels: &[usize], level: usize, rotation: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::level_set(number, levels, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tiles the mask over the shape and crops it, the perforation pattern itself.
#[wasm_bindgen]
pub fn two_mask(mask: JsValue, shape: &[usize]) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrlyrs::math::two::mask(&mask, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

/// Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.
#[wasm_bindgen]
pub fn two_merge(cells: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
    let value = mrlyrs::math::two::merge(&cells, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the design the name picks, deepened to the level and rotated by quarter-turns.
#[wasm_bindgen]
pub fn two_named(design: JsValue, number: usize, level: usize, rotation: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrlyrs::gen::recipe::Design>(&design)?;
    let value = mrlyrs::math::two::named(design, number, level, rotation).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.
#[wasm_bindgen]
pub fn two_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::net(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds a random cell, each seed site drawn on with probability density, deepened to the level.
#[wasm_bindgen]
pub fn two_noise(number: usize, level: usize, density: f64, rng: &mut hand::Rng) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds an all-filled cell of the given size and level.
#[wasm_bindgen]
pub fn two_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::ones(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// The five by five mask a carried mosaic lays its four tiles out under.
#[wasm_bindgen]
pub fn two_payload_frame() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::payload::frame();
    hand::tensor_to_js(&value)
}

/// Counts the faces of filled sites open to emptiness or the border.
#[wasm_bindgen]
pub fn two_perimeter(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::perimeter(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn two_png(cell: JsValue, scale: usize, outline: JsValue, width: usize, shape: JsValue) -> Result<Vec<u8>, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let shape = hand::from_js::<mrlyrs::math::two::Shape>(&shape)?;
    let value = mrlyrs::math::two::png(&cell, scale, outline, width, shape).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the point fractal, its seed on at every odd-odd site, deepened to the level.
#[wasm_bindgen]
pub fn two_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::point(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Reads the payload back from a framed sheet, the plain fourth cell naming the sites.
#[wasm_bindgen]
pub fn two_read(sheet: JsValue, carrier: JsValue) -> Result<Vec<u8>, JsValue> {
    let sheet = hand::cell2d_from_js(&sheet)?;
    let carrier = hand::cell2d_from_js(&carrier)?;
    let value = mrlyrs::math::two::read(&sheet, &carrier).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the framed sheet of four same-sized cells, the fourth carrying the payload.
#[wasm_bindgen]
pub fn two_sheet(cells: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cells = hand::array_from_js::<_, 4>(&cells, hand::cell2d_from_js)?;
    let value = mrlyrs::math::two::sheet(&cells, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Tiles quarter-turned copies of the cell as the 2d mask directs.
#[wasm_bindgen]
pub fn two_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.
#[wasm_bindgen]
pub fn two_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::star(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.
#[wasm_bindgen]
pub fn two_svg(cell: JsValue, scale: usize, outline: JsValue, width: usize, shape: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let shape = hand::from_js::<mrlyrs::math::two::Shape>(&shape)?;
    let value = mrlyrs::math::two::svg(&cell, scale, outline, width, shape);
    Ok(value)
}

/// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
#[wasm_bindgen]
pub fn two_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| hand::map_from_js::<u8, _>(x1, hand::from_js::<String>))?;
    let value = mrlyrs::math::two::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

/// Lifts the flat cell into a cube one site deep, colors and tags with it.
#[wasm_bindgen]
pub fn two_to_3d(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::to_3d(&cell);
    hand::cell3d_to_js(&value)
}

/// Serializes the cell to a JSON string of its types, with colors and tags when present.
#[wasm_bindgen]
pub fn two_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::to_json(&cell);
    Ok(value)
}

/// Builds the vline fractal, its seed striped along odd columns, deepened to the level.
#[wasm_bindgen]
pub fn two_vline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::vline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.
#[wasm_bindgen]
pub fn two_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::void(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Counts the empty sites of the cell.
#[wasm_bindgen]
pub fn two_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrlyrs::math::two::voids(&cell);
    Ok(value)
}

/// Builds the vtree fractal, its seed striped along even columns, deepened to the level.
#[wasm_bindgen]
pub fn two_vtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::vtree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// Builds an all-empty cell of the given size and level.
#[wasm_bindgen]
pub fn two_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::two::zeros(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

/// The most cells a code walk visits, so that the walk stays within `2^20` codes.
#[wasm_bindgen]
pub fn bang_baseq_WALK_LIMIT() -> Result<usize, JsValue> {
    let value = mrlyrs::math::bang::baseq::WALK_LIMIT;
    Ok(value)
}

/// The five antis of the plane, the complements of the five classics in order.
#[wasm_bindgen]
pub fn bang_catalog_ANTIS_2D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::ANTIS_2D;
    hand::to_js(&value)
}

/// The six antis of the cube: point, dust, the three lines and the star.
#[wasm_bindgen]
pub fn bang_catalog_ANTIS_3D() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::bang::catalog::ANTIS_3D;
    hand::to_js(&value)
}

/// The widest side a profile spans.
#[wasm_bindgen]
pub fn counts_diagonal_WIDEST() -> Result<usize, JsValue> {
    let value = mrlyrs::math::counts::diagonal::WIDEST;
    Ok(value)
}

/// The largest corner count the tally press accepts, keeping its table a million rows.
#[wasm_bindgen]
pub fn press_CORNERS() -> Result<usize, JsValue> {
    let value = mrlyrs::math::press::CORNERS;
    Ok(value)
}

/// The default residue base.
#[wasm_bindgen]
pub fn rules_BASE() -> Result<usize, JsValue> {
    let value = mrlyrs::math::rules::BASE;
    Ok(value)
}

/// The refine output ceiling in cells.
#[wasm_bindgen]
pub fn shape_REFINE_LIMIT() -> Result<usize, JsValue> {
    let value = mrlyrs::math::shape::REFINE_LIMIT;
    Ok(value)
}

/// The triangle code for a filled site.
#[wasm_bindgen]
pub fn six_FILL() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::FILL;
    Ok(value)
}

/// The triangle code for the backdrop outside the figure.
#[wasm_bindgen]
pub fn six_GRID() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::GRID;
    Ok(value)
}

/// The triangle code for a cube's left face in the iso view.
#[wasm_bindgen]
pub fn six_LEFT() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::LEFT;
    Ok(value)
}

/// The triangle code for a cube's right face in the iso view.
#[wasm_bindgen]
pub fn six_RIGHT() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::RIGHT;
    Ok(value)
}

/// The triangle code for a cube's top face in the iso view.
#[wasm_bindgen]
pub fn six_UP() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::UP;
    Ok(value)
}

/// The triangle code for an empty site.
#[wasm_bindgen]
pub fn six_VOID() -> Result<u8, JsValue> {
    let value = mrlyrs::math::six::VOID;
    Ok(value)
}

/// The most laps of a line or a polygon.
#[wasm_bindgen]
pub fn spirograph_LAPS_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::LAPS_CAP;
    Ok(value)
}

/// The most pencils a wheel seats.
#[wasm_bindgen]
pub fn spirograph_PENCIL_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::PENCIL_CAP;
    Ok(value)
}

/// The most points one trace returns.
#[wasm_bindgen]
pub fn spirograph_POINT_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::POINT_CAP;
    Ok(value)
}

/// The largest radius of a ring or a wheel.
#[wasm_bindgen]
pub fn spirograph_RADIUS_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::RADIUS_CAP;
    Ok(value)
}

/// The largest raster side a cover rasters.
#[wasm_bindgen]
pub fn spirograph_RASTER_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::math::spirograph::RASTER_CAP;
    Ok(value)
}

/// The fewest and the most sides of a polygon track.
#[wasm_bindgen]
pub fn spirograph_SIDES() -> Result<JsValue, JsValue> {
    let value = mrlyrs::math::spirograph::SIDES;
    hand::to_js(&value)
}

/// A single design with its place in the orbit structure.
#[wasm_bindgen]
pub struct bang_Design {
    inner: mrlyrs::math::bang::Design,
}

#[wasm_bindgen]
impl bang_Design {
    /// Reads the Design from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<bang_Design, JsValue> {
        Ok(bang_Design { inner: hand::from_js(&data)? })
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
pub struct bang_Universe {
    inner: mrlyrs::math::bang::Universe,
}

#[wasm_bindgen]
impl bang_Universe {
    /// Reads the Universe from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<bang_Universe, JsValue> {
        Ok(bang_Universe { inner: hand::from_js(&data)? })
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
        hand::list_to_js(&value, |x1| Ok(JsValue::from(bang_Design { inner: x1.clone() })))
    }
    /// Returns the designs whose codes lead their orbits.
    pub fn canonical(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.canonical();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(bang_Design { inner: x1.clone() })))
    }
    /// Returns the design at a code with its precomputed orbit facts.
    pub fn design(&self, code: JsValue) -> Result<bang_Design, JsValue> {
        let code = hand::code_from_js(&code)?;
        let value = self.inner.design(code);
        Ok(bang_Design { inner: value })
    }
    /// Returns the number of distinct orbits.
    pub fn distinct(&self) -> Result<usize, JsValue> {
        let value = self.inner.distinct();
        Ok(value)
    }
    /// Enumerates every orbit of a dimension from 1 to 4.
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize) -> Result<bang_Universe, JsValue> {
        let value = mrlyrs::math::bang::Universe::new(dimension).map_err(hand::throw)?;
        Ok(bang_Universe { inner: value })
    }
}

/// The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions.
#[wasm_bindgen]
pub struct counts_Exposure {
    inner: mrlyrs::math::counts::Exposure,
}

#[wasm_bindgen]
impl counts_Exposure {
    /// Reads the Exposure from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<counts_Exposure, JsValue> {
        Ok(counts_Exposure { inner: hand::from_js(&data)? })
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
        hand::list_to_js(&value, |x1| Ok(hand::tuple_to_js(&[JsValue::from_str(&x1.0.to_string()), JsValue::from_str(&x1.1.to_string())])))
    }
    #[wasm_bindgen(setter)]
    pub fn set_axes(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, |x1| Ok((hand::u128_from_js(&hand::item(x1, 0)?)?, hand::u128_from_js(&hand::item(x1, 1)?)?)))?;
        self.inner.axes = value;
        Ok(())
    }
    /// Returns the exposed faces of the level-fold Kronecker power, or none past a u128.
    pub fn at(&self, level: u32) -> Result<JsValue, JsValue> {
        let value = self.inner.at(level);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    /// Folds the counts from the filled residue corners at a side number, without rendering the tile.
    pub fn from_corners(filled: JsValue, number: usize, dimension: usize, base: usize) -> Result<counts_Exposure, JsValue> {
        let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
        let value = mrlyrs::math::counts::Exposure::from_corners(&filled, number, dimension, base);
        Ok(counts_Exposure { inner: value })
    }
    /// Reads the counts off a rendered tile.
    pub fn of_tile(tile: JsValue) -> Result<counts_Exposure, JsValue> {
        let tile = hand::tensor_from_js(&tile)?;
        let value = mrlyrs::math::counts::Exposure::of_tile(&tile);
        Ok(counts_Exposure { inner: value })
    }
    /// Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.
    pub fn recurrence(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.recurrence();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
}

/// A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle.
#[wasm_bindgen]
pub struct graph_Layout {
    inner: mrlyrs::math::graph::Layout,
}

#[wasm_bindgen]
impl graph_Layout {
    /// Returns the mean net force per node in units of `k` after the last tick.
    pub fn energy(&self) -> Result<f64, JsValue> {
        let value = self.inner.energy();
        Ok(value)
    }
    /// Starts a layout from a network's own positions and branches.
    pub fn from_network(network: &graph_Network, seed: JsValue) -> Result<graph_Layout, JsValue> {
        let seed = hand::u64_from_js(&seed)?;
        let value = mrlyrs::math::graph::Layout::from_network(&network.inner, seed).map_err(hand::throw)?;
        Ok(graph_Layout { inner: value })
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
    pub fn new(positions: &[f64], branches: JsValue, dim: usize, seed: JsValue) -> Result<graph_Layout, JsValue> {
        let branches = hand::from_js::<Vec<(usize, usize)>>(&branches)?;
        let seed = hand::u64_from_js(&seed)?;
        let value = mrlyrs::math::graph::Layout::new(positions, &branches, dim, seed).map_err(hand::throw)?;
        Ok(graph_Layout { inner: value })
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
pub struct graph_Network {
    inner: mrlyrs::math::graph::Network,
}

#[wasm_bindgen]
impl graph_Network {
    /// Reads the Network from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<graph_Network, JsValue> {
        Ok(graph_Network { inner: hand::from_js(&data)? })
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
        self.inner.add_branch(parent, child, radius).map_err(hand::throw)?;
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
    pub fn new(dim: usize) -> Result<graph_Network, JsValue> {
        let value = mrlyrs::math::graph::Network::new(dim);
        Ok(graph_Network { inner: value })
    }
}

/// A square grid of f32 samples.
#[wasm_bindgen]
pub struct moire_Field {
    inner: mrlyrs::math::moire::Field,
}

#[wasm_bindgen]
impl moire_Field {
    /// Reads the Field from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<moire_Field, JsValue> {
        Ok(moire_Field { inner: hand::from_js(&data)? })
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
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<moire_Field, JsValue> {
        let value = mrlyrs::math::moire::Field::from_data(data, size).map_err(hand::throw)?;
        Ok(moire_Field { inner: value })
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
    pub fn new(size: usize) -> Result<moire_Field, JsValue> {
        let value = mrlyrs::math::moire::Field::new(size);
        Ok(moire_Field { inner: value })
    }
    /// Returns the samples scaled into 0..1, symmetric about zero on request.
    pub fn normalized(&self, symmetric: bool) -> Result<Vec<f32>, JsValue> {
        let value = self.inner.normalized(symmetric);
        Ok(value)
    }
}

/// One named moire recipe: the design, the scales it stacks and the lattice it samples.
#[wasm_bindgen]
pub struct moire_Preset {
    inner: mrlyrs::math::moire::Preset,
}

#[wasm_bindgen]
impl moire_Preset {
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
    pub fn carpet(limit: usize) -> Result<moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::carpet(limit);
        Ok(moire_Preset { inner: value })
    }
    /// Samples the preset into a square field of the given side.
    pub fn field(&self, size: usize) -> Result<moire_Field, JsValue> {
        let value = self.inner.field(size).map_err(hand::throw)?;
        Ok(moire_Field { inner: value })
    }
    /// The parity heatmap: odd scales of the low corner summed on the square lattice.
    pub fn heatmap(limit: usize) -> Result<moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::heatmap(limit);
        Ok(moire_Preset { inner: value })
    }
    /// The hive: the parity heatmap sampled on the hexagonal lattice.
    pub fn hive(limit: usize) -> Result<moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::hive(limit);
        Ok(moire_Preset { inner: value })
    }
    /// The parity weave: the same odd scales folded to their parity instead of summed.
    pub fn weave(limit: usize) -> Result<moire_Preset, JsValue> {
        let value = mrlyrs::math::moire::Preset::weave(limit);
        Ok(moire_Preset { inner: value })
    }
}

/// A cubic grid of f32 samples, x-major.
#[wasm_bindgen]
pub struct moire_Volume {
    inner: mrlyrs::math::moire::Volume,
}

#[wasm_bindgen]
impl moire_Volume {
    /// Reads the Volume from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<moire_Volume, JsValue> {
        Ok(moire_Volume { inner: hand::from_js(&data)? })
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
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<moire_Volume, JsValue> {
        let value = mrlyrs::math::moire::Volume::from_data(data, size).map_err(hand::throw)?;
        Ok(moire_Volume { inner: value })
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
    pub fn new(size: usize) -> Result<moire_Volume, JsValue> {
        let value = mrlyrs::math::moire::Volume::new(size);
        Ok(moire_Volume { inner: value })
    }
    /// Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.
    pub fn plane(&self, frame: JsValue, out: usize) -> Result<JsValue, JsValue> {
        let frame = hand::from_js::<mrlyrs::math::moire::Frame>(&frame)?;
        let value = self.inner.plane(&frame, out);
        Ok(hand::tuple_to_js(&[hand::typed(&(value.0)[..]), hand::typed(&(value.1)[..])]))
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
pub struct name_Bang {
    inner: mrlyrs::math::name::Bang,
}

#[wasm_bindgen]
impl name_Bang {
    /// Reads the Bang from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<name_Bang, JsValue> {
        Ok(name_Bang { inner: hand::from_js(&data)? })
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
    pub fn checked(&self) -> Result<name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::checked(self.inner.clone()).map_err(hand::throw)?;
        Ok(name_Bang { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(name_Bang { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(name_Bang { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<name_Bang, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(name_Bang { inner: value })
    }
    /// Pins a code to its dimension and base on the square lattice.
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue, dim: usize, base: usize) -> Result<name_Bang, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::name::Bang::new(code, dim, base);
        Ok(name_Bang { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_file(&self.inner).map_err(hand::throw)?;
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
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_mrly(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_url(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
}

/// A design sequence's address: the design, the reading taken off it and the index it runs along.
#[wasm_bindgen]
pub struct name_Sequence {
    inner: mrlyrs::math::name::Sequence,
}

#[wasm_bindgen]
impl name_Sequence {
    /// Reads the Sequence from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<name_Sequence, JsValue> {
        Ok(name_Sequence { inner: hand::from_js(&data)? })
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
    pub fn checked(&self) -> Result<name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::checked(self.inner.clone()).map_err(hand::throw)?;
        Ok(name_Sequence { inner: value })
    }
    /// Returns the design pinned to its dimension and base.
    pub fn design(&self) -> Result<name_Bang, JsValue> {
        let value = self.inner.design();
        Ok(name_Bang { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(name_Sequence { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(name_Sequence { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<name_Sequence, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(name_Sequence { inner: value })
    }
    /// Pins a design's reading to its measure and axis.
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue, dim: usize, base: usize, measure: &str, axis: &str) -> Result<name_Sequence, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::name::Sequence::new(code, dim, base, measure, axis);
        Ok(name_Sequence { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_file(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    /// Prints the canonical JSON object.
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    pub fn to_mrly(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_mrly(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_url(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
}

/// A magic word: an ordered list of design letters, first letter outermost, each at its own side.
#[wasm_bindgen]
pub struct name_Word {
    inner: mrlyrs::math::name::Word,
}

#[wasm_bindgen]
impl name_Word {
    /// Reads the Word from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<name_Word, JsValue> {
        Ok(name_Word { inner: hand::from_js(&data)? })
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
    pub fn checked(&self) -> Result<name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::checked(self.inner.clone()).map_err(hand::throw)?;
        Ok(name_Word { inner: value })
    }
    /// Reads a filename back into the value, or an error.
    pub fn from_file(text: &str) -> Result<name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_file(text).map_err(hand::throw)?;
        Ok(name_Word { inner: value })
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    pub fn from_json(text: &str) -> Result<name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_json(text).map_err(hand::throw)?;
        Ok(name_Word { inner: value })
    }
    /// Reads a path and query string back into the value, or an error.
    pub fn from_url(text: &str) -> Result<name_Word, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_url(text).map_err(hand::throw)?;
        Ok(name_Word { inner: value })
    }
    /// Returns every letter as a design pinned to the word's dimension and its own base.
    pub fn letters(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.letters();
        hand::list_to_js(&value, |x1| Ok(JsValue::from(name_Bang { inner: x1.clone() })))
    }
    /// Pins an ordered letter list at base 2.
    #[wasm_bindgen(constructor)]
    pub fn new(dim: usize, letters: JsValue) -> Result<name_Word, JsValue> {
        let letters = hand::list_from_js(&letters, |x1| Ok((hand::u128_from_js(&hand::item(x1, 0)?)?, hand::from_js::<usize>(&hand::item(x1, 1)?)?)))?;
        let value = mrlyrs::math::name::Word::new(dim, &letters).map_err(hand::throw)?;
        Ok(name_Word { inner: value })
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_file(&self.inner).map_err(hand::throw)?;
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
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_mrly(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_url(&self.inner).map_err(hand::throw)?;
        Ok(value)
    }
}

/// The tally press: one pass over the integers weighs every design of a universe at once.
#[wasm_bindgen]
pub struct press_Press {
    inner: mrlyrs::math::press::Press,
}

#[wasm_bindgen]
impl press_Press {
    /// Reads the Press from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<press_Press, JsValue> {
        Ok(press_Press { inner: hand::from_js(&data)? })
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
    pub fn new(dimension: usize, base: usize) -> Result<press_Press, JsValue> {
        let value = mrlyrs::math::press::Press::new(dimension, base).map_err(hand::throw)?;
        Ok(press_Press { inner: value })
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
pub struct roulette_Nodes {
    inner: mrlyrs::math::roulette::Nodes,
}

#[wasm_bindgen]
impl roulette_Nodes {
    /// Reads the Nodes from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<roulette_Nodes, JsValue> {
        Ok(roulette_Nodes { inner: hand::from_js(&data)? })
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
    pub fn default_() -> Result<roulette_Nodes, JsValue> {
        let value = mrlyrs::math::roulette::Nodes::default();
        Ok(roulette_Nodes { inner: value })
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
pub struct shape_Frac {
    inner: mrlyrs::math::shape::Frac,
}

#[wasm_bindgen]
impl shape_Frac {
    /// Reads the Frac from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<shape_Frac, JsValue> {
        Ok(shape_Frac { inner: hand::from_js(&data)? })
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
    pub fn minus(&self, other: &shape_Frac) -> Result<shape_Frac, JsValue> {
        let value = self.inner.minus(other.inner).map_err(hand::throw)?;
        Ok(shape_Frac { inner: value })
    }
    /// Builds the reduced fraction num over den.
    #[wasm_bindgen(constructor)]
    pub fn new(num: JsValue, den: JsValue) -> Result<shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let den = hand::i64_from_js(&den)?;
        let value = mrlyrs::math::shape::Frac::new(num, den).map_err(hand::throw)?;
        Ok(shape_Frac { inner: value })
    }
    /// Returns the exact sum.
    pub fn plus(&self, other: &shape_Frac) -> Result<shape_Frac, JsValue> {
        let value = self.inner.plus(other.inner).map_err(hand::throw)?;
        Ok(shape_Frac { inner: value })
    }
    /// Returns the exact product.
    pub fn times(&self, other: &shape_Frac) -> Result<shape_Frac, JsValue> {
        let value = self.inner.times(other.inner).map_err(hand::throw)?;
        Ok(shape_Frac { inner: value })
    }
    /// Wraps an integer as a fraction over one.
    pub fn whole(num: JsValue) -> Result<shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let value = mrlyrs::math::shape::Frac::whole(num);
        Ok(shape_Frac { inner: value })
    }
}

/// The exact reading of a cut layer: how many cells were inked out of how many were read.
#[wasm_bindgen]
pub struct six_star_Share {
    inner: mrlyrs::math::six::star::Share,
}

#[wasm_bindgen]
impl six_star_Share {
    /// Reads the Share from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<six_star_Share, JsValue> {
        Ok(six_star_Share { inner: hand::from_js(&data)? })
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
        Ok(hand::tuple_to_js(&[JsValue::from(value.0), JsValue::from(value.1)]))
    }
    /// The share as a real number.
    pub fn value(&self) -> Result<f64, JsValue> {
        let value = self.inner.value();
        Ok(value)
    }
}

/// The ghost star of a coded cube's hexagonal cut stack, read in the cell frame.
#[wasm_bindgen]
pub struct six_star_Star {
    inner: mrlyrs::math::six::star::Star,
}

#[wasm_bindgen]
impl six_star_Star {
    /// Reads the Star from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<six_star_Star, JsValue> {
        Ok(six_star_Star { inner: hand::from_js(&data)? })
    }
    /// Writes the Star as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.
    pub fn arm(&self, number: usize, half: usize) -> Result<six_star_Share, JsValue> {
        let value = self.inner.arm(number, half).map_err(hand::throw)?;
        Ok(six_star_Share { inner: value })
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
    pub fn hexagon(&self, number: usize) -> Result<six_star_Share, JsValue> {
        let value = self.inner.hexagon(number).map_err(hand::throw)?;
        Ok(six_star_Share { inner: value })
    }
    /// Reads the star of a base-2 space code, the carpet being `23`.
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue) -> Result<six_star_Star, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::math::six::star::Star::new(code).map_err(hand::throw)?;
        Ok(six_star_Star { inner: value })
    }
}

/// A three-component vector of f32.
#[wasm_bindgen]
pub struct three_Vec3 {
    inner: mrlyrs::math::three::Vec3,
}

#[wasm_bindgen]
impl three_Vec3 {
    /// Reads the Vec3 from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<three_Vec3, JsValue> {
        Ok(three_Vec3 { inner: hand::from_js(&data)? })
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
    pub fn cross(&self, o: &three_Vec3) -> Result<three_Vec3, JsValue> {
        let value = self.inner.cross(o.inner);
        Ok(three_Vec3 { inner: value })
    }
    /// Returns the dot product of the two vectors.
    pub fn dot(&self, o: &three_Vec3) -> Result<f32, JsValue> {
        let value = self.inner.dot(o.inner);
        Ok(value)
    }
    /// Builds a vector from its components.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Result<three_Vec3, JsValue> {
        let value = mrlyrs::math::three::Vec3::new(x, y, z);
        Ok(three_Vec3 { inner: value })
    }
    /// Multiplies every component by the scalar.
    pub fn scale(&self, s: f32) -> Result<three_Vec3, JsValue> {
        let value = self.inner.scale(s);
        Ok(three_Vec3 { inner: value })
    }
}

/// One ordered layer of a magic composition: a coded design at its own side number.
#[wasm_bindgen]
pub struct bang_MagicLayer {}

#[wasm_bindgen]
impl bang_MagicLayer {
    /// Pins a design to the side number it renders at.
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(design: &name_Bang, number: usize) -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::bang::MagicLayer::new(design.inner.clone(), number);
        hand::to_js(&value)
    }
}

/// The named infinite schedules over an ordered pair of letters.
#[wasm_bindgen]
pub struct bang_word_Schedule {}

#[wasm_bindgen]
impl bang_word_Schedule {
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
pub struct moire_Layer {}

#[wasm_bindgen]
impl moire_Layer {
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
pub struct moire_Spec {}

#[wasm_bindgen]
impl moire_Spec {
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
pub struct name_Lattice {}

#[wasm_bindgen]
impl name_Lattice {
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
pub struct shape_Region {}

#[wasm_bindgen]
impl shape_Region {
    /// Swaps In and Out, keeping Cut.
    pub fn flip(region: JsValue) -> Result<JsValue, JsValue> {
        let region = hand::from_js::<mrlyrs::math::shape::Region>(&region)?;
        let value = region.flip();
        hand::to_js(&value)
    }
}

/// The three classes of layer count the `1/L^2` term of the decay reads.
#[wasm_bindgen]
pub struct six_star_Branch {}

#[wasm_bindgen]
impl six_star_Branch {
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
pub struct spin_Blend {}

#[wasm_bindgen]
impl spin_Blend {
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
pub struct spirograph_Seats {}

#[wasm_bindgen]
impl spirograph_Seats {
    /// Returns the default Seats.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrlyrs::math::spirograph::Seats::default();
        hand::to_js(&value)
    }
}
