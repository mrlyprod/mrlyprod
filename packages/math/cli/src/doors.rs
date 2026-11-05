use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "math.arcs.curves",
        signature: "(code: Code, base: usize, level: usize) -> [math.arcs.Curve]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arcs_curves),
    },
    Function {
        name: "math.arcs.draw",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Arcs",
        docs: "",
        uncallable: None,
        door: Some(door_math_arcs_draw),
    },
    Function {
        name: "math.arcs.law",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Law?",
        docs: "",
        uncallable: None,
        door: Some(door_math_arcs_law),
    },
    Function {
        name: "math.arcs.trace",
        signature: "(side: usize, on: [bool]) -> math.arcs.Arcs",
        docs: "",
        uncallable: None,
        door: Some(door_math_arcs_trace),
    },
    Function {
        name: "math.arrays.essential",
        signature: "(sensors: [u64]) -> [bool]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_essential),
    },
    Function {
        name: "math.arrays.fractal",
        signature: "(generator: [u64], base: u64, level: u32) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_fractal),
    },
    Function {
        name: "math.arrays.holes",
        signature: "(sensors: [u64]) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_holes),
    },
    Function {
        name: "math.arrays.law",
        signature: "(generator: [u64], level: u32) -> (u64, u64)?",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_law),
    },
    Function {
        name: "math.arrays.lost",
        signature: "(sensors: [u64], sensor: u64) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_lost),
    },
    Function {
        name: "math.arrays.paired",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_paired),
    },
    Function {
        name: "math.arrays.weights",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_arrays_weights),
    },
    Function {
        name: "math.atoms.carpet_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_carpet_2d),
    },
    Function {
        name: "math.atoms.carpet_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_carpet_3d),
    },
    Function {
        name: "math.atoms.carpet_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_carpet_nd),
    },
    Function {
        name: "math.atoms.dust_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_dust_2d),
    },
    Function {
        name: "math.atoms.dust_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_dust_3d),
    },
    Function {
        name: "math.atoms.dust_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_dust_nd),
    },
    Function {
        name: "math.atoms.hline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_hline_2d),
    },
    Function {
        name: "math.atoms.htree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_htree_2d),
    },
    Function {
        name: "math.atoms.line_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_line_nd),
    },
    Function {
        name: "math.atoms.net_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_net_2d),
    },
    Function {
        name: "math.atoms.net_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_net_3d),
    },
    Function {
        name: "math.atoms.net_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_net_nd),
    },
    Function {
        name: "math.atoms.noise_2d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_noise_2d),
    },
    Function {
        name: "math.atoms.noise_3d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_noise_3d),
    },
    Function {
        name: "math.atoms.ones_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_ones_2d),
    },
    Function {
        name: "math.atoms.ones_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_ones_3d),
    },
    Function {
        name: "math.atoms.point_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_point_2d),
    },
    Function {
        name: "math.atoms.point_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_point_3d),
    },
    Function {
        name: "math.atoms.point_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_point_nd),
    },
    Function {
        name: "math.atoms.star_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_star_2d),
    },
    Function {
        name: "math.atoms.star_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_star_3d),
    },
    Function {
        name: "math.atoms.star_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_star_nd),
    },
    Function {
        name: "math.atoms.tree_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_tree_nd),
    },
    Function {
        name: "math.atoms.vline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_vline_2d),
    },
    Function {
        name: "math.atoms.void_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_void_2d),
    },
    Function {
        name: "math.atoms.void_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_void_3d),
    },
    Function {
        name: "math.atoms.void_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_void_nd),
    },
    Function {
        name: "math.atoms.vtree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_vtree_2d),
    },
    Function {
        name: "math.atoms.xline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_xline_3d),
    },
    Function {
        name: "math.atoms.xtree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_xtree_3d),
    },
    Function {
        name: "math.atoms.yline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_yline_3d),
    },
    Function {
        name: "math.atoms.ytree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_ytree_3d),
    },
    Function {
        name: "math.atoms.zeros_2d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_zeros_2d),
    },
    Function {
        name: "math.atoms.zeros_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_zeros_3d),
    },
    Function {
        name: "math.atoms.zline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_zline_3d),
    },
    Function {
        name: "math.atoms.ztree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_atoms_ztree_3d),
    },
    Function {
        name: "math.automaton.Automaton.abscissa",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_abscissa),
    },
    Function {
        name: "math.automaton.Automaton.base",
        signature: "(self: math.automaton.Automaton) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_base),
    },
    Function {
        name: "math.automaton.Automaton.cofactor",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_cofactor),
    },
    Function {
        name: "math.automaton.Automaton.denominator",
        signature: "(self: math.automaton.Automaton) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_denominator),
    },
    Function {
        name: "math.automaton.Automaton.matrix",
        signature: "(self: math.automaton.Automaton) -> [[f64]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_matrix),
    },
    Function {
        name: "math.automaton.Automaton.new",
        signature: "(rule: math.memory.Rule) -> math.automaton.Automaton",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_new),
    },
    Function {
        name: "math.automaton.Automaton.peel",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_peel),
    },
    Function {
        name: "math.automaton.Automaton.period",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_period),
    },
    Function {
        name: "math.automaton.Automaton.perron",
        signature: "(self: math.automaton.Automaton) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_perron),
    },
    Function {
        name: "math.automaton.Automaton.residue",
        signature: "(self: math.automaton.Automaton, w0: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_residue),
    },
    Function {
        name: "math.automaton.Automaton.rule",
        signature: "(self: math.automaton.Automaton) -> math.memory.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_rule),
    },
    Function {
        name: "math.automaton.Automaton.states",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_states),
    },
    Function {
        name: "math.automaton.Automaton.with_peel",
        signature: "(rule: math.memory.Rule, peel: usize) -> math.automaton.Automaton",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_with_peel),
    },
    Function {
        name: "math.automaton.Automaton.zeta",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_automaton_automaton_zeta),
    },
    Function {
        name: "math.bang.Design.anf",
        signature: "(self: math.bang.Design) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_design_anf),
    },
    Function {
        name: "math.bang.Design.degree",
        signature: "(self: math.bang.Design) -> i32",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_design_degree),
    },
    Function {
        name: "math.bang.Design.name",
        signature: "(self: math.bang.Design) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_design_name),
    },
    Function {
        name: "math.bang.Design.rule",
        signature: "(self: math.bang.Design) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_design_rule),
    },
    Function {
        name: "math.bang.MagicLayer.new",
        signature: "(design: math.name.Bang, number: usize) -> math.bang.MagicLayer",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_magiclayer_new),
    },
    Function {
        name: "math.bang.Universe.all",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_all),
    },
    Function {
        name: "math.bang.Universe.canonical",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_canonical),
    },
    Function {
        name: "math.bang.Universe.design",
        signature: "(self: math.bang.Universe, code: Code) -> math.bang.Design",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_design),
    },
    Function {
        name: "math.bang.Universe.distinct",
        signature: "(self: math.bang.Universe) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_distinct),
    },
    Function {
        name: "math.bang.Universe.new",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_new),
    },
    Function {
        name: "math.bang.bang",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_bang),
    },
    Function {
        name: "math.bang.baseq.axis_maps",
        signature: "(base: usize) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_axis_maps),
    },
    Function {
        name: "math.bang.baseq.bracelets",
        signature: "(max_base: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_bracelets),
    },
    Function {
        name: "math.bang.baseq.canonical",
        signature: "(group: [[usize]], code: Code) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_canonical),
    },
    Function {
        name: "math.bang.baseq.carry",
        signature: "(element: [usize], code: Code) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_carry),
    },
    Function {
        name: "math.bang.baseq.class_sequence",
        signature: "(max_dimension: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_class_sequence),
    },
    Function {
        name: "math.bang.baseq.classes",
        signature: "(dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_classes),
    },
    Function {
        name: "math.bang.baseq.distinct_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_distinct_designs),
    },
    Function {
        name: "math.bang.baseq.even_fill_is_balanced",
        signature: "(number: usize, dimension: usize, popcount: u128) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_even_fill_is_balanced),
    },
    Function {
        name: "math.bang.baseq.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_fill_from_corners),
    },
    Function {
        name: "math.bang.baseq.group",
        signature: "(base: usize, dimension: usize) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_group),
    },
    Function {
        name: "math.bang.baseq.group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_group_order),
    },
    Function {
        name: "math.bang.baseq.orbit",
        signature: "(group: [[usize]], code: Code) -> [Code]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_orbit),
    },
    Function {
        name: "math.bang.baseq.predicted_group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_predicted_group_order),
    },
    Function {
        name: "math.bang.baseq.representatives",
        signature: "(base: usize, dimension: usize) -> [(Code, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_representatives),
    },
    Function {
        name: "math.bang.baseq.sequence",
        signature: "(base: usize, max_dimension: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_sequence),
    },
    Function {
        name: "math.bang.baseq.total_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_baseq_total_designs),
    },
    Function {
        name: "math.bang.catalog.Design.all",
        signature: "() -> [math.bang.catalog.Design; 16]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_catalog_design_all),
    },
    Function {
        name: "math.bang.catalog.antis",
        signature: "(dimension: usize) -> [math.bang.catalog.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_catalog_antis),
    },
    Function {
        name: "math.bang.catalog.classics",
        signature: "(dimension: usize) -> [math.bang.catalog.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_catalog_classics),
    },
    Function {
        name: "math.bang.code.get",
        signature: "(self: Code) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_code_get),
    },
    Function {
        name: "math.bang.code_to_corners",
        signature: "(code: Code, dimension: usize, base: usize) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_code_to_corners),
    },
    Function {
        name: "math.bang.corners",
        signature: "(dimension: usize) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_corners),
    },
    Function {
        name: "math.bang.corners_to_code",
        signature: "(filled: [[u8]], dimension: usize, base: usize) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_corners_to_code),
    },
    Function {
        name: "math.bang.factory.create",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_factory_create),
    },
    Function {
        name: "math.bang.factory.create_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_factory_create_from_corners),
    },
    Function {
        name: "math.bang.factory.create_named",
        signature: "(spec: str, number: usize, level: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_factory_create_named),
    },
    Function {
        name: "math.bang.factory.residue_corners",
        signature: "(dimension: usize, base: usize) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_factory_residue_corners),
    },
    Function {
        name: "math.bang.factory.total_codes",
        signature: "(dimension: usize, base: usize) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_factory_total_codes),
    },
    Function {
        name: "math.bang.level_cap",
        signature: "(number: usize, dimension: usize, budget: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_level_cap),
    },
    Function {
        name: "math.bang.levels_code",
        signature: "(dimension: usize, base: usize, levels: [usize]) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_levels_code),
    },
    Function {
        name: "math.bang.magic",
        signature: "(layers: [math.bang.MagicLayer]) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_magic),
    },
    Function {
        name: "math.bang.magic_named",
        signature: "(layers: [(str, usize)]) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_magic_named),
    },
    Function {
        name: "math.bang.random_code",
        signature: "(dimension: usize, base: usize, rng: Rng(seed), min_fills: usize) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_random_code),
    },
    Function {
        name: "math.bang.sources",
        signature: "(catalog: math.bang.catalog.Catalog, dimension: usize) -> [math.bang.catalog.Source]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_sources),
    },
    Function {
        name: "math.bang.symmetries",
        signature: "(dimension: usize) -> [([usize], [u8])]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_symmetries),
    },
    Function {
        name: "math.bang.total_exposure",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_total_exposure),
    },
    Function {
        name: "math.bang.touches_every_corner",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_touches_every_corner),
    },
    Function {
        name: "math.bang.universe.anf",
        signature: "(code: Code, dimension: usize) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_anf),
    },
    Function {
        name: "math.bang.universe.anf_string",
        signature: "(code: Code, dimension: usize) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_anf_string),
    },
    Function {
        name: "math.bang.universe.apply",
        signature: "(element: ([usize], [u8]), corner: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_apply),
    },
    Function {
        name: "math.bang.universe.corner_index",
        signature: "(corner: [u8]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_corner_index),
    },
    Function {
        name: "math.bang.universe.degree",
        signature: "(code: Code, dimension: usize) -> i32",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_degree),
    },
    Function {
        name: "math.bang.universe.orbit",
        signature: "(code: Code, dimension: usize) -> [Code]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_orbit),
    },
    Function {
        name: "math.bang.universe.permutations",
        signature: "(n: usize) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_permutations),
    },
    Function {
        name: "math.bang.universe_codes",
        signature: "(dimension: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_universe_codes),
    },
    Function {
        name: "math.bang.word.Schedule.all",
        signature: "() -> [math.bang.word.Schedule; 3]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_all),
    },
    Function {
        name: "math.bang.word.Schedule.frequencies",
        signature: "(self: math.bang.word.Schedule) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_frequencies),
    },
    Function {
        name: "math.bang.word.Schedule.place",
        signature: "(self: math.bang.word.Schedule, index: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_place),
    },
    Function {
        name: "math.bang.word.components",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_components),
    },
    Function {
        name: "math.bang.word.constant_functional",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_constant_functional),
    },
    Function {
        name: "math.bang.word.dimension",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_dimension),
    },
    Function {
        name: "math.bang.word.fill",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_fill),
    },
    Function {
        name: "math.bang.word.fills",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_fills),
    },
    Function {
        name: "math.bang.word.letter",
        signature: "(layer: math.bang.MagicLayer) -> math.bang.word.Letter",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_letter),
    },
    Function {
        name: "math.bang.word.native",
        signature: "(layers: [math.bang.MagicLayer]) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_native),
    },
    Function {
        name: "math.bang.word.period",
        signature: "(layers: [math.bang.MagicLayer]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_period),
    },
    Function {
        name: "math.bang.word.prefixes",
        signature: "(layers: [math.bang.MagicLayer]) -> [math.bang.word.Counts]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_prefixes),
    },
    Function {
        name: "math.bang.word.rates",
        signature: "(layers: [math.bang.MagicLayer]) -> [(f64, f64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_rates),
    },
    Function {
        name: "math.bang.word.side",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_side),
    },
    Function {
        name: "math.bang.word.spell",
        signature: "(schedule: math.bang.word.Schedule, pair: (math.bang.MagicLayer, math.bang.MagicLayer), length: usize) -> [math.bang.MagicLayer]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_spell),
    },
    Function {
        name: "math.bang.word.staircase",
        signature: "(depth: usize) -> [math.bang.MagicLayer]",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_staircase),
    },
    Function {
        name: "math.bang.word.thue_morse",
        signature: "(index: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_bang_word_thue_morse),
    },
    Function {
        name: "math.cell.census.edges",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_census_edges),
    },
    Function {
        name: "math.cell.census.exposure",
        signature: "(cell: Cell2d|Cell3d) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_census_exposure),
    },
    Function {
        name: "math.cell.census.fills",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_census_fills),
    },
    Function {
        name: "math.cell.census.vertices",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_census_vertices),
    },
    Function {
        name: "math.cell.census.voids",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_census_voids),
    },
    Function {
        name: "math.cell.geometry.merge_reps",
        signature: "(cells: [Cell2d|Cell3d], reps: [usize]) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_geometry_merge_reps),
    },
    Function {
        name: "math.cell.geometry.perforate",
        signature: "(mask: Tensor, cell: Cell2d|Cell3d, value: u8) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_geometry_perforate),
    },
    Function {
        name: "math.cell.grow",
        signature: "(pattern: Tensor, level: usize) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_grow),
    },
    Function {
        name: "math.cell.models.anti",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_anti),
    },
    Function {
        name: "math.cell.models.binarize",
        signature: "(self: Cell2d|Cell3d, threshold: u8) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_binarize),
    },
    Function {
        name: "math.cell.models.binarize_otsu",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_binarize_otsu),
    },
    Function {
        name: "math.cell.models.blur",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, wrap: bool) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_blur),
    },
    Function {
        name: "math.cell.models.combine",
        signature: "(self: Cell2d|Cell3d, other: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_combine),
    },
    Function {
        name: "math.cell.models.counting_dtype",
        signature: "(mask: Tensor) -> core.Dtype",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_counting_dtype),
    },
    Function {
        name: "math.cell.models.depth",
        signature: "(self: Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_depth),
    },
    Function {
        name: "math.cell.models.dtype_for",
        signature: "(peak: i64) -> core.Dtype",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_dtype_for),
    },
    Function {
        name: "math.cell.models.fractal",
        signature: "(self: Cell2d|Cell3d, level: usize) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_fractal),
    },
    Function {
        name: "math.cell.models.height",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_height),
    },
    Function {
        name: "math.cell.models.invert",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_invert),
    },
    Function {
        name: "math.cell.models.layers",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_layers),
    },
    Function {
        name: "math.cell.models.neighbors",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, target: u8, wrap: bool) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_neighbors),
    },
    Function {
        name: "math.cell.models.new",
        signature: "(types: Tensor) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_new),
    },
    Function {
        name: "math.cell.models.orient",
        signature: "(self: Cell3d, index: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_orient),
    },
    Function {
        name: "math.cell.models.pad",
        signature: "(self: Cell2d|Cell3d, count: usize, value: u8) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_pad),
    },
    Function {
        name: "math.cell.models.paint",
        signature: "(self: Cell2d|Cell3d, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_paint),
    },
    Function {
        name: "math.cell.models.perforate",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, value: u8) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_perforate),
    },
    Function {
        name: "math.cell.models.rotate",
        signature: "(self: Cell2d, k: usize) -> Cell2d | (self: Cell3d, k: usize, axes: (usize, usize)) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_rotate),
    },
    Function {
        name: "math.cell.models.tile",
        signature: "(self: Cell2d, width: usize, height: usize) -> Cell2d | (self: Cell3d, width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_tile),
    },
    Function {
        name: "math.cell.models.types",
        signature: "(self: Cell2d|Cell3d) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_types),
    },
    Function {
        name: "math.cell.models.width",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_models_width),
    },
    Function {
        name: "math.cell.paint",
        signature: "(cell: Cell2d|Cell3d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_paint),
    },
    Function {
        name: "math.cell.serializer.byte_cube",
        signature: "(value: Json) -> [[[u8]]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_cube),
    },
    Function {
        name: "math.cell.serializer.byte_grid",
        signature: "(value: Json) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_grid),
    },
    Function {
        name: "math.cell.serializer.color_grid",
        signature: "(value: Json) -> [[[u8; 4]]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_color_grid),
    },
    Function {
        name: "math.cell.serializer.count_cube",
        signature: "(value: Json) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_cube),
    },
    Function {
        name: "math.cell.serializer.count_grid",
        signature: "(value: Json) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_grid),
    },
    Function {
        name: "math.cell.serializer.parse",
        signature: "(text: str) -> Json",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_parse),
    },
    Function {
        name: "math.cell.serializer.tag_layer",
        signature: "(counts: [i64], shape: [usize]) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_tag_layer),
    },
    Function {
        name: "math.cell.serializer.types_field",
        signature: "(data: Json) -> Json",
        docs: "",
        uncallable: None,
        door: Some(door_math_cell_serializer_types_field),
    },
    Function {
        name: "math.coprime.gasket",
        signature: "(level: u32) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_coprime_gasket),
    },
    Function {
        name: "math.counts.Closed.text",
        signature: "(self: math.counts.Closed) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_closed_text),
    },
    Function {
        name: "math.counts.Exposure.at",
        signature: "(self: math.counts.Exposure, level: u32) -> u128?",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_at),
    },
    Function {
        name: "math.counts.Exposure.from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> math.counts.Exposure",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_from_corners),
    },
    Function {
        name: "math.counts.Exposure.of_tile",
        signature: "(tile: Tensor) -> math.counts.Exposure",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile),
    },
    Function {
        name: "math.counts.Exposure.recurrence",
        signature: "(self: math.counts.Exposure) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence),
    },
    Function {
        name: "math.counts.Placements.at",
        signature: "(self: math.counts.Placements, level: u32) -> math.counts.Skeleton?",
        docs: "",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.of_tile",
        signature: "(tile: Tensor) -> math.counts.Placements",
        docs: "",
        uncallable: Some("math::counts::Placements has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.recurrence",
        signature: "(self: math.counts.Placements, k: usize?) -> [i128]?",
        docs: "",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.roots",
        signature: "(self: math.counts.Placements, k: usize?) -> [(u128, u32)]",
        docs: "",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.cells",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.edges",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.faces",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.vertices",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.centered_hexagonal",
        signature: "(m: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_centered_hexagonal),
    },
    Function {
        name: "math.counts.cut_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_cut_fills),
    },
    Function {
        name: "math.counts.cut_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_cut_voids),
    },
    Function {
        name: "math.counts.dimension",
        signature: "(code: Code, number: usize, base_dimension: usize, base: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_dimension),
    },
    Function {
        name: "math.counts.edges_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_edges_of_tile),
    },
    Function {
        name: "math.counts.exposure",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure),
    },
    Function {
        name: "math.counts.exposure_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile_2),
    },
    Function {
        name: "math.counts.exposure_recurrence",
        signature: "(tile: Tensor) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence_2),
    },
    Function {
        name: "math.counts.fill",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_fill),
    },
    Function {
        name: "math.counts.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, _dimension: usize, level: u32, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_fill_from_corners),
    },
    Function {
        name: "math.counts.fill_polynomial",
        signature: "(corners: [[u8]], dimension: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_fill_polynomial),
    },
    Function {
        name: "math.counts.grid",
        signature: "(number: usize, dimension: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_grid),
    },
    Function {
        name: "math.counts.ladder.cap",
        signature: "(base: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_cap),
    },
    Function {
        name: "math.counts.ladder.carry_matrix",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_carry_matrix),
    },
    Function {
        name: "math.counts.ladder.characteristic",
        signature: "(rows: [[i128]]) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_characteristic),
    },
    Function {
        name: "math.counts.ladder.determinant",
        signature: "(rows: [[i128]]) -> i128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_determinant),
    },
    Function {
        name: "math.counts.ladder.digit_polynomial",
        signature: "(base: usize, dimension: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_digit_polynomial),
    },
    Function {
        name: "math.counts.ladder.even_block",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_even_block),
    },
    Function {
        name: "math.counts.ladder.fill",
        signature: "(base: usize, dimension: usize) -> i128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_fill),
    },
    Function {
        name: "math.counts.ladder.ladder",
        signature: "(base: usize, dimension: usize, levels: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_ladder),
    },
    Function {
        name: "math.counts.ladder.perron",
        signature: "(rows: [[i128]]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_perron),
    },
    Function {
        name: "math.counts.ladder.sign",
        signature: "(base: usize, dimension: usize) -> i32",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_sign),
    },
    Function {
        name: "math.counts.ladder.spectral_ratio",
        signature: "(base: usize, dimension: usize) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_spectral_ratio),
    },
    Function {
        name: "math.counts.ladder.trace",
        signature: "(rows: [[i128]]) -> i128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ladder_trace),
    },
    Function {
        name: "math.counts.limit",
        signature: "(code: Code, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_limit),
    },
    Function {
        name: "math.counts.odd_power",
        signature: "(dimension: usize) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_odd_power),
    },
    Function {
        name: "math.counts.pairs",
        signature: "(tile: Tensor) -> [(u128, u128)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_pairs),
    },
    Function {
        name: "math.counts.positions",
        signature: "(residue: usize, number: usize, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_positions),
    },
    Function {
        name: "math.counts.pro_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_pro_fills),
    },
    Function {
        name: "math.counts.pro_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_pro_voids),
    },
    Function {
        name: "math.counts.profile_of_tile",
        signature: "(tile: Tensor, level: u32) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_profile_of_tile),
    },
    Function {
        name: "math.counts.ratio",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_ratio),
    },
    Function {
        name: "math.counts.rational",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_rational),
    },
    Function {
        name: "math.counts.six.grid_triangles",
        signature: "(number: usize, level: u32) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_grid_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_boundary",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_boundary),
    },
    Function {
        name: "math.counts.six.solid_slice_core_edges",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_core_nodes",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_nodes),
    },
    Function {
        name: "math.counts.six.solid_slice_edges",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_interior",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_interior),
    },
    Function {
        name: "math.counts.six.solid_slice_triangles",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_vertices",
        signature: "(number: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_vertices),
    },
    Function {
        name: "math.counts.skeleton",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, level: u32) -> math.counts.Skeleton",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.skeleton_of_tile",
        signature: "(tile: Tensor, level: u32) -> math.counts.Skeleton?",
        docs: "",
        uncallable: Some("math::counts::Skeleton has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.skeleton_recurrence",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, k: usize?) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_skeleton_recurrence),
    },
    Function {
        name: "math.counts.surface",
        signature: "(code: Code, number: usize, level: u32, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_surface),
    },
    Function {
        name: "math.counts.void",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_counts_void),
    },
    Function {
        name: "math.design.digits_of",
        signature: "(mask: u32, base: u64) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_digits_of),
    },
    Function {
        name: "math.design.echo_series",
        signature: "(values: [u64], log_x: [f64], exponent: f64) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_echo_series),
    },
    Function {
        name: "math.design.elements",
        signature: "(base: u64, digits: [u64], depth: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_elements),
    },
    Function {
        name: "math.design.log_grid",
        signature: "(values: [u64], samples: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_log_grid),
    },
    Function {
        name: "math.design.median_floor",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_median_floor),
    },
    Function {
        name: "math.design.meter",
        signature: "(mu: [i8]) -> [i64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_meter),
    },
    Function {
        name: "math.design.nearest",
        signature: "(value: f64, list: [f64]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_nearest),
    },
    Function {
        name: "math.design.peaks",
        signature: "(gamma: [f64], score: [f64], band: (f64, f64), threshold: f64) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_peaks),
    },
    Function {
        name: "math.design.pole_lattice",
        signature: "(base: u64, top: f64) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_pole_lattice),
    },
    Function {
        name: "math.design.resample",
        signature: "(values: [u64], running: [i64], exponent: f64, log_x: [f64]) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_resample),
    },
    Function {
        name: "math.design.score",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_score),
    },
    Function {
        name: "math.design.size",
        signature: "(digits: [u64], depth: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_size),
    },
    Function {
        name: "math.design.spectrum",
        signature: "(log_x: [f64], series: [f64]) -> ([f64], [f64])",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_spectrum),
    },
    Function {
        name: "math.design.upper_rms",
        signature: "(series: [f64]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_design_upper_rms),
    },
    Function {
        name: "math.dissection.cap",
        signature: "(y: u64) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_cap),
    },
    Function {
        name: "math.dissection.chain_exponent",
        signature: "(base: u64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_chain_exponent),
    },
    Function {
        name: "math.dissection.chain_margin",
        signature: "(base: u64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_chain_margin),
    },
    Function {
        name: "math.dissection.chain_root",
        signature: "(base: u64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_chain_root),
    },
    Function {
        name: "math.dissection.chain_wall",
        signature: "() -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_chain_wall),
    },
    Function {
        name: "math.dissection.consecutive",
        signature: "(digits: [u64]) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_consecutive),
    },
    Function {
        name: "math.dissection.fraction",
        signature: "(a: u64, y: u64, cap: u64) -> (u64, u64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_fraction),
    },
    Function {
        name: "math.dissection.kappa",
        signature: "(base: u64, digits: [u64]) -> (u64, u64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_kappa),
    },
    Function {
        name: "math.dissection.masses",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_masses),
    },
    Function {
        name: "math.dissection.reach",
        signature: "(base: u64, missing: u64) -> str",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_reach),
    },
    Function {
        name: "math.dissection.reading",
        signature: "(base: u64, fill: usize, masses: [f64]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_reading),
    },
    Function {
        name: "math.dissection.regions",
        signature: "(base: u64, level: u32, z: u64) -> [math.dissection.Region]",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_regions),
    },
    Function {
        name: "math.dissection.tally",
        signature: "(base: u64, digits: [u64], level: usize, samples: usize) -> math.dissection.Tally",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_tally),
    },
    Function {
        name: "math.dissection.weights",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_dissection_weights),
    },
    Function {
        name: "math.game.cell",
        signature: "(point: [f64], base: u8, level: u32) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_cell),
    },
    Function {
        name: "math.game.cells",
        signature: "(base: u8, dimension: u8, level: u32) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_cells),
    },
    Function {
        name: "math.game.counts",
        signature: "(matrix: [[u64]], start: usize, levels: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_counts),
    },
    Function {
        name: "math.game.de_bruijn",
        signature: "(m: u32, k: u32) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_de_bruijn),
    },
    Function {
        name: "math.game.mosaic_game",
        signature: "(mosaic: math.game.Mosaic) -> math.game.MosaicGame",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_mosaic_game),
    },
    Function {
        name: "math.game.named_games",
        signature: "() -> [math.game.NamedGame]",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_named_games),
    },
    Function {
        name: "math.game.perron",
        signature: "(matrix: [[u64]]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_perron),
    },
    Function {
        name: "math.game.reverse",
        signature: "(rule: math.memory.Rule) -> math.memory.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_reverse),
    },
    Function {
        name: "math.game.slice_game",
        signature: "(base: u8, digits: [u8], p: i64, q: i64) -> math.game.SliceGame",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_slice_game),
    },
    Function {
        name: "math.game.successors",
        signature: "(rule: math.memory.Rule, history: [u8]) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_successors),
    },
    Function {
        name: "math.game.word_cell",
        signature: "(word: [u8], base: u8, dimension: u8) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_game_word_cell),
    },
    Function {
        name: "math.graph.Layout.energy",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.from_network",
        signature: "(network: math.graph.Network, seed: u64) -> math.graph.Layout",
        docs: "",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ideal",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.moved",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.new",
        signature: "(positions: [f64], branches: [(usize, usize)], dim: usize, seed: u64) -> math.graph.Layout",
        docs: "",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.nodes",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.positions",
        signature: "(self: math.graph.Layout) -> [f64]",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.step",
        signature: "(self: math.graph.Layout, ticks: usize) -> f64",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.temperature",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ticks",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_branch",
        signature: "(self: math.graph.Network, parent: usize, child: usize, radius: f64) -> null",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_node",
        signature: "(self: math.graph.Network, position: [f64]) -> usize",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.adjacency",
        signature: "(self: math.graph.Network) -> {usize: [usize]}",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_network_adjacency),
    },
    Function {
        name: "math.graph.Network.degree",
        signature: "(self: math.graph.Network) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_network_degree),
    },
    Function {
        name: "math.graph.Network.new",
        signature: "(dim: usize) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_network_new),
    },
    Function {
        name: "math.graph.census",
        signature: "(network: math.graph.Network) -> math.graph.Census",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_census),
    },
    Function {
        name: "math.graph.components",
        signature: "(network: math.graph.Network) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_components),
    },
    Function {
        name: "math.graph.core_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_core_graph),
    },
    Function {
        name: "math.graph.edge_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_edge_graph),
    },
    Function {
        name: "math.graph.fractal_dimension",
        signature: "(network: math.graph.Network, samples: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_fractal_dimension),
    },
    Function {
        name: "math.graph.junctions",
        signature: "(network: math.graph.Network) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_junctions),
    },
    Function {
        name: "math.graph.largest_component",
        signature: "(network: math.graph.Network) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_largest_component),
    },
    Function {
        name: "math.graph.roles",
        signature: "(network: math.graph.Network) -> [math.graph.Role]",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_roles),
    },
    Function {
        name: "math.graph.tips",
        signature: "(network: math.graph.Network) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_tips),
    },
    Function {
        name: "math.graph.total_length",
        signature: "(network: math.graph.Network) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_total_length),
    },
    Function {
        name: "math.graph.tunnel_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_graph_tunnel_graph),
    },
    Function {
        name: "math.ladder.Design.abscissa",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_abscissa),
    },
    Function {
        name: "math.ladder.Design.base",
        signature: "(self: math.ladder.Design) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_base),
    },
    Function {
        name: "math.ladder.Design.digits",
        signature: "(self: math.ladder.Design) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_digits),
    },
    Function {
        name: "math.ladder.Design.new",
        signature: "(base: u64, digits: [u64]) -> math.ladder.Design",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_new),
    },
    Function {
        name: "math.ladder.Design.peel",
        signature: "(self: math.ladder.Design) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_peel),
    },
    Function {
        name: "math.ladder.Design.period",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_period),
    },
    Function {
        name: "math.ladder.Design.pole",
        signature: "(self: math.ladder.Design, m: usize, j: i64) -> num.zeta.Complex",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_pole),
    },
    Function {
        name: "math.ladder.Design.with_peel",
        signature: "(base: u64, digits: [u64], peel: usize) -> math.ladder.Design",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_design_with_peel),
    },
    Function {
        name: "math.ladder.cofactor",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_cofactor),
    },
    Function {
        name: "math.ladder.residue",
        signature: "(design: math.ladder.Design, m: usize, j: i64, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_residue),
    },
    Function {
        name: "math.ladder.zeta",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_ladder_zeta),
    },
    Function {
        name: "math.memory.Rule.accepts",
        signature: "(self: math.memory.Rule, word: [usize]) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_accepts),
    },
    Function {
        name: "math.memory.Rule.allowed",
        signature: "(self: math.memory.Rule, window: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_allowed),
    },
    Function {
        name: "math.memory.Rule.alphabet",
        signature: "(self: math.memory.Rule) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_alphabet),
    },
    Function {
        name: "math.memory.Rule.codes",
        signature: "(self: math.memory.Rule) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_codes),
    },
    Function {
        name: "math.memory.Rule.full",
        signature: "(dimension: usize, width: usize) -> math.memory.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_full),
    },
    Function {
        name: "math.memory.Rule.letters",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_letters),
    },
    Function {
        name: "math.memory.Rule.new",
        signature: "(dimension: usize, width: usize, code: u64) -> math.memory.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_new),
    },
    Function {
        name: "math.memory.Rule.states",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_states),
    },
    Function {
        name: "math.memory.Rule.windows",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_rule_windows),
    },
    Function {
        name: "math.memory.allowed_windows",
        signature: "(rule: math.memory.Rule) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_allowed_windows),
    },
    Function {
        name: "math.memory.cells",
        signature: "(rule: math.memory.Rule, level: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_cells),
    },
    Function {
        name: "math.memory.counts",
        signature: "(rule: math.memory.Rule, levels: usize) -> [u64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_counts),
    },
    Function {
        name: "math.memory.exponent",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_exponent),
    },
    Function {
        name: "math.memory.kappa",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_kappa),
    },
    Function {
        name: "math.memory.perron",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_perron),
    },
    Function {
        name: "math.memory.transfer",
        signature: "(rule: math.memory.Rule) -> [[u64]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_memory_transfer),
    },
    Function {
        name: "math.moire.Field.as_f64",
        signature: "(self: math.moire.Field) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_as_f64),
    },
    Function {
        name: "math.moire.Field.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Field",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_from_data),
    },
    Function {
        name: "math.moire.Field.max",
        signature: "(self: math.moire.Field) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_max),
    },
    Function {
        name: "math.moire.Field.mean",
        signature: "(self: math.moire.Field) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_mean),
    },
    Function {
        name: "math.moire.Field.min",
        signature: "(self: math.moire.Field) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_min),
    },
    Function {
        name: "math.moire.Field.new",
        signature: "(size: usize) -> math.moire.Field",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_new),
    },
    Function {
        name: "math.moire.Field.normalized",
        signature: "(self: math.moire.Field, symmetric: bool) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_field_normalized),
    },
    Function {
        name: "math.moire.Layer.new",
        signature: "(spec: math.moire.Spec, number: usize) -> math.moire.Layer",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_layer_new),
    },
    Function {
        name: "math.moire.Preset.carpet",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_preset_carpet),
    },
    Function {
        name: "math.moire.Preset.field",
        signature: "(self: math.moire.Preset, size: usize) -> math.moire.Field",
        docs: "",
        uncallable: Some("math::moire::Preset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.moire.Preset.heatmap",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_preset_heatmap),
    },
    Function {
        name: "math.moire.Preset.hive",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_preset_hive),
    },
    Function {
        name: "math.moire.Preset.weave",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_preset_weave),
    },
    Function {
        name: "math.moire.Spec.new",
        signature: "(code: u128, base: usize, dimension: usize) -> math.moire.Spec",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_spec_new),
    },
    Function {
        name: "math.moire.Volume.at",
        signature: "(self: math.moire.Volume, x: usize, y: usize, z: usize) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_at),
    },
    Function {
        name: "math.moire.Volume.count",
        signature: "(self: math.moire.Volume, level: f32) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_count),
    },
    Function {
        name: "math.moire.Volume.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Volume",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_from_data),
    },
    Function {
        name: "math.moire.Volume.max",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_max),
    },
    Function {
        name: "math.moire.Volume.min",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_min),
    },
    Function {
        name: "math.moire.Volume.new",
        signature: "(size: usize) -> math.moire.Volume",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_new),
    },
    Function {
        name: "math.moire.Volume.plane",
        signature: "(self: math.moire.Volume, frame: math.moire.Frame, out: usize) -> ([f32], [u8])",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_plane),
    },
    Function {
        name: "math.moire.Volume.sample",
        signature: "(self: math.moire.Volume, p: [f64; 3]) -> f32?",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_sample),
    },
    Function {
        name: "math.moire.Volume.solid",
        signature: "(self: math.moire.Volume, level: f32) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume_solid),
    },
    Function {
        name: "math.moire.all",
        signature: "(limit: usize) -> [math.moire.Preset]",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_all),
    },
    Function {
        name: "math.moire.frame",
        signature: "(normal: [f64; 3], offset: f64) -> math.moire.Frame",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_frame),
    },
    Function {
        name: "math.moire.layer",
        signature: "(params: math.moire.Layer) -> [bool]",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_layer),
    },
    Function {
        name: "math.moire.named",
        signature: "(name: str, limit: usize) -> math.moire.Preset",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_named),
    },
    Function {
        name: "math.moire.pairs.correlation",
        signature: "(m: usize, n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_pairs_correlation),
    },
    Function {
        name: "math.moire.pairs.sampled",
        signature: "(m: usize, n: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_pairs_sampled),
    },
    Function {
        name: "math.moire.pairs.witness",
        signature: "(scale: usize) -> math.moire.pairs.Witness",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_pairs_witness),
    },
    Function {
        name: "math.moire.render",
        signature: "(field: math.moire.Field, colorizer: core.Colorizer, levels: usize, symmetric: bool, invert: bool, scale: usize) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_render),
    },
    Function {
        name: "math.moire.sample.axes",
        signature: "(size: usize, lattice: math.moire.Lattice, row: usize) -> ([f64], [f64])",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_sample_axes),
    },
    Function {
        name: "math.moire.sample.membership",
        signature: "(code: u128, base: usize, dimension: usize) -> [bool]",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_sample_membership),
    },
    Function {
        name: "math.moire.sample.pack",
        signature: "(residues: [usize], base: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_sample_pack),
    },
    Function {
        name: "math.moire.stack",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_stack),
    },
    Function {
        name: "math.moire.stack_codes",
        signature: "(specs: [math.moire.Spec], number: usize, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_stack_codes),
    },
    Function {
        name: "math.moire.volume",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, size: usize) -> math.moire.Volume",
        docs: "",
        uncallable: None,
        door: Some(door_math_moire_volume),
    },
    Function {
        name: "math.name.Bang.cells",
        signature: "(self: math.name.Bang) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_cells),
    },
    Function {
        name: "math.name.Bang.checked",
        signature: "(self: math.name.Bang) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_checked),
    },
    Function {
        name: "math.name.Bang.from_file",
        signature: "(text: str) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_from_file),
    },
    Function {
        name: "math.name.Bang.from_json",
        signature: "(text: str) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_from_json),
    },
    Function {
        name: "math.name.Bang.from_url",
        signature: "(text: str) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_from_url),
    },
    Function {
        name: "math.name.Bang.new",
        signature: "(code: u128, dim: usize, base: usize) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_new),
    },
    Function {
        name: "math.name.Bang.to_file",
        signature: "(self: math.name.Bang) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_to_file),
    },
    Function {
        name: "math.name.Bang.to_id",
        signature: "(self: math.name.Bang) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_to_id),
    },
    Function {
        name: "math.name.Bang.to_json",
        signature: "(self: math.name.Bang) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_to_json),
    },
    Function {
        name: "math.name.Bang.to_prose",
        signature: "(self: math.name.Bang) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_to_prose),
    },
    Function {
        name: "math.name.Bang.to_url",
        signature: "(self: math.name.Bang) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_bang_to_url),
    },
    Function {
        name: "math.name.Lattice.default",
        signature: "() -> math.name.Lattice",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_lattice_default),
    },
    Function {
        name: "math.name.Lattice.is_square",
        signature: "(self: math.name.Lattice) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_lattice_is_square),
    },
    Function {
        name: "math.name.Lattice.units",
        signature: "(self: math.name.Lattice) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_lattice_units),
    },
    Function {
        name: "math.name.Sequence.checked",
        signature: "(self: math.name.Sequence) -> math.name.Sequence",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_checked),
    },
    Function {
        name: "math.name.Sequence.design",
        signature: "(self: math.name.Sequence) -> math.name.Bang",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_design),
    },
    Function {
        name: "math.name.Sequence.from_file",
        signature: "(text: str) -> math.name.Sequence",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_from_file),
    },
    Function {
        name: "math.name.Sequence.from_json",
        signature: "(text: str) -> math.name.Sequence",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_from_json),
    },
    Function {
        name: "math.name.Sequence.from_url",
        signature: "(text: str) -> math.name.Sequence",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_from_url),
    },
    Function {
        name: "math.name.Sequence.new",
        signature: "(code: u128, dim: usize, base: usize, measure: str, axis: str) -> math.name.Sequence",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_new),
    },
    Function {
        name: "math.name.Sequence.to_file",
        signature: "(self: math.name.Sequence) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_to_file),
    },
    Function {
        name: "math.name.Sequence.to_id",
        signature: "(self: math.name.Sequence) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_to_id),
    },
    Function {
        name: "math.name.Sequence.to_json",
        signature: "(self: math.name.Sequence) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_to_json),
    },
    Function {
        name: "math.name.Sequence.to_prose",
        signature: "(self: math.name.Sequence) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_to_prose),
    },
    Function {
        name: "math.name.Sequence.to_url",
        signature: "(self: math.name.Sequence) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_sequence_to_url),
    },
    Function {
        name: "math.name.Word.bases",
        signature: "(self: math.name.Word) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_bases),
    },
    Function {
        name: "math.name.Word.checked",
        signature: "(self: math.name.Word) -> math.name.Word",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_checked),
    },
    Function {
        name: "math.name.Word.from_file",
        signature: "(text: str) -> math.name.Word",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_from_file),
    },
    Function {
        name: "math.name.Word.from_json",
        signature: "(text: str) -> math.name.Word",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_from_json),
    },
    Function {
        name: "math.name.Word.from_url",
        signature: "(text: str) -> math.name.Word",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_from_url),
    },
    Function {
        name: "math.name.Word.letters",
        signature: "(self: math.name.Word) -> [math.name.Bang]",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_letters),
    },
    Function {
        name: "math.name.Word.new",
        signature: "(dim: usize, letters: [(u128, usize)]) -> math.name.Word",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_new),
    },
    Function {
        name: "math.name.Word.to_file",
        signature: "(self: math.name.Word) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_to_file),
    },
    Function {
        name: "math.name.Word.to_id",
        signature: "(self: math.name.Word) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_to_id),
    },
    Function {
        name: "math.name.Word.to_json",
        signature: "(self: math.name.Word) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_to_json),
    },
    Function {
        name: "math.name.Word.to_prose",
        signature: "(self: math.name.Word) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_to_prose),
    },
    Function {
        name: "math.name.Word.to_url",
        signature: "(self: math.name.Word) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_word_to_url),
    },
    Function {
        name: "math.name.of",
        signature: "(code: Code, dim: usize, base: usize) -> String?",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_of),
    },
    Function {
        name: "math.name.words",
        signature: "(dim: usize, base: usize) -> [(String, String)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_name_words),
    },
    Function {
        name: "math.nodal.figures",
        signature: "(code: u64, number: usize, level: usize, count: usize) -> math.nodal.Figures",
        docs: "",
        uncallable: None,
        door: Some(door_math_nodal_figures),
    },
    Function {
        name: "math.nodal.holds",
        signature: "(code: u64, number: usize, level: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_nodal_holds),
    },
    Function {
        name: "math.press.Press.add",
        signature: "(self: math.press.Press, number: u128, weight: i128) -> null",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.press.Press.new",
        signature: "(dimension: usize, base: usize) -> math.press.Press",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_press_new),
    },
    Function {
        name: "math.press.Press.total",
        signature: "(self: math.press.Press, code: Code) -> i128",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_press_total),
    },
    Function {
        name: "math.press.Press.totals",
        signature: "(self: math.press.Press) -> [i128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_press_totals),
    },
    Function {
        name: "math.press.containing",
        signature: "(number: u128, dimension: usize, base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_containing),
    },
    Function {
        name: "math.press.coordinates",
        signature: "(number: u128, dimension: usize, base: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_coordinates),
    },
    Function {
        name: "math.press.count_below",
        signature: "(code: Code, dimension: usize, base: usize, limit: u128) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_count_below),
    },
    Function {
        name: "math.press.distinct",
        signature: "(number: u128, dimension: usize, base: usize) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_distinct),
    },
    Function {
        name: "math.press.interleave",
        signature: "(coords: [u128], base: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_interleave),
    },
    Function {
        name: "math.press.layer_table",
        signature: "(layer: math.bang.MagicLayer) -> [bool]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_layer_table),
    },
    Function {
        name: "math.press.member",
        signature: "(code: Code, number: u128, dimension: usize, base: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_member),
    },
    Function {
        name: "math.press.members",
        signature: "(code: Code, dimension: usize, base: usize, count: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_members),
    },
    Function {
        name: "math.press.profile",
        signature: "(code: Code, dimension: usize, base: usize, level: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_profile),
    },
    Function {
        name: "math.press.usage",
        signature: "(number: u128, dimension: usize, base: usize) -> Code",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_usage),
    },
    Function {
        name: "math.press.word_count",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_word_count),
    },
    Function {
        name: "math.press.word_member",
        signature: "(layers: [math.bang.MagicLayer], number: u128) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_word_member),
    },
    Function {
        name: "math.press.word_members",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_word_members),
    },
    Function {
        name: "math.press.word_profile",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_press_word_profile),
    },
    Function {
        name: "math.radix.Base.class",
        signature: "(self: math.radix.Base, z: (i64, i64)) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_class),
    },
    Function {
        name: "math.radix.Base.congruent",
        signature: "(self: math.radix.Base, z: (i64, i64), w: (i64, i64)) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_congruent),
    },
    Function {
        name: "math.radix.Base.group",
        signature: "(self: math.radix.Base) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_group),
    },
    Function {
        name: "math.radix.Base.mirrored",
        signature: "(self: math.radix.Base) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_mirrored),
    },
    Function {
        name: "math.radix.Base.new",
        signature: "(ring: num.gauss.Ring, value: (i64, i64)) -> math.radix.Base",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_new),
    },
    Function {
        name: "math.radix.Base.norm",
        signature: "(self: math.radix.Base) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_norm),
    },
    Function {
        name: "math.radix.Base.power",
        signature: "(self: math.radix.Base, level: usize) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_power),
    },
    Function {
        name: "math.radix.Base.residues",
        signature: "(self: math.radix.Base) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_residues),
    },
    Function {
        name: "math.radix.Base.ring",
        signature: "(self: math.radix.Base) -> num.gauss.Ring",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_ring),
    },
    Function {
        name: "math.radix.Base.value",
        signature: "(self: math.radix.Base) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_base_value),
    },
    Function {
        name: "math.radix.Radix.base",
        signature: "(self: math.radix.Radix) -> math.radix.Base",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_base),
    },
    Function {
        name: "math.radix.Radix.canonical",
        signature: "(self: math.radix.Radix) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_canonical),
    },
    Function {
        name: "math.radix.Radix.code",
        signature: "(self: math.radix.Radix) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_code),
    },
    Function {
        name: "math.radix.Radix.digits",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_digits),
    },
    Function {
        name: "math.radix.Radix.dimension",
        signature: "(self: math.radix.Radix) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_dimension),
    },
    Function {
        name: "math.radix.Radix.distinct",
        signature: "(self: math.radix.Radix, level: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_distinct),
    },
    Function {
        name: "math.radix.Radix.fill",
        signature: "(self: math.radix.Radix, level: usize) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_fill),
    },
    Function {
        name: "math.radix.Radix.from_code",
        signature: "(base: math.radix.Base, code: u128) -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_from_code),
    },
    Function {
        name: "math.radix.Radix.new",
        signature: "(base: math.radix.Base, digits: [(i64, i64)], twists: [(i64, i64)]) -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_new),
    },
    Function {
        name: "math.radix.Radix.plane",
        signature: "(self: math.radix.Radix, level: usize) -> [(f64, f64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_plane),
    },
    Function {
        name: "math.radix.Radix.ring",
        signature: "(self: math.radix.Radix) -> num.gauss.Ring",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_ring),
    },
    Function {
        name: "math.radix.Radix.size",
        signature: "(self: math.radix.Radix) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_size),
    },
    Function {
        name: "math.radix.Radix.twists",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_twists),
    },
    Function {
        name: "math.radix.Radix.with_twists",
        signature: "(self: math.radix.Radix, units: [usize]) -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_with_twists),
    },
    Function {
        name: "math.radix.Radix.words",
        signature: "(self: math.radix.Radix, level: usize) -> [(i64, i64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_radix_words),
    },
    Function {
        name: "math.radix.flowsnake",
        signature: "() -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_flowsnake),
    },
    Function {
        name: "math.radix.gasket",
        signature: "() -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_gasket),
    },
    Function {
        name: "math.radix.koch",
        signature: "() -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_koch),
    },
    Function {
        name: "math.radix.terdragon",
        signature: "() -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_terdragon),
    },
    Function {
        name: "math.radix.tile",
        signature: "(m: u64, code: u128) -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_tile),
    },
    Function {
        name: "math.radix.twindragon",
        signature: "() -> math.radix.Radix",
        docs: "",
        uncallable: None,
        door: Some(door_math_radix_twindragon),
    },
    Function {
        name: "math.roulette.Nodes.default",
        signature: "() -> math.roulette.Nodes",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes_default),
    },
    Function {
        name: "math.roulette.Nodes.pair",
        signature: "(self: math.roulette.Nodes, i: usize, j: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes_pair),
    },
    Function {
        name: "math.roulette.Nodes.paired",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes_paired),
    },
    Function {
        name: "math.roulette.Nodes.selved",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes_selved),
    },
    Function {
        name: "math.roulette.Nodes.total",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes_total),
    },
    Function {
        name: "math.roulette.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize, tol: f64) -> math.roulette.Nodes",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_nodes),
    },
    Function {
        name: "math.roulette.side",
        signature: "(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> i32",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_side),
    },
    Function {
        name: "math.roulette.spread",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [math.spirograph.Pencil]",
        docs: "",
        uncallable: None,
        door: Some(door_math_roulette_spread),
    },
    Function {
        name: "math.rules.render",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_rules_render),
    },
    Function {
        name: "math.rules.tree_axes",
        signature: "(dimension: usize, free_axis: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_rules_tree_axes),
    },
    Function {
        name: "math.shape.Frac.minus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_frac_minus),
    },
    Function {
        name: "math.shape.Frac.new",
        signature: "(num: i64, den: i64) -> math.shape.Frac",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_frac_new),
    },
    Function {
        name: "math.shape.Frac.plus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_frac_plus),
    },
    Function {
        name: "math.shape.Frac.times",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_frac_times),
    },
    Function {
        name: "math.shape.Frac.whole",
        signature: "(num: i64) -> math.shape.Frac",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_frac_whole),
    },
    Function {
        name: "math.shape.Region.flip",
        signature: "(self: math.shape.Region) -> math.shape.Region",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_region_flip),
    },
    Function {
        name: "math.shape.census",
        signature: "(shape: math.shape.Shape, types: Tensor) -> math.shape.ShapeCensus",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_census),
    },
    Function {
        name: "math.shape.classify",
        signature: "(shape: math.shape.Shape, side: usize, index: [usize]) -> math.shape.Region",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_classify),
    },
    Function {
        name: "math.shape.crop",
        signature: "(types: Tensor, shape: math.shape.Shape, keep_cut: bool) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_crop),
    },
    Function {
        name: "math.shape.crossing_shell",
        signature: "(radius: u64, number: u64, level: u32) -> [(u64, u64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_crossing_shell),
    },
    Function {
        name: "math.shape.crossing_tree",
        signature: "(radius: u64, number: u64, keep: [bool]) -> math.shape.Shell",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_crossing_tree),
    },
    Function {
        name: "math.shape.fit",
        signature: "(shape: math.shape.Shape, extents: [usize]) -> math.shape.Shape",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_fit),
    },
    Function {
        name: "math.shape.named",
        signature: "(name: str, dimension: usize, radius: math.shape.Frac) -> math.shape.Shape",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_named),
    },
    Function {
        name: "math.shape.radial_census",
        signature: "(types: Tensor, centre: [i64], r_max: u64) -> [math.shape.RadialCounts]",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_radial_census),
    },
    Function {
        name: "math.shape.reach",
        signature: "(name: str, dimension: usize, den: u32) -> u32?",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_reach),
    },
    Function {
        name: "math.shape.refine",
        signature: "(types: Tensor, shape: math.shape.Shape, base: usize, extra: usize, keep_cut: bool) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_refine),
    },
    Function {
        name: "math.shape.regions",
        signature: "(shape: math.shape.Shape, dims: [usize]) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_regions),
    },
    Function {
        name: "math.shape.shapes",
        signature: "(dimension: usize) -> [String]",
        docs: "",
        uncallable: None,
        door: Some(door_math_shape_shapes),
    },
    Function {
        name: "math.six.anti",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_anti),
    },
    Function {
        name: "math.six.binarize",
        signature: "(self: Cell6d, threshold: u8) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_binarize),
    },
    Function {
        name: "math.six.binarize_otsu",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_binarize_otsu),
    },
    Function {
        name: "math.six.blank",
        signature: "(radius: usize, orient: math.six.Orientation, fill: u8, void: u8) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_blank),
    },
    Function {
        name: "math.six.blur",
        signature: "(self: Cell6d, mask: Tensor, wrap: bool) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_blur),
    },
    Function {
        name: "math.six.census",
        signature: "(cell: Cell6d, include_grid: bool) -> math.six.Census",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_census),
    },
    Function {
        name: "math.six.components",
        signature: "(cell: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_components),
    },
    Function {
        name: "math.six.copies",
        signature: "(rings: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_copies),
    },
    Function {
        name: "math.six.cut",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_cut),
    },
    Function {
        name: "math.six.cut_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_cut_design),
    },
    Function {
        name: "math.six.disc",
        signature: "(cell: Cell6d, rings: usize) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_disc),
    },
    Function {
        name: "math.six.east",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_east),
    },
    Function {
        name: "math.six.euler",
        signature: "(cell: Cell6d, include_grid: bool) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_euler),
    },
    Function {
        name: "math.six.fills",
        signature: "(cell: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_fills),
    },
    Function {
        name: "math.six.fills_only",
        signature: "(cell: Cell6d) -> math.six.Census",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_fills_only),
    },
    Function {
        name: "math.six.framed",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_framed),
    },
    Function {
        name: "math.six.from_json",
        signature: "(text: str) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_from_json),
    },
    Function {
        name: "math.six.giant",
        signature: "(cell: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_giant),
    },
    Function {
        name: "math.six.giant_network",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_giant_network),
    },
    Function {
        name: "math.six.height",
        signature: "(self: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_height),
    },
    Function {
        name: "math.six.holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_holes),
    },
    Function {
        name: "math.six.image",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_image),
    },
    Function {
        name: "math.six.is_cube",
        signature: "(cell: Cell3d) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_is_cube),
    },
    Function {
        name: "math.six.is_hex",
        signature: "(cell: Cell2d) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_is_hex),
    },
    Function {
        name: "math.six.iso",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_iso),
    },
    Function {
        name: "math.six.iso_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_iso_design),
    },
    Function {
        name: "math.six.new",
        signature: "(cell: Cell2d, projection: math.six.Projection, orientation: math.six.Orientation, start: u8) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_new),
    },
    Function {
        name: "math.six.north",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_north),
    },
    Function {
        name: "math.six.orientation",
        signature: "(width: usize, height: usize) -> math.six.Orientation",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_orientation),
    },
    Function {
        name: "math.six.pad",
        signature: "(cell: Cell6d, k: usize, value: u8) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_pad),
    },
    Function {
        name: "math.six.paint",
        signature: "(cell: Cell6d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_paint),
    },
    Function {
        name: "math.six.perforate",
        signature: "(self: Cell6d, mask: Tensor, value: u8) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_perforate),
    },
    Function {
        name: "math.six.pro",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_pro),
    },
    Function {
        name: "math.six.pro_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_pro_design),
    },
    Function {
        name: "math.six.radial",
        signature: "(cell: Cell6d, radius: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_radial),
    },
    Function {
        name: "math.six.radial_crop",
        signature: "(cell: Cell2d, radius: usize, size: (usize, usize)) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_radial_crop),
    },
    Function {
        name: "math.six.radial_mask",
        signature: "(radius: usize, orient: math.six.Orientation) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_radial_mask),
    },
    Function {
        name: "math.six.raster",
        signature: "(cell: Cell6d, size: usize) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_raster),
    },
    Function {
        name: "math.six.rect_image",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_rect_image),
    },
    Function {
        name: "math.six.rect_svg",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_rect_svg),
    },
    Function {
        name: "math.six.rim_holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_rim_holes),
    },
    Function {
        name: "math.six.skin",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_skin),
    },
    Function {
        name: "math.six.slice_core_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_slice_core_graph),
    },
    Function {
        name: "math.six.slice_dual_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_slice_dual_graph),
    },
    Function {
        name: "math.six.slice_edge_graph",
        signature: "(cell: Cell6d, value: u8?) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_slice_edge_graph),
    },
    Function {
        name: "math.six.slice_tunnel_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_slice_tunnel_graph),
    },
    Function {
        name: "math.six.south",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_south),
    },
    Function {
        name: "math.six.spectral_exponent",
        signature: "(cell: Cell6d, window: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_spectral_exponent),
    },
    Function {
        name: "math.six.star.Branch.constant",
        signature: "(self: math.six.star.Branch) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_branch_constant),
    },
    Function {
        name: "math.six.star.Branch.name",
        signature: "(self: math.six.star.Branch) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_branch_name),
    },
    Function {
        name: "math.six.star.Branch.of",
        signature: "(layers: usize) -> math.six.star.Branch",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_branch_of),
    },
    Function {
        name: "math.six.star.Branch.residual",
        signature: "(self: math.six.star.Branch) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_branch_residual),
    },
    Function {
        name: "math.six.star.Share.reduced",
        signature: "(self: math.six.star.Share) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_share_reduced),
    },
    Function {
        name: "math.six.star.Share.value",
        signature: "(self: math.six.star.Share) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_share_value),
    },
    Function {
        name: "math.six.star.Star.arm",
        signature: "(self: math.six.star.Star, number: usize, half: usize) -> math.six.star.Share",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_star_arm),
    },
    Function {
        name: "math.six.star.Star.cell",
        signature: "(self: math.six.star.Star, number: usize, x: i64, z: i64) -> bool?",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_star_cell),
    },
    Function {
        name: "math.six.star.Star.excesses",
        signature: "(self: math.six.star.Star, layers: usize, half: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_star_excesses),
    },
    Function {
        name: "math.six.star.Star.hexagon",
        signature: "(self: math.six.star.Star, number: usize) -> math.six.star.Share",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_star_hexagon),
    },
    Function {
        name: "math.six.star.Star.new",
        signature: "(code: u128) -> math.six.star.Star",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_star_new),
    },
    Function {
        name: "math.six.star.arm_law",
        signature: "(number: usize) -> math.six.star.Share",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_arm_law),
    },
    Function {
        name: "math.six.star.chi8",
        signature: "(number: usize) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_chi8),
    },
    Function {
        name: "math.six.star.constant",
        signature: "() -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_constant),
    },
    Function {
        name: "math.six.star.decay",
        signature: "(excesses: [f64], layers: usize) -> math.six.star.Decay",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_decay),
    },
    Function {
        name: "math.six.star.width_law",
        signature: "(half: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_star_width_law),
    },
    Function {
        name: "math.six.svg",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize, start: usize?) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_svg),
    },
    Function {
        name: "math.six.tessellate",
        signature: "(cell: Cell6d, mask: Tensor) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_tessellate),
    },
    Function {
        name: "math.six.tile",
        signature: "(cell: Cell6d, width: usize, height: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_tile),
    },
    Function {
        name: "math.six.tile_cell",
        signature: "(cell: Cell6d, width: usize, height: usize, crop: bool) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_tile_cell),
    },
    Function {
        name: "math.six.tile_crop",
        signature: "(cell: Cell2d, size: (usize, usize)) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_tile_crop),
    },
    Function {
        name: "math.six.tile_step",
        signature: "(size: (usize, usize)) -> (usize, usize)",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_tile_step),
    },
    Function {
        name: "math.six.to_json",
        signature: "(cell: Cell6d) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_to_json),
    },
    Function {
        name: "math.six.triangles",
        signature: "(cell: Cell6d, start: usize?) -> [([(i64, i64); 3], [u8; 4])]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_triangles),
    },
    Function {
        name: "math.six.west",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_west),
    },
    Function {
        name: "math.six.width",
        signature: "(self: Cell6d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_six_width),
    },
    Function {
        name: "math.spectrum.clusters",
        signature: "(eigenvalues: [f64], tolerance: f64) -> [(f64, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_clusters),
    },
    Function {
        name: "math.spectrum.laplacian",
        signature: "(network: math.graph.Network, normalised: bool) -> [[f64]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian),
    },
    Function {
        name: "math.spectrum.laplacian_spectrum",
        signature: "(network: math.graph.Network, normalised: bool) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian_spectrum),
    },
    Function {
        name: "math.spectrum.multiplicity",
        signature: "(eigenvalues: [f64], value: f64, tolerance: f64) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_multiplicity),
    },
    Function {
        name: "math.spectrum.spectral_exponent",
        signature: "(eigenvalues: [f64], window: f64) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_exponent),
    },
    Function {
        name: "math.spectrum.spectral_fit",
        signature: "(eigenvalues: [f64], window: f64) -> (f64, f64, usize)?",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_fit),
    },
    Function {
        name: "math.spectrum.spectral_points",
        signature: "(eigenvalues: [f64]) -> [(f64, f64)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_points),
    },
    Function {
        name: "math.spectrum.symmetric_eigenvalues",
        signature: "(matrix: [[f64]]) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spectrum_symmetric_eigenvalues),
    },
    Function {
        name: "math.spin.Blend.fold",
        signature: "(self: math.spin.Blend, values: [f32]) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_blend_fold),
    },
    Function {
        name: "math.spin.Blend.named",
        signature: "(name: str) -> math.spin.Blend?",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_blend_named),
    },
    Function {
        name: "math.spin.arcs",
        signature: "(data: [f32], size: usize, radius: f64) -> [(f64, f64, f32)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_arcs),
    },
    Function {
        name: "math.spin.harmonics",
        signature: "(data: [f32], size: usize, rings: usize, orders: usize) -> [f64]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_harmonics),
    },
    Function {
        name: "math.spin.mass",
        signature: "(profile: [f32], size: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_mass),
    },
    Function {
        name: "math.spin.mass_within",
        signature: "(profile: [f32], size: usize, radius: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_mass_within),
    },
    Function {
        name: "math.spin.petals",
        signature: "(copies: usize, order: usize) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_petals),
    },
    Function {
        name: "math.spin.profile",
        signature: "(data: [f32], size: usize, steps: usize) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_profile),
    },
    Function {
        name: "math.spin.radial",
        signature: "(data: [f32], size: usize, out: usize, copies: usize, step: f64, blend: math.spin.Blend, samples: usize) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_radial),
    },
    Function {
        name: "math.spin.reach",
        signature: "(size: usize) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_reach),
    },
    Function {
        name: "math.spin.ring",
        signature: "(data: [f32], size: usize, radius: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_ring),
    },
    Function {
        name: "math.spin.turns",
        signature: "(power: [f64]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_turns),
    },
    Function {
        name: "math.spin.wheel",
        signature: "(profile: [f32], size: usize) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spin_wheel),
    },
    Function {
        name: "math.spirograph.Seats.default",
        signature: "() -> math.spirograph.Seats",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_seats_default),
    },
    Function {
        name: "math.spirograph.cell",
        signature: "(width: usize, height: usize, reach: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_cell),
    },
    Function {
        name: "math.spirograph.cover",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool, samples: usize, side: usize) -> math.spirograph.Cover",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_cover),
    },
    Function {
        name: "math.spirograph.disc",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> math.spirograph.Disc",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_disc),
    },
    Function {
        name: "math.spirograph.distinct",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_distinct),
    },
    Function {
        name: "math.spirograph.frame",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> [f64; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_frame),
    },
    Function {
        name: "math.spirograph.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> u64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_nodes),
    },
    Function {
        name: "math.spirograph.pencils",
        signature: "(types: [u8], width: usize, height: usize, mode: str, reach: f64, jitter: f64, seed: u32) -> [math.spirograph.Pencil]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_pencils),
    },
    Function {
        name: "math.spirograph.point",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil, s: f64) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_point),
    },
    Function {
        name: "math.spirograph.pose",
        signature: "(track: math.spirograph.Track, s: f64) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_pose),
    },
    Function {
        name: "math.spirograph.representatives",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_representatives),
    },
    Function {
        name: "math.spirograph.seats",
        signature: "(pencils: [math.spirograph.Pencil]) -> math.spirograph.Seats",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_seats),
    },
    Function {
        name: "math.spirograph.signed_area",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_signed_area),
    },
    Function {
        name: "math.spirograph.trace",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_trace),
    },
    Function {
        name: "math.spirograph.track",
        signature: "(kind: str, ring: usize, wheel: usize, sides: usize, laps: usize) -> math.spirograph.Track",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_track),
    },
    Function {
        name: "math.spirograph.turn",
        signature: "(track: math.spirograph.Track, s: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_spirograph_turn),
    },
    Function {
        name: "math.sumset.Pair.clean",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_clean),
    },
    Function {
        name: "math.sumset.Pair.copy",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_copy),
    },
    Function {
        name: "math.sumset.Pair.energy",
        signature: "(self: math.sumset.Pair) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_energy),
    },
    Function {
        name: "math.sumset.Pair.four",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_four),
    },
    Function {
        name: "math.sumset.Pair.gap",
        signature: "(self: math.sumset.Pair) -> (u64, u64)?",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_gap),
    },
    Function {
        name: "math.sumset.Pair.largest",
        signature: "(self: math.sumset.Pair) -> u64",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_largest),
    },
    Function {
        name: "math.sumset.Pair.new",
        signature: "(three: u32, four: u32) -> math.sumset.Pair",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_new),
    },
    Function {
        name: "math.sumset.Pair.ratio",
        signature: "(self: math.sumset.Pair, energy: u128) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_ratio),
    },
    Function {
        name: "math.sumset.Pair.scale",
        signature: "(self: math.sumset.Pair) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_scale),
    },
    Function {
        name: "math.sumset.Pair.three",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pair_three),
    },
    Function {
        name: "math.sumset.Sumset.contains",
        signature: "(self: math.sumset.Sumset, x: u64) -> bool?",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.count",
        signature: "(self: math.sumset.Sumset, x: u64) -> u64?",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.density",
        signature: "(self: math.sumset.Sumset, x: u64) -> f64?",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.extremes",
        signature: "(self: math.sumset.Sumset, edges: [u64]) -> [(f64, f64)?]",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.fills",
        signature: "(self: math.sumset.Sumset, low: u64, high: u64, cells: usize) -> [f64]",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.level",
        signature: "(self: math.sumset.Sumset) -> u32",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.new",
        signature: "(level: u32) -> math.sumset.Sumset",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Serialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.top",
        signature: "(self: math.sumset.Sumset) -> u64",
        docs: "",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.pairs",
        signature: "(level: u32) -> [math.sumset.Pair]",
        docs: "",
        uncallable: None,
        door: Some(door_math_sumset_pairs),
    },
    Function {
        name: "math.three.Vec3.cross",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> math.three.Vec3",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_vec3_cross),
    },
    Function {
        name: "math.three.Vec3.dot",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> f32",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_vec3_dot),
    },
    Function {
        name: "math.three.Vec3.new",
        signature: "(x: f32, y: f32, z: f32) -> math.three.Vec3",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_vec3_new),
    },
    Function {
        name: "math.three.Vec3.scale",
        signature: "(self: math.three.Vec3, s: f32) -> math.three.Vec3",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_vec3_scale),
    },
    Function {
        name: "math.three.carpet",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_carpet),
    },
    Function {
        name: "math.three.census",
        signature: "(cell: Cell3d) -> math.three.Census",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_census),
    },
    Function {
        name: "math.three.core_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_core_graph),
    },
    Function {
        name: "math.three.create",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_create),
    },
    Function {
        name: "math.three.diagonal_slice",
        signature: "(code: Code, number: usize, level: usize, base: usize, height: usize) -> [[u32; 3]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_diagonal_slice),
    },
    Function {
        name: "math.three.diagonal_svg",
        signature: "(code: Code, number: usize, level: usize, base: usize, heights: [usize], scale: usize) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_diagonal_svg),
    },
    Function {
        name: "math.three.dust",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_dust),
    },
    Function {
        name: "math.three.edge_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_edge_graph),
    },
    Function {
        name: "math.three.euler",
        signature: "(cell: Cell3d) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_euler),
    },
    Function {
        name: "math.three.extrude",
        signature: "(cell: Cell2d, axis: usize, depth: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_extrude),
    },
    Function {
        name: "math.three.extrude_cube",
        signature: "(cell: Cell3d, depth: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_extrude_cube),
    },
    Function {
        name: "math.three.faces",
        signature: "(cell: Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_faces),
    },
    Function {
        name: "math.three.fills",
        signature: "(cell: Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_fills),
    },
    Function {
        name: "math.three.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, base: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_from_corners),
    },
    Function {
        name: "math.three.from_json",
        signature: "(text: str) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_from_json),
    },
    Function {
        name: "math.three.from_strings",
        signature: "(data: [[String]]) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_from_strings),
    },
    Function {
        name: "math.three.hidden",
        signature: "(cell: Cell3d) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_hidden),
    },
    Function {
        name: "math.three.level_set",
        signature: "(number: usize, levels: [usize], level: usize, base: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_level_set),
    },
    Function {
        name: "math.three.magic",
        signature: "(cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_magic),
    },
    Function {
        name: "math.three.manhattan_layers",
        signature: "(cell: Cell3d) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_manhattan_layers),
    },
    Function {
        name: "math.three.merge",
        signature: "(cells: [Cell3d], width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_merge),
    },
    Function {
        name: "math.three.mosaic",
        signature: "(mask: Tensor, cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_mosaic),
    },
    Function {
        name: "math.three.named",
        signature: "(design: math.bang.catalog.Design, number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_named),
    },
    Function {
        name: "math.three.net",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_net),
    },
    Function {
        name: "math.three.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_noise),
    },
    Function {
        name: "math.three.ones",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_ones),
    },
    Function {
        name: "math.three.orientations",
        signature: "() -> [(usize, usize, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_orientations),
    },
    Function {
        name: "math.three.point",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_point),
    },
    Function {
        name: "math.three.profile",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> [u128]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_profile),
    },
    Function {
        name: "math.three.project",
        signature: "(point: [u32; 3]) -> (f64, f64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_project),
    },
    Function {
        name: "math.three.quads",
        signature: "(cell: Cell3d) -> [math.three.Quad]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_quads),
    },
    Function {
        name: "math.three.shadow",
        signature: "(point: [u32; 3]) -> (i64, i64)",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_shadow),
    },
    Function {
        name: "math.three.slice",
        signature: "(cell: Cell3d, axis: usize, index: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_slice),
    },
    Function {
        name: "math.three.special",
        signature: "(mask: Tensor, cell: Cell3d) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_special),
    },
    Function {
        name: "math.three.sponge.deep",
        signature: "(radius: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_deep),
    },
    Function {
        name: "math.three.sponge.dimension",
        signature: "() -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_dimension),
    },
    Function {
        name: "math.three.sponge.distance",
        signature: "(point: [f64; 3]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_distance),
    },
    Function {
        name: "math.three.sponge.exact",
        signature: "(radius: f64) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_exact),
    },
    Function {
        name: "math.three.sponge.profile",
        signature: "(radius: f64) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_profile),
    },
    Function {
        name: "math.three.sponge.reading",
        signature: "(radius: f64) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_reading),
    },
    Function {
        name: "math.three.sponge.tube",
        signature: "(radius: f64) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_tube),
    },
    Function {
        name: "math.three.sponge.volume",
        signature: "(radius: f64) -> f64?",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_sponge_volume),
    },
    Function {
        name: "math.three.star",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_star),
    },
    Function {
        name: "math.three.support",
        signature: "(counts: [u128]) -> (usize, usize)?",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_support),
    },
    Function {
        name: "math.three.surface",
        signature: "(cell: Cell3d) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_surface),
    },
    Function {
        name: "math.three.text",
        signature: "(cell: Cell3d, glyphs: {u8: String}?) -> [String]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_text),
    },
    Function {
        name: "math.three.to_json",
        signature: "(cell: Cell3d) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_to_json),
    },
    Function {
        name: "math.three.to_obj",
        signature: "(cell: Cell3d) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_to_obj),
    },
    Function {
        name: "math.three.to_strings",
        signature: "(cell: Cell3d) -> [[String]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_to_strings),
    },
    Function {
        name: "math.three.tunnel_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_tunnel_graph),
    },
    Function {
        name: "math.three.void",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_void),
    },
    Function {
        name: "math.three.voids",
        signature: "(cell: Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_voids),
    },
    Function {
        name: "math.three.volume",
        signature: "(cell: Cell3d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_volume),
    },
    Function {
        name: "math.three.wires",
        signature: "(cell: Cell3d) -> [[math.three.Vec3; 2]]",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_wires),
    },
    Function {
        name: "math.three.xline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_xline),
    },
    Function {
        name: "math.three.xtree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_xtree),
    },
    Function {
        name: "math.three.yline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_yline),
    },
    Function {
        name: "math.three.ytree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_ytree),
    },
    Function {
        name: "math.three.zeros",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_zeros),
    },
    Function {
        name: "math.three.zline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_zline),
    },
    Function {
        name: "math.three.ztree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_three_ztree),
    },
    Function {
        name: "math.tourbillon.eyes",
        signature: "(qmax: usize) -> [math.tourbillon.Eye]",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_eyes),
    },
    Function {
        name: "math.tourbillon.field",
        signature: "(top: usize, size: usize, schedule: str, increment: f64, set: str, weights: str, mode: str, blend: str, seed: u32) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_field),
    },
    Function {
        name: "math.tourbillon.layers",
        signature: "(top: usize, schedule: str, increment: f64, set: str, weights: str, seed: u32) -> [math.tourbillon.Layer]",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_layers),
    },
    Function {
        name: "math.tourbillon.period",
        signature: "(increment: f64) -> usize?",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_period),
    },
    Function {
        name: "math.tourbillon.sharing",
        signature: "(list: [math.tourbillon.Layer]) -> (usize, usize)",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_sharing),
    },
    Function {
        name: "math.tourbillon.stack",
        signature: "(list: [math.tourbillon.Layer], size: usize, mode: str, blend: math.spin.Blend) -> [f32]",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_stack),
    },
    Function {
        name: "math.tourbillon.stats",
        signature: "(field: [f32], size: usize, top: usize, schedule: str, increment: f64, set: str, weights: str, blend: str, seed: u32) -> math.tourbillon.Stats",
        docs: "",
        uncallable: None,
        door: Some(door_math_tourbillon_stats),
    },
    Function {
        name: "math.two.capacity",
        signature: "(cell: Cell2d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_capacity),
    },
    Function {
        name: "math.two.carpet",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_carpet),
    },
    Function {
        name: "math.two.census",
        signature: "(cell: Cell2d) -> math.two.Census",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_census),
    },
    Function {
        name: "math.two.create",
        signature: "(code: Code, number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_create),
    },
    Function {
        name: "math.two.dust",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_dust),
    },
    Function {
        name: "math.two.embed",
        signature: "(cell: Cell2d, payload: [u8]) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_embed),
    },
    Function {
        name: "math.two.euler",
        signature: "(cell: Cell2d) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_euler),
    },
    Function {
        name: "math.two.extract",
        signature: "(carrier: Cell2d, carried: Cell2d) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_extract),
    },
    Function {
        name: "math.two.fills",
        signature: "(cell: Cell2d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_fills),
    },
    Function {
        name: "math.two.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_from_corners),
    },
    Function {
        name: "math.two.from_json",
        signature: "(text: str) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_from_json),
    },
    Function {
        name: "math.two.from_strings",
        signature: "(rows: [String]) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_from_strings),
    },
    Function {
        name: "math.two.hline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_hline),
    },
    Function {
        name: "math.two.htree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_htree),
    },
    Function {
        name: "math.two.image",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_image),
    },
    Function {
        name: "math.two.level_set",
        signature: "(number: usize, levels: [usize], level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_level_set),
    },
    Function {
        name: "math.two.mask",
        signature: "(mask: Tensor, shape: [usize]) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_mask),
    },
    Function {
        name: "math.two.merge",
        signature: "(cells: [Cell2d], width: usize, height: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_merge),
    },
    Function {
        name: "math.two.named",
        signature: "(design: math.bang.catalog.Design, number: usize, level: usize, rotation: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_named),
    },
    Function {
        name: "math.two.net",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_net),
    },
    Function {
        name: "math.two.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_noise),
    },
    Function {
        name: "math.two.ones",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_ones),
    },
    Function {
        name: "math.two.payload.frame",
        signature: "() -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_payload_frame),
    },
    Function {
        name: "math.two.perimeter",
        signature: "(cell: Cell2d) -> u128",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_perimeter),
    },
    Function {
        name: "math.two.point",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_point),
    },
    Function {
        name: "math.two.read",
        signature: "(sheet: Cell2d, carrier: Cell2d) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_read),
    },
    Function {
        name: "math.two.sheet",
        signature: "(cells: [Cell2d; 4], payload: [u8]) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_sheet),
    },
    Function {
        name: "math.two.special",
        signature: "(mask: Tensor, cell: Cell2d) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_special),
    },
    Function {
        name: "math.two.star",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_star),
    },
    Function {
        name: "math.two.svg",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_svg),
    },
    Function {
        name: "math.two.text",
        signature: "(cell: Cell2d, glyphs: {u8: String}?) -> [String]",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_text),
    },
    Function {
        name: "math.two.to_3d",
        signature: "(cell: Cell2d) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_to_3d),
    },
    Function {
        name: "math.two.to_json",
        signature: "(cell: Cell2d) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_to_json),
    },
    Function {
        name: "math.two.vline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_vline),
    },
    Function {
        name: "math.two.void",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_void),
    },
    Function {
        name: "math.two.voids",
        signature: "(cell: Cell2d) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_voids),
    },
    Function {
        name: "math.two.vtree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_vtree),
    },
    Function {
        name: "math.two.zeros",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_math_two_zeros),
    },
];

