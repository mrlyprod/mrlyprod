import { readFileSync } from "node:fs";
import { initSync } from "./index.js";

initSync({ module: readFileSync(new URL("./pkg/mrly_num_js_bg.wasm", import.meta.url)) });

export * from "./index.js";
export { default } from "./index.js";
