import * as wasm from "./pkg/life/mrlyjs_life.js";

export { default, initSync } from "./pkg/life/mrlyjs_life.js";
export const Rng = wasm.Rng;
export const Boundary = wasm.Boundary;
export const Config = wasm.Config;
export const Counts = wasm.Counts;
export const Fate = wasm.Fate;
export const Life = wasm.Life;
export const Rule = wasm.Rule;
export const Source = wasm.Source;
export const affine = wasm.affine;
export const animate = wasm.animate;
export const churn = wasm.churn;
export const corner_bits = wasm.corner_bits;
export const counts = wasm.counts;
export const crop = wasm.crop;
export const cube_orbit = wasm.cube_orbit;
export const design_mask = wasm.design_mask;
export const entropy = wasm.entropy;
export const frames = wasm.frames;
export const gasket = wasm.gasket;
export const genus = wasm.genus;
export const heatmap = wasm.heatmap;
export const history = wasm.history;
export const lambda = wasm.lambda;
export const lattice_index = wasm.lattice_index;
export const mask_offsets = wasm.mask_offsets;
export const moore = wasm.moore;
export const movie = wasm.movie;
export const next_grid = wasm.next_grid;
export const npn_class = wasm.npn_class;
export const outer_totalistic = wasm.outer_totalistic;
export const popcount = wasm.popcount;
export const reversible = wasm.reversible;
export const rule_degree = wasm.rule_degree;
export const rule_name = wasm.rule_name;
export const single_seed = wasm.single_seed;
export const step = wasm.step;
export const surjective = wasm.surjective;
export const tessellate = wasm.tessellate;
export const wolfram_class = wasm.wolfram_class;
export const elementary = {
    output: wasm.elementary_output,
};
export const render = {
    frame: wasm.render_frame,
};
export const source = {
    sequence: wasm.source_sequence,
};
