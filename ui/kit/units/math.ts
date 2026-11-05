import init, * as math from "@mrlyprod/math";
import url from "@mrlyprod/math/math.wasm";

export { math };
export const ready = init({ module_or_path: url });
