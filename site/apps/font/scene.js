import { text as file } from '../../lib/export.js';
import { tidy } from '../../lib/knobs.js';
import { TINTS, board, rng, veil } from '../../lib/scene.js';
import { grid, rects } from '../../lib/svg.js';
import { GAP, SLOT, cells, common, cut, gallery, index, pen, plan, sample, shift, slug, written } from './engine.js';

export const VIEWS = [['pens', 'Pens'], ['text', 'Text']];

const TEXT = (value) => value.view === 'text';

const PENS = (value) => !TEXT(value);

export const named = (value) => (TEXT(value) ? `font-${slug(value.text || sample(rng(value.seed)))}` : 'font-pens');

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'pens', options: VIEWS, group: 'View' },
  { key: 'text', label: 'Text', kind: 'text', def: '', wide: true, group: 'View', when: TEXT },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.25, max: 4, step: 0.25, unit: 'x', group: 'Motion' },
  { key: 'hold', label: 'Hold', kind: 'slider', def: 25, min: 0, max: 100, step: 1, unit: 'frames', group: 'Motion' },
  { key: 'loop', label: 'Loop', kind: 'toggle', def: 1, group: 'Motion' },
  { key: 'flags', label: 'Flags', kind: 'toggle', def: 1, group: 'Look', when: PENS },
  { key: 'pad', label: 'Pad', kind: 'slider', def: 1, min: 0, max: 4, step: 1, unit: 'cells', group: 'Look', when: TEXT },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 40, step: 2, unit: 'px', zero: 'Fit', group: 'Look', when: TEXT },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const ABOUT = [
  'Every glyph of MrlyFont writes itself in the order its pen table gives, one cell a frame at 25 a second, then holds and writes again.',
  'Each pen has a stroke count and a floor, the fewest strokes that can write it. A stroke walks 4-adjacent cells; a lift is any step that is not.',
  'A plate marks a flagged pen: written over its floor on purpose, like the wordmark letters, or lifting five times or more, which only a new shape can help.',
  'Text writes your own words cell by cell, then folds them into one stack and back. A tap rewrites a pen or the text; a click in the gallery adds a glyph.',
];

export const PAGE = {
  spec: SPEC,
  name: named,
  about: ABOUT,
  record: true,
  keys: [
    { key: 'Enter', label: 'Write', act: 'write', button: true },
    { key: 'j', label: 'JSON', act: 'json' },
    { key: 's', label: 'SVG', act: 'svg' },
  ],
  actions: {
    write: (scene) => scene.write(),
    json: (scene) => file(JSON.stringify(scene.json()), scene.name(), 'json'),
    svg: (scene) => file(scene.svg(), scene.name(), 'svg'),
  },
};

export const units = () => import('./unit.js');

const FIT = 0.92;

const PITCH = SLOT + GAP;

const PLATE = 0.14;

export function layout(view, { rows, cols }, cell) {
  const fit = board(view, Math.max(1, cols), Math.max(1, rows), FIT);
  if (!cell) return fit;
  const side = Math.min(fit.cell, Math.max(1, Math.round(cell * view.dpr)));
  const w = side * cols;
  const h = side * rows;
  return { x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h, cell: side };
}

export function tile(view, n) {
  let best = null;
  for (let across = 1; across <= n; across++) {
    const down = Math.ceil(n / across);
    const cell = Math.max(1, Math.floor(Math.min((view.w * FIT) / (across * PITCH - GAP), (view.h * FIT) / (down * PITCH - GAP))));
    const spare = across * down - n;
    if (!best || cell > best.cell || (cell === best.cell && spare < best.spare)) best = { across, down, cell, spare };
  }
  const w = (best.across * PITCH - GAP) * best.cell;
  const h = (best.down * PITCH - GAP) * best.cell;
  return { ...best, n, x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h };
}

