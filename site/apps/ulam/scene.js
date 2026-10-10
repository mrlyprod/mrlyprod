import { tidy } from '../../lib/knobs.js';
import { TINTS, veil } from '../../lib/scene.js';
import { DIM, HALF, HEX, HIT, LABEL, LATTICES, LEAD, LIT, MARKS, OUTLINE, RAIL, SOIL, WIDTH, choose, factors, fitted, landed, lay, palette, picture, share, span, study, table } from './engine.js';

export const SPEC = [
  { key: 'roll', label: 'Random', kind: 'toggle', def: 1, group: 'Sheet' },
  { key: 'lattice', label: 'Lattice', kind: 'segment', def: 'square', options: LATTICES, group: 'Sheet', when: (value) => !value.roll },
  { key: 'fit', label: 'Whole sheet', kind: 'toggle', def: 0, group: 'Sheet' },
  { key: 'rings', label: 'Rings', kind: 'slider', def: 100, min: 3, max: 200, step: 1, group: 'Sheet', when: (value) => value.fit === 1 },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 3, min: 2, max: 40, step: 1, unit: 'px', group: 'Sheet', when: (value) => value.fit !== 1 },
  { key: 'mark', label: 'Mark', kind: 'pick', def: 'prime', options: MARKS, group: 'Mark' },
  { key: 'a', label: 'a', kind: 'slider', def: 4, min: 0, max: LEAD, step: 1, zero: 'Off', group: 'Line', when: (value) => !value.roll },
  { key: 'b', label: 'b', kind: 'slider', def: -2, min: -30, max: 30, step: 1, group: 'Line', when: (value) => !value.roll && value.a > 0 },
  { key: 'c', label: 'c', kind: 'slider', def: 41, min: 1, max: 200, step: 1, group: 'Line', when: (value) => !value.roll && value.a > 0 },
  { key: 'faint', label: 'Composites', kind: 'toggle', def: 1, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Again', act: 'again', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Line', act: 'line', when: (value) => value.roll === 1 || value.a > 0 },
    { key: 'l', label: 'Lattice', act: 'lattice' },
  ],
  actions: {
    again: (scene) => scene.again?.(),
    line: (scene, e) => scene.line?.(e.key === 'ArrowLeft' ? -1 : 1),
    lattice: (scene) => scene.lattice?.(),
  },
};

export const units = () => import('./unit.js');

export const FACE = 'Noto Sans Mono';
const GAP = 8;
const INSET = 0.07;
const TEXT = 0.42;
const RING = 0.8;
const CHUNK = 4096;
const TURN = Math.PI * 2;

