#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

use mrly_core::hand::js as hand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn num_apollonian_flat(name: &str, cap: JsValue) -> Result<Vec<f64>, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrly_num::apollonian::flat(name, cap).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_form(u: JsValue, v: JsValue) -> Result<JsValue, JsValue> {
    let u = hand::array_from_js::<_, 4>(&u, hand::i64_from_js)?;
    let v = hand::array_from_js::<_, 4>(&v, hand::i64_from_js)?;
    let value = mrly_num::apollonian::form(u, v);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_apollonian_frame(p: JsValue) -> Result<Vec<f64>, JsValue> {
    let p = hand::from_js::<mrly_num::apollonian::Packing>(&p)?;
    let value = mrly_num::apollonian::frame(&p);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn num_apollonian_grow(name: &str, cap: JsValue) -> Result<JsValue, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrly_num::apollonian::grow(name, cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_apollonian_is_ford(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrly_num::apollonian::is_ford(c.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_on_line(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrly_num::apollonian::on_line(c.inner);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_reflect(q: JsValue, at: usize) -> Result<num_apollonian_Circle, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly_num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly_num::apollonian::reflect(&q, at);
    Ok(num_apollonian_Circle { inner: value })
}

#[wasm_bindgen]
pub fn num_apollonian_root(name: &str) -> Result<JsValue, JsValue> {
    let value = mrly_num::apollonian::root(name).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

#[wasm_bindgen]
pub fn num_apollonian_shadow(p: JsValue, order: usize) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrly_num::apollonian::Packing>(&p)?;
    let value = mrly_num::apollonian::shadow(&p, order).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_apollonian_sound(q: JsValue) -> Result<bool, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly_num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly_num::apollonian::sound(&q);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_swap(q: JsValue, at: usize) -> Result<JsValue, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly_num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly_num::apollonian::swap(&q, at);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

#[wasm_bindgen]
pub fn num_apollonian_touches(p: JsValue) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrly_num::apollonian::Packing>(&p)?;
    let value = mrly_num::apollonian::touches(&p);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_blend_add(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly_num::blend::add(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_cauchy(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly_num::blend::cauchy(&a, &b).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_characteristic(coefficients: JsValue) -> Result<JsValue, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrly_num::blend::characteristic(&coefficients);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

#[wasm_bindgen]
pub fn num_blend_decimate(a: JsValue, step: usize, offset: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly_num::blend::decimate(&a, step, offset).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_delta(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly_num::blend::delta(&a);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_growth(coefficients: JsValue) -> Result<f64, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrly_num::blend::growth(&coefficients);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_blend_hadamard(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly_num::blend::hadamard(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_recurrence(terms: JsValue) -> Result<JsValue, JsValue> {
    let terms = hand::list_from_js(&terms, hand::i128_from_js)?;
    let value = mrly_num::blend::recurrence(&terms);
    hand::option_to_js(value.as_ref(), |x1| {
        hand::list_to_js(x1, |x2| {
            Ok(hand::tuple_to_js(&[
                JsValue::from_str(&x2.0.to_string()),
                JsValue::from_str(&x2.1.to_string()),
            ]))
        })
    })
}

#[wasm_bindgen]
pub fn num_blend_scale(a: JsValue, factor: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let factor = hand::i128_from_js(&factor)?;
    let value = mrly_num::blend::scale(&a, factor);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_shift(a: JsValue, count: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly_num::blend::shift(&a, count);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_sigma(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly_num::blend::sigma(&a).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_blend_sub(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly_num::blend::sub(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

#[wasm_bindgen]
pub fn num_boolean_is_balanced(code: JsValue, n: usize) -> Result<bool, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_num::boolean::is_balanced(code, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_boolean_nonlinearity(code: JsValue, n: usize) -> Result<i64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_num::boolean::nonlinearity(code, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_boolean_sac(code: JsValue, n: usize) -> Result<f64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_num::boolean::sac(code, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_boolean_walsh_spectrum(code: JsValue, n: usize) -> Result<Vec<i64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_num::boolean::walsh_spectrum(code, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_chladni_ladder(cap: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::chladni::ladder(cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_chladni_lines(field: &[f64], side: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::chladni::lines(field, side).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_chladni_plate(n: usize, m: usize, sign: i32, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::chladni::plate(n, m, sign, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_chladni_shake(
    field: &[f64],
    side: usize,
    grains: &[f32],
    from: JsValue,
    steps: usize,
    seed: JsValue,
) -> Result<Vec<f32>, JsValue> {
    let from = hand::u64_from_js(&from)?;
    let seed = hand::u64_from_js(&seed)?;
    let value =
        mrly_num::chladni::shake(field, side, grains, from, steps, seed).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_aliquot(number: usize) -> Result<usize, JsValue> {
    let value = mrly_num::factor::aliquot(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_coprime(a: usize, b: usize) -> Result<bool, JsValue> {
    let value = mrly_num::factor::coprime(a, b);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_divisors(number: JsValue) -> Result<Vec<u64>, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly_num::factor::divisors(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_factorial(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::factor::factorial(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_factor_factorize(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::factor::factorize(number);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_factor_factorize_wide(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly_num::factor::factorize_wide(number);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            hand::to_js(&x1.1)?,
        ]))
    })
}

#[wasm_bindgen]
pub fn num_factor_gcd(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::u128_from_js(&a)?;
    let b = hand::u128_from_js(&b)?;
    let value = mrly_num::factor::gcd(a, b);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_factor_lcm(a: usize, b: usize) -> Result<usize, JsValue> {
    let value = mrly_num::factor::lcm(a, b);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_mobius(number: usize) -> Result<i8, JsValue> {
    let value = mrly_num::factor::mobius(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_mobius_sieve(limit: usize) -> Result<Vec<i8>, JsValue> {
    let value = mrly_num::factor::mobius_sieve(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_radical(number: usize) -> Result<usize, JsValue> {
    let value = mrly_num::factor::radical(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_reduce(numerator: JsValue, denominator: JsValue) -> Result<JsValue, JsValue> {
    let numerator = hand::u128_from_js(&numerator)?;
    let denominator = hand::u128_from_js(&denominator)?;
    let value = mrly_num::factor::reduce(numerator, denominator);
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

#[wasm_bindgen]
pub fn num_factor_sigma(number: usize, power: u32) -> Result<JsValue, JsValue> {
    let value = mrly_num::factor::sigma(number, power);
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_factor_squarefree(number: usize) -> Result<bool, JsValue> {
    let value = mrly_num::factor::squarefree(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_totient(number: usize) -> Result<usize, JsValue> {
    let value = mrly_num::factor::totient(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_totients(n: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly_num::factor::totients(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_factor_twisted(number: usize, rhythm: &[i8]) -> Result<i64, JsValue> {
    let value = mrly_num::factor::twisted(number, rhythm);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_convolve(field: &[f64], kernel: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::fft::convolve(field, kernel, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_convolve_with(
    field: &[f64],
    kernel_re: &[f64],
    kernel_im: &[f64],
    size: usize,
) -> Result<Vec<f64>, JsValue> {
    let value =
        mrly_num::fft::convolve_with(field, kernel_re, kernel_im, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_embed_kernel(mask: &[u8], side: usize, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::fft::embed_kernel(mask, side, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_log_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::fft::log_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_magnitude_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::fft::magnitude_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_peak_ring(profile: &[f64]) -> Result<usize, JsValue> {
    let value = mrly_num::fft::peak_ring(profile);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_peak_wavelength(profile: &[f64], size: usize) -> Result<f64, JsValue> {
    let value = mrly_num::fft::peak_wavelength(profile, size);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_radial_profile(spectrum: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::fft::radial_profile(spectrum, size).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_fft_transform(field: &[f64], size: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::fft::transform(field, size).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

#[wasm_bindgen]
pub fn num_gauss_classes(ring: JsValue, bound: JsValue) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
    let bound = hand::u64_from_js(&bound)?;
    let value = mrly_num::gauss::classes(ring, bound);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn num_gauss_peak(ring: JsValue, limit: usize) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
    let value = mrly_num::gauss::peak(ring, limit);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_gauss_shells(ring: JsValue, limit: usize) -> Result<Vec<u32>, JsValue> {
    let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
    let value = mrly_num::gauss::shells(ring, limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_kummer_carry(prime: JsValue, top: JsValue, bottom: JsValue) -> Result<JsValue, JsValue> {
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::carry(prime, top, bottom).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_kummer_columns(
    k: JsValue,
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
) -> Result<JsValue, JsValue> {
    let k = hand::u64_from_js(&k)?;
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::columns(k, prime, top, bottom).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_kummer_count(
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
    level: u32,
) -> Result<JsValue, JsValue> {
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::count(prime, top, bottom, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_kummer_member(
    k: JsValue,
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
) -> Result<bool, JsValue> {
    let k = hand::u64_from_js(&k)?;
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::member(k, prime, top, bottom).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_kummer_valuation(
    k: JsValue,
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
) -> Result<u32, JsValue> {
    let k = hand::u64_from_js(&k)?;
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::valuation(k, prime, top, bottom).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_kummer_witness(
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
) -> Result<JsValue, JsValue> {
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly_num::kummer::witness(prime, top, bottom).map_err(hand::throw)?;
    hand::option_to_js(value.as_ref(), |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

#[wasm_bindgen]
pub fn num_lattice_coprime_pairs(n: usize) -> Result<u64, JsValue> {
    let value = mrly_num::lattice::coprime_pairs(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_lattice_farey(order: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::lattice::farey(order);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_lattice_grid(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::lattice::grid(n);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_lattice_new_nodes(n: usize) -> Result<u64, JsValue> {
    let value = mrly_num::lattice::new_nodes(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_lattice_pi_estimate(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::lattice::pi_estimate(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_lattice_recovered(n: usize, dimension: u32) -> Result<f64, JsValue> {
    let value = mrly_num::lattice::recovered(n, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_lattice_visible_density(dimension: u32) -> Result<f64, JsValue> {
    let value = mrly_num::lattice::visible_density(dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_lattice_zeta_factor(dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrly_num::lattice::zeta_factor(dimension);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_lattice_zeta_whole(s: u32) -> Result<f64, JsValue> {
    let value = mrly_num::lattice::zeta_whole(s).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_boundary(word: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::boundary(word);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_difference(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::difference(a, b);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_digits(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::digits(length);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_doubling(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::doubling(length);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_faults(a: &[u8], b: &[u8]) -> Result<usize, JsValue> {
    let value = mrly_num::morse::faults(a, b);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_fold(grid: &[u8], side: usize, number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::morse::fold(grid, side, number).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_morse_letter(place: JsValue) -> Result<u8, JsValue> {
    let place = hand::u64_from_js(&place)?;
    let value = mrly_num::morse::letter(place);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_lift(kind: JsValue, side: usize) -> Result<Vec<u8>, JsValue> {
    let kind = hand::from_js::<mrly_num::morse::Lift>(&kind)?;
    let value = mrly_num::morse::lift(kind, side);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_mask(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::mask(a, b);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_power(tile: &[u8], number: usize, level: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::power(tile, number, level).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_repeat(tile: &[u8], number: usize, side: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::repeat(tile, number, side);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_runs(word: &[u8]) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::morse::runs(word);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_signs(types: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::signs(types);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_stage(rounds: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::stage(rounds);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_substitution(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::substitution(length);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_upsample(grid: &[u8], side: usize, scale: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly_num::morse::upsample(grid, side, scale);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_chart(top: usize, bins: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::chart(top, bins);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_flags(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::flags(limit);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_goldbach(number: usize) -> Result<usize, JsValue> {
    let value = mrly_num::prime::goldbach(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_goldbach_record(top: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::prime::goldbach_record(top);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_is_prime(number: usize) -> Result<bool, JsValue> {
    let value = mrly_num::prime::is_prime(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_pile(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly_num::prime::pile(number);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_prime_count(n: usize) -> Result<usize, JsValue> {
    let value = mrly_num::prime::prime_count(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_prime_from(number: usize) -> Result<usize, JsValue> {
    let value = mrly_num::prime::prime_from(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_primes(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::prime::primes(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_prime_rectangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::rectangles(number);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_splits(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::splits(number);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_squares(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::squares(number);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_prime_study(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::prime::study(limit);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_arcs(design: JsValue, cut: f64) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let value = mrly_num::readings::arcs(design, cut);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_battery(stream: &[i8]) -> Result<JsValue, JsValue> {
    let value = mrly_num::readings::battery(stream);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_bias(stream: &[i8], exponents: &[u32]) -> Result<Vec<i64>, JsValue> {
    let value = mrly_num::readings::bias(stream, exponents);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_readings_designs() -> Result<JsValue, JsValue> {
    let value = mrly_num::readings::designs();
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_model(
    design: JsValue,
    cut: f64,
    exponents: &[u32],
) -> Result<Vec<f64>, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let value = mrly_num::readings::model(design, cut, exponents);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_readings_name(design: JsValue) -> Result<String, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let value = mrly_num::readings::name(design);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_readings_near_heads(design: JsValue, reading: JsValue) -> Result<JsValue, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let reading = hand::from_js::<mrly_num::readings::Reading>(&reading)?;
    let value = mrly_num::readings::near_heads(design, reading);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_sign(design: JsValue, n: JsValue) -> Result<i8, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let n = hand::u64_from_js(&n)?;
    let value = mrly_num::readings::sign(design, n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_readings_stream(
    design: JsValue,
    reading: JsValue,
    count: u32,
) -> Result<Vec<i8>, JsValue> {
    let design = hand::from_js::<mrly_num::readings::Design>(&design)?;
    let reading = hand::from_js::<mrly_num::readings::Reading>(&reading)?;
    let value = mrly_num::readings::stream(design, reading, count);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_basel(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::basel(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_bernoulli(count: usize) -> Result<JsValue, JsValue> {
    let value = mrly_num::series::bernoulli(count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

#[wasm_bindgen]
pub fn num_series_beta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::beta(s, terms);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_binary(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::series::binary(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_catalan(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::series::catalan(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_chi3(number: usize) -> Result<i8, JsValue> {
    let value = mrly_num::series::chi3(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_chi4(number: usize) -> Result<i8, JsValue> {
    let value = mrly_num::series::chi4(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_chi8(number: usize) -> Result<i8, JsValue> {
    let value = mrly_num::series::chi8(number);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_dirichlet(s: f64, rhythm: &[i8], terms: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::dirichlet(s, rhythm, terms);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_e_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::e_partial(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_euler_gamma_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::euler_gamma_partial(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_euler_product(s: f64, limit: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::euler_product(s, limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_evens(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::series::evens(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_fibonacci(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::series::fibonacci(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_harmonic(terms: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::harmonic(terms);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_lambda(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::lambda(s, terms).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_leibniz(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::leibniz(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_li(x: f64) -> Result<f64, JsValue> {
    let value = mrly_num::series::li(x);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_mertens(n: usize) -> Result<i64, JsValue> {
    let value = mrly_num::series::mertens(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_odds(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly_num::series::odds(limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_visible(limit: usize, dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrly_num::series::visible(limit, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_series_wallis_half_pi(n: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::wallis_half_pi(n);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_wallis_quarter_pi(factors: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::wallis_quarter_pi(factors);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_zeta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrly_num::series::zeta(s, terms).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_cells(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::cells(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_sieve_exponent(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::exponent(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_flat_word(side: JsValue, levels: usize) -> Result<Vec<u64>, JsValue> {
    let side = hand::u64_from_js(&side)?;
    let value = mrly_num::sieve::flat_word(side, levels);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_holes(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::holes(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_sieve_limit(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::limit(&word, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_sieve_odd_word(levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly_num::sieve::odd_word(levels);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_parity_product(dimension: u32, odd: bool) -> Result<f64, JsValue> {
    let value = mrly_num::sieve::parity_product(dimension, odd).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_punctures(word: JsValue, dimension: u32) -> Result<Vec<u64>, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::punctures(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_raster(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::raster(&word).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::typed(&(value.1)[..]),
    ]))
}

#[wasm_bindgen]
pub fn num_sieve_ratio(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::ratio(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_row_fill(profile: JsValue, side: JsValue) -> Result<JsValue, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let side = hand::u64_from_js(&side)?;
    let value = mrly_num::sieve::row_fill(&profile, side).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_sieve_row_law(profile: JsValue) -> Result<JsValue, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let value = mrly_num::sieve::row_law(&profile).map_err(hand::throw)?;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_sieve_row_profile(code: JsValue, dimension: usize) -> Result<Vec<u64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly_num::sieve::row_profile(code, dimension).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_row_settle(
    profile: JsValue,
    stops: &[usize],
    even: bool,
) -> Result<Vec<f64>, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let value = mrly_num::sieve::row_settle(&profile, stops, even).map_err(hand::throw)?;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_side(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly_num::sieve::side(&word).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

#[wasm_bindgen]
pub fn num_sieve_solid_limit() -> Result<f64, JsValue> {
    let value = mrly_num::sieve::solid_limit();
    Ok(value)
}

#[wasm_bindgen]
pub fn num_spiral_diagonal(
    lattice: JsValue,
    side: usize,
    a: JsValue,
    b: JsValue,
    c: JsValue,
) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
    let a = hand::i64_from_js(&a)?;
    let b = hand::i64_from_js(&b)?;
    let c = hand::i64_from_js(&c)?;
    let value = mrly_num::spiral::diagonal(lattice, side, a, b, c);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_spiral_level_of(n: JsValue, base: JsValue) -> Result<u32, JsValue> {
    let n = hand::u64_from_js(&n)?;
    let base = hand::u64_from_js(&base)?;
    let value = mrly_num::spiral::level_of(n, base);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_spiral_marks(mark: JsValue, limit: usize) -> Result<Vec<i8>, JsValue> {
    let mark = hand::from_js::<mrly_num::spiral::Mark>(&mark)?;
    let value = mrly_num::spiral::marks(mark, limit);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_spiral_snail(base: JsValue, top: JsValue, growth: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let top = hand::u64_from_js(&top)?;
    let growth = hand::from_js::<mrly_num::spiral::Growth>(&growth)?;
    let value = mrly_num::spiral::snail(base, top, growth);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_zeta_bump(u: f64) -> Result<f64, JsValue> {
    let value = mrly_num::zeta::bump(u);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_corrections(p: f64) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::zeta::corrections(p);
    Ok(value.to_vec())
}

#[wasm_bindgen]
pub fn num_zeta_kernel(p: f64) -> Result<f64, JsValue> {
    let value = mrly_num::zeta::kernel(p);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_mellin(s: JsValue) -> Result<JsValue, JsValue> {
    let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
    let value = mrly_num::zeta::mellin(s);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_zeta_novelty_main() -> Result<f64, JsValue> {
    let value = mrly_num::zeta::novelty_main();
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_novelty_wave(gammas: &[f64], coef: JsValue, log_y: f64) -> Result<f64, JsValue> {
    let coef = hand::from_js::<Vec<mrly_num::zeta::Complex>>(&coef)?;
    let value = mrly_num::zeta::novelty_wave(gammas, &coef, log_y);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_psi_curve(gammas: &[f64], samples: &[f64]) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::zeta::psi_curve(gammas, samples);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_psi_formula(x: f64, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrly_num::zeta::psi_formula(x, gammas);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_psi_gap(x: usize, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrly_num::zeta::psi_gap(x, gammas);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_psi_stair(x: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly_num::zeta::psi_stair(x);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_raise(base: f64, exponent: JsValue) -> Result<JsValue, JsValue> {
    let exponent = hand::from_js::<mrly_num::zeta::Complex>(&exponent)?;
    let value = mrly_num::zeta::raise(base, exponent);
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_zeta_sharp_novelty(prefix: JsValue, y: f64) -> Result<f64, JsValue> {
    let prefix = hand::list_from_js(&prefix, hand::u64_from_js)?;
    let value = mrly_num::zeta::sharp_novelty(&prefix, y);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_smoothed_novelty(phi: JsValue, y: f64, main: f64) -> Result<f64, JsValue> {
    let phi = hand::list_from_js(&phi, hand::u64_from_js)?;
    let value = mrly_num::zeta::smoothed_novelty(&phi, y, main);
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_CIRCLE_CAP() -> Result<usize, JsValue> {
    let value = mrly_num::apollonian::CIRCLE_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_CURVATURE_CAP() -> Result<i64, JsValue> {
    let value = mrly_num::apollonian::CURVATURE_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_ORDER_CAP() -> Result<usize, JsValue> {
    let value = mrly_num::apollonian::ORDER_CAP;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_apollonian_ROOTS() -> Result<JsValue, JsValue> {
    let value = mrly_num::apollonian::ROOTS;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_apollonian_STRIDE() -> Result<usize, JsValue> {
    let value = mrly_num::apollonian::STRIDE;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_kummer_MOST() -> Result<u64, JsValue> {
    let value = mrly_num::kummer::MOST;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_kummer_TABLE() -> Result<u64, JsValue> {
    let value = mrly_num::kummer::TABLE;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_morse_LIFTS() -> Result<JsValue, JsValue> {
    let value = mrly_num::morse::LIFTS;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_readings_READINGS() -> Result<JsValue, JsValue> {
    let value = mrly_num::readings::READINGS;
    hand::to_js(&value)
}

#[wasm_bindgen]
pub fn num_series_APERY() -> Result<f64, JsValue> {
    let value = mrly_num::series::APERY;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_BASEL() -> Result<f64, JsValue> {
    let value = mrly_num::series::BASEL;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_CATALAN() -> Result<f64, JsValue> {
    let value = mrly_num::series::CATALAN;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_EULER() -> Result<f64, JsValue> {
    let value = mrly_num::series::EULER;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_series_VISIBLE() -> Result<f64, JsValue> {
    let value = mrly_num::series::VISIBLE;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_sieve_PLANE_LIMIT() -> Result<f64, JsValue> {
    let value = mrly_num::sieve::PLANE_LIMIT;
    Ok(value)
}

#[wasm_bindgen]
pub fn num_zeta_JOIN() -> Result<f64, JsValue> {
    let value = mrly_num::zeta::JOIN;
    Ok(value)
}

#[wasm_bindgen]
pub struct num_apollonian_Circle {
    inner: mrly_num::apollonian::Circle,
}

#[wasm_bindgen]
impl num_apollonian_Circle {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_apollonian_Circle, JsValue> {
        Ok(num_apollonian_Circle {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
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
    pub fn centre(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.centre();
        hand::to_js(&value)
    }
    pub fn is_line(&self) -> Result<bool, JsValue> {
        let value = self.inner.is_line();
        Ok(value)
    }
    pub fn radius(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.radius();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct num_gauss_Window {
    inner: mrly_num::gauss::Window,
}

#[wasm_bindgen]
impl num_gauss_Window {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_gauss_Window, JsValue> {
        Ok(num_gauss_Window {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn census(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.census();
        hand::to_js(&value)
    }
    pub fn class(&self, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = self.inner.class(a, b);
        hand::to_js(&value)
    }
    pub fn holds(&self, a: JsValue, b: JsValue) -> Result<bool, JsValue> {
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = self.inner.holds(a, b);
        Ok(value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(ring: JsValue, radius: JsValue) -> Result<num_gauss_Window, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = mrly_num::gauss::Window::new(ring, radius);
        Ok(num_gauss_Window { inner: value })
    }
    pub fn points(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.points();
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    pub fn radius(&self) -> Result<u64, JsValue> {
        let value = self.inner.radius();
        Ok(value)
    }
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct num_prime_Sieve {
    inner: mrly_num::prime::Sieve,
}

#[wasm_bindgen]
impl num_prime_Sieve {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_prime_Sieve, JsValue> {
        Ok(num_prime_Sieve {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn count(&self) -> Result<usize, JsValue> {
        let value = self.inner.count();
        Ok(value)
    }
    pub fn done(&self) -> Result<bool, JsValue> {
        let value = self.inner.done();
        Ok(value)
    }
    pub fn finish(&mut self) -> Result<(), JsValue> {
        self.inner.finish();
        Ok(())
    }
    #[wasm_bindgen(constructor)]
    pub fn new(limit: usize) -> Result<num_prime_Sieve, JsValue> {
        let value = mrly_num::prime::Sieve::new(limit);
        Ok(num_prime_Sieve { inner: value })
    }
    pub fn rank(&self) -> Result<usize, JsValue> {
        let value = self.inner.rank();
        Ok(value)
    }
    pub fn step(&mut self) -> Result<usize, JsValue> {
        let value = self.inner.step();
        Ok(value)
    }
    pub fn struck(&self) -> Result<usize, JsValue> {
        let value = self.inner.struck();
        Ok(value)
    }
    pub fn types(&self) -> Result<Vec<u8>, JsValue> {
        let value = self.inner.types();
        Ok(value.to_vec())
    }
}

#[wasm_bindgen]
pub struct num_zeta_Line {
    inner: mrly_num::zeta::Line,
}

#[wasm_bindgen]
impl num_zeta_Line {
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<num_zeta_Line, JsValue> {
        Ok(num_zeta_Line {
            inner: hand::from_js(&data)?,
        })
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    pub fn count(&self, t: f64) -> Result<usize, JsValue> {
        let value = self.inner.count(t);
        Ok(value)
    }
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<num_zeta_Line, JsValue> {
        let value = mrly_num::zeta::Line::default();
        Ok(num_zeta_Line { inner: value })
    }
    pub fn exact(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.exact(t);
        Ok(value)
    }
    pub fn gram(&self, n: JsValue) -> Result<f64, JsValue> {
        let n = hand::i64_from_js(&n)?;
        let value = self.inner.gram(n);
        Ok(value)
    }
    pub fn maclaurin(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.maclaurin(t);
        hand::to_js(&value)
    }
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<num_zeta_Line, JsValue> {
        let value = mrly_num::zeta::Line::new();
        Ok(num_zeta_Line { inner: value })
    }
    pub fn novelty_coefficients(&self, gammas: &[f64]) -> Result<JsValue, JsValue> {
        let value = self.inner.novelty_coefficients(gammas);
        hand::to_js(&value)
    }
    pub fn pair(&self, s: JsValue) -> Result<JsValue, JsValue> {
        let s = hand::from_js::<mrly_num::zeta::Complex>(&s)?;
        let value = self.inner.pair(s);
        hand::to_js(&value)
    }
    pub fn point(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.point(t);
        hand::to_js(&value)
    }
    pub fn seam(&self, t0: f64, t1: f64, steps: usize) -> Result<f64, JsValue> {
        let value = self.inner.seam(t0, t1, steps);
        Ok(value)
    }
    pub fn siegel(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.siegel(t);
        Ok(value)
    }
    pub fn theta(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.theta(t);
        Ok(value)
    }
    pub fn z(&self, t: f64) -> Result<f64, JsValue> {
        let value = self.inner.z(t);
        Ok(value)
    }
    pub fn zeros(&self, count: usize) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.zeros(count);
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct num_gauss_Class {}

#[wasm_bindgen]
impl num_gauss_Class {
    pub fn prime(class_: JsValue) -> Result<bool, JsValue> {
        let class_ = hand::from_js::<mrly_num::gauss::Class>(&class_)?;
        let value = class_.prime();
        Ok(value)
    }
    pub fn word(class_: JsValue) -> Result<String, JsValue> {
        let class_ = hand::from_js::<mrly_num::gauss::Class>(&class_)?;
        let value = class_.word();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct num_gauss_Ring {}

#[wasm_bindgen]
impl num_gauss_Ring {
    pub fn associates(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
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
    pub fn canon(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.canon(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn conjugate(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.conjugate(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn count(ring: JsValue, radius: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.count(radius);
        Ok(value)
    }
    pub fn div_rem(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
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
            hand::tuple_to_js(&[JsValue::from(value.0.0), JsValue::from(value.0.1)]),
            hand::tuple_to_js(&[JsValue::from(value.1.0), JsValue::from(value.1.1)]),
        ]))
    }
    pub fn fate(ring: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let n = hand::u64_from_js(&n)?;
        let value = ring.fate(n);
        hand::to_js(&value)
    }
    pub fn gaussian_gcd(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
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
    pub fn inert(ring: JsValue, p: JsValue) -> Result<bool, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let p = hand::u64_from_js(&p)?;
        let value = ring.inert(p);
        Ok(value)
    }
    pub fn mul(ring: JsValue, arg1: JsValue, arg2: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
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
    pub fn named(name: &str) -> Result<JsValue, JsValue> {
        let value = mrly_num::gauss::Ring::named(name);
        hand::to_js(&value)
    }
    pub fn nearest(ring: JsValue, x: f64, y: f64) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let value = ring.nearest(x, y);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn norm(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.norm(a, b);
        Ok(value)
    }
    pub fn place(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.place(a, b);
        hand::to_js(&value)
    }
    pub fn ramified(ring: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let value = ring.ramified();
        Ok(value)
    }
    pub fn reach(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.reach(a, b);
        Ok(value)
    }
    pub fn symmetry(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let value = ring.symmetry();
        Ok(value)
    }
    pub fn top(ring: JsValue, radius: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.top(radius);
        Ok(value)
    }
    pub fn turn(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.turn(a, b);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    pub fn units(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let value = ring.units();
        Ok(value)
    }
    pub fn whole(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly_num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.whole(a, b);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
    }
}

#[wasm_bindgen]
pub struct num_morse_Lift {}

#[wasm_bindgen]
impl num_morse_Lift {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_num::morse::Lift::all();
        hand::to_js(&value)
    }
    pub fn at(lift: JsValue, i: JsValue, j: JsValue) -> Result<u8, JsValue> {
        let lift = hand::from_js::<mrly_num::morse::Lift>(&lift)?;
        let i = hand::u64_from_js(&i)?;
        let j = hand::u64_from_js(&j)?;
        let value = lift.at(i, j);
        Ok(value)
    }
    pub fn formula(lift: JsValue) -> Result<String, JsValue> {
        let lift = hand::from_js::<mrly_num::morse::Lift>(&lift)?;
        let value = lift.formula();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct num_readings_Design {}

#[wasm_bindgen]
impl num_readings_Design {
    pub fn digits(base: u8, mask: u8) -> Result<JsValue, JsValue> {
        let value = mrly_num::readings::Design::digits(base, mask);
        hand::to_js(&value)
    }
    pub fn rudin() -> Result<JsValue, JsValue> {
        let value = mrly_num::readings::Design::rudin();
        hand::to_js(&value)
    }
    pub fn ternary() -> Result<JsValue, JsValue> {
        let value = mrly_num::readings::Design::ternary();
        hand::to_js(&value)
    }
    pub fn thue() -> Result<JsValue, JsValue> {
        let value = mrly_num::readings::Design::thue();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct num_readings_Reading {}

#[wasm_bindgen]
impl num_readings_Reading {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_num::readings::Reading::all();
        hand::to_js(&value)
    }
    pub fn formula(reading: JsValue) -> Result<String, JsValue> {
        let reading = hand::from_js::<mrly_num::readings::Reading>(&reading)?;
        let value = reading.formula();
        Ok(value)
    }
}

#[wasm_bindgen]
pub struct num_spiral_Growth {}

#[wasm_bindgen]
impl num_spiral_Growth {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_num::spiral::Growth::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct num_spiral_Lattice {}

#[wasm_bindgen]
impl num_spiral_Lattice {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_num::spiral::Lattice::all();
        hand::to_js(&value)
    }
    pub fn count(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let value = lattice.count(side);
        Ok(value)
    }
    pub fn n(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.n(x, y);
        Ok(value)
    }
    pub fn radius(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let value = lattice.radius(side);
        Ok(value)
    }
    pub fn ring(lattice: JsValue, n: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.ring(n);
        Ok(value)
    }
    pub fn ring_of(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.ring_of(x, y);
        Ok(value)
    }
    pub fn xy(lattice: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let lattice = hand::from_js::<mrly_num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.xy(n);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
}

#[wasm_bindgen]
pub struct num_spiral_Mark {}

#[wasm_bindgen]
impl num_spiral_Mark {
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrly_num::spiral::Mark::all();
        hand::to_js(&value)
    }
}

#[wasm_bindgen]
pub struct num_zeta_Complex {}

#[wasm_bindgen]
impl num_zeta_Complex {
    pub fn abs(z: JsValue) -> Result<f64, JsValue> {
        let z = hand::from_js::<mrly_num::zeta::Complex>(&z)?;
        let value = mrly_num::zeta::Complex::abs(z);
        Ok(value)
    }
    pub fn arg(z: JsValue) -> Result<f64, JsValue> {
        let z = hand::from_js::<mrly_num::zeta::Complex>(&z)?;
        let value = mrly_num::zeta::Complex::arg(z);
        Ok(value)
    }
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly_num::zeta::Complex::default();
        hand::to_js(&value)
    }
    pub fn exp(z: JsValue) -> Result<JsValue, JsValue> {
        let z = hand::from_js::<mrly_num::zeta::Complex>(&z)?;
        let value = mrly_num::zeta::Complex::exp(z);
        hand::to_js(&value)
    }
    pub fn ln(z: JsValue) -> Result<JsValue, JsValue> {
        let z = hand::from_js::<mrly_num::zeta::Complex>(&z)?;
        let value = mrly_num::zeta::Complex::ln(z);
        hand::to_js(&value)
    }
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(re: f64, im: f64) -> Result<JsValue, JsValue> {
        let value = mrly_num::zeta::Complex::new(re, im);
        hand::to_js(&value)
    }
    pub fn turn(angle: f64) -> Result<JsValue, JsValue> {
        let value = mrly_num::zeta::Complex::turn(angle);
        hand::to_js(&value)
    }
}
