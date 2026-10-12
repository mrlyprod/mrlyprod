import { tidy } from '../../lib/knobs.js';
import { TINTS, paints, rgb, rng, veil } from '../../lib/scene.js';
import { sheet } from '../../lib/svg.js';
import { CAPS, HOLD, INKS, LEAST, LOOKS, PAD, ROOTS, SIX, STRIDE, clock, count, glyph, hit, layout, named, nib, reach, roll, shade, study } from './engine.js';
import { circles, clip, labels, rules } from './shapes.js';

export const SPEC = [
  { key: 'root', label: 'Root quadruple', kind: 'pick', def: 'strip', options: ROOTS.map((name) => [name, named(name)]), group: 'Packing' },
  { key: 'cap', label: 'Curvature cap T', kind: 'pick', def: 2048, options: CAPS.map((cap) => [cap, String(cap)]), group: 'Packing' },
  { key: 'ink', label: 'Ink', kind: 'pick', def: 'size', options: INKS, group: 'Look' },
  { key: 'look', label: 'Discs', kind: 'segment', def: 'wash', options: LOOKS, group: 'Look' },
  { key: 'labels', label: 'Curvatures', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const ABOUT = [
  'Four mutually tangent circles obey Descartes\' rule. Swap one for the other circle that touches the remaining three and its curvature is k\' = 2(k1 + k2 + k3) - k4, with no square root. Written as the integer triple (k, kx, ky), a circle moves by the same rule in all three numbers, so an integer root quadruple grows the whole packing in exact integers, up to the curvature cap T.',
  'The strip root is two lines a unit apart. The circles it grows that rest on the lower line are exactly the Ford circles: over each reduced fraction a/b sits a circle of curvature 2b^2 touching the line at a/b. The Ford ink sets them apart.',
  'Info reads the Farey stack at the deepest depth Q the cap covers, 2Q^2 at most T: a node at each fraction a/b with b at most Q, lit floor(Q/b) times, every one touched by the Ford circle over it. The stack is the one [the Farey note](/research/farey/) builds.',
  'Tap a circle to read its triple and the fraction it rests on; Reroll draws a new root, cap and look. The growth, its six invariant checks, the census N(T), the Ford test and the stack are computed in Rust. The dimension of the packing, which sets how fast N(T) grows, is in [the Apollonian note](/research/apollonian/).',
];

export const units = () => import('./unit.js');

export const file = (value) => `apollonian-${value.root === 'strip' ? 'strip' : `bounded-${String(value.root).replace('-', '').split(',').join('-')}`}-${value.cap}`;

export const PAGE = {
  spec: SPEC,
  units,
  name: file,
  about: ABOUT,
  keys: [{ key: ['ArrowUp', 'ArrowDown'], label: 'Cap T', act: 'cap' }],
  actions: {
    cap: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

export const FACE = 'Noto Sans Mono';

const TAU = Math.PI * 2;
const WASH = 0.15;
const DIM = 0.45;
const EDGE = 1.5;
const HULL = 1.3;
const HALO = [4, 2];
const NIBS = 16;

const mix = (a, b, t) => `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(', ')})`;

const font = (size) => `${size}px "${FACE}", ui-monospace, monospace`;

export const fg = (paper) => {
  const [r, g, b] = rgb(paper);
  return 0.299 * r + 0.587 * g + 0.114 * b < 128 ? '#ffffff' : '#000000';
};

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('apollonian: the scene wants mrlyjs/num as opts.num');
  const ctx = canvas.getContext('2d');
  const whole = !view.lock;
  const tell = opts.onTell;
  let value = null;
  let plan = null;
  let lap = -1;
  let drawn = 0;
  let layer = null;
  let pen = null;
  let box = null;
  let picked = null;
  let dirty = true;
  let paper = '';
  let accent = '';
  let ink = '';
  let dim = '';
  let inks = [];
  const tone = (i) => {
    if (i < plan.roots) return 0;
    if (value.ink === 'size') return 1 + plan.hues[i];
    if (value.ink === 'shade') return 1 + plan.bands[i];
    if (value.ink === 'ford') return plan.fords[i] ? 1 : 2;
    return 1;
  };
  const fills = (t) => (value.look === 'wash' ? veil(inks[t], WASH) : value.look === 'solid' ? inks[t] : null);
  const strokes = (t) => (value.look === 'solid' ? paper : inks[t]);
  const paint = () => {
    const skin = view.look();
    paper = skin.paper;
    accent = skin.accent;
    ink = fg(paper);
    const p = rgb(paper);
    const a = rgb(accent);
    dim = mix(p, rgb(ink), DIM);
    const rest = {
      size: () => SIX.map((hue) => paints(canvas, hue).accent),
      shade: () => Array.from({ length: plan.octaves }, (_, b) => mix(p, a, shade(b, plan.octaves))),
      ford: () => [accent, dim],
      one: () => [ink],
    };
    inks = [value.look === 'solid' ? dim : ink, ...rest[value.ink]()];
  };
  const look = () => [value.root, value.cap, value.ink, value.look, value.labels, value.tint].join('|');
  const reset = () => {
    box = layout(plan.frame, view.w, view.h, PAD * view.dpr);
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    pen.textAlign = 'center';
    pen.textBaseline = 'middle';
    drawn = 0;
    dirty = true;
  };
  const X = (x) => box.ox + x * box.k;
  const Y = (y) => box.oy - y * box.k;
  const period = (u, v, k) => {
    const [x0, y0, x1, y1] = plan.frame;
    return [u(x0), v(y1), (x1 - x0) * k, (y1 - y0) * k];
  };
  const sorted = (from, to, k) => {
    const sets = new Map();
    for (let i = from; i < to; i++) {
      const key = tone(i) * NIBS + Math.round(nib(plan.data[i * STRIDE + 2] * k) * 10);
      const set = sets.get(key);
      if (set) set.push(i);
      else sets.set(key, [i]);
    }
    return sets;
  };
  const extend = (to) => {
    if (to <= drawn) return;
    const { data, ints, strip } = plan;
    pen.save();
    if (strip) {
      pen.beginPath();
      pen.rect(...period(X, Y, box.k));
      pen.clip();
    }
    for (const [key, list] of sorted(drawn, to, box.k / view.dpr)) {
      const t = Math.floor(key / NIBS);
      pen.beginPath();
      for (const i of list) {
        const r = data[i * STRIDE + 2] * box.k;
        const x = X(data[i * STRIDE]);
        const y = Y(data[i * STRIDE + 1]);
        pen.moveTo(x + r, y);
        pen.arc(x, y, r, 0, TAU);
      }
      const fill = fills(t);
      if (fill) {
        pen.fillStyle = fill;
        pen.fill();
      }
      pen.strokeStyle = strokes(t);
      pen.lineWidth = ((key % NIBS) / 10) * view.dpr;
      pen.stroke();
    }
    if (value.labels) {
      let last = 0;
      for (let i = drawn; i < to; i++) {
        const text = String(ints[i * 3]);
        const size = Math.floor(glyph(data[i * STRIDE + 2] * box.k, text.length));
        if (size < LEAST * view.dpr) continue;
        if (size !== last) pen.font = font(size);
        last = size;
        pen.fillStyle = strokes(tone(i));
        pen.fillText(text, X(data[i * STRIDE]), Y(data[i * STRIDE + 1]));
      }
    }
    pen.restore();
    drawn = to;
  };
  const container = () => {
    const [x0, y0, x1, y1] = plan.frame;
    if (plan.strip) {
      ctx.strokeStyle = dim;
      ctx.lineWidth = EDGE * view.dpr;
      ctx.beginPath();
      for (const y of [y0, y1]) {
        ctx.moveTo(X(x0), Y(y));
        ctx.lineTo(X(x1), Y(y));
      }
      ctx.stroke();
    } else if (plan.hull) {
      const { cx, cy, r } = plan.hull;
      ctx.beginPath();
      ctx.arc(X(cx), Y(cy), r * box.k, 0, TAU);
      if (value.look !== 'rings') {
        ctx.fillStyle = veil(ink, WASH);
        ctx.fill();
      }
      ctx.strokeStyle = ink;
      ctx.lineWidth = HULL * view.dpr;
      ctx.stroke();
    }
  };
  const halo = () => {
    const { data } = plan;
    ctx.beginPath();
    ctx.arc(X(data[picked * STRIDE]), Y(data[picked * STRIDE + 1]), data[picked * STRIDE + 2] * box.k, 0, TAU);
    ctx.strokeStyle = paper;
    ctx.lineWidth = HALO[0] * view.dpr;
    ctx.stroke();
    ctx.strokeStyle = accent;
    ctx.lineWidth = HALO[1] * view.dpr;
    ctx.stroke();
  };
  const reached = () => {
    const { loop, share } = clock(view.t, plan.span, HOLD, whole || view.still);
    return { loop, to: count(plan.data, plan.roots, plan.total, reach(plan.k0, value.cap, share)) };
  };
  const draw = () => {
    const { loop, to } = reached();
    if (!layer || loop !== lap || to < drawn) {
      reset();
      lap = loop;
    }
    if (!dirty && to === drawn) return;
    extend(to);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    container();
    ctx.drawImage(layer, 0, 0);
    if (picked !== null) halo();
    dirty = false;
  };
  const read = (i) => {
    const { ints, data, strip, at } = plan;
    const k = ints[i * 3];
    const x = ints[i * 3 + 1];
    const y = ints[i * 3 + 2];
    const c = num.apollonian.Circle.from({ k, x, y });
    const line = strip && num.apollonian.on_line(c);
    const ford = line && num.apollonian.is_ford(c);
    c.free();
    const cx = data[i * STRIDE];
    const touch = ford ? (at.get(cx) ?? (Number.isInteger(cx) ? { num: cx, den: 1 } : null)) : null;
    return { k, x, y, line, ford, num: touch?.num, den: touch?.den };
  };
  const facts = () => ({ ...plan.facts, circle: picked === null ? null : read(picked) });
  const tap = (u, v) => {
    if (!box) return null;
    const x = (u * view.w - box.ox) / box.k;
    const y = (box.oy - v * view.h) / box.k;
    const [x0, , x1] = plan.frame;
    const i = !plan.strip || (x >= x0 && x <= x1) ? hit(plan.data, drawn, x, y) : -1;
    picked = i < 0 || picked === i ? null : i;
    dirty = true;
    draw();
    tell?.(facts());
    return picked === null ? null : read(picked);
  };
  const set = (next) => {
    const rolled = next.root === undefined ? roll(rng(next.seed)) : null;
    const was = { plan, look: value && look() };
    value = tidy(SPEC, rolled ? { ...rolled, ...next, root: rolled.root } : next);
    plan = study(num, value.root, value.cap);
    if (plan !== was.plan) picked = null;
    if (look() !== was.look) {
      paint();
      layer = null;
    }
    dirty = true;
    tell?.(facts());
  };
  const theme = () => {
    paint();
    layer = null;
  };
  const size = () => {
    layer = null;
  };
  const svg = () => {
    const dpr = view.dpr;
    const w = view.w / dpr;
    const h = view.h / dpr;
    const css = layout(plan.frame, w, h, PAD);
    const U = (x) => css.ox + x * css.k;
    const V = (y) => css.oy - y * css.k;
    const { data, ints, strip, frame } = plan;
    const [x0, y0, x1, y1] = frame;
    const parts = [];
    if (strip) parts.push(rules([[U(x0), V(y0), U(x1), V(y0)], [U(x0), V(y1), U(x1), V(y1)]], dim, EDGE));
    else if (plan.hull) parts.push(circles([[U(plan.hull.cx), V(plan.hull.cy), plan.hull.r * css.k]], { fill: value.look === 'rings' ? 'none' : veil(ink, WASH), stroke: ink, width: HULL }));
    const body = [];
    for (const [key, list] of sorted(0, drawn, css.k)) {
      const t = Math.floor(key / NIBS);
      body.push(circles(list.map((i) => [U(data[i * STRIDE]), V(data[i * STRIDE + 1]), data[i * STRIDE + 2] * css.k]), { fill: fills(t) ?? 'none', stroke: strokes(t), width: (key % NIBS) / 10 }));
    }
    if (value.labels) {
      const texts = new Map();
      for (let i = 0; i < drawn; i++) {
        const text = String(ints[i * 3]);
        const size = Math.floor(glyph(data[i * STRIDE + 2] * css.k, text.length));
        if (size < LEAST) continue;
        const colour = strokes(tone(i));
        if (!texts.has(colour)) texts.set(colour, []);
        texts.get(colour).push([U(data[i * STRIDE]), V(data[i * STRIDE + 1]), size, text]);
      }
      for (const [colour, list] of texts) body.push(labels(list, colour, FACE));
    }
    parts.push(strip ? clip('period', period(U, V, css.k), body.join('')) : body.join(''));
    return sheet(w, h, parts.join(''));
  };
  const json = () => {
    const { ints, roots, total } = plan;
    const list = [];
    for (let i = roots; i < total; i++) list.push([ints[i * 3], ints[i * 3 + 1], ints[i * 3 + 2]]);
    return JSON.stringify({ root: value.root, quadruple: plan.quad, cap: value.cap, circles: list });
  };
  set(opts);
  return { draw, size, theme, set, tap, svg, json, still: whole, font: FACE };
}
