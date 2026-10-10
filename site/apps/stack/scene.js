import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { COMBINES, FINE, KINDS, RAMPS, angle, colorizer, field, live, paint, shown, study } from './engine.js';

export const SPEC = [
  { key: 'kind', label: 'Stack', kind: 'segment', def: 'tourbillon', options: KINDS, group: 'Stack' },
  { key: 'limit', label: 'Scales', kind: 'slider', def: 41, min: 3, max: 99, step: 2, group: 'Stack' },
  { key: 'combine', label: 'Combine', kind: 'pick', def: 'sum', options: (value) => (value.kind === 'moire' ? COMBINES : COMBINES.filter(([key]) => key !== 'hive')), group: 'Stack' },
  { key: 'increment', label: 'Turn', kind: 'slider', def: 1, min: 0, max: 360, step: 0.5, unit: 'deg', group: 'Spin', when: (value) => value.kind === 'tourbillon' },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 16, min: 2, max: 64, step: 1, group: 'Look' },
  { key: 'ramp', label: 'Ramp', kind: 'pick', def: 'tint', options: RAMPS, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const DESIGN = { base: 3, code: '', level: 1 };

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [{ key: 'Enter', label: 'Again', act: 'again', button: true }],
  actions: { again: (scene) => scene.again?.() },
};

export const units = () => import('./unit.js');

export const FIT = 0.94;
export const REST = 1000;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return { draw: () => {}, facts: null };
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(SPEC, opts), base: opts.base, code: opts.code, level: opts.level, seed: opts.seed >>> 0 };
  const plan = study(math, value, view.rand);
  const n = plan.numbers.length;
  const kind = value.kind;
  const smooth = kind !== 'moire';
  const quick = kind === 'tourbillon' ? live(plan.layers) : FINE;
  const sheets = new Map();
  let origin = 0;
  let last = -1;
  let at = null;
  let held = null;
  let dirty = true;
  let bins = colorizer(value.ramp, value.levels, view.look());
  const sheet = (size) => {
    if (!sheets.has(size)) sheets.set(size, new OffscreenCanvas(size, size));
    return sheets.get(size);
  };
  const render = () => {
    const image = paint(math, held, bins, value.levels, value.invert);
    const { colors } = image;
    sheet(held.size).getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(colors.buffer, colors.byteOffset, colors.length), held.size, held.size), 0, 0);
    dirty = true;
  };
  const compute = (where, size) => {
    held?.field.free();
    held = field(math, plan, where, size);
    at = where;
    render();
  };
  const blit = () => {
    const side = Math.floor(Math.min(view.w, view.h) * FIT);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = smooth;
    ctx.drawImage(sheet(held.size), Math.floor((view.w - side) / 2), Math.floor((view.h - side) / 2), side, side);
    dirty = false;
  };
  const draw = () => {
    const t = view.t - origin;
    if (kind === 'moire') {
      const k = shown(n, t, view.still);
      if (k !== at) compute(k, FINE);
    } else if (kind === 'tourbillon') {
      const size = view.still || view.fixed || view.t === last ? FINE : quick;
      const turned = angle(value.increment, t, view.still);
      if (turned !== at || size !== held.size) compute(turned, size);
    } else if (!held) compute(n, FINE);
    last = view.t;
    if (dirty) blit();
  };
  const theme = () => {
    bins = colorizer(value.ramp, value.levels, view.look());
    if (held) render();
  };
  const size = () => {
    dirty = true;
  };
  const again = () => {
    origin = view.t;
    if (!view.still) draw();
  };
  const stop = () => {
    held?.field.free();
    held = null;
  };
  const scene = { draw, size, theme, again, stop, facts: plan.facts };
  return kind === 'star' ? { ...scene, every: REST } : scene;
}
