import init, * as font from 'mrlyjs/font';
import url from '../../pkg/font/mrlyjs_font_bg.wasm';
export { font };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
