#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_math::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn math_arcs_curves(code: JsValue, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::arcs::curves(code, base, level).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_arcs_draw(code: JsValue, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::arcs::draw(code, base, level).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_arcs_law(code: JsValue, base: usize, level: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::arcs::law(code, base, level).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_arcs_trace(side: usize, on: JsValue) -> Result<JsValue, JsValue> {
    let on = hand::from_js::<Vec<bool>>(&on)?;
    let value = mrly_math::arcs::trace(side, &on).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_arrays_essential(sensors: JsValue) -> Result<JsValue, JsValue> {
    let sensors = hand::list_from_js(&sensors, hand::u64_from_js)?;
    let value = mrly_math::arrays::essential(&sensors);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_arrays_fractal(
    generator: JsValue,
    base: JsValue,
    level: u32,
) -> Result<Vec<u64>, JsValue> {
    let generator = hand::list_from_js(&generator, hand::u64_from_js)?;
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::arrays::fractal(&generator, base, level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_arrays_holes(sensors: JsValue) -> Result<Vec<i64>, JsValue> {
    let sensors = hand::list_from_js(&sensors, hand::u64_from_js)?;
    let value = mrly_math::arrays::holes(&sensors);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_arrays_law(generator: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let generator = hand::list_from_js(&generator, hand::u64_from_js)?;
    let value = mrly_math::arrays::law(&generator, level);
    hand::option_to_js(value.as_ref(), |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_arrays_lost(sensors: JsValue, sensor: JsValue) -> Result<Vec<i64>, JsValue> {
    let sensors = hand::list_from_js(&sensors, hand::u64_from_js)?;
    let sensor = hand::u64_from_js(&sensor)?;
    let value = mrly_math::arrays::lost(&sensors, sensor);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_arrays_paired(sensors: JsValue) -> Result<Vec<u64>, JsValue> {
    let sensors = hand::list_from_js(&sensors, hand::u64_from_js)?;
    let value = mrly_math::arrays::paired(&sensors);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_arrays_weights(sensors: JsValue) -> Result<Vec<u64>, JsValue> {
    let sensors = hand::list_from_js(&sensors, hand::u64_from_js)?;
    let value = mrly_math::arrays::weights(&sensors);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_atoms_carpet_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::carpet_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_carpet_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::carpet_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_carpet_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::carpet_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_dust_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::dust_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_dust_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::dust_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_dust_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::dust_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_hline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::hline_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_htree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::htree_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_line_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::line_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_net_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::net_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_net_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::net_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_net_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::net_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_noise_2d(
    n: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::noise_2d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_noise_3d(
    n: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::noise_3d(n, density, rng.stream());
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_ones_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::ones_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_ones_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::ones_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_point_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::point_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_point_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::point_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_point_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::point_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_star_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::star_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_star_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::star_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_star_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::star_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_tree_nd(n: usize, rank: usize, axis: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::tree_nd(n, rank, axis);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_vline_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::vline_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_void_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::void_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_void_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::void_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_void_nd(n: usize, rank: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::void_nd(n, rank);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_vtree_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::vtree_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_xline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::xline_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_xtree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::xtree_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_yline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::yline_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_ytree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::ytree_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_zeros_2d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::zeros_2d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_zeros_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::zeros_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_zline_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::zline_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_atoms_ztree_3d(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::atoms::ztree_3d(n);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_bang(dimension: usize) -> Result<math_bang_Universe, JsValue> {
    let value = mrly_math::bang::bang(dimension).map_err(hand::throw)?;
    Ok(math_bang_Universe { inner: value })
}

#[wasm_bindgen]
pub fn math_bang_baseq_axis_maps(base: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::axis_maps(base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_baseq_bracelets(max_base: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::bracelets(max_base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_bang_baseq_canonical(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::baseq::canonical(&group, code).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_baseq_carry(element: &[usize], code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::baseq::carry(element, code);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_baseq_class_sequence(max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::class_sequence(max_dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_bang_baseq_classes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::classes(dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_distinct_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::distinct_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_even_fill_is_balanced(
    number: usize,
    dimension: usize,
    popcount: JsValue,
) -> Result<JsValue, JsValue> {
    let popcount = hand::u128_from_js(&popcount)?;
    let value = mrly_math::bang::baseq::even_fill_is_balanced(number, dimension, popcount)
        .map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_fill_from_corners(
    filled: JsValue,
    number: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrly_math::bang::baseq::fill_from_corners(&filled, number, dimension);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_group(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::group(base, dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_baseq_group_order(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_orbit(group: JsValue, code: JsValue) -> Result<JsValue, JsValue> {
    let group = hand::from_js::<Vec<Vec<usize>>>(&group)?;
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::baseq::orbit(&group, code);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

#[wasm_bindgen]
pub fn math_bang_baseq_predicted_group_order(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrly_math::bang::baseq::predicted_group_order(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_baseq_representatives(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::representatives(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&hand::code_to_js(x1.0)),
            hand::to_js(&x1.1)?,
        ]))
    })
}

#[wasm_bindgen]
pub fn math_bang_baseq_sequence(base: usize, max_dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::sequence(base, max_dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_bang_baseq_total_designs(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::baseq::total_designs(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_catalog_antis(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::antis(dimension);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_catalog_classics(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::classics(dimension);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_code_get(code: JsValue) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = code.get();
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_code_to_corners(
    code: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::code_to_corners(code, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_corners(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::corners(dimension);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_corners_to_code(
    filled: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrly_math::bang::corners_to_code(&filled, dimension, base);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_factory_create(
    code: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::factory::create(code, number, dimension, base, level)
        .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

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
        mrly_math::bang::factory::create_from_corners(&filled, number, dimension, base, level)
            .map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_factory_create_named(
    spec: &str,
    number: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::factory::create_named(spec, number, level).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_factory_residue_corners(
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::factory::residue_corners(dimension, base);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_factory_total_codes(dimension: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::factory::total_codes(dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_level_cap(
    number: usize,
    dimension: usize,
    budget: usize,
) -> Result<usize, JsValue> {
    let value = mrly_math::bang::level_cap(number, dimension, budget);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_levels_code(
    dimension: usize,
    base: usize,
    levels: &[usize],
) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::levels_code(dimension, base, levels);
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_magic(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::magic(&layers).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_magic_named(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<(String, usize)>>(&layers)?;
    let layers_view: Vec<(&str, usize)> =
        layers.iter().map(|(a0, a1)| (a0.as_str(), *a1)).collect();
    let value = mrly_math::bang::magic_named(&layers_view).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_random_code(
    dimension: usize,
    base: usize,
    rng: &mut hand::Rng,
    min_fills: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::random_code(dimension, base, rng.stream(), min_fills)
        .map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_bang_sources(catalog: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let catalog = hand::from_js::<mrly_math::bang::catalog::Catalog>(&catalog)?;
    let value = mrly_math::bang::sources(&catalog, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_symmetries(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::symmetries(dimension);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            hand::typed(&(x1.0)[..]),
            hand::typed(&(x1.1)[..]),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_bang_total_exposure(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::total_exposure(code, dimension);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_touches_every_corner(code: JsValue, dimension: usize) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::touches_every_corner(code, dimension);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_anf(code: JsValue, dimension: usize) -> Result<Vec<u8>, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::universe::anf(code, dimension);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_anf_string(code: JsValue, dimension: usize) -> Result<String, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::universe::anf_string(code, dimension);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_apply(element: JsValue, corner: &[u8]) -> Result<Vec<u8>, JsValue> {
    let element = hand::from_js::<(Vec<usize>, Vec<u8>)>(&element)?;
    let value = mrly_math::bang::universe::apply(&element, corner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_corner_index(corner: &[u8]) -> Result<usize, JsValue> {
    let value = mrly_math::bang::universe::corner_index(corner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_degree(code: JsValue, dimension: usize) -> Result<i32, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::universe::degree(code, dimension);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_universe_orbit(code: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::bang::universe::orbit(code, dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&hand::code_to_js(*x1))))
}

#[wasm_bindgen]
pub fn math_bang_universe_permutations(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::universe::permutations(n);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_bang_universe_codes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::universe_codes(dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_bang_word_components(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::components(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_word_constant_functional(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::constant_functional(&layers).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_word_dimension(layers: JsValue) -> Result<f64, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::dimension(&layers).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_word_fill(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::fill(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_word_fills(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::fills(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_bang_word_letter(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrly_math::bang::MagicLayer>(&layer)?;
    let value = mrly_math::bang::word::letter(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_word_native(layers: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::native(&layers);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_word_period(layers: JsValue) -> Result<usize, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::period(&layers);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_word_prefixes(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::prefixes(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_word_rates(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::rates(&layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_word_side(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::bang::word::side(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_bang_word_spell(
    schedule: JsValue,
    pair: JsValue,
    length: usize,
) -> Result<JsValue, JsValue> {
    let schedule = hand::from_js::<mrly_math::bang::word::Schedule>(&schedule)?;
    let pair = hand::from_js::<(mrly_math::bang::MagicLayer, mrly_math::bang::MagicLayer)>(&pair)?;
    let value = mrly_math::bang::word::spell(schedule, pair, length);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_word_staircase(depth: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::word::staircase(depth).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_word_thue_morse(index: usize) -> Result<usize, JsValue> {
    let value = mrly_math::bang::word::thue_morse(index);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_cell_census_edges(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::cell::census::edges::<2>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::census::edges::<3>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_census_exposure(cell: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::cell::census::exposure::<2>(&cell);
            Ok(JsValue::from_str(&value.to_string()))
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::census::exposure::<3>(&cell);
            Ok(JsValue::from_str(&value.to_string()))
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_census_fills(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::cell::census::fills::<2>(&cell);
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::census::fills::<3>(&cell);
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_census_vertices(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::cell::census::vertices::<2>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::census::vertices::<3>(&cell).map_err(hand::throw)?;
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_census_voids(cell: JsValue) -> Result<usize, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::cell::census::voids::<2>(&cell);
            Ok(value)
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::census::voids::<3>(&cell);
            Ok(value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_geometry_merge_reps(cells: JsValue, reps: &[usize]) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value =
                mrly_math::cell::geometry::merge_reps::<2>(&cells, reps).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value =
                mrly_math::cell::geometry::merge_reps::<3>(&cells, reps).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

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
            let value = mrly_math::cell::geometry::perforate::<2>(&mask, &cell, value)
                .map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::cell::geometry::perforate::<3>(&mask, &cell, value)
                .map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_cell_grow(pattern: JsValue, level: usize) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&pattern)? {
        2 => {
            let pattern = hand::tensor_from_js(&pattern)?;
            let value = mrly_math::cell::grow::<2>(pattern, level).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let pattern = hand::tensor_from_js(&pattern)?;
            let value = mrly_math::cell::grow::<3>(pattern, level).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

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

#[wasm_bindgen]
pub fn math_cell_models_counting_dtype(mask: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_math::cell::models::counting_dtype(&mask);
    hand::to_js(&value)
}

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

#[wasm_bindgen]
pub fn math_cell_models_dtype_for(peak: JsValue) -> Result<JsValue, JsValue> {
    let peak = hand::i64_from_js(&peak)?;
    let value = mrly_math::cell::models::dtype_for(peak);
    hand::to_js(&value)
}

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

#[wasm_bindgen]
pub fn math_cell_models_new(types: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&types)? {
        2 => {
            let types = hand::tensor_from_js(&types)?;
            let value = mrly_math::cell::models::CellNd::<2>::new(types).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let types = hand::tensor_from_js(&types)?;
            let value = mrly_math::cell::models::CellNd::<3>::new(types).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

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
            let mode = hand::from_js::<mrly_core::Mode>(&mode)?;
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
            let mode = hand::from_js::<mrly_core::Mode>(&mode)?;
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
            let mode = hand::from_js::<Option<mrly_core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value =
                mrly_math::cell::paint::<2>(cell, custom.as_ref(), mode, rng_stream.as_mut())
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
            let mode = hand::from_js::<Option<mrly_core::Mode>>(&mode)?;
            let mut rng_stream = hand::stream_from_js(&rng)?;
            let value =
                mrly_math::cell::paint::<3>(cell, custom.as_ref(), mode, rng_stream.as_mut())
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

#[wasm_bindgen]
pub fn math_cell_serializer_byte_cube(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrly_core::Json>(&value)?;
    let value = mrly_math::cell::serializer::byte_cube(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..])))
    })
}

#[wasm_bindgen]
pub fn math_cell_serializer_byte_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrly_core::Json>(&value)?;
    let value = mrly_math::cell::serializer::byte_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_cell_serializer_color_grid(value: JsValue) -> Result<JsValue, JsValue> {
    let value = hand::from_js::<mrly_core::Json>(&value)?;
    let value = mrly_math::cell::serializer::color_grid(&value).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(hand::typed(&(*x2)[..])))
    })
}

#[wasm_bindgen]
pub fn math_cell_serializer_count_cube(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrly_core::Json>(&value)?;
    let value = mrly_math::cell::serializer::count_cube(&value).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_cell_serializer_count_grid(value: JsValue) -> Result<Vec<i64>, JsValue> {
    let value = hand::from_js::<mrly_core::Json>(&value)?;
    let value = mrly_math::cell::serializer::count_grid(&value).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_cell_serializer_parse(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_math::cell::serializer::parse(text).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_cell_serializer_tag_layer(
    counts: JsValue,
    shape: Vec<usize>,
) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::i64_from_js)?;
    let value = mrly_math::cell::serializer::tag_layer(&counts, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_cell_serializer_types_field(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<mrly_core::Json>(&data)?;
    let value = mrly_math::cell::serializer::types_field(&data).map_err(hand::throw)?;
    hand::to_js(value)
}

#[wasm_bindgen]
pub fn math_coprime_gasket(level: u32) -> Result<u64, JsValue> {
    let value = mrly_math::coprime::gasket(level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_centered_hexagonal(m: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::centered_hexagonal(m);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_cut_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::cut_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_cut_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::cut_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_dimension(
    code: JsValue,
    number: usize,
    base_dimension: usize,
    base: usize,
) -> Result<f64, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrly_math::counts::dimension(code, number, base_dimension, base).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_edges_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::edges_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_exposure(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrly_math::counts::exposure(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_exposure_of_tile_(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::exposure_of_tile(&tile, level);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_exposure_recurrence_(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::exposure_recurrence(&tile);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

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
        mrly_math::counts::fill(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_fill_from_corners(
    filled: JsValue,
    number: usize,
    _dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrly_math::counts::fill_from_corners(&filled, number, _dimension, level, base);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_fill_polynomial(corners: JsValue, dimension: usize) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value = mrly_math::counts::fill_polynomial(&corners, dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_grid(number: usize, dimension: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::grid(number, dimension, level);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_ladder_cap(base: usize) -> Result<usize, JsValue> {
    let value = mrly_math::counts::ladder::cap(base).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_ladder_carry_matrix(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::ladder::carry_matrix(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string())))
    })
}

#[wasm_bindgen]
pub fn math_counts_ladder_characteristic(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrly_math::counts::ladder::characteristic(&rows).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_ladder_determinant(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrly_math::counts::ladder::determinant(&rows).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_ladder_digit_polynomial(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrly_math::counts::ladder::digit_polynomial(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_ladder_even_block(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::ladder::even_block(base, dimension).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string())))
    })
}

#[wasm_bindgen]
pub fn math_counts_ladder_fill(base: usize, dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::ladder::fill(base, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_ladder_ladder(
    base: usize,
    dimension: usize,
    levels: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::ladder::ladder(base, dimension, levels).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_ladder_perron(rows: JsValue) -> Result<f64, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrly_math::counts::ladder::perron(&rows).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_ladder_sign(base: usize, dimension: usize) -> Result<i32, JsValue> {
    let value = mrly_math::counts::ladder::sign(base, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_ladder_spectral_ratio(
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::ladder::spectral_ratio(base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_counts_ladder_trace(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::list_from_js(&rows, |x1| hand::list_from_js(x1, hand::i128_from_js))?;
    let value = mrly_math::counts::ladder::trace(&rows);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_limit(
    code: JsValue,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::limit(code, dimension, level, base).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

#[wasm_bindgen]
pub fn math_counts_odd_power(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::odd_power(dimension);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_pairs(tile: JsValue) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::pairs(&tile);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_counts_positions(
    residue: usize,
    number: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::positions(residue, number, base);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_pro_fills(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::pro_fills(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_pro_voids(code: JsValue, number: usize, level: u32) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::pro_voids(code, number, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_profile_of_tile(tile: JsValue, level: u32) -> Result<JsValue, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::profile_of_tile(&tile, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

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
        mrly_math::counts::ratio(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_rational(
    code: JsValue,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrly_math::counts::rational(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

#[wasm_bindgen]
pub fn math_counts_six_grid_triangles(number: usize, level: u32) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::grid_triangles(number, level);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_boundary(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_boundary(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_core_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_core_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_core_nodes(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_core_nodes(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_edges(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_edges(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_interior(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_interior(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_triangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_triangles(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_six_solid_slice_vertices(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::counts::six::solid_slice_vertices(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_counts_skeleton(
    code: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
    level: u32,
) -> Result<math_counts_Skeleton, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrly_math::counts::skeleton(code, number, dimension, base, level).map_err(hand::throw)?;
    Ok(math_counts_Skeleton { inner: value })
}

#[wasm_bindgen]
pub fn math_counts_skeleton_of_tile(
    tile: JsValue,
    level: u32,
) -> Result<Option<math_counts_Skeleton>, JsValue> {
    let tile = hand::tensor_from_js(&tile)?;
    let value = mrly_math::counts::skeleton_of_tile(&tile, level).map_err(hand::throw)?;
    Ok(value.map(|inner| math_counts_Skeleton { inner }))
}

#[wasm_bindgen]
pub fn math_counts_skeleton_recurrence(
    code: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
    k: JsValue,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let k = hand::from_js::<Option<usize>>(&k)?;
    let value = mrly_math::counts::skeleton_recurrence(code, number, dimension, base, k)
        .map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_counts_surface(
    code: JsValue,
    number: usize,
    level: u32,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::counts::surface(code, number, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

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
        mrly_math::counts::void(code, number, dimension, level, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_design_digits_of(mask: u32, base: JsValue) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::design::digits_of(mask, base);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_echo_series(
    values: JsValue,
    log_x: &[f64],
    exponent: f64,
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrly_math::design::echo_series(&values, log_x, exponent);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_elements(
    base: JsValue,
    digits: JsValue,
    depth: usize,
) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::design::elements(base, &digits, depth);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_log_grid(values: JsValue, samples: usize) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrly_math::design::log_grid(&values, samples);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_median_floor(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_math::design::median_floor(power, width);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_meter(mu: &[i8]) -> Result<Vec<i64>, JsValue> {
    let value = mrly_math::design::meter(mu);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_nearest(value: f64, list: &[f64]) -> Result<f64, JsValue> {
    let value = mrly_math::design::nearest(value, list);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_peaks(
    gamma: &[f64],
    score: &[f64],
    band: JsValue,
    threshold: f64,
) -> Result<Vec<usize>, JsValue> {
    let band = hand::from_js::<(f64, f64)>(&band)?;
    let value = mrly_math::design::peaks(gamma, score, band, threshold).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_pole_lattice(base: JsValue, top: f64) -> Result<Vec<f64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::design::pole_lattice(base, top);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_resample(
    values: JsValue,
    running: JsValue,
    exponent: f64,
    log_x: &[f64],
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let running = hand::list_from_js(&running, hand::i64_from_js)?;
    let value = mrly_math::design::resample(&values, &running, exponent, log_x);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_score(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_math::design::score(power, width);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_size(digits: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::design::size(&digits, depth);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_design_spectrum(log_x: &[f64], series: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrly_math::design::spectrum(log_x, series).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

#[wasm_bindgen]
pub fn math_design_upper_rms(series: &[f64]) -> Result<f64, JsValue> {
    let value = mrly_math::design::upper_rms(series);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_cap(y: JsValue) -> Result<u64, JsValue> {
    let y = hand::u64_from_js(&y)?;
    let value = mrly_math::dissection::cap(y);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_chain_exponent(base: JsValue) -> Result<f64, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::dissection::chain_exponent(base);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_chain_margin(base: JsValue) -> Result<f64, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::dissection::chain_margin(base);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_chain_root(base: JsValue) -> Result<f64, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::dissection::chain_root(base);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_chain_wall() -> Result<u64, JsValue> {
    let value = mrly_math::dissection::chain_wall();
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_consecutive(digits: JsValue) -> Result<bool, JsValue> {
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::dissection::consecutive(&digits);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_fraction(a: JsValue, y: JsValue, cap: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::u64_from_js(&a)?;
    let y = hand::u64_from_js(&y)?;
    let cap = hand::u64_from_js(&cap)?;
    let value = mrly_math::dissection::fraction(a, y, cap);
    Ok(hand::tuple_to_js(&[
        JsValue::from(value.0),
        JsValue::from(value.1),
    ]))
}

#[wasm_bindgen]
pub fn math_dissection_kappa(base: JsValue, digits: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::dissection::kappa(base, &digits);
    Ok(hand::tuple_to_js(&[
        JsValue::from(value.0),
        JsValue::from(value.1),
    ]))
}

#[wasm_bindgen]
pub fn math_dissection_masses(
    base: JsValue,
    digits: JsValue,
    level: u32,
) -> Result<Vec<f64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::dissection::masses(base, &digits, level);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_reach(base: JsValue, missing: JsValue) -> Result<String, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let missing = hand::u64_from_js(&missing)?;
    let value = mrly_math::dissection::reach(base, missing);
    Ok(value.to_string())
}

#[wasm_bindgen]
pub fn math_dissection_reading(base: JsValue, fill: usize, masses: &[f64]) -> Result<f64, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrly_math::dissection::reading(base, fill, masses);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_regions(base: JsValue, level: u32, z: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let z = hand::u64_from_js(&z)?;
    let value = mrly_math::dissection::regions(base, level, z);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_dissection_tally(
    base: JsValue,
    digits: JsValue,
    level: usize,
    samples: usize,
) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::dissection::tally(base, &digits, level, samples);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_dissection_weights(
    base: JsValue,
    digits: JsValue,
    level: u32,
) -> Result<Vec<f64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrly_math::dissection::weights(base, &digits, level);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_cell(point: &[f64], base: u8, level: u32) -> Result<u64, JsValue> {
    let value = mrly_math::game::cell(point, base, level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_cells(base: u8, dimension: u8, level: u32) -> Result<u64, JsValue> {
    let value = mrly_math::game::cells(base, dimension, level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_counts(matrix: JsValue, start: usize, levels: usize) -> Result<Vec<u64>, JsValue> {
    let matrix = hand::list_from_js(&matrix, |x1| hand::list_from_js(x1, hand::u64_from_js))?;
    let value = mrly_math::game::counts(&matrix, start, levels).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_de_bruijn(m: u32, k: u32) -> Result<Vec<u8>, JsValue> {
    let value = mrly_math::game::de_bruijn(m, k).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_mosaic_game(mosaic: JsValue) -> Result<JsValue, JsValue> {
    let mosaic = hand::from_js::<mrly_math::game::Mosaic>(&mosaic)?;
    let value = mrly_math::game::mosaic_game(&mosaic).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_game_named_games() -> Result<JsValue, JsValue> {
    let value = mrly_math::game::named_games();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_game_perron(matrix: JsValue) -> Result<f64, JsValue> {
    let matrix = hand::list_from_js(&matrix, |x1| hand::list_from_js(x1, hand::u64_from_js))?;
    let value = mrly_math::game::perron(&matrix).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_reverse(rule: &math_memory_Rule) -> Result<math_memory_Rule, JsValue> {
    let value = mrly_math::game::reverse(&rule.inner);
    Ok(math_memory_Rule { inner: value })
}

#[wasm_bindgen]
pub fn math_game_slice_game(
    base: u8,
    digits: &[u8],
    p: JsValue,
    q: JsValue,
) -> Result<JsValue, JsValue> {
    let p = hand::i64_from_js(&p)?;
    let q = hand::i64_from_js(&q)?;
    let value = mrly_math::game::slice_game(base, digits, p, q).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_game_successors(rule: &math_memory_Rule, history: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_math::game::successors(&rule.inner, history);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_word_cell(word: &[u8], base: u8, dimension: u8) -> Result<u64, JsValue> {
    let value = mrly_math::game::word_cell(word, base, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_census(network: &math_graph_Network) -> Result<JsValue, JsValue> {
    let value = mrly_math::graph::census(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_graph_components(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrly_math::graph::components(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_core_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrly_math::graph::core_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_graph_edge_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrly_math::graph::edge_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_graph_fractal_dimension(
    network: &math_graph_Network,
    samples: usize,
) -> Result<f64, JsValue> {
    let value = mrly_math::graph::fractal_dimension(&network.inner, samples);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_junctions(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrly_math::graph::junctions(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_largest_component(
    network: &math_graph_Network,
) -> Result<math_graph_Network, JsValue> {
    let value = mrly_math::graph::largest_component(&network.inner).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_graph_roles(network: &math_graph_Network) -> Result<JsValue, JsValue> {
    let value = mrly_math::graph::roles(&network.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_graph_tips(network: &math_graph_Network) -> Result<usize, JsValue> {
    let value = mrly_math::graph::tips(&network.inner).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_total_length(network: &math_graph_Network) -> Result<f64, JsValue> {
    let value = mrly_math::graph::total_length(&network.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_graph_tunnel_graph(grid: JsValue) -> Result<math_graph_Network, JsValue> {
    let grid = hand::tensor_from_js(&grid)?;
    let value = mrly_math::graph::tunnel_graph(&grid).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_ladder_cofactor(
    design: &math_ladder_Design,
    s: JsValue,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
    let value = mrly_math::ladder::cofactor(&design.inner, s, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_ladder_residue(
    design: &math_ladder_Design,
    m: usize,
    j: JsValue,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let j = hand::i64_from_js(&j)?;
    let value = mrly_math::ladder::residue(&design.inner, m, j, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_ladder_zeta(
    design: &math_ladder_Design,
    s: JsValue,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
    let value = mrly_math::ladder::zeta(&design.inner, s, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_memory_allowed_windows(rule: &math_memory_Rule) -> Result<usize, JsValue> {
    let value = mrly_math::memory::allowed_windows(&rule.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_cells(rule: &math_memory_Rule, level: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly_math::memory::cells(&rule.inner, level);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_counts(rule: &math_memory_Rule, levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly_math::memory::counts(&rule.inner, levels);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_exponent(rule: &math_memory_Rule) -> Result<f64, JsValue> {
    let value = mrly_math::memory::exponent(&rule.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_kappa(rule: &math_memory_Rule) -> Result<f64, JsValue> {
    let value = mrly_math::memory::kappa(&rule.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_perron(rule: &math_memory_Rule) -> Result<f64, JsValue> {
    let value = mrly_math::memory::perron(&rule.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_transfer(rule: &math_memory_Rule) -> Result<JsValue, JsValue> {
    let value = mrly_math::memory::transfer(&rule.inner);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_moire_all(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::moire::all(limit);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(math_moire_Preset { inner: x1.clone() }))
    })
}

#[wasm_bindgen]
pub fn math_moire_frame(normal: JsValue, offset: f64) -> Result<JsValue, JsValue> {
    let normal = hand::from_js::<[f64; 3]>(&normal)?;
    let value = mrly_math::moire::frame(normal, offset).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_moire_layer(params: JsValue) -> Result<JsValue, JsValue> {
    let params = hand::from_js::<mrly_math::moire::Layer>(&params)?;
    let value = mrly_math::moire::layer(&params).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_moire_named(name: &str, limit: usize) -> Result<math_moire_Preset, JsValue> {
    let value = mrly_math::moire::named(name, limit).map_err(hand::throw)?;
    Ok(math_moire_Preset { inner: value })
}

#[wasm_bindgen]
pub fn math_moire_pairs_correlation(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrly_math::moire::pairs::correlation(m, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_moire_pairs_sampled(m: usize, n: usize) -> Result<f64, JsValue> {
    let value = mrly_math::moire::pairs::sampled(m, n).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_moire_pairs_witness(scale: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::moire::pairs::witness(scale).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_moire_render(
    field: &math_moire_Field,
    colorizer: JsValue,
    levels: usize,
    symmetric: bool,
    invert: bool,
    scale: usize,
) -> Result<JsValue, JsValue> {
    let colorizer = hand::from_js::<mrly_core::Colorizer>(&colorizer)?;
    let value =
        mrly_math::moire::render(&field.inner, &colorizer, levels, symmetric, invert, scale)
            .map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn math_moire_sample_axes(
    size: usize,
    lattice: JsValue,
    row: usize,
) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrly_math::moire::Lattice>(&lattice)?;
    let value = mrly_math::moire::sample::axes(size, lattice, row);
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

#[wasm_bindgen]
pub fn math_moire_sample_membership(
    code: JsValue,
    base: usize,
    dimension: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_math::moire::sample::membership(code, base, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_moire_sample_pack(residues: &[usize], base: usize) -> Result<usize, JsValue> {
    let value = mrly_math::moire::sample::pack(residues, base);
    Ok(value)
}

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
    let spec = hand::from_js::<mrly_math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrly_math::moire::Combine>(&combine)?;
    let lattice = hand::from_js::<mrly_math::moire::Lattice>(&lattice)?;
    let value = mrly_math::moire::stack(spec, numbers, combine, level, lattice, size, slices)
        .map_err(hand::throw)?;
    Ok(math_moire_Field { inner: value })
}

#[wasm_bindgen]
pub fn math_moire_stack_codes(
    specs: JsValue,
    number: usize,
    level: usize,
    lattice: JsValue,
    size: usize,
    slices: &[f64],
) -> Result<math_moire_Field, JsValue> {
    let specs = hand::from_js::<Vec<mrly_math::moire::Spec>>(&specs)?;
    let lattice = hand::from_js::<mrly_math::moire::Lattice>(&lattice)?;
    let value = mrly_math::moire::stack_codes(&specs, number, level, lattice, size, slices)
        .map_err(hand::throw)?;
    Ok(math_moire_Field { inner: value })
}

#[wasm_bindgen]
pub fn math_moire_volume(
    spec: JsValue,
    numbers: &[usize],
    combine: JsValue,
    level: usize,
    size: usize,
) -> Result<math_moire_Volume, JsValue> {
    let spec = hand::from_js::<mrly_math::moire::Spec>(&spec)?;
    let combine = hand::from_js::<mrly_math::moire::Combine>(&combine)?;
    let value =
        mrly_math::moire::volume(spec, numbers, combine, level, size).map_err(hand::throw)?;
    Ok(math_moire_Volume { inner: value })
}

#[wasm_bindgen]
pub fn math_name_of(code: JsValue, dim: usize, base: usize) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::name::of(code, dim, base);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_name_words(dim: usize, base: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::name::words(dim, base);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_nodal_figures(
    code: JsValue,
    number: usize,
    level: usize,
    count: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::u64_from_js(&code)?;
    let value = mrly_math::nodal::figures(code, number, level, count).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_nodal_holds(code: JsValue, number: usize, level: usize) -> Result<bool, JsValue> {
    let code = hand::u64_from_js(&code)?;
    let value = mrly_math::nodal::holds(code, number, level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_press_containing(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::containing(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_press_coordinates(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::coordinates(number, dimension, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_press_count_below(
    code: JsValue,
    dimension: usize,
    base: usize,
    limit: JsValue,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let limit = hand::u128_from_js(&limit)?;
    let value = mrly_math::press::count_below(code, dimension, base, limit).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_press_distinct(number: JsValue, dimension: usize, base: usize) -> Result<u32, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::distinct(number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_press_interleave(coords: JsValue, base: usize) -> Result<JsValue, JsValue> {
    let coords = hand::list_from_js(&coords, hand::u128_from_js)?;
    let value = mrly_math::press::interleave(&coords, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_press_layer_table(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer = hand::from_js::<mrly_math::bang::MagicLayer>(&layer)?;
    let value = mrly_math::press::layer_table(&layer).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_press_member(
    code: JsValue,
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<bool, JsValue> {
    let code = hand::code_from_js(&code)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::member(code, number, dimension, base).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_press_members(
    code: JsValue,
    dimension: usize,
    base: usize,
    count: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::press::members(code, dimension, base, count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_press_profile(
    code: JsValue,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::press::profile(code, dimension, base, level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_press_usage(
    number: JsValue,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::usage(number, dimension, base).map_err(hand::throw)?;
    Ok(JsValue::from_str(&hand::code_to_js(value)))
}

#[wasm_bindgen]
pub fn math_press_word_count(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::press::word_count(&layers).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_press_word_member(layers: JsValue, number: JsValue) -> Result<bool, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let number = hand::u128_from_js(&number)?;
    let value = mrly_math::press::word_member(&layers, number).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_press_word_members(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::press::word_members(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_press_word_profile(layers: JsValue) -> Result<JsValue, JsValue> {
    let layers = hand::from_js::<Vec<mrly_math::bang::MagicLayer>>(&layers)?;
    let value = mrly_math::press::word_profile(&layers).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_radix_flowsnake() -> Result<math_radix_Radix, JsValue> {
    let value = mrly_math::radix::flowsnake().map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_radix_gasket() -> Result<math_radix_Radix, JsValue> {
    let value = mrly_math::radix::gasket().map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_radix_koch() -> Result<math_radix_Radix, JsValue> {
    let value = mrly_math::radix::koch().map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_radix_terdragon() -> Result<math_radix_Radix, JsValue> {
    let value = mrly_math::radix::terdragon().map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_radix_tile(m: JsValue, code: JsValue) -> Result<math_radix_Radix, JsValue> {
    let m = hand::u64_from_js(&m)?;
    let code = hand::u128_from_js(&code)?;
    let value = mrly_math::radix::tile(m, code).map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_radix_twindragon() -> Result<math_radix_Radix, JsValue> {
    let value = mrly_math::radix::twindragon().map_err(hand::throw)?;
    Ok(math_radix_Radix { inner: value })
}

#[wasm_bindgen]
pub fn math_roulette_nodes(
    track: JsValue,
    pencils: JsValue,
    samples: usize,
    tol: f64,
) -> Result<math_roulette_Nodes, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::roulette::nodes(&track, &pencils, samples, tol).map_err(hand::throw)?;
    Ok(math_roulette_Nodes { inner: value })
}

#[wasm_bindgen]
pub fn math_roulette_side(a: JsValue, b: JsValue, c: JsValue) -> Result<i32, JsValue> {
    let a = hand::from_js::<[f64; 2]>(&a)?;
    let b = hand::from_js::<[f64; 2]>(&b)?;
    let c = hand::from_js::<[f64; 2]>(&c)?;
    let value = mrly_math::roulette::side(a, b, c);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_roulette_spread(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::roulette::spread(&track, &pencils, exact);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_rules_render(
    filled: JsValue,
    number: usize,
    dimension: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
    let value = mrly_math::rules::render(&filled, number, dimension, base).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_rules_tree_axes(dimension: usize, free_axis: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_math::rules::tree_axes(dimension, free_axis);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_shape_census(shape: JsValue, types: JsValue) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let types = hand::tensor_from_js(&types)?;
    let value = mrly_math::shape::census(&shape, &types).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_classify(
    shape: JsValue,
    side: usize,
    index: &[usize],
) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let value = mrly_math::shape::classify(&shape, side, index).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_crop(types: JsValue, shape: JsValue, keep_cut: bool) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let value = mrly_math::shape::crop(&types, &shape, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_crossing_shell(
    radius: JsValue,
    number: JsValue,
    level: u32,
) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let value = mrly_math::shape::crossing_shell(radius, number, level);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_shape_crossing_tree(
    radius: JsValue,
    number: JsValue,
    keep: JsValue,
) -> Result<JsValue, JsValue> {
    let radius = hand::u64_from_js(&radius)?;
    let number = hand::u64_from_js(&number)?;
    let keep = hand::from_js::<Vec<bool>>(&keep)?;
    let value = mrly_math::shape::crossing_tree(radius, number, &keep);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_fit(shape: JsValue, extents: &[usize]) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let value = mrly_math::shape::fit(&shape, extents).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_named(
    name: &str,
    dimension: usize,
    radius: &math_shape_Frac,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::shape::named(name, dimension, radius.inner).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_radial_census(
    types: JsValue,
    centre: JsValue,
    r_max: JsValue,
) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let centre = hand::list_from_js(&centre, hand::i64_from_js)?;
    let r_max = hand::u64_from_js(&r_max)?;
    let value = mrly_math::shape::radial_census(&types, &centre, r_max);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_reach(name: &str, dimension: usize, den: u32) -> Result<JsValue, JsValue> {
    let value = mrly_math::shape::reach(name, dimension, den).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_refine(
    types: JsValue,
    shape: JsValue,
    base: usize,
    extra: usize,
    keep_cut: bool,
) -> Result<JsValue, JsValue> {
    let types = hand::tensor_from_js(&types)?;
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let value =
        mrly_math::shape::refine(&types, &shape, base, extra, keep_cut).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_regions(shape: JsValue, dims: &[usize]) -> Result<JsValue, JsValue> {
    let shape = hand::from_js::<mrly_math::shape::Shape>(&shape)?;
    let value = mrly_math::shape::regions(&shape, dims).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_shape_shapes(dimension: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::shape::shapes(dimension);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_anti(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.anti();
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_binarize(cell: JsValue, threshold: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize(threshold);
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_binarize_otsu(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.binarize_otsu();
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_blank(
    radius: usize,
    orient: JsValue,
    fill: u8,
    void_: u8,
) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrly_math::six::Orientation>(&orient)?;
    let value = mrly_math::six::blank(radius, orient, fill, void_).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_blur(cell: JsValue, mask: JsValue, wrap: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.blur(&mask, wrap).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_census(cell: JsValue, include_grid: bool) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::census(&cell, include_grid);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_components(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::components(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_copies(rings: usize) -> Result<usize, JsValue> {
    let value = mrly_math::six::copies(rings).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_cut(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::six::cut(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_cut_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::six::cut_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_disc(cell: JsValue, rings: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::disc(&cell, rings).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_east(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrly_math::six::east(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_six_euler(cell: JsValue, include_grid: bool) -> Result<i64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::euler(&cell, include_grid);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::fills(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_fills_only(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::fills_only(&cell);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_framed(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::framed(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_math::six::from_json(text).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_giant(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::giant(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_giant_network(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::giant_network(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_six_height(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.height();
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_image(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let value = mrly_math::six::image(&cell, scale, outline, width).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_is_cube(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::six::is_cube(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_is_hex(cell: JsValue) -> Result<bool, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::six::is_hex(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_iso(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::six::iso(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_iso_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::six::iso_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_new(
    cell: JsValue,
    projection: JsValue,
    orientation: JsValue,
    start: u8,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let projection = hand::from_js::<mrly_math::six::Projection>(&projection)?;
    let orientation = hand::from_js::<mrly_math::six::Orientation>(&orientation)?;
    let value = mrly_math::six::Cell6d::new(cell, projection, orientation, start);
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_north(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrly_math::six::north(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_six_orientation(width: usize, height: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::six::orientation(width, height).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_pad(cell: JsValue, k: usize, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::pad(&cell, k, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

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
    let mode = hand::from_js::<Option<mrly_core::Mode>>(&mode)?;
    let mut rng_stream = hand::stream_from_js(&rng)?;
    let value = mrly_math::six::paint(cell, custom.as_ref(), mode, rng_stream.as_mut())
        .map_err(hand::throw)?;
    if let Some(stream) = &rng_stream {
        hand::stream_to_js(&rng, stream)?;
    }
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_perforate(cell: JsValue, mask: JsValue, value: u8) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = cell.perforate(&mask, value).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_pro(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::six::pro(&cell).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_pro_design(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::six::pro_design(code, number, level, base).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_radial(cell: JsValue, radius: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::radial(&cell, radius).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_radial_crop(
    cell: JsValue,
    radius: usize,
    size: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrly_math::six::radial_crop(&cell, radius, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_radial_mask(radius: usize, orient: JsValue) -> Result<JsValue, JsValue> {
    let orient = hand::from_js::<mrly_math::six::Orientation>(&orient)?;
    let value = mrly_math::six::radial_mask(radius, orient);
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_raster(cell: JsValue, size: usize) -> Result<Vec<f32>, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::raster(&cell, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_rect_image(
    cell: JsValue,
    scale: usize,
    start: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrly_math::six::rect_image(&cell, scale, start).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_rect_svg(cell: JsValue, scale: usize, start: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrly_math::six::rect_svg(&cell, scale, start).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_rim_holes(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::rim_holes(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_skin(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::skin(&cell);
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_slice_core_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::slice_core_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_six_slice_dual_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::slice_dual_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_six_slice_edge_graph(
    cell: JsValue,
    value: JsValue,
) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = hand::from_js::<Option<u8>>(&value)?;
    let value = mrly_math::six::slice_edge_graph(&cell, value).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_six_slice_tunnel_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::slice_tunnel_graph(&cell).map_err(hand::throw)?;
    Ok(math_graph_Network { inner: value })
}

#[wasm_bindgen]
pub fn math_six_south(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrly_math::six::south(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_six_spectral_exponent(cell: JsValue, window: f64) -> Result<f64, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::spectral_exponent(&cell, window).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_star_arm_law(number: usize) -> Result<math_six_star_Share, JsValue> {
    let value = mrly_math::six::star::arm_law(number).map_err(hand::throw)?;
    Ok(math_six_star_Share { inner: value })
}

#[wasm_bindgen]
pub fn math_six_star_chi8(number: usize) -> Result<i64, JsValue> {
    let value = mrly_math::six::star::chi8(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_star_constant() -> Result<f64, JsValue> {
    let value = mrly_math::six::star::constant();
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_star_decay(excesses: &[f64], layers: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::six::star::decay(excesses, layers).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_star_width_law(half: usize) -> Result<f64, JsValue> {
    let value = mrly_math::six::star::width_law(half);
    Ok(value)
}

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
    let value = mrly_math::six::svg(&cell, scale, outline, width, start).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_tessellate(cell: JsValue, mask: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_math::six::tessellate(&cell, &mask).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_tile(cell: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::tile(&cell, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_tile_cell(
    cell: JsValue,
    width: usize,
    height: usize,
    crop: bool,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::tile_cell(&cell, width, height, crop).map_err(hand::throw)?;
    hand::cell6d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_tile_crop(cell: JsValue, size: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrly_math::six::tile_crop(&cell, size).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_tile_step(size: JsValue) -> Result<JsValue, JsValue> {
    let size = hand::from_js::<(usize, usize)>(&size)?;
    let value = mrly_math::six::tile_step(size).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_six_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = mrly_math::six::to_json(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_triangles(cell: JsValue, start: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let start = hand::from_js::<Option<usize>>(&start)?;
    let value = mrly_math::six::triangles(&cell, start).map_err(hand::throw)?;
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

#[wasm_bindgen]
pub fn math_six_west(x: JsValue, y: JsValue) -> Result<JsValue, JsValue> {
    let x = hand::i64_from_js(&x)?;
    let y = hand::i64_from_js(&y)?;
    let value = mrly_math::six::west(x, y);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_six_width(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell6d_from_js(&cell)?;
    let value = cell.width();
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spectrum_clusters(eigenvalues: &[f64], tolerance: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::spectrum::clusters(eigenvalues, tolerance).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spectrum_laplacian(
    network: &math_graph_Network,
    normalised: bool,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::spectrum::laplacian(&network.inner, normalised).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

#[wasm_bindgen]
pub fn math_spectrum_laplacian_spectrum(
    network: &math_graph_Network,
    normalised: bool,
) -> Result<Vec<f64>, JsValue> {
    let value =
        mrly_math::spectrum::laplacian_spectrum(&network.inner, normalised).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spectrum_multiplicity(
    eigenvalues: &[f64],
    value: f64,
    tolerance: f64,
) -> Result<usize, JsValue> {
    let value = mrly_math::spectrum::multiplicity(eigenvalues, value, tolerance);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spectrum_spectral_exponent(
    eigenvalues: &[f64],
    window: f64,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::spectrum::spectral_exponent(eigenvalues, window);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spectrum_spectral_fit(eigenvalues: &[f64], window: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::spectrum::spectral_fit(eigenvalues, window);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spectrum_spectral_points(eigenvalues: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrly_math::spectrum::spectral_points(eigenvalues);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spectrum_symmetric_eigenvalues(matrix: JsValue) -> Result<Vec<f64>, JsValue> {
    let matrix = hand::from_js::<Vec<Vec<f64>>>(&matrix)?;
    let value = mrly_math::spectrum::symmetric_eigenvalues(&matrix).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_arcs(data: &[f32], size: usize, radius: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::spin::arcs(data, size, radius).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spin_harmonics(
    data: &[f32],
    size: usize,
    rings: usize,
    orders: usize,
) -> Result<Vec<f64>, JsValue> {
    let value = mrly_math::spin::harmonics(data, size, rings, orders).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_mass(profile: &[f32], size: usize) -> Result<f64, JsValue> {
    let value = mrly_math::spin::mass(profile, size);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_mass_within(profile: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrly_math::spin::mass_within(profile, size, radius);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_petals(copies: usize, order: usize) -> Result<usize, JsValue> {
    let value = mrly_math::spin::petals(copies, order);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_profile(data: &[f32], size: usize, steps: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrly_math::spin::profile(data, size, steps).map_err(hand::throw)?;
    Ok(value)
}

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
    let blend = hand::from_js::<mrly_math::spin::Blend>(&blend)?;
    let value = mrly_math::spin::radial(data, size, out, copies, step, blend, samples)
        .map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_reach(size: usize) -> Result<f64, JsValue> {
    let value = mrly_math::spin::reach(size);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_ring(data: &[f32], size: usize, radius: f64) -> Result<f64, JsValue> {
    let value = mrly_math::spin::ring(data, size, radius).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_turns(power: &[f64]) -> Result<usize, JsValue> {
    let value = mrly_math::spin::turns(power);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spin_wheel(profile: &[f32], size: usize) -> Result<Vec<f32>, JsValue> {
    let value = mrly_math::spin::wheel(profile, size);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_cell(width: usize, height: usize, reach: f64) -> Result<f64, JsValue> {
    let value = mrly_math::spirograph::cell(width, height, reach);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_cover(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
    samples: usize,
    side: usize,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::cover(&track, &pencils, exact, samples, side)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_disc(track: JsValue, pencils: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::disc(&track, &pencils).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_distinct(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<usize, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::distinct(&track, &pencils, exact);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_frame(track: JsValue, pencils: JsValue) -> Result<Vec<f64>, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::frame(&track, &pencils);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn math_spirograph_nodes(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::nodes(&track, &pencils, exact);
    hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
}

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
    let value = mrly_math::spirograph::pencils(types, width, height, mode, reach, jitter, seed)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_point(track: JsValue, pencil: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrly_math::spirograph::Pencil>(&pencil)?;
    let value = mrly_math::spirograph::point(&track, &pencil, s);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_pose(track: JsValue, s: f64) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let value = mrly_math::spirograph::pose(&track, s);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_representatives(
    track: JsValue,
    pencils: JsValue,
    exact: bool,
) -> Result<Vec<usize>, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::representatives(&track, &pencils, exact);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_seats(pencils: JsValue) -> Result<JsValue, JsValue> {
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::seats(&pencils);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_signed_area(track: JsValue, pencil: JsValue) -> Result<JsValue, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencil = hand::from_js::<mrly_math::spirograph::Pencil>(&pencil)?;
    let value = mrly_math::spirograph::signed_area(&track, &pencil);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_trace(
    track: JsValue,
    pencils: JsValue,
    samples: usize,
) -> Result<Vec<f32>, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let pencils = hand::from_js::<Vec<mrly_math::spirograph::Pencil>>(&pencils)?;
    let value = mrly_math::spirograph::trace(&track, &pencils, samples).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_track(
    kind: &str,
    ring: usize,
    wheel: usize,
    sides: usize,
    laps: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrly_math::spirograph::track(kind, ring, wheel, sides, laps).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_spirograph_turn(track: JsValue, s: f64) -> Result<f64, JsValue> {
    let track = hand::from_js::<mrly_math::spirograph::Track>(&track)?;
    let value = mrly_math::spirograph::turn(&track, s);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_sumset_pairs(level: u32) -> Result<JsValue, JsValue> {
    let value = mrly_math::sumset::pairs(level).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(math_sumset_Pair { inner: *x1 }))
    })
}

#[wasm_bindgen]
pub fn math_three_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::carpet(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_core_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::three::core_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::three::core_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_three_create(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::three::create(code, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_diagonal_slice(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
    height: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value =
        mrly_math::three::diagonal_slice(code, number, level, base, height).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

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
    let value = mrly_math::three::diagonal_svg(code, number, level, base, heights, scale)
        .map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::dust(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_edge_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::three::edge_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::three::edge_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_three_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_extrude(cell: JsValue, axis: usize, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::three::extrude(&cell, axis, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_extrude_cube(cell: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::extrude_cube(&cell, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_faces(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::faces(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::fills(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_from_corners(
    corners: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value =
        mrly_math::three::from_corners(&corners, number, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::from_json(text).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_from_strings(data: JsValue) -> Result<JsValue, JsValue> {
    let data = hand::from_js::<Vec<Vec<String>>>(&data)?;
    let value = mrly_math::three::from_strings(&data).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_hidden(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::hidden(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_three_level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::level_set(number, levels, level, base).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_magic(cells: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value = mrly_math::three::magic::<2>(&cells).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value = mrly_math::three::magic::<3>(&cells).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_three_manhattan_layers(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::manhattan_layers(cell);
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_merge(
    cells: JsValue,
    width: usize,
    height: usize,
    depth: usize,
) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
    let value = mrly_math::three::merge(&cells, width, height, depth).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_mosaic(mask: JsValue, cells: JsValue) -> Result<JsValue, JsValue> {
    match hand::cell_rank(&hand::first(&cells)?)? {
        2 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
            let value = mrly_math::three::mosaic::<2>(&mask, &cells).map_err(hand::throw)?;
            hand::cell2d_to_js(&value)
        }
        3 => {
            let mask = hand::tensor_from_js(&mask)?;
            let cells = hand::list_from_js(&cells, hand::cell3d_from_js)?;
            let value = mrly_math::three::mosaic::<3>(&mask, &cells).map_err(hand::throw)?;
            hand::cell3d_to_js(&value)
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_three_named(design: JsValue, number: usize, level: usize) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_math::bang::catalog::Design>(&design)?;
    let value = mrly_math::three::named(design, number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::net(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_noise(
    number: usize,
    level: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value =
        mrly_math::three::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::ones(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_orientations() -> Result<JsValue, JsValue> {
    let value = mrly_math::three::orientations();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::point(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_profile(
    code: JsValue,
    number: usize,
    level: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::three::profile(code, number, level, base).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn math_three_project(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrly_math::three::project(point);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_quads(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::quads(&cell);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_shadow(point: JsValue) -> Result<JsValue, JsValue> {
    let point = hand::from_js::<[u32; 3]>(&point)?;
    let value = mrly_math::three::shadow(point);
    Ok(hand::tuple_to_js(&[
        JsValue::from(value.0),
        JsValue::from(value.1),
    ]))
}

#[wasm_bindgen]
pub fn math_three_slice(cell: JsValue, axis: usize, index: usize) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::slice(&cell, axis, index).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_sponge_deep(radius: f64) -> Result<f64, JsValue> {
    let value = mrly_math::three::sponge::deep(radius);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_dimension() -> Result<f64, JsValue> {
    let value = mrly_math::three::sponge::dimension();
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_distance(point: JsValue) -> Result<f64, JsValue> {
    let point = hand::from_js::<[f64; 3]>(&point)?;
    let value = mrly_math::three::sponge::distance(point);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_exact(radius: f64) -> Result<f64, JsValue> {
    let value = mrly_math::three::sponge::exact(radius);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_profile(radius: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::sponge::profile(radius);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_sponge_reading(radius: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::sponge::reading(radius);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_sponge_tube(radius: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::sponge::tube(radius);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_sponge_volume(radius: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::sponge::volume(radius);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::star(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_support(counts: JsValue) -> Result<JsValue, JsValue> {
    let counts = hand::list_from_js(&counts, hand::u128_from_js)?;
    let value = mrly_math::three::support(&counts);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_surface(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::surface(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_three_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| {
        hand::map_from_js::<u8, _>(x1, hand::from_js::<String>)
    })?;
    let value = mrly_math::three::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::to_json(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_to_obj(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::to_obj(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_to_strings(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::to_strings(&cell);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_tunnel_graph(cell: JsValue) -> Result<math_graph_Network, JsValue> {
    match hand::cell_rank(&cell)? {
        2 => {
            let cell = hand::cell2d_from_js(&cell)?;
            let value = mrly_math::three::tunnel_graph::<2>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        3 => {
            let cell = hand::cell3d_from_js(&cell)?;
            let value = mrly_math::three::tunnel_graph::<3>(&cell).map_err(hand::throw)?;
            Ok(math_graph_Network { inner: value })
        }
        rank => Err(hand::refuse(&format!(
            "a 2d or 3d cell was wanted, not {rank}d."
        ))),
    }
}

#[wasm_bindgen]
pub fn math_three_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::void(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::voids(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_volume(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::volume(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_wires(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell3d_from_js(&cell)?;
    let value = mrly_math::three::wires(&cell);
    hand::list_to_js(&value, |x1| {
        hand::list_to_js(x1, |x2| Ok(JsValue::from(math_three_Vec3 { inner: *x2 })))
    })
}

#[wasm_bindgen]
pub fn math_three_xline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::xline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_xtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::xtree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_yline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::yline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_ytree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::ytree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::zeros(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_zline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::zline(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_three_ztree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::three::ztree(number, level).map_err(hand::throw)?;
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_tourbillon_eyes(qmax: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::tourbillon::eyes(qmax);
    hand::to_js(&value)
}

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
    let value = mrly_math::tourbillon::field(
        top, size, schedule, increment, set, weights, mode, blend, seed,
    )
    .map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_tourbillon_layers(
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    seed: u32,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::tourbillon::layers(top, schedule, increment, set, weights, seed)
        .map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_tourbillon_period(increment: f64) -> Result<JsValue, JsValue> {
    let value = mrly_math::tourbillon::period(increment);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_tourbillon_sharing(list: JsValue) -> Result<JsValue, JsValue> {
    let list = hand::from_js::<Vec<mrly_math::tourbillon::Layer>>(&list)?;
    let value = mrly_math::tourbillon::sharing(&list);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_tourbillon_stack(
    list: JsValue,
    size: usize,
    mode: &str,
    blend: JsValue,
) -> Result<Vec<f32>, JsValue> {
    let list = hand::from_js::<Vec<mrly_math::tourbillon::Layer>>(&list)?;
    let blend = hand::from_js::<mrly_math::spin::Blend>(&blend)?;
    let value = mrly_math::tourbillon::stack(&list, size, mode, blend).map_err(hand::throw)?;
    Ok(value)
}

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
    let value = mrly_math::tourbillon::stats(
        field, size, top, schedule, increment, set, weights, blend, seed,
    )
    .map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_capacity(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::capacity(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_carpet(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::carpet(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_census(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::census(&cell).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_create(
    code: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let code = hand::code_from_js(&code)?;
    let value = mrly_math::two::create(code, number, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_dust(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::dust(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_embed(cell: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::embed(&cell, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_euler(cell: JsValue) -> Result<i64, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::euler(&cell).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_extract(carrier: JsValue, carried: JsValue) -> Result<Vec<u8>, JsValue> {
    let carrier = hand::cell2d_from_js(&carrier)?;
    let carried = hand::cell2d_from_js(&carried)?;
    let value = mrly_math::two::extract(&carrier, &carried).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_fills(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::fills(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_from_corners(
    corners: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let corners = hand::from_js::<Vec<Vec<u8>>>(&corners)?;
    let value = mrly_math::two::from_corners(&corners, number, level, rotation, base)
        .map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_from_json(text: &str) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::from_json(text).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_from_strings(rows: JsValue) -> Result<JsValue, JsValue> {
    let rows = hand::from_js::<Vec<String>>(&rows)?;
    let value = mrly_math::two::from_strings(&rows).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_hline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::hline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_htree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::htree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_image(
    cell: JsValue,
    scale: usize,
    outline: JsValue,
    width: usize,
    shape: JsValue,
) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let outline = hand::option_from_js(&outline, hand::color_from_js)?;
    let shape = hand::from_js::<mrly_math::two::Shape>(&shape)?;
    let value = mrly_math::two::image(&cell, scale, outline, width, shape).map_err(hand::throw)?;
    hand::image_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_level_set(
    number: usize,
    levels: &[usize],
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<JsValue, JsValue> {
    let value =
        mrly_math::two::level_set(number, levels, level, rotation, base).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_mask(mask: JsValue, shape: &[usize]) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let value = mrly_math::two::mask(&mask, shape).map_err(hand::throw)?;
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_merge(cells: JsValue, width: usize, height: usize) -> Result<JsValue, JsValue> {
    let cells = hand::list_from_js(&cells, hand::cell2d_from_js)?;
    let value = mrly_math::two::merge(&cells, width, height).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_named(
    design: JsValue,
    number: usize,
    level: usize,
    rotation: usize,
) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_math::bang::catalog::Design>(&design)?;
    let value = mrly_math::two::named(design, number, level, rotation).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_net(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::net(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_noise(
    number: usize,
    level: usize,
    density: f64,
    rng: &mut hand::Rng,
) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::noise(number, level, density, rng.stream()).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_ones(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::ones(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_payload_frame() -> Result<JsValue, JsValue> {
    let value = mrly_math::two::payload::frame();
    hand::tensor_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_perimeter(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::perimeter(&cell);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn math_two_point(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::point(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_read(sheet: JsValue, carrier: JsValue) -> Result<Vec<u8>, JsValue> {
    let sheet = hand::cell2d_from_js(&sheet)?;
    let carrier = hand::cell2d_from_js(&carrier)?;
    let value = mrly_math::two::read(&sheet, &carrier).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_sheet(cells: JsValue, payload: &[u8]) -> Result<JsValue, JsValue> {
    let cells = hand::array_from_js::<_, 4>(&cells, hand::cell2d_from_js)?;
    let value = mrly_math::two::sheet(&cells, payload).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_special(mask: JsValue, cell: JsValue) -> Result<JsValue, JsValue> {
    let mask = hand::tensor_from_js(&mask)?;
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::special(&mask, &cell).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_star(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::star(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

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
    let shape = hand::from_js::<mrly_math::two::Shape>(&shape)?;
    let value = mrly_math::two::svg(&cell, scale, outline, width, shape);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_text(cell: JsValue, glyphs: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let glyphs = hand::option_from_js(&glyphs, |x1| {
        hand::map_from_js::<u8, _>(x1, hand::from_js::<String>)
    })?;
    let value = mrly_math::two::text(&cell, glyphs.as_ref());
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_to_3d(cell: JsValue) -> Result<JsValue, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::to_3d(&cell);
    hand::cell3d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_to_json(cell: JsValue) -> Result<String, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::to_json(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_vline(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::vline(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_void(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::void(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_voids(cell: JsValue) -> Result<usize, JsValue> {
    let cell = hand::cell2d_from_js(&cell)?;
    let value = mrly_math::two::voids(&cell);
    Ok(value)
}

#[wasm_bindgen]
pub fn math_two_vtree(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::vtree(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_two_zeros(number: usize, level: usize) -> Result<JsValue, JsValue> {
    let value = mrly_math::two::zeros(number, level).map_err(hand::throw)?;
    hand::cell2d_to_js(&value)
}

#[wasm_bindgen]
pub fn math_arrays_MOST() -> Result<u64, JsValue> {
    let value = mrly_math::arrays::MOST;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_automaton_ROUNDING() -> Result<f64, JsValue> {
    let value = mrly_math::automaton::ROUNDING;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_baseq_WALK_LIMIT() -> Result<usize, JsValue> {
    let value = mrly_math::bang::baseq::WALK_LIMIT;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_bang_catalog_ANTIS_2D() -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::ANTIS_2D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_catalog_ANTIS_3D() -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::ANTIS_3D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_catalog_CLASSICS_2D() -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::CLASSICS_2D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_bang_catalog_CLASSICS_3D() -> Result<JsValue, JsValue> {
    let value = mrly_math::bang::catalog::CLASSICS_3D;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_coprime_GASKET_LEVELS() -> Result<u32, JsValue> {
    let value = mrly_math::coprime::GASKET_LEVELS;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_counts_diagonal_WIDEST() -> Result<usize, JsValue> {
    let value = mrly_math::counts::diagonal::WIDEST;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_design_ZETA_ORDINATES() -> Result<Vec<f64>, JsValue> {
    let value = mrly_math::design::ZETA_ORDINATES;
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn math_dissection_BAR_A() -> Result<f64, JsValue> {
    let value = mrly_math::dissection::BAR_A;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_BAR_B() -> Result<f64, JsValue> {
    let value = mrly_math::dissection::BAR_B;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_CAP_BASE() -> Result<u64, JsValue> {
    let value = mrly_math::dissection::CAP_BASE;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_CERTIFIED() -> Result<JsValue, JsValue> {
    let value = mrly_math::dissection::CERTIFIED;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn math_dissection_DIGIT_WALL() -> Result<u64, JsValue> {
    let value = mrly_math::dissection::DIGIT_WALL;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_FIRST_BELOW() -> Result<u64, JsValue> {
    let value = mrly_math::dissection::FIRST_BELOW;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_GAMMA() -> Result<f64, JsValue> {
    let value = mrly_math::dissection::GAMMA;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_dissection_WINDOW_WALL() -> Result<u64, JsValue> {
    let value = mrly_math::dissection::WINDOW_WALL;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_game_LAP() -> Result<u64, JsValue> {
    let value = mrly_math::game::LAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_ladder_ROUNDING() -> Result<f64, JsValue> {
    let value = mrly_math::ladder::ROUNDING;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_SPAN() -> Result<usize, JsValue> {
    let value = mrly_math::memory::SPAN;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_SWEEPS() -> Result<usize, JsValue> {
    let value = mrly_math::memory::SWEEPS;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_memory_TOLERANCE() -> Result<f64, JsValue> {
    let value = mrly_math::memory::TOLERANCE;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_press_CORNERS() -> Result<usize, JsValue> {
    let value = mrly_math::press::CORNERS;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_rules_BASE() -> Result<usize, JsValue> {
    let value = mrly_math::rules::BASE;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_shape_REFINE_LIMIT() -> Result<usize, JsValue> {
    let value = mrly_math::shape::REFINE_LIMIT;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_FILL() -> Result<u8, JsValue> {
    let value = mrly_math::six::FILL;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_GRID() -> Result<u8, JsValue> {
    let value = mrly_math::six::GRID;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_LEFT() -> Result<u8, JsValue> {
    let value = mrly_math::six::LEFT;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_RIGHT() -> Result<u8, JsValue> {
    let value = mrly_math::six::RIGHT;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_UP() -> Result<u8, JsValue> {
    let value = mrly_math::six::UP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_six_VOID() -> Result<u8, JsValue> {
    let value = mrly_math::six::VOID;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_LAPS_CAP() -> Result<usize, JsValue> {
    let value = mrly_math::spirograph::LAPS_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_PENCIL_CAP() -> Result<usize, JsValue> {
    let value = mrly_math::spirograph::PENCIL_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_POINT_CAP() -> Result<usize, JsValue> {
    let value = mrly_math::spirograph::POINT_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_RADIUS_CAP() -> Result<usize, JsValue> {
    let value = mrly_math::spirograph::RADIUS_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_RASTER_CAP() -> Result<usize, JsValue> {
    let value = mrly_math::spirograph::RASTER_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_spirograph_SIDES() -> Result<JsValue, JsValue> {
    let value = mrly_math::spirograph::SIDES;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn math_sumset_DEEPEST() -> Result<u32, JsValue> {
    let value = mrly_math::sumset::DEEPEST;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_sumset_WIDEST() -> Result<u32, JsValue> {
    let value = mrly_math::sumset::WIDEST;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_COVER() -> Result<f64, JsValue> {
    let value = mrly_math::three::sponge::COVER;
    Ok(value)
}

#[wasm_bindgen]
pub fn math_three_sponge_EDGE() -> Result<f64, JsValue> {
    let value = mrly_math::three::sponge::EDGE;
    Ok(value)
}

#[wasm_bindgen]
pub struct math_automaton_Automaton {
    inner: mrly_math::automaton::Automaton,
}

#[wasm_bindgen]
impl math_automaton_Automaton {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_automaton_Automaton, JsValue> {
        Ok(math_automaton_Automaton {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    pub fn cofactor(&self, s: JsValue, tolerance: f64) -> Result<JsValue, JsValue> {
        let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
        let value = self.inner.cofactor(s, tolerance).map_err(hand::throw)?;
        hand::to_js(&value)
    }
    pub fn denominator(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.denominator();
        Ok(value)
    }
    pub fn matrix(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.matrix();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    #[wasm_bindgen(constructor)]
    pub fn new(rule: &math_memory_Rule) -> Result<math_automaton_Automaton, JsValue> {
        let value = mrly_math::automaton::Automaton::new(&rule.inner).map_err(hand::throw)?;
        Ok(math_automaton_Automaton { inner: value })
    }
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    pub fn perron(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.perron();
        hand::to_js(&value)
    }
    pub fn residue(&self, w0: JsValue, tolerance: f64) -> Result<JsValue, JsValue> {
        let w0 = hand::from_js::<mrly_num::zeta::Complex>(&w0)?;
        let value = self.inner.residue(w0, tolerance).map_err(hand::throw)?;
        hand::to_js(&value)
    }
    pub fn rule(&self) -> Result<math_memory_Rule, JsValue> {
        let value = self.inner.rule();
        Ok(math_memory_Rule { inner: value })
    }
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    pub fn with_peel(
        rule: &math_memory_Rule,
        peel: usize,
    ) -> Result<math_automaton_Automaton, JsValue> {
        let value =
            mrly_math::automaton::Automaton::with_peel(&rule.inner, peel).map_err(hand::throw)?;
        Ok(math_automaton_Automaton { inner: value })
    }
    pub fn zeta(&self, s: JsValue, tolerance: f64) -> Result<JsValue, JsValue> {
        let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
        let value = self.inner.zeta(s, tolerance).map_err(hand::throw)?;
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_bang_Design {
    inner: mrly_math::bang::Design,
}

#[wasm_bindgen]
impl math_bang_Design {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_bang_Design, JsValue> {
        Ok(math_bang_Design {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn anf(&self) -> Result<String, JsValue> {
        let value = self.inner.anf();
        Ok(value)
    }
    pub fn degree(&self) -> Result<i32, JsValue> {
        let value = self.inner.degree();
        Ok(value)
    }
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name().map_err(hand::throw)?;
        Ok(value)
    }
    pub fn rule(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.rule();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
}

#[wasm_bindgen]
pub struct math_bang_Universe {
    inner: mrly_math::bang::Universe,
}

#[wasm_bindgen]
impl math_bang_Universe {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_bang_Universe, JsValue> {
        Ok(math_bang_Universe {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn all(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.all();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_bang_Design { inner: x1.clone() }))
        })
    }
    pub fn canonical(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.canonical();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_bang_Design { inner: x1.clone() }))
        })
    }
    pub fn design(&self, code: JsValue) -> Result<math_bang_Design, JsValue> {
        let code = hand::code_from_js(&code)?;
        let value = self.inner.design(code);
        Ok(math_bang_Design { inner: value })
    }
    pub fn distinct(&self) -> Result<usize, JsValue> {
        let value = self.inner.distinct();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize) -> Result<math_bang_Universe, JsValue> {
        let value = mrly_math::bang::Universe::new(dimension).map_err(hand::throw)?;
        Ok(math_bang_Universe { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_counts_Closed {
    inner: mrly_math::counts::Closed,
}

#[wasm_bindgen]
impl math_counts_Closed {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_counts_Closed, JsValue> {
        Ok(math_counts_Closed {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn text(&self) -> Result<String, JsValue> {
        let value = self.inner.text();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_counts_Exposure {
    inner: mrly_math::counts::Exposure,
}

#[wasm_bindgen]
impl math_counts_Exposure {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_counts_Exposure, JsValue> {
        Ok(math_counts_Exposure {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn at(&self, level: u32) -> Result<JsValue, JsValue> {
        let value = self.inner.at(level);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    pub fn from_corners(
        filled: JsValue,
        number: usize,
        dimension: usize,
        base: usize,
    ) -> Result<math_counts_Exposure, JsValue> {
        let filled = hand::from_js::<Vec<Vec<u8>>>(&filled)?;
        let value = mrly_math::counts::Exposure::from_corners(&filled, number, dimension, base);
        Ok(math_counts_Exposure { inner: value })
    }
    pub fn of_tile(tile: JsValue) -> Result<math_counts_Exposure, JsValue> {
        let tile = hand::tensor_from_js(&tile)?;
        let value = mrly_math::counts::Exposure::of_tile(&tile);
        Ok(math_counts_Exposure { inner: value })
    }
    pub fn recurrence(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.recurrence();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
}

#[wasm_bindgen]
pub struct math_counts_Placements {
    inner: mrly_math::counts::Placements,
}

#[wasm_bindgen]
impl math_counts_Placements {
    pub fn at(&self, level: u32) -> Result<Option<math_counts_Skeleton>, JsValue> {
        let value = self.inner.at(level);
        Ok(value.map(|inner| math_counts_Skeleton { inner }))
    }
    pub fn of_tile(tile: JsValue) -> Result<math_counts_Placements, JsValue> {
        let tile = hand::tensor_from_js(&tile)?;
        let value = mrly_math::counts::Placements::of_tile(&tile).map_err(hand::throw)?;
        Ok(math_counts_Placements { inner: value })
    }
    pub fn recurrence(&self, k: JsValue) -> Result<JsValue, JsValue> {
        let k = hand::from_js::<Option<usize>>(&k)?;
        let value = self.inner.recurrence(k);
        hand::option_to_js(value.as_ref(), |x1| {
            hand::list_to_js(x1, |x2| Ok(JsValue::from_str(&x2.to_string())))
        })
    }
    pub fn roots(&self, k: JsValue) -> Result<JsValue, JsValue> {
        let k = hand::from_js::<Option<usize>>(&k)?;
        let value = self.inner.roots(k);
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from_str(&x1.0.to_string()),
                hand::to_js(&x1.1)?,
            ]))
        })
    }
}

#[wasm_bindgen]
pub struct math_counts_Skeleton {
    inner: mrly_math::counts::Skeleton,
}

#[wasm_bindgen]
impl math_counts_Skeleton {
    #[wasm_bindgen(getter)]
    pub fn counts(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.counts.clone();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
    #[wasm_bindgen(setter)]
    pub fn set_counts(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::list_from_js(&value, hand::u128_from_js)?;
        self.inner.counts = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn euler(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.euler;
        Ok(JsValue::from_str(&value.to_string()))
    }
    #[wasm_bindgen(setter)]
    pub fn set_euler(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::i128_from_js(&value)?;
        self.inner.euler = value;
        Ok(())
    }
    pub fn cells(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.cells();
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn edges(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.edges();
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn faces(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.faces();
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn vertices(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.vertices();
        Ok(JsValue::from_str(&value.to_string()))
    }
}

#[wasm_bindgen]
pub struct math_graph_Layout {
    inner: mrly_math::graph::Layout,
}

#[wasm_bindgen]
impl math_graph_Layout {
    pub fn energy(&self) -> Result<f64, JsValue> {
        let value = self.inner.energy();
        Ok(value)
    }
    pub fn from_network(
        network: &math_graph_Network,
        seed: JsValue,
    ) -> Result<math_graph_Layout, JsValue> {
        let seed = hand::u64_from_js(&seed)?;
        let value =
            mrly_math::graph::Layout::from_network(&network.inner, seed).map_err(hand::throw)?;
        Ok(math_graph_Layout { inner: value })
    }
    pub fn ideal(&self) -> Result<f64, JsValue> {
        let value = self.inner.ideal();
        Ok(value)
    }
    pub fn moved(&self) -> Result<f64, JsValue> {
        let value = self.inner.moved();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        positions: &[f64],
        branches: JsValue,
        dim: usize,
        seed: JsValue,
    ) -> Result<math_graph_Layout, JsValue> {
        let branches = hand::from_js::<Vec<(usize, usize)>>(&branches)?;
        let seed = hand::u64_from_js(&seed)?;
        let value =
            mrly_math::graph::Layout::new(positions, &branches, dim, seed).map_err(hand::throw)?;
        Ok(math_graph_Layout { inner: value })
    }
    pub fn nodes(&self) -> Result<usize, JsValue> {
        let value = self.inner.nodes();
        Ok(value)
    }
    pub fn positions(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.positions();
        Ok(value.to_vec())
    }
    pub fn step(&mut self, ticks: usize) -> Result<f64, JsValue> {
        let value = self.inner.step(ticks);
        Ok(value)
    }
    pub fn temperature(&self) -> Result<f64, JsValue> {
        let value = self.inner.temperature();
        Ok(value)
    }
    pub fn ticks(&self) -> Result<usize, JsValue> {
        let value = self.inner.ticks();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_graph_Network {
    inner: mrly_math::graph::Network,
}

#[wasm_bindgen]
impl math_graph_Network {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_graph_Network, JsValue> {
        Ok(math_graph_Network {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.nodes.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_nodes(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrly_math::graph::Node>>(&value)?;
        self.inner.nodes = value;
        Ok(())
    }
    #[wasm_bindgen(getter)]
    pub fn branches(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.branches.clone();
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_branches(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<Vec<mrly_math::graph::Branch>>(&value)?;
        self.inner.branches = value;
        Ok(())
    }
    pub fn add_branch(&mut self, parent: usize, child: usize, radius: f64) -> Result<(), JsValue> {
        self.inner
            .add_branch(parent, child, radius)
            .map_err(hand::throw)?;
        Ok(())
    }
    pub fn add_node(&mut self, position: Vec<f64>) -> Result<usize, JsValue> {
        let value = self.inner.add_node(position).map_err(hand::throw)?;
        Ok(value)
    }
    pub fn adjacency(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.adjacency().map_err(hand::throw)?;
        hand::map_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    pub fn degree(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.degree().map_err(hand::throw)?;
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(dim: usize) -> Result<math_graph_Network, JsValue> {
        let value = mrly_math::graph::Network::new(dim);
        Ok(math_graph_Network { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_ladder_Design {
    inner: mrly_math::ladder::Design,
}

#[wasm_bindgen]
impl math_ladder_Design {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_ladder_Design, JsValue> {
        Ok(math_ladder_Design {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    pub fn digits(&self) -> Result<Vec<u64>, JsValue> {
        let value = self.inner.digits();
        Ok(value.to_vec())
    }
    #[wasm_bindgen(constructor)]
    pub fn new(base: JsValue, digits: JsValue) -> Result<math_ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value = mrly_math::ladder::Design::new(base, &digits).map_err(hand::throw)?;
        Ok(math_ladder_Design { inner: value })
    }
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    pub fn pole(&self, m: usize, j: JsValue) -> Result<JsValue, JsValue> {
        let j = hand::i64_from_js(&j)?;
        let value = self.inner.pole(m, j);
        hand::to_js(&value)
    }
    pub fn with_peel(
        base: JsValue,
        digits: JsValue,
        peel: usize,
    ) -> Result<math_ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value =
            mrly_math::ladder::Design::with_peel(base, &digits, peel).map_err(hand::throw)?;
        Ok(math_ladder_Design { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_memory_Rule {
    inner: mrly_math::memory::Rule,
}

#[wasm_bindgen]
impl math_memory_Rule {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_memory_Rule, JsValue> {
        Ok(math_memory_Rule {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn accepts(&self, word: &[usize]) -> Result<bool, JsValue> {
        let value = self.inner.accepts(word);
        Ok(value)
    }
    pub fn allowed(&self, window: usize) -> Result<bool, JsValue> {
        let value = self.inner.allowed(window);
        Ok(value)
    }
    pub fn alphabet(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.alphabet();
        Ok(value)
    }
    pub fn codes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.codes();
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn full(dimension: usize, width: usize) -> Result<math_memory_Rule, JsValue> {
        let value = mrly_math::memory::Rule::full(dimension, width).map_err(hand::throw)?;
        Ok(math_memory_Rule { inner: value })
    }
    pub fn letters(&self) -> Result<usize, JsValue> {
        let value = self.inner.letters();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize, width: usize, code: JsValue) -> Result<math_memory_Rule, JsValue> {
        let code = hand::u64_from_js(&code)?;
        let value = mrly_math::memory::Rule::new(dimension, width, code).map_err(hand::throw)?;
        Ok(math_memory_Rule { inner: value })
    }
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    pub fn windows(&self) -> Result<usize, JsValue> {
        let value = self.inner.windows();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_moire_Field {
    inner: mrly_math::moire::Field,
}

#[wasm_bindgen]
impl math_moire_Field {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_moire_Field, JsValue> {
        Ok(math_moire_Field {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn as_f64(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.as_f64();
        Ok(value)
    }
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<math_moire_Field, JsValue> {
        let value = mrly_math::moire::Field::from_data(data, size).map_err(hand::throw)?;
        Ok(math_moire_Field { inner: value })
    }
    pub fn max(&self) -> Result<f32, JsValue> {
        let value = self.inner.max();
        Ok(value)
    }
    pub fn mean(&self) -> Result<f64, JsValue> {
        let value = self.inner.mean();
        Ok(value)
    }
    pub fn min(&self) -> Result<f32, JsValue> {
        let value = self.inner.min();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Result<math_moire_Field, JsValue> {
        let value = mrly_math::moire::Field::new(size);
        Ok(math_moire_Field { inner: value })
    }
    pub fn normalized(&self, symmetric: bool) -> Result<Vec<f32>, JsValue> {
        let value = self.inner.normalized(symmetric);
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_moire_Preset {
    inner: mrly_math::moire::Preset,
}

#[wasm_bindgen]
impl math_moire_Preset {
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Result<String, JsValue> {
        let value = self.inner.name;
        Ok(value.to_string())
    }
    #[wasm_bindgen(getter)]
    pub fn spec(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.spec;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_spec(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_math::moire::Spec>(&value)?;
        self.inner.spec = value;
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
    pub fn combine(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.combine;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_combine(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_math::moire::Combine>(&value)?;
        self.inner.combine = value;
        Ok(())
    }
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
    #[wasm_bindgen(getter)]
    pub fn lattice(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.lattice;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_lattice(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_math::moire::Lattice>(&value)?;
        self.inner.lattice = value;
        Ok(())
    }
    pub fn carpet(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrly_math::moire::Preset::carpet(limit);
        Ok(math_moire_Preset { inner: value })
    }
    pub fn field(&self, size: usize) -> Result<math_moire_Field, JsValue> {
        let value = self.inner.field(size).map_err(hand::throw)?;
        Ok(math_moire_Field { inner: value })
    }
    pub fn heatmap(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrly_math::moire::Preset::heatmap(limit);
        Ok(math_moire_Preset { inner: value })
    }
    pub fn hive(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrly_math::moire::Preset::hive(limit);
        Ok(math_moire_Preset { inner: value })
    }
    pub fn weave(limit: usize) -> Result<math_moire_Preset, JsValue> {
        let value = mrly_math::moire::Preset::weave(limit);
        Ok(math_moire_Preset { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_moire_Volume {
    inner: mrly_math::moire::Volume,
}

#[wasm_bindgen]
impl math_moire_Volume {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_moire_Volume, JsValue> {
        Ok(math_moire_Volume {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn at(&self, x: usize, y: usize, z: usize) -> Result<f32, JsValue> {
        let value = self.inner.at(x, y, z);
        Ok(value)
    }
    pub fn count(&self, level: f32) -> Result<usize, JsValue> {
        let value = self.inner.count(level);
        Ok(value)
    }
    pub fn from_data(data: Vec<f32>, size: usize) -> Result<math_moire_Volume, JsValue> {
        let value = mrly_math::moire::Volume::from_data(data, size).map_err(hand::throw)?;
        Ok(math_moire_Volume { inner: value })
    }
    pub fn max(&self) -> Result<f32, JsValue> {
        let value = self.inner.max();
        Ok(value)
    }
    pub fn min(&self) -> Result<f32, JsValue> {
        let value = self.inner.min();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Result<math_moire_Volume, JsValue> {
        let value = mrly_math::moire::Volume::new(size);
        Ok(math_moire_Volume { inner: value })
    }
    pub fn plane(&self, frame: JsValue, out: usize) -> Result<JsValue, JsValue> {
        let frame = hand::from_js::<mrly_math::moire::Frame>(&frame)?;
        let value = self.inner.plane(&frame, out);
        Ok(hand::tuple_to_js(&[
            hand::typed(&(value.0)[..]),
            hand::typed(&(value.1)[..]),
        ]))
    }
    pub fn sample(&self, p: JsValue) -> Result<JsValue, JsValue> {
        let p = hand::from_js::<[f64; 3]>(&p)?;
        let value = self.inner.sample(p);
        hand::to_js(&value)
    }
    pub fn solid(&self, level: f32) -> Result<JsValue, JsValue> {
        let value = self.inner.solid(level);
        hand::tensor_to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_name_Bang {
    inner: mrly_math::name::Bang,
}

#[wasm_bindgen]
impl math_name_Bang {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Bang, JsValue> {
        Ok(math_name_Bang {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(getter)]
    pub fn lattice(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.lattice;
        hand::to_js(&value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_lattice(&mut self, value: JsValue) -> Result<(), JsValue> {
        let value = hand::from_js::<mrly_math::name::Lattice>(&value)?;
        self.inner.lattice = value;
        Ok(())
    }
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
    pub fn cells(&self) -> Result<u32, JsValue> {
        let value = self.inner.cells().map_err(hand::throw)?;
        Ok(value)
    }
    pub fn checked(&self) -> Result<math_name_Bang, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::checked(self.inner.clone())
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    pub fn from_file(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    pub fn from_json(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    pub fn from_url(text: &str) -> Result<math_name_Bang, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Bang { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue, dim: usize, base: usize) -> Result<math_name_Bang, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrly_math::name::Bang::new(code, dim, base);
        Ok(math_name_Bang { inner: value })
    }
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Bang as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_name_Sequence {
    inner: mrly_math::name::Sequence,
}

#[wasm_bindgen]
impl math_name_Sequence {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Sequence, JsValue> {
        Ok(math_name_Sequence {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn checked(&self) -> Result<math_name_Sequence, JsValue> {
        let value =
            <mrly_math::name::Sequence as mrly_core::name::Named>::checked(self.inner.clone())
                .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    pub fn design(&self) -> Result<math_name_Bang, JsValue> {
        let value = self.inner.design();
        Ok(math_name_Bang { inner: value })
    }
    pub fn from_file(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    pub fn from_json(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    pub fn from_url(text: &str) -> Result<math_name_Sequence, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Sequence { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        code: JsValue,
        dim: usize,
        base: usize,
        measure: &str,
        axis: &str,
    ) -> Result<math_name_Sequence, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrly_math::name::Sequence::new(code, dim, base, measure, axis);
        Ok(math_name_Sequence { inner: value })
    }
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Sequence as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_name_Word {
    inner: mrly_math::name::Word,
}

#[wasm_bindgen]
impl math_name_Word {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_name_Word, JsValue> {
        Ok(math_name_Word {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn side(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.side.clone();
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_side(&mut self, value: Vec<usize>) -> Result<(), JsValue> {
        self.inner.side = value;
        Ok(())
    }
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
    pub fn bases(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.bases();
        Ok(value)
    }
    pub fn checked(&self) -> Result<math_name_Word, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::checked(self.inner.clone())
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    pub fn from_file(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::from_file(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    pub fn from_json(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::from_json(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    pub fn from_url(text: &str) -> Result<math_name_Word, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::from_url(text)
            .map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    pub fn letters(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.letters();
        hand::list_to_js(&value, |x1| {
            Ok(JsValue::from(math_name_Bang { inner: x1.clone() }))
        })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(dim: usize, letters: JsValue) -> Result<math_name_Word, JsValue> {
        let letters = hand::list_from_js(&letters, |x1| {
            Ok((
                hand::u128_from_js(&hand::item(x1, 0)?)?,
                hand::from_js::<usize>(&hand::item(x1, 1)?)?,
            ))
        })?;
        let value = mrly_math::name::Word::new(dim, &letters).map_err(hand::throw)?;
        Ok(math_name_Word { inner: value })
    }
    pub fn to_file(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::to_file(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_id(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::to_id(&self.inner);
        Ok(value)
    }
    pub fn to_json(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::to_json(&self.inner);
        Ok(value)
    }
    pub fn to_prose(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::to_prose(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
    pub fn to_url(&self) -> Result<String, JsValue> {
        let value = <mrly_math::name::Word as mrly_core::name::Named>::to_url(&self.inner)
            .map_err(hand::throw)?;
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_press_Press {
    inner: mrly_math::press::Press,
}

#[wasm_bindgen]
impl math_press_Press {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_press_Press, JsValue> {
        Ok(math_press_Press {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn add(&mut self, number: JsValue, weight: JsValue) -> Result<(), JsValue> {
        let number = hand::u128_from_js(&number)?;
        let weight = hand::i128_from_js(&weight)?;
        self.inner.add(number, weight);
        Ok(())
    }
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize, base: usize) -> Result<math_press_Press, JsValue> {
        let value = mrly_math::press::Press::new(dimension, base).map_err(hand::throw)?;
        Ok(math_press_Press { inner: value })
    }
    pub fn total(&self, code: JsValue) -> Result<JsValue, JsValue> {
        let code = hand::code_from_js(&code)?;
        let value = self.inner.total(code);
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn totals(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.totals();
        hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
    }
}

#[wasm_bindgen]
pub struct math_radix_Base {
    inner: mrly_math::radix::Base,
}

#[wasm_bindgen]
impl math_radix_Base {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_radix_Base, JsValue> {
        Ok(math_radix_Base {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn class(&self, z: JsValue) -> Result<usize, JsValue> {
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let value = self.inner.class(z).map_err(hand::throw)?;
        Ok(value)
    }
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
    pub fn group(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.group().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    pub fn mirrored(&self) -> Result<bool, JsValue> {
        let value = self.inner.mirrored();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(ring: JsValue, value: JsValue) -> Result<math_radix_Base, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let value = (
            hand::i64_from_js(&hand::item(&value, 0)?)?,
            hand::i64_from_js(&hand::item(&value, 1)?)?,
        );
        let value = mrly_math::radix::Base::new(ring, value).map_err(hand::throw)?;
        Ok(math_radix_Base { inner: value })
    }
    pub fn norm(&self) -> Result<u64, JsValue> {
        let value = self.inner.norm();
        Ok(value)
    }
    pub fn power(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.power(level);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn residues(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.residues().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    pub fn value(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.value();
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
}

#[wasm_bindgen]
pub struct math_radix_Radix {
    inner: mrly_math::radix::Radix,
}

#[wasm_bindgen]
impl math_radix_Radix {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_radix_Radix, JsValue> {
        Ok(math_radix_Radix {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn base(&self) -> Result<math_radix_Base, JsValue> {
        let value = self.inner.base();
        Ok(math_radix_Base { inner: value })
    }
    pub fn canonical(&self) -> Result<bool, JsValue> {
        let value = self.inner.canonical().map_err(hand::throw)?;
        Ok(value)
    }
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code().map_err(hand::throw)?;
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn digits(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.digits();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    pub fn dimension(&self) -> Result<f64, JsValue> {
        let value = self.inner.dimension();
        Ok(value)
    }
    pub fn distinct(&self, level: usize) -> Result<usize, JsValue> {
        let value = self.inner.distinct(level);
        Ok(value)
    }
    pub fn fill(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.fill(level);
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn from_code(base: &math_radix_Base, code: JsValue) -> Result<math_radix_Radix, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrly_math::radix::Radix::from_code(base.inner, code).map_err(hand::throw)?;
        Ok(math_radix_Radix { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        base: &math_radix_Base,
        digits: JsValue,
        twists: JsValue,
    ) -> Result<math_radix_Radix, JsValue> {
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
            mrly_math::radix::Radix::new(base.inner, digits, twists).map_err(hand::throw)?;
        Ok(math_radix_Radix { inner: value })
    }
    pub fn plane(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.plane(level);
        hand::to_js(&value)
    }
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    pub fn size(&self) -> Result<usize, JsValue> {
        let value = self.inner.size();
        Ok(value)
    }
    pub fn twists(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.twists();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    pub fn with_twists(&self, units: &[usize]) -> Result<math_radix_Radix, JsValue> {
        let value = self.inner.clone().with_twists(units).map_err(hand::throw)?;
        Ok(math_radix_Radix { inner: value })
    }
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

#[wasm_bindgen]
pub struct math_roulette_Nodes {
    inner: mrly_math::roulette::Nodes,
}

#[wasm_bindgen]
impl math_roulette_Nodes {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_roulette_Nodes, JsValue> {
        Ok(math_roulette_Nodes {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<math_roulette_Nodes, JsValue> {
        let value = mrly_math::roulette::Nodes::default();
        Ok(math_roulette_Nodes { inner: value })
    }
    pub fn pair(&self, i: usize, j: usize) -> Result<usize, JsValue> {
        let value = self.inner.pair(i, j);
        Ok(value)
    }
    pub fn paired(&self) -> Result<usize, JsValue> {
        let value = self.inner.paired();
        Ok(value)
    }
    pub fn selved(&self) -> Result<usize, JsValue> {
        let value = self.inner.selved();
        Ok(value)
    }
    pub fn total(&self) -> Result<usize, JsValue> {
        let value = self.inner.total();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_shape_Frac {
    inner: mrly_math::shape::Frac,
}

#[wasm_bindgen]
impl math_shape_Frac {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_shape_Frac, JsValue> {
        Ok(math_shape_Frac {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn minus(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.minus(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(num: JsValue, den: JsValue) -> Result<math_shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let den = hand::i64_from_js(&den)?;
        let value = mrly_math::shape::Frac::new(num, den).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    pub fn plus(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.plus(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    pub fn times(&self, other: &math_shape_Frac) -> Result<math_shape_Frac, JsValue> {
        let value = self.inner.times(other.inner).map_err(hand::throw)?;
        Ok(math_shape_Frac { inner: value })
    }
    pub fn whole(num: JsValue) -> Result<math_shape_Frac, JsValue> {
        let num = hand::i64_from_js(&num)?;
        let value = mrly_math::shape::Frac::whole(num);
        Ok(math_shape_Frac { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_six_star_Share {
    inner: mrly_math::six::star::Share,
}

#[wasm_bindgen]
impl math_six_star_Share {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_six_star_Share, JsValue> {
        Ok(math_six_star_Share {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn reduced(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.reduced();
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn value(&self) -> Result<f64, JsValue> {
        let value = self.inner.value();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_six_star_Star {
    inner: mrly_math::six::star::Star,
}

#[wasm_bindgen]
impl math_six_star_Star {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_six_star_Star, JsValue> {
        Ok(math_six_star_Star {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn arm(&self, number: usize, half: usize) -> Result<math_six_star_Share, JsValue> {
        let value = self.inner.arm(number, half).map_err(hand::throw)?;
        Ok(math_six_star_Share { inner: value })
    }
    pub fn cell(&self, number: usize, x: JsValue, z: JsValue) -> Result<JsValue, JsValue> {
        let x = hand::i64_from_js(&x)?;
        let z = hand::i64_from_js(&z)?;
        let value = self.inner.cell(number, x, z);
        hand::to_js(&value)
    }
    pub fn excesses(&self, layers: usize, half: usize) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.excesses(layers, half).map_err(hand::throw)?;
        Ok(value)
    }
    pub fn hexagon(&self, number: usize) -> Result<math_six_star_Share, JsValue> {
        let value = self.inner.hexagon(number).map_err(hand::throw)?;
        Ok(math_six_star_Share { inner: value })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(code: JsValue) -> Result<math_six_star_Star, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrly_math::six::star::Star::new(code).map_err(hand::throw)?;
        Ok(math_six_star_Star { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_sumset_Pair {
    inner: mrly_math::sumset::Pair,
}

#[wasm_bindgen]
impl math_sumset_Pair {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_sumset_Pair, JsValue> {
        Ok(math_sumset_Pair {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn clean(&self) -> Result<bool, JsValue> {
        let value = self.inner.clean();
        Ok(value)
    }
    pub fn copy(&self) -> Result<bool, JsValue> {
        let value = self.inner.copy();
        Ok(value)
    }
    pub fn energy(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.energy();
        Ok(JsValue::from_str(&value.to_string()))
    }
    pub fn four(&self) -> Result<u32, JsValue> {
        let value = self.inner.four();
        Ok(value)
    }
    pub fn gap(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.gap();
        hand::option_to_js(value.as_ref(), |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    pub fn largest(&self) -> Result<u64, JsValue> {
        let value = self.inner.largest();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(three: u32, four: u32) -> Result<math_sumset_Pair, JsValue> {
        let value = mrly_math::sumset::Pair::new(three, four).map_err(hand::throw)?;
        Ok(math_sumset_Pair { inner: value })
    }
    pub fn ratio(&self, energy: JsValue) -> Result<f64, JsValue> {
        let energy = hand::u128_from_js(&energy)?;
        let value = self.inner.ratio(energy);
        Ok(value)
    }
    pub fn scale(&self) -> Result<f64, JsValue> {
        let value = self.inner.scale();
        Ok(value)
    }
    pub fn three(&self) -> Result<u32, JsValue> {
        let value = self.inner.three();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_sumset_Sumset {
    inner: mrly_math::sumset::Sumset,
}

#[wasm_bindgen]
impl math_sumset_Sumset {
    pub fn contains(&self, x: JsValue) -> Result<JsValue, JsValue> {
        let x = hand::u64_from_js(&x)?;
        let value = self.inner.contains(x);
        hand::to_js(&value)
    }
    pub fn count(&self, x: JsValue) -> Result<JsValue, JsValue> {
        let x = hand::u64_from_js(&x)?;
        let value = self.inner.count(x);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
    }
    pub fn density(&self, x: JsValue) -> Result<JsValue, JsValue> {
        let x = hand::u64_from_js(&x)?;
        let value = self.inner.density(x);
        hand::to_js(&value)
    }
    pub fn extremes(&self, edges: JsValue) -> Result<JsValue, JsValue> {
        let edges = hand::list_from_js(&edges, hand::u64_from_js)?;
        let value = self.inner.extremes(&edges).map_err(hand::throw)?;
        hand::to_js(&value)
    }
    pub fn fills(&self, low: JsValue, high: JsValue, cells: usize) -> Result<Vec<f64>, JsValue> {
        let low = hand::u64_from_js(&low)?;
        let high = hand::u64_from_js(&high)?;
        let value = self.inner.fills(low, high, cells).map_err(hand::throw)?;
        Ok(value)
    }
    pub fn level(&self) -> Result<u32, JsValue> {
        let value = self.inner.level();
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(level: u32) -> Result<math_sumset_Sumset, JsValue> {
        let value = mrly_math::sumset::Sumset::new(level).map_err(hand::throw)?;
        Ok(math_sumset_Sumset { inner: value })
    }
    pub fn top(&self) -> Result<u64, JsValue> {
        let value = self.inner.top();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_three_Vec3 {
    inner: mrly_math::three::Vec3,
}

#[wasm_bindgen]
impl math_three_Vec3 {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<math_three_Vec3, JsValue> {
        Ok(math_three_Vec3 {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn cross(&self, o: &math_three_Vec3) -> Result<math_three_Vec3, JsValue> {
        let value = self.inner.cross(o.inner);
        Ok(math_three_Vec3 { inner: value })
    }
    pub fn dot(&self, o: &math_three_Vec3) -> Result<f32, JsValue> {
        let value = self.inner.dot(o.inner);
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Result<math_three_Vec3, JsValue> {
        let value = mrly_math::three::Vec3::new(x, y, z);
        Ok(math_three_Vec3 { inner: value })
    }
    pub fn scale(&self, s: f32) -> Result<math_three_Vec3, JsValue> {
        let value = self.inner.scale(s);
        Ok(math_three_Vec3 { inner: value })
    }
}

#[wasm_bindgen]
pub struct math_bang_MagicLayer {}

#[wasm_bindgen]
impl math_bang_MagicLayer {
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(design: &math_name_Bang, number: usize) -> Result<JsValue, JsValue> {
        let value = mrly_math::bang::MagicLayer::new(design.inner.clone(), number);
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_bang_catalog_Design {}

#[wasm_bindgen]
impl math_bang_catalog_Design {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_math::bang::catalog::Design::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_bang_word_Schedule {}

#[wasm_bindgen]
impl math_bang_word_Schedule {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_math::bang::word::Schedule::all();
        hand::to_js(&value)
    }
    pub fn frequencies(schedule: JsValue) -> Result<JsValue, JsValue> {
        let schedule = hand::from_js::<mrly_math::bang::word::Schedule>(&schedule)?;
        let value = schedule.frequencies();
        hand::to_js(&value)
    }
    pub fn place(schedule: JsValue, index: usize) -> Result<usize, JsValue> {
        let schedule = hand::from_js::<mrly_math::bang::word::Schedule>(&schedule)?;
        let value = schedule.place(index);
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_moire_Layer {}

#[wasm_bindgen]
impl math_moire_Layer {
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(spec: JsValue, number: usize) -> Result<JsValue, JsValue> {
        let spec = hand::from_js::<mrly_math::moire::Spec>(&spec)?;
        let value = mrly_math::moire::Layer::new(spec, number);
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_moire_Spec {}

#[wasm_bindgen]
impl math_moire_Spec {
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(code: JsValue, base: usize, dimension: usize) -> Result<JsValue, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrly_math::moire::Spec::new(code, base, dimension);
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_name_Lattice {}

#[wasm_bindgen]
impl math_name_Lattice {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_math::name::Lattice::default();
        hand::to_js(&value)
    }
    pub fn is_square(lattice: JsValue) -> Result<bool, JsValue> {
        let lattice = hand::from_js::<mrly_math::name::Lattice>(&lattice)?;
        let value = lattice.is_square();
        Ok(value)
    }
    pub fn units(lattice: JsValue) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrly_math::name::Lattice>(&lattice)?;
        let value = lattice.units();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct math_shape_Region {}

#[wasm_bindgen]
impl math_shape_Region {
    pub fn flip(region: JsValue) -> Result<JsValue, JsValue> {
        let region = hand::from_js::<mrly_math::shape::Region>(&region)?;
        let value = region.flip();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_six_star_Branch {}

#[wasm_bindgen]
impl math_six_star_Branch {
    pub fn constant(branch: JsValue) -> Result<f64, JsValue> {
        let branch = hand::from_js::<mrly_math::six::star::Branch>(&branch)?;
        let value = branch.constant();
        Ok(value)
    }
    pub fn name(branch: JsValue) -> Result<String, JsValue> {
        let branch = hand::from_js::<mrly_math::six::star::Branch>(&branch)?;
        let value = branch.name();
        Ok(value)
    }
    pub fn of(layers: usize) -> Result<JsValue, JsValue> {
        let value = mrly_math::six::star::Branch::of(layers);
        hand::to_js(&value)
    }
    pub fn residual(branch: JsValue) -> Result<JsValue, JsValue> {
        let branch = hand::from_js::<mrly_math::six::star::Branch>(&branch)?;
        let value = branch.residual();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_spin_Blend {}

#[wasm_bindgen]
impl math_spin_Blend {
    pub fn fold(blend: JsValue, values: &[f32]) -> Result<f32, JsValue> {
        let blend = hand::from_js::<mrly_math::spin::Blend>(&blend)?;
        let value = blend.fold(values);
        Ok(value)
    }
    pub fn named(name: &str) -> Result<JsValue, JsValue> {
        let value = mrly_math::spin::Blend::named(name);
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct math_spirograph_Seats {}

#[wasm_bindgen]
impl math_spirograph_Seats {
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_math::spirograph::Seats::default();
        hand::to_js(&value)
    }
}
