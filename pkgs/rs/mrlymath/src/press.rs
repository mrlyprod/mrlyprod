use crate::bang::factory::{code_to_corners, MagicLayer};
use crate::bang::universe::Code;
use mrlycore::errors::{value_error, Result};

/// The largest corner count the tally press accepts, keeping its table a million rows.
pub const CORNERS: usize = 20;

fn corner_count(dimension: usize, base: usize) -> usize {
    let count = base.pow(dimension as u32);
    assert!(count < 128, "too many corners for a u128 code");
    count
}

/// Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.
///
/// A number is read in base `base` to the power of the dimension, so each digit is one
/// residue corner of the design cube, and zero uses exactly the zero corner.
///
/// ```
/// assert_eq!(mrlymath::press::usage(0, 2, 2), 1);
/// assert_eq!(mrlymath::press::usage(6, 2, 2), 0b0110);
/// ```
pub fn usage(number: u128, dimension: usize, base: usize) -> Code {
    let radix = corner_count(dimension, base) as u128;
    if number == 0 {
        return 1;
    }
    let mut out: Code = 0;
    let mut rest = number;
    while rest > 0 {
        out |= 1 << (rest % radix);
        rest /= radix;
    }
    out
}

/// Returns whether every digit vector of the number lies in the design.
///
/// This is the scalar membership rule of the sequence press: the number's base
/// `base` to the dimension digits, read as residue corners, must all be filled.
/// At dimension one it is the classic restricted-digit set.
///
/// ```
/// let members: Vec<u128> = (0..30).filter(|&n| mrlymath::press::member(0b0111, n, 2, 2)).collect();
/// assert_eq!(members, vec![0, 1, 2, 4, 5, 6, 8, 9, 10, 16, 17, 18, 20, 21, 22, 24, 25, 26]);
/// ```
pub fn member(code: Code, number: u128, dimension: usize, base: usize) -> bool {
    usage(number, dimension, base) & !code == 0
}

/// Returns the count of distinct digit vectors the number uses.
pub fn distinct(number: u128, dimension: usize, base: usize) -> u32 {
    usage(number, dimension, base).count_ones()
}

/// Returns the number of designs of the dimension and base that contain the number.
///
/// A design contains the number exactly when it fills every used corner, so the count
/// is two to the free corners, and the average membership over all designs is one over
/// two to the distinct-vector count.
///
/// ```
/// assert_eq!(mrlymath::press::containing(6, 2, 2), 4);
/// ```
pub fn containing(number: u128, dimension: usize, base: usize) -> u128 {
    1 << (corner_count(dimension, base) as u32 - distinct(number, dimension, base))
}

/// Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.
///
/// ```
/// assert_eq!(mrlymath::press::coordinates(6, 2, 2), vec![1, 2]);
/// ```
pub fn coordinates(number: u128, dimension: usize, base: usize) -> Vec<u128> {
    let radix = corner_count(dimension, base) as u128;
    let mut out = vec![0u128; dimension];
    let mut rest = number;
    let mut place: u128 = 1;
    while rest > 0 {
        let mut corner = rest % radix;
        for axis in (0..dimension).rev() {
            out[axis] += (corner % base as u128) * place;
            corner /= base as u128;
        }
        rest /= radix;
        place *= base as u128;
    }
    out
}

/// Weaves dimension coordinates back into their single interleaved number.
///
/// Panics when the woven number passes a hundred and twenty-eight bits.
pub fn interleave(coords: &[u128], base: usize) -> u128 {
    assert!(!coords.is_empty(), "interleave needs at least one coordinate");
    let dimension = coords.len();
    let radix = corner_count(dimension, base) as u128;
    let mut digits = Vec::new();
    let mut rest: Vec<u128> = coords.to_vec();
    while rest.iter().any(|&c| c > 0) {
        let mut corner: u128 = 0;
        for value in rest.iter_mut() {
            corner = corner * base as u128 + *value % base as u128;
            *value /= base as u128;
        }
        digits.push(corner);
    }
    let mut out: u128 = 0;
    for &corner in digits.iter().rev() {
        out = out
            .checked_mul(radix)
            .and_then(|v| v.checked_add(corner))
            .expect("the woven number passes a hundred and twenty-eight bits");
    }
    out
}

