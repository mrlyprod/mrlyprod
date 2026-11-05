import init, * as num from "@mrlyprod/num";
import url from "@mrlyprod/num/num.wasm";

export { num };
export const ready = init({ module_or_path: url });
