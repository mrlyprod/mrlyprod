import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, veil } from '../../lib/scene.js';
import { runs } from '../../lib/svg.js';
import { DESIGN } from '../designs/scene.js';
import { FADE, FILL, LOOKS, PAD, arc, cap, drift, fit, phase, picture, study } from './engine.js';

const SHAPE = DESIGN.map((row) => (row.key === 'base' ? { ...row, def: 3 } : row));

export const SPEC = [
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value.base), step: 1, group: 'Arcs' },
  { key: 'look', label: 'Show', kind: 'segment', def: 'all', options: LOOKS, group: 'Arcs' },
  { key: 'width', label: 'Width', kind: 'slider', def: 0.16, min: 0.04, max: 0.5, step: 0.02, group: 'Arcs' },
  { key: 'cells', label: 'Cells', kind: 'toggle', def: 1, group: 'Arcs' },
  { key: 'wander', label: 'Wander', kind: 'toggle', def: 1, group: 'Motion', when: (value) => value.look !== 'strands' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

export const units = () => import('./unit.js');

const REST = 1000;
const CHUNK = 8192;

const mix = (a, b, t) => `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(', ')})`;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('arcs: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const shape = tidy(SHAPE, opts);
  const value = { ...shape, ...tidy(SPEC, { ...opts, ...shape }) };
  const plan = study(math, value, view.rand);
  const { side, filled, order, dir, bounds, closed, lit, clock } = plan;
  const live = value.wander === 1 && value.look !== 'strands' && lit.length > 0;
  const spans = runs({ shape: [side, side], types: filled });
  let shades = null;
  let box = null;
  let base = null;
  let layer = null;
  let pen = null;
  let stamped = 0;
  let last = -1;
  let dirty = true;
  const paint = () => {
    const look = view.look();
    shades = { accent: look.accent, dim: mix(rgb(look.accent), rgb(look.paper), FADE), fill: veil(look.accent, FILL) };
  };
  const thick = () => Math.max(view.dpr, value.width * box.px);
  const sweep = (target, a, d, f) => {
    const g = arc(plan, a);
    const r = box.px / 2;
    const cx = box.x + g.cx * box.px;
    const cy = box.y + g.cy * box.px;
    if (d > 0) {
      target.moveTo(cx, box.y + (g.y + 0.5) * box.px);
      target.arc(cx, cy, r, g.av, g.av + g.sweep * f, g.sweep < 0);
    } else {
      target.moveTo(box.x + (g.x + 0.5) * box.px, cy);
      target.arc(cx, cy, r, g.ah, g.ah - g.sweep * f, g.sweep > 0);
    }
  };
  const trace = (target, k, f) => {
    const from = bounds[k];
    const n = bounds[k + 1] - from;
    const whole = f >= 1 ? n : Math.floor(f * n);
    const part = f >= 1 ? 0 : f * n - whole;
    target.beginPath();
    for (let j = 0; j < whole; j++) sweep(target, order[from + j], dir[from + j], 1);
    if (part > 0) sweep(target, order[from + whole], dir[from + whole], part);
    target.stroke();
  };
  const lay = (target, loops, color) => {
    target.strokeStyle = color;
    target.lineWidth = thick();
    let open = 0;
    target.beginPath();
    for (let k = 0; k < closed.length; k++) {
      if (closed[k] !== loops) continue;
      for (let j = bounds[k]; j < bounds[k + 1]; j++) {
        sweep(target, order[j], dir[j], 1);
        if (++open >= CHUNK) {
          target.stroke();
          target.beginPath();
          open = 0;
        }
      }
    }
    if (open) target.stroke();
  };
  const blank = () => {
    const canvas = new OffscreenCanvas(view.w, view.h);
    const ink = canvas.getContext('2d');
    ink.setTransform(1, 0, 0, 1, 0, 0);
    ink.clearRect(0, 0, view.w, view.h);
    ink.lineCap = 'round';
    ink.lineJoin = 'round';
    return [canvas, ink];
  };
  const rewind = () => {
    pen.clearRect(0, 0, view.w, view.h);
    pen.drawImage(base, 0, 0);
    stamped = 0;
  };
  const reset = () => {
    box = fit(side, view.w, view.h, PAD * view.dpr);
    const [back, ink] = blank();
    if (value.cells && spans.length) {
      ink.fillStyle = shades.fill;
      ink.beginPath();
      for (const [row, col, len] of spans) ink.rect(box.x + col * box.px, box.y + row * box.px, len * box.px, box.px);
      ink.fill();
    }
    if (value.look !== 'loops') lay(ink, 0, shades.dim);
    if (value.look !== 'strands') lay(ink, 1, live ? shades.dim : shades.accent);
    if (!live) {
      layer = back;
      pen = ink;
      stamped = clock.byEnd.length;
      return;
    }
    base = back;
    [layer, pen] = blank();
    rewind();
  };
  const extend = (now) => {
    if (stamped >= clock.byEnd.length) return;
    pen.strokeStyle = shades.accent;
    pen.lineWidth = thick();
    while (stamped < clock.byEnd.length && clock.end[clock.byEnd[stamped]] <= now) trace(pen, lit[clock.byEnd[stamped++]], 1);
  };
  const draw = () => {
    if (!live && !dirty && layer) return;
    dirty = false;
    const now = live ? phase(view.t, clock.span, view.still) : Infinity;
    if (!layer) reset();
    else if (now < last) rewind();
    last = now;
    extend(now);
    const [dx, dy] = live && !view.still ? drift(view.t, box) : [0, 0];
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.setTransform(1, 0, 0, 1, dx, dy);
    ctx.drawImage(layer, 0, 0);
    if (!live || now === Infinity) return;
    ctx.strokeStyle = shades.accent;
    ctx.lineWidth = thick();
    ctx.lineCap = 'round';
    for (const k of clock.byStart) {
      if (clock.start[k] > now) break;
      if (clock.end[k] > now) trace(ctx, lit[k], (now - clock.start[k]) / clock.dur[k]);
    }
  };
  const touch = () => {
    layer = null;
    dirty = true;
    if (!live) draw();
  };
  const theme = () => {
    paint();
    touch();
  };
  const svg = () => picture(plan, value, shades);
  paint();
  return { draw, every: live ? undefined : REST, size: touch, theme, svg, facts: plan.facts };
}
