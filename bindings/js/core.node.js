import { readFileSync } from "node:fs";
import { initSync } from "./core.js";

initSync({ module: readFileSync(new URL("./pkg/core/mrly_js_core_bg.wasm", import.meta.url)) });

export * from "./core.js";
export { default } from "./core.js";