/// Returns the first members of a design in ascending order.
///
/// Stops early where the next member would pass a hundred and twenty-eight bits.
///
/// ```
/// assert_eq!(mrlymath::press::members(0b10, 1, 2, 5), vec![1, 3, 7, 15, 31]);
/// ```
pub fn members(code: Code, dimension: usize, base: usize, count: usize) -> Vec<u128> {
    let radix = corner_count(dimension, base) as u128;
    let allowed: Vec<u128> = (0..radix).filter(|&i| (code >> i) & 1 == 1).collect();
    let mut out = Vec::with_capacity(count);
    if count == 0 || allowed.is_empty() {
        return out;
    }
    if allowed[0] == 0 {
        out.push(0);
    }
    let mut length = 1usize;
    while out.len() < count {
        let mut slots = vec![0usize; length];
        if allowed[0] == 0 {
            slots[0] = 1;
            if allowed.len() == 1 {
                return out;
            }
        }
        'level: loop {
            let mut value: u128 = 0;
            let mut fits = true;
            for &slot in &slots {
                match value.checked_mul(radix).and_then(|v| v.checked_add(allowed[slot])) {
                    Some(next) => value = next,
                    None => {
                        fits = false;
                        break;
                    }
                }
            }
            if !fits {
                return out;
            }
            out.push(value);
            if out.len() == count {
                return out;
            }
            for place in (0..length).rev() {
                slots[place] += 1;
                if slots[place] < allowed.len() {
                    continue 'level;
                }
                slots[place] = usize::from(place == 0 && allowed[0] == 0);
            }
            break;
        }
        length += 1;
        if radix.checked_pow(length as u32 - 1).is_none() {
            return out;
        }
    }
    out
}

/// Counts the members of a design below the limit.
///
/// ```
/// assert_eq!(mrlymath::press::count_below(0b0111, 2, 2, 27), 18);
/// ```
pub fn count_below(code: Code, dimension: usize, base: usize, limit: u128) -> u128 {
    let radix = corner_count(dimension, base) as u128;
    let allowed: Vec<u128> = (0..radix).filter(|&i| (code >> i) & 1 == 1).collect();
    if limit == 0 || allowed.is_empty() {
        return 0;
    }
    let mut digits = Vec::new();
    let mut rest = limit;
    while rest > 0 {
        digits.push(rest % radix);
        rest /= radix;
    }
    digits.reverse();
    let k = allowed.len() as u128;
    let lead = allowed.iter().filter(|&&v| v != 0).count() as u128;
    let mut total: u128 = if allowed[0] == 0 { 1 } else { 0 };
    let mut power: u128 = 1;
    for _ in 1..digits.len() {
        total += lead * power;
        power *= k;
    }
    for (place, &digit) in digits.iter().enumerate() {
        let below = allowed
            .iter()
            .filter(|&&v| v < digit && (place > 0 || v != 0))
            .count() as u128;
        let tail = digits.len() - place - 1;
        total += below * k.pow(tail as u32);
        if !allowed.contains(&digit) {
            break;
        }
    }
    total
}

/// The tally press: one pass over the integers weighs every design of a universe at once.
///
/// Each added number lands its weight in the bucket of its corner-usage mask, and a
/// design's total is the sum over the submasks of its code, so a single sweep prices
/// a Mertens sum, a member count or a prime count for all two to the corners designs.
pub struct Press {
    /// The design dimension of the universe.
    pub dimension: usize,
    /// The numeral base of the universe.
    pub base: usize,
    corners: usize,
    tallies: Vec<i128>,
}

impl Press {
    /// Builds an empty press over every design of the dimension and base.
    ///
    /// Panics past twenty corners, where the bucket table leaves a million rows.
    pub fn new(dimension: usize, base: usize) -> Press {
        let corners = corner_count(dimension, base);
        assert!(corners <= CORNERS, "the tally press holds at most twenty corners");
        Press {
            dimension,
            base,
            corners,
            tallies: vec![0; 1 << corners],
        }
    }
    /// Adds a weighted number to its usage bucket.
    pub fn add(&mut self, number: u128, weight: i128) {
        self.tallies[usage(number, self.dimension, self.base) as usize] += weight;
    }
    /// Returns the total weight the design at a code has collected.
    pub fn total(&self, code: Code) -> i128 {
        let mut sum = self.tallies[0];
        let mut sub = code;
        while sub != 0 {
            sum += self.tallies[sub as usize];
            sub = (sub - 1) & code;
        }
        sum
    }
    /// Returns every design's total in code order by one subset-sum transform.
    pub fn totals(&self) -> Vec<i128> {
        let mut out = self.tallies.clone();
        for bit in 0..self.corners {
            for mask in 0..out.len() {
                if mask >> bit & 1 == 1 {
                    out[mask] += out[mask ^ (1 << bit)];
                }
            }
        }
        out
    }
}

