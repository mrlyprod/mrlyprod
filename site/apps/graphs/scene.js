import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb } from '../../lib/scene.js';
import { DESIGN } from '../designs/scene.js';
import { FADES, KINDS, LAYOUTS, NUMBERS, PAD, bounds, cap, cube, dimOf, lay, pace, picture, reached, share, study, ticks } from './engine.js';
import { space } from './space.js';

export const SPEC = [
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Lattice' },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value), step: 1, group: 'Lattice' },
  { key: 'graph', label: 'Graph', kind: 'pick', def: '', options: KINDS, group: 'Network' },
  { key: 'layout', label: 'Layout', kind: 'segment', def: '', options: LAYOUTS, group: 'Network' },
  { key: 'grow', label: 'Grow', kind: 'slider', def: 8, min: 0, max: 60, step: 1, unit: 's', group: 'Motion' },
  { key: 'spin', label: 'Spin', kind: 'toggle', def: 1, group: 'Motion', when: (value) => dimOf(value) === 3 },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  record: true,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Graph', act: 'graph' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    graph: (scene, e) => scene.next?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const REST = 1000;
const DOT = 0.22;
const LINE = 0.09;
const DOTS = [1.5, 7];
const LINES = [1, 3];
const BALL = 0.2;
const ROD = 0.07;
const HOME = DESIGN.map((row) => (row.key === 'base' ? { ...row, def: 3 } : row));

const mix = (a, b, t) => `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(', ')})`;

export function tones(accent, paper) {
  const a = rgb(accent);
  const p = rgb(paper);
  return FADES.map((t) => (t ? mix(a, p, t) : accent));
}

const clamp = (x, [lo, hi], dpr) => Math.min(hi * dpr, Math.max(lo * dpr, x));

/* MOTION */

export function motion(math, plan, span, seed) {
  const force = plan.layout === 'force';
  const step = pace(plan.count, plan.dim);
  let layout = null;
  let done = 0;
  let version = 0;
  let positions = plan.positions;
  const at = (t, still) => {
    const k = reached(share(t, span, still), plan.count, plan.dim);
    if (force) {
      const want = ticks(t, span, still, step);
      if (!layout || want < done) {
        layout?.free();
        layout = math.graph.Layout.from_network(plan.net, seed);
        done = 0;
        positions = layout.positions();
        version++;
      }
      if (want > done) {
        layout.step(want - done);
        done = want;
        positions = layout.positions();
        version++;
      }
    }
    return { k, m: plan.order.shown[k], positions, version };
  };
  const free = () => {
    layout?.free();
    layout = null;
  };
  return { at, free };
}

/* STAGES */

function flat(canvas, view, plan, mover, span) {
  const ctx = canvas.getContext('2d');
  const { count, pairs, order, shade } = plan;
  let shades = [];
  let box = null;
  let held = null;
  let dirty = true;
  const paint = () => {
    const { accent, paper } = view.look();
    shades = tones(accent, paper);
  };
  const fit = (positions, w, h, dpr) => {
    const frame = lay(bounds(positions, 2, count), w, h, PAD * dpr);
    return { ...frame, dot: clamp(DOT * frame.k, DOTS, dpr), line: clamp(LINE * frame.k, LINES, dpr) };
  };
  const strokes = (positions, from, to) => {
    const X = (i) => box.ox + positions[2 * i] * box.k;
    const Y = (i) => box.oy + positions[2 * i + 1] * box.k;
    ctx.globalCompositeOperation = 'destination-over';
    ctx.lineCap = 'round';
    ctx.lineWidth = box.line;
    shades.forEach((color, s) => {
      let open = false;
      ctx.strokeStyle = color;
      ctx.beginPath();
      for (let j = from; j < to; j++) {
        if (shade.branches[j] !== s) continue;
        const b = order.branches[j] * 2;
        ctx.moveTo(X(pairs[b]), Y(pairs[b]));
        ctx.lineTo(X(pairs[b + 1]), Y(pairs[b + 1]));
        open = true;
      }
      if (open) ctx.stroke();
    });
    ctx.globalCompositeOperation = 'source-over';
  };
  const dots = (positions, from, to) => {
    shades.forEach((color, s) => {
      let open = false;
      ctx.fillStyle = color;
      ctx.beginPath();
      for (let i = from; i < to; i++) {
        if (shade.nodes[i] !== s) continue;
        const n = order.nodes[i];
        const x = box.ox + positions[2 * n] * box.k;
        const y = box.oy + positions[2 * n + 1] * box.k;
        ctx.moveTo(x + box.dot, y);
        ctx.arc(x, y, box.dot, 0, TAU);
        open = true;
      }
      if (open) ctx.fill();
    });
  };
  const draw = () => {
    const now = mover.at(view.t, view.still);
    const fresh = dirty || !held || held.version !== now.version || now.k < held.k;
    if (!fresh && now.k === held.k) return;
    if (fresh) {
      box = fit(now.positions, view.w, view.h, view.dpr);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.clearRect(0, 0, view.w, view.h);
      held = { version: now.version, k: 0, m: 0 };
    }
    strokes(now.positions, held.m, now.m);
    dots(now.positions, held.k, now.k);
    held = { version: now.version, k: now.k, m: now.m };
    dirty = false;
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const theme = () => {
    paint();
    touch();
  };
  const svg = () => {
    const now = mover.at(view.t, view.still);
    const w = view.w / view.dpr;
    const h = view.h / view.dpr;
    const frame = fit(now.positions, w, h, 1);
    const X = (i) => frame.ox + now.positions[2 * i] * frame.k;
    const Y = (i) => frame.oy + now.positions[2 * i + 1] * frame.k;
    const marks = Array.from({ length: now.k }, (_, i) => [X(order.nodes[i]), Y(order.nodes[i]), shade.nodes[i]]);
    const lines = Array.from({ length: now.m }, (_, j) => {
      const b = order.branches[j] * 2;
      return [X(pairs[b]), Y(pairs[b]), X(pairs[b + 1]), Y(pairs[b + 1]), shade.branches[j]];
    });
    return picture(w, h, marks, lines, frame.dot, frame.line, shades);
  };
  paint();
  return { draw, every: !span && plan.layout !== 'force' ? REST : undefined, size: touch, theme, svg };
}

function solid(canvas, view, plan, mover, span, three, spin) {
  const start = view.rand() * TAU;
  const { count, pairs, order, shade } = plan;
  const at = new Float64Array(count * 3);
  let stage = null;
  let gone = false;
  let placed = -1;
  let shown = -1;
  let shades = [];
  const paint = () => {
    const { accent, paper } = view.look();
    shades = tones(accent, paper);
  };
  const fit = (positions) => {
    const { mid, scale } = cube(bounds(positions, 3, count));
    for (let i = 0; i < count * 3; i++) at[i] = (positions[i] - mid[i % 3]) * scale;
    return scale;
  };
  const draw = () => {
    if (!stage) return;
    const now = mover.at(view.t, view.still);
    if (placed !== now.version) {
      const scale = fit(now.positions);
      stage.place(at, BALL * scale, ROD * scale);
      placed = now.version;
    }
    if (shown !== now.k) {
      stage.show(now.k, now.m);
      shown = now.k;
    }
    stage.draw();
  };
  const build = (mod) => {
    if (gone) return;
    stage = space(mod, canvas, view, plan, { start, spin });
    stage.tone(shades);
    draw();
  };
  const size = () => {
    stage?.size();
    draw();
  };
  const theme = () => {
    paint();
    stage?.tone(shades);
    draw();
  };
  const turn = (dx, dy) => stage?.turn(dx, dy);
  const svg = () => {
    if (!stage) return '';
    const now = mover.at(view.t, view.still);
    const w = view.w / view.dpr;
    const h = view.h / view.dpr;
    const scale = fit(now.positions);
    const px = stage.unit(h);
    const seen = stage.project(at, w, h);
    const marks = Array.from({ length: now.k }, (_, i) => {
      const n = order.nodes[i] * 3;
      return [seen[n], seen[n + 1], shade.nodes[i], seen[n + 2]];
    }).sort((p, q) => q[3] - p[3]);
    const lines = Array.from({ length: now.m }, (_, j) => {
      const b = order.branches[j] * 2;
      const a = pairs[b] * 3;
      const c = pairs[b + 1] * 3;
      return [seen[a], seen[a + 1], seen[c], seen[c + 1], shade.branches[j], (seen[a + 2] + seen[c + 2]) / 2];
    }).sort((p, q) => q[5] - p[5]);
    return picture(w, h, marks, lines, BALL * scale * px, 2 * ROD * scale * px, shades);
  };
  const stop = () => {
    gone = true;
    stage?.stop();
    stage = null;
  };
  paint();
  if (three) build(three);
  else import('three').then(build, () => {});
  return { draw, every: !spin && !span && plan.layout !== 'force' ? REST : undefined, size, theme, turn, svg, stop };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('graphs: the scene wants mrlyjs/math as opts.math');
  const value = { ...tidy(HOME, opts), ...tidy(SPEC, opts) };
  const seed = opts.seed >>> 0;
  const plan = study(math, value, view.rand, view.still);
  const span = value.grow * 1000;
  const mover = motion(math, plan, span, seed);
  const stage = plan.dim === 3 ? solid(canvas, view, plan, mover, span, opts.three, value.spin) : flat(canvas, view, plan, mover, span);
  const stop = () => {
    stage.stop?.();
    mover.free();
    plan.net.free();
  };
  const json = () => JSON.stringify({ ...plan.facts, network: plan.net.toJSON() });
  return { ...stage, stop, json, facts: plan.facts };
}
