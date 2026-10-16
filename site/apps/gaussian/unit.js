import init, * as num from '@mrlyprod/mrly/num';
import url from '../../pkg/num/mrly_js_num_bg.wasm';
export { num };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
