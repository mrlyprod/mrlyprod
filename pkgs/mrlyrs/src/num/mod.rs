#![doc = include_str!("README.md")]

/// The Apollonian gasket: a packing grown from its root quadruple in exact integers, the Ford circles it rests on the line, and the Farey stack they shadow.
pub mod apollonian;
/// The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.
pub mod automaton;
/// The sequence blender: term ops, exact recurrences and growth rates.
pub mod blend;
/// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
pub mod boolean;
/// The classic sequences and the exact arithmetic under them.
pub mod classics;
/// The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.
pub mod design;
/// The divisor arithmetic: factorizations, divisors, radicals and the Mobius values.
pub mod factor;
/// The fast Fourier transform in one and two dimensions.
pub mod fft;
/// The elementary formulas: the partial products and sums that walk to pi, e and gamma, and the prime counts beside them.
pub mod formulas;
/// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
pub mod gauss;
/// The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.
pub mod ladder;
/// The visible lattice: totients, coprime pairs, the constant a dimension recovers and the Farey nodes.
pub mod lattice;
/// The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.
pub mod memory;
/// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
pub mod morse;
/// The prime objects: values, ranks, gaps and the shape readings of a number.
pub mod prime;
/// The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.
pub mod radix;
/// The infinite sums: zeta and its Dirichlet cousins, the visible count and the Bernoulli fractions.
pub mod series;
/// The punctured schedules: the Wallis sieve and its kin, their words, rasters, punctures and limits.
pub mod sieve;
/// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
pub mod spiral;
/// The critical line: zeta at one half plus i t and off it, its zeros, the prime staircase they rebuild and the novelty meter their waves predict.
pub mod zeta;
