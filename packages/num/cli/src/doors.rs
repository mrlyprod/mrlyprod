use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "num.apollonian.Circle.centre",
        signature: "(self: num.apollonian.Circle) -> (f64, f64)?",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_circle_centre),
    },
    Function {
        name: "num.apollonian.Circle.is_line",
        signature: "(self: num.apollonian.Circle) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_circle_is_line),
    },
    Function {
        name: "num.apollonian.Circle.radius",
        signature: "(self: num.apollonian.Circle) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_circle_radius),
    },
    Function {
        name: "num.apollonian.flat",
        signature: "(name: str, cap: i64) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_flat),
    },
    Function {
        name: "num.apollonian.form",
        signature: "(u: [i64; 4], v: [i64; 4]) -> i128",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_form),
    },
    Function {
        name: "num.apollonian.frame",
        signature: "(p: num.apollonian.Packing) -> [f64; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_frame),
    },
    Function {
        name: "num.apollonian.grow",
        signature: "(name: str, cap: i64) -> num.apollonian.Packing",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_grow),
    },
    Function {
        name: "num.apollonian.is_ford",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_is_ford),
    },
    Function {
        name: "num.apollonian.on_line",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_on_line),
    },
    Function {
        name: "num.apollonian.reflect",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> num.apollonian.Circle",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_reflect),
    },
    Function {
        name: "num.apollonian.root",
        signature: "(name: str) -> [num.apollonian.Circle; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_root),
    },
    Function {
        name: "num.apollonian.shadow",
        signature: "(p: num.apollonian.Packing, order: usize) -> num.apollonian.Shadow",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_shadow),
    },
    Function {
        name: "num.apollonian.sound",
        signature: "(q: [num.apollonian.Circle; 4]) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_sound),
    },
    Function {
        name: "num.apollonian.swap",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> [num.apollonian.Circle; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_swap),
    },
    Function {
        name: "num.apollonian.touches",
        signature: "(p: num.apollonian.Packing) -> [num.apollonian.Touch]",
        docs: "",
        uncallable: None,
        door: Some(door_num_apollonian_touches),
    },
    Function {
        name: "num.blend.add",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_add),
    },
    Function {
        name: "num.blend.cauchy",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_cauchy),
    },
    Function {
        name: "num.blend.characteristic",
        signature: "(coefficients: [(i128, i128)]) -> [(i128, i128)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_characteristic),
    },
    Function {
        name: "num.blend.decimate",
        signature: "(a: [i128], step: usize, offset: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_decimate),
    },
    Function {
        name: "num.blend.delta",
        signature: "(a: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_delta),
    },
    Function {
        name: "num.blend.growth",
        signature: "(coefficients: [(i128, i128)]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_growth),
    },
    Function {
        name: "num.blend.hadamard",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_hadamard),
    },
    Function {
        name: "num.blend.recurrence",
        signature: "(terms: [i128]) -> [(i128, i128)]?",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_recurrence),
    },
    Function {
        name: "num.blend.scale",
        signature: "(a: [i128], factor: i128) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_scale),
    },
    Function {
        name: "num.blend.shift",
        signature: "(a: [i128], count: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_shift),
    },
    Function {
        name: "num.blend.sigma",
        signature: "(a: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_sigma),
    },
    Function {
        name: "num.blend.sub",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_num_blend_sub),
    },
    Function {
        name: "num.boolean.is_balanced",
        signature: "(code: u128, n: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_boolean_is_balanced),
    },
    Function {
        name: "num.boolean.nonlinearity",
        signature: "(code: u128, n: usize) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_num_boolean_nonlinearity),
    },
    Function {
        name: "num.boolean.sac",
        signature: "(code: u128, n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_boolean_sac),
    },
    Function {
        name: "num.boolean.walsh_spectrum",
        signature: "(code: u128, n: usize) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_boolean_walsh_spectrum),
    },
    Function {
        name: "num.chladni.ladder",
        signature: "(cap: usize) -> [num.chladni.Rung]",
        docs: "",
        uncallable: None,
        door: Some(door_num_chladni_ladder),
    },
    Function {
        name: "num.chladni.lines",
        signature: "(field: [f64], side: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_chladni_lines),
    },
    Function {
        name: "num.chladni.plate",
        signature: "(n: usize, m: usize, sign: i32, size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_chladni_plate),
    },
    Function {
        name: "num.chladni.shake",
        signature: "(field: [f64], side: usize, grains: [f32], from: u64, steps: usize, seed: u64) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_num_chladni_shake),
    },
    Function {
        name: "num.factor.aliquot",
        signature: "(number: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_aliquot),
    },
    Function {
        name: "num.factor.coprime",
        signature: "(a: usize, b: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_coprime),
    },
    Function {
        name: "num.factor.divisors",
        signature: "(number: u64) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_divisors),
    },
    Function {
        name: "num.factor.factorial",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_factorial),
    },
    Function {
        name: "num.factor.factorize",
        signature: "(number: usize) -> [(usize, u32)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_factorize),
    },
    Function {
        name: "num.factor.factorize_wide",
        signature: "(number: u64) -> [(u64, u32)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_factorize_wide),
    },
    Function {
        name: "num.factor.gcd",
        signature: "(a: u128, b: u128) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_gcd),
    },
    Function {
        name: "num.factor.lcm",
        signature: "(a: usize, b: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_lcm),
    },
    Function {
        name: "num.factor.mobius",
        signature: "(number: usize) -> i8",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_mobius),
    },
    Function {
        name: "num.factor.mobius_sieve",
        signature: "(limit: usize) -> [i8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_mobius_sieve),
    },
    Function {
        name: "num.factor.radical",
        signature: "(number: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_radical),
    },
    Function {
        name: "num.factor.reduce",
        signature: "(numerator: u128, denominator: u128) -> (u128, u128)",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_reduce),
    },
    Function {
        name: "num.factor.sigma",
        signature: "(number: usize, power: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_sigma),
    },
    Function {
        name: "num.factor.squarefree",
        signature: "(number: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_squarefree),
    },
    Function {
        name: "num.factor.totient",
        signature: "(number: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_totient),
    },
    Function {
        name: "num.factor.totients",
        signature: "(n: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_totients),
    },
    Function {
        name: "num.factor.twisted",
        signature: "(number: usize, rhythm: [i8]) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_num_factor_twisted),
    },
    Function {
        name: "num.fft.convolve",
        signature: "(field: [f64], kernel: [f64], size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_convolve),
    },
    Function {
        name: "num.fft.convolve_with",
        signature: "(field: [f64], kernel_re: [f64], kernel_im: [f64], size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_convolve_with),
    },
    Function {
        name: "num.fft.embed_kernel",
        signature: "(mask: [u8], side: usize, size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_embed_kernel),
    },
    Function {
        name: "num.fft.log_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_log_spectrum),
    },
    Function {
        name: "num.fft.magnitude_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_magnitude_spectrum),
    },
    Function {
        name: "num.fft.peak_ring",
        signature: "(profile: [f64]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_peak_ring),
    },
    Function {
        name: "num.fft.peak_wavelength",
        signature: "(profile: [f64], size: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_peak_wavelength),
    },
    Function {
        name: "num.fft.radial_profile",
        signature: "(spectrum: [f64], size: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_radial_profile),
    },
    Function {
        name: "num.fft.transform",
        signature: "(field: [f64], size: usize) -> ([f64], [f64])",
        docs: "",
        uncallable: None,
        door: Some(door_num_fft_transform),
    },
    Function {
        name: "num.gauss.Class.prime",
        signature: "(self: num.gauss.Class) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_class_prime),
    },
    Function {
        name: "num.gauss.Class.word",
        signature: "(self: num.gauss.Class) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_class_word),
    },
    Function {
        name: "num.gauss.Ring.associates",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_associates),
    },
    Function {
        name: "num.gauss.Ring.canon",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_canon),
    },
    Function {
        name: "num.gauss.Ring.conjugate",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_conjugate),
    },
    Function {
        name: "num.gauss.Ring.count",
        signature: "(self: num.gauss.Ring, radius: u64) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_count),
    },
    Function {
        name: "num.gauss.Ring.div_rem",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> ((i64, i64), (i64, i64))",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_div_rem),
    },
    Function {
        name: "num.gauss.Ring.fate",
        signature: "(self: num.gauss.Ring, n: u64) -> num.gauss.Class",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_fate),
    },
    Function {
        name: "num.gauss.Ring.gaussian_gcd",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_gaussian_gcd),
    },
    Function {
        name: "num.gauss.Ring.inert",
        signature: "(self: num.gauss.Ring, p: u64) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_inert),
    },
    Function {
        name: "num.gauss.Ring.mul",
        signature: "(self: num.gauss.Ring, arg1: (i64, i64), arg2: (i64, i64)) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_mul),
    },
    Function {
        name: "num.gauss.Ring.named",
        signature: "(name: str) -> num.gauss.Ring?",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_named),
    },
    Function {
        name: "num.gauss.Ring.nearest",
        signature: "(self: num.gauss.Ring, x: f64, y: f64) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_nearest),
    },
    Function {
        name: "num.gauss.Ring.norm",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_norm),
    },
    Function {
        name: "num.gauss.Ring.place",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_place),
    },
    Function {
        name: "num.gauss.Ring.ramified",
        signature: "(self: num.gauss.Ring) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_ramified),
    },
    Function {
        name: "num.gauss.Ring.reach",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_reach),
    },
    Function {
        name: "num.gauss.Ring.symmetry",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_symmetry),
    },
    Function {
        name: "num.gauss.Ring.top",
        signature: "(self: num.gauss.Ring, radius: u64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_top),
    },
    Function {
        name: "num.gauss.Ring.turn",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_turn),
    },
    Function {
        name: "num.gauss.Ring.units",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_units),
    },
    Function {
        name: "num.gauss.Ring.whole",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64?",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_ring_whole),
    },
    Function {
        name: "num.gauss.Window.census",
        signature: "(self: num.gauss.Window) -> num.gauss.Census",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_census),
    },
    Function {
        name: "num.gauss.Window.class",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> num.gauss.Class",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_class),
    },
    Function {
        name: "num.gauss.Window.holds",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_holds),
    },
    Function {
        name: "num.gauss.Window.new",
        signature: "(ring: num.gauss.Ring, radius: u64) -> num.gauss.Window",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_new),
    },
    Function {
        name: "num.gauss.Window.points",
        signature: "(self: num.gauss.Window) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_points),
    },
    Function {
        name: "num.gauss.Window.radius",
        signature: "(self: num.gauss.Window) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_radius),
    },
    Function {
        name: "num.gauss.Window.ring",
        signature: "(self: num.gauss.Window) -> num.gauss.Ring",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_window_ring),
    },
    Function {
        name: "num.gauss.classes",
        signature: "(ring: num.gauss.Ring, bound: u64) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_classes),
    },
    Function {
        name: "num.gauss.peak",
        signature: "(ring: num.gauss.Ring, limit: usize) -> (usize, u32)",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_peak),
    },
    Function {
        name: "num.gauss.shells",
        signature: "(ring: num.gauss.Ring, limit: usize) -> [u32]",
        docs: "",
        uncallable: None,
        door: Some(door_num_gauss_shells),
    },
    Function {
        name: "num.kummer.carry",
        signature: "(prime: u64, top: u64, bottom: u64) -> num.kummer.Carry",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_carry),
    },
    Function {
        name: "num.kummer.columns",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> [num.kummer.Column]",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_columns),
    },
    Function {
        name: "num.kummer.count",
        signature: "(prime: u64, top: u64, bottom: u64, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_count),
    },
    Function {
        name: "num.kummer.member",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_member),
    },
    Function {
        name: "num.kummer.valuation",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_valuation),
    },
    Function {
        name: "num.kummer.witness",
        signature: "(prime: u64, top: u64, bottom: u64) -> (u64, u64)?",
        docs: "",
        uncallable: None,
        door: Some(door_num_kummer_witness),
    },
    Function {
        name: "num.lattice.coprime_pairs",
        signature: "(n: usize) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_coprime_pairs),
    },
    Function {
        name: "num.lattice.farey",
        signature: "(order: usize) -> [num.lattice.Node]",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_farey),
    },
    Function {
        name: "num.lattice.grid",
        signature: "(n: usize) -> [num.lattice.Node2d]",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_grid),
    },
    Function {
        name: "num.lattice.new_nodes",
        signature: "(n: usize) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_new_nodes),
    },
    Function {
        name: "num.lattice.pi_estimate",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_pi_estimate),
    },
    Function {
        name: "num.lattice.recovered",
        signature: "(n: usize, dimension: u32) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_recovered),
    },
    Function {
        name: "num.lattice.visible_density",
        signature: "(dimension: u32) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_visible_density),
    },
    Function {
        name: "num.lattice.zeta_factor",
        signature: "(dimension: u32) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_zeta_factor),
    },
    Function {
        name: "num.lattice.zeta_whole",
        signature: "(s: u32) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_lattice_zeta_whole),
    },
    Function {
        name: "num.morse.Lift.all",
        signature: "() -> [num.morse.Lift; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_lift_all),
    },
    Function {
        name: "num.morse.Lift.at",
        signature: "(self: num.morse.Lift, i: u64, j: u64) -> u8",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_lift_at),
    },
    Function {
        name: "num.morse.Lift.formula",
        signature: "(self: num.morse.Lift) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_lift_formula),
    },
    Function {
        name: "num.morse.boundary",
        signature: "(word: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_boundary),
    },
    Function {
        name: "num.morse.difference",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_difference),
    },
    Function {
        name: "num.morse.digits",
        signature: "(length: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_digits),
    },
    Function {
        name: "num.morse.doubling",
        signature: "(length: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_doubling),
    },
    Function {
        name: "num.morse.faults",
        signature: "(a: [u8], b: [u8]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_faults),
    },
    Function {
        name: "num.morse.fold",
        signature: "(grid: [u8], side: usize, number: usize) -> num.morse.Fold",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_fold),
    },
    Function {
        name: "num.morse.letter",
        signature: "(place: u64) -> u8",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_letter),
    },
    Function {
        name: "num.morse.lift",
        signature: "(kind: num.morse.Lift, side: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_lift),
    },
    Function {
        name: "num.morse.mask",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_mask),
    },
    Function {
        name: "num.morse.power",
        signature: "(tile: [u8], number: usize, level: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_power),
    },
    Function {
        name: "num.morse.repeat",
        signature: "(tile: [u8], number: usize, side: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_repeat),
    },
    Function {
        name: "num.morse.runs",
        signature: "(word: [u8]) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_runs),
    },
    Function {
        name: "num.morse.signs",
        signature: "(types: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_signs),
    },
    Function {
        name: "num.morse.stage",
        signature: "(rounds: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_stage),
    },
    Function {
        name: "num.morse.substitution",
        signature: "(length: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_substitution),
    },
    Function {
        name: "num.morse.upsample",
        signature: "(grid: [u8], side: usize, scale: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_morse_upsample),
    },
    Function {
        name: "num.prime.Sieve.count",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_count),
    },
    Function {
        name: "num.prime.Sieve.done",
        signature: "(self: num.prime.Sieve) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_done),
    },
    Function {
        name: "num.prime.Sieve.finish",
        signature: "(self: num.prime.Sieve) -> null",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.new",
        signature: "(limit: usize) -> num.prime.Sieve",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_new),
    },
    Function {
        name: "num.prime.Sieve.rank",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_rank),
    },
    Function {
        name: "num.prime.Sieve.step",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.struck",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_struck),
    },
    Function {
        name: "num.prime.Sieve.types",
        signature: "(self: num.prime.Sieve) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_sieve_types),
    },
    Function {
        name: "num.prime.chart",
        signature: "(top: usize, bins: usize) -> [num.prime.Reading]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_chart),
    },
    Function {
        name: "num.prime.flags",
        signature: "(limit: usize) -> [bool]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_flags),
    },
    Function {
        name: "num.prime.goldbach",
        signature: "(number: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_goldbach),
    },
    Function {
        name: "num.prime.goldbach_record",
        signature: "(top: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_goldbach_record),
    },
    Function {
        name: "num.prime.is_prime",
        signature: "(number: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_is_prime),
    },
    Function {
        name: "num.prime.pile",
        signature: "(number: u64) -> num.prime.Pile",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_pile),
    },
    Function {
        name: "num.prime.prime_count",
        signature: "(n: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_prime_count),
    },
    Function {
        name: "num.prime.prime_from",
        signature: "(number: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_prime_from),
    },
    Function {
        name: "num.prime.primes",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_primes),
    },
    Function {
        name: "num.prime.rectangles",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_rectangles),
    },
    Function {
        name: "num.prime.splits",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_splits),
    },
    Function {
        name: "num.prime.squares",
        signature: "(number: usize) -> (usize, usize)?",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_squares),
    },
    Function {
        name: "num.prime.study",
        signature: "(limit: usize) -> [num.prime.Prime]",
        docs: "",
        uncallable: None,
        door: Some(door_num_prime_study),
    },
    Function {
        name: "num.readings.Design.digits",
        signature: "(base: u8, mask: u8) -> num.readings.Design",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_design_digits),
    },
    Function {
        name: "num.readings.Design.rudin",
        signature: "() -> num.readings.Design",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_design_rudin),
    },
    Function {
        name: "num.readings.Design.ternary",
        signature: "() -> num.readings.Design",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_design_ternary),
    },
    Function {
        name: "num.readings.Design.thue",
        signature: "() -> num.readings.Design",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_design_thue),
    },
    Function {
        name: "num.readings.Reading.all",
        signature: "() -> [num.readings.Reading; 5]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_reading_all),
    },
    Function {
        name: "num.readings.Reading.formula",
        signature: "(self: num.readings.Reading) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_reading_formula),
    },
    Function {
        name: "num.readings.arcs",
        signature: "(design: num.readings.Design, cut: f64) -> [num.readings.Arc]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_arcs),
    },
    Function {
        name: "num.readings.battery",
        signature: "(stream: [i8]) -> num.readings.Battery",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_battery),
    },
    Function {
        name: "num.readings.bias",
        signature: "(stream: [i8], exponents: [u32]) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_bias),
    },
    Function {
        name: "num.readings.designs",
        signature: "() -> [num.readings.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_designs),
    },
    Function {
        name: "num.readings.model",
        signature: "(design: num.readings.Design, cut: f64, exponents: [u32]) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_model),
    },
    Function {
        name: "num.readings.name",
        signature: "(design: num.readings.Design) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_name),
    },
    Function {
        name: "num.readings.near_heads",
        signature: "(design: num.readings.Design, reading: num.readings.Reading) -> [num.readings.Head]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_near_heads),
    },
    Function {
        name: "num.readings.sign",
        signature: "(design: num.readings.Design, n: u64) -> i8",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_sign),
    },
    Function {
        name: "num.readings.stream",
        signature: "(design: num.readings.Design, reading: num.readings.Reading, count: u32) -> [i8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_readings_stream),
    },
    Function {
        name: "num.series.basel",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_basel),
    },
    Function {
        name: "num.series.bernoulli",
        signature: "(count: usize) -> [(i128, i128)]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_bernoulli),
    },
    Function {
        name: "num.series.beta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_beta),
    },
    Function {
        name: "num.series.binary",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_binary),
    },
    Function {
        name: "num.series.catalan",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_catalan),
    },
    Function {
        name: "num.series.chi3",
        signature: "(number: usize) -> i8",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_chi3),
    },
    Function {
        name: "num.series.chi4",
        signature: "(number: usize) -> i8",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_chi4),
    },
    Function {
        name: "num.series.chi8",
        signature: "(number: usize) -> i8",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_chi8),
    },
    Function {
        name: "num.series.dirichlet",
        signature: "(s: f64, rhythm: [i8], terms: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_dirichlet),
    },
    Function {
        name: "num.series.e_partial",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_e_partial),
    },
    Function {
        name: "num.series.euler_gamma_partial",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_euler_gamma_partial),
    },
    Function {
        name: "num.series.euler_product",
        signature: "(s: f64, limit: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_euler_product),
    },
    Function {
        name: "num.series.evens",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_evens),
    },
    Function {
        name: "num.series.fibonacci",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_fibonacci),
    },
    Function {
        name: "num.series.harmonic",
        signature: "(terms: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_harmonic),
    },
    Function {
        name: "num.series.lambda",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_lambda),
    },
    Function {
        name: "num.series.leibniz",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_leibniz),
    },
    Function {
        name: "num.series.li",
        signature: "(x: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_li),
    },
    Function {
        name: "num.series.mertens",
        signature: "(n: usize) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_mertens),
    },
    Function {
        name: "num.series.odds",
        signature: "(limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_odds),
    },
    Function {
        name: "num.series.visible",
        signature: "(limit: usize, dimension: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_visible),
    },
    Function {
        name: "num.series.wallis_half_pi",
        signature: "(n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_wallis_half_pi),
    },
    Function {
        name: "num.series.wallis_quarter_pi",
        signature: "(factors: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_wallis_quarter_pi),
    },
    Function {
        name: "num.series.zeta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_series_zeta),
    },
    Function {
        name: "num.sieve.cells",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_cells),
    },
    Function {
        name: "num.sieve.exponent",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_exponent),
    },
    Function {
        name: "num.sieve.flat_word",
        signature: "(side: u64, levels: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_flat_word),
    },
    Function {
        name: "num.sieve.holes",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_holes),
    },
    Function {
        name: "num.sieve.limit",
        signature: "(word: [u64], dimension: u32) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_limit),
    },
    Function {
        name: "num.sieve.odd_word",
        signature: "(levels: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_odd_word),
    },
    Function {
        name: "num.sieve.parity_product",
        signature: "(dimension: u32, odd: bool) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_parity_product),
    },
    Function {
        name: "num.sieve.punctures",
        signature: "(word: [u64], dimension: u32) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_punctures),
    },
    Function {
        name: "num.sieve.raster",
        signature: "(word: [u64]) -> (usize, [u8])",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_raster),
    },
    Function {
        name: "num.sieve.ratio",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_ratio),
    },
    Function {
        name: "num.sieve.row_fill",
        signature: "(profile: [u64], side: u64) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_row_fill),
    },
    Function {
        name: "num.sieve.row_law",
        signature: "(profile: [u64]) -> num.sieve.Row",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_row_law),
    },
    Function {
        name: "num.sieve.row_profile",
        signature: "(code: u128, dimension: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_row_profile),
    },
    Function {
        name: "num.sieve.row_settle",
        signature: "(profile: [u64], stops: [usize], even: bool) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_row_settle),
    },
    Function {
        name: "num.sieve.side",
        signature: "(word: [u64]) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_side),
    },
    Function {
        name: "num.sieve.solid_limit",
        signature: "() -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_sieve_solid_limit),
    },
    Function {
        name: "num.spiral.Growth.all",
        signature: "() -> [num.spiral.Growth; 2]",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_growth_all),
    },
    Function {
        name: "num.spiral.Lattice.all",
        signature: "() -> [num.spiral.Lattice; 2]",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_all),
    },
    Function {
        name: "num.spiral.Lattice.count",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_count),
    },
    Function {
        name: "num.spiral.Lattice.n",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_n),
    },
    Function {
        name: "num.spiral.Lattice.radius",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_radius),
    },
    Function {
        name: "num.spiral.Lattice.ring",
        signature: "(self: num.spiral.Lattice, n: u64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring),
    },
    Function {
        name: "num.spiral.Lattice.ring_of",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring_of),
    },
    Function {
        name: "num.spiral.Lattice.xy",
        signature: "(self: num.spiral.Lattice, n: u64) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_lattice_xy),
    },
    Function {
        name: "num.spiral.Mark.all",
        signature: "() -> [num.spiral.Mark; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_mark_all),
    },
    Function {
        name: "num.spiral.diagonal",
        signature: "(lattice: num.spiral.Lattice, side: usize, a: i64, b: i64, c: i64) -> num.spiral.Diagonal",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_diagonal),
    },
    Function {
        name: "num.spiral.level_of",
        signature: "(n: u64, base: u64) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_level_of),
    },
    Function {
        name: "num.spiral.marks",
        signature: "(mark: num.spiral.Mark, limit: usize) -> [i8]",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_marks),
    },
    Function {
        name: "num.spiral.snail",
        signature: "(base: u64, top: u64, growth: num.spiral.Growth) -> num.spiral.Snail",
        docs: "",
        uncallable: None,
        door: Some(door_num_spiral_snail),
    },
    Function {
        name: "num.zeta.Complex.abs",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_abs),
    },
    Function {
        name: "num.zeta.Complex.arg",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_arg),
    },
    Function {
        name: "num.zeta.Complex.default",
        signature: "() -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_default),
    },
    Function {
        name: "num.zeta.Complex.exp",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_exp),
    },
    Function {
        name: "num.zeta.Complex.ln",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_ln),
    },
    Function {
        name: "num.zeta.Complex.new",
        signature: "(re: f64, im: f64) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_new),
    },
    Function {
        name: "num.zeta.Complex.turn",
        signature: "(angle: f64) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_complex_turn),
    },
    Function {
        name: "num.zeta.Line.count",
        signature: "(self: num.zeta.Line, t: f64) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_count),
    },
    Function {
        name: "num.zeta.Line.default",
        signature: "() -> num.zeta.Line",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_default),
    },
    Function {
        name: "num.zeta.Line.exact",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_exact),
    },
    Function {
        name: "num.zeta.Line.gram",
        signature: "(self: num.zeta.Line, n: i64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_gram),
    },
    Function {
        name: "num.zeta.Line.maclaurin",
        signature: "(self: num.zeta.Line, t: f64) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_maclaurin),
    },
    Function {
        name: "num.zeta.Line.new",
        signature: "() -> num.zeta.Line",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_new),
    },
    Function {
        name: "num.zeta.Line.novelty_coefficients",
        signature: "(self: num.zeta.Line, gammas: [f64]) -> [num.zeta.Complex]",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_novelty_coefficients),
    },
    Function {
        name: "num.zeta.Line.pair",
        signature: "(self: num.zeta.Line, s: num.zeta.Complex) -> (num.zeta.Complex, num.zeta.Complex)",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_pair),
    },
    Function {
        name: "num.zeta.Line.point",
        signature: "(self: num.zeta.Line, t: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_point),
    },
    Function {
        name: "num.zeta.Line.seam",
        signature: "(self: num.zeta.Line, t0: f64, t1: f64, steps: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_seam),
    },
    Function {
        name: "num.zeta.Line.siegel",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_siegel),
    },
    Function {
        name: "num.zeta.Line.theta",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_theta),
    },
    Function {
        name: "num.zeta.Line.z",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_z),
    },
    Function {
        name: "num.zeta.Line.zeros",
        signature: "(self: num.zeta.Line, count: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_line_zeros),
    },
    Function {
        name: "num.zeta.bump",
        signature: "(u: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_bump),
    },
    Function {
        name: "num.zeta.corrections",
        signature: "(p: f64) -> [f64; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_corrections),
    },
    Function {
        name: "num.zeta.kernel",
        signature: "(p: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_kernel),
    },
    Function {
        name: "num.zeta.mellin",
        signature: "(s: num.zeta.Complex) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_mellin),
    },
    Function {
        name: "num.zeta.novelty_main",
        signature: "() -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_novelty_main),
    },
    Function {
        name: "num.zeta.novelty_wave",
        signature: "(gammas: [f64], coef: [num.zeta.Complex], log_y: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_novelty_wave),
    },
    Function {
        name: "num.zeta.psi_curve",
        signature: "(gammas: [f64], samples: [f64]) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_psi_curve),
    },
    Function {
        name: "num.zeta.psi_formula",
        signature: "(x: f64, gammas: [f64]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_psi_formula),
    },
    Function {
        name: "num.zeta.psi_gap",
        signature: "(x: usize, gammas: [f64]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_psi_gap),
    },
    Function {
        name: "num.zeta.psi_stair",
        signature: "(x: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_psi_stair),
    },
    Function {
        name: "num.zeta.raise",
        signature: "(base: f64, exponent: num.zeta.Complex) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_raise),
    },
    Function {
        name: "num.zeta.sharp_novelty",
        signature: "(prefix: [u64], y: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_sharp_novelty),
    },
    Function {
        name: "num.zeta.smoothed_novelty",
        signature: "(phi: [u64], y: f64, main: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_num_zeta_smoothed_novelty),
    },
];

