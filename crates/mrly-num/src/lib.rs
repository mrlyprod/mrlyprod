//! The integers: primes, divisors, series, sieves, lattices and spectra.
//!
//! Plain numbers and byte grids go in; counts, fractions, rates and measurements come out.
//! Every answer is exact where the integers allow and a stated approximation where they do not.
//!
//! # Files
//!
//! - `apollonian`: an integral circle packing, the Ford circles on its line, the Farey stack beneath.
//! - `blend`: term ops on sequences, the exact recurrence behind one, its growth rate.
//! - `boolean`: a truth table's Walsh spectrum, nonlinearity, balance and avalanche.
//! - `chladni`: the figures of a ringing square plate, their ladder by pitch, the sand that runs to their lines and the lines themselves.
//! - `factor`: factorizations, divisors, totients, radicals, Mobius values, gcd and lcm.
//! - `fft`: the fast Fourier transform in one and two dimensions.
//! - `gauss`: the Gaussian and the Eisenstein integers, their classes, windows and shells.
//! - `kummer`: the Kummer sets `{k : p does not divide C(a k, b k)}` read by their carry automaton, their mass and the witness that they are no digit design.
//! - `lattice`: coprime pairs, the Farey nodes of a window, the constant a dimension recovers.
//! - `morse`: the Thue-Morse world: the digit rule, the substitution, the lifts and the runs.
//! - `prime`: the sieve and its readings, ranks, gaps, counts and the shapes a number makes.
//! - `series`: the classic sequences, zeta and its cousins, the partials walking to pi, e and gamma.
//! - `sieve`: the Wallis sieve and its kin as schedule words, with their rasters and limits.
//! - `spiral`: the whole numbers wound on the square and the hexagonal lattice, marked and read.
//! - `zeta`: zeta on the critical line, its zeros, the prime staircase they rebuild.
//!
//! # Doors
//!
//! - The divisor arithmetic: [`gcd`](crate::factor::gcd), [`factorial`](crate::factor::factorial), [`divisors`](crate::factor::divisors), [`mobius`](crate::factor::mobius).
//! - The primality test: [`is_prime`](crate::prime::is_prime).
//! - The zeta value above one: [`zeta`](crate::series::zeta).

/// The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.
pub mod apollonian;
/// The sequence blender: term ops, exact recurrences and growth rates.
pub mod blend;
/// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
pub mod boolean;
/// The Chladni plate: the figures of a free square plate, their ladder by pitch, sand shaken onto a raster's still lines and those lines traced.
pub mod chladni;
/// The divisor arithmetic: factorizations, divisors, totients, radicals, the Mobius values and the exact whole-number arithmetic under them.
pub mod factor;
/// The fast Fourier transform in one and two dimensions.
pub mod fft;
/// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
pub mod gauss;
/// The Kummer sets: the `k` with a prime `p` not dividing `C(a k, b k)`, read digit by digit from the units by the carry automaton of Kummer's theorem, counted below `p^level` and set against Legendre's valuation.
pub mod kummer;
/// The visible lattice: coprime pairs, the constant a dimension recovers and the Farey nodes.
pub mod lattice;
/// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
pub mod morse;
/// The prime objects: the sieve and its readings, values, ranks, gaps, the counts they make and the shape readings of a number.
pub mod prime;
/// The infinite sums: the classic sequences, zeta and its Dirichlet cousins, the partials that walk to pi, e and gamma, the visible count and the Bernoulli fractions.
pub mod series;
/// The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.
pub mod sieve;
/// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
pub mod spiral;
/// The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.
pub mod zeta;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct Readme;

#[cfg(test)]
mod tests {
    use super::chladni::Rung;
    use super::gauss::Ring;
    use super::lattice::farey;
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
        let point = Complex::new(0.5, 14.134_725);
        assert_eq!(
            point,
            serde_json::from_str(&serde_json::to_string(&point).unwrap()).unwrap()
        );
        let rung = Rung {
            n: 3,
            m: 5,
            sign: -1,
            pitch: 34f64.sqrt(),
        };
        assert_eq!(
            rung,
            serde_json::from_str(&serde_json::to_string(&rung).unwrap()).unwrap()
        );
        assert_eq!(
            Ring::Eisenstein,
            serde_json::from_str(&serde_json::to_string(&Ring::Eisenstein).unwrap()).unwrap()
        );
    }
}
