export * as core from "./core.js";
export * as font from "./font.js";
export * as life from "./life.js";
export * as math from "./math.js";
export * as num from "./num.js";
export * as tile from "./tile.js";

/** Loads every unit's wasm; each unit also loads on its own with its default export. */
export declare function init(): Promise<void>;

export default init;