fn door_num_apollonian_circle_centre(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::centre(a0)))
}

fn door_num_apollonian_circle_is_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::is_line(a0)))
}

fn door_num_apollonian_circle_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::radius(a0)))
}

fn door_num_apollonian_flat(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_num::apollonian::flat(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_form(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [i64; 4] = take!(name, 0, &args[0]);
    let a1: [i64; 4] = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_num::apollonian::form(a0, a1).to_string(),
    ))
}

fn door_num_apollonian_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::frame(&a0)))
}

fn door_num_apollonian_grow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_num::apollonian::grow(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_is_ford(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::is_ford(a0)))
}

fn door_num_apollonian_on_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::on_line(a0)))
}

fn door_num_apollonian_reflect(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::apollonian::reflect(&a0, a1)))
}

fn door_num_apollonian_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_num::apollonian::root(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::apollonian::shadow(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_sound(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::sound(&a0)))
}

fn door_num_apollonian_swap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::apollonian::swap(&a0, a1)))
}

fn door_num_apollonian_touches(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::touches(&a0)))
}

fn door_num_blend_add(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::add(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_cauchy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    match mrly_num::blend::cauchy(&a0, &a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::characteristic(&a0) {
            list0.push({
                let parts1 = item0;
                Value::Array(vec![
                    Value::String(parts1.0.to_string()),
                    Value::String(parts1.1.to_string()),
                ])
            });
        }
        Value::Array(list0)
    })
}

