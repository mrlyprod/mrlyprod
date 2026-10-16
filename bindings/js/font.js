import * as wasm from "./pkg/font/mrly_js_font.js";

export { default, initSync } from "./pkg/font/mrly_js_font.js";
export const Rng = wasm.Rng;
export const FPS = wasm.font_FPS;
export const Glyph = wasm.font_Glyph;
export const HOLD = wasm.font_HOLD;
export const all = wasm.font_all;
export const animate = wasm.font_animate;
export const cycle = wasm.font_cycle;
export const digits = wasm.font_digits;
export const draft = wasm.font_draft;
export const extras = wasm.font_extras;
export const floor = wasm.font_floor;
export const glyph = wasm.font_glyph;
export const lower = wasm.font_lower;
export const lowers = wasm.font_lowers;
export const map = wasm.font_map;
export const merge = wasm.font_merge;
export const name_of = wasm.font_name_of;
export const path = wasm.font_path;
export const raster = wasm.font_raster;
export const specials = wasm.font_specials;
export const strokes = wasm.font_strokes;
export const supported = wasm.font_supported;
export const trim = wasm.font_trim;
export const uppers = wasm.font_uppers;
export const paths = {
    penned: wasm.font_paths_penned,
};
export const pens = {
    all: wasm.font_pens_all,
};
