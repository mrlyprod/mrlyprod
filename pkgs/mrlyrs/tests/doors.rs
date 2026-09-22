use mrlyrs::core::cell::{magic, merge, mosaic, remap, rot90_map, tile_map, Cell};
use mrlyrs::core::colors::Color;
use mrlyrs::core::ramp::Colorizer;
use mrlyrs::core::resample::{block, resample, Filter};
use mrlyrs::core::rng::Rng;
use mrlyrs::core::tensor::{Dtype, Tensor};
use mrlyrs::core::{gif, png, unpng};
use mrlyrs::gen::draw::ConfigNd;
use mrlyrs::gen::name::Tile as Name;
use mrlyrs::gen::recipe::{Catalog, Design, Group, Source, Tile as Recipe};
use mrlyrs::life::models::Config;
use mrlyrs::life::source::{Counts, Source as Seq};
use mrlyrs::math::bang::factory::MagicLayer;
use mrlyrs::math::bang::Code;
use mrlyrs::math::cell::models::{Cell2d, Cell3d, CellNd};
use mrlyrs::math::graph::models::{Branch, Network};
use mrlyrs::math::name::Bang;
use mrlyrs::math::name::{Named, Word};
use mrlyrs::math::shape::{Frac, Shape};
use mrlyrs::math::six::{Orientation, Projection};
use mrlyrs::{core, font, gen, life, math, num};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::str::FromStr;

fn turn(_: Source, rng: &mut Rng) -> usize {
    rng.below(4)
}

fn square() -> Tensor {
    Tensor::of(vec![1, 2, 3, 4], vec![2, 2]).unwrap()
}

fn unit_cell() -> Cell {
    Cell::new(math::atoms::ones_2d(2))
}

fn empty_2d() -> Cell2d {
    Cell2d::new(Tensor::new(vec![0, 0])).unwrap()
}

fn empty_3d() -> Cell3d {
    Cell3d::new(Tensor::new(vec![0, 0, 0])).unwrap()
}

fn stray_network() -> Network {
    let mut net = Network::new(1);
    net.add_node(vec![0.0]).unwrap();
    net.branches.push(Branch {
        parent: 0,
        child: 9,
        radius: 1.0,
    });
    net
}

fn lonely_network() -> Network {
    let mut net = Network::new(1);
    net.add_node(vec![0.0]).unwrap();
    net
}

fn wild_shape() -> Shape {
    Shape::Ball {
        center: vec![Frac {
            num: 1,
            den: i64::MAX,
        }],
        radius: Frac::new(1, 2).unwrap(),
    }
}