fn door_num_blend_decimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::blend::decimate(&a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_delta(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::delta(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_growth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok(give!(mrly_num::blend::growth(&a0)))
}

fn door_num_blend_hadamard(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::hadamard(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok(match mrly_num::blend::recurrence(&a0) {
        Some(item0) => {
            let mut list1 = Vec::new();
            for item1 in item0 {
                list1.push({
                    let parts2 = item1;
                    Value::Array(vec![
                        Value::String(parts2.0.to_string()),
                        Value::String(parts2.1.to_string()),
                    ])
                });
            }
            Value::Array(list1)
        }
        None => Value::Null,
    })
}

fn door_num_blend_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: i128 = small(name, 1, &args[1])?;
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::scale(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_shift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::shift(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    match mrly_num::blend::sigma(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_sub(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::sub(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_boolean_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::is_balanced(a0, a1)))
}

fn door_num_boolean_nonlinearity(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::nonlinearity(a0, a1)))
}

fn door_num_boolean_sac(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::sac(a0, a1)))
}

fn door_num_boolean_walsh_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::walsh_spectrum(a0, a1)))
}

fn door_num_chladni_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::chladni::ladder(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_lines(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::chladni::lines(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_plate(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_num::chladni::plate(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_shake(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f32> = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: u64 = take!(name, 5, &args[5]);
    match mrly_num::chladni::shake(&a0, a1, &a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_aliquot(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::aliquot(a0)))
}

fn door_num_factor_coprime(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::coprime(a0, a1)))
}

fn door_num_factor_divisors(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::divisors(a0)))
}

fn door_num_factor_factorial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::factor::factorial(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_factorize(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::factorize(a0)))
}

