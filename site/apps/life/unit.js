import initLife, * as life from 'mrlyjs/life';
import lifeUrl from '../../pkg/life/mrlyjs_life_bg.wasm';
import initMath, * as math from 'mrlyjs/math';
import mathUrl from '../../pkg/math/mrlyjs_math_bg.wasm';
export { life, math };
export const ready = Promise.all([initLife({ module_or_path: new URL(lifeUrl, import.meta.url) }), initMath({ module_or_path: new URL(mathUrl, import.meta.url) })]);
