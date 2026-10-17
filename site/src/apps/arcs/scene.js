import { palette } from '../../lib/theme/palette.gen.js';
import { dark, light } from '../../lib/theme/theme.js';
import { tidy } from '../../lib/app/spec.js';
import { HUES, TINTS, mix, rgb } from '../../lib/color.js';
import { runs } from '../../lib/app/svg.js';
import { BASES, FADE, LOOKS, PAD, arc, cap, fit, picture, resolve, study, thick } from './engine.js';
import { wasm } from '../../lib/units/index.js';

const HIDE = () => false;

export const SPEC = [
  { key: 'base', label: 'Base', kind: 'segment', def: 2, options: BASES, when: HIDE },
  { key: 'code', label: 'Code', kind: 'text', def: '', when: HIDE },
  { key: 'level', label: 'Level', kind: 'slider', def: 5, min: 1, max: (value, units) => units && cap(units.math, value.base), step: 1, group: 'Arcs' },
  { key: 'cells', label: 'Shade cells', kind: 'toggle', def: 1, group: 'Arcs' },
  { key: 'look', label: 'Show', kind: 'segment', def: 'all', options: LOOKS, group: 'Arcs' },
  { key: 'width', label: 'Width', kind: 'slider', def: 0, min: 0, max: 0.5, step: 0.02, zero: 'Auto', group: 'Arcs' },
  { key: 'tint', label: 'Loops', kind: 'pick', def: 'orange', options: TINTS, group: 'Colour' },
  { key: 'strand', label: 'Strands', kind: 'pick', def: 'blue', options: TINTS, group: 'Colour' },
];

export const ABOUT = [
  'Every cell carries two quarter arcs, each joining the midpoints of two neighbouring edges. A filled cell takes the arcs round its lower-left and upper-right corners, an empty cell the other two.',
  'The arcs join into curves. Strands run from border to border, and there are always twice the side of them. Loops close on themselves, and the design decides how many.',
  'A law shows where one is proved: the carpet at base 3 and all sixteen codes at base 2. Every count comes from the crate, by union-find over the edge midpoints, and row 0 is at the bottom.',
  'Trace, or a tap, lights the loops one by one. Reroll rolls a design that loops at this level. [Designs](/designs/) lists the designs, and [Two](/two/) repeats one into a sheet.',
];

const units = wasm('math');

export const file = (value, unit) => `arcs-${unit?.math ? resolve(unit.math, value.base, value.code) : String(value.code ?? '').trim() || 'carpet'}-base${value.base}-level${value.level}`.replace(/[^\w.+-]+/g, '-');

export const PAGE = {
  spec: SPEC,
  units,
  name: file,
  about: ABOUT,
  keys: [
    { key: 'Enter', label: 'Trace', act: 'trace', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Design', act: 'design' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    trace: (scene) => scene.trace?.(),
    design: (scene, e) => scene.step?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

const CHUNK = 8192;

const FLIP = (h) => [1, 0, 0, -1, 0, h];

const bright = (color) => rgb(color).reduce((a, b) => a + b, 0) > 382;

export function shades(canvas, look, strand) {
  const css = typeof getComputedStyle === 'function' && canvas?.nodeType ? getComputedStyle(canvas) : null;
  const read = (name, fallback) => (css && css.getPropertyValue(name).trim()) || fallback;
  const other = HUES.includes(strand) ? read(`--${strand}`, palette[strand]) : read('--accent', light.accent);
  return { loop: look.accent, strand: other, dim: mix(rgb(look.accent), rgb(look.paper), FADE), fill: read('--line', bright(look.paper) ? light.line : dark.line) };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, onTell } = opts;
  if (!math) throw new Error('arcs: the scene wants @mrlyprod/mrly/math as opts.math');
  const ctx = canvas.getContext('2d');
  const clean = (next) => tidy(SPEC, next, { math });
  let value = clean(opts);
  let plan = null;
  let spans = [];
  let inks = null;
  let box = null;
  let layer = null;
  let pen = null;
  let stamped = 0;
  let tracing = null;
  let dirty = true;
  const plot = () => {
    plan = study(math, value);
    spans = plan.side ? runs({ shape: [plan.side, plan.side], types: plan.filled }) : [];
    tracing = null;
    onTell?.(plan.facts);
  };
  const paint = () => {
    inks = shades(canvas, view.look(), value.strand);
  };
  const width = () => thick(value.width, box.px, view.dpr);
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
    const { order, dir, bounds } = plan;
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
    const { order, dir, bounds, closed } = plan;
    target.strokeStyle = color;
    target.lineWidth = width();
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
    const sheet = new OffscreenCanvas(view.w, view.h);
    const ink = sheet.getContext('2d');
    ink.setTransform(...FLIP(view.h));
    ink.lineCap = 'round';
    ink.lineJoin = 'round';
    return [sheet, ink];
  };
  const reset = () => {
    box = fit(plan.side, view.w, view.h, PAD * view.dpr);
    const [back, ink] = blank();
    if (value.cells && spans.length) {
      ink.fillStyle = inks.fill;
      ink.beginPath();
      for (const [row, col, len] of spans) ink.rect(box.x + col * box.px, box.y + row * box.px, len * box.px, box.px);
      ink.fill();
    }
    if (value.look !== 'loops') lay(ink, 0, inks.strand);
    if (value.look !== 'strands') lay(ink, 1, tracing ? inks.dim : inks.loop);
    if (!tracing) {
      layer = back;
      return;
    }
    [layer, pen] = blank();
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.drawImage(back, 0, 0);
    pen.setTransform(...FLIP(view.h));
    stamped = 0;
  };
  const extend = (now) => {
    const { clock, lit } = plan;
    if (stamped >= clock.byEnd.length) return;
    pen.strokeStyle = inks.loop;
    pen.lineWidth = width();
    while (stamped < clock.byEnd.length && clock.end[clock.byEnd[stamped]] <= now) trace(pen, lit[clock.byEnd[stamped++]], 1);
  };
  const draw = () => {
    if (!dirty && !tracing) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (!plan.side) return;
    if (!layer) reset();
    const now = tracing ? view.t - tracing.from : Infinity;
    if (tracing && now >= plan.clock.span) {
      tracing = null;
      reset();
    } else if (tracing) extend(now);
    ctx.drawImage(layer, 0, 0);
    if (!tracing) return;
    const { clock, lit } = plan;
    ctx.setTransform(...FLIP(view.h));
    ctx.strokeStyle = inks.loop;
    ctx.lineWidth = width();
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
    for (const k of clock.byStart) {
      if (clock.start[k] > now) break;
      if (clock.end[k] > now) trace(ctx, lit[k], (now - clock.start[k]) / clock.dur[k]);
    }
  };
  const touch = () => {
    layer = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    touch();
  };
  const start = () => {
    if (!plan.side || value.look === 'strands' || !plan.lit.length) return false;
    if (view.still) view.wake?.();
    tracing = { from: view.t };
    touch();
    draw();
    return true;
  };
  const set = (next) => {
    const was = value;
    value = clean(next);
    const same = (keys) => keys.every((key) => value[key] === was[key]);
    if (!same(['base', 'code', 'level'])) plot();
    else if (same(['look', 'cells', 'width', 'tint', 'strand'])) return;
    if (value.look === 'strands') tracing = null;
    if (!same(['tint', 'strand'])) paint();
    touch();
  };
  const svg = () => (plan.side ? picture(plan, value, inks) : null);
  paint();
  plot();
  return {
    still: true,
    draw,
    size: touch,
    theme,
    set,
    trace: start,
    svg,
    get facts() {
      return plan.facts;
    },
  };
}