const BAD: &[(&str, fn())] = &[
    // CORE
    ("core::tensor::of, a short buffer", || {
        let _ = Tensor::of(vec![1, 2, 3], vec![2, 2]);
    }),
    ("core::tensor::of, a long buffer", || {
        let _ = Tensor::of(vec![1, 2, 3, 4, 5], vec![2, 2]);
    }),
    ("core::tensor::rot90, an axis past the rank", || {
        let _ = square().rot90(1, (0, 2));
    }),
    ("core::tensor::rot90, one axis twice", || {
        let _ = square().rot90(1, (1, 1));
    }),
    ("core::tensor::tile, too few reps", || {
        let _ = square().tile(&[2]);
    }),
    ("core::tensor::tile, no reps", || {
        let _ = square().tile(&[]);
    }),
    ("core::tensor::get, an index of the wrong rank", || {
        let _ = square().get(&[0]);
    }),
    ("core::tensor::get, an index past the extent", || {
        let _ = square().get(&[2, 0]);
    }),
    ("core::tensor::set, an index past the extent", || {
        let _ = square().set(&[0, 2], 9);
    }),
    ("core::tensor::bytes, a wide tensor", || {
        let _ = Tensor::typed(vec![2, 2], Dtype::U16).bytes();
    }),
    ("core::tensor::bytes_mut, a wide tensor", || {
        let _ = Tensor::typed(vec![2, 2], Dtype::I32).bytes_mut();
    }),
    ("core::tensor::flip, an axis past the rank", || {
        let _ = square().flip(2);
    }),
    ("core::tensor::transpose, an axis past the rank", || {
        let _ = square().transpose(0, 2);
    }),
    ("core::cell::rot90_map, one axis twice", || {
        let _ = rot90_map(&[3, 3], 1, (0, 0));
    }),
    ("core::cell::tile_map, too many reps", || {
        let _ = tile_map(&[3, 3], &[2, 2, 2]);
    }),
    ("core::cell::remap, a map shorter than the cell", || {
        let _ = remap(&unit_cell(), &[0, 1, 2], &[2, 2]);
    }),
    ("core::cell::merge, no cells", || {
        let _ = merge(&[], &[1]);
    }),
    ("core::cell::merge, reps that do not hold the cells", || {
        let _ = merge(&[unit_cell(), unit_cell()], &[2]);
    }),
    ("core::cell::mosaic, a mask past the cell list", || {
        let mask = Tensor::of(vec![0, 2, 0, 0], vec![2, 2]).unwrap();
        let _ = mosaic(&mask, std::slice::from_ref(&unit_cell()));
    }),
    ("core::cell::magic, one layer", || {
        let _ = magic(&[unit_cell()]);
    }),
    ("core::colors::from_hex, a non-ascii hex", || {
        let _ = Color::from_hex("a\u{e9}bcd");
    }),
    ("core::colors::from_hex, an emoji", || {
        let _ = Color::from_hex("\u{1f600}\u{1f600}");
    }),
    ("core::colors::from_hex, an empty text", || {
        let _ = Color::from_hex("");
    }),
    ("core::colors::to_hex, a transparent color", || {
        let _ = Color::rgba(0, 0, 0, 0).to_hex();
    }),
    ("core::codec::png, a zero scale", || {
        let _ = png(&[[0, 0, 0, 255]], 1, 1, 0);
    }),
    (
        "core::codec::png, a size that is not the pixel count",
        || {
            let _ = png(&[[0, 0, 0, 255]], 2, 2, 1);
        },
    ),
    ("core::codec::png, no pixels", || {
        let _ = png(&[], 0, 0, 1);
    }),
    ("core::codec::png, a scale past the width", || {
        let _ = png(&[[0, 0, 0, 255]], 1, 1, 1 << 40);
    }),
    ("core::codec::unpng, no bytes", || {
        let _ = unpng(&[]);
    }),
    ("core::codec::unpng, bytes that are not a png", || {
        let _ = unpng(b"not a png at all");
    }),
    ("core::codec::gif, a zero delay", || {
        let _ = gif(&[&[0u8, 1, 1, 0][..]], &[[0, 0, 0, 255]], 2, 2, 0, 5);
    }),
    ("core::codec::gif, no frames", || {
        let _ = gif(&[], &[[0, 0, 0, 255]], 2, 2, 1, 5);
    }),
    ("core::codec::gif, an empty palette", || {
        let _ = gif(&[&[0u8, 1, 1, 0][..]], &[], 2, 2, 1, 5);
    }),
    ("core::codec::gif, an index past the palette", || {
        let _ = gif(&[&[0u8, 1, 2, 0][..]], &[[0, 0, 0, 255]], 2, 2, 1, 5);
    }),
    ("core::ramp::color, a zero maximum", || {
        let _ = mrlyrs::core::ramp::color(&Colorizer::heat(), usize::MAX, 0);
    }),
    ("core::resample::resample, a zero target", || {
        let _ = resample(&[[0u8, 0, 0, 255]; 4], 2, 2, 0, 4, Filter::Nearest);
    }),
    (
        "core::resample::resample, a size that is not the pixels",
        || {
            let _ = resample(&[[0u8, 0, 0, 255]; 4], 3, 2, 4, 4, Filter::Nearest);
        },
    ),
    ("core::resample::resample, a target past the width", || {
        let _ = resample(&[[0u8, 0, 0, 255]; 4], 2, 2, usize::MAX, 2, Filter::Nearest);
    }),
    (
        "core::resample::block, a size that is not the buffer",
        || {
            let _ = block(&[1u8, 2, 3], 2, 2, 1);
        },
    ),
    ("core::resample::block, a factor past the width", || {
        let _ = block(&[1u8, 2, 3, 4], 2, 2, usize::MAX);
    }),
    ("core::rng::choice, an empty slice", || {
        let empty: [usize; 0] = [];
        let _ = Rng::new(2).choice(&empty);
    }),
    ("core::rng::below, a zero bound", || {
        let _ = Rng::new(2).below(0);
    }),
    ("core::rng::range, a high below the low", || {
        let _ = Rng::new(2).range(9, 1);
    }),
    ("core::rng::sample_indices, more than there are", || {
        let _ = Rng::new(2).sample_indices(2, 9);
    }),
    // NUM
    ("num::factor::factorial, one past the ceiling", || {
        let _ = num::factor::factorial(35);
    }),
    ("num::factor::factorial, a four byte maximum", || {
        let _ = num::factor::factorial(u32::MAX as usize);
    }),
    ("num::factor::gcd, two zeroes", || {
        let _ = num::factor::gcd(0, 0);
    }),
    ("num::factor::gcd, the widest pair", || {
        let _ = num::factor::gcd(u128::MAX, u128::MAX);
    }),
    ("num::factor::divisors, a zero", || {
        let _ = num::factor::divisors(0);
    }),
    ("num::factor::mobius, a zero", || {
        let _ = num::factor::mobius(0);
    }),
    ("num::prime::is_prime, the widest number", || {
        let _ = num::prime::is_prime(usize::MAX);
    }),
    ("num::series::zeta, an s of one", || {
        let _ = num::series::zeta(1.0, 10);
    }),
    ("num::series::zeta, an s that is not a number", || {
        let _ = num::series::zeta(f64::NAN, 10);
    }),
    ("num::series::visible, a zero dimension", || {
        let _ = num::series::visible(10, 0);
    }),
    ("num::series::bernoulli, a count past thirty two", || {
        let _ = num::series::bernoulli(33);
    }),
    ("num::lattice::zeta_whole, an s of one", || {
        let _ = num::lattice::zeta_whole(1);
    }),
    ("num::lattice::visible_density, a dimension of one", || {
        let _ = num::lattice::visible_density(1);
    }),
    ("num::blend::decimate, a zero step", || {
        let _ = num::blend::decimate(&[1, 2, 3], 0, 0);
    }),
    ("num::blend::sigma, a sum past the width", || {
        let _ = num::blend::sigma(&[i128::MAX, 1]);
    }),
    (
        "num::design::peaks, an axis shorter than the scores",
        || {
            let _ = num::design::peaks(&[1.0, 2.0], &[1.0, 2.0, 3.0], (0.0, 1.0), 0.5);
        },
    ),
    (
        "num::design::spectrum, a pole list under the samples",
        || {
            let _ = num::design::spectrum(&[0.0, 1.0], &[1.0, 2.0, 3.0, 4.0]);
        },
    ),
    ("num::fft::fft, two buffers of unequal length", || {
        let mut re = vec![0.0; 8];
        let mut im = vec![0.0; 4];
        let _ = num::fft::fft(&mut re, &mut im, false);
    }),
    ("num::fft::fft, a length that is not a power of two", || {
        let mut re = vec![0.0; 6];
        let mut im = vec![0.0; 6];
        let _ = num::fft::fft(&mut re, &mut im, false);
    }),
    (
        "num::fft::fft2, a buffer that is not the side squared",
        || {
            let mut re = vec![0.0; 9];
            let mut im = vec![0.0; 9];
            let _ = num::fft::fft2(&mut re, &mut im, 4, false);
        },
    ),
    (
        "num::fft::transform, a buffer that is not the side squared",
        || {
            let _ = num::fft::transform(&[1.0; 9], 4);
        },
    ),
    ("num::fft::embed_kernel, an even mask side", || {
        let _ = num::fft::embed_kernel(&[1; 4], 2, 8);
    }),
    ("num::fft::embed_kernel, a mask past the field", || {
        let _ = num::fft::embed_kernel(&[1; 81], 9, 8);
    }),
    (
        "num::fft::convolve, a kernel that is not the side squared",
        || {
            let _ = num::fft::convolve(&[1.0; 16], &[1.0; 9], 4);
        },
    ),
    ("num::ladder::Design::new, a digit past the base", || {
        let _ = num::ladder::Design::new(5, &[3]);
    }),
    (
        "num::ladder::Design::with_peel, a digit past the base",
        || {
            let _ = num::ladder::Design::with_peel(5, &[0, 7], 3);
        },
    ),
    ("num::memory::Rule::new, a zero dimension", || {
        let _ = num::memory::Rule::new(0, 1, 0);
    }),
    ("num::memory::Rule::new, a code past its width", || {
        let _ = num::memory::Rule::new(1, 2, 16);
    }),
    (
        "num::memory::Rule::full, a width past the dimension",
        || {
            let _ = num::memory::Rule::full(3, 3);
        },
    ),
    ("num::radix::Base::new, a unit base", || {
        let _ = num::radix::Base::new(num::gauss::Ring::Gaussian, (1, 0));
    }),
    ("num::radix::tile, an m of twelve", || {
        let _ = num::radix::tile(12, 1);
    }),
    ("num::radix::tile, a code past its range", || {
        let _ = num::radix::tile(3, 1 << 9);
    }),
    ("num::sieve::side, a letter that is not odd", || {
        let _ = num::sieve::side(&[4]);
    }),
    ("num::sieve::raster, a letter that is not odd", || {
        let _ = num::sieve::raster(&[0]);
    }),
    ("num::sieve::punctures, a letter that is not odd", || {
        let _ = num::sieve::punctures(&[2], 2);
    }),
    ("num::blend::cauchy, a sum past the width", || {
        let _ = num::blend::cauchy(&[i128::MAX, 1], &[2, 1]);
    }),
    ("num::lattice::recovered, a zero dimension", || {
        let _ = num::lattice::recovered(10, 0);
    }),
    (
        "num::fft::magnitude_spectrum, a buffer that is not the side squared",
        || {
            let _ = num::fft::magnitude_spectrum(&[1.0; 9], 4);
        },
    ),
    (
        "num::fft::radial_profile, a buffer that is not the side squared",
        || {
            let _ = num::fft::radial_profile(&[1.0; 9], 4);
        },
    ),
    (
        "num::fft::convolve_with, a field that is not the side squared",
        || {
            let _ = num::fft::convolve_with(&[1.0; 9], &[1.0; 16], &[1.0; 16], 4);
        },
    ),
    ("num::sieve::cells, a letter that is not odd", || {
        let _ = num::sieve::cells(&[4], 2);
    }),
    ("num::sieve::holes, a letter that is not odd", || {
        let _ = num::sieve::holes(&[4, 3], 2);
    }),
    ("num::sieve::limit, a letter that is not odd", || {
        let _ = num::sieve::limit(&[4, 6], 2);
    }),
    (
        "num::radix::Radix::from_code, a code past its range",
        || {
            let base = num::radix::Base::new(num::gauss::Ring::Eisenstein, (3, 0)).unwrap();
            let _ = num::radix::Radix::from_code(base, 1 << 9);
        },
    ),
    (
        "num::radix::Radix::new, digits that do not cover the residues",
        || {
            let base = num::radix::Base::new(num::gauss::Ring::Eisenstein, (3, 0)).unwrap();
            let _ = num::radix::Radix::new(base, vec![(0, 0), (1, 0)], vec![(1, 0); 3]);
        },
    ),
    ("num::radix::with_twists, a twist past the digits", || {
        let _ = num::radix::twindragon().unwrap().with_twists(&[0, 9]);
    }),
    ("num::memory::Rule::full, a zero dimension", || {
        let _ = num::memory::Rule::full(0, 1);
    }),
    ("num::ladder::Design::new, a base of one", || {
        let _ = num::ladder::Design::new(1, &[0, 1]);
    }),
    // MATH
    ("math::atoms::carpet_2d, a zero side", || {
        let _ = math::atoms::carpet_2d(0);
    }),
    (
        "math::spectrum::symmetric_eigenvalues, a crooked matrix",
        || {
            let _ = math::spectrum::symmetric_eigenvalues(&[vec![1.0, 2.0]]);
        },
    ),
    ("math::spectrum::symmetric_eigenvalues, a nan", || {
        let _ = math::spectrum::symmetric_eigenvalues(&[vec![f64::NAN]]);
    }),
    (
        "math::spectrum::clusters, a nan in the sorted slice",
        || {
            let _ = math::spectrum::clusters(&[0.0, f64::NAN], 1e-9);
        },
    ),
    (
        "math::spectrum::laplacian_spectrum, a lone node normalised",
        || {
            let _ = math::spectrum::laplacian_spectrum(&lonely_network(), true);
        },
    ),
    (
        "math::spin::arcs, a raster that is not the side squared",
        || {
            let _ = math::spin::arcs(&[1.0f32; 15], 4, 1.0);
        },
    ),
    (
        "math::spin::profile, a raster that is not the side squared",
        || {
            let _ = math::spin::profile(&[1.0f32; 15], 4, 8);
        },
    ),
    (
        "math::moire::field::from_data, a count that is not the side squared",
        || {
            let _ = math::moire::field::Field::from_data(vec![0.0; 3], 2);
        },
    ),
    ("math::moire::pairs::sampled, a zero scale", || {
        let _ = math::moire::pairs::sampled(0, 3);
    }),
    ("math::graph::models::add_node, a stray dimension", || {
        let _ = Network::new(2).add_node(vec![0.0]);
    }),
    (
        "math::graph::models::add_branch, a node past the list",
        || {
            let _ = Network::new(2).add_branch(0, 5, 1.0);
        },
    ),
    (
        "math::graph::models::degree, a branch past the nodes",
        || {
            let _ = stray_network().degree();
        },
    ),
    (
        "math::graph::census::census, a branch past the nodes",
        || {
            let _ = math::graph::census::census(&stray_network());
        },
    ),
    (
        "math::cell::models::Cell2d::new, a tensor of rank three",
        || {
            let _ = Cell2d::new(Tensor::new(vec![2, 2, 2]));
        },
    ),
    (
        "math::cell::models::Cell3d::new, a tensor of rank two",
        || {
            let _ = Cell3d::new(Tensor::new(vec![2, 2]));
        },
    ),
    (
        "math::cell::models::CellNd::new, a tensor of the wrong rank",
        || {
            let _ = CellNd::<1>::new(Tensor::new(vec![2, 2]));
        },
    ),
    ("math::two::carpet, a zero side", || {
        let _ = math::two::carpet(0, 1);
    }),
    ("math::two::carpet, a zero level", || {
        let _ = math::two::carpet(3, 0);
    }),
    ("math::two::to_json, an empty cell", || {
        let _ = math::two::to_json(&empty_2d());
    }),
    ("math::two::census, an empty cell", || {
        let _ = math::two::census::census(&empty_2d());
    }),
    ("math::three::census::surface, an empty cell", || {
        let _ = math::three::census::surface(&empty_3d());
    }),
    (
        "math::counts::counting::fill, a code past its range",
        || {
            let _ = math::counts::counting::fill(Code::from(16u64), 3, 2, 1, 2);
        },
    ),
    ("math::counts::counting::fill, a dimension of seven", || {
        let _ = math::counts::counting::fill(Code::from(1u64), 3, 7, 1, 2);
    }),
    ("math::counts::counting::limit, a level past a u128", || {
        let _ = math::counts::counting::limit(Code::from(7u64), 2, 1000, 2);
    }),
    ("math::bang::bang, a zero dimension", || {
        let _ = math::bang::universe::bang(0);
    }),
    ("math::bang::bang, a dimension of five", || {
        let _ = math::bang::universe::bang(5);
    }),
    ("math::bang::Universe::distinct, the plane", || {
        let _ = math::bang::universe::bang(2).unwrap().distinct();
    }),
    ("math::bang::baseq::canonical, an empty group", || {
        let _ = math::bang::baseq::canonical(&[], Code::from(1u64));
    }),
    (
        "math::bang::baseq::group_order, a dimension past the factorial",
        || {
            let _ = math::bang::baseq::group_order(2, 35);
        },
    ),
    (
        "math::bang::baseq::predicted_group_order, a dimension past the factorial",
        || {
            let _ = math::bang::baseq::predicted_group_order(2, 35);
        },
    ),
    (
        "math::bang::baseq::group_order, a dimension past a u128",
        || {
            let _ = math::bang::baseq::group_order(2, 200);
        },
    ),
    (
        "math::bang::baseq::predicted_group_order, a dimension past a u128",
        || {
            let _ = math::bang::baseq::predicted_group_order(3, 200);
        },
    ),
    (
        "math::bang::baseq::total_designs, a dimension of seven",
        || {
            let _ = math::bang::baseq::total_designs(2, 7);
        },
    ),
    (
        "math::bang::catalog::universe_codes, a dimension of five",
        || {
            let _ = math::bang::catalog::universe_codes(5);
        },
    ),
    (
        "math::bang::factory::code_to_corners, a code one past the corners",
        || {
            let _ = math::bang::factory::code_to_corners(Code::from(16u64), 2, 2);
        },
    ),
    (
        "math::bang::factory::code_to_corners, a dimension of seven",
        || {
            let _ = math::bang::factory::code_to_corners(Code::from(1u64), 7, 2);
        },
    ),
    (
        "math::bang::factory::total_codes, a dimension of seven",
        || {
            let _ = math::bang::factory::total_codes(7, 2);
        },
    ),
    ("math::bang::factory::magic, no layers", || {
        let _ = math::bang::factory::magic(&[]);
    }),
    ("math::six::geometry::blank, a zero radius", || {
        let _ = math::six::geometry::blank(0, Orientation::Horizontal, 1, 0);
    }),
    ("math::six::star::Star::new, a code past its range", || {
        let _ = math::six::star::Star::new(1 << 9);
    }),
    (
        "math::press::Press::new, a universe past its corners",
        || {
            let _ = math::press::Press::new(7, 2);
        },
    ),
    ("math::press::interleave, an empty weave", || {
        let _ = math::press::interleave(&[], 2);
    }),
    ("math::rules::render, a zero number", || {
        let _ = math::rules::render(&[], 0, 2, 2);
    }),
    ("math::shape::Frac::new, a zero denominator", || {
        let _ = Frac::new(1, 0);
    }),
    ("math::shape::Frac::plus, a sum past an i64", || {
        let _ = Frac::whole(i64::MAX).plus(Frac::whole(i64::MAX));
    }),
    ("math::shape::classify, a center past an i64", || {
        let _ = math::shape::classify(&wild_shape(), 1, &[0]);
    }),
    ("math::shape::census, a center past an i64", || {
        let _ = math::shape::census(&wild_shape(), &math::atoms::ones_2d(1));
    }),
    ("math::name::Bang::from_json, an empty text", || {
        let _ = math::name::Bang::from_json("");
    }),
    ("math::name::Bang::from_json, a code past its range", || {
        let _ = math::name::Bang::from_json(r#"{"kind":"bang","dim":2,"code":16}"#);
    }),
    ("math::name::Sequence::from_json, an empty text", || {
        let _ = math::name::Sequence::from_json("");
    }),
    ("math::name::Word::from_json, an empty text", || {
        let _ = Word::from_json("");
    }),
    (
        "math::name::Word::new, a letter the grammar cannot draw",
        || {
            let _ = Word::new(2, &[(7, 3)]);
        },
    ),
    ("math::name::Bang::from_json, a text without a kind", || {
        let _ = math::name::Bang::from_json("{}");
    }),
    ("math::name::Bang::from_json, a broken json", || {
        let _ = math::name::Bang::from_json(r#"{"kind":"bang""#);
    }),
    (
        "math::spin::ring, a raster that is not the side squared",
        || {
            let _ = math::spin::ring(&[1.0f32; 15], 4, 1.0);
        },
    ),
    (
        "math::spin::harmonics, a raster that is not the side squared",
        || {
            let _ = math::spin::harmonics(&[1.0f32; 15], 4, 8, 4);
        },
    ),
    ("math::spectrum::laplacian, a lone node normalised", || {
        let _ = math::spectrum::laplacian(&lonely_network(), true);
    }),
    ("math::graph::census::tips, a branch past the nodes", || {
        let _ = math::graph::census::tips(&stray_network());
    }),
    (
        "math::graph::census::components, a branch past the nodes",
        || {
            let _ = math::graph::census::components(&stray_network());
        },
    ),
    (
        "math::graph::models::adjacency, a branch past the nodes",
        || {
            let _ = stray_network().adjacency();
        },
    ),
    ("math::moire::pairs::sampled, a zero second scale", || {
        let _ = math::moire::pairs::sampled(3, 0);
    }),
    (
        "math::counts::counting::void, a code past its range",
        || {
            let _ = math::counts::counting::void(Code::from(16u64), 3, 2, 1, 2);
        },
    ),
    (
        "math::counts::counting::ratio, a code past its range",
        || {
            let _ = math::counts::counting::ratio(Code::from(16u64), 3, 2, 1, 2);
        },
    ),
    (
        "math::counts::counting::rational, a code past its range",
        || {
            let _ = math::counts::counting::rational(Code::from(16u64), 3, 2, 1, 2);
        },
    ),
    (
        "math::counts::counting::dimension, a code past its range",
        || {
            let _ = math::counts::counting::dimension(Code::from(16u64), 3, 2, 2);
        },
    ),
    (
        "math::bang::baseq::representatives, a walk past the limit",
        || {
            let _ = math::bang::baseq::representatives(3, 4);
        },
    ),
    ("math::bang::catalog::sources, a dimension of five", || {
        let _ = math::bang::catalog::sources(&Catalog::Universe, 5);
    }),
    (
        "math::bang::word::components, layers of two dimensions",
        || {
            let plane = MagicLayer::new(Bang::new(7, 2, 2), 3);
            let cube = MagicLayer::new(Bang::new(23, 3, 2), 3);
            let _ = math::bang::word::components(&[plane, cube]);
        },
    ),
    ("math::bang::word::fill, no layers", || {
        let _ = math::bang::word::fill(&[]);
    }),
    ("math::press::usage, a dimension of seven", || {
        let _ = math::press::usage(1, 7, 2);
    }),
    ("math::press::word_member, no layers", || {
        let _ = math::press::word_member(&[], 0);
    }),
    ("math::six::raster::raster, a zero size", || {
        let cell = math::six::cut_design(Code::from(23u64), 3, 1, 2).unwrap();
        let _ = math::six::raster::raster(&cell, 0);
    }),
    ("math::six::star::hexagon, an even layer", || {
        let _ = math::six::star::Star::new(23).unwrap().hexagon(4);
    }),
    ("math::six::star::arm, a zero layer", || {
        let _ = math::six::star::Star::new(23).unwrap().arm(0, 0);
    }),
    ("math::shape::named, a box past an i64", || {
        let _ = math::shape::named("box", 2, Frac::whole(i64::MAX));
    }),
    ("math::shape::refine, a zero base", || {
        let ball = math::shape::named("ball", 2, Frac::new(1, 2).unwrap()).unwrap();
        let _ = math::shape::refine(&math::atoms::ones_2d(3), &ball, 0, 1, false);
    }),
    ("math::shape::regions, a center past an i64", || {
        let _ = math::shape::regions(&wild_shape(), &[1]);
    }),
    ("math::shape::crop, a center past an i64", || {
        let _ = math::shape::crop(&math::atoms::ones_2d(1), &wild_shape(), true);
    }),
    ("math::rules::render, a zero dimension", || {
        let _ = math::rules::render(&[], 3, 0, 2);
    }),
    ("math::rules::render, a zero base", || {
        let _ = math::rules::render(&[], 3, 2, 0);
    }),
    (
        "math::name::Sequence::from_json, a measure the axis has not",
        || {
            let _ = math::name::Sequence::from_json(
                r#"{"kind":"sequence","dim":2,"code":7,"measure":"area","axis":"side"}"#,
            );
        },
    ),
    (
        "math::name::Word::new, a letter past the code range",
        || {
            let _ = Word::new(2, &[(273, 3), (9, 2)]);
        },
    ),
    // LIFE
    ("life::animate, an even mask", || {
        let even = Cell2d::new(Tensor::full(vec![2, 2], 1)).unwrap();
        let config = Config::new(even, vec![3].into(), vec![2, 3].into());
        let seed = Cell2d::new(Tensor::full(vec![3, 3], 1)).unwrap();
        let _ = life::animate(&seed, &config);
    }),
    (
        "life::elementary::history, a row that is not binary",
        || {
            let _ = life::elementary::history(&[0, 2, 0], 90, 1, false);
        },
    ),
    ("life::elementary::step, a row that is not binary", || {
        let _ = life::elementary::step(&[1, 0, 9], 90, true);
    }),
    ("life::mask::design_mask, a code past its range", || {
        let _ = life::mask::design_mask(2, Code::from(7u64), 4, 1);
    }),
    ("life::mask::design_mask, a zero level", || {
        let _ = life::mask::design_mask(1, Code::from(1u64), 3, 0);
    }),
    ("life::render::heatmap, a zero scale", || {
        let _ = life::render::heatmap(&[empty_2d()], 0);
    }),
    ("life::render::heatmap, grids of two sizes", || {
        let odd = Cell2d::new(Tensor::new(vec![3, 3])).unwrap();
        let _ = life::render::heatmap(&[empty_2d(), odd], 4);
    }),
    ("life::source::counts, a code past the universe", || {
        let _ = life::source::counts(Seq::CodeFills(1 << 20), 8, true, true);
    }),
    (
        "life::source::Counts::values, a code past the universe",
        || {
            let _ = Counts::drawn(Seq::CodeFills(1 << 20), false, false).values(8);
        },
    ),
    ("life::metrics::entropy, an empty grid", || {
        let _ = life::metrics::entropy(&empty_2d());
    }),
    ("life::metrics::churn, no grids", || {
        let _ = life::metrics::churn(&[]);
    }),
    (
        "life::mask::design_mask, a dimension the lattice has not",
        || {
            let _ = life::mask::design_mask(3, Code::from(7u64), 3, 1);
        },
    ),
    ("life::mask::design_mask, a level past the grid", || {
        let _ = life::mask::design_mask(2, Code::from(7u64), 3, 1000);
    }),
    (
        "life::source::counts, a void code past the universe",
        || {
            let _ = life::source::counts(Seq::CodeVoids(1 << 20), 8, true, true);
        },
    ),
    (
        "life::models::Config::counts, a drawn source past the universe",
        || {
            let mask = Cell2d::new(Tensor::full(vec![3, 3], 1)).unwrap();
            let drawn = Counts::drawn(Seq::CodeFills(1 << 20), false, false);
            let _ = Config::new(mask, drawn, vec![2, 3].into()).counts();
        },
    ),
    // FONT
    ("font::glyph, a char the alphabet has not", || {
        let _ = font::glyph('\u{1f600}');
    }),
    ("font::supported, the whole alphabet", || {
        let _ = font::supported();
    }),
    ("font::raster::raster, an empty text", || {
        let _ = font::raster::raster("");
    }),
    ("font::raster::raster, a char the alphabet has not", || {
        let _ = font::raster::raster("\u{1f600}");
    }),
    ("font::paths::path, a char the alphabet has not", || {
        let _ = font::paths::path('\u{1f600}');
    }),
    ("font::paths::floor, no rows", || {
        let _ = font::paths::floor(&[]);
    }),
    ("font::paths::draft, no rows", || {
        let _ = font::paths::draft(&[]);
    }),
    ("font::animate::animate, an empty text", || {
        let _ = font::animate::animate("", 0);
    }),
    ("font::animate::merge, an empty text", || {
        let _ = font::animate::merge("", 0);
    }),
    ("font::animate::cycle, an empty merge", || {
        let _ = font::animate::cycle(&font::animate::animate("a", 1), &[], 1);
    }),
    ("font::animate::cycle, an empty write", || {
        let _ = font::animate::cycle(&font::animate::animate("", 0), &[vec![0]], 1);
    }),
    // GEN
    ("gen::background, a zero width", || {
        let _ = gen::background(7, 0, 3);
    }),
    ("gen::background, a zero height", || {
        let _ = gen::background(7, 2, 0);
    }),
    ("gen::recipe::Tile::new, a recipe with no slots", || {
        let _ = Recipe::new(Group::General).check();
    }),
    ("gen::recipe::Tile::size, sizes that disagree", || {
        let _ = Recipe::new(Group::General).size(5, 5).check();
    }),
    ("gen::draw::create, a config that draws no tile", || {
        let narrow: ConfigNd<2> = ConfigNd {
            min_size: 9,
            max_size: 3,
            ..Default::default()
        };
        let _ = gen::draw::create(&narrow, turn, &mut Rng::new(1));
    }),
    ("gen::draw::create, a config with no catalog", || {
        let sourceless: ConfigNd<2> = ConfigNd {
            catalog: Catalog::Codes(Vec::new()),
            ..Default::default()
        };
        let _ = gen::draw::create(&sourceless, turn, &mut Rng::new(1));
    }),
    ("gen::draw::random_tile, a size below the floor", || {
        let _ = gen::draw::random_tile::<2>(1, turn, &mut Rng::new(1));
    }),
    ("gen::build::build_2d, a recipe with no slots", || {
        let _ = gen::build::build_2d(&Recipe::new(Group::General));
    }),
    ("gen::build::build_2d, a cubic design in the plane", || {
        let mut cubic = Recipe::new(Group::General);
        cubic.sources = vec![Source::Classic(Design::Xtree)];
        cubic.numbers = vec![3];
        cubic.levels = vec![1];
        cubic.rotations = vec![0];
        cubic.resize();
        let _ = gen::build::build_2d(&cubic);
    }),
    ("gen::build::build_3d, a recipe with no slots", || {
        let _ = gen::build::build_3d(&Recipe::new(Group::General));
    }),
    ("gen::build::build_6d, a recipe with no slots", || {
        let bare = gen::build::HexTile {
            projection: Projection::Iso,
            tile: Recipe::new(Group::General),
        };
        let _ = gen::build::build_6d(&bare);
    }),
    ("gen::name::Tile::of, a recipe with no slots", || {
        let _ = Name::of(&Recipe::new(Group::General));
    }),
    ("gen::name::Tile::from_json, an empty text", || {
        let _ = Name::from_json("");
    }),
    ("gen::name::Tile::from_json, a code past its range", || {
        let _ = Name::from_json(r#"{"kind":"tile","code":16,"side":3}"#);
    }),
    (
        "gen::variation::create, a config that draws no tile",
        || {
            let _ = gen::variation::create(&gen::variation::Config::default(), &mut Rng::new(5));
        },
    ),
    ("gen::draw::create, a config with no groups", || {
        let groupless: ConfigNd<2> = ConfigNd {
            groups: Vec::new(),
            ..Default::default()
        };
        let _ = gen::draw::create(&groupless, turn, &mut Rng::new(1));
    }),
    ("gen::build::build_3d, a flat design in the cube", || {
        let mut flat = Recipe::new(Group::General);
        flat.sources = vec![Source::Classic(Design::Htree)];
        flat.numbers = vec![3];
        flat.levels = vec![1];
        flat.rotations = vec![0];
        flat.resize();
        let _ = gen::build::build_3d(&flat);
    }),
    (
        "gen::variation::create, a config that draws no tile",
        || {
            let narrow = gen::variation::Config {
                tile: ConfigNd {
                    min_size: 9,
                    max_size: 3,
                    ..Default::default()
                },
                ..Default::default()
            };
            let _ = gen::variation::create(&narrow, &mut Rng::new(5));
        },
    ),
    ("gen::variation::create, a config with no catalog", || {
        let sourceless = gen::variation::Config {
            tile: ConfigNd {
                catalog: Catalog::Codes(Vec::new()),
                ..Default::default()
            },
            ..Default::default()
        };
        let _ = gen::variation::create(&sourceless, &mut Rng::new(5));
    }),
    (
        "gen::name::Tile::from_json, a side the design has not",
        || {
            let _ = Name::from_json(r#"{"kind":"tile","code":7,"side":99}"#);
        },
    ),
    (
        "gen::name::Tile::from_json, a text that is not json",
        || {
            let _ = Name::from_json("tile code 7, side 3, level 2");
        },
    ),
    // FROM STR
    ("math::bang::Code::from_str, an empty text", || {
        let _ = Code::from_str("");
    }),
    ("gen::recipe::Group::from_str, an empty text", || {
        let _ = Group::from_str("");
    }),
    ("gen::recipe::Parity::from_str, an empty text", || {
        let _ = mrlyrs::gen::recipe::Parity::from_str("");
    }),
    (
        "math::bang::catalog::Design::from_str, an empty text",
        || {
            let _ = Design::from_str("");
        },
    ),
    (
        "math::bang::word::Schedule::from_str, an empty text",
        || {
            let _ = math::bang::word::Schedule::from_str("");
        },
    ),
    ("core::paint::Edition::from_str, an empty text", || {
        let _ = core::paint::Edition::from_str("");
    }),
    ("core::paint::Ink::from_str, an empty text", || {
        let _ = core::paint::Ink::from_str("");
    }),
    ("core::paint::Scheme::from_str, an empty text", || {
        let _ = core::paint::Scheme::from_str("");
    }),
    ("core::paint::Target::from_str, an empty text", || {
        let _ = core::paint::Target::from_str("");
    }),
    ("num::spiral::Lattice::from_str, an empty text", || {
        let _ = num::spiral::Lattice::from_str("");
    }),
    ("num::spiral::Mark::from_str, an empty text", || {
        let _ = num::spiral::Mark::from_str("");
    }),
    ("num::spiral::Growth::from_str, an empty text", || {
        let _ = num::spiral::Growth::from_str("");
    }),
    ("num::morse::Lift::from_str, an empty text", || {
        let _ = num::morse::Lift::from_str("");
    }),
    ("life::Fate::from_str, an empty text", || {
        let _ = life::Fate::from_str("");
    }),
];

#[test]
fn no_door_panics_on_a_bad_input() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let panicked: Vec<&str> = BAD
        .iter()
        .filter(|(_, call)| catch_unwind(AssertUnwindSafe(call)).is_err())
        .map(|(name, _)| *name)
        .collect();
    std::panic::set_hook(hook);
    assert!(panicked.is_empty(), "{panicked:?}");
}