fn layer_radix(layer: &MagicLayer) -> u128 {
    (layer.number as u128).pow(layer.design.dimension as u32)
}

/// Returns the allowed digit table of one magic layer, one flag per cell of its tile.
///
/// A cell is allowed when its coordinate residues form a filled corner, which is the
/// tile the layer renders read as a digit alphabet.
pub fn layer_table(layer: &MagicLayer) -> Result<Vec<bool>> {
    let corners = code_to_corners(layer.design.code, layer.design.dimension, layer.design.base)?;
    let dimension = layer.design.dimension;
    let base = layer.design.base;
    let side = layer.number;
    let mut out = Vec::with_capacity(side.pow(dimension as u32));
    for cell in 0..side.pow(dimension as u32) {
        let mut rest = cell;
        let mut residue = vec![0u8; dimension];
        for axis in (0..dimension).rev() {
            residue[axis] = ((rest % side) % base) as u8;
            rest /= side;
        }
        out.push(corners.contains(&residue));
    }
    Ok(out)
}

fn word_tables(layers: &[MagicLayer]) -> Result<Vec<Vec<bool>>> {
    if layers.is_empty() {
        return value_error("a word needs at least one layer.");
    }
    let dimension = layers[0].design.dimension;
    if layers.iter().any(|l| l.design.dimension != dimension) {
        return value_error("all word layers must have the same dimension.");
    }
    layers.iter().map(layer_table).collect()
}

/// Counts the members of a magic word from its layer fills, without enumeration.
pub fn word_count(layers: &[MagicLayer]) -> Result<u128> {
    let tables = word_tables(layers)?;
    Ok(tables
        .iter()
        .map(|t| t.iter().filter(|&&b| b).count() as u128)
        .product())
}

/// Returns whether the number lies in the magic word's composed design.
///
/// The number is read in the word's mixed radix, one digit per layer with the first
/// layer most significant, and every digit must land on an allowed cell of its tile.
/// A number past the word's domain is an error.
pub fn word_member(layers: &[MagicLayer], number: u128) -> Result<bool> {
    let tables = word_tables(layers)?;
    let mut rest = number;
    let mut ok = true;
    for (layer, table) in layers.iter().zip(&tables).rev() {
        let radix = layer_radix(layer);
        ok &= table[(rest % radix) as usize];
        rest /= radix;
    }
    if rest > 0 {
        return value_error(format!("number {number} lies past the word's domain."));
    }
    Ok(ok)
}

/// Enumerates every member of the magic word in ascending order.
///
/// The member count is the product of the layer fills, so measure with `word_count`
/// before pressing a word too rich to hold.
pub fn word_members(layers: &[MagicLayer]) -> Result<Vec<u128>> {
    let tables = word_tables(layers)?;
    let alphabets: Vec<Vec<u128>> = tables
        .iter()
        .map(|t| {
            t.iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(i, _)| i as u128)
                .collect()
        })
        .collect();
    if alphabets.iter().any(|a| a.is_empty()) {
        return Ok(Vec::new());
    }
    let radixes: Vec<u128> = layers.iter().map(layer_radix).collect();
    let mut out = Vec::new();
    let mut slots = vec![0usize; layers.len()];
    loop {
        let mut value: u128 = 0;
        for (place, &slot) in slots.iter().enumerate() {
            value = value * radixes[place] + alphabets[place][slot];
        }
        out.push(value);
        let mut place = layers.len();
        loop {
            if place == 0 {
                return Ok(out);
            }
            place -= 1;
            slots[place] += 1;
            if slots[place] < alphabets[place].len() {
                break;
            }
            slots[place] = 0;
        }
    }
}

