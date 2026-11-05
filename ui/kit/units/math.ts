import init, * as math from "@mrlyprod/math";
import url from "@mrlyprod/math/pkg/mrly_math_js_bg.wasm";

export { math };
export const ready = init({ module_or_path: url });