fn door_num_factor_factorize_wide(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::factorize_wide(a0)))
}

fn door_num_factor_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(Value::String(mrly_num::factor::gcd(a0, a1).to_string()))
}

fn door_num_factor_lcm(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::lcm(a0, a1)))
}

fn door_num_factor_mobius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::mobius(a0)))
}

fn door_num_factor_mobius_sieve(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::mobius_sieve(a0)))
}

fn door_num_factor_radical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::radical(a0)))
}

fn door_num_factor_reduce(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok({
        let parts0 = mrly_num::factor::reduce(a0, a1);
        Value::Array(vec![
            Value::String(parts0.0.to_string()),
            Value::String(parts0.1.to_string()),
        ])
    })
}

fn door_num_factor_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(mrly_num::factor::sigma(a0, a1).to_string()))
}

fn door_num_factor_squarefree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::squarefree(a0)))
}

fn door_num_factor_totient(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::totient(a0)))
}

fn door_num_factor_totients(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::totients(a0)))
}

fn door_num_factor_twisted(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::twisted(a0, &a1)))
}

fn door_num_fft_convolve(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::fft::convolve(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_convolve_with(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_num::fft::convolve_with(&a0, &a1, &a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_embed_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::fft::embed_kernel(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_log_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::log_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_magnitude_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::magnitude_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_peak_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::fft::peak_ring(&a0)))
}

fn door_num_fft_peak_wavelength(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::fft::peak_wavelength(&a0, a1)))
}

