import { readFileSync } from "node:fs";
import { initSync } from "./tile.js";

initSync({ module: readFileSync(new URL("./pkg/tile/mrly_js_tile_bg.wasm", import.meta.url)) });

export * from "./tile.js";
export { default } from "./tile.js";
