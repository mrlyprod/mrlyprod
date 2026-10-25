import init, * as life from "@mrlyprod/mrly/life";
import url from "../../../wasm/life/mrly_js_life_bg.wasm";

export { life };
export const ready = init({ module_or_path: url });