fn door_num_fft_radial_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::radial_profile(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_transform(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::transform(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_gauss_class_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Class::prime(a0)))
}

fn door_num_gauss_class_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Class::word(a0)))
}

fn door_num_gauss_ring_associates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::associates(a0, a1, a2)))
}

fn door_num_gauss_ring_canon(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::canon(a0, a1, a2)))
}

fn door_num_gauss_ring_conjugate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::conjugate(a0, a1, a2)))
}

fn door_num_gauss_ring_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::count(a0, a1)))
}

fn door_num_gauss_ring_div_rem(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::div_rem(a0, a1, a2)))
}

fn door_num_gauss_ring_fate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::fate(a0, a1)))
}

fn door_num_gauss_ring_gaussian_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::gaussian_gcd(a0, a1, a2)))
}

fn door_num_gauss_ring_inert(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::inert(a0, a1)))
}

fn door_num_gauss_ring_mul(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::mul(a0, a1, a2)))
}

fn door_num_gauss_ring_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::named(a0.as_str())))
}

fn door_num_gauss_ring_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::nearest(a0, a1, a2)))
}

fn door_num_gauss_ring_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::norm(a0, a1, a2)))
}

fn door_num_gauss_ring_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::place(a0, a1, a2)))
}

fn door_num_gauss_ring_ramified(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::ramified(a0)))
}

