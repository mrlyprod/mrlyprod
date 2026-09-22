//! The instruments of number: primes, divisors, series, spectra, lattices and the designs the digits draw.
//!
//! Plain numbers and byte grids go in; counts, fractions, rates and measurements come out.
//! Every answer is exact where the integers allow and a stated approximation where they do not.
//!
//! # Files
//!
//! - `apollonian`: an integral circle packing, the Ford circles on its line, the Farey stack beneath.
//! - `automaton`: the Dirichlet series of a memory design, continued through its transfer matrix.
//! - `blend`: term ops on sequences, the exact recurrence behind one, its growth rate.
//! - `boolean`: a truth table's Walsh spectrum, nonlinearity, balance and avalanche.
//! - `design`: the digit designs on the line, their Mobius meter and the ordinates it carries.
//! - `factor`: factorizations, divisors, totients, radicals, Mobius values, gcd and lcm.
//! - `fft`: the fast Fourier transform in one and two dimensions.
//! - `gauss`: the Gaussian and the Eisenstein integers, their classes, windows and shells.
//! - `ladder`: the Dirichlet series of a digit design, continued to the plane, each value with its bound.
//! - `lattice`: coprime pairs, the Farey nodes of a window, the constant a dimension recovers.
//! - `memory`: a rule on consecutive digits, its transfer matrix, its words, its Perron root.
//! - `morse`: the Thue-Morse world: the digit rule, the substitution, the lifts and the runs.
//! - `prime`: the sieve and its readings, ranks, gaps, counts and the shapes a number makes.
//! - `radix`: a digit set inside the residues of a base in a ring, and where its words land.
//! - `series`: the classic sequences, zeta and its cousins, the partials walking to pi, e and gamma.
//! - `sieve`: the Wallis sieve and its kin as schedule words, with their rasters and limits.
//! - `spiral`: the whole numbers wound on the square and the hexagonal lattice, marked and read.
//! - `zeta`: zeta on the critical line, its zeros, the prime staircase they rebuild.
//!
//! # Doors
//!
//! - The divisor arithmetic: [`gcd`](crate::num::factor::gcd), [`factorial`](crate::num::factor::factorial), [`divisors`](crate::num::factor::divisors), [`mobius`](crate::num::factor::mobius).
//! - The primality test: [`is_prime`](crate::num::prime::is_prime).
//! - The zeta value above one: [`zeta`](crate::num::series::zeta).

/// The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.
pub mod apollonian;
/// The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.
///
/// A claim witness with no caller in the crate: it witnesses `research/claims/memory-dial.md`.
pub mod automaton;
/// The sequence blender: term ops, exact recurrences and growth rates.
pub mod blend;
/// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
pub mod boolean;
/// The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.
pub mod design;
/// The divisor arithmetic: factorizations, divisors, totients, radicals, the Mobius values and the exact whole-number arithmetic under them.
pub mod factor;
/// The fast Fourier transform in one and two dimensions.
pub mod fft;
/// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
pub mod gauss;
/// The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.
///
/// A claim witness with no caller in the crate: it witnesses `research/claims/zeros-of-the-design-zeta.md`.
pub mod ladder;
/// The visible lattice: coprime pairs, the constant a dimension recovers and the Farey nodes.
pub mod lattice;
/// The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.
pub mod memory;
/// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
pub mod morse;
/// The prime objects: the sieve and its readings, values, ranks, gaps, the counts they make and the shape readings of a number.
pub mod prime;
/// The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.
pub mod radix;
/// The infinite sums: the classic sequences, zeta and its Dirichlet cousins, the partials that walk to pi, e and gamma, the visible count and the Bernoulli fractions.
pub mod series;
/// The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.
pub mod sieve;
/// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
pub mod spiral;
/// The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.
pub mod zeta;

#[cfg(test)]
mod tests {
    use super::gauss::Ring;
    use super::lattice::farey;
    use super::memory::Rule;
    use super::prime::study;
    use super::zeta::Complex;

    #[test]
    fn serde_round_trips() {
        let prime = study(10)[2];
        assert_eq!(
            prime,
            serde_json::from_str(&serde_json::to_string(&prime).unwrap()).unwrap()
        );
        let node = farey(3)[1];
        assert_eq!(
            node,
            serde_json::from_str(&serde_json::to_string(&node).unwrap()).unwrap()
        );
        let rule = Rule::new(1, 2, 7).unwrap();
        assert_eq!(
            rule,
            serde_json::from_str(&serde_json::to_string(&rule).unwrap()).unwrap()
        );
        let point = Complex::new(0.5, 14.134_725);
        assert_eq!(
            point,
            serde_json::from_str(&serde_json::to_string(&point).unwrap()).unwrap()
        );
        assert_eq!(
            Ring::Eisenstein,
            serde_json::from_str(&serde_json::to_string(&Ring::Eisenstein).unwrap()).unwrap()
        );
    }
}
