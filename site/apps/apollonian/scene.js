import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, rng, veil } from '../../lib/scene.js';
import { sheet } from '../../lib/svg.js';
import { BAND, CAPS, DUST, HOLD, LEAST, LOOKS, PAD, ROOTS, STRIDE, choose, clock, count, depth, glyph, hit, layout, named, reach, shade, stack, study, thick } from './engine.js';
import { box as rect, discs, labels, rings, rules } from './shapes.js';

const onStrip = (value) => choose(value.root, rng(value.seed)) === 'strip';

export const SPEC = [
  { key: 'root', label: 'Root', kind: 'pick', def: '', options: [['', 'Random'], ...ROOTS.map((name) => [name, named(name)])], group: 'Packing' },
  { key: 'cap', label: 'Cap', kind: 'pick', def: 2048, options: CAPS.map((cap) => [cap, String(cap)]), group: 'Packing' },
  { key: 'look', label: 'Look', kind: 'pick', def: 'fill', options: LOOKS, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
  { key: 'ford', label: 'Ford', kind: 'toggle', def: 0, group: 'Look', when: onStrip },
  { key: 'order', label: 'Depth', kind: 'slider', def: 0, min: 0, max: (value) => stack(value.cap), step: 1, group: 'Stack', when: onStrip },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Again', act: 'again', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Cap', act: 'cap' },
  ],
  actions: {
    again: (scene) => scene.again(),
    cap: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

export const units = () => import('./unit.js');

export const FACE = 'Noto Sans Mono';

const TAU = Math.PI * 2;
const WASH = 0.14;
const EDGE = 0.45;
const GAP = 6;
const HALO = [4, 2];
const DIM = 0.2;
const FAINT = 0.3;
const REST = 0.4;
const WIDTHS = [0.6, 1.4];
const TICK = 4;

const mix = (a, b, t) => `rgb(${a.map((v, i) => Math.round(v + (b[i] - v) * t)).join(', ')})`;

const font = (size) => `${size}px "${FACE}", ui-monospace, monospace`;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const num = opts.num;
  if (!num) throw new Error('apollonian: the scene wants mrlyjs/num as opts.num');
  const ctx = canvas.getContext('2d');
  const value = tidy(SPEC, opts);
  const name = choose(value.root, view.rand);
  const plan = study(num, name, value.cap);
  const { bars, ...counts } = depth(num, plan, value.order);
  const { data, ints, bands, fords, total, roots, strip, octaves } = plan;
  const look = value.look;
  const once = Boolean(opts.once);
  const apart = strip && value.ford === 1;
  const widths = apart ? WIDTHS : Array.from({ length: octaves }, (_, b) => thick(b, octaves));
  const tones = widths.length;
  const tone = apart ? (i) => fords[i] : (i) => bands[i];
  let origin = 0;
  let lap = -1;
  let drawn = 0;
  let lit = 0;
  let layer = null;
  let pen = null;
  let box = null;
  let picked = null;
  let accent = '#000';
  let paper = '#fff';
  let wash = '';
  let shades = [];
  let edges = [];
  const paint = () => {
    const paints = view.look();
    accent = paints.accent;
    paper = paints.paper;
    const a = rgb(accent);
    const p = rgb(paper);
    shades = apart ? [mix(p, a, FAINT), accent] : Array.from({ length: octaves }, (_, b) => mix(p, a, shade(b, octaves)));
    edges = apart ? [veil(accent, REST), accent] : shades.map(() => accent);
    wash = mix(p, a, WASH);
  };
  const band = () => (strip && bars.length ? Math.round(view.h * BAND) : 0);
  const reset = () => {
    box = layout(plan.frame, strip, view.w, view.h, PAD * view.dpr, band());
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext('2d');
    pen.textAlign = 'center';
    pen.textBaseline = 'middle';
    drawn = 0;
    lit = 0;
  };
  const X = (x) => box.ox + x * box.k;
  const Y = (y) => box.oy - y * box.k;
  const extend = (to, K) => {
    const [j0, j1] = box.periods;
    if (to > drawn) {
      const groups = Array.from({ length: tones }, () => []);
      for (let i = drawn; i < to; i++) if (data[i * STRIDE + 2] * box.k >= DUST) groups[tone(i)].push(i);
      groups.forEach((group, t) => {
        if (!group.length) return;
        pen.beginPath();
        for (const i of group) {
          const r = data[i * STRIDE + 2] * box.k;
          const y = Y(data[i * STRIDE + 1]);
          for (let j = j0; j <= j1; j++) {
            const x = X(data[i * STRIDE] + j);
            pen.moveTo(x + r, y);
            pen.arc(x, y, r, 0, TAU);
          }
        }
        if (look === 'fill') {
          pen.fillStyle = shades[t];
          pen.fill();
        } else {
          pen.strokeStyle = edges[t];
          pen.lineWidth = widths[t] * view.dpr;
          pen.stroke();
        }
      });
      if (look === 'labels') {
        let last = 0;
        let ink = '';
        for (let i = drawn; i < to; i++) {
          const text = String(ints[i * 3]);
          const size = Math.floor(glyph(data[i * STRIDE + 2] * box.k, text.length));
          if (size < LEAST * view.dpr) continue;
          if (size !== last) pen.font = font(size);
          last = size;
          if (edges[tone(i)] !== ink) {
            ink = edges[tone(i)];
            pen.fillStyle = ink;
          }
          const y = Y(data[i * STRIDE + 1]);
          for (let j = j0; j <= j1; j++) pen.fillText(text, X(data[i * STRIDE] + j), y);
        }
      }
      if (apart) {
        const half = TICK * view.dpr;
        pen.beginPath();
        for (let i = drawn; i < to; i++) {
          if (!fords[i]) continue;
          for (let j = j0; j <= j1; j++) {
            const x = X(data[i * STRIDE] + j);
            pen.moveTo(x, box.line - half);
            pen.lineTo(x, box.line + half);
          }
        }
        pen.strokeStyle = accent;
        pen.lineWidth = view.dpr;
        pen.stroke();
      }
      drawn = to;
    }
    if (lit < bars.length && bars[lit].k <= K) {
      const top = box.line + GAP * view.dpr;
      const tall = band() - 2 * GAP * view.dpr;
      const groups = new Map();
      while (lit < bars.length && bars[lit].k <= K) {
        const bar = bars[lit++];
        if (!groups.has(bar.h)) groups.set(bar.h, []);
        groups.get(bar.h).push(bar.x);
      }
      pen.lineWidth = view.dpr;
      for (const [h, xs] of groups) {
        pen.strokeStyle = veil(accent, DIM + (1 - DIM) * h);
        pen.beginPath();
        for (const x of xs) {
          for (let j = j0; j <= j1; j++) {
            const px = X(x + j);
            pen.moveTo(px, top);
            pen.lineTo(px, top + tall * h);
          }
        }
        pen.stroke();
      }
    }
  };
  const container = () => {
    if (strip) {
      const top = Y(plan.frame[3]);
      const bottom = box.line;
      if (look === 'fill') {
        ctx.fillStyle = wash;
        ctx.fillRect(0, top, view.w, bottom - top);
      } else {
        ctx.strokeStyle = veil(accent, EDGE);
        ctx.lineWidth = view.dpr;
        ctx.beginPath();
        ctx.moveTo(0, top);
        ctx.lineTo(view.w, top);
        ctx.moveTo(0, bottom);
        ctx.lineTo(view.w, bottom);
        ctx.stroke();
      }
    } else if (plan.hull) {
      const { cx, cy, r } = plan.hull;
      ctx.beginPath();
      ctx.arc(X(cx), Y(cy), r * box.k, 0, TAU);
      if (look === 'fill') {
        ctx.fillStyle = wash;
        ctx.fill();
      } else {
        ctx.strokeStyle = accent;
        ctx.lineWidth = thick(0, octaves) * view.dpr;
        ctx.stroke();
      }
    }
  };
  const halo = () => {
    const { i, j } = picked;
    ctx.beginPath();
    ctx.arc(X(data[i * STRIDE] + j), Y(data[i * STRIDE + 1]), data[i * STRIDE + 2] * box.k, 0, TAU);
    ctx.strokeStyle = paper;
    ctx.lineWidth = HALO[0] * view.dpr;
    ctx.stroke();
    ctx.strokeStyle = accent;
    ctx.lineWidth = HALO[1] * view.dpr;
    ctx.stroke();
  };
  const reached = () => {
    const { loop, share } = clock(view.t - origin, plan.span, HOLD, view.still, once);
    const K = reach(plan.k0, value.cap, share);
    return { loop, K, to: count(data, roots, total, K) };
  };
  const draw = () => {
    const { loop, K, to } = reached();
    if (!layer || loop !== lap || to < drawn) {
      reset();
      lap = loop;
    }
    extend(to, K);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    container();
    ctx.drawImage(layer, 0, 0);
    if (picked) halo();
  };
  const read = (i, j) => {
    const k = ints[i * 3];
    const x = ints[i * 3 + 1] + j * k;
    const y = ints[i * 3 + 2];
    const c = num.apollonian.Circle.from({ k, x, y });
    const line = strip && num.apollonian.on_line(c);
    const ford = line && num.apollonian.is_ford(c);
    c.free();
    const cx = data[i * STRIDE];
    const base = ford ? (plan.at.get(cx) ?? (Number.isInteger(cx) ? { num: cx, den: 1 } : null)) : null;
    const touch = base && { num: base.num + j * base.den, den: base.den };
    return { k, x, y, line, ford, num: touch?.num, den: touch?.den };
  };
  const tap = (u, v) => {
    if (!box) return null;
    let x = (u * view.w - box.ox) / box.k;
    const y = (box.oy - v * view.h) / box.k;
    let j = 0;
    if (strip) {
      j = Math.floor(x);
      x -= j;
    }
    const i = hit(data, drawn, x, y);
    picked = i < 0 || (picked && picked.i === i && picked.j === j) ? null : { i, j };
    draw();
    return picked ? read(i, j) : null;
  };
  const again = () => {
    origin = view.t;
    picked = null;
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
  const svg = () => {
    const dpr = view.dpr;
    const w = view.w / dpr;
    const h = view.h / dpr;
    const tall = band() / dpr;
    const css = layout(plan.frame, strip, w, h, PAD, tall);
    const U = (x) => css.ox + x * css.k;
    const V = (y) => css.oy - y * css.k;
    const [j0, j1] = css.periods;
    const spread = (cx, fn) => {
      for (let j = j0; j <= j1; j++) fn(U(cx + j));
    };
    const parts = [];
    if (strip) {
      const top = V(plan.frame[3]);
      parts.push(look === 'fill' ? rect(0, top, w, css.line - top, wash) : rules([[0, top, w, top], [0, css.line, w, css.line]], veil(accent, EDGE), 1));
    } else if (plan.hull) {
      const { cx, cy, r } = plan.hull;
      parts.push(look === 'fill' ? discs([[U(cx), V(cy), r * css.k]], wash) : rings([[U(cx), V(cy), r * css.k]], accent, thick(0, octaves)));
    }
    const groups = Array.from({ length: tones }, () => []);
    const texts = Array.from({ length: tones }, () => []);
    const ticks = [];
    for (let i = 0; i < drawn; i++) {
      if (apart && fords[i]) spread(data[i * STRIDE], (x) => ticks.push([x, css.line - TICK, x, css.line + TICK]));
      const r = data[i * STRIDE + 2] * css.k;
      if (r < DUST) continue;
      const y = V(data[i * STRIDE + 1]);
      spread(data[i * STRIDE], (x) => groups[tone(i)].push([x, y, r]));
      if (look !== 'labels') continue;
      const text = String(ints[i * 3]);
      const size = Math.floor(glyph(r, text.length));
      if (size >= LEAST) spread(data[i * STRIDE], (x) => texts[tone(i)].push([x, y, size, text]));
    }
    groups.forEach((group, t) => parts.push(look === 'fill' ? discs(group, shades[t]) : rings(group, edges[t], widths[t])));
    texts.forEach((group, t) => parts.push(labels(group, edges[t], FACE)));
    parts.push(rules(ticks, accent, 1));
    const top = css.line + GAP;
    const room = tall - 2 * GAP;
    const struts = [];
    for (let b = 0; b < lit; b++) spread(bars[b].x, (x) => struts.push([x, top, x, top + room * bars[b].h, DIM + (1 - DIM) * bars[b].h]));
    parts.push(rules(struts, accent, 1));
    return sheet(w, h, parts.join(''));
  };
  const json = () => {
    const circles = [];
    for (let i = roots; i < total; i++) circles.push([ints[i * 3], ints[i * 3 + 1], ints[i * 3 + 2]]);
    return JSON.stringify({ root: name, quadruple: plan.quad, cap: value.cap, circles });
  };
  paint();
  return { draw, size, theme, again, tap, svg, json, facts: { ...plan.facts, ...counts }, font: FACE };
}
