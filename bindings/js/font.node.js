import { readFileSync } from "node:fs";
import { initSync } from "./font.js";

initSync({ module: readFileSync(new URL("./pkg/font/mrly_js_font_bg.wasm", import.meta.url)) });

export * from "./font.js";
export { default } from "./font.js";
