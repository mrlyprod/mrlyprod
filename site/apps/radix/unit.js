import initMath, * as math from '@mrlyprod/mrly/math';
import mathUrl from '../../pkg/math/mrly_js_math_bg.wasm';
import initNum, * as num from '@mrlyprod/mrly/num';
import numUrl from '../../pkg/num/mrly_js_num_bg.wasm';
export { math, num };
export const ready = Promise.all([initMath({ module_or_path: new URL(mathUrl, import.meta.url) }), initNum({ module_or_path: new URL(numUrl, import.meta.url) })]);
