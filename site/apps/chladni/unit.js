import initNum, * as num from 'mrlyjs/num';
import numUrl from '../../pkg/num/mrlyjs_num_bg.wasm';
import initMath, * as math from 'mrlyjs/math';
import mathUrl from '../../pkg/math/mrlyjs_math_bg.wasm';
export { num, math };
export const ready = Promise.all([initNum({ module_or_path: new URL(numUrl, import.meta.url) }), initMath({ module_or_path: new URL(mathUrl, import.meta.url) })]);
