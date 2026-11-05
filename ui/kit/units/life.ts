import init, * as life from "@mrlyprod/life";
import url from "@mrlyprod/life/life.wasm";

export { life };
export const ready = init({ module_or_path: url });