fn door_math_arcs_curves(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::curves(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_draw(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::draw(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::law(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<bool> = take!(name, 1, &args[1]);
    match mrly_math::arcs::trace(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_essential(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::essential(&a0)))
}

fn door_math_arrays_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::arrays::fractal(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::holes(&a0)))
}

fn door_math_arrays_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::arrays::law(&a0, a1)))
}

fn door_math_arrays_lost(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::arrays::lost(&a0, a1)))
}

fn door_math_arrays_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::paired(&a0)))
}

fn door_math_arrays_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::weights(&a0)))
}

fn door_math_atoms_carpet_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::carpet_2d(a0)))
}

fn door_math_atoms_carpet_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::carpet_3d(a0)))
}

fn door_math_atoms_carpet_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::carpet_nd(a0, a1)))
}

fn door_math_atoms_dust_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::dust_2d(a0)))
}

fn door_math_atoms_dust_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::dust_3d(a0)))
}

fn door_math_atoms_dust_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::dust_nd(a0, a1)))
}

fn door_math_atoms_hline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::hline_2d(a0)))
}

fn door_math_atoms_htree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::htree_2d(a0)))
}

fn door_math_atoms_line_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::atoms::line_nd(a0, a1, a2)))
}

fn door_math_atoms_net_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::net_2d(a0)))
}

