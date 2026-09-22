import * as wasm from "./pkg/font/mrlyjs_font.js";

export { default, initSync } from "./pkg/font/mrlyjs_font.js";
export const Rng = wasm.Rng;
export const FPS = wasm.FPS;
export const Glyph = wasm.Glyph;
export const HOLD = wasm.HOLD;
export const all = wasm.all;
export const animate = wasm.animate;
export const cycle = wasm.cycle;
export const digits = wasm.digits;
export const draft = wasm.draft;
export const extras = wasm.extras;
export const floor = wasm.floor;
export const glyph = wasm.glyph;
export const lower = wasm.lower;
export const lowers = wasm.lowers;
export const map = wasm.map;
export const merge = wasm.merge;
export const name_of = wasm.name_of;
export const path = wasm.path;
export const raster = wasm.raster;
export const specials = wasm.specials;
export const strokes = wasm.strokes;
export const supported = wasm.supported;
export const trim = wasm.trim;
export const uppers = wasm.uppers;
export const paths = {
    penned: wasm.paths_penned,
};
export const pens = {
    all: wasm.pens_all,
};
