import { tidy } from '../../lib/knobs.js';
import { board, rng, veil } from '../../lib/scene.js';
import { BOARD, LIMIT, LINE, ROWS, SPANS, STARTS, destiny, dimOf, fate, hash, lattice, maskName, maskOf, population, ruleOf, sow, told } from './engine.js';

export const PANE = () => false;

const NUMBERS = [3, 5, 7, 9].map((n) => [n, String(n)]);
const LEVELS = [1, 2, 3];
const LINES = ['noise', 'one'];
export const SOUP = (value) => value.from === 'noise';

export const SPEC = [
  { key: 'dim', label: 'Dim', kind: 'segment', def: 2, options: [[1, '1D'], [2, '2D']], group: 'Mask', when: PANE },
  { key: 'code', label: 'Code', kind: 'text', def: '7', group: 'Mask', when: PANE },
  { key: 'number', label: 'Side', kind: 'pick', def: 3, options: NUMBERS, group: 'Mask', when: PANE },
  { key: 'level', label: 'Level', kind: 'pick', def: 1, options: (value) => LEVELS.filter((n) => value.number ** n <= SPANS[dimOf(value)]).map((n) => [n, String(n)]), group: 'Mask', when: PANE },
  { key: 'born', label: 'Born', kind: 'text', def: '3', group: 'Rule', when: PANE },
  { key: 'stay', label: 'Stay', kind: 'text', def: '23', group: 'Rule', when: PANE },
  { key: 'wrap', label: 'Wrap', kind: 'toggle', def: 1, group: 'Rule', when: PANE },
  { key: 'from', label: 'Start', kind: 'pick', def: 'noise', options: (value) => (dimOf(value) === 1 ? STARTS.filter(([one]) => LINES.includes(one)) : STARTS), group: 'Board', when: PANE },
  { key: 'density', label: 'Density', kind: 'slider', def: 0.3, min: 0.05, max: 0.95, step: 0.01, group: 'Board', when: PANE },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 25, min: 1, max: 60, step: 1, unit: '/s', group: 'Board', when: PANE },
];

export const PRESETS = [
  ['Conway', { dim: 2, code: '7', number: 3, level: 1, born: '3', stay: '23' }],
  ['Cantor-Life', { dim: 1, code: '1', number: 3, level: 3, born: '3', stay: '23' }],
  ['Menger row', { dim: 1, code: '3', number: 3, level: 2, born: '3', stay: '23' }],
  ['Nine-cell XOR', { dim: 2, code: '15', number: 3, level: 1, born: '1357', stay: '02468' }],
  ['Rule 150', { dim: 1, code: '1', number: 3, level: 1, born: '1', stay: '02' }],
];

export const wears = (value, preset) => Object.entries(preset).every(([key, v]) => String(value[key]) === String(v));

const slug = (text) => String(text).toLowerCase().replace(/[^a-z0-9.]+/g, '-').replace(/^-+|-+$/g, '');

export const worn = (presets, value) => presets.find(([, one]) => wears(value, one))?.[0] ?? '';

export const PAGE = {
  spec: SPEC,
  gestures: true,
  record: true,
  about: [
    "Conway's Life is one rule on the eight cells around a cell: a dead cell with three live neighbours is born, a live cell with two or three stays, and every other cell dies.",
    'Here the neighbourhood is a design. The carpet at side 3, level 1 is the Moore ring; any other code, side and level is another life. Born and Stay count live mask cells, as digits or as a named sequence.',
    'Paint cells by hand, then press Play. Run to fate replays the board for up to 512 generations and says whether it dies, freezes, loops, or is still moving.',
    'In 1D the board is one line of 320 cells, drawn top to bottom with the newest row last. Wrap joins the edges. A side-3 line at level 1 runs elementary rules, as on [Wolfram](/wolfram/); the whole family is in [the automata note](/research/automata/).',
  ],
  name: (value) => ['life', slug(worn(PRESETS, value) || `${dimOf(value)}d ${value.code} ${value.number} ${value.level} b${value.born} s${value.stay}`), SOUP(value) ? value.seed : slug(value.from)].join('-'),
  keys: [
    { key: 'Enter', label: (value, scene) => (scene?.playing?.() ? 'Pause' : 'Play'), act: 'play', button: true },
    { key: 'n', label: 'Step', act: 'step' },
    { key: 'f', label: 'Run to fate', act: 'fate' },
    { key: ['0', 'Home'], label: 'Reset', act: 'reset' },
    { key: 'c', label: 'Clear', act: 'clear' },
  ],
  actions: { play: (scene) => scene.toggle(), step: (scene) => scene.step(), fate: (scene) => scene.doom(), reset: (scene) => scene.reset(), clear: (scene) => scene.clear() },
};

