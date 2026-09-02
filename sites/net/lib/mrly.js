import init, * as wasm from '../pkg/mrlyweb.js';
import wasmUrl from '../pkg/mrlyweb_bg.wasm';

export const mrly = wasm;

export async function ready() {
  const at = wasmUrl.startsWith('.') ? new URL(wasmUrl, import.meta.url) : wasmUrl;
  await init({ module_or_path: at });
  globalThis.mrly = wasm;
  return wasm;
}

export const ink = {
  bg: '#0b0d10', deep: '#07090b', panel: '#12161b', line: '#1f262e', fg: '#e8ecf1', dim: '#7f8a97',
  blue: '#5cc8ff', orange: '#ff8a5c', gold: '#ffd166', green: '#6ee7a8', pink: '#ff7ab6',
};

export const role = [ink.dim, ink.gold, ink.blue, ink.pink];

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

export const plusminus = { plus: ink.orange, minus: ink.blue, empty: ink.deep };

export function signs(canvas, grid, hues = plusminus) {
  const [w, h] = [grid.width, grid.height];
  const ramp = [rgb(hues.plus ?? plusminus.plus), rgb(hues.minus ?? plusminus.minus), rgb(hues.empty ?? plusminus.empty)];
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