/// Returns the diagonal slice profile of a magic word by the substitution product.
///
/// The profile of a tile lists, per coordinate sum, its filled cells, and the profile
/// of a Kronecker word is the product of its layer profiles with strides, so no cell
/// of the composed design is ever enumerated.
pub fn word_profile(layers: &[MagicLayer]) -> Result<Vec<u128>> {
    let tables = word_tables(layers)?;
    let dimension = layers[0].design.dimension;
    let mut out = vec![1u128];
    let mut stride: usize = 1;
    for (layer, table) in layers.iter().zip(&tables).rev() {
        let side = layer.number;
        let mut profile = vec![0u128; dimension * (side - 1) + 1];
        for (cell, &filled) in table.iter().enumerate() {
            if filled {
                let mut rest = cell;
                let mut total = 0usize;
                for _ in 0..dimension {
                    total += rest % side;
                    rest /= side;
                }
                profile[total] += 1;
            }
        }
        let mut next = vec![0u128; (profile.len() - 1) * stride + out.len()];
        for (i, &a) in profile.iter().enumerate() {
            if a == 0 {
                continue;
            }
            for (j, &b) in out.iter().enumerate() {
                next[i * stride + j] += a * b;
            }
        }
        out = next;
        stride *= side;
    }
    Ok(out)
}

