import init, * as math from '@mrlyprod/mrly/math';
import url from '../../pkg/math/mrly_js_math_bg.wasm';
export { math };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
