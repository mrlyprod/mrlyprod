import * as wasm from "./pkg/tile/mrly_js_tile.js";

export { default, initSync } from "./pkg/tile/mrly_js_tile.js";
export const Rng = wasm.Rng;
export const Group = wasm.tile_Group;
export const Parity = wasm.tile_Parity;
export const Tile = wasm.tile_Tile;
export const background = wasm.tile_background;
export const classic_code = wasm.tile_classic_code;
export const classic_code_nd = wasm.tile_classic_code_nd;
export const hex_key = wasm.tile_hex_key;
export const random_design = wasm.tile_random_design;
export const random_rotation = wasm.tile_random_rotation;
export const tree_mask = wasm.tile_tree_mask;
export const build = {
    Config2d: wasm.tile_build_Config2d,
    Config3d: wasm.tile_build_Config3d,
    build_2d: wasm.tile_build_build_2d,
    build_3d: wasm.tile_build_build_3d,
    build_6d: wasm.tile_build_build_6d,
    create_2d: wasm.tile_build_create_2d,
    create_3d: wasm.tile_build_create_3d,
    create_6d: wasm.tile_build_create_6d,
    random_tile_2d: wasm.tile_build_random_tile_2d,
    random_tile_3d: wasm.tile_build_random_tile_3d,
    random_tile_6d: wasm.tile_build_random_tile_6d,
};
export const name = {
    Slots: wasm.tile_name_Slots,
    Tile: wasm.tile_name_Tile,
};
export const recipe = {
    CLASSICS_2D: wasm.tile_recipe_CLASSICS_2D,
    CLASSICS_3D: wasm.tile_recipe_CLASSICS_3D,
    Design: wasm.tile_recipe_Design,
    MAX_LEVEL: wasm.tile_recipe_MAX_LEVEL,
    MAX_SIDE: wasm.tile_recipe_MAX_SIDE,
    MAX_SLOTS: wasm.tile_recipe_MAX_SLOTS,
    MIN_SIDE: wasm.tile_recipe_MIN_SIDE,
    classics: wasm.tile_recipe_classics,
    generals: wasm.tile_recipe_generals,
    nestings: wasm.tile_recipe_nestings,
    powers: wasm.tile_recipe_powers,
    products: wasm.tile_recipe_products,
    size: wasm.tile_recipe_size,
};
export const variation = {
    Config: wasm.tile_variation_Config,
    File: wasm.tile_variation_File,
    Variation: wasm.tile_variation_Variation,
    create: wasm.tile_variation_create,
    generate: wasm.tile_variation_generate,
    render: wasm.tile_variation_render,
};
