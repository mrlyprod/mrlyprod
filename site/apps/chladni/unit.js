import initNum, * as num from '@mrlyprod/mrly/num';
import numUrl from '../../pkg/num/mrly_js_num_bg.wasm';
import initMath, * as math from '@mrlyprod/mrly/math';
import mathUrl from '../../pkg/math/mrly_js_math_bg.wasm';
export { num, math };
export const ready = Promise.all([initNum({ module_or_path: new URL(numUrl, import.meta.url) }), initMath({ module_or_path: new URL(mathUrl, import.meta.url) })]);
