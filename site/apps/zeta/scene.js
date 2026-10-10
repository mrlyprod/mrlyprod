import { tidy } from '../../lib/knobs.js';
import { TINTS, veil } from '../../lib/scene.js';
import { AXIS, BEAD, DASH, DOT, FACES, GHOST, HEAD, LINE, LOOKS, PAD, PIP, REACH, SPANS, TOP, TRAIL, ZEROS, bands, choose, fit, flares, index, longest, passed, phase, picture, study, table } from './engine.js';
import { stairs } from './stairs.js';

const walking = (value) => value.face === 'walk';

const climbing = (value) => value.face === 'stairs';

export const SPEC = [
  { key: 'face', label: 'Face', kind: 'segment', def: 'walk', options: FACES, group: 'Face' },
  { key: 'from', label: 'From', kind: 'number', def: 0, min: 0, max: TOP, group: 'Walk', when: walking },
  { key: 'span', label: 'Span', kind: 'slider', def: 60, min: SPANS[0], max: SPANS[1], step: 10, group: 'Walk', when: walking },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 2, min: 0.5, max: 10, step: 0.5, unit: 't/s', group: 'Walk', when: walking },
  { key: 'zeros', label: 'Zeros', kind: 'slider', def: 10, min: 0, max: ZEROS, step: 1, group: 'Stairs', when: climbing },
  { key: 'x', label: 'Up to', kind: 'slider', def: 100, min: REACH[0], max: REACH[1], step: 10, group: 'Stairs', when: climbing },
  { key: 'look', label: 'Look', kind: 'segment', def: 'line', options: LOOKS, group: 'Look', when: walking },
  { key: 'trail', label: 'Trail', kind: 'slider', def: 10, min: 1, max: (value) => Math.min(value.span, longest(value.from, value.span)), step: 1, group: 'Look', when: walking },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [{ key: 'Enter', label: 'Again', act: 'again', button: true }],
  actions: { again: (scene) => scene.again() },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const RING = 3;
const BURST = 0.08;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('zeta: the scene wants mrlyjs/num as opts.num');
  const value = tidy(SPEC, opts);
  return climbing(value) ? stairs(canvas, view, value, opts) : walker(canvas, view, value, opts);
}

function walker(canvas, view, value, opts) {
  const ctx = canvas.getContext('2d');
  const from = choose(value, view.rand);
  const plan = study(opts.num, from, value.span);
  const { xs, ys, n, per, zeros, frame } = plan;
  const length = (value.span / value.speed) * 1000;
  const kept = Math.round(value.trail * per);
  const every = Math.max(1, Math.round(per * DOT));
  const lined = value.look !== 'dots';
  const dotted = value.look !== 'line';
  let origin = 0;
  let box = null;
  let layer = null;
  let pen = null;
  let drawn = -1;
  let shown = 0;
  let accent = '#000';
  let told = -1;
  let gone = false;
  const paint = () => {
    accent = view.look().accent;
  };
  const X = (i) => box.ox + xs[i] * box.k;
  const Y = (i) => box.oy - ys[i] * box.k;
  const run = (target, a, b, color, width) => {
    if (b <= a) return;
    target.strokeStyle = color;
    target.lineWidth = width;
    target.beginPath();
    target.moveTo(X(a), Y(a));
    for (let i = a + 1; i <= b; i++) target.lineTo(X(i), Y(i));
    target.stroke();
  };
  const beads = (target, a, b, color, radius) => {
    target.fillStyle = color;
    target.beginPath();
    for (let i = Math.ceil(a / every) * every; i <= b; i += every) {
      const x = X(i);
      const y = Y(i);
      target.moveTo(x + radius, y);
      target.arc(x, y, radius, 0, TAU);
    }
    target.fill();
  };
  const reset = () => {
    box = fit(frame, view.w, view.h, PAD * view.dpr);
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    pen.lineJoin = 'round';
    pen.lineCap = 'round';
    drawn = -1;
  };
  const extend = (to) => {
    if (to <= drawn) return;
    if (lined) run(pen, Math.max(0, drawn), to, accent, LINE * view.dpr);
    else beads(pen, drawn + 1, to, accent, BEAD * view.dpr);
    drawn = to;
  };
  const axes = () => {
    const dpr = view.dpr;
    ctx.strokeStyle = veil(accent, AXIS);
    ctx.lineWidth = dpr;
    ctx.beginPath();
    ctx.moveTo(0, box.oy);
    ctx.lineTo(view.w, box.oy);
    ctx.moveTo(box.ox, 0);
    ctx.lineTo(box.ox, view.h);
    ctx.stroke();
    ctx.setLineDash(DASH.map((d) => d * dpr));
    ctx.beginPath();
    ctx.arc(box.ox, box.oy, box.k, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
  };
  const trail = (head) => {
    const dpr = view.dpr;
    for (const { from: a, to: b, first, alpha } of bands(head, kept)) {
      if (lined) run(ctx, a, b, veil(accent, alpha), TRAIL * dpr);
      if (dotted) beads(ctx, first, b, veil(accent, alpha), PIP * dpr);
    }
  };
  const flash = (play) => {
    const dpr = view.dpr;
    const reach = BURST * Math.min(view.w, view.h);
    for (const k of flares(zeros, from, value.speed, play)) {
      ctx.strokeStyle = veil(accent, 1 - k);
      ctx.lineWidth = dpr + RING * dpr * (1 - k);
      ctx.beginPath();
      ctx.arc(box.ox, box.oy, reach * (1 - (1 - k) ** 2), 0, TAU);
      ctx.stroke();
    }
  };
  const tell = (count) => {
    if (count === told || gone) return;
    told = count;
    opts.onPass?.(count, count ? zeros[count - 1] : null);
  };
  const draw = () => {
    const { at: share, veil: fade, play } = phase(view.t - origin, length, view.still);
    const i = index(share, n);
    if (!layer || i < drawn) reset();
    extend(i);
    shown = i;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.lineJoin = 'round';
    axes();
    ctx.globalAlpha = fade * GHOST;
    ctx.drawImage(layer, 0, 0);
    ctx.globalAlpha = fade;
    trail(i);
    if (!view.still) flash(play);
    ctx.fillStyle = accent;
    ctx.beginPath();
    ctx.arc(X(i), Y(i), HEAD * view.dpr, 0, TAU);
    ctx.fill();
    ctx.globalAlpha = 1;
    tell(passed(zeros, from + i / per));
  };
  const touch = () => {
    layer = null;
  };
  const theme = () => {
    paint();
    touch();
  };
  const again = () => {
    origin = view.t;
    if (view.still) view.wake?.();
    else draw();
  };
  const stop = () => {
    gone = true;
  };
  const svg = () => picture(plan, { w: view.w / view.dpr, h: view.h / view.dpr, pad: PAD }, shown, kept, value.look, accent);
  const csv = () => table(plan);
  paint();
  return { draw, size: touch, theme, again, stop, svg, csv, facts: plan.facts };
}
