import { tidy } from '../../lib/app/spec.js';
import { TINTS } from '../../lib/color.js';
import { rng } from '../../lib/rand.js';
import { BEAT, COMBINES, CROPS, GRAIN, RAMPS, SIZES, TOP, climb, colorizer, crop, css, field, heat, law, matrix, odd, paint, palette, study } from '../../lib/moire/engine.js';
import { fit, shell } from '../../lib/moire/sheet.js';

const VIEWS = [['field', 'Field'], ['matrix', 'R-matrix']];

const stacks = (value) => value.view !== 'matrix';

const reads = (value) => !stacks(value);

const cropped = (value) => stacks(value) && Boolean(value.crop);

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'field', options: VIEWS, group: 'Stack' },
  { key: 'limit', label: 'Scales up to', kind: 'slider', def: TOP, min: 1, max: TOP, step: 2, group: 'Stack' },
  { key: 'combine', label: 'Combine', kind: 'pick', def: 'sum', options: COMBINES, when: stacks, group: 'Stack' },
  { key: 'size', label: 'Size', kind: 'segment', def: 512, options: SIZES.map((n) => [n, String(n)]), when: stacks, group: 'Stack' },
  { key: 'scale', label: 'Carpet m', kind: 'slider', def: 9, min: 3, max: (value) => Math.max(3, odd(value.limit).at(-1)), step: 2, keep: true, when: reads, group: 'R-matrix' },
  { key: 'mate', label: 'Against n', kind: 'slider', def: 3, min: 3, max: (value) => Math.max(3, value.scale - 2), step: 2, keep: true, when: (value) => reads(value) && value.scale > 3, group: 'R-matrix' },
  { key: 'ramp', label: 'Ramp', kind: 'pick', def: 'tint', options: RAMPS, when: stacks, group: 'Look' },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 16, min: 2, max: 64, step: 1, when: stacks, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, when: stacks, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
  { key: 'crop', label: 'Crop', kind: 'pick', def: '', options: CROPS, when: stacks, group: 'Crop' },
  { key: 'radius', label: 'Radius', kind: 'slider', def: 16, min: 1, max: GRAIN, step: 1, unit: `of ${GRAIN}`, when: cropped, group: 'Crop' },
  { key: 'anti', label: 'Anti', kind: 'toggle', def: 0, when: cropped, group: 'Crop' },
];

export const DESIGN = { base: 2, code: '9', level: 1 };

const named = (value) => (value.view === 'matrix' ? `moire-matrix-${value.limit}` : `moire-${value.code}-${value.limit}`);

export const PAGE = {
  defaults: DESIGN,
  spec: SPEC,
  record: true,
  gestures: true,
  name: named,
  about: [
    'One design sampled at scale 1, 3, 5 and on, the layers stacked into one field. Each new scale lays a finer grid over the coarse ones, and that is where the interference comes from. Play, Space or a tap on the field climbs the stack two at a time.',
    'The heatmap sums the parity of the low corner over the odd scales, the weave folds the same layers to their parity, the hive samples them on the hexagonal lattice, and the carpet keeps eight corners of nine in base 3. The page opens on the void, the base-2 checkerboard, stacked to 41: on both diagonals every layer is lit, so the X is the top of the field.',
    'The R-matrix view reads a law off the stack: two parity carpets at odd scales correlate to exactly zero when the scales are coprime and to a positive number when they share a factor, so a scale whose row is zero against every earlier scale is prime. Tap a cell to pick the pair.',
    'The closed form and its proof are in [the stack note](/research/stack/), and the stack that counts pi is in [the pi note](/research/pi/).',
  ],
  keys: [{ key: 'Enter', label: (value, scene) => (scene?.playing?.() ? 'Stop' : 'Play'), act: 'play', button: true }],
  actions: { play: (scene) => scene.toggle?.(), reroll: (scene) => scene.random?.() },
};

const read = (opts) => ({ ...tidy(SPEC, opts), base: opts.base, code: opts.code, level: opts.level, seed: opts.seed >>> 0 });

const GAP = 0.06;

/* MATRIX */

export function cells(view, n) {
  const box = fit(view.w, view.h);
  const cell = Math.max(1, Math.floor(box.side / Math.max(1, n)));
  const span = cell * n;
  return { x: box.x + Math.floor((box.side - span) / 2), y: box.y + Math.floor((box.side - span) / 2), cell, n };
}

export function cellAt(grid, x, y) {
  const i = Math.floor((y - grid.y) / grid.cell);
  const j = Math.floor((x - grid.x) / grid.cell);
  return i >= 0 && j >= 0 && i < grid.n && j < grid.n ? [i, j] : null;
}

