#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// Grows the named packing and lays every circle of positive curvature as `x, y, r, k` in one flat run: the root's own circles first in root order, then the grown ones in curvature order.
#[wasm_bindgen]
pub fn num_apollonian_flat(name: &str, cap: JsValue) -> Result<Vec<f64>, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrly::num::apollonian::flat(name, cap).map_err(hand::throw)?;
    Ok(value)
}

/// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
#[wasm_bindgen]
pub fn num_apollonian_form(u: JsValue, v: JsValue) -> Result<JsValue, JsValue> {
    let u = hand::array_from_js::<_, 4>(&u, hand::i64_from_js)?;
    let v = hand::array_from_js::<_, 4>(&v, hand::i64_from_js)?;
    let value = mrly::num::apollonian::form(u, v);
    Ok(JsValue::from_str(&value.to_string()))
}

/// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
#[wasm_bindgen]
pub fn num_apollonian_frame(p: JsValue) -> Result<Vec<f64>, JsValue> {
    let p = hand::from_js::<mrly::num::apollonian::Packing>(&p)?;
    let value = mrly::num::apollonian::frame(&p);
    Ok(value.to_vec())
}

/// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
#[wasm_bindgen]
pub fn num_apollonian_grow(name: &str, cap: JsValue) -> Result<JsValue, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrly::num::apollonian::grow(name, cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
#[wasm_bindgen]
pub fn num_apollonian_is_ford(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrly::num::apollonian::is_ford(c.inner);
    Ok(value)
}

/// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
#[wasm_bindgen]
pub fn num_apollonian_on_line(c: &num_apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrly::num::apollonian::on_line(c.inner);
    Ok(value)
}

/// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
#[wasm_bindgen]
pub fn num_apollonian_reflect(q: JsValue, at: usize) -> Result<num_apollonian_Circle, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly::num::apollonian::reflect(&q, at);
    Ok(num_apollonian_Circle { inner: value })
}

/// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
#[wasm_bindgen]
pub fn num_apollonian_root(name: &str) -> Result<JsValue, JsValue> {
    let value = mrly::num::apollonian::root(name).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

/// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
#[wasm_bindgen]
pub fn num_apollonian_shadow(p: JsValue, order: usize) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrly::num::apollonian::Packing>(&p)?;
    let value = mrly::num::apollonian::shadow(&p, order).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
#[wasm_bindgen]
pub fn num_apollonian_sound(q: JsValue) -> Result<bool, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly::num::apollonian::sound(&q);
    Ok(value)
}

/// The quadruple with the circle at the seat replaced by its reflection.
#[wasm_bindgen]
pub fn num_apollonian_swap(q: JsValue, at: usize) -> Result<JsValue, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrly::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrly::num::apollonian::swap(&q, at);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(num_apollonian_Circle { inner: *x1 }))
    })
}

/// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
#[wasm_bindgen]
pub fn num_apollonian_touches(p: JsValue) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrly::num::apollonian::Packing>(&p)?;
    let value = mrly::num::apollonian::touches(&p);
    hand::to_js(&value)
}

