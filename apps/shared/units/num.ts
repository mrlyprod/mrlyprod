import init, * as num from "@mrlyprod/num";
import url from "@mrlyprod/num/pkg/mrly_num_js_bg.wasm";

export { num };
export const ready = init({ module_or_path: url });