fn door_math_atoms_net_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::net_3d(a0)))
}

fn door_math_atoms_net_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::net_nd(a0, a1)))
}

fn door_math_atoms_noise_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly_math::atoms::noise_2d(a0, a1, &mut a2)))
}

fn door_math_atoms_noise_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly_math::atoms::noise_3d(a0, a1, &mut a2)))
}

fn door_math_atoms_ones_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ones_2d(a0)))
}

fn door_math_atoms_ones_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ones_3d(a0)))
}

fn door_math_atoms_point_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::point_2d(a0)))
}

fn door_math_atoms_point_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::point_3d(a0)))
}

fn door_math_atoms_point_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::point_nd(a0, a1)))
}

fn door_math_atoms_star_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::star_2d(a0)))
}

fn door_math_atoms_star_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::star_3d(a0)))
}

fn door_math_atoms_star_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::star_nd(a0, a1)))
}

fn door_math_atoms_tree_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::atoms::tree_nd(a0, a1, a2)))
}

fn door_math_atoms_vline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::vline_2d(a0)))
}

fn door_math_atoms_void_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::void_2d(a0)))
}

fn door_math_atoms_void_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::void_3d(a0)))
}

fn door_math_atoms_void_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::void_nd(a0, a1)))
}

fn door_math_atoms_vtree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::vtree_2d(a0)))
}

