//! The designs in space.
//!
//! A small integer code picks the filled corners of a hypercube, that seed grows level by level
//! into a fractal design, and the same code always unfolds into the same shape, so a design can be
//! named, counted and drawn again from its number alone. Half the module generates and half
//! measures; everything rests on the tensors and cells of [`crate::core`] and the sequences of
//! [`crate::num`].
//!
//! - `atoms` fills a tensor with a carpet, a net, beams or noise.
//! - `bang` enumerates the design codes, their symmetries and their counts.
//! - `cell` holds the N-dimensional cell and the pipeline the fixed dimensions share.
//! - `two`, `three` and `six` run that pipeline for flat cells, cubes and hexagons.
//! - `counts` gives the same fills, surfaces, hex slices and carry ladder in closed form.
//! - `graph` lifts a grid into nodes and branches; `spectrum` reads the Laplacian spectra off it.
//! - `nodal` rings the plate a design cuts: its lowest modes, their nodal domains and Courant bounds, the Chladni figures of the design.
//! - `shape` crops a cell against a rational shape, cell by cell, with no floats.
//! - `rules` marks the cells of a hypercube whose coordinate residues satisfy a rule.
//! - `moire` layers one design at many scales into an interference field.
//! - `press` weighs the integers a design's digit rule keeps.
//! - `spin` spins a raster about its centre; `tourbillon` stacks the turned parity carpets.
//! - `spirograph` rolls a byte grid as a wheel; `roulette` counts where its curves cross.
//! - `name` prints and parses the one canonical JSON object of every design, rule, tile and word.
//! - `arrays`: a sparse sensor array, its difference coarray and weights, its essential sensors, and the fractal array a generator grows.
//! - `automaton`: the Dirichlet series of a memory design, continued through its transfer matrix.
//! - `design`: the digit designs on the line, their Mobius meter and the ordinates it carries.
//! - `dissection`: the missing-digit sets cut by Dirichlet approximation: the four regions, the digit transform's `l^1` masses, the chain certificate and its wall.
//! - `ladder`: the Dirichlet series of a digit design, continued to the plane, each value with its bound.
//! - `memory`: a rule on consecutive digits, its transfer matrix, its words, its Perron root.
//! - `radix`: a digit set inside the residues of a base in a ring, and where its words land.
//! - `sumset`: the base-3 design plus the base-4 design, Erdos problem 125: its bit array, its density and the energy of its levels.
//!
//! The doors are [`crate::math::atoms::carpet_2d`], [`crate::math::bang::bang`],
//! [`crate::math::two::carpet`], [`crate::math::two::census()`], [`crate::math::two::to_json`],
//! [`crate::math::counts::fill`], [`crate::math::three::census::surface`] and
//! [`crate::math::spectrum::laplacian_spectrum`].

/// The Truchet arcs of a flat design: every cell two quarter arcs, the loops and strands they join into, and the proved loop laws.
pub mod arcs;
/// The sparse arrays: sensor positions, their difference coarray and its weights, the essential sensors whose loss opens a hole, and the fractal array a generator grows with the essential count Theorem A proves.
pub mod arrays;
/// Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.
pub mod atoms;
/// The matrix ladder: the Dirichlet series of a memory design continued through its transfer matrix, its determinant cofactor and the residues on its pole combs.
///
/// A claim witness with no caller in the crate: it witnesses `research/claims/memory-dial.md`.
pub mod automaton;
/// The universe of design codes: corners, symmetries and their counts.
pub mod bang;
/// The dimension-generic cell and the pipeline the fixed dimensions share.
pub mod cell;
/// The closed-form counts: fills, surfaces, hex slices and the carry ladder, without rendering.
pub mod counts;
/// The digit designs on the integer line: their elements, their Mobius meter, its density echo and the ordinates its spectrum carries.
pub mod design;
/// The unconditional dissection of a missing-digit set: the four regions of the frequency grid, the `l^1` masses of the digit transform, the chain certificate against `1/5`, the constant `kappa_F` and the set's own meter and prime count.
pub mod dissection;
/// The spatial network: its nodes, branches, extraction and census.
pub mod graph;
/// The peeled ladder: the Dirichlet series of a digit design continued to the whole plane, its Lyndon cofactor and the residues at its poles, each value carrying its bound.
///
/// A claim witness with no caller in the crate: it witnesses `research/claims/zeros-of-the-design-zeta.md`.
pub mod ladder;
/// The memory designs: a rule on `k` consecutive digits, its transfer matrix, the words it accepts and the Perron root that is their dimension.
pub mod memory;
/// The moire fields layered from sampled designs.
pub mod moire;
/// The mrly names: one canonical JSON object for every mathematical thing.
pub mod name;
/// The Chladni figures of a design: the lowest modes of the plate it cuts, in a canonical basis, with their nodal domains, Courant bounds and rasters.
pub mod nodal;
/// The sequence press: the integers a design's digit rule keeps, weighed all at once.
pub mod press;
/// The radix designs: a digit set inside the residues of a base in a ring, a unit twist per digit, and the points their words land on.
pub mod radix;
/// The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.
pub mod roulette;
/// The residue rules that mark a hypercube's cells.
pub mod rules;
/// The exact crop machinery: rational shapes classified cell by cell, no floats.
pub mod shape;
/// The hexagon world: cubes flattened to triangle-meshed hexes.
pub mod six;
/// The symmetric eigensolver and the Laplacian spectra it reads off a network.
pub mod spectrum;
/// The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.
pub mod spin;
/// The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.
pub mod spirograph;
/// The sumset of Erdos problem 125: the base-3 design `{0, 1}` plus the base-4 design `{0, 1}` as a bit array, its density below `x`, the gaps its level pairs leave and their additive energy.
pub mod sumset;
/// The cube pipeline: designs, tiles, graphs and renderings in three dimensions.
pub mod three;
/// The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.
pub mod tourbillon;
/// The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.
pub mod two;

// SERDE

#[cfg(test)]
pub(crate) fn round_trip<T>(value: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let text = serde_json::to_string(&value).unwrap();
    assert_eq!(serde_json::from_str::<T>(&text).unwrap(), value, "{text}");
}

#[cfg(test)]
mod tests {
    use super::round_trip;
    use crate::math::memory::Rule;
    use crate::math::shape::{Frac, Region};
    use crate::math::spin::Blend;
    use crate::math::spirograph::{Kind, Pencil};
    use crate::math::tourbillon::Eye;

    #[test]
    fn serde_round_trips() {
        round_trip(Rule::new(1, 2, 7).unwrap());
        round_trip(Frac { num: -3, den: 4 });
        round_trip(Region::Cut);
        round_trip(Blend::Parity);
        round_trip(Pencil {
            x: 0.5,
            y: -0.25,
            seat: (1, -2),
            kind: Kind::Corner,
        });
        round_trip(Eye {
            angle: 45.0,
            numer: 90,
            denom: 2,
        });
    }
}