fn door_num_gauss_ring_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::reach(a0, a1, a2)))
}

fn door_num_gauss_ring_symmetry(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::symmetry(a0)))
}

fn door_num_gauss_ring_top(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::top(a0, a1)))
}

fn door_num_gauss_ring_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::turn(a0, a1, a2)))
}

fn door_num_gauss_ring_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::units(a0)))
}

fn door_num_gauss_ring_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::whole(a0, a1, a2)))
}

fn door_num_gauss_window_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::census(&a0)))
}

fn door_num_gauss_window_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Window::class(&a0, a1, a2)))
}

fn door_num_gauss_window_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Window::holds(&a0, a1, a2)))
}

fn door_num_gauss_window_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Window::new(a0, a1)))
}

fn door_num_gauss_window_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::points(&a0)))
}

fn door_num_gauss_window_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::radius(&a0)))
}

fn door_num_gauss_window_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::ring(&a0)))
}

fn door_num_gauss_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::classes(a0, a1)))
}

fn door_num_gauss_peak(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::peak(a0, a1)))
}

fn door_num_gauss_shells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::shells(a0, a1)))
}

fn door_num_kummer_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_num::kummer::carry(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_columns(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::columns(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    match mrly_num::kummer::count(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_valuation(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::valuation(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_num::kummer::witness(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_coprime_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::coprime_pairs(a0)))
}

fn door_num_lattice_farey(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::farey(a0)))
}

fn door_num_lattice_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::grid(a0)))
}