fn door_math_atoms_xline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::xline_3d(a0)))
}

fn door_math_atoms_xtree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::xtree_3d(a0)))
}

fn door_math_atoms_yline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::yline_3d(a0)))
}

fn door_math_atoms_ytree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ytree_3d(a0)))
}

fn door_math_atoms_zeros_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zeros_2d(a0)))
}

fn door_math_atoms_zeros_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zeros_3d(a0)))
}

fn door_math_atoms_zline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zline_3d(a0)))
}

fn door_math_atoms_ztree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ztree_3d(a0)))
}

fn door_math_automaton_automaton_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::abscissa(&a0)))
}

fn door_math_automaton_automaton_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::base(&a0)))
}

fn door_math_automaton_automaton_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_denominator(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::denominator(&a0)))
}

fn door_math_automaton_automaton_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::matrix(&a0)))
}

fn door_math_automaton_automaton_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    match mrly_math::automaton::Automaton::new(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::peel(&a0)))
}

fn door_math_automaton_automaton_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::period(&a0)))
}

fn door_math_automaton_automaton_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::perron(&a0)))
}

fn door_math_automaton_automaton_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::residue(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::rule(&a0)))
}

fn door_math_automaton_automaton_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::states(&a0)))
}

fn door_math_automaton_automaton_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::automaton::Automaton::with_peel(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::anf(&a0)))
}

