import initLife, * as life from '@mrlyprod/mrly/life';
import lifeUrl from '../../pkg/life/mrly_js_life_bg.wasm';
import initMath, * as math from '@mrlyprod/mrly/math';
import mathUrl from '../../pkg/math/mrly_js_math_bg.wasm';
export { life, math };
export const ready = Promise.all([initLife({ module_or_path: new URL(lifeUrl, import.meta.url) }), initMath({ module_or_path: new URL(mathUrl, import.meta.url) })]);