fn door_num_lattice_new_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::new_nodes(a0)))
}

fn door_num_lattice_pi_estimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::pi_estimate(a0)))
}

fn door_num_lattice_recovered(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::lattice::recovered(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_visible_density(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_num::lattice::visible_density(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_zeta_factor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::zeta_factor(a0)))
}

fn door_num_lattice_zeta_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_num::lattice::zeta_whole(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_lift_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::morse::Lift::all()))
}

fn door_num_morse_lift_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::Lift::at(a0, a1, a2)))
}

fn door_num_morse_lift_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::Lift::formula(a0)))
}

fn door_num_morse_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::boundary(&a0)))
}

fn door_num_morse_difference(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::difference(&a0, &a1)))
}

fn door_num_morse_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::digits(a0)))
}

fn door_num_morse_doubling(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::doubling(a0)))
}

fn door_num_morse_faults(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::faults(&a0, &a1)))
}

fn door_num_morse_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::morse::fold(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::letter(a0)))
}

fn door_num_morse_lift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::lift(a0, a1)))
}

fn door_num_morse_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::mask(&a0, &a1)))
}

fn door_num_morse_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::morse::power(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_repeat(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::repeat(&a0, a1, a2)))
}

fn door_num_morse_runs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::runs(&a0)))
}

fn door_num_morse_signs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::signs(&a0)))
}

fn door_num_morse_stage(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::stage(a0)))
}

fn door_num_morse_substitution(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::substitution(a0)))
}

fn door_num_morse_upsample(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::upsample(&a0, a1, a2)))
}

fn door_num_prime_sieve_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::count(&a0)))
}

fn door_num_prime_sieve_done(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::done(&a0)))
}

fn door_num_prime_sieve_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::new(a0)))
}

fn door_num_prime_sieve_rank(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::rank(&a0)))
}

fn door_num_prime_sieve_struck(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::struck(&a0)))
}

fn door_num_prime_sieve_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::types(&a0)))
}

fn door_num_prime_chart(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::prime::chart(a0, a1)))
}

fn door_num_prime_flags(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::flags(a0)))
}

fn door_num_prime_goldbach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::goldbach(a0)))
}

fn door_num_prime_goldbach_record(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::goldbach_record(a0)))
}

fn door_num_prime_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::is_prime(a0)))
}

fn door_num_prime_pile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::pile(a0)))
}

fn door_num_prime_prime_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::prime_count(a0)))
}

fn door_num_prime_prime_from(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::prime_from(a0)))
}

fn door_num_prime_primes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::primes(a0)))
}

fn door_num_prime_rectangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::rectangles(a0)))
}

fn door_num_prime_splits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::splits(a0)))
}

fn door_num_prime_squares(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::squares(a0)))
}

fn door_num_prime_study(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::study(a0)))
}

fn door_num_readings_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::Design::digits(a0, a1)))
}

fn door_num_readings_design_rudin(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::rudin()))
}

fn door_num_readings_design_ternary(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::ternary()))
}

fn door_num_readings_design_thue(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::thue()))
}

fn door_num_readings_reading_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Reading::all()))
}

fn door_num_readings_reading_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::readings::Reading = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::Reading::formula(a0)))
}

fn door_num_readings_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::arcs(a0, a1)))
}

fn door_num_readings_battery(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::battery(&a0)))
}

fn door_num_readings_bias(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    let a1: Vec<u32> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::bias(&a0, &a1)))
}

fn door_num_readings_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::designs()))
}

fn door_num_readings_model(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: Vec<u32> = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::readings::model(a0, a1, &a2)))
}

fn door_num_readings_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::name(a0)))
}

fn door_num_readings_near_heads(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::readings::Reading = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::near_heads(a0, a1)))
}

fn door_num_readings_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::sign(a0, a1)))
}

fn door_num_readings_stream(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::readings::Reading = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::readings::stream(a0, a1, a2)))
}