fn door_math_bang_design_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::degree(&a0)))
}

fn door_math_bang_design_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    match mrly_math::bang::Design::name(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::rule(&a0)))
}

fn door_math_bang_magiclayer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::MagicLayer::new(a0, a1)))
}

fn door_math_bang_universe_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::all(&a0)))
}

fn door_math_bang_universe_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::canonical(&a0)))
}

fn door_math_bang_universe_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(give!(mrly_math::bang::Universe::design(&a0, a1)))
}

fn door_math_bang_universe_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::distinct(&a0)))
}

fn door_math_bang_universe_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::Universe::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_bang(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::bang(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_axis_maps(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::baseq::axis_maps(a0)))
}

fn door_math_bang_baseq_bracelets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::baseq::bracelets(a0) {
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

fn door_math_bang_baseq_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    match mrly_math::bang::baseq::canonical(&a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly_math::bang::baseq::carry(&a0, a1).get().to_string(),
    ))
}

fn door_math_bang_baseq_class_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::baseq::class_sequence(a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::bang::baseq::classes(a0).to_string(),
    ))
}

fn door_math_bang_baseq_distinct_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::distinct_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_even_fill_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u128 = big(name, 2, &args[2])?;
    match mrly_math::bang::baseq::even_fill_is_balanced(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::baseq::fill_from_corners(&a0, a1, a2).to_string(),
    ))
}

