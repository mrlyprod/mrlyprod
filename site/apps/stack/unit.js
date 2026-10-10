import init, * as math from 'mrlyjs/math';
import url from '../../pkg/math/mrlyjs_math_bg.wasm';
export { math };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
