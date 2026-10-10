import { tidy } from '../../lib/knobs.js';
import { rgb, veil } from '../../lib/scene.js';
import { lines } from '../../lib/svg.js';
import { BANDS, KINDS, MODES, NUMBERS, PAD, fit, index, shade, share, span, study } from './engine.js';

const POLYGONS = ['polyin', 'polyout'];

export const SPEC = [
  { key: 'kind', label: 'Track', kind: 'pick', def: 'in', options: KINDS, group: 'Track' },
  { key: 'ring', label: 'Ring', kind: 'slider', def: 7, min: 1, max: 24, step: 1, group: 'Track' },
  { key: 'wheel', label: 'Wheel', kind: 'slider', def: 3, min: 1, max: 24, step: 1, group: 'Track' },
  { key: 'sides', label: 'Sides', kind: 'slider', def: 4, min: 3, max: 12, step: 1, group: 'Track', when: (value) => POLYGONS.includes(value.kind) },
  { key: 'laps', label: 'Laps', kind: 'slider', def: 2, min: 1, max: 24, step: 1, group: 'Track', when: (value) => value.kind !== 'in' && value.kind !== 'out' },
  { key: 'code', label: 'Code', kind: 'text', def: '', group: 'Wheel' },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Wheel' },
  { key: 'level', label: 'Level', kind: 'slider', def: 1, min: 1, max: 3, step: 1, group: 'Wheel' },
  { key: 'mode', label: 'Pencils', kind: 'pick', def: 'fill', options: MODES, group: 'Wheel' },
  { key: 'reach', label: 'Reach', kind: 'slider', def: 0.9, min: 0.2, max: 1.3, step: 0.05, unit: 'r', group: 'Wheel' },
  { key: 'jitter', label: 'Jitter', kind: 'slider', def: 0, min: 0, max: 1, step: 0.05, group: 'Wheel' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [{ key: 'Enter', label: 'Again', act: 'again', button: true }],
  actions: { again: (scene) => scene.again() },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const FADE = 0.55;
const LINE = 1.25;
const DOT = 2.5;
const DOTS = 256;
const INSET = 0.08;
const DASH = [4, 6];
const TRACK = 0.3;
const RING = 0.45;
const CELLS = 0.2;
const FILL = 0.16;
const SHAPE = 3;

const mix = (a, b, t) => `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(', ')})`;

const mask = (shape, color) => {
  const { side } = shape;
  const page = new OffscreenCanvas(side, side);
  const ink = page.getContext('2d');
  ink.fillStyle = color;
  for (let row = 0; row < side; row++) {
    for (let col = 0; col < side; col++) {
      if (shape.mask[row * side + col] !== SHAPE) continue;
      let run = 1;
      while (col + run < side && shape.mask[row * side + col + run] === SHAPE) run++;
      ink.fillRect(col, row, run, 1);
      col += run;
    }
  }
  return page;
};

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('spirograph: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const value = tidy(SPEC, opts);
  const plan = study(math, { ...value, seed: opts.seed >>> 0 }, view.rand);
  const { trace, samples, pencils, track, cell, side, bands, frame, shape } = plan;
  const length = span(plan.beats);
  const groups = Array.from({ length: BANDS }, () => []);
  bands.forEach((band, k) => groups[band].push(k));
  let origin = 0;
  let drawn = -1;
  let layer = null;
  let pen = null;
  let stamp = null;
  let cover = null;
  let box = null;
  let shades = [];
  let accent = '#000';
  const paint = () => {
    const look = view.look();
    accent = look.accent;
    const a = rgb(look.accent);
    const p = rgb(look.paper);
    shades = Array.from({ length: BANDS }, (_, b) => mix(a, p, FADE * (1 - b / (BANDS - 1))));
    cover = shape ? mask(shape, veil(accent, FILL)) : null;
  };
  const at = (k, i) => [box.ox + trace[(k * samples + i) * 2] * box.k, box.oy - trace[(k * samples + i) * 2 + 1] * box.k];
  const rail = (target, dpr) => {
    if (!track.outline.length) return;
    target.beginPath();
    track.outline.forEach(([x, y], i) => (i ? target.lineTo(box.ox + x * box.k, box.oy - y * box.k) : target.moveTo(box.ox + x * box.k, box.oy - y * box.k)));
    if (track.closed) target.closePath();
    target.strokeStyle = veil(accent, TRACK);
    target.lineWidth = dpr;
    target.setLineDash(DASH.map((d) => d * dpr));
    target.stroke();
    target.setLineDash([]);
  };
  const press = () => {
    const [height, width] = cell.shape;
    const unit = side * track.wheel * box.k;
    const edge = Math.ceil(Math.max(width, height) * unit) + 2;
    stamp = new OffscreenCanvas(edge, edge);
    const ink = stamp.getContext('2d');
    ink.fillStyle = veil(accent, CELLS);
    const gap = unit * INSET;
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        if (cell.types[row * width + col]) ink.fillRect(edge / 2 + (col - width / 2) * unit + gap, edge / 2 + (row - height / 2) * unit + gap, unit - 2 * gap, unit - 2 * gap);
      }
    }
  };
  const reset = () => {
    box = fit(frame, view.w, view.h, PAD * view.dpr);
    press();
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.clearRect(0, 0, view.w, view.h);
    pen.lineJoin = 'round';
    pen.lineCap = 'round';
    rail(pen, view.dpr);
    pen.lineWidth = LINE * view.dpr;
    drawn = -1;
  };
  const extend = (to) => {
    if (to <= drawn) return;
    const from = Math.max(0, drawn);
    groups.forEach((group, band) => {
      if (!group.length) return;
      pen.beginPath();
      for (const k of group) {
        const [x, y] = at(k, from);
        pen.moveTo(x, y);
        for (let i = from + 1; i <= to; i++) {
          const [u, v] = at(k, i);
          pen.lineTo(u, v);
        }
      }
      pen.strokeStyle = shades[band];
      pen.stroke();
    });
    drawn = to;
  };
  const fill = (tint) => {
    const { x, y, radius } = shape.disc;
    ctx.globalAlpha = tint;
    ctx.drawImage(cover, box.ox + (x - radius) * box.k, box.oy - (y + radius) * box.k, 2 * radius * box.k, 2 * radius * box.k);
    ctx.globalAlpha = 1;
  };
  const wheel = (i) => {
    const s = (track.total * i) / (samples - 1);
    const [cx, cy] = math.spirograph.pose(track, s);
    const phi = math.spirograph.turn(track, s);
    const dpr = view.dpr;
    const x = box.ox + cx * box.k;
    const y = box.oy - cy * box.k;
    const r = track.wheel * box.k;
    ctx.lineWidth = dpr;
    ctx.strokeStyle = veil(accent, RING);
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.stroke();
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(-phi);
    ctx.drawImage(stamp, -stamp.width / 2, -stamp.height / 2);
    ctx.restore();
    if (pencils.length <= DOTS) {
      pencils.forEach((_, k) => {
        const [px, py] = at(k, i);
        ctx.fillStyle = shades[bands[k]];
        ctx.beginPath();
        ctx.arc(px, py, DOT * dpr, 0, TAU);
        ctx.fill();
      });
    }
    ctx.fillStyle = accent;
    ctx.beginPath();
    ctx.arc(x, y, 0.8 * DOT * dpr, 0, TAU);
    ctx.fill();
  };
  const draw = () => {
    const now = view.t - origin;
    const i = index(share(now, length, view.still), samples);
    if (!layer || i < drawn) reset();
    extend(i);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    const tint = cover ? shade(now, length, view.still) : 0;
    if (tint > 0) fill(tint);
    ctx.drawImage(layer, 0, 0);
    wheel(i);
  };
  const again = () => {
    origin = view.t;
    if (view.still) view.wake?.();
    else draw();
  };
  const theme = () => {
    paint();
    layer = null;
  };
  const size = () => {
    layer = null;
  };
  const picture = () => {
    const css = fit(frame, view.w / view.dpr, view.h / view.dpr, PAD);
    const to = (x, y) => [css.ox + x * css.k, css.oy - y * css.k];
    const strokes = [{ points: track.outline.flatMap(([x, y]) => to(x, y)), color: veil(accent, TRACK), width: 1, dash: DASH, closed: track.closed }];
    pencils.forEach((_, k) => {
      const points = new Float64Array(samples * 2);
      for (let i = 0; i < samples; i++) [points[i * 2], points[i * 2 + 1]] = to(trace[(k * samples + i) * 2], trace[(k * samples + i) * 2 + 1]);
      strokes.push({ points, color: shades[bands[k]], width: LINE });
    });
    return lines({ width: view.w / view.dpr, height: view.h / view.dpr, strokes });
  };
  paint();
  return { draw, size, theme, again, svg: picture, facts: plan.facts };
}
