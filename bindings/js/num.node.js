import { readFileSync } from "node:fs";
import { initSync } from "./num.js";

initSync({ module: readFileSync(new URL("./pkg/num/mrly_js_num_bg.wasm", import.meta.url)) });

export * from "./num.js";
export { default } from "./num.js";
