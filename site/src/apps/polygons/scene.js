import { palette as PAINT } from '../../lib/theme/palette.gen.js';
import { tidy } from '../../lib/app/spec.js';
import { HUES, TINTS, rgb } from '../../lib/color.js';
import { BAND, CROSS, FIRST, PALETTES, SHAPES, WAVES, art, glow, live, paint, peak, ripple, spread, tiles, wave } from './engine.js';

const named = (word) => word[0].toUpperCase() + word.slice(1);

const moving = (v) => Boolean(v.shimmer || v.morph);

export const SPEC = [
  { key: 'shape', label: 'Shape', kind: 'pick', def: 'square', options: SHAPES.map((one) => [one, named(one)]), group: 'Tiles' },
  { key: 'size', label: 'Size', kind: 'slider', def: 0, min: 0, max: 240, step: 8, unit: 'px', zero: 'Fit', group: 'Tiles' },
  { key: 'palette', label: 'Palette', kind: 'pick', def: '', options: [['', 'Seeded'], ...Object.keys(PALETTES).map((one) => [one, named(one)])], group: 'Colour' },
  { key: 'noise', label: 'Noise', kind: 'slider', def: 0.9, min: 0, max: 2, step: 0.05, group: 'Colour' },
  { key: 'spark', label: 'Sparks', kind: 'slider', def: 0.04, min: 0, max: 0.3, step: 0.01, group: 'Colour' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Colour' },
  { key: 'shimmer', label: 'Shimmer', kind: 'toggle', def: 1, group: 'Waves' },
  { key: 'morph', label: 'Recolour', kind: 'toggle', def: 1, group: 'Waves' },
  { key: 'wave', label: 'Wave', kind: 'pick', def: 'diagonal', options: WAVES.map((one) => [one, named(one)]), group: 'Waves', when: moving },
  { key: 'every', label: 'Every', kind: 'slider', def: 6, min: 3, max: 30, step: 1, unit: 's', group: 'Waves', when: moving },
  { key: 'outline', label: 'Outline', kind: 'toggle', def: 0, group: 'Outline' },
  { key: 'line', label: 'Width', kind: 'slider', def: 2, min: 1, max: 12, step: 1, unit: 'px', group: 'Outline', when: (v) => Boolean(v.outline) },
  { key: 'stroke', label: 'Colour', kind: 'pick', def: 'ground', options: [['ground', 'Ground'], ['ink', 'Ink'], ['accent', 'Tint'], ...HUES.map((hue) => [hue, named(hue)])], group: 'Outline', when: (v) => Boolean(v.outline) },
];

export const PAGE = {
  spec: SPEC,
  keys: [{ key: 'Enter', label: 'Wave', act: 'again', button: true }],
  actions: { again: (scene) => scene.again() },
  record: true,
};

const FIT = 20;

const LEVELS = 8;

const LIGHT = 0.3;

const FACES = { cube: [0.18, 0, -0.24], mix: [0, -0.12, 0.14] };

const TONES = { tint1: ['paper', 'accent', 0.3], tint2: ['paper', 'accent', 0.55], tint3: ['paper', 'accent', 0.8], ink1: ['paper', 'fg', 0.12], ink2: ['paper', 'fg', 0.3], ink3: ['paper', 'fg', 0.55], ink4: ['paper', 'fg', 0.8] };

const blend = (a, b, x) => a.map((v, i) => v + (b[i] - v) * x);

const css = (c) => `rgb(${c.map(Math.round).join(', ')})`;

const trace = (pen, cell) => {
  const p = cell.pts;
  pen.moveTo(p[0][0], p[0][1]);
  for (let i = 1; i < p.length; i++) pen.lineTo(p[i][0], p[i][1]);
  pen.closePath();
};

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const knobs = tidy(SPEC, opts);
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const every = knobs.every * 1000;
  const cache = canvas.ownerDocument?.createElement('canvas') ?? new OffscreenCanvas(1, 1);
  const pen = cache.getContext('2d');
  let grid = null;
  let k = new Int32Array(0);
  let inks = {};
  const tones = new Map();
  const looks = new Map();
  const plans = new Map();
  let manual = [];
  let done = 0;
  let taps = 0;
  let then = -1;
  let shown = false;
  let stale = true;
  const read = () => {
    const style = typeof getComputedStyle === 'function' ? getComputedStyle(canvas) : null;
    const get = (name, fallback) => style?.getPropertyValue(name).trim() || fallback;
    const { paper, accent } = view.look();
    inks = { paper: rgb(paper), accent: rgb(accent), fg: rgb(get('--fg', PAINT.black)) };
    for (const hue of HUES) inks[hue] = rgb(get(`--${hue}`, PAINT[hue]));
    for (const [name, [from, to, x]] of Object.entries(TONES)) inks[name] = blend(inks[from], inks[to], x);
    inks.ground = inks.paper;
    inks.ink = inks.fg;
    tones.clear();
  };
  const tone = (name, face) => {
    const key = `${name} ${face}`;
    if (!tones.has(key)) {
      const lift = FACES[knobs.shape]?.[face] ?? 0;
      tones.set(key, css(lift ? blend(inks[name], lift > 0 ? [255, 255, 255] : [0, 0, 0], Math.abs(lift)) : inks[name]));
    }
    return tones.get(key);
  };
  const look = (n) => {
    if (!looks.has(n)) {
      const one = art(seed, n, knobs.palette);
      looks.set(n, { ...one, along: spread(one.turn, view.w, view.h) });
    }
    return looks.get(n);
  };
  const plan = (n) => {
    if (!plans.has(n)) plans.set(n, wave(knobs.wave, seed, n, view.w, view.h, FIRST + n * every));
    return plans.get(n);
  };
  const index = (t) => (moving(knobs) && t >= FIRST ? Math.floor((t - FIRST) / every) : -1);
  const within = (one, from, to) => one.start <= to && one.start + CROSS + BAND >= from;
  const waves = (from, to) => {
    const n = index(to);
    return [n - 1, n].filter((m) => m >= 0).map(plan).filter((one) => within(one, from, to)).concat(manual);
  };
  const count = (cell, t) => {
    if (!knobs.morph) return 0;
    const n = index(t);
    let c = done + Math.max(n, 0);
    if (n >= 0 && peak(plan(n), cell) <= t) c++;
    for (const one of manual) if (peak(one, cell) <= t) c++;
    return c;
  };
  const colour = (c) => {
    const cell = grid.cells[c];
    const one = look(k[c]);
    return tone(paint(cell, one, one.along, knobs), cell.face);
  };
  const edge = (on) => {
    on.strokeStyle = css(inks[knobs.stroke]);
    on.lineWidth = knobs.line * view.dpr;
    on.stroke();
  };
  const seam = (on, col) => {
    if (grid.crisp) return;
    on.strokeStyle = col;
    on.lineWidth = 1;
    on.stroke();
  };
  const fill = (c) => {
    const col = colour(c);
    pen.beginPath();
    trace(pen, grid.cells[c]);
    pen.fillStyle = col;
    pen.fill();
    seam(pen, col);
    if (knobs.outline) edge(pen);
  };
  const rebuild = (t) => {
    grid ??= tiles(knobs.shape, view.w, view.h, (knobs.size * view.dpr || Math.max(view.w, view.h) / FIT));
    if (cache.width !== view.w || cache.height !== view.h) {
      cache.width = view.w;
      cache.height = view.h;
    }
    k = Int32Array.from(grid.cells, (cell) => count(cell, t));
    pen.fillStyle = css(inks.paper);
    pen.fillRect(0, 0, view.w, view.h);
    const groups = new Map();
    grid.cells.forEach((cell, c) => {
      const col = colour(c);
      if (!groups.has(col)) groups.set(col, []);
      groups.get(col).push(cell);
    });
    for (const [col, list] of groups) {
      pen.beginPath();
      for (const cell of list) trace(pen, cell);
      pen.fillStyle = col;
      pen.fill();
      seam(pen, col);
    }
    if (knobs.outline) {
      pen.beginPath();
      for (const cell of grid.cells) trace(pen, cell);
      edge(pen);
    }
    stale = false;
  };
  const flip = (t) => {
    if (!knobs.morph) return false;
    let any = false;
    for (const one of waves(then, t)) {
      grid.cells.forEach((cell, c) => {
        const at = peak(one, cell);
        if (at <= then || at > t) return;
        k[c]++;
        fill(c);
        any = true;
      });
    }
    return any;
  };
  const retire = (t) => {
    manual = manual.filter((one) => {
      const over = t > one.start + CROSS + BAND;
      if (over && knobs.morph) done++;
      return !over;
    });
  };
  const shine = (t) => {
    if (!knobs.shimmer) return null;
    const lit = waves(t, t).filter((one) => live(one, t));
    if (!lit.length) return null;
    const buckets = Array.from({ length: LEVELS }, () => []);
    let any = false;
    for (const one of lit) {
      for (const cell of grid.cells) {
        const g = glow(one, cell, t);
        if (g < 1 / LEVELS / 2) continue;
        buckets[Math.min(LEVELS - 1, Math.round(g * LEVELS) - 1)].push(cell);
        any = true;
      }
    }
    return any ? buckets : null;
  };
  const draw = () => {
    const t = view.t;
    let fresh = stale || t < then;
    if (fresh) rebuild(t);
    else fresh = flip(t);
    retire(t);
    const buckets = shine(t);
    then = t;
    if (!fresh && !buckets && !shown) return;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.drawImage(cache, 0, 0);
    shown = Boolean(buckets);
    if (!buckets) return;
    buckets.forEach((list, b) => {
      if (!list.length) return;
      ctx.beginPath();
      for (const cell of list) trace(ctx, cell);
      ctx.fillStyle = `rgba(255, 255, 255, ${((b + 1) / LEVELS) * LIGHT})`;
      ctx.fill();
      if (knobs.outline) edge(ctx);
    });
  };
  const size = () => {
    grid = null;
    looks.clear();
    plans.clear();
    stale = true;
  };
  const theme = () => {
    read();
    stale = true;
  };
  const start = (one) => {
    manual.push(one);
    if (view.still) view.wake?.();
  };
  const poke = (fx, fy) => start(ripple(fx * view.w, fy * view.h, view.w, view.h, view.t));
  const again = () => start(wave(knobs.wave, seed, -++taps, view.w, view.h, view.t));
  read();
  return { draw, size, theme, poke, again };
}
