import init, * as num from 'mrlyjs/num';
import url from '../../pkg/num/mrlyjs_num_bg.wasm';
export { num };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
