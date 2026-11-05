import init, * as font from "@mrlyprod/font";
import url from "@mrlyprod/font/font.wasm";

export { font };
export const ready = init({ module_or_path: url });
