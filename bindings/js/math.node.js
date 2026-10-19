import { readFileSync } from "node:fs";
import { initSync } from "./math.js";

initSync({ module: readFileSync(new URL("./pkg/math/mrly_js_math_bg.wasm", import.meta.url)) });

export * from "./math.js";
export { default } from "./math.js";
