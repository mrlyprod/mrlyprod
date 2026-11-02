import init, * as font from "@mrlyprod/font";
import url from "../../../crates/font/js/pkg/mrly_font_js_bg.wasm";

export { font };
export const ready = init({ module_or_path: url });
