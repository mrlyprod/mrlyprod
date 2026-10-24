import init, * as num from "@mrlyprod/mrly/num";
import url from "../../../wasm/num/mrly_js_num_bg.wasm";

export { num };
export const ready = init({ module_or_path: url });
