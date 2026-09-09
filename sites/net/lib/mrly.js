import init, * as wasm from '../pkg/mrlydemo.js';
import wasmUrl from '../pkg/mrlydemo_bg.wasm';
import { palette, dark, light } from '../../kit/ui/palette.js';

export const mrly = wasm;
export { palette, dark, light };

// THEME

const scheme = typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;

export function isDark() {
  if (typeof document === 'undefined') return true;
  const set = document.documentElement.dataset.theme;
  return set ? set === 'dark' : Boolean(scheme?.matches);
}

export function theme() {
  return isDark() ? dark : light;
}

export const ink = new Proxy({}, { get: (_, key) => theme()[key] });

export const role = () => [ink.dim, ink.yellow, ink.blue, ink.pink];

export const plusminus = () => ({ plus: ink.orange, minus: ink.blue, empty: ink.deep });

function tint() {
  if (typeof wasm.set_theme === 'function') wasm.set_theme(isDark());
}

export async function ready() {
  const at = wasmUrl.startsWith('.') ? new URL(wasmUrl, import.meta.url) : wasmUrl;
  await init({ module_or_path: at });
  globalThis.mrly = wasm;
  tint();
  window.addEventListener('theme', tint);
  scheme?.addEventListener('change', tint);
  return wasm;
}

export function rgb(hex) {
  const n = parseInt(hex.slice(1), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

// FLAT

export function blit(canvas, pixels) {
  canvas.width = pixels.width;
  canvas.height = pixels.height;
  const image = new ImageData(new Uint8ClampedArray(pixels.rgba), pixels.width, pixels.height);
  canvas.getContext('2d').putImageData(image, 0, 0);
}

export function paint(canvas, grid, on = ink.fg, off = ink.deep) {
  const [w, h] = [grid.width, grid.height];
  const a = rgb(on), b = rgb(off);
  const rgba = new Uint8ClampedArray(w * h * 4);
  for (let i = 0; i < w * h; i++) {
    const c = grid.types[i] ? a : b;
    rgba.set(c, i * 4);
    rgba[i * 4 + 3] = 255;
  }
  canvas.width = w;
  canvas.height = h;
  canvas.getContext('2d').putImageData(new ImageData(rgba, w, h), 0, 0);
}

// SIGNED

export function signs(canvas, grid, hues = {}) {
  const [w, h] = [grid.width, grid.height];
  const base = plusminus();
  const ramp = [rgb(hues.plus ?? base.plus), rgb(hues.minus ?? base.minus), rgb(hues.empty ?? base.empty)];
  const rgba = new Uint8ClampedArray(w * h * 4);
  for (let i = 0; i < w * h; i++) {
    rgba.set(ramp[grid.types[i]] ?? ramp[2], i * 4);
    rgba[i * 4 + 3] = 255;
  }
  canvas.width = w;
  canvas.height = h;
  canvas.getContext('2d').putImageData(new ImageData(rgba, w, h), 0, 0);
}

export function fit(canvas, height) {
  const scale = Math.min(devicePixelRatio || 1, 2);
  const w = canvas.clientWidth;
  canvas.width = w * scale;
  canvas.height = height * scale;
  canvas.style.height = height + 'px';
  const ctx = canvas.getContext('2d');
  ctx.setTransform(scale, 0, 0, scale, 0, 0);
  return [ctx, w, height];
}