/// Returns the diagonal slice profile of one design pressed to a fractal level.
pub fn profile(code: Code, dimension: usize, base: usize, level: usize) -> Result<Vec<u128>> {
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let layer = MagicLayer::new(crate::name::Bang::new(code, dimension, base), base);
    word_profile(&vec![layer; level])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::factory::create;
    use crate::name::Bang;

    #[test]
    fn usage_of_zero_is_the_zero_corner() {
        assert_eq!(usage(0, 2, 2), 1);
        assert_eq!(usage(0, 1, 10), 1);
        assert_eq!(distinct(0, 2, 2), 1);
    }

    #[test]
    fn membership_matches_a_digit_check_at_base_ten() {
        let no_seven: Code = !(1 << 7) & ((1 << 10) - 1);
        for n in 0..10_000u128 {
            let digits_clean = !n.to_string().contains('7');
            assert_eq!(member(no_seven, n, 1, 10), digits_clean, "{n}");
        }
    }

    #[test]
    fn members_of_the_repunit_design_are_the_mersenne_numbers() {
        assert_eq!(members(0b10, 1, 2, 6), vec![1, 3, 7, 15, 31, 63]);
    }

    #[test]
    fn members_walk_ascending_and_agree_with_membership() {
        for code in [0b0111u128, 0b0110, 0b1001, 0b1111] {
            let list = members(code, 2, 2, 40);
            for pair in list.windows(2) {
                assert!(pair[0] < pair[1]);
            }
            let scanned: Vec<u128> = (0..200).filter(|&n| member(code, n, 2, 2)).collect();
            let shared = list.len().min(scanned.len());
            assert_eq!(list[..shared], scanned[..shared], "{code}");
        }
    }

    #[test]
    fn count_below_agrees_with_the_member_walk() {
        for code in [0b0111u128, 0b0110, 0b1011, 0b0001, 0b0000] {
            let list = members(code, 2, 2, 60);
            for limit in 0..300u128 {
                let walked = list.iter().filter(|&&m| m < limit).count() as u128;
                if list.len() < 60 || walked < 60 {
                    assert_eq!(count_below(code, 2, 2, limit), walked, "{code} {limit}");
                }
            }
        }
    }

    #[test]
    fn the_member_count_at_a_level_boundary_is_the_geometric_sum() {
        let code: Code = 0b0111;
        let k: u128 = 3;
        for level in 1..6u32 {
            let boundary = 4u128.pow(level);
            let full: u128 = 1 + (k - 1) * (k.pow(level) - 1) / (k - 1);
            assert_eq!(count_below(code, 2, 2, boundary), full);
        }
    }

    #[test]
    fn membership_matches_the_rendered_fractal() {
        for code in [7u128, 6, 9, 11] {
            let level = 3;
            let tile = create(code, 2, 2, 2, level).unwrap();
            let side = 1u128 << level;
            for n in 0..4u128.pow(level as u32) {
                let coords = coordinates(n, 2, 2);
                let flat = (coords[0] * side + coords[1]) as usize;
                let filled = tile.bytes()[flat] == 1;
                let padded = member(code, n, 2, 2)
                    && (code & 1 == 1 || n >= 4u128.pow(level as u32 - 1));
                assert_eq!(padded, filled, "{code} {n}");
            }
        }
    }

    #[test]
    fn coordinates_and_interleave_round_trip() {
        for n in 0..5_000u128 {
            assert_eq!(interleave(&coordinates(n, 2, 2), 2), n);
            assert_eq!(interleave(&coordinates(n, 3, 2), 2), n);
            assert_eq!(interleave(&coordinates(n, 2, 3), 3), n);
        }
    }

    #[test]
    fn containing_counts_the_designs_that_hold_the_number() {
        for n in 0..500u128 {
            let direct = (0..16u128).filter(|&code| member(code, n, 2, 2)).count();
            assert_eq!(containing(n, 2, 2), direct as u128, "{n}");
        }
    }

    #[test]
    fn the_membership_average_over_all_designs_is_two_to_minus_distinct() {
        for n in 0..2_000u128 {
            assert_eq!(containing(n, 2, 2), 1 << (4 - distinct(n, 2, 2)));
            assert_eq!(containing(n, 3, 2), 1 << (8 - distinct(n, 3, 2)));
        }
    }

    #[test]
    fn the_press_totals_agree_with_direct_member_sums() {
        let mut press = Press::new(2, 2);
        let weights: Vec<i128> = (0..600).map(|n| (n as i128 % 7) - 3).collect();
        for (n, &w) in weights.iter().enumerate() {
            press.add(n as u128, w);
        }
        let totals = press.totals();
        for code in 0..16u128 {
            let direct: i128 = weights
                .iter()
                .enumerate()
                .filter(|(n, _)| member(code, *n as u128, 2, 2))
                .map(|(_, &w)| w)
                .sum();
            assert_eq!(press.total(code), direct, "{code}");
            assert_eq!(totals[code as usize], direct, "{code}");
        }
    }

    #[test]
    fn a_native_word_is_the_stationary_press() {
        let layer = MagicLayer::new(Bang::new(7, 2, 2), 2);
        let word = vec![layer; 3];
        for n in 0..64u128 {
            let padded = member(7, n, 2, 2);
            assert_eq!(word_member(&word, n).unwrap(), padded, "{n}");
        }
        assert_eq!(word_count(&word).unwrap(), 27);
    }

    #[test]
    fn word_members_match_the_magic_tensor() {
        let word = [
            MagicLayer::new(Bang::new(7, 2, 2), 3),
            MagicLayer::new(Bang::new(14, 2, 2), 5),
        ];
        let tensor = crate::bang::factory::magic(&word).unwrap();
        let side = 15u128;
        let list = word_members(&word).unwrap();
        assert_eq!(list.len() as u128, word_count(&word).unwrap());
        for n in 0..word.iter().map(layer_radix).product::<u128>() {
            let mut rest = n;
            let mut x = 0u128;
            let mut y = 0u128;
            let mut place = 1u128;
            for layer in word.iter().rev() {
                let cell = rest % layer_radix(layer);
                rest /= layer_radix(layer);
                let s = layer.number as u128;
                x += (cell / s) * place;
                y += (cell % s) * place;
                place *= s;
            }
            let filled = tensor.bytes()[(x * side + y) as usize] == 1;
            assert_eq!(word_member(&word, n).unwrap(), filled, "{n}");
            assert_eq!(list.contains(&n), filled, "{n}");
        }
    }

    #[test]
    fn word_profiles_match_the_rendered_diagonal_sums() {
        let word = [
            MagicLayer::new(Bang::new(7, 2, 2), 3),
            MagicLayer::new(Bang::new(14, 2, 2), 5),
        ];
        let tensor = crate::bang::factory::magic(&word).unwrap();
        let side = 15usize;
        let mut direct = vec![0u128; 2 * side - 1];
        for (flat, &b) in tensor.bytes().iter().enumerate() {
            if b == 1 {
                direct[flat / side + flat % side] += 1;
            }
        }
        assert_eq!(word_profile(&word).unwrap(), direct);
    }

    #[test]
    fn profile_totals_are_the_fill_powers() {
        let native = profile(7, 2, 2, 4).unwrap();
        assert_eq!(native.iter().sum::<u128>(), 3u128.pow(4));
        let sponge = vec![MagicLayer::new(Bang::new(23, 3, 2), 3); 3];
        let classic = word_profile(&sponge).unwrap();
        assert_eq!(classic.iter().sum::<u128>(), 20u128.pow(3));
    }

    #[test]
    #[should_panic(expected = "the tally press holds at most twenty corners")]
    fn the_press_refuses_a_universe_past_twenty_corners() {
        let _ = Press::new(5, 2);
    }

    #[test]
    fn a_word_refuses_mismatched_dimensions_and_numbers_past_its_domain() {
        let plane = MagicLayer::new(Bang::new(7, 2, 2), 3);
        let cube = MagicLayer::new(Bang::new(23, 3, 2), 3);
        assert!(word_member(&[plane, cube], 0).is_err());
        assert!(word_member(&[plane], 9).is_err());
        assert!(word_member(&[], 0).is_err());
    }
}
