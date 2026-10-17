import init, * as font from '@mrlyprod/mrly/font';
import url from '../../../wasm/font/mrly_js_font_bg.wasm';
export { font };
export const ready = init({ module_or_path: new URL(url, import.meta.url) });