fn door_math_bang_baseq_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::baseq::group(a0, a1)))
}

fn door_math_bang_baseq_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::baseq::orbit(&a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_predicted_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::predicted_group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::representatives(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.get().to_string()),
                        give!(parts1.1),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::sequence(a0, a1) {
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

fn door_math_bang_baseq_total_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::total_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_catalog_design_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::bang::catalog::Design::all()))
}

fn door_math_bang_catalog_antis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::catalog::antis(a0)))
}

fn door_math_bang_catalog_classics(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::catalog::classics(a0)))
}

fn door_math_bang_code_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Code = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::bang::Code::get(a0).to_string()))
}

fn door_math_bang_code_to_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::bang::code_to_corners(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::corners(a0)))
}

fn door_math_bang_corners_to_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::corners_to_code(&a0, a1, a2)
            .get()
            .to_string(),
    ))
}

fn door_math_bang_factory_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::bang::factory::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::bang::factory::create_from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::bang::factory::create_named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_residue_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::factory::residue_corners(a0, a1)))
}

fn door_math_bang_factory_total_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::factory::total_codes(a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_level_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::bang::level_cap(a0, a1, a2)))
}

fn door_math_bang_levels_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::levels_code(a0, a1, &a2).get().to_string(),
    ))
}

fn door_math_bang_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::magic(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_magic_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(String, usize)> = take!(name, 0, &args[0]);
    let b0: Vec<(&str, usize)> = a0.iter().map(|item| (item.0.as_str(), item.1)).collect();
    match mrly_math::bang::magic_named(&b0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_random_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::bang::random_code(a0, a1, &mut a2, a3) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_sources(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::catalog::Catalog = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::sources(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_symmetries(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::symmetries(a0)))
}

fn door_math_bang_total_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::total_exposure(a0, a1)))
}

fn door_math_bang_touches_every_corner(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::touches_every_corner(a0, a1)))
}

fn door_math_bang_universe_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::anf(a0, a1)))
}

fn door_math_bang_universe_anf_string(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::anf_string(a0, a1)))
}

fn door_math_bang_universe_apply(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: (Vec<usize>, Vec<u8>) = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::apply(&a0, &a1)))
}

fn door_math_bang_universe_corner_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::universe::corner_index(&a0)))
}

fn door_math_bang_universe_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::degree(a0, a1)))
}

fn door_math_bang_universe_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::universe::orbit(a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_universe_permutations(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::universe::permutations(a0)))
}

fn door_math_bang_universe_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::universe_codes(a0) {
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

fn door_math_bang_word_schedule_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::bang::word::Schedule::all()))
}

fn door_math_bang_word_schedule_frequencies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::Schedule::frequencies(a0)))
}

fn door_math_bang_word_schedule_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::word::Schedule::place(a0, a1)))
}

fn door_math_bang_word_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::components(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_constant_functional(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::constant_functional(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::dimension(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::fill(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::fills(&a0) {
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

fn door_math_bang_word_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly_math::bang::word::letter(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_native(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::native(&a0)))
}

fn door_math_bang_word_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::period(&a0)))
}

fn door_math_bang_word_prefixes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::prefixes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_rates(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::rates(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_spell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: (mrly_math::bang::MagicLayer, mrly_math::bang::MagicLayer) = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::bang::word::spell(a0, a1, a2)))
}

fn door_math_bang_word_staircase(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::word::staircase(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_thue_morse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::thue_morse(a0)))
}

