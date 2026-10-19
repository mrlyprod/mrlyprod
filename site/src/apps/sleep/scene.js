import { tidy } from '../../lib/app/spec.js';
import { TINTS } from '../../lib/color.js';
import { pick } from '../../lib/rand.js';
import { DESIGNS, LEVELS, NUMBERS, ink, tile } from '../../lib/designs/tiles.js';

const HIDE = () => false;

const named = (word) => word[0].toUpperCase() + word.slice(1);

const MIXED = ['', 'Mixed'];

export const TILES = [['', ''], ...DESIGNS.flatMap((design) => NUMBERS.map((n) => [design, n]))];

export const SPEC = [
  { key: 'design', label: 'Design', kind: 'pick', def: '', options: [MIXED, ...DESIGNS.map((design) => [design, named(design)])], group: 'Shape', when: HIDE },
  { key: 'number', label: 'Number', kind: 'pick', def: '', options: [MIXED, ...NUMBERS.map((n) => [n, String(n)])], group: 'Shape', when: HIDE },
  { key: 'level', label: 'Level', kind: 'pick', def: '', options: [MIXED, ...LEVELS.map((n) => [n, String(n)])], group: 'Shape' },
  { key: 'count', label: 'Tiles', kind: 'slider', def: 8, min: 1, max: 16, step: 1, group: 'Shape' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 180, min: 60, max: 600, step: 20, unit: 'px/s', group: 'Motion' },
  { key: 'size', label: 'Size', kind: 'slider', def: 0.42, min: 0.1, max: 0.8, step: 0.02, group: 'Size' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Next tile', act: 'next', button: true, menu: 'View' },
    { key: ['ArrowLeft', 'ArrowUp'], label: 'Previous tile', act: 'prev' },
    { key: ['ArrowRight', 'ArrowDown'], label: 'Next', act: 'next' },
  ],
  actions: { next: (scene) => scene.step?.(1), prev: (scene) => scene.step?.(-1) },
};

export const SAVERS = [
  { id: 'crowd', title: 'Crowd', pick: (rand) => ({ design: '', count: 7 + Math.floor(rand() * 3), size: 0.42, speed: 80 }) },
  { id: 'giants', title: 'Giants', pick: (rand) => ({ design: pick(rand, DESIGNS), count: 3, size: 0.7, speed: 60 }) },
];

const DEEP = 0.4;

const GOLD = 0.618034;

const depth = (k) => DEEP + (1 - DEEP) * ((k * GOLD) % 1);

const veil = (d) => 0.25 + 0.65 * d;

const FINE = 25;

export function next(value, by) {
  const at = TILES.findIndex(([design, number]) => design === value.design && number === value.number);
  const [design, number] = TILES[(at + by + TILES.length) % TILES.length];
  return { design, number };
}

export function bounce(p, v, max) {
  if (p <= 0) return { p: Math.min(-p, max), v: Math.abs(v), hit: true };
  if (p >= max) return { p: Math.max(2 * max - p, 0), v: -Math.abs(v), hit: true };
  return { p, v, hit: false };
}

export function make(canvas, view, opts) {
  const ctx = canvas.getContext('2d');
  const rand = () => view.rand();
  let knobs = tidy(SPEC, opts);
  let seed = opts.seed;
  let crowd = [];
  let order = [];
  let last = view.t;
  const roll = () => {
    const design = knobs.design || pick(rand, DESIGNS);
    const number = knobs.number || pick(rand, NUMBERS);
    const level = knobs.level || pick(rand, LEVELS);
    return tile(design, number, view.lock && number ** level > FINE ? 1 : level);
  };
  const fit = (one) => {
    const n = one.mark.size;
    const reach = Math.min(Math.sqrt(view.w * view.h) * knobs.size * one.d, view.w, view.h);
    one.side = Math.max(1, Math.floor(reach / n)) * n;
    one.x = Math.min(one.x, Math.max(0, view.w - one.side));
    one.y = Math.min(one.y, Math.max(0, view.h - one.side));
  };
  const spawn = (k) => {
    const one = { d: depth(k), mark: roll(), vx: (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand()), vy: (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand()), x: 0, y: 0, side: 0 };
    fit(one);
    one.x = rand() * Math.max(0, view.w - one.side);
    one.y = rand() * Math.max(0, view.h - one.side);
    return one;
  };
  const gather = () => {
    crowd.length = Math.min(crowd.length, knobs.count);
    while (crowd.length < knobs.count) crowd.push(spawn(crowd.length));
    order = [...crowd].sort((a, b) => a.d - b.d);
  };
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = view.look().accent;
    for (const one of order) {
      ctx.globalAlpha = veil(one.d);
      ink(ctx, one.mark, Math.round(one.x), Math.round(one.y), one.side);
    }
    ctx.globalAlpha = 1;
  };
  const move = (one, run) => {
    const hx = bounce(one.x + one.vx * run * one.d, one.vx, Math.max(0, view.w - one.side));
    const hy = bounce(one.y + one.vy * run * one.d, one.vy, Math.max(0, view.h - one.side));
    one.x = hx.p;
    one.vx = hx.v;
    one.y = hy.p;
    one.vy = hy.v;
    if (!hx.hit && !hy.hit) return;
    one.mark = roll();
    fit(one);
  };
  const draw = () => {
    const run = (knobs.speed * view.dpr * (view.t - last)) / 1000;
    last = view.t;
    if (run > 0) for (const one of crowd) move(one, run);
    paint();
  };
  const size = () => crowd.forEach(fit);
  const set = (next) => {
    const was = knobs;
    knobs = tidy(SPEC, next);
    if (next.seed !== seed) {
      seed = next.seed;
      crowd = [];
    } else if (['design', 'number', 'level'].some((key) => knobs[key] !== was[key])) {
      for (const one of crowd) one.mark = roll();
    }
    size();
    gather();
  };
  gather();
  return { draw, size, theme: paint, set };
}
