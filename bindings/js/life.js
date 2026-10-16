import * as wasm from "./pkg/life/mrly_js_life.js";

export { default, initSync } from "./pkg/life/mrly_js_life.js";
export const Rng = wasm.Rng;
export const Boundary = wasm.life_Boundary;
export const Config = wasm.life_Config;
export const Counts = wasm.life_Counts;
export const Fate = wasm.life_Fate;
export const Life = wasm.life_Life;
export const Rule = wasm.life_Rule;
export const Source = wasm.life_Source;
export const affine = wasm.life_affine;
export const animate = wasm.life_animate;
export const churn = wasm.life_churn;
export const corner_bits = wasm.life_corner_bits;
export const counts = wasm.life_counts;
export const crop = wasm.life_crop;
export const cube_orbit = wasm.life_cube_orbit;
export const design_mask = wasm.life_design_mask;
export const entropy = wasm.life_entropy;
export const frames = wasm.life_frames;
export const gasket = wasm.life_gasket;
export const genus = wasm.life_genus;
export const heatmap = wasm.life_heatmap;
export const history = wasm.life_history;
export const lambda = wasm.life_lambda;
export const lattice_index = wasm.life_lattice_index;
export const mask_offsets = wasm.life_mask_offsets;
export const moore = wasm.life_moore;
export const next_grid = wasm.life_next_grid;
export const npn_class = wasm.life_npn_class;
export const outer_totalistic = wasm.life_outer_totalistic;
export const popcount = wasm.life_popcount;
export const reversible = wasm.life_reversible;
export const rule_degree = wasm.life_rule_degree;
export const rule_name = wasm.life_rule_name;
export const single_seed = wasm.life_single_seed;
export const step = wasm.life_step;
export const surjective = wasm.life_surjective;
export const tessellate = wasm.life_tessellate;
export const wolfram_class = wasm.life_wolfram_class;
export const elementary = {
    output: wasm.life_elementary_output,
};
export const render = {
    frame: wasm.life_render_frame,
};
export const source = {
    sequence: wasm.life_source_sequence,
};