/// Adds two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn num_blend_add(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly::num::blend::add(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Convolves two sequences, keeping the exact prefix their shared length affords.
#[wasm_bindgen]
pub fn num_blend_cauchy(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly::num::blend::cauchy(&a, &b).map_err(hand::throw)?;
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
    let value = mrly::num::blend::characteristic(&coefficients);
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
    let value = mrly::num::blend::decimate(&a, step, offset).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the first differences of a sequence, one term shorter.
#[wasm_bindgen]
pub fn num_blend_delta(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly::num::blend::delta(&a);
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
    let value = mrly::num::blend::growth(&coefficients);
    Ok(value)
}

/// Multiplies two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn num_blend_hadamard(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly::num::blend::hadamard(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
#[wasm_bindgen]
pub fn num_blend_recurrence(terms: JsValue) -> Result<JsValue, JsValue> {
    let terms = hand::list_from_js(&terms, hand::i128_from_js)?;
    let value = mrly::num::blend::recurrence(&terms);
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
    let value = mrly::num::blend::scale(&a, factor);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Drops the first terms of a sequence.
#[wasm_bindgen]
pub fn num_blend_shift(a: JsValue, count: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly::num::blend::shift(&a, count);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the partial sums of a sequence.
#[wasm_bindgen]
pub fn num_blend_sigma(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrly::num::blend::sigma(&a).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Subtracts the second sequence from the first over their shared length.
#[wasm_bindgen]
pub fn num_blend_sub(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrly::num::blend::sub(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Reports whether the packed function outputs one on exactly half of its inputs.
#[wasm_bindgen]
pub fn num_boolean_is_balanced(code: JsValue, n: usize) -> Result<bool, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly::num::boolean::is_balanced(code, n);
    Ok(value)
}

/// Returns how far the packed function sits from every affine function, zero when it is one.
#[wasm_bindgen]
pub fn num_boolean_nonlinearity(code: JsValue, n: usize) -> Result<i64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly::num::boolean::nonlinearity(code, n);
    Ok(value)
}

/// Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.
#[wasm_bindgen]
pub fn num_boolean_sac(code: JsValue, n: usize) -> Result<f64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly::num::boolean::sac(code, n);
    Ok(value)
}

/// Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.
#[wasm_bindgen]
pub fn num_boolean_walsh_spectrum(code: JsValue, n: usize) -> Result<Vec<i64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly::num::boolean::walsh_spectrum(code, n);
    Ok(value)
}

/// Lists every distinct plate figure with `0 <= n <= m <= cap` but `(0, 0)`, both signs when `n < m` and the sign 1 when `n == m`, by pitch, then `n`, then the sign 1 before -1.
#[wasm_bindgen]
pub fn num_chladni_ladder(cap: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::chladni::ladder(cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Traces the zero set of a plate raster by marching squares on the lattice of cell centres, its holes and its rim ghosted to the mean of their plate neighbours so the lines run to every free edge: segments `x0, y0, x1, y1` in `[0, 1]` back to back, pieces inside a NaN cell dropped, a saddle joined by the mean of its four corners.
#[wasm_bindgen]
pub fn num_chladni_lines(field: &[f64], side: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::chladni::lines(field, side).map_err(hand::throw)?;
    Ok(value)
}

/// Samples the Chladni figure `cos(n pi x) cos(m pi y) + sign cos(m pi x) cos(n pi y)` of a free square plate at the cell centres `(i + 1/2) / size` of a row-major `size` by `size` raster, `x` along a row, scaled so the largest magnitude is 1, or all zero when no centre rises past `1e-12`; when `n == m` it is the single product for either sign.
#[wasm_bindgen]
pub fn num_chladni_plate(n: usize, m: usize, sign: i32, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::chladni::plate(n, m, sign, size).map_err(hand::throw)?;
    Ok(value)
}

/// Shakes sand on a plate raster: grains are `x, y` pairs in `[0, 1)` over the row-major `side` by `side` field, a NaN pair an empty slot that stays NaN, a NaN cell a hole. Each step moves a grain a random length `kick |f| + floor` in a random direction plus a drift `-0.3 f grad f / max|grad f|^2`, with `kick = 0.6 / max|grad f|` read off the raster by finite differences and a fixed kick on a flat field, the field read bilinearly with its holes and its rim ghosted to the mean of their plate neighbours. The rim reflects; a grain on a hole falls through and becomes NaN. Step `from + i` of grain `j` draws from a counter keyed by `(seed, from + i, j)`, so six steps are three and three.
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
        mrly::num::chladni::shake(field, side, grains, from, steps, seed).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
#[wasm_bindgen]
pub fn num_factor_aliquot(number: usize) -> Result<usize, JsValue> {
    let value = mrly::num::factor::aliquot(number);
    Ok(value)
}

/// Returns whether two numbers share no divisor above one.
#[wasm_bindgen]
pub fn num_factor_coprime(a: usize, b: usize) -> Result<bool, JsValue> {
    let value = mrly::num::factor::coprime(a, b);
    Ok(value)
}

/// Builds every divisor of a wide number from its factorization, ascending, empty for zero.
#[wasm_bindgen]
pub fn num_factor_divisors(number: JsValue) -> Result<Vec<u64>, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly::num::factor::divisors(number);
    Ok(value)
}

/// Returns the factorial of the number, the product of one through it, erring past thirty-four.
#[wasm_bindgen]
pub fn num_factor_factorial(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::factor::factorial(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_factor_factorize(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::factor::factorize(number);
    hand::to_js(&value)
}

/// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_factor_factorize_wide(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly::num::factor::factorize_wide(number);
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
    let value = mrly::num::factor::gcd(a, b);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the least common multiple of two numbers, zero when either side is zero.
#[wasm_bindgen]
pub fn num_factor_lcm(a: usize, b: usize) -> Result<usize, JsValue> {
    let value = mrly::num::factor::lcm(a, b);
    Ok(value)
}

/// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
#[wasm_bindgen]
pub fn num_factor_mobius(number: usize) -> Result<i8, JsValue> {
    let value = mrly::num::factor::mobius(number);
    Ok(value)
}

/// Sieves the Mobius values of zero through the limit in one pass.
#[wasm_bindgen]
pub fn num_factor_mobius_sieve(limit: usize) -> Result<Vec<i8>, JsValue> {
    let value = mrly::num::factor::mobius_sieve(limit);
    Ok(value)
}

/// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
#[wasm_bindgen]
pub fn num_factor_radical(number: usize) -> Result<usize, JsValue> {
    let value = mrly::num::factor::radical(number);
    Ok(value)
}

/// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
#[wasm_bindgen]
pub fn num_factor_reduce(numerator: JsValue, denominator: JsValue) -> Result<JsValue, JsValue> {
    let numerator = hand::u128_from_js(&numerator)?;
    let denominator = hand::u128_from_js(&denominator)?;
    let value = mrly::num::factor::reduce(numerator, denominator);
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

/// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
#[wasm_bindgen]
pub fn num_factor_sigma(number: usize, power: u32) -> Result<JsValue, JsValue> {
    let value = mrly::num::factor::sigma(number, power);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether no prime squares into the number, true for one and false for zero.
#[wasm_bindgen]
pub fn num_factor_squarefree(number: usize) -> Result<bool, JsValue> {
    let value = mrly::num::factor::squarefree(number);
    Ok(value)
}

/// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
#[wasm_bindgen]
pub fn num_factor_totient(number: usize) -> Result<usize, JsValue> {
    let value = mrly::num::factor::totient(number);
    Ok(value)
}

/// Sieves the Euler totients of zero through n in one pass, the run beside the single value.
#[wasm_bindgen]
pub fn num_factor_totients(n: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly::num::factor::totients(n);
    Ok(value)
}

/// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
#[wasm_bindgen]
pub fn num_factor_twisted(number: usize, rhythm: &[i8]) -> Result<i64, JsValue> {
    let value = mrly::num::factor::twisted(number, rhythm);
    Ok(value)
}

/// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
#[wasm_bindgen]
pub fn num_fft_convolve(field: &[f64], kernel: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::fft::convolve(field, kernel, size).map_err(hand::throw)?;
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
        mrly::num::fft::convolve_with(field, kernel_re, kernel_im, size).map_err(hand::throw)?;
    Ok(value)
}

/// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
#[wasm_bindgen]
pub fn num_fft_embed_kernel(mask: &[u8], side: usize, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::fft::embed_kernel(mask, side, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
#[wasm_bindgen]
pub fn num_fft_log_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::fft::log_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
#[wasm_bindgen]
pub fn num_fft_magnitude_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::fft::magnitude_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn num_fft_peak_ring(profile: &[f64]) -> Result<usize, JsValue> {
    let value = mrly::num::fft::peak_ring(profile);
    Ok(value)
}

/// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn num_fft_peak_wavelength(profile: &[f64], size: usize) -> Result<f64, JsValue> {
    let value = mrly::num::fft::peak_wavelength(profile, size);
    Ok(value)
}

/// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
#[wasm_bindgen]
pub fn num_fft_radial_profile(spectrum: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::fft::radial_profile(spectrum, size).map_err(hand::throw)?;
    Ok(value)
}

/// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
#[wasm_bindgen]
pub fn num_fft_transform(field: &[f64], size: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::fft::transform(field, size).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.
#[wasm_bindgen]
pub fn num_gauss_classes(ring: JsValue, bound: JsValue) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
    let bound = hand::u64_from_js(&bound)?;
    let value = mrly::num::gauss::classes(ring, bound);
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
    let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
    let value = mrly::num::gauss::peak(ring, limit);
    hand::to_js(&value)
}

/// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
#[wasm_bindgen]
pub fn num_gauss_shells(ring: JsValue, limit: usize) -> Result<Vec<u32>, JsValue> {
    let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
    let value = mrly::num::gauss::shells(ring, limit);
    Ok(value)
}

/// Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom.
#[wasm_bindgen]
pub fn num_kummer_carry(prime: JsValue, top: JsValue, bottom: JsValue) -> Result<JsValue, JsValue> {
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly::num::kummer::carry(prime, top, bottom).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again.
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
    let value = mrly::num::kummer::columns(k, prime, top, bottom).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close.
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
    let value = mrly::num::kummer::count(prime, top, bottom, level).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`.
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
    let value = mrly::num::kummer::member(k, prime, top, bottom).map_err(hand::throw)?;
    Ok(value)
}

/// Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`.
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
    let value = mrly::num::kummer::valuation(k, prime, top, bottom).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`.
#[wasm_bindgen]
pub fn num_kummer_witness(
    prime: JsValue,
    top: JsValue,
    bottom: JsValue,
) -> Result<JsValue, JsValue> {
    let prime = hand::u64_from_js(&prime)?;
    let top = hand::u64_from_js(&top)?;
    let bottom = hand::u64_from_js(&bottom)?;
    let value = mrly::num::kummer::witness(prime, top, bottom).map_err(hand::throw)?;
    hand::option_to_js(value.as_ref(), |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
#[wasm_bindgen]
pub fn num_lattice_coprime_pairs(n: usize) -> Result<u64, JsValue> {
    let value = mrly::num::lattice::coprime_pairs(n);
    Ok(value)
}

/// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.
#[wasm_bindgen]
pub fn num_lattice_farey(order: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::lattice::farey(order);
    hand::to_js(&value)
}

/// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
#[wasm_bindgen]
pub fn num_lattice_grid(n: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::lattice::grid(n);
    hand::to_js(&value)
}

/// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
#[wasm_bindgen]
pub fn num_lattice_new_nodes(n: usize) -> Result<u64, JsValue> {
    let value = mrly::num::lattice::new_nodes(n);
    Ok(value)
}

/// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
#[wasm_bindgen]
pub fn num_lattice_pi_estimate(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::lattice::pi_estimate(n);
    Ok(value)
}

/// Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.
#[wasm_bindgen]
pub fn num_lattice_recovered(n: usize, dimension: u32) -> Result<f64, JsValue> {
    let value = mrly::num::lattice::recovered(n, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The density the visible count of a window in the dimension walks to, one over zeta of the dimension.
#[wasm_bindgen]
pub fn num_lattice_visible_density(dimension: u32) -> Result<f64, JsValue> {
    let value = mrly::num::lattice::visible_density(dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.
#[wasm_bindgen]
pub fn num_lattice_zeta_factor(dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrly::num::lattice::zeta_factor(dimension);
    hand::to_js(&value)
}

/// The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.
#[wasm_bindgen]
pub fn num_lattice_zeta_whole(s: u32) -> Result<f64, JsValue> {
    let value = mrly::num::lattice::zeta_whole(s).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the run-boundary word, one wherever a letter differs from the next.
#[wasm_bindgen]
pub fn num_morse_boundary(word: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::boundary(word);
    Ok(value)
}

/// Exclusive-ors two grids of the same length, site by site.
#[wasm_bindgen]
pub fn num_morse_difference(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::difference(a, b);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the digit rule.
#[wasm_bindgen]
pub fn num_morse_digits(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::digits(length);
    Ok(value)
}

/// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
#[wasm_bindgen]
pub fn num_morse_doubling(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::doubling(length);
    Ok(value)
}

/// Counts the sites where two grids of the same length differ.
#[wasm_bindgen]
pub fn num_morse_faults(a: &[u8], b: &[u8]) -> Result<usize, JsValue> {
    let value = mrly::num::morse::faults(a, b);
    Ok(value)
}

/// Tests a grid against the Kronecker power of its own corner tile.
#[wasm_bindgen]
pub fn num_morse_fold(grid: &[u8], side: usize, number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::morse::fold(grid, side, number).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
#[wasm_bindgen]
pub fn num_morse_letter(place: JsValue) -> Result<u8, JsValue> {
    let place = hand::u64_from_js(&place)?;
    let value = mrly::num::morse::letter(place);
    Ok(value)
}

/// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
#[wasm_bindgen]
pub fn num_morse_lift(kind: JsValue, side: usize) -> Result<Vec<u8>, JsValue> {
    let kind = hand::from_js::<mrly::num::morse::Lift>(&kind)?;
    let value = mrly::num::morse::lift(kind, side);
    Ok(value)
}

/// Keeps the sites lit in both grids of the same length, site by site.
#[wasm_bindgen]
pub fn num_morse_mask(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::mask(a, b);
    Ok(value)
}

/// Folds a tile of the side into its Kronecker power at the level, one bit per site.
#[wasm_bindgen]
pub fn num_morse_power(tile: &[u8], number: usize, level: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::power(tile, number, level).map_err(hand::throw)?;
    Ok(value)
}

/// Repeats a tile until it fills a grid of the side.
#[wasm_bindgen]
pub fn num_morse_repeat(tile: &[u8], number: usize, side: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::repeat(tile, number, side);
    Ok(value)
}

/// Returns the lengths of the maximal blocks of one repeated letter, in order.
#[wasm_bindgen]
pub fn num_morse_runs(word: &[u8]) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::morse::runs(word);
    Ok(value)
}

/// Reads a design plus-minus: a filled cell is plus, zero, and a void is minus, one.
#[wasm_bindgen]
pub fn num_morse_signs(types: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::signs(types);
    Ok(value)
}

/// Returns the substitution stage after the rounds, a word of length two to the rounds.
#[wasm_bindgen]
pub fn num_morse_stage(rounds: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::stage(rounds);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
#[wasm_bindgen]
pub fn num_morse_substitution(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::substitution(length);
    Ok(value)
}

/// Blows a grid up by the scale, every site becoming a scale-by-scale block.
#[wasm_bindgen]
pub fn num_morse_upsample(grid: &[u8], side: usize, scale: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrly::num::morse::upsample(grid, side, scale);
    Ok(value)
}

/// Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.
#[wasm_bindgen]
pub fn num_prime_chart(top: usize, bins: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::chart(top, bins);
    hand::to_js(&value)
}

/// Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.
#[wasm_bindgen]
pub fn num_prime_flags(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::flags(limit);
    hand::to_js(&value)
}

/// Returns the count of unordered pairs of primes summing to the number, zero below four.
#[wasm_bindgen]
pub fn num_prime_goldbach(number: usize) -> Result<usize, JsValue> {
    let value = mrly::num::prime::goldbach(number);
    Ok(value)
}

/// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
#[wasm_bindgen]
pub fn num_prime_goldbach_record(top: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::prime::goldbach_record(top);
    Ok(value)
}

/// Returns whether the number is prime, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn num_prime_is_prime(number: usize) -> Result<bool, JsValue> {
    let value = mrly::num::prime::is_prime(number);
    Ok(value)
}

/// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
#[wasm_bindgen]
pub fn num_prime_pile(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrly::num::prime::pile(number);
    hand::to_js(&value)
}

/// Returns the count of primes at or below n.
#[wasm_bindgen]
pub fn num_prime_prime_count(n: usize) -> Result<usize, JsValue> {
    let value = mrly::num::prime::prime_count(n);
    Ok(value)
}

/// Returns the smallest prime at or above the number.
#[wasm_bindgen]
pub fn num_prime_prime_from(number: usize) -> Result<usize, JsValue> {
    let value = mrly::num::prime::prime_from(number);
    Ok(value)
}

/// Returns the primes up to the limit, the finished sieve read as a list.
#[wasm_bindgen]
pub fn num_prime_primes(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::prime::primes(limit);
    Ok(value)
}

/// Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.
#[wasm_bindgen]
pub fn num_prime_rectangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::rectangles(number);
    hand::to_js(&value)
}

/// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
#[wasm_bindgen]
pub fn num_prime_splits(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::splits(number);
    hand::to_js(&value)
}

/// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
#[wasm_bindgen]
pub fn num_prime_squares(number: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::squares(number);
    hand::to_js(&value)
}

/// Returns one prime object for every prime up to and including the limit.
#[wasm_bindgen]
pub fn num_prime_study(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::prime::study(limit);
    hand::to_js(&value)
}

/// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
#[wasm_bindgen]
pub fn num_series_basel(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::basel(n);
    Ok(value)
}

/// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
#[wasm_bindgen]
pub fn num_series_bernoulli(count: usize) -> Result<JsValue, JsValue> {
    let value = mrly::num::series::bernoulli(count).map_err(hand::throw)?;
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
    let value = mrly::num::series::beta(s, terms);
    Ok(value)
}

/// Returns the powers of two up to the limit.
#[wasm_bindgen]
pub fn num_series_binary(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::series::binary(limit);
    Ok(value)
}

/// Returns the distinct Catalan numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_catalan(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::series::catalan(limit);
    Ok(value)
}

/// Returns the mod-three rhythm of the number: zero, one, minus one.
#[wasm_bindgen]
pub fn num_series_chi3(number: usize) -> Result<i8, JsValue> {
    let value = mrly::num::series::chi3(number);
    Ok(value)
}

/// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
#[wasm_bindgen]
pub fn num_series_chi4(number: usize) -> Result<i8, JsValue> {
    let value = mrly::num::series::chi4(number);
    Ok(value)
}

/// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
#[wasm_bindgen]
pub fn num_series_chi8(number: usize) -> Result<i8, JsValue> {
    let value = mrly::num::series::chi8(number);
    Ok(value)
}

/// Returns the L-series partial sum with a periodic rhythm painted on the terms.
#[wasm_bindgen]
pub fn num_series_dirichlet(s: f64, rhythm: &[i8], terms: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::dirichlet(s, rhythm, terms);
    Ok(value)
}

/// Returns one plus one over n raised to the n, walking to the natural base.
#[wasm_bindgen]
pub fn num_series_e_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::e_partial(n);
    Ok(value)
}

/// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
#[wasm_bindgen]
pub fn num_series_euler_gamma_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::euler_gamma_partial(n);
    Ok(value)
}

/// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
#[wasm_bindgen]
pub fn num_series_euler_product(s: f64, limit: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::euler_product(s, limit);
    Ok(value)
}

/// Returns the even numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_evens(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::series::evens(limit);
    Ok(value)
}

/// Returns the distinct Fibonacci numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_fibonacci(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::series::fibonacci(limit);
    Ok(value)
}

/// Returns the partial harmonic sum, the reciprocals of one through the term count.
#[wasm_bindgen]
pub fn num_series_harmonic(terms: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::harmonic(terms);
    Ok(value)
}

/// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
#[wasm_bindgen]
pub fn num_series_lambda(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::lambda(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
#[wasm_bindgen]
pub fn num_series_leibniz(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::leibniz(n);
    Ok(value)
}

/// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
#[wasm_bindgen]
pub fn num_series_li(x: f64) -> Result<f64, JsValue> {
    let value = mrly::num::series::li(x);
    Ok(value)
}

/// Returns the Mertens function at n, the Mobius values of one through n summed.
#[wasm_bindgen]
pub fn num_series_mertens(n: usize) -> Result<i64, JsValue> {
    let value = mrly::num::series::mertens(n);
    Ok(value)
}

/// Returns the odd numbers up to the limit.
#[wasm_bindgen]
pub fn num_series_odds(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrly::num::series::odds(limit);
    Ok(value)
}

/// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
#[wasm_bindgen]
pub fn num_series_visible(limit: usize, dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrly::num::series::visible(limit, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
#[wasm_bindgen]
pub fn num_series_wallis_half_pi(n: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::wallis_half_pi(n);
    Ok(value)
}

/// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
#[wasm_bindgen]
pub fn num_series_wallis_quarter_pi(factors: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::wallis_quarter_pi(factors);
    Ok(value)
}

/// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
#[wasm_bindgen]
pub fn num_series_zeta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrly::num::series::zeta(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
#[wasm_bindgen]
pub fn num_sieve_cells(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::cells(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
#[wasm_bindgen]
pub fn num_sieve_exponent(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::exponent(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
#[wasm_bindgen]
pub fn num_sieve_flat_word(side: JsValue, levels: usize) -> Result<Vec<u64>, JsValue> {
    let side = hand::u64_from_js(&side)?;
    let value = mrly::num::sieve::flat_word(side, levels);
    Ok(value)
}

/// Returns the punctures the word makes, one per surviving cell at every level.
#[wasm_bindgen]
pub fn num_sieve_holes(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::holes(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
#[wasm_bindgen]
pub fn num_sieve_limit(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::limit(&word, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
#[wasm_bindgen]
pub fn num_sieve_odd_word(levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrly::num::sieve::odd_word(levels);
    Ok(value)
}

/// Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.
#[wasm_bindgen]
pub fn num_sieve_parity_product(dimension: u32, odd: bool) -> Result<f64, JsValue> {
    let value = mrly::num::sieve::parity_product(dimension, odd).map_err(hand::throw)?;
    Ok(value)
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
#[wasm_bindgen]
pub fn num_sieve_punctures(word: JsValue, dimension: u32) -> Result<Vec<u64>, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::punctures(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
#[wasm_bindgen]
pub fn num_sieve_raster(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::raster(&word).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::typed(&(value.1)[..]),
    ]))
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
#[wasm_bindgen]
pub fn num_sieve_ratio(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::ratio(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`.
#[wasm_bindgen]
pub fn num_sieve_row_fill(profile: JsValue, side: JsValue) -> Result<JsValue, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let side = hand::u64_from_js(&side)?;
    let value = mrly::num::sieve::row_fill(&profile, side).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply.
#[wasm_bindgen]
pub fn num_sieve_row_law(profile: JsValue) -> Result<JsValue, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let value = mrly::num::sieve::row_law(&profile).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`.
#[wasm_bindgen]
pub fn num_sieve_row_profile(code: JsValue, dimension: usize) -> Result<Vec<u64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrly::num::sieve::row_profile(code, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`.
#[wasm_bindgen]
pub fn num_sieve_row_settle(
    profile: JsValue,
    stops: &[usize],
    even: bool,
) -> Result<Vec<f64>, JsValue> {
    let profile = hand::list_from_js(&profile, hand::u64_from_js)?;
    let value = mrly::num::sieve::row_settle(&profile, stops, even).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the side of the word, the product of its letters' sides.
#[wasm_bindgen]
pub fn num_sieve_side(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrly::num::sieve::side(&word).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
#[wasm_bindgen]
pub fn num_sieve_solid_limit() -> Result<f64, JsValue> {
    let value = mrly::num::sieve::solid_limit();
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
    let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
    let a = hand::i64_from_js(&a)?;
    let b = hand::i64_from_js(&b)?;
    let c = hand::i64_from_js(&c)?;
    let value = mrly::num::spiral::diagonal(lattice, side, a, b, c);
    hand::to_js(&value)
}

/// Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.
#[wasm_bindgen]
pub fn num_spiral_level_of(n: JsValue, base: JsValue) -> Result<u32, JsValue> {
    let n = hand::u64_from_js(&n)?;
    let base = hand::u64_from_js(&base)?;
    let value = mrly::num::spiral::level_of(n, base);
    Ok(value)
}

/// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
#[wasm_bindgen]
pub fn num_spiral_marks(mark: JsValue, limit: usize) -> Result<Vec<i8>, JsValue> {
    let mark = hand::from_js::<mrly::num::spiral::Mark>(&mark)?;
    let value = mrly::num::spiral::marks(mark, limit);
    Ok(value)
}

/// Winds one to the top on the square spiral and lays a square tile on every cell, the snail.
#[wasm_bindgen]
pub fn num_spiral_snail(base: JsValue, top: JsValue, growth: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let top = hand::u64_from_js(&top)?;
    let growth = hand::from_js::<mrly::num::spiral::Growth>(&growth)?;
    let value = mrly::num::spiral::snail(base, top, growth);
    hand::to_js(&value)
}

/// The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.
#[wasm_bindgen]
pub fn num_zeta_bump(u: f64) -> Result<f64, JsValue> {
    let value = mrly::num::zeta::bump(u);
    Ok(value)
}

/// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
#[wasm_bindgen]
pub fn num_zeta_corrections(p: f64) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::zeta::corrections(p);
    Ok(value.to_vec())
}

/// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
#[wasm_bindgen]
pub fn num_zeta_kernel(p: f64) -> Result<f64, JsValue> {
    let value = mrly::num::zeta::kernel(p);
    Ok(value)
}

/// Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.
#[wasm_bindgen]
pub fn num_zeta_mellin(s: JsValue) -> Result<JsValue, JsValue> {
    let s = hand::from_js::<mrly::num::zeta::Complex>(&s)?;
    let value = mrly::num::zeta::mellin(s);
    hand::to_js(&value)
}

/// Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.
#[wasm_bindgen]
pub fn num_zeta_novelty_main() -> Result<f64, JsValue> {
    let value = mrly::num::zeta::novelty_main();
    Ok(value)
}

/// Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.
#[wasm_bindgen]
pub fn num_zeta_novelty_wave(gammas: &[f64], coef: JsValue, log_y: f64) -> Result<f64, JsValue> {
    let coef = hand::from_js::<Vec<mrly::num::zeta::Complex>>(&coef)?;
    let value = mrly::num::zeta::novelty_wave(gammas, &coef, log_y);
    Ok(value)
}

/// Returns the explicit formula over the zeros at the given ordinates at every sample x.
#[wasm_bindgen]
pub fn num_zeta_psi_curve(gammas: &[f64], samples: &[f64]) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::zeta::psi_curve(gammas, samples);
    Ok(value)
}

/// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
#[wasm_bindgen]
pub fn num_zeta_psi_formula(x: f64, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrly::num::zeta::psi_formula(x, gammas);
    Ok(value)
}

/// Returns how far the explicit formula over the zeros at the given ordinates lands from the staircase at the whole number x.
#[wasm_bindgen]
pub fn num_zeta_psi_gap(x: usize, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrly::num::zeta::psi_gap(x, gammas);
    Ok(value)
}

/// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
#[wasm_bindgen]
pub fn num_zeta_psi_stair(x: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrly::num::zeta::psi_stair(x);
    Ok(value)
}

/// Returns a positive real base raised to a complex exponent.
#[wasm_bindgen]
pub fn num_zeta_raise(base: f64, exponent: JsValue) -> Result<JsValue, JsValue> {
    let exponent = hand::from_js::<mrly::num::zeta::Complex>(&exponent)?;
    let value = mrly::num::zeta::raise(base, exponent);
    hand::to_js(&value)
}

/// Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.
#[wasm_bindgen]
pub fn num_zeta_sharp_novelty(prefix: JsValue, y: f64) -> Result<f64, JsValue> {
    let prefix = hand::list_from_js(&prefix, hand::u64_from_js)?;
    let value = mrly::num::zeta::sharp_novelty(&prefix, y);
    Ok(value)
}

/// Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.
#[wasm_bindgen]
pub fn num_zeta_smoothed_novelty(phi: JsValue, y: f64, main: f64) -> Result<f64, JsValue> {
    let phi = hand::list_from_js(&phi, hand::u64_from_js)?;
    let value = mrly::num::zeta::smoothed_novelty(&phi, y, main);
    Ok(value)
}

/// The most circles one growth makes before it gives up.
#[wasm_bindgen]
pub fn num_apollonian_CIRCLE_CAP() -> Result<usize, JsValue> {
    let value = mrly::num::apollonian::CIRCLE_CAP;
    Ok(value)
}

/// The largest curvature a packing is grown to.
#[wasm_bindgen]
pub fn num_apollonian_CURVATURE_CAP() -> Result<i64, JsValue> {
    let value = mrly::num::apollonian::CURVATURE_CAP;
    Ok(value)
}

/// The deepest the Farey stack is read against a packing.
#[wasm_bindgen]
pub fn num_apollonian_ORDER_CAP() -> Result<usize, JsValue> {
    let value = mrly::num::apollonian::ORDER_CAP;
    Ok(value)
}

/// The root quadruples on offer: the strip first, then the bounded packings named by their curvatures.
#[wasm_bindgen]
pub fn num_apollonian_ROOTS() -> Result<JsValue, JsValue> {
    let value = mrly::num::apollonian::ROOTS;
    hand::to_js(&value)
}

/// The numbers [`flat`] lays per circle: the centre's two coordinates, the radius and the curvature.
#[wasm_bindgen]
pub fn num_apollonian_STRIDE() -> Result<usize, JsValue> {
    let value = mrly::num::apollonian::STRIDE;
    Ok(value)
}

/// The largest top `a` a Kummer set takes here, so an automaton holds at most `32 * 32` carry states.
#[wasm_bindgen]
pub fn num_kummer_MOST() -> Result<u64, JsValue> {
    let value = mrly::num::kummer::MOST;
    Ok(value)
}

/// The most moves a [`Carry`] table holds, its states times the prime, `2^24`.
#[wasm_bindgen]
pub fn num_kummer_TABLE() -> Result<u64, JsValue> {
    let value = mrly::num::kummer::TABLE;
    Ok(value)
}

/// Lists the lifts in the order the gallery draws them.
#[wasm_bindgen]
pub fn num_morse_LIFTS() -> Result<JsValue, JsValue> {
    let value = mrly::num::morse::LIFTS;
    hand::to_js(&value)
}

/// The Apery constant, the value zeta takes at three.
#[wasm_bindgen]
pub fn num_series_APERY() -> Result<f64, JsValue> {
    let value = mrly::num::series::APERY;
    Ok(value)
}

/// The Basel constant, pi squared over six, the value zeta takes at two.
#[wasm_bindgen]
pub fn num_series_BASEL() -> Result<f64, JsValue> {
    let value = mrly::num::series::BASEL;
    Ok(value)
}

/// The Catalan constant, the value the Dirichlet beta function takes at two.
#[wasm_bindgen]
pub fn num_series_CATALAN() -> Result<f64, JsValue> {
    let value = mrly::num::series::CATALAN;
    Ok(value)
}

/// The Euler constant, the limit of the harmonic sum less the logarithm.
#[wasm_bindgen]
pub fn num_series_EULER() -> Result<f64, JsValue> {
    let value = mrly::num::series::EULER;
    Ok(value)
}

/// The visible density, six over pi squared, the share of lattice pairs that are coprime.
#[wasm_bindgen]
pub fn num_series_VISIBLE() -> Result<f64, JsValue> {
    let value = mrly::num::series::VISIBLE;
    Ok(value)
}

/// The limit of the plane Wallis sieve's surviving area, pi over four.
#[wasm_bindgen]
pub fn num_sieve_PLANE_LIMIT() -> Result<f64, JsValue> {
    let value = mrly::num::sieve::PLANE_LIMIT;
    Ok(value)
}

/// The t where the walk hands over from Euler-Maclaurin to Riemann-Siegel.
#[wasm_bindgen]
pub fn num_zeta_JOIN() -> Result<f64, JsValue> {
    let value = mrly::num::zeta::JOIN;
    Ok(value)
}

/// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
#[wasm_bindgen]
pub struct num_apollonian_Circle {
    inner: mrly::num::apollonian::Circle,
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

/// The symmetric window of one ring: every point within a reach, with the norms sieved once.
#[wasm_bindgen]
pub struct num_gauss_Window {
    inner: mrly::num::gauss::Window,
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = mrly::num::gauss::Window::new(ring, radius);
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

/// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
#[wasm_bindgen]
pub struct num_prime_Sieve {
    inner: mrly::num::prime::Sieve,
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
        let value = mrly::num::prime::Sieve::new(limit);
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

/// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
#[wasm_bindgen]
pub struct num_zeta_Line {
    inner: mrly::num::zeta::Line,
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
    /// Counts the zeros on the line below t: the sign changes of Z on ten steps a Gram interval, the Riemann-Siegel main sum turned from step to step and its remainder read only where it could flip the sign.
    pub fn count(&self, t: f64) -> Result<usize, JsValue> {
        let value = self.inner.count(t);
        Ok(value)
    }
    /// Returns the default Line.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<num_zeta_Line, JsValue> {
        let value = mrly::num::zeta::Line::default();
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
    pub fn maclaurin(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.maclaurin(t);
        hand::to_js(&value)
    }
    /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<num_zeta_Line, JsValue> {
        let value = mrly::num::zeta::Line::new();
        Ok(num_zeta_Line { inner: value })
    }
    /// Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.
    pub fn novelty_coefficients(&self, gammas: &[f64]) -> Result<JsValue, JsValue> {
        let value = self.inner.novelty_coefficients(gammas);
        hand::to_js(&value)
    }
    /// Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.
    pub fn pair(&self, s: JsValue) -> Result<JsValue, JsValue> {
        let s = hand::from_js::<mrly::num::zeta::Complex>(&s)?;
        let value = self.inner.pair(s);
        hand::to_js(&value)
    }
    /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
    pub fn point(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.point(t);
        hand::to_js(&value)
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

/// What a point of the ring is.
#[wasm_bindgen]
pub struct num_gauss_Class {}

#[wasm_bindgen]
impl num_gauss_Class {
    /// Returns whether the class is prime.
    pub fn prime(class_: JsValue) -> Result<bool, JsValue> {
        let class_ = hand::from_js::<mrly::num::gauss::Class>(&class_)?;
        let value = class_.prime();
        Ok(value)
    }
    /// Returns the class as a word.
    pub fn word(class_: JsValue) -> Result<String, JsValue> {
        let class_ = hand::from_js::<mrly::num::gauss::Class>(&class_)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.count(radius);
        Ok(value)
    }
    /// Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.
    pub fn div_rem(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
    /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
    pub fn fate(ring: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let n = hand::u64_from_js(&n)?;
        let value = ring.fate(n);
        hand::to_js(&value)
    }
    /// Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.
    pub fn gaussian_gcd(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let p = hand::u64_from_js(&p)?;
        let value = ring.inert(p);
        Ok(value)
    }
    /// Returns the product of two points.
    pub fn mul(ring: JsValue, arg1: JsValue, arg2: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let value = mrly::num::gauss::Ring::named(name);
        hand::to_js(&value)
    }
    /// Returns the point nearest a place in the plane.
    pub fn nearest(ring: JsValue, x: f64, y: f64) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let value = ring.nearest(x, y);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the norm of a point: its squared length.
    pub fn norm(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.norm(a, b);
        Ok(value)
    }
    /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
    pub fn place(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.place(a, b);
        hand::to_js(&value)
    }
    /// Returns the one rational prime that ramifies: 2 or 3.
    pub fn ramified(ring: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let value = ring.ramified();
        Ok(value)
    }
    /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
    pub fn reach(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.reach(a, b);
        Ok(value)
    }
    /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
    pub fn symmetry(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let value = ring.symmetry();
        Ok(value)
    }
    /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
    pub fn top(ring: JsValue, radius: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.top(radius);
        Ok(value)
    }
    /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
    pub fn turn(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
        let value = ring.units();
        Ok(value)
    }
    /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
    pub fn whole(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrly::num::gauss::Ring>(&ring)?;
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
        let value = mrly::num::morse::Lift::all();
        hand::to_js(&value)
    }
    /// Returns the sign at a site, zero for plus one and one for minus one.
    pub fn at(lift: JsValue, i: JsValue, j: JsValue) -> Result<u8, JsValue> {
        let lift = hand::from_js::<mrly::num::morse::Lift>(&lift)?;
        let i = hand::u64_from_js(&i)?;
        let j = hand::u64_from_js(&j)?;
        let value = lift.at(i, j);
        Ok(value)
    }
    /// Returns the lift's formula, written the way the page prints it.
    pub fn formula(lift: JsValue) -> Result<String, JsValue> {
        let lift = hand::from_js::<mrly::num::morse::Lift>(&lift)?;
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
        let value = mrly::num::spiral::Growth::all();
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
        let value = mrly::num::spiral::Lattice::all();
        hand::to_js(&value)
    }
    /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
    pub fn count(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
        let value = lattice.count(side);
        Ok(value)
    }
    /// Returns the number at a cell, one at the origin.
    pub fn n(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.n(x, y);
        Ok(value)
    }
    /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
    pub fn radius(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
        let value = lattice.radius(side);
        Ok(value)
    }
    /// Returns the ring a number sits on, zero for one.
    pub fn ring(lattice: JsValue, n: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.ring(n);
        Ok(value)
    }
    /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
    pub fn ring_of(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.ring_of(x, y);
        Ok(value)
    }
    /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
    pub fn xy(lattice: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let lattice = hand::from_js::<mrly::num::spiral::Lattice>(&lattice)?;
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
        let value = mrly::num::spiral::Mark::all();
        hand::to_js(&value)
    }
}

/// A complex number: a real and an imaginary part.
#[wasm_bindgen]
pub struct num_zeta_Complex {}

#[wasm_bindgen]
impl num_zeta_Complex {
    /// Returns the modulus.
    pub fn abs(z: JsValue) -> Result<f64, JsValue> {
        let z = hand::from_js::<mrly::num::zeta::Complex>(&z)?;
        let value = mrly::num::zeta::Complex::abs(z);
        Ok(value)
    }
    /// Returns the principal argument.
    pub fn arg(z: JsValue) -> Result<f64, JsValue> {
        let z = hand::from_js::<mrly::num::zeta::Complex>(&z)?;
        let value = mrly::num::zeta::Complex::arg(z);
        Ok(value)
    }
    /// Returns the default Complex.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<JsValue, JsValue> {
        let value = mrly::num::zeta::Complex::default();
        hand::to_js(&value)
    }
    /// Returns the exponential.
    pub fn exp(z: JsValue) -> Result<JsValue, JsValue> {
        let z = hand::from_js::<mrly::num::zeta::Complex>(&z)?;
        let value = mrly::num::zeta::Complex::exp(z);
        hand::to_js(&value)
    }
    /// Returns the principal logarithm.
    pub fn ln(z: JsValue) -> Result<JsValue, JsValue> {
        let z = hand::from_js::<mrly::num::zeta::Complex>(&z)?;
        let value = mrly::num::zeta::Complex::ln(z);
        hand::to_js(&value)
    }
    /// Builds a complex number from its parts.
    #[wasm_bindgen(js_name = "new")]
    pub fn new_(re: f64, im: f64) -> Result<JsValue, JsValue> {
        let value = mrly::num::zeta::Complex::new(re, im);
        hand::to_js(&value)
    }
    /// Returns a unit complex number at the given angle.
    pub fn turn(angle: f64) -> Result<JsValue, JsValue> {
        let value = mrly::num::zeta::Complex::turn(angle);
        hand::to_js(&value)
    }
}