fn door_math_cell_census_edges(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::edges::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::edges::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_exposure(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly_math::cell::census::exposure::<2>(&a0).to_string(),
            ))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly_math::cell::census::exposure::<3>(&a0).to_string(),
            ))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_fills(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::fills::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::fills::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_vertices(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::vertices::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::vertices::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_voids(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::voids::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::voids::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_merge_reps(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly_math::cell::geometry::merge_reps::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly_math::cell::geometry::merge_reps::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 1)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::geometry::perforate::<2>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::geometry::perforate::<3>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_grow(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::grow::<2>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::grow::<3>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_anti(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::anti(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::anti(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::binarize(
                a0, a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::binarize(
                a0, a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize_otsu(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::binarize_otsu(
                a0
            )))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::binarize_otsu(
                a0
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_blur(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_combine(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::combine(
                &a0, &a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::combine(
                &a0, &a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_counting_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::counting_dtype(&a0)))
}

fn door_math_cell_models_depth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::CellNd::<3>::depth(&a0)))
}

fn door_math_cell_models_dtype_for(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::dtype_for(a0)))
}

fn door_math_cell_models_fractal(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<2>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<3>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_height(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::height(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::height(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_invert(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::invert(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::invert(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_layers(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::layers(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::layers(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_neighbors(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<2>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<3>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_new(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            match mrly_math::cell::models::CellNd::<2>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            match mrly_math::cell::models::CellNd::<3>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_orient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::cell::models::CellNd::<3>::orient(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_models_pad(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::pad(a0, a1, a2)))
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::pad(a0, a1, a2)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly_core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::models::CellNd::<2>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly_core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::models::CellNd::<3>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_rotate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<2>::rotate(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: (usize, usize) = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::rotate(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_tile(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::tile(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            let a3: usize = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<3>::tile(a0, a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_types(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::types(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::types(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_width(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::width(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::width(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::paint::<2>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::paint::<3>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_serializer_byte_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::byte_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_byte_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::byte_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_color_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::color_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::count_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::count_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_tag_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::cell::serializer::tag_layer(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_types_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::types_field(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_coprime_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_math::coprime::gasket(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_closed_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::counts::Closed = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::counts::Closed::text(&a0)))
}

fn door_math_counts_exposure_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::counts::Exposure = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::Exposure::at(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::counts::Exposure::from_corners(
        &a0, a1, a2, a3
    )))
}

fn door_math_counts_exposure_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::counts::Exposure::of_tile(&a0)))
}

fn door_math_counts_exposure_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::counts::Exposure = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::Exposure::recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_centered_hexagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::counts::centered_hexagonal(a0).to_string(),
    ))
}

fn door_math_counts_cut_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::cut_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_cut_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::cut_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::dimension(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_edges_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::edges_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::exposure(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_exposure_of_tile_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::exposure_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_recurrence_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::exposure_recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::fill(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    Ok(Value::String(
        mrly_math::counts::fill_from_corners(&a0, a1, a2, a3, a4).to_string(),
    ))
}

fn door_math_counts_fill_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::fill_polynomial(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::counts::grid(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_ladder_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::ladder::cap(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_carry_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::carry_matrix(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::characteristic(&a0) {
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

fn door_math_counts_ladder_determinant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::determinant(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_digit_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::digit_polynomial(a0, a1) {
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

fn door_math_counts_ladder_even_block(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::even_block(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::fill(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::counts::ladder::ladder(a0, a1, a2) {
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

fn door_math_counts_ladder_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::perron(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::sign(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_spectral_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::spectral_ratio(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    Ok(Value::String(
        mrly_math::counts::ladder::trace(&a0).to_string(),
    ))
}

fn door_math_counts_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::limit(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_odd_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::odd_power(a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::pairs(&a0) {
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

fn door_math_counts_positions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::counts::positions(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_pro_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::pro_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_pro_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::pro_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_profile_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::counts::profile_of_tile(&a0, a1) {
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

fn door_math_counts_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::ratio(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_rational(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::rational(a0, a1, a2, a3, a4) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_grid_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_math::counts::six::grid_triangles(a0, a1).to_string(),
    ))
}

fn door_math_counts_six_solid_slice_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_boundary(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_core_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_core_nodes(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_interior(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_interior(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_triangles(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_vertices(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_vertices(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_skeleton_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrly_math::counts::skeleton_recurrence(a0, a1, a2, a3, a4) {
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

fn door_math_counts_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::surface(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::void(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_digits_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::digits_of(a0, a1)))
}

fn door_math_design_echo_series(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::design::echo_series(&a0, &a1, a2)))
}

fn door_math_design_elements(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::design::elements(a0, &a1, a2)))
}

fn door_math_design_log_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::log_grid(&a0, a1)))
}

fn door_math_design_median_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::median_floor(&a0, a1)))
}

fn door_math_design_meter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::design::meter(&a0)))
}

fn door_math_design_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::nearest(a0, &a1)))
}

fn door_math_design_peaks(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: (f64, f64) = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::design::peaks(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_pole_lattice(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::pole_lattice(a0, a1)))
}

fn door_math_design_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: Vec<f64> = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::design::resample(&a0, &a1, a2, &a3)))
}

fn door_math_design_score(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::score(&a0, a1)))
}

fn door_math_design_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(mrly_math::design::size(&a0, a1).to_string()))
}

fn door_math_design_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    match mrly_math::design::spectrum(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_upper_rms(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::design::upper_rms(&a0)))
}

fn door_math_dissection_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::cap(a0)))
}

fn door_math_dissection_chain_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_exponent(a0)))
}

fn door_math_dissection_chain_margin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_margin(a0)))
}

fn door_math_dissection_chain_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_root(a0)))
}

fn door_math_dissection_chain_wall(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::dissection::chain_wall()))
}

fn door_math_dissection_consecutive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::consecutive(&a0)))
}

fn door_math_dissection_fraction(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::fraction(a0, a1, a2)))
}

fn door_math_dissection_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::dissection::kappa(a0, &a1)))
}

fn door_math_dissection_masses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::masses(a0, &a1, a2)))
}

fn door_math_dissection_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::dissection::reach(a0, a1)))
}

fn door_math_dissection_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::reading(a0, a1, &a2)))
}

fn door_math_dissection_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::regions(a0, a1, a2)))
}

fn door_math_dissection_tally(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::dissection::tally(a0, &a1, a2, a3)))
}

fn door_math_dissection_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::weights(a0, &a1, a2)))
}

fn door_math_game_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::game::cell(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::game::cells(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u64>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::game::counts(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_de_bruijn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::game::de_bruijn(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_mosaic_game(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::game::Mosaic = take!(name, 0, &args[0]);
    match mrly_math::game::mosaic_game(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_named_games(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::game::named_games()))
}

fn door_math_game_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<u64>> = take!(name, 0, &args[0]);
    match mrly_math::game::perron(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_reverse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::game::reverse(&a0)))
}

fn door_math_game_slice_game(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    match mrly_math::game::slice_game(a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_successors(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::game::successors(&a0, &a1)))
}

fn door_math_game_word_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::game::word_cell(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_adjacency(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::Network::adjacency(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::Network::degree(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::graph::Network::new(a0)))
}

fn door_math_graph_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::edge_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_fractal_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::graph::fractal_dimension(&a0, a1)))
}

fn door_math_graph_junctions(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::junctions(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_largest_component(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::largest_component(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_roles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::roles(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_tips(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::tips(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_total_length(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::graph::total_length(&a0)))
}

fn door_math_graph_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::abscissa(&a0)))
}

fn door_math_ladder_design_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::base(&a0)))
}

fn door_math_ladder_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::digits(&a0)))
}

fn door_math_ladder_design_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    match mrly_math::ladder::Design::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::peel(&a0)))
}

fn door_math_ladder_design_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::period(&a0)))
}

fn door_math_ladder_design_pole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::ladder::Design::pole(&a0, a1, a2)))
}

fn door_math_ladder_design_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::ladder::Design::with_peel(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::ladder::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::ladder::residue(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::ladder::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_accepts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::Rule::accepts(&a0, &a1)))
}

fn door_math_memory_rule_allowed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::Rule::allowed(&a0, a1)))
}

fn door_math_memory_rule_alphabet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::alphabet(&a0)))
}

fn door_math_memory_rule_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::memory::Rule::codes(&a0).to_string(),
    ))
}

fn door_math_memory_rule_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::memory::Rule::full(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::letters(&a0)))
}

fn door_math_memory_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_math::memory::Rule::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::states(&a0)))
}

fn door_math_memory_rule_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::windows(&a0)))
}

fn door_math_memory_allowed_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::allowed_windows(&a0)))
}

fn door_math_memory_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::cells(&a0, a1)))
}

fn door_math_memory_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::counts(&a0, a1)))
}

fn door_math_memory_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::exponent(&a0)))
}

fn door_math_memory_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::kappa(&a0)))
}

fn door_math_memory_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::perron(&a0)))
}

fn door_math_memory_transfer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::transfer(&a0)))
}

fn door_math_moire_field_as_f64(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::as_f64(&a0)))
}

fn door_math_moire_field_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::Field::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_field_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::max(&a0)))
}

fn door_math_moire_field_mean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::mean(&a0)))
}

fn door_math_moire_field_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::min(&a0)))
}

fn door_math_moire_field_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::new(a0)))
}

fn door_math_moire_field_normalized(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Field::normalized(&a0, a1)))
}

fn door_math_moire_layer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Layer::new(a0, a1)))
}

fn door_math_moire_preset_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::carpet(a0)))
}

fn door_math_moire_preset_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::heatmap(a0)))
}

fn door_math_moire_preset_hive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::hive(a0)))
}

fn door_math_moire_preset_weave(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::weave(a0)))
}

fn door_math_moire_spec_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::Spec::new(a0, a1, a2)))
}

fn door_math_moire_volume_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::moire::Volume::at(&a0, a1, a2, a3)))
}

fn door_math_moire_volume_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::count(&a0, a1)))
}

fn door_math_moire_volume_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::Volume::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::max(&a0)))
}

fn door_math_moire_volume_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::min(&a0)))
}

fn door_math_moire_volume_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::new(a0)))
}

fn door_math_moire_volume_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: mrly_math::moire::Frame = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::Volume::plane(&a0, &a1, a2)))
}

fn door_math_moire_volume_sample(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: [f64; 3] = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::sample(&a0, a1)))
}

fn door_math_moire_volume_solid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::solid(&a0, a1)))
}

fn door_math_moire_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::all(a0)))
}

fn door_math_moire_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::moire::frame(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Layer = take!(name, 0, &args[0]);
    match mrly_math::moire::layer(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::named(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_correlation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::pairs::correlation(a0, a1)))
}

fn door_math_moire_pairs_sampled(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::pairs::sampled(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::moire::pairs::witness(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    let a1: mrly_core::Colorizer = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly_math::moire::render(&a0, &a1, a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::moire::Lattice = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::sample::axes(a0, a1, a2)))
}

fn door_math_moire_sample_membership(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::moire::sample::membership(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_pack(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::sample::pack(&a0, a1)))
}

fn door_math_moire_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly_math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::moire::Lattice = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    let a6: Vec<f64> = take!(name, 6, &args[6]);
    match mrly_math::moire::stack(a0, &a1, a2, a3, a4, a5, &a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_stack_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<mrly_math::moire::Spec> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly_math::moire::Lattice = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: Vec<f64> = take!(name, 5, &args[5]);
    match mrly_math::moire::stack_codes(&a0, a1, a2, a3, a4, &a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly_math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::moire::volume(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match mrly_math::name::Bang::cells(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::name::Bang::new(a0, a1, a2)))
}

fn door_math_name_bang_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Bang as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_bang_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Bang as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_bang_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_lattice_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::name::Lattice::default()))
}

fn door_math_name_lattice_is_square(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Lattice::is_square(&a0)))
}

fn door_math_name_lattice_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Lattice::units(a0)))
}

fn door_math_name_sequence_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Sequence::design(&a0)))
}

fn door_math_name_sequence_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    Ok(give!(mrly_math::name::Sequence::new(
        a0,
        a1,
        a2,
        a3.as_str(),
        a4.as_str()
    )))
}

fn door_math_name_sequence_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Sequence as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_sequence_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Sequence as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_sequence_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_bases(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Word::bases(&a0)))
}

fn door_math_name_word_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Word::letters(&a0)))
}

fn door_math_name_word_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<(u128, usize)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push({
                let parts1 = parts(name, 1, item0, 2)?;
                (big(name, 1, &parts1[0])?, take!(name, 1, &parts1[1]))
            });
        }
        list0
    };
    match mrly_math::name::Word::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Word as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_word_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Word as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_word_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::name::of(a0, a1, a2)))
}

fn door_math_name_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::name::words(a0, a1)))
}

fn door_math_nodal_figures(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::nodal::figures(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_nodal_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::nodal::holds(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::press::Press::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::press::Press = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly_math::press::Press::total(&a0, a1).to_string(),
    ))
}

fn door_math_press_press_totals(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::press::Press = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::press::Press::totals(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_press_containing(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::containing(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_coordinates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::coordinates(a0, a1, a2) {
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

fn door_math_press_count_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u128 = big(name, 3, &args[3])?;
    match mrly_math::press::count_below(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::distinct(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_interleave(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::press::interleave(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_layer_table(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly_math::press::layer_table(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: u128 = big(name, 1, &args[1])?;
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::members(a0, a1, a2, a3) {
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

fn door_math_press_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::profile(a0, a1, a2, a3) {
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

fn door_math_press_usage(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::usage(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_count(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::press::word_member(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_members(&a0) {
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

fn door_math_press_word_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_profile(&a0) {
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

fn door_math_radix_base_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly_math::radix::Base::class(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_congruent(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::radix::Base::congruent(a0, a1, a2)))
}

fn door_math_radix_base_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    match mrly_math::radix::Base::group(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_mirrored(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::mirrored(a0)))
}

fn door_math_radix_base_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly_math::radix::Base::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::norm(a0)))
}

fn door_math_radix_base_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Base::power(a0, a1)))
}

fn door_math_radix_base_residues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    match mrly_math::radix::Base::residues(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::ring(a0)))
}

fn door_math_radix_base_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::value(a0)))
}

fn door_math_radix_radix_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::base(&a0)))
}

fn door_math_radix_radix_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    match mrly_math::radix::Radix::canonical(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    match mrly_math::radix::Radix::code(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::digits(&a0)))
}

fn door_math_radix_radix_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::dimension(&a0)))
}

fn door_math_radix_radix_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::distinct(&a0, a1)))
}