fn door_num_series_basel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::basel(a0)))
}

fn door_num_series_bernoulli(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::series::bernoulli(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.to_string()),
                        Value::String(parts1.1.to_string()),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_beta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::series::beta(a0, a1)))
}

fn door_num_series_binary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::binary(a0)))
}

fn door_num_series_catalan(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::catalan(a0)))
}

fn door_num_series_chi3(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi3(a0)))
}

fn door_num_series_chi4(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi4(a0)))
}

fn door_num_series_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi8(a0)))
}

fn door_num_series_dirichlet(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::series::dirichlet(a0, &a1, a2)))
}

fn door_num_series_e_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::e_partial(a0)))
}

fn door_num_series_euler_gamma_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::euler_gamma_partial(a0)))
}

fn door_num_series_euler_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::series::euler_product(a0, a1)))
}

fn door_num_series_evens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::evens(a0)))
}

fn door_num_series_fibonacci(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::fibonacci(a0)))
}

fn door_num_series_harmonic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::harmonic(a0)))
}

fn door_num_series_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::series::lambda(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_leibniz(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::leibniz(a0)))
}

fn door_num_series_li(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::li(a0)))
}

fn door_num_series_mertens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::mertens(a0)))
}

fn door_num_series_odds(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::odds(a0)))
}

fn door_num_series_visible(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::series::visible(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_wallis_half_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::wallis_half_pi(a0)))
}

fn door_num_series_wallis_quarter_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::wallis_quarter_pi(a0)))
}

fn door_num_series_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::series::zeta(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::cells(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_flat_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::sieve::flat_word(a0, a1)))
}

fn door_num_sieve_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::holes(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::limit(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_odd_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::sieve::odd_word(a0)))
}

fn door_num_sieve_parity_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_num::sieve::parity_product(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_punctures(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::punctures(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::raster(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::ratio(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    match mrly_num::sieve::row_fill(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::row_law(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::sieve::row_profile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_settle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_num::sieve::row_settle(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_solid_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::sieve::solid_limit()))
}

fn door_num_spiral_growth_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Growth::all()))
}

fn door_num_spiral_lattice_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Lattice::all()))
}

fn door_num_spiral_lattice_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::count(a0, a1)))
}

fn door_num_spiral_lattice_n(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::Lattice::n(a0, a1, a2)))
}

fn door_num_spiral_lattice_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::radius(a0, a1)))
}

fn door_num_spiral_lattice_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::ring(a0, a1)))
}

fn door_num_spiral_lattice_ring_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::Lattice::ring_of(a0, a1, a2)))
}

fn door_num_spiral_lattice_xy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::xy(a0, a1)))
}

fn door_num_spiral_mark_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Mark::all()))
}

fn door_num_spiral_diagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    let a4: i64 = take!(name, 4, &args[4]);
    Ok(give!(mrly_num::spiral::diagonal(a0, a1, a2, a3, a4)))
}

fn door_num_spiral_level_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::level_of(a0, a1)))
}

fn door_num_spiral_marks(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Mark = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::marks(a0, a1)))
}

fn door_num_spiral_snail(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: mrly_num::spiral::Growth = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::snail(a0, a1, a2)))
}

fn door_num_zeta_complex_abs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::abs(a0)))
}

fn door_num_zeta_complex_arg(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::arg(a0)))
}

fn door_num_zeta_complex_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Complex::default()))
}

fn door_num_zeta_complex_exp(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::exp(a0)))
}

fn door_num_zeta_complex_ln(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::ln(a0)))
}

fn door_num_zeta_complex_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Complex::new(a0, a1)))
}

fn door_num_zeta_complex_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::turn(a0)))
}

fn door_num_zeta_line_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::count(&a0, a1)))
}

fn door_num_zeta_line_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Line::default()))
}

fn door_num_zeta_line_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::exact(&a0, a1)))
}

fn door_num_zeta_line_gram(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::gram(&a0, a1)))
}

fn door_num_zeta_line_maclaurin(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::maclaurin(&a0, a1)))
}

fn door_num_zeta_line_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Line::new()))
}

fn door_num_zeta_line_novelty_coefficients(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::novelty_coefficients(&a0, &a1)))
}

fn door_num_zeta_line_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::pair(&a0, a1)))
}

fn door_num_zeta_line_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::point(&a0, a1)))
}

fn door_num_zeta_line_seam(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_num::zeta::Line::seam(&a0, a1, a2, a3)))
}

fn door_num_zeta_line_siegel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::siegel(&a0, a1)))
}

fn door_num_zeta_line_theta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::theta(&a0, a1)))
}

fn door_num_zeta_line_z(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::z(&a0, a1)))
}

fn door_num_zeta_line_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::zeros(&a0, a1)))
}

fn door_num_zeta_bump(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::bump(a0)))
}

fn door_num_zeta_corrections(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::corrections(a0)))
}

fn door_num_zeta_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::kernel(a0)))
}

fn door_num_zeta_mellin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::mellin(a0)))
}

fn door_num_zeta_novelty_main(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::novelty_main()))
}

fn door_num_zeta_novelty_wave(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<mrly_num::zeta::Complex> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::zeta::novelty_wave(&a0, &a1, a2)))
}

fn door_num_zeta_psi_curve(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_curve(&a0, &a1)))
}

fn door_num_zeta_psi_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_formula(a0, &a1)))
}

fn door_num_zeta_psi_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_gap(a0, &a1)))
}

fn door_num_zeta_psi_stair(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::psi_stair(a0)))
}

fn door_num_zeta_raise(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::raise(a0, a1)))
}

fn door_num_zeta_sharp_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::sharp_novelty(&a0, a1)))
}

fn door_num_zeta_smoothed_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::zeta::smoothed_novelty(&a0, a1, a2)))
}