function board(ctx, view, look, table, trial) {
  const { scales, grid: rows } = table;
  const n = scales.length;
  if (!n) return null;
  const ink = palette(look);
  const grid = cells(view, n);
  const { x, y, cell } = grid;
  const gap = Math.max(1, Math.round(cell * GAP));
  rows.forEach((row, i) => row.forEach((r, j) => {
    ctx.fillStyle = css(heat(ink, r));
    ctx.fillRect(x + j * cell, y + i * cell, cell - gap, cell - gap);
  }));
  ctx.lineWidth = Math.max(1, view.dpr);
  ctx.strokeStyle = css(ink.dim);
  ctx.beginPath();
  ctx.moveTo(x, y);
  ctx.lineTo(x + n * cell, y + n * cell);
  ctx.stroke();
  const line = Math.max(2, Math.round(view.dpr * 1.5));
  ctx.lineWidth = line;
  const i = scales.indexOf(trial.row);
  const j = scales.indexOf(trial.mate);
  if (i >= 0) {
    ctx.strokeStyle = css(ink.yellow);
    ctx.strokeRect(x - line, y + i * cell - line, n * cell + line, cell + line);
  }
  if (j >= 0) {
    ctx.strokeStyle = css(ink.pink);
    ctx.strokeRect(x + j * cell - line, y - line, cell + line, n * cell + line);
  }
  return grid;
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  let value = read(opts);
  let plan = null;
  let held = null;
  let trial = null;
  let grid = null;
  let error = '';
  let tone = 0;
  let playing = false;
  let from = value.limit;
  let origin = 0;
  let dirty = true;
  let told = '';
  let failed = '';
  let gone = false;
  const done = { plan: '', field: '', paint: '', law: '' };
  const planKey = () => [value.base, value.code, value.level, value.limit, value.combine, value.seed].join(' ');
  const fieldKey = () => [planKey(), value.size, value.crop, value.radius, value.anti].join(' ');
  const paintKey = () => [fieldKey(), value.ramp, value.levels, value.invert, value.tint, tone].join(' ');
  const lawKey = () => [value.limit, value.scale, value.mate].join(' ');
  const facts = () => {
    if (error) return { error };
    if (reads(value)) {
      const w = trial.witness;
      return { view: 'matrix', row: trial.row, mate: trial.mate, r: trial.r, prime: w?.prime ?? false, max: w?.max ?? 0, at: w?.at ?? 0, k: trial.k };
    }
    return { view: 'field', name: plan.name, code: plan.code, base: plan.base, level: plan.level, layers: plan.numbers.length, scales: plan.numbers, size: held.size };
  };
  const tell = () => {
    if (gone) return;
    const now = facts();
    const key = JSON.stringify(now);
    if (key === told) return;
    told = key;
    opts.onFacts?.(now);
  };
  const stack = () => {
    if (done.plan !== planKey()) {
      plan = study(math, value, rng(value.seed));
      done.plan = planKey();
    }
    if (done.field !== fieldKey()) {
      held?.field.free();
      held = null;
      held = field(math, plan, value.size, crop(math, value.crop, value.radius, value.anti, value.size));
      done.field = fieldKey();
    }
    if (done.paint !== paintKey()) {
      pad.put(paint(math, held, colorizer(value.ramp, value.levels, view.look()), value.levels, value.invert), held.size);
      done.paint = paintKey();
      dirty = true;
    }
  };
  const weigh = () => {
    if (done.law !== lawKey()) {
      trial = law(math, value.limit, value.scale, value.mate);
      done.law = lawKey();
      dirty = true;
    }
  };
  const sync = () => {
    const want = reads(value) ? lawKey() : paintKey();
    if (want === failed) return;
    try {
      if (reads(value)) weigh();
      else stack();
      error = '';
      failed = '';
    } catch (fault) {
      error = String(fault?.message ?? fault);
      failed = want;
      dirty = true;
    }
    tell();
  };
  const frame = () => {
    pad.clear();
    grid = null;
    if (!error && reads(value)) grid = board(pad.ctx, view, view.look(), matrix(math, value.limit), trial);
    else if (!error && held) pad.blit(held.size, fit(view.w, view.h), false);
    dirty = false;
  };
  const draw = () => {
    if (view.fixed && !playing) toggle();
    if (playing) {
      const k = climb(from, 1 + Math.floor((view.t - origin) / BEAT));
      if (k !== value.limit) {
        value = { ...value, limit: k };
        opts.onLimit?.(k);
      }
    }
    sync();
    if (dirty) frame();
  };
  const toggle = () => {
    playing = !playing;
    if (playing) {
      from = value.limit;
      origin = view.t;
      view.wake?.();
    }
    return playing;
  };
  const set = (next) => {
    const was = value.limit;
    value = read(next);
    if (playing && value.limit !== was) {
      from = value.limit;
      origin = view.t;
    }
    dirty = true;
  };
  const touch = (u, v, start) => {
    if (stacks(value)) {
      if (start) toggle();
      return null;
    }
    const at = grid && cellAt(grid, u * view.w, v * view.h);
    if (!at) return null;
    const { scales } = matrix(math, value.limit);
    const [i, j] = at;
    return i === j ? { scale: scales[i] } : { scale: scales[Math.max(i, j)], mate: scales[Math.min(i, j)] };
  };
  const theme = () => {
    tone += 1;
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const stop = () => {
    gone = true;
    held?.field.free();
    held = null;
  };
  return { draw, set, toggle, touch, theme, size, stop, playing: () => playing };
}
