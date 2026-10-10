import { tidy } from '../../lib/knobs.js';
import { board, veil } from '../../lib/scene.js';
import { BOARD, LINE, MASK, ROWS, STARTS, fate, hash, maskName, maskOf, population, ruleOf, sow } from './engine.js';

const NUMBERS = [3, 5, 7, 9];
const LEVELS = [1, 2, 3];

const LIFE = (value) => value.mode !== 'wolfram';
const WOLFRAM = (value) => value.mode === 'wolfram';
const MASKED = (value) => value.mode !== 'wolfram' || value.from === 'design';
const SOUP = (value) => value.from === 'noise';

export const SPEC = [
  { key: 'mode', label: 'Mode', kind: 'segment', def: 'life', options: [['life', 'Life'], ['wolfram', 'Wolfram']], group: 'Rule' },
  { key: 'born', label: 'Born', kind: 'text', def: '3', group: 'Rule', when: LIFE },
  { key: 'stay', label: 'Stay', kind: 'text', def: '23', group: 'Rule', when: LIFE },
  { key: 'rule', label: 'Rule', kind: 'slider', def: 110, min: 0, max: 255, step: 1, group: 'Rule', when: WOLFRAM },
  { key: 'code', label: 'Code', kind: 'text', def: '7', group: 'Mask', when: MASKED },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Mask', when: MASKED },
  { key: 'level', label: 'Level', kind: 'pick', def: 1, options: (value) => LEVELS.filter((n) => value.number ** n <= MASK).map((n) => [n, String(n)]), group: 'Mask', when: LIFE },
  { key: 'from', label: 'Start', kind: 'pick', def: 'noise', options: STARTS, group: 'Board' },
  { key: 'density', label: 'Density', kind: 'slider', def: 0.3, min: 0.05, max: 0.95, step: 0.05, group: 'Board', when: SOUP },
  { key: 'wrap', label: 'Wrap', kind: 'toggle', def: 1, group: 'Board' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 12, min: 1, max: 60, step: 1, unit: '/s', group: 'Pace' },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  record: true,
  keys: [
    { key: ['n', 'Enter'], label: 'Step', act: 'step' },
    { key: 'c', label: 'Clear', act: 'clear' },
  ],
  actions: { step: (scene) => scene.step(), clear: (scene) => scene.clear() },
};

export const units = () => import('./unit.js');

export const BOARD_KEYS = ['mode', 'from', 'code', 'number', 'level', 'density', 'seed'];

export const HOLD = 3000;

