import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { DIM, HEX, HIT, LATTICES, LEADS, LIT, MARKS, MISS, REACH, SIDES, SOIL, fitted, ink, landed, lay, palette, picture, quadratic, read, share, span, study, table, toned } from './engine.js';

export const SPEC = [
  { key: 'lattice', label: 'Lattice', kind: 'segment', def: 'square', options: LATTICES, group: 'Sheet' },
  { key: 'side', label: 'Side', kind: 'slider', def: 201, min: SIDES[0], max: SIDES[1], step: 2, group: 'Sheet' },
  { key: 'a', label: 'a', kind: 'segment', def: 4, options: LEADS, group: 'Sheet' },
  { key: 'b', label: 'b', kind: 'number', def: -2, min: -REACH, max: REACH, step: 1, group: 'Sheet' },
  { key: 'c', label: 'c', kind: 'number', def: 41, min: -REACH, max: REACH, step: 1, group: 'Sheet' },
  { key: 'mark', label: 'Mark', kind: 'pick', def: 'prime', options: MARKS, group: 'Mark' },
  { key: 'faint', label: 'Faint composites', kind: 'toggle', def: 1, group: 'Mark' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: 'yellow', options: TINTS, group: 'Mark' },
];

const lined = (value) => value.a > 0;

const term = (k, word) => (k ? `${k < 0 ? '-' : '+'}${Math.abs(k)}${word}` : '');

export const named = (value) => ['ulam', value.lattice, value.side, ...(lined(value) ? [`${value.a}k2${term(value.b, 'k')}${term(value.c, '')}`] : []), ...(value.mark === 'prime' ? [] : [value.mark])].join('-');

