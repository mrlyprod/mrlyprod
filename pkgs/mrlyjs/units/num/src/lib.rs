#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]

mod hand;

use wasm_bindgen::prelude::*;

/// The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.
#[wasm_bindgen]
pub fn apollonian_form(u: JsValue, v: JsValue) -> Result<JsValue, JsValue> {
    let u = hand::array_from_js::<_, 4>(&u, hand::i64_from_js)?;
    let v = hand::array_from_js::<_, 4>(&v, hand::i64_from_js)?;
    let value = mrlyrs::num::apollonian::form(u, v);
    Ok(JsValue::from_str(&value.to_string()))
}

/// The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.
#[wasm_bindgen]
pub fn apollonian_frame(p: JsValue) -> Result<Vec<f64>, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::frame(&p);
    Ok(value.to_vec())
}

/// Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.
#[wasm_bindgen]
pub fn apollonian_grow(name: &str, cap: JsValue) -> Result<JsValue, JsValue> {
    let cap = hand::i64_from_js(&cap)?;
    let value = mrlyrs::num::apollonian::grow(name, cap).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.
#[wasm_bindgen]
pub fn apollonian_is_ford(c: &apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrlyrs::num::apollonian::is_ford(c.inner);
    Ok(value)
}

/// Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.
#[wasm_bindgen]
pub fn apollonian_on_line(c: &apollonian_Circle) -> Result<bool, JsValue> {
    let value = mrlyrs::num::apollonian::on_line(c.inner);
    Ok(value)
}

/// Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.
#[wasm_bindgen]
pub fn apollonian_reflect(q: JsValue, at: usize) -> Result<apollonian_Circle, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::reflect(&q, at);
    Ok(apollonian_Circle { inner: value })
}

/// The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.
#[wasm_bindgen]
pub fn apollonian_root(name: &str) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::apollonian::root(name).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(apollonian_Circle { inner: *x1 }))
    })
}

/// Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.
#[wasm_bindgen]
pub fn apollonian_shadow(p: JsValue, order: usize) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::shadow(&p, order).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.
#[wasm_bindgen]
pub fn apollonian_sound(q: JsValue) -> Result<bool, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::sound(&q);
    Ok(value)
}

/// The quadruple with the circle at the seat replaced by its reflection.
#[wasm_bindgen]
pub fn apollonian_swap(q: JsValue, at: usize) -> Result<JsValue, JsValue> {
    let q = hand::array_from_js::<_, 4>(&q, |x1| {
        hand::from_js::<mrlyrs::num::apollonian::Circle>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::apollonian::swap(&q, at);
    hand::list_to_js(&value, |x1| {
        Ok(JsValue::from(apollonian_Circle { inner: *x1 }))
    })
}

/// The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.
#[wasm_bindgen]
pub fn apollonian_touches(p: JsValue) -> Result<JsValue, JsValue> {
    let p = hand::from_js::<mrlyrs::num::apollonian::Packing>(&p)?;
    let value = mrlyrs::num::apollonian::touches(&p);
    hand::to_js(&value)
}

/// Adds two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn blend_add(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::add(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Convolves two sequences, keeping the exact prefix their shared length affords.
#[wasm_bindgen]
pub fn blend_cauchy(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::cauchy(&a, &b).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the monic characteristic polynomial of a recurrence, highest power first.
#[wasm_bindgen]
pub fn blend_characteristic(coefficients: JsValue) -> Result<JsValue, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrlyrs::num::blend::characteristic(&coefficients);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

/// Keeps every step-th term from the offset onward.
#[wasm_bindgen]
pub fn blend_decimate(a: JsValue, step: usize, offset: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::decimate(&a, step, offset).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the first differences of a sequence, one term shorter.
#[wasm_bindgen]
pub fn blend_delta(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::delta(&a);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.
#[wasm_bindgen]
pub fn blend_growth(coefficients: JsValue) -> Result<f64, JsValue> {
    let coefficients = hand::list_from_js(&coefficients, |x1| {
        Ok((
            hand::i128_from_js(&hand::item(x1, 0)?)?,
            hand::i128_from_js(&hand::item(x1, 1)?)?,
        ))
    })?;
    let value = mrlyrs::num::blend::growth(&coefficients);
    Ok(value)
}

/// Multiplies two sequences term by term over their shared length.
#[wasm_bindgen]
pub fn blend_hadamard(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::hadamard(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
#[wasm_bindgen]
pub fn blend_recurrence(terms: JsValue) -> Result<JsValue, JsValue> {
    let terms = hand::list_from_js(&terms, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::recurrence(&terms);
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
pub fn blend_scale(a: JsValue, factor: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let factor = hand::i128_from_js(&factor)?;
    let value = mrlyrs::num::blend::scale(&a, factor);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Drops the first terms of a sequence.
#[wasm_bindgen]
pub fn blend_shift(a: JsValue, count: usize) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::shift(&a, count);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Returns the partial sums of a sequence.
#[wasm_bindgen]
pub fn blend_sigma(a: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::sigma(&a).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Subtracts the second sequence from the first over their shared length.
#[wasm_bindgen]
pub fn blend_sub(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::list_from_js(&a, hand::i128_from_js)?;
    let b = hand::list_from_js(&b, hand::i128_from_js)?;
    let value = mrlyrs::num::blend::sub(&a, &b);
    hand::list_to_js(&value, |x1| Ok(JsValue::from_str(&x1.to_string())))
}

/// Reports whether the packed function outputs one on exactly half of its inputs.
#[wasm_bindgen]
pub fn boolean_is_balanced(code: JsValue, n: usize) -> Result<bool, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::is_balanced(code, n);
    Ok(value)
}

/// Returns how far the packed function sits from every affine function, zero when it is one.
#[wasm_bindgen]
pub fn boolean_nonlinearity(code: JsValue, n: usize) -> Result<i64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::nonlinearity(code, n);
    Ok(value)
}

/// Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.
#[wasm_bindgen]
pub fn boolean_sac(code: JsValue, n: usize) -> Result<f64, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::sac(code, n);
    Ok(value)
}

/// Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.
#[wasm_bindgen]
pub fn boolean_walsh_spectrum(code: JsValue, n: usize) -> Result<Vec<i64>, JsValue> {
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::boolean::walsh_spectrum(code, n);
    Ok(value)
}

/// Returns the digits a bitmask names inside the base, ascending.
#[wasm_bindgen]
pub fn design_digits_of(mask: u32, base: JsValue) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::design::digits_of(mask, base);
    Ok(value)
}

/// Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.
#[wasm_bindgen]
pub fn design_echo_series(
    values: JsValue,
    log_x: &[f64],
    exponent: f64,
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrlyrs::num::design::echo_series(&values, log_x, exponent);
    Ok(value)
}

/// Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.
#[wasm_bindgen]
pub fn design_elements(base: JsValue, digits: JsValue, depth: usize) -> Result<Vec<u64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrlyrs::num::design::elements(base, &digits, depth);
    Ok(value)
}

/// Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.
#[wasm_bindgen]
pub fn design_log_grid(values: JsValue, samples: usize) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let value = mrlyrs::num::design::log_grid(&values, samples);
    Ok(value)
}

/// Returns the running median of the power over a window of the given width, the window clamped at the ends.
#[wasm_bindgen]
pub fn design_median_floor(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::median_floor(power, width);
    Ok(value)
}

/// Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.
#[wasm_bindgen]
pub fn design_meter(mu: &[i8]) -> Result<Vec<i64>, JsValue> {
    let value = mrlyrs::num::design::meter(mu);
    Ok(value)
}

/// Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.
#[wasm_bindgen]
pub fn design_nearest(value: f64, list: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::design::nearest(value, list);
    Ok(value)
}

/// Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.
#[wasm_bindgen]
pub fn design_peaks(
    gamma: &[f64],
    score: &[f64],
    band: JsValue,
    threshold: f64,
) -> Result<Vec<usize>, JsValue> {
    let band = hand::from_js::<(f64, f64)>(&band)?;
    let value = mrlyrs::num::design::peaks(gamma, score, band, threshold).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.
#[wasm_bindgen]
pub fn design_pole_lattice(base: JsValue, top: f64) -> Result<Vec<f64>, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::design::pole_lattice(base, top);
    Ok(value)
}

/// Reads the running meter at every point of the log grid and divides by x to the exponent.
#[wasm_bindgen]
pub fn design_resample(
    values: JsValue,
    running: JsValue,
    exponent: f64,
    log_x: &[f64],
) -> Result<Vec<f64>, JsValue> {
    let values = hand::list_from_js(&values, hand::u64_from_js)?;
    let running = hand::list_from_js(&running, hand::i64_from_js)?;
    let value = mrlyrs::num::design::resample(&values, &running, exponent, log_x);
    Ok(value)
}

/// Returns the power over its local median floor, the score a peak is read against.
#[wasm_bindgen]
pub fn design_score(power: &[f64], width: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::score(power, width);
    Ok(value)
}

/// Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them.
#[wasm_bindgen]
pub fn design_size(digits: JsValue, depth: usize) -> Result<JsValue, JsValue> {
    let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
    let value = mrlyrs::num::design::size(&digits, depth);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.
#[wasm_bindgen]
pub fn design_spectrum(log_x: &[f64], series: &[f64]) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::design::spectrum(log_x, series).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.
#[wasm_bindgen]
pub fn design_upper_rms(series: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::design::upper_rms(series);
    Ok(value)
}

/// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
#[wasm_bindgen]
pub fn factor_aliquot(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::aliquot(number);
    Ok(value)
}

/// Returns whether two numbers share no divisor above one.
#[wasm_bindgen]
pub fn factor_coprime(a: usize, b: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::factor::coprime(a, b);
    Ok(value)
}

/// Builds every divisor of a wide number from its factorization, ascending, empty for zero.
#[wasm_bindgen]
pub fn factor_divisors(number: JsValue) -> Result<Vec<u64>, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::factor::divisors(number);
    Ok(value)
}

/// Returns the factorial of the number, the product of one through it, erring past thirty-four.
#[wasm_bindgen]
pub fn factor_factorial(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::factorial(number).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn factor_factorize(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::factorize(number);
    hand::to_js(&value)
}

/// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn factor_factorize_wide(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::factor::factorize_wide(number);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            hand::to_js(&x1.1)?,
        ]))
    })
}

/// Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.
#[wasm_bindgen]
pub fn factor_gcd(a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
    let a = hand::u128_from_js(&a)?;
    let b = hand::u128_from_js(&b)?;
    let value = mrlyrs::num::factor::gcd(a, b);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the least common multiple of two numbers, zero when either side is zero.
#[wasm_bindgen]
pub fn factor_lcm(a: usize, b: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::lcm(a, b);
    Ok(value)
}

/// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
#[wasm_bindgen]
pub fn factor_mobius(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::factor::mobius(number);
    Ok(value)
}

/// Sieves the Mobius values of zero through the limit in one pass.
#[wasm_bindgen]
pub fn factor_mobius_sieve(limit: usize) -> Result<Vec<i8>, JsValue> {
    let value = mrlyrs::num::factor::mobius_sieve(limit);
    Ok(value)
}

/// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
#[wasm_bindgen]
pub fn factor_radical(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::radical(number);
    Ok(value)
}

/// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
#[wasm_bindgen]
pub fn factor_reduce(numerator: JsValue, denominator: JsValue) -> Result<JsValue, JsValue> {
    let numerator = hand::u128_from_js(&numerator)?;
    let denominator = hand::u128_from_js(&denominator)?;
    let value = mrlyrs::num::factor::reduce(numerator, denominator);
    Ok(hand::tuple_to_js(&[
        JsValue::from_str(&value.0.to_string()),
        JsValue::from_str(&value.1.to_string()),
    ]))
}

/// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
#[wasm_bindgen]
pub fn factor_sigma(number: usize, power: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::factor::sigma(number, power);
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns whether no prime squares into the number, true for one and false for zero.
#[wasm_bindgen]
pub fn factor_squarefree(number: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::factor::squarefree(number);
    Ok(value)
}

/// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
#[wasm_bindgen]
pub fn factor_totient(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::factor::totient(number);
    Ok(value)
}

/// Sieves the Euler totients of zero through n in one pass, the run beside the single value.
#[wasm_bindgen]
pub fn factor_totients(n: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::factor::totients(n);
    Ok(value)
}

/// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
#[wasm_bindgen]
pub fn factor_twisted(number: usize, rhythm: &[i8]) -> Result<i64, JsValue> {
    let value = mrlyrs::num::factor::twisted(number, rhythm);
    Ok(value)
}

/// Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.
#[wasm_bindgen]
pub fn fft_convolve(field: &[f64], kernel: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::convolve(field, kernel, size).map_err(hand::throw)?;
    Ok(value)
}

/// Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.
#[wasm_bindgen]
pub fn fft_convolve_with(
    field: &[f64],
    kernel_re: &[f64],
    kernel_im: &[f64],
    size: usize,
) -> Result<Vec<f64>, JsValue> {
    let value =
        mrlyrs::num::fft::convolve_with(field, kernel_re, kernel_im, size).map_err(hand::throw)?;
    Ok(value)
}

/// Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.
#[wasm_bindgen]
pub fn fft_embed_kernel(mask: &[u8], side: usize, size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::embed_kernel(mask, side, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.
#[wasm_bindgen]
pub fn fft_log_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::log_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.
#[wasm_bindgen]
pub fn fft_magnitude_spectrum(field: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::magnitude_spectrum(field, size).map_err(hand::throw)?;
    Ok(value)
}

/// Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn fft_peak_ring(profile: &[f64]) -> Result<usize, JsValue> {
    let value = mrlyrs::num::fft::peak_ring(profile);
    Ok(value)
}

/// Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.
#[wasm_bindgen]
pub fn fft_peak_wavelength(profile: &[f64], size: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::fft::peak_wavelength(profile, size);
    Ok(value)
}

/// Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.
#[wasm_bindgen]
pub fn fft_radial_profile(spectrum: &[f64], size: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::fft::radial_profile(spectrum, size).map_err(hand::throw)?;
    Ok(value)
}

/// Transforms a real size-square field forward by fft2, returning the real and imaginary parts.
#[wasm_bindgen]
pub fn fft_transform(field: &[f64], size: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::fft::transform(field, size).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::typed(&(value.0)[..]),
        hand::typed(&(value.1)[..]),
    ]))
}

/// Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.
#[wasm_bindgen]
pub fn gauss_classes(ring: JsValue, bound: JsValue) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let bound = hand::u64_from_js(&bound)?;
    let value = mrlyrs::num::gauss::classes(ring, bound);
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from(x1.0),
            JsValue::from(x1.1),
        ]))
    })
}

/// Returns the norm from one through the limit with the most points and that count, the earliest on a tie.
#[wasm_bindgen]
pub fn gauss_peak(ring: JsValue, limit: usize) -> Result<JsValue, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let value = mrlyrs::num::gauss::peak(ring, limit);
    hand::to_js(&value)
}

/// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
#[wasm_bindgen]
pub fn gauss_shells(ring: JsValue, limit: usize) -> Result<Vec<u32>, JsValue> {
    let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
    let value = mrlyrs::num::gauss::shells(ring, limit);
    Ok(value)
}

/// Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.
#[wasm_bindgen]
pub fn ladder_cofactor(
    design: &ladder_Design,
    s: &zeta_Complex,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::num::ladder::cofactor(&design.inner, s.inner, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.
#[wasm_bindgen]
pub fn ladder_residue(
    design: &ladder_Design,
    m: usize,
    j: JsValue,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let j = hand::i64_from_js(&j)?;
    let value =
        mrlyrs::num::ladder::residue(&design.inner, m, j, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Returns `zeta_F(s)` and the bound it is known to.
#[wasm_bindgen]
pub fn ladder_zeta(
    design: &ladder_Design,
    s: &zeta_Complex,
    tolerance: f64,
) -> Result<JsValue, JsValue> {
    let value =
        mrlyrs::num::ladder::zeta(&design.inner, s.inner, tolerance).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        JsValue::from(zeta_Complex { inner: value.0 }),
        hand::to_js(&value.1)?,
    ]))
}

/// Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.
#[wasm_bindgen]
pub fn lattice_coprime_pairs(n: usize) -> Result<u64, JsValue> {
    let value = mrlyrs::num::lattice::coprime_pairs(n);
    Ok(value)
}

/// Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.
#[wasm_bindgen]
pub fn lattice_farey(order: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::farey(order);
    hand::to_js(&value)
}

/// Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.
#[wasm_bindgen]
pub fn lattice_grid(n: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::grid(n);
    hand::to_js(&value)
}

/// Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.
#[wasm_bindgen]
pub fn lattice_new_nodes(n: usize) -> Result<u64, JsValue> {
    let value = mrlyrs::num::lattice::new_nodes(n);
    Ok(value)
}

/// Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.
#[wasm_bindgen]
pub fn lattice_pi_estimate(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::pi_estimate(n);
    Ok(value)
}

/// Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.
#[wasm_bindgen]
pub fn lattice_recovered(n: usize, dimension: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::recovered(n, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The density the visible count of a window in the dimension walks to, one over zeta of the dimension.
#[wasm_bindgen]
pub fn lattice_visible_density(dimension: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::visible_density(dimension).map_err(hand::throw)?;
    Ok(value)
}

/// The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.
#[wasm_bindgen]
pub fn lattice_zeta_factor(dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::lattice::zeta_factor(dimension);
    hand::to_js(&value)
}

/// The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.
#[wasm_bindgen]
pub fn lattice_zeta_whole(s: u32) -> Result<f64, JsValue> {
    let value = mrlyrs::num::lattice::zeta_whole(s).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the count of allowed windows `card W`, the bits the code sets inside its window range.
#[wasm_bindgen]
pub fn memory_allowed_windows(rule: &memory_Rule) -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::allowed_windows(&rule.inner);
    Ok(value)
}

/// Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.
#[wasm_bindgen]
pub fn memory_cells(rule: &memory_Rule, level: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::memory::cells(&rule.inner, level);
    Ok(value)
}

/// Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.
#[wasm_bindgen]
pub fn memory_counts(rule: &memory_Rule, levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::memory::counts(&rule.inner, levels);
    Ok(value)
}

/// Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.
#[wasm_bindgen]
pub fn memory_exponent(rule: &memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::exponent(&rule.inner);
    Ok(value)
}

/// Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.
#[wasm_bindgen]
pub fn memory_kappa(rule: &memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::kappa(&rule.inner);
    Ok(value)
}

/// Returns the Perron root of the transfer matrix, the count's growth per level.
#[wasm_bindgen]
pub fn memory_perron(rule: &memory_Rule) -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::perron(&rule.inner);
    Ok(value)
}

/// Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.
#[wasm_bindgen]
pub fn memory_transfer(rule: &memory_Rule) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::memory::transfer(&rule.inner);
    hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
}

/// Returns the run-boundary word, one wherever a letter differs from the next.
#[wasm_bindgen]
pub fn morse_boundary(word: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::boundary(word);
    Ok(value)
}

/// Exclusive-ors two grids of the same length, site by site.
#[wasm_bindgen]
pub fn morse_difference(a: &[u8], b: &[u8]) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::difference(a, b);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the digit rule.
#[wasm_bindgen]
pub fn morse_digits(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::digits(length);
    Ok(value)
}

/// Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.
#[wasm_bindgen]
pub fn morse_doubling(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::doubling(length);
    Ok(value)
}

/// Counts the sites where two grids of the same length differ.
#[wasm_bindgen]
pub fn morse_faults(a: &[u8], b: &[u8]) -> Result<usize, JsValue> {
    let value = mrlyrs::num::morse::faults(a, b);
    Ok(value)
}

/// Tests a grid against the Kronecker power of its own corner tile.
#[wasm_bindgen]
pub fn morse_fold(grid: &[u8], side: usize, number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::morse::fold(grid, side, number).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the Thue-Morse letter at the place, the parity of its binary digit sum.
#[wasm_bindgen]
pub fn morse_letter(place: JsValue) -> Result<u8, JsValue> {
    let place = hand::u64_from_js(&place)?;
    let value = mrlyrs::num::morse::letter(place);
    Ok(value)
}

/// Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.
#[wasm_bindgen]
pub fn morse_lift(kind: JsValue, side: usize) -> Result<Vec<u8>, JsValue> {
    let kind = hand::from_js::<mrlyrs::num::morse::Lift>(&kind)?;
    let value = mrlyrs::num::morse::lift(kind, side);
    Ok(value)
}

/// Folds a tile of the side into its Kronecker power at the level, one bit per site.
#[wasm_bindgen]
pub fn morse_power(tile: &[u8], number: usize, level: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::power(tile, number, level).map_err(hand::throw)?;
    Ok(value)
}

/// Repeats a tile until it fills a grid of the side.
#[wasm_bindgen]
pub fn morse_repeat(tile: &[u8], number: usize, side: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::repeat(tile, number, side);
    Ok(value)
}

/// Returns the lengths of the maximal blocks of one repeated letter, in order.
#[wasm_bindgen]
pub fn morse_runs(word: &[u8]) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::morse::runs(word);
    Ok(value)
}

/// Returns the substitution stage after the rounds, a word of length two to the rounds.
#[wasm_bindgen]
pub fn morse_stage(rounds: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::stage(rounds);
    Ok(value)
}

/// Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.
#[wasm_bindgen]
pub fn morse_substitution(length: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::substitution(length);
    Ok(value)
}

/// Blows a grid up by the scale, every site becoming a scale-by-scale block.
#[wasm_bindgen]
pub fn morse_upsample(grid: &[u8], side: usize, scale: usize) -> Result<Vec<u8>, JsValue> {
    let value = mrlyrs::num::morse::upsample(grid, side, scale);
    Ok(value)
}

/// Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.
#[wasm_bindgen]
pub fn prime_chart(top: usize, bins: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::chart(top, bins);
    hand::to_js(&value)
}

/// Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.
#[wasm_bindgen]
pub fn prime_flags(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::flags(limit);
    hand::to_js(&value)
}

/// Returns the count of unordered pairs of primes summing to the number, zero below four.
#[wasm_bindgen]
pub fn prime_goldbach(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::goldbach(number);
    Ok(value)
}

/// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
#[wasm_bindgen]
pub fn prime_goldbach_record(top: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::prime::goldbach_record(top);
    Ok(value)
}

/// Returns whether the number is prime, by trial division on the six-step wheel.
#[wasm_bindgen]
pub fn prime_is_prime(number: usize) -> Result<bool, JsValue> {
    let value = mrlyrs::num::prime::is_prime(number);
    Ok(value)
}

/// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
#[wasm_bindgen]
pub fn prime_pile(number: JsValue) -> Result<JsValue, JsValue> {
    let number = hand::u64_from_js(&number)?;
    let value = mrlyrs::num::prime::pile(number);
    hand::to_js(&value)
}

/// Returns the count of primes at or below n.
#[wasm_bindgen]
pub fn prime_prime_count(n: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::prime_count(n);
    Ok(value)
}

/// Returns the smallest prime at or above the number.
#[wasm_bindgen]
pub fn prime_prime_from(number: usize) -> Result<usize, JsValue> {
    let value = mrlyrs::num::prime::prime_from(number);
    Ok(value)
}

/// Returns the primes up to the limit, the finished sieve read as a list.
#[wasm_bindgen]
pub fn prime_primes(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::prime::primes(limit);
    Ok(value)
}

/// Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.
#[wasm_bindgen]
pub fn prime_rectangles(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::rectangles(number);
    hand::to_js(&value)
}

/// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
#[wasm_bindgen]
pub fn prime_splits(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::splits(number);
    hand::to_js(&value)
}

/// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
#[wasm_bindgen]
pub fn prime_squares(number: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::squares(number);
    hand::to_js(&value)
}

/// Returns one prime object for every prime up to and including the limit.
#[wasm_bindgen]
pub fn prime_study(limit: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::prime::study(limit);
    hand::to_js(&value)
}

/// Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.
#[wasm_bindgen]
pub fn radix_flowsnake() -> Result<radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::flowsnake().map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.
#[wasm_bindgen]
pub fn radix_gasket() -> Result<radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::gasket().map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.
#[wasm_bindgen]
pub fn radix_koch() -> Result<radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::koch().map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.
#[wasm_bindgen]
pub fn radix_terdragon() -> Result<radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::terdragon().map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.
#[wasm_bindgen]
pub fn radix_tile(m: JsValue, code: JsValue) -> Result<radix_Radix, JsValue> {
    let m = hand::u64_from_js(&m)?;
    let code = hand::u128_from_js(&code)?;
    let value = mrlyrs::num::radix::tile(m, code).map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.
#[wasm_bindgen]
pub fn radix_twindragon() -> Result<radix_Radix, JsValue> {
    let value = mrlyrs::num::radix::twindragon().map_err(hand::throw)?;
    Ok(radix_Radix { inner: value })
}

/// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
#[wasm_bindgen]
pub fn series_basel(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::basel(n);
    Ok(value)
}

/// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
#[wasm_bindgen]
pub fn series_bernoulli(count: usize) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::series::bernoulli(count).map_err(hand::throw)?;
    hand::list_to_js(&value, |x1| {
        Ok(hand::tuple_to_js(&[
            JsValue::from_str(&x1.0.to_string()),
            JsValue::from_str(&x1.1.to_string()),
        ]))
    })
}

/// Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.
#[wasm_bindgen]
pub fn series_beta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::beta(s, terms);
    Ok(value)
}