export const units = () => import('./unit.js');

export const HOLD = 3000;

const FIT = 0.96;
const GROUND = 0.07;
const OLD = 0.55;
const OPS = 4e6;
const STILL = 24;
const THUMB = 24;
const EPS = 1e-6;

/* PAINT */

export function runs(ctx, types, cols, rows, x, y, cell, high = cell) {
  const snap = cell >= 1 ? Math.round : (n) => n;
  ctx.beginPath();
  for (let r = 0; r < rows; r++) {
    const top = snap(y + r * high);
    const tall = snap(y + (r + 1) * high) - top;
    for (let c = 0; c < cols; c++) {
      if (!types[r * cols + c]) continue;
      const from = c;
      while (c + 1 < cols && types[r * cols + c + 1]) c++;
      const left = snap(x + from * cell);
      ctx.rect(left, top, snap(x + (c + 1) * cell) - left, tall);
    }
  }
  ctx.fill();
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const { life, math } = opts;
  let value = { ...tidy(SPEC, opts), seed: opts.seed };
  let shape = null;
  let rule = null;
  let name = '';
  let width = BOARD;
  let height = BOARD;
  let ring = null;
  let types = null;
  let age = null;
  let gen = 0;
  let seen = [];
  let word = '';
  let verdict = '';
  let error = '';
  let pop = 0;
  let playing = Boolean(view.lock);
  let origin = 0;
  let lay = null;
  let skin = null;
  let shown = -1;
  let dirty = true;
  let gone = false;
  let brush = 1;
  let hands = false;
  let since = -1;
  const linear = () => shape.dim === 1;
  const mould = () => {
    shape = maskOf(life, value);
    shape.named = maskName(math, shape.code, shape.dim);
    shape.label = `${shape.named || `code ${shape.code}`}, side ${shape.number}, level ${shape.level}`;
    width = linear() ? LINE : BOARD;
    height = linear() ? 1 : BOARD;
  };
  const judge = () => {
    rule = ruleOf(life, value, shape.cells);
    name = shape.named ? `${rule.name} on ${shape.named}` : rule.name;
  };
  const grid = () => ({ shape: [height, width], types });
  const begin = (next) => {
    types = next;
    age = Uint8Array.from(types);
    ring = linear() ? new Uint8Array(ROWS * LINE) : null;
    ring?.set(types, 0);
    gen = 0;
    seen = [];
    pop = population(types);
    word = fate(seen, hash(types), pop);
    verdict = '';
    origin = view.t;
    since = -1;
    shown = -1;
  };
  const sowed = (rand = rng(value.seed)) => begin(sow(value, rand, math, width, height));
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const tell = () => !gone && opts.onStat?.({ gen, pop, fate: verdict || word, name, counts: [rule.birth, rule.survive], mask: shape.mask, cells: shape.cells, lattice: lattice(shape.lattice), label: shape.label, error });
  const advance = () => {
    types = life.next_grid(grid(), rule.birth, rule.survive, shape.mask, value.wrap ? 'Wrap' : 'Constant').types;
    if (ring) ring.set(types, ((gen + 1) % ROWS) * LINE);
    else for (let i = 0; i < types.length; i++) age[i] = types[i] ? Math.min(255, age[i] + 1) : 0;
    gen++;
    pop = population(types);
    word = fate(seen, hash(types), pop);
  };
  const settle = () => {
    if (hands || !word) {
      since = -1;
      return;
    }
    if (since < 0) since = view.t;
    else if (view.t - since >= HOLD) {
      begin(sow(value, view.rand, math, width, height));
      tell();
      whole();
    }
  };
  const budget = () => Math.max(1, Math.min(32, Math.floor(OPS / (types.length * Math.max(1, shape.cells)))));
  const layout = () => {
    if (!linear()) return board(view, BOARD, BOARD, FIT);
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
    const { x, y, cell } = lay;
    ctx.clearRect(0, 0, view.w, view.h);
    ground(x, y, BOARD * cell, BOARD * cell);
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
  const row = (g) => runs(ctx, ring.subarray((g % ROWS) * LINE, (g % ROWS) * LINE + LINE), LINE, 1, lay.x, band(g), lay.cell);
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
    if (!linear()) flatPaint();
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
    if (view.fixed && !playing) toggle();
    if (playing) {
      const speed = value.speed;
      const need = Math.floor(((view.t - origin) * speed) / 1000 + EPS) - gen;
      if (need > 0) {
        const n = Math.min(need, budget());
        for (let i = 0; i < n; i++) advance();
        if (need > n) origin = view.t - (gen * 1000) / speed;
        dirty = true;
        tell();
      }
    }
    if (view.lock) settle();
    if (dirty) paint(false);
  };
  const toggle = () => {
    playing = !playing;
    if (playing) {
      origin = view.t - (gen * 1000) / value.speed;
      view.wake?.();
    }
    tell();
  };
  const step = () => {
    advance();
    origin -= 1000 / value.speed;
    tell();
    paint(false);
  };
  const doom = () => {
    try {
      verdict = told(destiny(life, grid(), shape.mask, rule, value.wrap, LIMIT));
      error = '';
    } catch (thrown) {
      error = String(thrown?.message ?? thrown);
    }
    tell();
  };
  const reset = () => {
    playing = false;
    sowed();
    tell();
    whole();
  };
  const clear = () => {
    hands = true;
    types = new Uint8Array(types.length);
    age = new Uint8Array(types.length);
    ring?.fill(0);
    gen = 0;
    seen = [];
    pop = 0;
    word = fate(seen, hash(types), 0);
    verdict = '';
    origin = view.t;
    tell();
    whole();
  };
  const touch = (u, v, start) => {
    const { x, y, cell } = lay;
    const col = Math.floor((u * view.w - x) / cell);
    const line = linear() ? 0 : Math.floor((v * view.h - y) / cell);
    if (col < 0 || col >= width || line < 0 || line >= height) return;
    if (linear() && Math.abs(v * view.h - band(gen) - cell / 2) > Math.max(cell, THUMB * view.dpr)) return;
    const i = line * width + col;
    hands = true;
    if (start) brush = types[i] ? 0 : 1;
    if (types[i] === brush) return;
    types[i] = brush;
    age[i] = brush;
    if (ring) ring[(gen % ROWS) * LINE + i] = brush;
    seen = [];
    pop = population(types);
    word = fate(seen, hash(types), pop);
    verdict = '';
    tell();
    paint(!linear());
  };
  const set = (next) => {
    const was = value;
    value = { ...tidy(SPEC, next), seed: next.seed };
    const moved = (...keys) => keys.some((key) => String(value[key]) !== String(was[key]));
    if (moved('dim', 'code', 'number', 'level')) mould();
    if (moved('dim', 'code', 'number', 'level', 'born', 'stay')) judge();
    if (moved('dim', 'from', 'density', 'seed') || (value.from === 'design' && moved('code', 'number'))) sowed();
    else if (moved('dim', 'code', 'number', 'level', 'born', 'stay', 'wrap')) verdict = '';
    if (moved('speed')) origin = view.t - (gen * 1000) / value.speed;
    tell();
    whole();
  };
  const stop = () => {
    gone = true;
  };
  dress();
  mould();
  judge();
  sowed(view.rand);
  if (view.still && view.lock) {
    lay = layout();
    const n = linear() ? lay.rows - 1 : Math.min(STILL, budget());
    for (let i = 0; i < n; i++) advance();
  }
  whole();
  tell();
  return { draw, size: whole, theme, set, toggle, playing: () => playing, step, doom, reset, clear, touch, stop };
}