export const PAGE = {
  spec: SPEC,
  name: named,
  about: [
    'Write 1 in the middle, wind 2, 3, 4 and on outward in a spiral, and light the primes. Diagonals appear: a straight line through the spiral takes one number from each ring, and those numbers are the values of a quadratic a k² + b k + c.',
    'On the square, ring k holds 8k cells and ends at (2k + 1)², so a diagonal has a = 4 and a line through the centre a = 1 or 2. On hexagons, ring r holds 6r cells and ends at 3r² + 3r + 1, so the straight lines have a = 3.',
    "Euler's m² - m + 41 is prime for m from 0 to 40. At m = 2k it is 4k² - 2k + 41, the line lit at the start: prime for its first 21 values, then broken by 1763 = 41 · 43.",
    'The mark is lit in the tint, pink is a Mobius value of minus one, and the quadratic lands orange on a prime and blue on a composite. Tap a cell for its number and its factors. Next c (Enter) and the arrows step c through the primes; Reroll rolls a new lattice, side and quadratic.',
  ],
  keys: [
    { key: 'Enter', label: 'Next c', act: 'next', button: true, when: lined },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Step c', act: 'line', when: lined },
  ],
  actions: {
    next: (scene) => scene.line?.(1),
    line: (scene, e) => scene.line?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

const GAP = 8;
const INSET = 0.07;
const RING = 0.8;
const HALO = 6;
const CHUNK = 4096;
const TURN = Math.PI * 2;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('ulam: the scene wants @mrlyprod/mrly/num as opts.num');
  const ctx = canvas.getContext('2d');
  let value = tidy(SPEC, opts);
  let plan = study(num, value, view.w, view.h);
  const length = span(plan.rings);
  let whole = false;
  let layer = null;
  let pen = null;
  let bed;
  let drawn = 0;
  let shown = -1;
  let dirty = true;
  let picked = null;
  let known = null;
  let paper;
  let tones;
  const paint = () => {
    const look = view.look();
    paper = look.paper;
    tones = palette(look.accent, paper);
  };
  const facts = () => (known = { ...read(num, plan, value), pick: picked });
  const tell = () => opts.onFacts?.(facts());
  const outline = (p, x, y) => {
    const { geo } = plan;
    const px = geo.px;
    if (geo.lattice === 'hex') {
      const r = geo.s * (px >= GAP * view.dpr ? 1 - INSET : 1);
      p.moveTo(x + HEX[0][0] * r, y + HEX[0][1] * r);
      for (let i = 1; i < HEX.length; i++) p.lineTo(x + HEX[i][0] * r, y + HEX[i][1] * r);
      p.closePath();
      return;
    }
    const g = px >= GAP * view.dpr ? Math.max(1, Math.round(px * INSET)) : 0;
    const left = Math.round(x - px / 2) + g;
    const top = Math.round(y - px / 2) + g;
    p.rect(left, top, Math.round(x + px / 2) - g - left, Math.round(y + px / 2) - g - top);
  };
  const stamp = (p, t, cells) => {
    const { geo } = plan;
    p.fillStyle = tones[t];
    let open = 0;
    for (const [x, y] of cells) {
      if (!open) p.beginPath();
      outline(p, ...geo.at(x, y));
      if (++open >= CHUNK) {
        p.fill();
        open = 0;
      }
    }
    if (open) p.fill();
  };
  const where = (ns) => ns.map((n) => [plan.xs[n], plan.ys[n]]);
  const ground = () => {
    if (!value.faint) return null;
    const plate = new OffscreenCanvas(view.w, view.h);
    stamp(plate.getContext('2d'), SOIL, where(Array.from({ length: plan.top }, (_, i) => i + 1)));
    return plate;
  };
  const reset = () => {
    if (bed === undefined) bed = ground();
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    if (bed) pen.drawImage(bed, 0, 0);
    drawn = 0;
  };
  const extend = (to) => {
    const { tone } = plan;
    const seen = [];
    for (let n = drawn + 1; n <= to; n++) if (tone[n] === LIT || tone[n] === DIM) seen.push(n);
    drawn = Math.max(drawn, to);
    for (const t of [LIT, DIM]) stamp(pen, t, where(seen.filter((n) => tone[n] === t)));
  };
  const overlay = (to) => {
    const { line } = plan;
    if (!line) return;
    const down = line.values.flatMap((n, i) => (n <= to ? [i] : []));
    for (const hit of [true, false]) stamp(ctx, hit ? HIT : MISS, down.filter((i) => line.hit[i] === hit).map((i) => line.cells[i]));
  };
  const ring = () => {
    if (!picked || picked.off) return;
    const { geo } = plan;
    const [x, y] = geo.at(picked.x, picked.y);
    ctx.strokeStyle = ink(paper);
    ctx.lineWidth = 2 * view.dpr;
    ctx.beginPath();
    ctx.arc(x, y, Math.max(geo.px * RING, HALO * view.dpr), 0, TURN);
    ctx.stroke();
  };
  const draw = () => {
    const n = whole ? plan.top : landed(num, plan.name, plan.rings, share(view.t, length, view.still));
    if (layer && n === shown && !dirty) return;
    if (!layer) reset();
    extend(n);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(layer, 0, 0);
    overlay(n);
    ring();
    if (n >= plan.top) whole = true;
    shown = n;
    dirty = false;
  };
  const fresh = (bare = true) => {
    layer = null;
    if (bare) bed = undefined;
    dirty = true;
  };
  const theme = () => {
    paint();
    fresh();
  };
  const size = () => {
    plan.geo = lay(num, plan.lattice, fitted(plan.lattice, plan.rings, view.w, view.h), view.w, view.h);
    fresh();
  };
  const set = (next) => {
    const was = value;
    value = tidy(SPEC, next);
    const moved = (...keys) => keys.some((key) => value[key] !== was[key]);
    whole = true;
    if (moved('lattice', 'side')) {
      plan = study(num, value, view.w, view.h);
      picked = null;
      fresh();
    } else {
      if (moved('mark', 'faint')) {
        plan = { ...plan, ...toned(num, value, plan.top) };
        fresh(moved('faint'));
      }
      if (moved('a', 'b', 'c')) plan = { ...plan, ...quadratic(num, value) };
    }
    if (moved('tint')) theme();
    dirty = true;
    tell();
  };
  const pick = (u, v) => {
    const { geo, name } = plan;
    const [x, y] = geo.cell(u * view.w, v * view.h);
    const at = Number(num.spiral.Lattice.ring_of(name, x, y));
    if (at > plan.rings) picked = { off: true };
    else if (picked && picked.x === x && picked.y === y) picked = null;
    else {
      const n = Number(num.spiral.Lattice.n(name, x, y));
      const pile = num.prime.pile(n);
      picked = { n, x, y, ring: at, prime: pile.prime, factors: pile.factors };
    }
    whole = true;
    dirty = true;
    draw();
    tell();
    return picked;
  };
  const svg = () => picture(plan, tones);
  const csv = () => (plan.line ? table(plan.line) : '');
  paint();
  facts();
  return {
    draw,
    size,
    theme,
    set,
    pick,
    svg,
    csv,
    get facts() {
      return known;
    },
  };
}
