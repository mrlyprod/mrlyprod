import { readFileSync } from "node:fs";
import { initSync } from "./life.js";

initSync({ module: readFileSync(new URL("./pkg/life/mrly_js_life_bg.wasm", import.meta.url)) });

export * from "./life.js";
export { default } from "./life.js";
