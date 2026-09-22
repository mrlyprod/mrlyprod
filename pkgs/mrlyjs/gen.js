import * as wasm from "./pkg/gen/mrlyjs_gen.js";

export { default, initSync } from "./pkg/gen/mrlyjs_gen.js";
export const Rng = wasm.Rng;
export const Group = wasm.Group;
export const Parity = wasm.Parity;
export const Tile = wasm.Tile;
export const background = wasm.background;
export const classic_code = wasm.classic_code;
export const classic_code_nd = wasm.classic_code_nd;
export const hex_key = wasm.hex_key;
export const random_design = wasm.random_design;
export const random_rotation = wasm.random_rotation;
export const tree_mask = wasm.tree_mask;
export const build = {
    build_2d: wasm.build_build_2d,
    build_3d: wasm.build_build_3d,
    build_6d: wasm.build_build_6d,
    create_2d: wasm.build_create_2d,
    create_3d: wasm.build_create_3d,
    create_6d: wasm.build_create_6d,
    random_tile_2d: wasm.build_random_tile_2d,
    random_tile_3d: wasm.build_random_tile_3d,
    random_tile_6d: wasm.build_random_tile_6d,
};
export const name = {
    Tile: wasm.name_Tile,
};
export const recipe = {
    CLASSICS_2D: wasm.recipe_CLASSICS_2D,
    CLASSICS_3D: wasm.recipe_CLASSICS_3D,
    Design: wasm.recipe_Design,
    MAX_LEVEL: wasm.recipe_MAX_LEVEL,
    MAX_SIDE: wasm.recipe_MAX_SIDE,
    MAX_SLOTS: wasm.recipe_MAX_SLOTS,
    MIN_SIDE: wasm.recipe_MIN_SIDE,
    classics: wasm.recipe_classics,
    generals: wasm.recipe_generals,
    nestings: wasm.recipe_nestings,
    powers: wasm.recipe_powers,
    products: wasm.recipe_products,
    size: wasm.recipe_size,
};
export const variation = {
    File: wasm.variation_File,
    Variation: wasm.variation_Variation,
    create: wasm.variation_create,
    generate: wasm.variation_generate,
    render: wasm.variation_render,
};
