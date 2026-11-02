import init, * as life from "@mrlyprod/life";
import url from "@mrlyprod/life/pkg/mrly_life_js_bg.wasm";

export { life };
export const ready = init({ module_or_path: url });