/// Returns the powers of two up to the limit.
#[wasm_bindgen]
pub fn series_binary(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::binary(limit);
    Ok(value)
}

/// Returns the distinct Catalan numbers up to the limit.
#[wasm_bindgen]
pub fn series_catalan(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::catalan(limit);
    Ok(value)
}

/// Returns the mod-three rhythm of the number: zero, one, minus one.
#[wasm_bindgen]
pub fn series_chi3(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi3(number);
    Ok(value)
}

/// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
#[wasm_bindgen]
pub fn series_chi4(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi4(number);
    Ok(value)
}

/// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
#[wasm_bindgen]
pub fn series_chi8(number: usize) -> Result<i8, JsValue> {
    let value = mrlyrs::num::series::chi8(number);
    Ok(value)
}

/// Returns the L-series partial sum with a periodic rhythm painted on the terms.
#[wasm_bindgen]
pub fn series_dirichlet(s: f64, rhythm: &[i8], terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::dirichlet(s, rhythm, terms);
    Ok(value)
}

/// Returns one plus one over n raised to the n, walking to the natural base.
#[wasm_bindgen]
pub fn series_e_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::e_partial(n);
    Ok(value)
}

/// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
#[wasm_bindgen]
pub fn series_euler_gamma_partial(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::euler_gamma_partial(n);
    Ok(value)
}

/// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
#[wasm_bindgen]
pub fn series_euler_product(s: f64, limit: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::euler_product(s, limit);
    Ok(value)
}

/// Returns the even numbers up to the limit.
#[wasm_bindgen]
pub fn series_evens(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::evens(limit);
    Ok(value)
}

/// Returns the distinct Fibonacci numbers up to the limit.
#[wasm_bindgen]
pub fn series_fibonacci(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::fibonacci(limit);
    Ok(value)
}

/// Returns the partial harmonic sum, the reciprocals of one through the term count.
#[wasm_bindgen]
pub fn series_harmonic(terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::harmonic(terms);
    Ok(value)
}

/// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
#[wasm_bindgen]
pub fn series_lambda(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::lambda(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
#[wasm_bindgen]
pub fn series_leibniz(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::leibniz(n);
    Ok(value)
}

/// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
#[wasm_bindgen]
pub fn series_li(x: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::li(x);
    Ok(value)
}

/// Returns the Mertens function at n, the Mobius values of one through n summed.
#[wasm_bindgen]
pub fn series_mertens(n: usize) -> Result<i64, JsValue> {
    let value = mrlyrs::num::series::mertens(n);
    Ok(value)
}

/// Returns the odd numbers up to the limit.
#[wasm_bindgen]
pub fn series_odds(limit: usize) -> Result<Vec<usize>, JsValue> {
    let value = mrlyrs::num::series::odds(limit);
    Ok(value)
}

/// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
#[wasm_bindgen]
pub fn series_visible(limit: usize, dimension: u32) -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::series::visible(limit, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
#[wasm_bindgen]
pub fn series_wallis_half_pi(n: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::wallis_half_pi(n);
    Ok(value)
}

/// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
#[wasm_bindgen]
pub fn series_wallis_quarter_pi(factors: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::wallis_quarter_pi(factors);
    Ok(value)
}

/// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
#[wasm_bindgen]
pub fn series_zeta(s: f64, terms: usize) -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::zeta(s, terms).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
#[wasm_bindgen]
pub fn sieve_cells(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::cells(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
#[wasm_bindgen]
pub fn sieve_exponent(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::exponent(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
#[wasm_bindgen]
pub fn sieve_flat_word(side: JsValue, levels: usize) -> Result<Vec<u64>, JsValue> {
    let side = hand::u64_from_js(&side)?;
    let value = mrlyrs::num::sieve::flat_word(side, levels);
    Ok(value)
}

/// Returns the punctures the word makes, one per surviving cell at every level.
#[wasm_bindgen]
pub fn sieve_holes(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::holes(&word, dimension).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
#[wasm_bindgen]
pub fn sieve_limit(word: JsValue, dimension: u32) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::limit(&word, dimension).map_err(hand::throw)?;
    hand::to_js(&value)
}

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
#[wasm_bindgen]
pub fn sieve_odd_word(levels: usize) -> Result<Vec<u64>, JsValue> {
    let value = mrlyrs::num::sieve::odd_word(levels);
    Ok(value)
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
#[wasm_bindgen]
pub fn sieve_punctures(word: JsValue, dimension: u32) -> Result<Vec<u64>, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::punctures(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
#[wasm_bindgen]
pub fn sieve_raster(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::raster(&word).map_err(hand::throw)?;
    Ok(hand::tuple_to_js(&[
        hand::to_js(&value.0)?,
        hand::typed(&(value.1)[..]),
    ]))
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
#[wasm_bindgen]
pub fn sieve_ratio(word: JsValue, dimension: u32) -> Result<f64, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::ratio(&word, dimension).map_err(hand::throw)?;
    Ok(value)
}

/// Returns the side of the word, the product of its letters' sides.
#[wasm_bindgen]
pub fn sieve_side(word: JsValue) -> Result<JsValue, JsValue> {
    let word = hand::list_from_js(&word, hand::u64_from_js)?;
    let value = mrlyrs::num::sieve::side(&word).map_err(hand::throw)?;
    Ok(JsValue::from_str(&value.to_string()))
}

/// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
#[wasm_bindgen]
pub fn sieve_solid_limit() -> Result<f64, JsValue> {
    let value = mrlyrs::num::sieve::solid_limit();
    Ok(value)
}

/// Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.
#[wasm_bindgen]
pub fn spiral_diagonal(
    lattice: JsValue,
    side: usize,
    a: JsValue,
    b: JsValue,
    c: JsValue,
) -> Result<JsValue, JsValue> {
    let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
    let a = hand::i64_from_js(&a)?;
    let b = hand::i64_from_js(&b)?;
    let c = hand::i64_from_js(&c)?;
    let value = mrlyrs::num::spiral::diagonal(lattice, side, a, b, c);
    hand::to_js(&value)
}

/// Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.
#[wasm_bindgen]
pub fn spiral_level_of(n: JsValue, base: JsValue) -> Result<u32, JsValue> {
    let n = hand::u64_from_js(&n)?;
    let base = hand::u64_from_js(&base)?;
    let value = mrlyrs::num::spiral::level_of(n, base);
    Ok(value)
}

/// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
#[wasm_bindgen]
pub fn spiral_marks(mark: JsValue, limit: usize) -> Result<Vec<i8>, JsValue> {
    let mark = hand::from_js::<mrlyrs::num::spiral::Mark>(&mark)?;
    let value = mrlyrs::num::spiral::marks(mark, limit);
    Ok(value)
}

/// Winds one to the top on the square spiral and lays a square tile on every cell, the snail.
#[wasm_bindgen]
pub fn spiral_snail(base: JsValue, top: JsValue, growth: JsValue) -> Result<JsValue, JsValue> {
    let base = hand::u64_from_js(&base)?;
    let top = hand::u64_from_js(&top)?;
    let growth = hand::from_js::<mrlyrs::num::spiral::Growth>(&growth)?;
    let value = mrlyrs::num::spiral::snail(base, top, growth);
    hand::to_js(&value)
}

/// The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.
#[wasm_bindgen]
pub fn zeta_bump(u: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::bump(u);
    Ok(value)
}

/// Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.
#[wasm_bindgen]
pub fn zeta_corrections(p: f64) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::zeta::corrections(p);
    Ok(value.to_vec())
}

/// Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.
#[wasm_bindgen]
pub fn zeta_kernel(p: f64) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::kernel(p);
    Ok(value)
}

/// Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.
#[wasm_bindgen]
pub fn zeta_mellin(s: &zeta_Complex) -> Result<zeta_Complex, JsValue> {
    let value = mrlyrs::num::zeta::mellin(s.inner);
    Ok(zeta_Complex { inner: value })
}

/// Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.
#[wasm_bindgen]
pub fn zeta_novelty_main() -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::novelty_main();
    Ok(value)
}

/// Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.
#[wasm_bindgen]
pub fn zeta_novelty_wave(gammas: &[f64], coef: JsValue, log_y: f64) -> Result<f64, JsValue> {
    let coef = hand::list_from_js(&coef, |x1| {
        hand::from_js::<mrlyrs::num::zeta::Complex>(&hand::plain(x1)?)
    })?;
    let value = mrlyrs::num::zeta::novelty_wave(gammas, &coef, log_y);
    Ok(value)
}

/// Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.
#[wasm_bindgen]
pub fn zeta_psi_formula(x: f64, gammas: &[f64]) -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::psi_formula(x, gammas);
    Ok(value)
}

/// Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.
#[wasm_bindgen]
pub fn zeta_psi_stair(x: usize) -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::zeta::psi_stair(x);
    Ok(value)
}

/// Returns a positive real base raised to a complex exponent.
#[wasm_bindgen]
pub fn zeta_raise(base: f64, exponent: &zeta_Complex) -> Result<zeta_Complex, JsValue> {
    let value = mrlyrs::num::zeta::raise(base, exponent.inner);
    Ok(zeta_Complex { inner: value })
}

/// Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.
#[wasm_bindgen]
pub fn zeta_sharp_novelty(prefix: JsValue, y: f64) -> Result<f64, JsValue> {
    let prefix = hand::list_from_js(&prefix, hand::u64_from_js)?;
    let value = mrlyrs::num::zeta::sharp_novelty(&prefix, y);
    Ok(value)
}

/// Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.
#[wasm_bindgen]
pub fn zeta_smoothed_novelty(phi: JsValue, y: f64, main: f64) -> Result<f64, JsValue> {
    let phi = hand::list_from_js(&phi, hand::u64_from_js)?;
    let value = mrlyrs::num::zeta::smoothed_novelty(&phi, y, main);
    Ok(value)
}

/// The most circles one growth makes before it gives up.
#[wasm_bindgen]
pub fn apollonian_CIRCLE_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::num::apollonian::CIRCLE_CAP;
    Ok(value)
}

/// The largest curvature a packing is grown to.
#[wasm_bindgen]
pub fn apollonian_CURVATURE_CAP() -> Result<i64, JsValue> {
    let value = mrlyrs::num::apollonian::CURVATURE_CAP;
    Ok(value)
}

/// The deepest the Farey stack is read against a packing.
#[wasm_bindgen]
pub fn apollonian_ORDER_CAP() -> Result<usize, JsValue> {
    let value = mrlyrs::num::apollonian::ORDER_CAP;
    Ok(value)
}

/// The root quadruples on offer: the strip first, then the bounded packings named by their curvatures.
#[wasm_bindgen]
pub fn apollonian_ROOTS() -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::apollonian::ROOTS;
    hand::to_js(&value)
}

/// The relative rounding allowance the double-precision matrix ladder charges against the scale it carries.
#[wasm_bindgen]
pub fn automaton_ROUNDING() -> Result<f64, JsValue> {
    let value = mrlyrs::num::automaton::ROUNDING;
    Ok(value)
}

/// The ordinates of the first fourteen nontrivial zeros of the Riemann zeta function, the imaginary parts of the zeros on the critical line in ascending order.
#[wasm_bindgen]
pub fn design_ZETA_ORDINATES() -> Result<Vec<f64>, JsValue> {
    let value = mrlyrs::num::design::ZETA_ORDINATES;
    Ok(value.to_vec())
}

/// The relative rounding allowance the double-precision ladder charges against the scale it carries.
#[wasm_bindgen]
pub fn ladder_ROUNDING() -> Result<f64, JsValue> {
    let value = mrlyrs::num::ladder::ROUNDING;
    Ok(value)
}

/// The largest digit span a rule may read, so its code fits a `u64`.
#[wasm_bindgen]
pub fn memory_SPAN() -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::SPAN;
    Ok(value)
}

/// The sweep cap of the power iteration.
#[wasm_bindgen]
pub fn memory_SWEEPS() -> Result<usize, JsValue> {
    let value = mrlyrs::num::memory::SWEEPS;
    Ok(value)
}

/// The absolute `l^1` move of the normalised iterate that stops the power iteration, counted only when it holds over three consecutive sweeps.
#[wasm_bindgen]
pub fn memory_TOLERANCE() -> Result<f64, JsValue> {
    let value = mrlyrs::num::memory::TOLERANCE;
    Ok(value)
}

/// Lists the lifts in the order the gallery draws them.
#[wasm_bindgen]
pub fn morse_LIFTS() -> Result<JsValue, JsValue> {
    let value = mrlyrs::num::morse::LIFTS;
    hand::to_js(&value)
}

/// The Apery constant, the value zeta takes at three.
#[wasm_bindgen]
pub fn series_APERY() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::APERY;
    Ok(value)
}

/// The Basel constant, pi squared over six, the value zeta takes at two.
#[wasm_bindgen]
pub fn series_BASEL() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::BASEL;
    Ok(value)
}

/// The Catalan constant, the value the Dirichlet beta function takes at two.
#[wasm_bindgen]
pub fn series_CATALAN() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::CATALAN;
    Ok(value)
}

/// The Euler constant, the limit of the harmonic sum less the logarithm.
#[wasm_bindgen]
pub fn series_EULER() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::EULER;
    Ok(value)
}

/// The visible density, six over pi squared, the share of lattice pairs that are coprime.
#[wasm_bindgen]
pub fn series_VISIBLE() -> Result<f64, JsValue> {
    let value = mrlyrs::num::series::VISIBLE;
    Ok(value)
}

/// The limit of the plane Wallis sieve's surviving area, pi over four.
#[wasm_bindgen]
pub fn sieve_PLANE_LIMIT() -> Result<f64, JsValue> {
    let value = mrlyrs::num::sieve::PLANE_LIMIT;
    Ok(value)
}

/// The t where the walk hands over from Euler-Maclaurin to Riemann-Siegel.
#[wasm_bindgen]
pub fn zeta_JOIN() -> Result<f64, JsValue> {
    let value = mrlyrs::num::zeta::JOIN;
    Ok(value)
}

/// A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing.
#[wasm_bindgen]
pub struct apollonian_Circle {
    inner: mrlyrs::num::apollonian::Circle,
}

