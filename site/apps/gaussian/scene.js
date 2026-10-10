import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, veil } from '../../lib/scene.js';
import { LOOKS, PAD, RINGS, cap, choose, hit, lay, palette, picture, reached, share, study, styleOf, table } from './engine.js';

export const SPEC = [
  { key: 'roll', label: 'Random', kind: 'toggle', def: 1, group: 'Window' },
  { key: 'ring', label: 'Ring', kind: 'pick', def: 'square', options: RINGS, group: 'Window', when: (value) => !value.roll },
  { key: 'limit', label: 'Reach', kind: 'slider', def: 40, min: 1, max: (value) => cap(value.ring), step: 1, group: 'Window', when: (value) => !value.roll },
  { key: 'look', label: 'Colour', kind: 'segment', def: 'fate', options: LOOKS, group: 'Look', when: (value) => !value.roll },
  { key: 'faint', label: 'Composites', kind: 'toggle', def: 1, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 32, step: 1, unit: 'px', zero: 'Fit', group: 'Size' },
  { key: 'grow', label: 'Grow', kind: 'slider', def: 12, min: 0, max: 60, step: 1, unit: 's', group: 'Motion' },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  record: true,
  keys: [
    { key: 'Enter', label: 'Again', act: 'again', button: true, when: (value) => value.grow > 0 },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Reach', act: 'reach' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Ring', act: 'ring' },
  ],
  actions: {
    again: (scene) => scene.again(),
    reach: (scene, e) => scene.reach?.(e.key === 'ArrowDown' ? -1 : 1),
    ring: (scene) => scene.turn?.(),
  },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const REST = 1000;
const CHUNK = 4096;
const GAP = 0.12;
const SMALL = 6;
const LINE = 0.15;
const RING = 3;
const DASH = [4, 3];
const DIM = 0.6;

const mix = (a, b, t) => {
  const p = rgb(a);
  const q = rgb(b);
  return `rgb(${p.map((v, i) => Math.round(v + (q[i] - v) * t)).join(', ')})`;
};

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('gaussian: the scene wants mrlyjs/num as opts.num');
  const ctx = canvas.getContext('2d');
  const value = tidy(SPEC, opts);
  const chosen = choose(value, view.rand);
  const plan = study(num, chosen.word, chosen.limit);
  const { word, count, a, b, norm, fate, ex, ey, frame, top } = plan;
  const span = value.grow * 1000;
  const round = word === 'hex';
  const style = new Uint8Array(count);
  for (let p = 0; p < count; p++) style[p] = styleOf(chosen.look, value.faint, fate[p], norm[p], top);
  let tones = [];
  let accent = '#000';
  let box = null;
  let layer = null;
  let pen = null;
  let drawn = -1;
  let origin = 0;
  let pick = null;
  let dirty = true;
  const paint = () => {
    const look = view.look();
    accent = look.accent;
    tones = palette(mix, veil, look.accent, look.paper);
  };
  const spot = (x, y) => [box.ox + (x * ex[0] + y * ey[0]) * box.k, box.oy - (x * ex[1] + y * ey[1]) * box.k];
  const half = () => {
    const s = box.k;
    const gap = s >= SMALL ? Math.max(view.dpr, GAP * s) : 0;
    return Math.max(0.5, (s - gap) / 2);
  };
  const mark = (target, x, y, h, hollow) => {
    if (round) {
      target.moveTo(x + h, y);
      target.arc(x, y, h, 0, TAU);
      return;
    }
    if (hollow) {
      target.rect(x - h, y - h, 2 * h, 2 * h);
      return;
    }
    const x0 = Math.round(x - h);
    const y0 = Math.round(y - h);
    target.rect(x0, y0, Math.max(1, Math.round(x + h) - x0), Math.max(1, Math.round(y + h) - y0));
  };
  const lays = (target, from, to, h) => {
    tones.forEach((tone, s) => {
      if (!tone) return;
      const inner = tone.hollow ? Math.max(0.5, h - (LINE * box.k) / 2) : h;
      target.fillStyle = tone.color;
      target.strokeStyle = tone.color;
      target.lineWidth = Math.max(view.dpr, LINE * box.k);
      let open = 0;
      const flush = () => {
        if (!open) return;
        if (tone.hollow) target.stroke();
        else target.fill();
        open = 0;
      };
      for (let p = from; p <= to; p++) {
        if (style[p] !== s) continue;
        if (!open) target.beginPath();
        const [x, y] = spot(a[p], b[p]);
        mark(target, x, y, inner, tone.hollow);
        if (++open >= CHUNK) flush();
      }
      flush();
    });
  };
  const extend = (to) => {
    if (to <= drawn) return;
    lays(pen, drawn + 1, to, half());
    drawn = to;
  };
  const reset = () => {
    box = lay(frame, view.w, view.h, PAD * view.dpr, value.cell * view.dpr);
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.clearRect(0, 0, view.w, view.h);
    drawn = -1;
  };
  const ring = (x, y, radius, width, color, dash) => {
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.setLineDash(dash);
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
  };
  const overlay = () => {
    const dpr = view.dpr;
    const radius = half() + RING * dpr;
    const [px, py] = spot(pick.a, pick.b);
    for (const [x, y] of pick.associates.slice(1)) {
      const [u, v] = spot(x, y);
      ring(u, v, radius, 1.5 * dpr, veil(accent, DIM), []);
    }
    const [cx, cy] = spot(pick.conjugate[0], pick.conjugate[1]);
    ring(cx, cy, radius, 1.5 * dpr, accent, DASH.map((d) => d * dpr));
    ring(px, py, radius, RING * dpr, accent, []);
  };
  const draw = () => {
    if (!span && !dirty && layer) return;
    dirty = false;
    const i = reached(share(view.t - origin, span, view.still), count);
    if (!layer || i < drawn) reset();
    extend(i);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(layer, 0, 0);
    if (pick) overlay();
  };
  const touch = () => {
    layer = null;
    dirty = true;
    if (!span) draw();
  };
  const theme = () => {
    paint();
    touch();
  };
  const at = (u, v) => {
    if (!box) reset();
    const x = (u * view.w - box.ox) / box.k;
    const y = (box.oy - v * view.h) / box.k;
    const [p, q] = num.gauss.Ring.nearest(plan.ring, x, y);
    pick = hit(num, plan, Number(p), Number(q));
    dirty = true;
    draw();
    return pick;
  };
  const again = () => {
    if (!span) return;
    origin = view.t;
    if (view.still) view.wake?.();
    else draw();
  };
  const svg = () => picture(plan, style, tones);
  const csv = () => table(plan);
  paint();
  return { draw, every: span ? undefined : REST, size: touch, theme, again, at, svg, csv, facts: { ...plan.facts, look: chosen.look } };
}