export function place(box, i) {
  const row = Math.floor(i / box.across);
  const last = row === box.down - 1 ? box.n - row * box.across : box.across;
  return [(i % box.across) * PITCH + Math.floor(((box.across - last) * PITCH) / 2), row * PITCH];
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const { font, onPlan } = opts;
  const pens = (opts.kinds ?? gallery(font)).flatMap((kind) => kind.glyphs);
  const starts = pens.map(() => 0);
  let value = { ...tidy(SPEC, opts), seed: opts.seed };
  let said = sample(view.rand);
  let made = written(font, value.text || said, value.pad);
  let show = plan(font, value, made);
  let start = 0;
  let base = 0;
  let from = view.t;
  let box = null;
  let shown = null;
  let last = 0;
  const phase = () => base + ((view.t - from) * show.fps * value.speed) / 1000;
  const tick = () => Math.floor(phase() + 1e-9);
  const now = () => (view.still ? 0 : tick());
  const fit = () => {
    box = TEXT(value) ? layout(view, show, value.cell) : tile(view, pens.length);
  };
  const told = () => onPlan?.({ text: show.text, rows: show.rows, cols: show.cols, frames: show.frames.length, fps: show.fps });
  const build = (words) => {
    if (made.text !== words || made.pad !== value.pad) made = written(font, words, value.pad);
    show = plan(font, value, made);
  };
  const frame = (k) => (view.still ? show.still : index(k - start, show.frames.length, value.loop));
  const at = (k, i) => {
    const n = pens[i].frames.length;
    return view.still ? n - 1 : pen(k - starts[i], n, value.hold, value.loop);
  };
  const key = (k) => {
    if (TEXT(value)) return frame(k);
    if (value.loop || view.still) return k;
    return Math.min(k, Math.max(...starts) + Math.max(...pens.map((one) => one.frames.length)));
  };
  const lit = (k) => {
    const out = [];
    pens.forEach((one, i) => {
      const [cx, cy] = place(box, i);
      const dx = cx + Math.floor((SLOT - one.cols) / 2);
      const dy = cy + Math.floor((SLOT - one.rows) / 2);
      for (const c of one.frames[at(k, i)]) out.push([dx + (c % one.cols), dy + Math.floor(c / one.cols)]);
    });
    return out;
  };
  const plates = (cell) => {
    const pad = Math.round(cell / 2);
    const side = SLOT * cell + 2 * pad;
    ctx.beginPath();
    pens.forEach((one, i) => {
      if (!one.flag) return;
      const [cx, cy] = place(box, i);
      const x = box.x + cx * cell - pad;
      const y = box.y + cy * cell - pad;
      if (ctx.roundRect) ctx.roundRect(x, y, side, side, cell);
      else ctx.rect(x, y, side, side);
    });
    ctx.fill();
  };
  const paint = (k) => {
    const { accent } = view.look();
    const { x, y, cell } = box;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (PENS(value) && value.flags) {
      ctx.fillStyle = veil(accent, PLATE);
      plates(cell);
    }
    ctx.fillStyle = accent;
    ctx.beginPath();
    if (TEXT(value)) for (const i of show.frames[frame(k)]) ctx.rect(x + (i % show.cols) * cell, y + Math.floor(i / show.cols) * cell, cell, cell);
    else for (const [cx, cy] of lit(k)) ctx.rect(x + cx * cell, y + cy * cell, cell, cell);
    ctx.fill();
    shown = key(k);
    last = k;
  };
  const draw = () => {
    const k = now();
    if (key(k) !== shown) paint(k);
  };
  const size = () => {
    fit();
    paint(now());
  };
  const theme = () => paint(now());
  const rewrite = (was, k, words) => {
    const old = show;
    const j = index(k - start, old.frames.length, was.loop);
    build(words);
    if (show.text !== old.text) {
      const kept = cells(font, common(old.text, show.text), value.pad);
      return k - (j > old.still ? kept : Math.min(j, kept));
    }
    if (value.loop !== was.loop) return k - Math.min(j, show.still);
    if (value.hold !== was.hold) return k - shift(j, old.moves, old.hold, show.hold);
    return k - Math.min(j, show.frames.length - 1);
  };
  const regrip = (was, k) => {
    pens.forEach((one, i) => {
      const n = one.frames.length;
      const d = k - starts[i];
      const c = was.loop ? index(d, n + was.hold) : Math.min(Math.max(0, d), n - 1);
      starts[i] = k - Math.min(c, (value.loop ? n + value.hold : n) - 1);
    });
  };
  const shuffle = (k) => {
    const most = Math.max(...pens.map((one) => one.frames.length));
    pens.forEach((_, i) => {
      starts[i] = k + Math.floor(view.rand() * most);
    });
  };
  const set = (next) => {
    const was = value;
    const p = phase();
    value = { ...tidy(SPEC, next), seed: next.seed };
    if (value.speed !== was.speed) {
      base = p;
      from = view.t;
    }
    const reseeded = value.seed !== was.seed;
    if (reseeded) said = sample(view.rand);
    const k = tick();
    const words = cut(value.text || said);
    if (value.view !== was.view) {
      build(words);
      start = k;
      starts.fill(k);
    } else if (TEXT(value) && (words !== show.text || value.pad !== was.pad || value.hold !== was.hold || value.loop !== was.loop)) start = rewrite(was, k, words);
    else if (PENS(value) && reseeded) shuffle(k);
    else if (PENS(value) && (value.hold !== was.hold || value.loop !== was.loop)) regrip(was, k);
    fit();
    shown = null;
    told();
  };
  const tap = (px, py) => {
    if (view.still) return -1;
    const k = tick();
    let hit = 0;
    if (TEXT(value)) start = k;
    else {
      hit = pens.findIndex((_, i) => {
        const [cx, cy] = place(box, i);
        const u = (px - box.x) / box.cell - cx + GAP / 2;
        const v = (py - box.y) / box.cell - cy + GAP / 2;
        return u >= 0 && u < PITCH && v >= 0 && v < PITCH;
      });
      if (hit >= 0) starts[hit] = k;
    }
    shown = null;
    draw();
    return hit;
  };
  const write = () => {
    const k = tick();
    start = k;
    starts.fill(k);
    shown = null;
    draw();
  };
  const json = () => {
    const fps = show.fps * value.speed;
    if (TEXT(value)) return { text: show.text, pad: value.pad, hold: value.hold, loop: value.loop, rows: show.rows, cols: show.cols, fps, frames: show.frames };
    return { hold: value.hold, loop: value.loop, fps, pens: pens.map(({ char, name, rows, cols, strokes, floor, lifts, flag, frames }) => ({ char, name, rows, cols, strokes, floor, lifts, flag, frames })) };
  };
  const svg = () => {
    const k = shown === null ? now() : last;
    const fill = view.look().accent;
    const unit = box.cell / view.dpr;
    if (TEXT(value)) return rects(grid(show.rows, show.cols, show.frames[frame(k)]), { fill, unit });
    const across = box.across * PITCH - GAP;
    return rects(grid(box.down * PITCH - GAP, across, lit(k).map(([cx, cy]) => cy * across + cx)), { fill, unit });
  };
  const name = () => named(value);
  fit();
  told();
  return { draw, size, theme, set, tap, write, json, svg, name, box: () => box, plan: () => show };
}