const FIT = 0.96;
const GROUND = 0.07;
const OLD = 0.55;
const OPS = 4e6;
const BURST = 64;
const STILL = 24;
const THUMB = 24;
const EPS = 1e-6;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const { life, math } = opts;
  let was = null;
  let cached = null;
  const now = () => {
    const current = opts.live?.current ?? opts;
    if (current !== was) {
      was = current;
      cached = tidy(SPEC, current);
    }
    return cached;
  };
  const value = now();
  const flat = value.mode !== 'wolfram';
  const { mask, cells, code } = flat ? maskOf(life, value) : { mask: null, cells: 0, code: 0 };
  const on = flat ? maskName(math, code) : '';
  const width = flat ? BOARD : LINE;
  let types = sow(value, view.rand, math);
  let age = Uint8Array.from(types);
  const ring = flat ? null : new Uint8Array(ROWS * LINE);
  if (ring) ring.set(types, 0);
  let gen = 0;
  let origin = 0;
  let speed = 0;
  let seen = [];
  let word = fate(seen, hash(types), population(types));
  let key = '';
  let rule = null;
  let name = '';
  let lay = null;
  let skin = null;
  let shown = -1;
  let dirty = true;
  let gone = false;
  let brush = 1;
  let hands = false;
  let since = -1;
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const read = () => {
    const live = now();
    if (flat) {
      const at = `${live.born}|${live.stay}`;
      if (at !== key) {
        key = at;
        rule = ruleOf(life, live, cells);
        name = on ? `${rule.name} on ${on}` : rule.name;
      }
    } else if (live.rule !== rule) {
      rule = live.rule;
      const gasket = life.gasket(rule);
      name = gasket ? `${life.rule_name(rule)} draws ${gasket}` : life.rule_name(rule);
    }
    return live;
  };
  const tell = () => !gone && opts.onStat?.({ gen, pop: population(types), fate: word, name, counts: flat ? [rule.birth, rule.survive] : null });
  const advance = (live) => {
    if (flat) {
      types = life.next_grid({ shape: [BOARD, BOARD], types }, rule.birth, rule.survive, mask, live.wrap ? 'Wrap' : 'Constant').types;
      for (let i = 0; i < types.length; i++) age[i] = types[i] ? Math.min(255, age[i] + 1) : 0;
    } else {
      types = life.step(types, rule, Boolean(live.wrap));
      ring.set(types, ((gen + 1) % ROWS) * LINE);
    }
    gen++;
    word = fate(seen, hash(types), population(types));
  };
  const reseed = (live) => {
    types = sow(live, view.rand, math);
    age = Uint8Array.from(types);
    if (ring) {
      ring.fill(0);
      ring.set(types, 0);
    }
    gen = 0;
    seen = [];
    word = fate(seen, hash(types), population(types));
    origin = view.t;
    since = -1;
    tell();
    whole();
  };
  const settle = (live) => {
    if (hands || !word) {
      since = -1;
      return;
    }
    if (since < 0) since = view.t;
    else if (view.t - since >= HOLD) reseed(live);
  };
  const budget = () => (flat ? Math.max(1, Math.min(32, Math.floor(OPS / (types.length * Math.max(1, cells))))) : BURST);
  const layout = () => {
    if (flat) return board(view, BOARD, BOARD, FIT);
    const cell = Math.max(1, Math.floor((view.w * FIT) / LINE));
    const rows = Math.max(1, Math.min(ROWS, Math.floor((view.h * FIT) / cell)));
    return { x: Math.floor((view.w - LINE * cell) / 2), y: Math.floor((view.h - rows * cell) / 2), w: LINE * cell, h: rows * cell, cell, rows };
  };
  const ground = (x, y, w, h) => {
    ctx.fillStyle = skin.dim;
    ctx.fillRect(x, y, w, h);
    ctx.fillStyle = skin.ink;
  };
  const flatPaint = () => {
    const { x, y, w, cell } = lay;
    ctx.clearRect(0, 0, view.w, view.h);
    ground(x, y, w, w);
    for (const [lit, alpha] of [[(a) => a === 1, 1], [(a) => a > 1, OLD]]) {
      ctx.globalAlpha = alpha;
      ctx.beginPath();
      for (let i = 0; i < types.length; i++) if (lit(age[i])) ctx.rect(x + (i % width) * cell, y + Math.floor(i / width) * cell, cell, cell);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  };
  const first = () => Math.max(0, gen - lay.rows + 1);
  const band = (g) => lay.y + (g - first()) * lay.cell;
  const row = (g) => {
    const at = (g % ROWS) * LINE;
    const y = band(g);
    ctx.beginPath();
    for (let i = 0; i < LINE; i++) if (ring[at + i]) ctx.rect(lay.x + i * lay.cell, y, lay.cell, lay.cell);
    ctx.fill();
  };
  const sheet = () => {
    ctx.clearRect(0, 0, view.w, view.h);
    ground(lay.x, lay.y, lay.w, lay.h);
    for (let g = first(); g <= gen; g++) row(g);
  };
  const more = () => {
    const { x, y, w, h, cell, rows } = lay;
    const k = gen - shown;
    if (gen < rows) {
      for (let g = shown + 1; g <= gen; g++) row(g);
      return;
    }
    if (k >= rows || Math.max(0, shown - rows + 1) !== first() - k) {
      sheet();
      return;
    }
    const keep = (rows - k) * cell;
    ctx.drawImage(canvas, x, y + k * cell, w, keep, x, y, w, keep);
    ctx.clearRect(x, y + keep, w, h - keep);
    ground(x, y + keep, w, h - keep);
    for (let g = shown + 1; g <= gen; g++) row(g);
  };
  const paint = (full) => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    if (flat) flatPaint();
    else if (full || shown < 0 || gen < shown) sheet();
    else if (gen === shown) {
      ctx.clearRect(lay.x, band(gen), lay.w, lay.cell);
      ground(lay.x, band(gen), lay.w, lay.cell);
      row(gen);
    } else {
      ctx.fillStyle = skin.ink;
      more();
    }
    shown = gen;
    dirty = false;
  };
  const whole = () => {
    lay = layout();
    paint(true);
  };
  const theme = () => {
    dress();
    whole();
  };
  const draw = () => {
    const live = read();
    if (live.speed !== speed) {
      speed = live.speed;
      origin = view.t - (gen * 1000) / speed;
    }
    const target = Math.floor(((view.t - origin) * speed) / 1000 + EPS);
    const need = target - gen;
    if (need > 0) {
      const n = Math.min(need, budget());
      for (let i = 0; i < n; i++) advance(live);
      if (need > n) origin = view.t - (gen * 1000) / speed;
      dirty = true;
      tell();
    }
    settle(live);
    if (dirty) paint(false);
  };
  const step = () => {
    advance(read());
    if (speed) origin -= 1000 / speed;
    tell();
    paint(false);
  };
  const clear = () => {
    read();
    hands = true;
    types = new Uint8Array(types.length);
    age = new Uint8Array(types.length);
    if (ring) ring.fill(0, 0, LINE);
    gen = 0;
    seen = [];
    word = fate(seen, hash(types), 0);
    origin = view.t;
    tell();
    whole();
  };
  const touch = (u, v, start) => {
    const { x, y, cell } = lay;
    const col = Math.floor((u * view.w - x) / cell);
    const line = flat ? Math.floor((v * view.h - y) / cell) : 0;
    if (col < 0 || col >= width || line < 0 || line >= (flat ? BOARD : 1)) return;
    if (!flat && Math.abs(v * view.h - band(gen) - cell / 2) > Math.max(cell, THUMB * view.dpr)) return;
    const i = line * width + col;
    hands = true;
    if (start) brush = types[i] ? 0 : 1;
    if (types[i] === brush) return;
    types[i] = brush;
    age[i] = brush;
    if (ring) ring[(gen % ROWS) * LINE + i] = brush;
    seen = [];
    word = fate(seen, hash(types), population(types));
    tell();
    paint(false);
  };
  const stop = () => {
    gone = true;
  };
  dress();
  speed = read().speed;
  if (view.still) {
    lay = layout();
    const live = now();
    const n = flat ? Math.min(STILL, budget()) : lay.rows - 1;
    for (let i = 0; i < n; i++) advance(live);
  }
  whole();
  tell();
  return { draw, size: whole, theme, step, clear, touch, stop };
}