fn door_math_radix_radix_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_math::radix::Radix::fill(&a0, a1).to_string(),
    ))
}

fn door_math_radix_radix_from_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::radix::Radix::from_code(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: Vec<(i64, i64)> = take!(name, 1, &args[1]);
    let a2: Vec<(i64, i64)> = take!(name, 2, &args[2]);
    match mrly_math::radix::Radix::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::plane(&a0, a1)))
}

fn door_math_radix_radix_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::ring(&a0)))
}

fn door_math_radix_radix_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::size(&a0)))
}

fn door_math_radix_radix_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::twists(&a0)))
}

fn door_math_radix_radix_with_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::radix::Radix::with_twists(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::words(&a0, a1)))
}

fn door_math_radix_flowsnake(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::flowsnake() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::gasket() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_koch(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::koch() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_terdragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::terdragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::radix::tile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_twindragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::twindragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_nodes_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::roulette::Nodes::default()))
}

fn door_math_roulette_nodes_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::Nodes::pair(&a0, a1, a2)))
}

fn door_math_roulette_nodes_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::paired(&a0)))
}

fn door_math_roulette_nodes_selved(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::selved(&a0)))
}

fn door_math_roulette_nodes_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::total(&a0)))
}

fn door_math_roulette_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::roulette::nodes(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: [f64; 2] = take!(name, 0, &args[0]);
    let a1: [f64; 2] = take!(name, 1, &args[1]);
    let a2: [f64; 2] = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::side(a0, a1, a2)))
}

fn door_math_roulette_spread(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::spread(&a0, &a1, a2)))
}

fn door_math_rules_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::rules::render(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_rules_tree_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::rules::tree_axes(a0, a1)))
}

fn door_math_shape_frac_minus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::minus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_plus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::plus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_times(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::times(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::Frac::whole(a0)))
}

fn door_math_shape_region_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::shape::Region = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::Region::flip(a0)))
}

fn door_math_shape_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    match mrly_math::shape::census(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_classify(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrly_math::shape::classify(&a0, a1, &a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Shape = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_math::shape::crop(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crossing_shell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::crossing_shell(a0, a1, a2)))
}

fn door_math_shape_crossing_tree(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: Vec<bool> = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::crossing_tree(a0, a1, &a2)))
}

fn door_math_shape_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::shape::fit(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly_math::shape::Frac = take!(name, 2, &args[2]);
    match mrly_math::shape::named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_radial_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::radial_census(&a0, &a1, a2)))
}

fn door_math_shape_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::shape::reach(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_refine(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Shape = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    match mrly_math::shape::refine(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::shape::regions(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_shapes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::shapes(a0)))
}

fn door_math_six_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::anti(a0)))
}

fn door_math_six_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::Cell6d::binarize(a0, a1)))
}

fn door_math_six_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::binarize_otsu(a0)))
}

fn door_math_six_blank(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Orientation = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    match mrly_math::six::blank(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_math::six::Cell6d::blur(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::census(&a0, a1)))
}

fn door_math_six_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_copies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::six::copies(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::cut(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::cut_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::disc(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_east(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::east(a0, a1)))
}

fn door_math_six_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::euler(&a0, a1)))
}

fn door_math_six_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::fills(&a0)))
}

fn door_math_six_fills_only(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::fills_only(&a0)))
}

fn door_math_six_framed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::framed(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::six::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::giant(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant_network(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::giant_network(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::height(&a0)))
}

fn door_math_six_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::image(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_is_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::is_cube(&a0)))
}

fn door_math_six_is_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::is_hex(&a0)))
}

fn door_math_six_iso(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::iso(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_iso_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::iso_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Projection = take!(name, 1, &args[1]);
    let a2: mrly_math::six::Orientation = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::six::Cell6d::new(a0, a1, a2, a3)))
}

fn door_math_six_north(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::north(a0, a1)))
}

fn door_math_six_orientation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::orientation(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::six::pad(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) {
        None
    } else {
        Some(
            {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect(),
        )
    };
    let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() {
        None
    } else {
        Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
    };
    match mrly_math::six::paint(a0, a1.as_ref(), a2, a3.as_mut()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::six::Cell6d::perforate(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::pro(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::pro_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::radial(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly_math::six::radial_crop(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Orientation = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::radial_mask(a0, a1)))
}

fn door_math_six_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::raster(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly_math::six::rect_image(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly_math::six::rect_svg(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rim_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::rim_holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_skin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::skin(&a0)))
}

fn door_math_six_slice_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_dual_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_dual_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<u8> = take!(name, 1, &args[1]);
    match mrly_math::six::slice_edge_graph(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_south(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::south(a0, a1)))
}

fn door_math_six_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::six::spectral_exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_branch_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::constant(a0)))
}

fn door_math_six_star_branch_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::name(a0)))
}

fn door_math_six_star_branch_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::of(a0)))
}

fn door_math_six_star_branch_residual(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::residual(a0)))
}

fn door_math_six_star_share_reduced(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Share::reduced(a0)))
}

fn door_math_six_star_share_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Share::value(a0)))
}

fn door_math_six_star_star_arm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::star::Star::arm(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::six::star::Star::cell(&a0, a1, a2, a3)))
}

fn door_math_six_star_star_excesses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::star::Star::excesses(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_hexagon(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::star::Star::hexagon(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u128 = big(name, 0, &args[0])?;
    match mrly_math::six::star::Star::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_arm_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::six::star::arm_law(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::chi8(a0)))
}

fn door_math_six_star_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::six::star::constant()))
}

fn door_math_six_star_decay(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::star::decay(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_width_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::width_law(a0)))
}

fn door_math_six_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrly_math::six::svg(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    match mrly_math::six::tessellate(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::tile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly_math::six::tile_cell(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: (usize, usize) = take!(name, 1, &args[1]);
    match mrly_math::six::tile_crop(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: (usize, usize) = take!(name, 0, &args[0]);
    match mrly_math::six::tile_step(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::to_json(&a0)))
}

fn door_math_six_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<usize> = take!(name, 1, &args[1]);
    match mrly_math::six::triangles(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_west(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::west(a0, a1)))
}

fn door_math_six_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::width(&a0)))
}

fn door_math_spectrum_clusters(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::spectrum::clusters(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_math::spectrum::laplacian(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_math::spectrum::laplacian_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_multiplicity(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spectrum::multiplicity(&a0, a1, a2)))
}

fn door_math_spectrum_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spectrum::spectral_exponent(&a0, a1)))
}

fn door_math_spectrum_spectral_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spectrum::spectral_fit(&a0, a1)))
}

fn door_math_spectrum_spectral_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spectrum::spectral_points(&a0)))
}

fn door_math_spectrum_symmetric_eigenvalues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<f64>> = take!(name, 0, &args[0]);
    match mrly_math::spectrum::symmetric_eigenvalues(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_blend_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spin::Blend = take!(name, 0, &args[0]);
    let a1: Vec<f32> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::Blend::fold(a0, &a1)))
}

fn door_math_spin_blend_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::Blend::named(a0.as_str())))
}

fn door_math_spin_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::spin::arcs(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_harmonics(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::spin::harmonics(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_mass(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::mass(&a0, a1)))
}

fn door_math_spin_mass_within(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spin::mass_within(&a0, a1, a2)))
}

fn door_math_spin_petals(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::petals(a0, a1)))
}

fn door_math_spin_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::spin::profile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: mrly_math::spin::Blend = take!(name, 5, &args[5]);
    let a6: usize = take!(name, 6, &args[6]);
    match mrly_math::spin::radial(&a0, a1, a2, a3, a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::reach(a0)))
}

fn door_math_spin_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::spin::ring(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_turns(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::turns(&a0)))
}

fn door_math_spin_wheel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::wheel(&a0, a1)))
}

fn door_math_spirograph_seats_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::spirograph::Seats::default()))
}

fn door_math_spirograph_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::cell(a0, a1, a2)))
}

fn door_math_spirograph_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::spirograph::cover(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    match mrly_math::spirograph::disc(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::distinct(&a0, &a1, a2)))
}

fn door_math_spirograph_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::frame(&a0, &a1)))
}

fn door_math_spirograph_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::nodes(&a0, &a1, a2)))
}

fn door_math_spirograph_pencils(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: f64 = take!(name, 5, &args[5]);
    let a6: u32 = take!(name, 6, &args[6]);
    match mrly_math::spirograph::pencils(&a0, a1, a2, a3.as_str(), a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly_math::spirograph::Pencil = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::point(&a0, &a1, a2)))
}

fn door_math_spirograph_pose(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::pose(&a0, a1)))
}

fn door_math_spirograph_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::representatives(&a0, &a1, a2)))
}

fn door_math_spirograph_seats(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::spirograph::Pencil> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spirograph::seats(&a0)))
}

fn door_math_spirograph_signed_area(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly_math::spirograph::Pencil = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::signed_area(&a0, &a1)))
}

fn door_math_spirograph_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::spirograph::trace(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_track(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::spirograph::track(a0.as_str(), a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::turn(&a0, a1)))
}

fn door_math_sumset_pair_clean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::clean(&a0)))
}

fn door_math_sumset_pair_copy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::copy(&a0)))
}

fn door_math_sumset_pair_energy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::sumset::Pair::energy(&a0).to_string(),
    ))
}

fn door_math_sumset_pair_four(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::four(&a0)))
}

fn door_math_sumset_pair_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::gap(&a0)))
}

fn door_math_sumset_pair_largest(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::largest(&a0)))
}

fn door_math_sumset_pair_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::sumset::Pair::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_sumset_pair_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(give!(mrly_math::sumset::Pair::ratio(&a0, a1)))
}

fn door_math_sumset_pair_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::scale(&a0)))
}

fn door_math_sumset_pair_three(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::three(&a0)))
}

fn door_math_sumset_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_math::sumset::pairs(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_vec3_cross(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly_math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::cross(a0, a1)))
}

fn door_math_three_vec3_dot(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly_math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::dot(a0, a1)))
}

fn door_math_three_vec3_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f32 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    let a2: f32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::three::Vec3::new(a0, a1, a2)))
}

fn door_math_three_vec3_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::scale(a0, a1)))
}

fn door_math_three_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::three::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_core_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::core_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::core_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::create(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::three::diagonal_slice(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Vec<usize> = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly_math::three::diagonal_svg(a0, a1, a2, a3, &a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_edge_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::edge_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::edge_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::three::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::extrude(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::extrude_cube(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_faces(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::faces(&a0)))
}

fn door_math_three_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::fills(&a0)))
}

fn door_math_three_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::from_corners(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::three::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<String>> = take!(name, 0, &args[0]);
    match mrly_math::three::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_hidden(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::three::hidden(&a0).to_string()))
}

fn door_math_three_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::level_set(a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_magic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            match mrly_math::three::magic::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            match mrly_math::three::magic::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_manhattan_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::manhattan_layers(a0)))
}

fn door_math_three_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::merge(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_mosaic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 1)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 1, &args[1]);
            match mrly_math::three::mosaic::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 1, &args[1]);
            match mrly_math::three::mosaic::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::catalog::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::named(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly_core::Rng::new(seed(name, 3, &args[3])?);
    match mrly_math::three::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_orientations(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::three::orientations()))
}

fn door_math_three_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::profile(a0, a1, a2, a3) {
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

fn door_math_three_project(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::project(a0)))
}

fn door_math_three_quads(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::quads(&a0)))
}

fn door_math_three_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::shadow(a0)))
}

fn door_math_three_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::slice(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    match mrly_math::three::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_sponge_deep(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::deep(a0)))
}

fn door_math_three_sponge_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::three::sponge::dimension()))
}

fn door_math_three_sponge_distance(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::distance(a0)))
}

fn door_math_three_sponge_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::exact(a0)))
}

fn door_math_three_sponge_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::profile(a0)))
}

fn door_math_three_sponge_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::reading(a0)))
}

fn door_math_three_sponge_tube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::tube(a0)))
}

fn door_math_three_sponge_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::volume(a0)))
}

fn door_math_three_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_support(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    Ok(give!(mrly_math::three::support(&a0)))
}

fn door_math_three_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::three::surface(&a0).to_string()))
}

fn door_math_three_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::text(&a0, a1.as_ref())))
}

fn door_math_three_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_json(&a0)))
}

fn door_math_three_to_obj(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_obj(&a0)))
}

fn door_math_three_to_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_strings(&a0)))
}

fn door_math_three_tunnel_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::tunnel_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::tunnel_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::voids(&a0)))
}

fn door_math_three_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::volume(&a0)))
}

fn door_math_three_wires(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::wires(&a0)))
}

fn door_math_three_xline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::xline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_xtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::xtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_yline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::yline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ytree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ytree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::zline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ztree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ztree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_eyes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::eyes(a0)))
}

fn door_math_tourbillon_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrly_math::tourbillon::field(
        a0,
        a1,
        a2.as_str(),
        a3,
        a4.as_str(),
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: String = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: u32 = take!(name, 5, &args[5]);
    match mrly_math::tourbillon::layers(a0, a1.as_str(), a2, a3.as_str(), a4.as_str(), a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::period(a0)))
}

fn door_math_tourbillon_sharing(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::tourbillon::Layer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::sharing(&a0)))
}

fn door_math_tourbillon_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly_math::tourbillon::Layer> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: mrly_math::spin::Blend = take!(name, 3, &args[3]);
    match mrly_math::tourbillon::stack(&a0, a1, a2.as_str(), a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_stats(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrly_math::tourbillon::stats(
        &a0,
        a1,
        a2,
        a3.as_str(),
        a4,
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_capacity(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::capacity(&a0)))
}

fn door_math_two_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly_math::two::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_embed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly_math::two::embed(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly_math::two::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_extract(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::extract(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::fills(&a0)))
}

fn door_math_two_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::two::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    match mrly_math::two::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_hline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::hline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_htree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::htree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::two::Shape = take!(name, 4, &args[4]);
    match mrly_math::two::image(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::level_set(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::two::mask(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::two::merge(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::catalog::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::two::named(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly_core::Rng::new(seed(name, 3, &args[3])?);
    match mrly_math::two::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_payload_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::two::payload::frame()))
}

fn door_math_two_perimeter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::two::perimeter(&a0).to_string()))
}

fn door_math_two_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::read(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_sheet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_math::cell::models::CellNd<2>; 4] = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly_math::two::sheet(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::two::Shape = take!(name, 4, &args[4]);
    Ok(give!(mrly_math::two::svg(&a0, a1, a2, a3, a4)))
}

fn door_math_two_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::two::text(&a0, a1.as_ref())))
}

fn door_math_two_to_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::to_3d(&a0)))
}

fn door_math_two_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::to_json(&a0)))
}

fn door_math_two_vline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::vline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::voids(&a0)))
}

fn door_math_two_vtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::vtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}
