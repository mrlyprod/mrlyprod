import { pick, rgb } from './frame.js';
import { build, drop } from './gl.js';

const CYCLE = 65536;
const FADE = 2048;
const MAX_ZOOM = 128;
const ROT = 1 / 2048;
const COLOR = 1 / 256;
const START_SCALE = 1 / 2;
const SAMPLES = 200;
const PROBE = 150;

export const MANDELBROT = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };
export const JULIA = { xMin: -1.5, xMax: 1.5, yMin: -1.5, yMax: 1.5 };
export const PRESETS = [[-0.4, 0.6], [-0.8, 0.156], [0.285, 0.01], [-0.7269, 0.1889], [-0.1, 0.651], [0.355, 0.355]];

/* MATH */

export function fit(v, w, h) {
  const vw = v.xMax - v.xMin;
  const vh = v.yMax - v.yMin;
  const ca = w / h;
  const cx = (v.xMin + v.xMax) / 2;
  const cy = (v.yMin + v.yMax) / 2;
  if (ca > vw / vh) {
    const nw = vh * ca;
    return { xMin: cx - nw / 2, xMax: cx + nw / 2, yMin: v.yMin, yMax: v.yMax };
  }
  const nh = vw / ca;
  return { xMin: v.xMin, xMax: v.xMax, yMin: cy - nh / 2, yMax: cy + nh / 2 };
}

export function autoMaxIter(zoom) {
  return 100 + Math.floor(50 * Math.log2(Math.max(zoom, 1)));
}

export function escaper(seed) {
  return (px, py, max) => {
    let zr = seed ? px : 0;
    let zi = seed ? py : 0;
    const cr = seed ? seed[0] : px;
    const ci = seed ? seed[1] : py;
    let n = 0;
    while (zr * zr + zi * zi <= 4 && n < max) {
      const t = zr * zr - zi * zi + cr;
      zi = 2 * zr * zi + ci;
      zr = t;
      n++;
    }
    return n;
  };
}

export function wayfind(escape, v, rand) {
  let best = -1;
  let x = (v.xMin + v.xMax) / 2;
  let y = (v.yMin + v.yMax) / 2;
  for (let i = 0; i < SAMPLES; i++) {
    const px = v.xMin + rand() * (v.xMax - v.xMin);
    const py = v.yMin + rand() * (v.yMax - v.yMin);
    const n = escape(px, py, PROBE);
    const score = n < PROBE ? n : 0;
    if (score > best) {
      best = score;
      x = px;
      y = py;
    }
  }
  return { x, y };
}

/* SAVER */

function fractal(canvas, view, home, seed) {
  const gl = canvas.getContext('webgl2', { powerPreference: 'low-power' });
  if (!gl) return { draw: () => {} };
  const kit = build(gl);
  if (!kit) return { draw: () => {}, stop: () => gl.getExtension('WEBGL_lose_context')?.loseContext() };
  const loc = kit.loc;
  const rand = view.rand;
  const escape = escaper(seed);
  const wide = () => {
    const cx = (home.xMin + home.xMax) / 2;
    const cy = (home.yMin + home.yMax) / 2;
    const hw = (home.xMax - home.xMin) / START_SCALE / 2;
    const hh = (home.yMax - home.yMin) / START_SCALE / 2;
    return fit({ xMin: cx - hw, xMax: cx + hw, yMin: cy - hh, yMax: cy + hh }, view.w, view.h);
  };
  let start = wide();
  let target = wayfind(escape, start, rand);
  let accent = rgb(view.look().accent);
  let dir = rand() < 0.5 ? 1 : -1;
  let born = performance.now();
  let rotation = rand() * Math.PI * 2;
  let clock = 0;
  const again = () => {
    start = wide();
    target = wayfind(escape, start, rand);
    accent = rgb(view.look().accent);
    dir = rand() < 0.5 ? 1 : -1;
    rotation = rand() * Math.PI * 2;
    born = performance.now();
  };
  const draw = () => {
    if (performance.now() - born >= CYCLE) again();
    const age = performance.now() - born;
    const opacity = view.still ? 1 : Math.max(0, Math.min(age / FADE, (CYCLE - age) / FADE, 1));
    canvas.style.opacity = String(opacity);
    if (opacity < 0.01) return;
    const zoom = Math.pow(MAX_ZOOM, age / CYCLE);
    const vw = (start.xMax - start.xMin) / zoom;
    const vh = (start.yMax - start.yMin) / zoom;
    rotation += ROT * dir;
    clock += COLOR;
    const primary = rgb(view.look().paper);
    gl.viewport(0, 0, view.w, view.h);
    gl.uniform2f(loc.resolution, view.w, view.h);
    gl.uniform4f(loc.viewport, target.x - vw / 2, target.x + vw / 2, target.y - vh / 2, target.y + vh / 2);
    gl.uniform1i(loc.julia, seed ? 1 : 0);
    gl.uniform2f(loc.c, seed ? seed[0] : 0, seed ? seed[1] : 0);
    gl.uniform1i(loc.maxIter, autoMaxIter(zoom));
    gl.uniform3f(loc.primary, primary[0] / 255, primary[1] / 255, primary[2] / 255);
    gl.uniform3f(loc.accent, accent[0] / 255, accent[1] / 255, accent[2] / 255);
    gl.uniform1f(loc.rotation, rotation);
    gl.uniform1f(loc.time, clock);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
  };
  const stop = () => {
    canvas.style.opacity = '';
    drop(gl, kit);
  };
  const theme = () => (accent = rgb(view.look().accent));
  return { draw, size: () => (start = wide()), theme, stop };
}

export const mandelbrot = (canvas, view) => fractal(canvas, view, MANDELBROT, null);

export const julia = (canvas, view) => fractal(canvas, view, JULIA, pick(view.rand, PRESETS));