#[wasm_bindgen]
impl apollonian_Circle {
    /// Reads the Circle from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<apollonian_Circle, JsValue> {
        Ok(apollonian_Circle {
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

/// A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from.
#[wasm_bindgen]
pub struct automaton_Automaton {
    inner: mrlyrs::num::automaton::Automaton,
}

#[wasm_bindgen]
impl automaton_Automaton {
    /// Reads the Automaton from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<automaton_Automaton, JsValue> {
        Ok(automaton_Automaton {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Automaton as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`].
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    /// Returns the base `q = 2^D`.
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    /// Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.
    pub fn cofactor(&self, s: &zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self
            .inner
            .cofactor(s.inner, tolerance)
            .map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
    /// Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.
    pub fn denominator(&self) -> Result<Vec<f64>, JsValue> {
        let value = self.inner.denominator();
        Ok(value)
    }
    /// Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`.
    pub fn matrix(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.matrix();
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Builds the ladder of a rule, choosing the peel depth.
    #[wasm_bindgen(constructor)]
    pub fn new(rule: &memory_Rule) -> Result<automaton_Automaton, JsValue> {
        let value = mrlyrs::num::automaton::Automaton::new(&rule.inner).map_err(hand::throw)?;
        Ok(automaton_Automaton { inner: value })
    }
    /// Returns the peel depth `P`.
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    /// Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.
    pub fn perron(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.perron();
        hand::to_js(&value)
    }
    /// Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.
    pub fn residue(&self, w0: &zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self
            .inner
            .residue(w0.inner, tolerance)
            .map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
    /// Returns the rule.
    pub fn rule(&self) -> Result<memory_Rule, JsValue> {
        let value = self.inner.rule();
        Ok(memory_Rule { inner: value })
    }
    /// Returns the state count `q^(k-1)`.
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    /// Builds the ladder at an explicit peel depth, at least the rule width and at least two.
    pub fn with_peel(rule: &memory_Rule, peel: usize) -> Result<automaton_Automaton, JsValue> {
        let value =
            mrlyrs::num::automaton::Automaton::with_peel(&rule.inner, peel).map_err(hand::throw)?;
        Ok(automaton_Automaton { inner: value })
    }
    /// Returns `zeta_W(s)` and the bound it is known to.
    pub fn zeta(&self, s: &zeta_Complex, tolerance: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.zeta(s.inner, tolerance).map_err(hand::throw)?;
        Ok(hand::tuple_to_js(&[
            JsValue::from(zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
    }
}

/// The symmetric window of one ring: every point within a reach, with the norms sieved once.
#[wasm_bindgen]
pub struct gauss_Window {
    inner: mrlyrs::num::gauss::Window,
}

#[wasm_bindgen]
impl gauss_Window {
    /// Reads the Window from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<gauss_Window, JsValue> {
        Ok(gauss_Window {
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
    pub fn new(ring: JsValue, radius: JsValue) -> Result<gauss_Window, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = mrlyrs::num::gauss::Window::new(ring, radius);
        Ok(gauss_Window { inner: value })
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

/// A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at.
#[wasm_bindgen]
pub struct ladder_Design {
    inner: mrlyrs::num::ladder::Design,
}

#[wasm_bindgen]
impl ladder_Design {
    /// Reads the Design from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<ladder_Design, JsValue> {
        Ok(ladder_Design {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Design as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the abscissa `alpha = log_q k`.
    pub fn abscissa(&self) -> Result<f64, JsValue> {
        let value = self.inner.abscissa();
        Ok(value)
    }
    /// Returns the base.
    pub fn base(&self) -> Result<u64, JsValue> {
        let value = self.inner.base();
        Ok(value)
    }
    /// Returns the digit set, ascending.
    pub fn digits(&self) -> Result<Vec<u64>, JsValue> {
        let value = self.inner.digits();
        Ok(value.to_vec())
    }
    /// Builds a design on the base and the digit set, choosing the peel depth.
    #[wasm_bindgen(constructor)]
    pub fn new(base: JsValue, digits: JsValue) -> Result<ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value = mrlyrs::num::ladder::Design::new(base, &digits).map_err(hand::throw)?;
        Ok(ladder_Design { inner: value })
    }
    /// Returns the peel depth.
    pub fn peel(&self) -> Result<usize, JsValue> {
        let value = self.inner.peel();
        Ok(value)
    }
    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> Result<f64, JsValue> {
        let value = self.inner.period();
        Ok(value)
    }
    /// Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.
    pub fn pole(&self, m: usize, j: JsValue) -> Result<zeta_Complex, JsValue> {
        let j = hand::i64_from_js(&j)?;
        let value = self.inner.pole(m, j);
        Ok(zeta_Complex { inner: value })
    }
    /// Builds a design at an explicit peel depth, at least two.
    pub fn with_peel(
        base: JsValue,
        digits: JsValue,
        peel: usize,
    ) -> Result<ladder_Design, JsValue> {
        let base = hand::u64_from_js(&base)?;
        let digits = hand::list_from_js(&digits, hand::u64_from_js)?;
        let value =
            mrlyrs::num::ladder::Design::with_peel(base, &digits, peel).map_err(hand::throw)?;
        Ok(ladder_Design { inner: value })
    }
}

/// A rule on `k` consecutive digits of a design word.
#[wasm_bindgen]
pub struct memory_Rule {
    inner: mrlyrs::num::memory::Rule,
}

#[wasm_bindgen]
impl memory_Rule {
    /// Reads the Rule from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<memory_Rule, JsValue> {
        Ok(memory_Rule {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Rule as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The dimension `D`, one to three.
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
    /// The window width `k`, at least one.
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
    /// The window code, bit `w` set when window `w` is allowed.
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
    /// Returns whether a word, coarsest digit first, is accepted.
    pub fn accepts(&self, word: &[usize]) -> Result<bool, JsValue> {
        let value = self.inner.accepts(word);
        Ok(value)
    }
    /// Returns whether the window is allowed, and false for any window out of range.
    pub fn allowed(&self, window: usize) -> Result<bool, JsValue> {
        let value = self.inner.allowed(window);
        Ok(value)
    }
    /// Returns the letters that stand in at least one allowed window.
    pub fn alphabet(&self) -> Result<Vec<usize>, JsValue> {
        let value = self.inner.alphabet();
        Ok(value)
    }
    /// Returns the count of rules of this shape, `2^(2^(k D))`.
    pub fn codes(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.codes();
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Returns the rule that allows every window.
    pub fn full(dimension: usize, width: usize) -> Result<memory_Rule, JsValue> {
        let value = mrlyrs::num::memory::Rule::full(dimension, width).map_err(hand::throw)?;
        Ok(memory_Rule { inner: value })
    }
    /// Returns the letter count `2^D`, the digit vectors of the cube's corners.
    pub fn letters(&self) -> Result<usize, JsValue> {
        let value = self.inner.letters();
        Ok(value)
    }
    /// Builds a rule from its dimension, its width and its code.
    #[wasm_bindgen(constructor)]
    pub fn new(dimension: usize, width: usize, code: JsValue) -> Result<memory_Rule, JsValue> {
        let code = hand::u64_from_js(&code)?;
        let value = mrlyrs::num::memory::Rule::new(dimension, width, code).map_err(hand::throw)?;
        Ok(memory_Rule { inner: value })
    }
    /// Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.
    pub fn states(&self) -> Result<usize, JsValue> {
        let value = self.inner.states();
        Ok(value)
    }
    /// Returns the window count `2^(k D)`.
    pub fn windows(&self) -> Result<usize, JsValue> {
        let value = self.inner.windows();
        Ok(value)
    }
}

/// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
#[wasm_bindgen]
pub struct prime_Sieve {
    inner: mrlyrs::num::prime::Sieve,
}

#[wasm_bindgen]
impl prime_Sieve {
    /// Reads the Sieve from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<prime_Sieve, JsValue> {
        Ok(prime_Sieve {
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
    pub fn new(limit: usize) -> Result<prime_Sieve, JsValue> {
        let value = mrlyrs::num::prime::Sieve::new(limit);
        Ok(prime_Sieve { inner: value })
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

/// The base of a radix design: a ring and an element of norm at least two, the scale every word is read against.
#[wasm_bindgen]
pub struct radix_Base {
    inner: mrlyrs::num::radix::Base,
}

#[wasm_bindgen]
impl radix_Base {
    /// Reads the Base from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<radix_Base, JsValue> {
        Ok(radix_Base {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Base as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the index in the canonical residue system of the class of a point.
    pub fn class(&self, z: JsValue) -> Result<usize, JsValue> {
        let z = (
            hand::i64_from_js(&hand::item(&z, 0)?)?,
            hand::i64_from_js(&hand::item(&z, 1)?)?,
        );
        let value = self.inner.class(z).map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns whether two points are congruent modulo the base.
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
    /// Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.
    pub fn group(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.group().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| Ok(hand::typed(&(*x1)[..])))
    }
    /// Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.
    pub fn mirrored(&self) -> Result<bool, JsValue> {
        let value = self.inner.mirrored();
        Ok(value)
    }
    /// Fixes a base in a ring.
    #[wasm_bindgen(constructor)]
    pub fn new(ring: JsValue, value: JsValue) -> Result<radix_Base, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = (
            hand::i64_from_js(&hand::item(&value, 0)?)?,
            hand::i64_from_js(&hand::item(&value, 1)?)?,
        );
        let value = mrlyrs::num::radix::Base::new(ring, value).map_err(hand::throw)?;
        Ok(radix_Base { inner: value })
    }
    /// Returns the norm `q` of the base: the count of residue classes and the square of the scale.
    pub fn norm(&self) -> Result<u64, JsValue> {
        let value = self.inner.norm();
        Ok(value)
    }
    /// Returns the base raised to a level.
    pub fn power(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.power(level);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.
    pub fn residues(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.residues().map_err(hand::throw)?;
        hand::list_to_js(&value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the ring.
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    /// Returns the base element.
    pub fn value(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.value();
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
}

/// A radix design: a digit set inside one ring, placed by a base with a unit twist per digit.
#[wasm_bindgen]
pub struct radix_Radix {
    inner: mrlyrs::num::radix::Radix,
}

#[wasm_bindgen]
impl radix_Radix {
    /// Reads the Radix from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<radix_Radix, JsValue> {
        Ok(radix_Radix {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Radix as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Returns the base.
    pub fn base(&self) -> Result<radix_Base, JsValue> {
        let value = self.inner.base();
        Ok(radix_Base { inner: value })
    }
    /// Returns whether every digit is the canonical representative of its class.
    pub fn canonical(&self) -> Result<bool, JsValue> {
        let value = self.inner.canonical().map_err(hand::throw)?;
        Ok(value)
    }
    /// Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.
    pub fn code(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.code().map_err(hand::throw)?;
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Returns the digits.
    pub fn digits(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.digits();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.
    pub fn dimension(&self) -> Result<f64, JsValue> {
        let value = self.inner.dimension();
        Ok(value)
    }
    /// Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.
    pub fn distinct(&self, level: usize) -> Result<usize, JsValue> {
        let value = self.inner.distinct(level);
        Ok(value)
    }
    /// Returns the count of words of a level, `|F|^L`.
    pub fn fill(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.fill(level);
        Ok(JsValue::from_str(&value.to_string()))
    }
    /// Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.
    pub fn from_code(base: &radix_Base, code: JsValue) -> Result<radix_Radix, JsValue> {
        let code = hand::u128_from_js(&code)?;
        let value = mrlyrs::num::radix::Radix::from_code(base.inner, code).map_err(hand::throw)?;
        Ok(radix_Radix { inner: value })
    }
    /// Builds a design from a base, a digit list and a unit twist per digit.
    #[wasm_bindgen(constructor)]
    pub fn new(
        base: &radix_Base,
        digits: JsValue,
        twists: JsValue,
    ) -> Result<radix_Radix, JsValue> {
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
            mrlyrs::num::radix::Radix::new(base.inner, digits, twists).map_err(hand::throw)?;
        Ok(radix_Radix { inner: value })
    }
    /// Returns the level-`L` points in the plane, the scaled words divided by `b^L`.
    pub fn plane(&self, level: usize) -> Result<JsValue, JsValue> {
        let value = self.inner.plane(level);
        hand::to_js(&value)
    }
    /// Returns the ring.
    pub fn ring(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.ring();
        hand::to_js(&value)
    }
    /// Returns the digit count `|F|`.
    pub fn size(&self) -> Result<usize, JsValue> {
        let value = self.inner.size();
        Ok(value)
    }
    /// Returns the twists.
    pub fn twists(&self) -> Result<JsValue, JsValue> {
        let value = self.inner.twists();
        hand::list_to_js(value, |x1| {
            Ok(hand::tuple_to_js(&[
                JsValue::from(x1.0),
                JsValue::from(x1.1),
            ]))
        })
    }
    /// Returns the design with the twists named by their index in the unit list, the units in turning order from one.
    pub fn with_twists(&self, units: &[usize]) -> Result<radix_Radix, JsValue> {
        let value = self.inner.clone().with_twists(units).map_err(hand::throw)?;
        Ok(radix_Radix { inner: value })
    }
    /// Returns the level-`L` points in exact ring coordinates scaled by `b^L`.
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

/// A complex number: a real and an imaginary part.
#[wasm_bindgen]
pub struct zeta_Complex {
    inner: mrlyrs::num::zeta::Complex,
}

#[wasm_bindgen]
impl zeta_Complex {
    /// Reads the Complex from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<zeta_Complex, JsValue> {
        Ok(zeta_Complex {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Complex as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// The real part.
    #[wasm_bindgen(getter)]
    pub fn re(&self) -> Result<f64, JsValue> {
        let value = self.inner.re;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_re(&mut self, value: f64) -> Result<(), JsValue> {
        self.inner.re = value;
        Ok(())
    }
    /// The imaginary part.
    #[wasm_bindgen(getter)]
    pub fn im(&self) -> Result<f64, JsValue> {
        let value = self.inner.im;
        Ok(value)
    }
    #[wasm_bindgen(setter)]
    pub fn set_im(&mut self, value: f64) -> Result<(), JsValue> {
        self.inner.im = value;
        Ok(())
    }
    /// Returns the modulus.
    pub fn abs(&self) -> Result<f64, JsValue> {
        let value = self.inner.abs();
        Ok(value)
    }
    /// Returns the principal argument.
    pub fn arg(&self) -> Result<f64, JsValue> {
        let value = self.inner.arg();
        Ok(value)
    }
    /// Returns the default Complex.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::default();
        Ok(zeta_Complex { inner: value })
    }
    /// Returns the exponential.
    pub fn exp(&self) -> Result<zeta_Complex, JsValue> {
        let value = self.inner.exp();
        Ok(zeta_Complex { inner: value })
    }
    /// Returns the principal logarithm.
    pub fn ln(&self) -> Result<zeta_Complex, JsValue> {
        let value = self.inner.ln();
        Ok(zeta_Complex { inner: value })
    }
    /// Builds a complex number from its parts.
    #[wasm_bindgen(constructor)]
    pub fn new(re: f64, im: f64) -> Result<zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::new(re, im);
        Ok(zeta_Complex { inner: value })
    }
    /// Returns a unit complex number at the given angle.
    pub fn turn(angle: f64) -> Result<zeta_Complex, JsValue> {
        let value = mrlyrs::num::zeta::Complex::turn(angle);
        Ok(zeta_Complex { inner: value })
    }
}

/// The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once.
#[wasm_bindgen]
pub struct zeta_Line {
    inner: mrlyrs::num::zeta::Line,
}

#[wasm_bindgen]
impl zeta_Line {
    /// Reads the Line from its plain data.
    #[wasm_bindgen(js_name = "from")]
    pub fn from_plain(data: JsValue) -> Result<zeta_Line, JsValue> {
        Ok(zeta_Line {
            inner: hand::from_js(&data)?,
        })
    }
    /// Writes the Line as plain data.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_plain(&self) -> Result<JsValue, JsValue> {
        hand::to_js(&self.inner)
    }
    /// Counts the zeros on the line below t.
    pub fn count(&self, t: f64) -> Result<usize, JsValue> {
        let value = self.inner.count(t);
        Ok(value)
    }
    /// Returns the default Line.
    #[wasm_bindgen(js_name = "default")]
    pub fn default_() -> Result<zeta_Line, JsValue> {
        let value = mrlyrs::num::zeta::Line::default();
        Ok(zeta_Line { inner: value })
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
    pub fn maclaurin(&self, t: f64) -> Result<zeta_Complex, JsValue> {
        let value = self.inner.maclaurin(t);
        Ok(zeta_Complex { inner: value })
    }
    /// Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<zeta_Line, JsValue> {
        let value = mrlyrs::num::zeta::Line::new();
        Ok(zeta_Line { inner: value })
    }
    /// Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.
    pub fn novelty_coefficients(&self, gammas: &[f64]) -> Result<JsValue, JsValue> {
        let value = self.inner.novelty_coefficients(gammas);
        hand::list_to_js(&value, |x1| Ok(JsValue::from(zeta_Complex { inner: *x1 })))
    }
    /// Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.
    pub fn pair(&self, s: &zeta_Complex) -> Result<JsValue, JsValue> {
        let value = self.inner.pair(s.inner);
        Ok(hand::tuple_to_js(&[
            JsValue::from(zeta_Complex { inner: value.0 }),
            JsValue::from(zeta_Complex { inner: value.1 }),
        ]))
    }
    /// Returns zeta on the line and Z(t) together, from the engine that serves the t.
    pub fn point(&self, t: f64) -> Result<JsValue, JsValue> {
        let value = self.inner.point(t);
        Ok(hand::tuple_to_js(&[
            JsValue::from(zeta_Complex { inner: value.0 }),
            hand::to_js(&value.1)?,
        ]))
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
pub struct gauss_Class {}

#[wasm_bindgen]
impl gauss_Class {
    /// Returns whether the class is prime.
    pub fn prime(class_: JsValue) -> Result<bool, JsValue> {
        let class_ = hand::from_js::<mrlyrs::num::gauss::Class>(&class_)?;
        let value = class_.prime();
        Ok(value)
    }
    /// Returns the class as a word.
    pub fn word(class_: JsValue) -> Result<String, JsValue> {
        let class_ = hand::from_js::<mrlyrs::num::gauss::Class>(&class_)?;
        let value = class_.word();
        Ok(value)
    }
}

/// The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice.
#[wasm_bindgen]
pub struct gauss_Ring {}

#[wasm_bindgen]
impl gauss_Ring {
    /// Returns the unit multiples of a point, the point first, turning anticlockwise.
    pub fn associates(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.count(radius);
        Ok(value)
    }
    /// Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.
    pub fn div_rem(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
            hand::tuple_to_js(&[JsValue::from(value.0 .0), JsValue::from(value.0 .1)]),
            hand::tuple_to_js(&[JsValue::from(value.1 .0), JsValue::from(value.1 .1)]),
        ]))
    }
    /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
    pub fn fate(ring: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let n = hand::u64_from_js(&n)?;
        let value = ring.fate(n);
        hand::to_js(&value)
    }
    /// Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.
    pub fn gaussian_gcd(ring: JsValue, z: JsValue, w: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let p = hand::u64_from_js(&p)?;
        let value = ring.inert(p);
        Ok(value)
    }
    /// Returns the product of two points.
    pub fn mul(ring: JsValue, arg1: JsValue, arg2: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let value = mrlyrs::num::gauss::Ring::named(name);
        hand::to_js(&value)
    }
    /// Returns the point nearest a place in the plane.
    pub fn nearest(ring: JsValue, x: f64, y: f64) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.nearest(x, y);
        Ok(hand::tuple_to_js(&[
            JsValue::from(value.0),
            JsValue::from(value.1),
        ]))
    }
    /// Returns the norm of a point: its squared length.
    pub fn norm(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.norm(a, b);
        Ok(value)
    }
    /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
    pub fn place(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.place(a, b);
        hand::to_js(&value)
    }
    /// Returns the one rational prime that ramifies: 2 or 3.
    pub fn ramified(ring: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.ramified();
        Ok(value)
    }
    /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
    pub fn reach(ring: JsValue, a: JsValue, b: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.reach(a, b);
        Ok(value)
    }
    /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
    pub fn symmetry(ring: JsValue) -> Result<usize, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.symmetry();
        Ok(value)
    }
    /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
    pub fn top(ring: JsValue, radius: JsValue) -> Result<u64, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let radius = hand::u64_from_js(&radius)?;
        let value = ring.top(radius);
        Ok(value)
    }
    /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
    pub fn turn(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
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
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let value = ring.units();
        Ok(value)
    }
    /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
    pub fn whole(ring: JsValue, a: JsValue, b: JsValue) -> Result<JsValue, JsValue> {
        let ring = hand::from_js::<mrlyrs::num::gauss::Ring>(&ring)?;
        let a = hand::i64_from_js(&a)?;
        let b = hand::i64_from_js(&b)?;
        let value = ring.whole(a, b);
        hand::option_to_js(value.as_ref(), |x1| Ok(JsValue::from(*x1)))
    }
}

/// The four ways the word lifts from a line to the plane, one sign at every site.
#[wasm_bindgen]
pub struct morse_Lift {}

#[wasm_bindgen]
impl morse_Lift {
    /// Returns every Lift in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::morse::Lift::all();
        hand::to_js(&value)
    }
    /// Returns the sign at a site, zero for plus one and one for minus one.
    pub fn at(lift: JsValue, i: JsValue, j: JsValue) -> Result<u8, JsValue> {
        let lift = hand::from_js::<mrlyrs::num::morse::Lift>(&lift)?;
        let i = hand::u64_from_js(&i)?;
        let j = hand::u64_from_js(&j)?;
        let value = lift.at(i, j);
        Ok(value)
    }
    /// Returns the lift's formula, written the way the page prints it.
    pub fn formula(lift: JsValue) -> Result<String, JsValue> {
        let lift = hand::from_js::<mrlyrs::num::morse::Lift>(&lift)?;
        let value = lift.formula();
        Ok(value)
    }
}

/// Which cells of the square winding grow into a tile.
#[wasm_bindgen]
pub struct spiral_Growth {}

#[wasm_bindgen]
impl spiral_Growth {
    /// Returns every Growth in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Growth::all();
        hand::to_js(&value)
    }
}

/// The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right.
#[wasm_bindgen]
pub struct spiral_Lattice {}

#[wasm_bindgen]
impl spiral_Lattice {
    /// Returns every Lattice in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Lattice::all();
        hand::to_js(&value)
    }
    /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
    pub fn count(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let value = lattice.count(side);
        Ok(value)
    }
    /// Returns the number at a cell, one at the origin.
    pub fn n(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.n(x, y);
        Ok(value)
    }
    /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
    pub fn radius(lattice: JsValue, side: usize) -> Result<usize, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let value = lattice.radius(side);
        Ok(value)
    }
    /// Returns the ring a number sits on, zero for one.
    pub fn ring(lattice: JsValue, n: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let n = hand::u64_from_js(&n)?;
        let value = lattice.ring(n);
        Ok(value)
    }
    /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
    pub fn ring_of(lattice: JsValue, x: JsValue, y: JsValue) -> Result<u64, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
        let x = hand::i64_from_js(&x)?;
        let y = hand::i64_from_js(&y)?;
        let value = lattice.ring_of(x, y);
        Ok(value)
    }
    /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
    pub fn xy(lattice: JsValue, n: JsValue) -> Result<JsValue, JsValue> {
        let lattice = hand::from_js::<mrlyrs::num::spiral::Lattice>(&lattice)?;
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
pub struct spiral_Mark {}

#[wasm_bindgen]
impl spiral_Mark {
    /// Returns every Mark in canonical order.
    pub fn all() -> Result<JsValue, JsValue> {
        let value = mrlyrs::num::spiral::Mark::all();
        hand::to_js(&value)
    }
}