export function said(pick) {
  if (!pick) return '';
  const where = `ring ${pick.ring}`;
  if (pick.n === 1) return `1, neither prime nor composite, ${where}`;
  if (pick.prime) return `${pick.n} prime, ${where}`;
  return `${pick.n} = ${factors(pick.factors)}, ${where}`;
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('ulam: the scene wants mrlyjs/num as opts.num');
  const ctx = canvas.getContext('2d');
  const named = tidy(SPEC, opts);
  const value = { ...named, ...choose(num, named, view.rand) };
  let plan = study(num, value, view.w, view.h, view.dpr);
  let length = span(plan.rings);
  let origin = 0;
  let layer = null;
  let pen = null;
  let bed;
  let drawn = 0;
  let shown = -1;
  let dirty = true;
  let picked = null;
  let digits = 0;
  let accent = '#000';
  let paper = '#fff';
  let dim = accent;
  let tones = [];
  const paint = () => {
    const look = view.look();
    accent = look.accent;
    paper = look.paper;
    dim = veil(accent, HALF);
    tones = palette(accent, paper);
  };
  const inside = (n) => {
    const { geo } = plan;
    const [x, y] = geo.at(plan.xs[n], plan.ys[n]);
    return x > -geo.px && x < view.w + geo.px && y > -geo.px && y < view.h + geo.px;
  };
  const label = (n, x, y, t) => {
    const px = plan.geo.px;
    const count = String(n).length;
    if (count !== digits) {
      digits = count;
      pen.font = `${Math.min(px * TEXT, (px * 0.9) / (0.62 * count))}px "${FACE}", ui-monospace, monospace`;
    }
    pen.fillStyle = t === LIT || t === HIT ? paper : t === DIM ? accent : dim;
    pen.fillText(String(n), x, y);
  };
  const outline = (p, x, y, edge) => {
    const { geo } = plan;
    const px = geo.px;
    if (geo.lattice === 'hex') {
      const r = geo.s * (px >= GAP * view.dpr ? 1 - INSET : 1) - edge;
      p.moveTo(x + HEX[0][0] * r, y + HEX[0][1] * r);
      for (let i = 1; i < HEX.length; i++) p.lineTo(x + HEX[i][0] * r, y + HEX[i][1] * r);
      p.closePath();
      return;
    }
    const g = (px >= GAP * view.dpr ? Math.max(1, Math.round(px * INSET)) : 0) + edge;
    const left = Math.round(x - px / 2) + g;
    const top = Math.round(y - px / 2) + g;
    p.rect(left, top, Math.round(x + px / 2) - g - left, Math.round(y + px / 2) - g - top);
  };
  const stamp = (p, t, seen, every = false) => {
    const one = tones[t];
    if (!one) return;
    const { geo, tone, xs, ys } = plan;
    const wide = Math.max(view.dpr, Math.round(geo.px * OUTLINE));
    if (one.hollow) {
      p.strokeStyle = one.color;
      p.lineWidth = wide;
    } else p.fillStyle = one.color;
    let open = 0;
    const flush = () => {
      if (!open) return;
      if (one.hollow) p.stroke();
      else p.fill();
      open = 0;
    };
    for (const n of seen) {
      if (!every && tone[n] !== t) continue;
      if (!open) p.beginPath();
      const [x, y] = geo.at(xs[n], ys[n]);
      outline(p, x, y, one.hollow ? wide / 2 : 0);
      if (++open >= CHUNK) flush();
    }
    flush();
  };
  const ground = () => {
    if (!value.faint) return null;
    const plate = new OffscreenCanvas(view.w, view.h);
    const seen = [];
    for (let n = 1; n <= plan.top; n++) if (inside(n)) seen.push(n);
    stamp(plate.getContext('2d'), SOIL, seen, true);
    return plate;
  };
  const reset = () => {
    if (bed === undefined) bed = ground();
    if (layer) pen.clearRect(0, 0, view.w, view.h);
    else {
      layer = new OffscreenCanvas(view.w, view.h);
      pen = layer.getContext('2d');
      pen.textAlign = 'center';
      pen.textBaseline = 'middle';
    }
    if (bed) pen.drawImage(bed, 0, 0);
    digits = 0;
    drawn = 0;
  };
  const extend = (to) => {
    const { geo, tone, xs, ys } = plan;
    const labels = geo.px >= LABEL * view.dpr;
    const seen = [];
    for (let n = drawn + 1; n <= to; n++) if ((labels || (tone[n] && tone[n] !== SOIL)) && inside(n)) seen.push(n);
    drawn = Math.max(drawn, to);
    for (let t = 1; t < SOIL; t++) stamp(pen, t, seen);
    if (!labels) return;
    for (const n of seen) {
      const [x, y] = geo.at(xs[n], ys[n]);
      label(n, x, y, tone[n]);
    }
  };
  const rail = (to) => {
    const { line, runs, geo } = plan;
    if (!line) return;
    ctx.strokeStyle = veil(accent, RAIL);
    ctx.lineWidth = Math.max(view.dpr, geo.px * WIDTH);
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
    ctx.beginPath();
    for (const run of runs) {
      let open = false;
      for (const i of run) {
        if (line.values[i] > to) {
          open = false;
          continue;
        }
        const [x, y] = geo.at(line.cells[i][0], line.cells[i][1]);
        if (open) ctx.lineTo(x, y);
        else ctx.moveTo(x, y);
        open = true;
      }
    }
    ctx.stroke();
  };
  const mark = () => {
    if (!picked) return;
    const { geo } = plan;
    const [x, y] = geo.at(picked.x, picked.y);
    ctx.strokeStyle = accent;
    ctx.lineWidth = 2 * view.dpr;
    ctx.beginPath();
    ctx.arc(x, y, geo.px * RING, 0, TURN);
    ctx.stroke();
  };
  const draw = () => {
    const n = landed(num, plan.name, plan.rings, share(view.t - origin, length, view.still));
    if (layer && n === shown && !dirty) return;
    if (!layer || n < drawn) reset();
    extend(n);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(layer, 0, 0);
    rail(n);
    mark();
    shown = n;
    dirty = false;
  };
  const again = () => {
    origin = view.t;
    if (view.still) view.wake?.();
    else draw();
  };
  const theme = () => {
    paint();
    layer = null;
    bed = undefined;
  };
  const size = () => {
    if (value.fit) plan.geo = lay(num, value.lattice, fitted(value.lattice, plan.rings, view.w, view.h), view.w, view.h);
    else {
      plan = study(num, value, view.w, view.h, view.dpr);
      length = span(plan.rings);
      opts.onFacts?.(plan.facts);
    }
    layer = null;
    bed = undefined;
  };
  const pick = (u, v) => {
    const { geo, name } = plan;
    const [x, y] = geo.cell(u * view.w, v * view.h);
    const ring = Number(num.spiral.Lattice.ring_of(name, x, y));
    const hit = ring <= plan.rings && !(picked && picked.x === x && picked.y === y);
    if (hit) {
      const n = Number(num.spiral.Lattice.n(name, x, y));
      const pile = num.prime.pile(n);
      picked = { n, x, y, ring, prime: pile.prime, factors: pile.factors };
    } else picked = null;
    dirty = true;
    draw();
    return picked;
  };
  const svg = () => picture(plan, inside, tones);
  const csv = () => (plan.line ? table(plan.line) : '');
  paint();
  return {
    draw,
    size,
    theme,
    again,
    pick,
    svg,
    csv,
    font: FACE,
    get facts() {
      return plan.facts;
    },
  };
}
