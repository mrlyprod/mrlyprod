import { palette } from '../../lib/theme/palette.gen.js';
import { dark, light } from '../../lib/theme/theme.js';
import { tidy } from '../../lib/app/spec.js';
import { rgb } from '../../lib/color.js';
import { DESIGN } from '../../lib/designs/engine.js';
import { CELLS, COLOURS, DESIGNED, DIM, FOLDS, GRIDS, LEVELS, LIFTS, PAD, PAINTS, READS, ROUNDS, VIEWS, cap, coded, fits, lay, picture, pictureOf, sideOf, sides, study, tower } from './engine.js';
import { wasm } from '../../lib/units/index.js';

const never = () => false;

const word = (value) => value.view === 'word';

const reads = (kind) => (value) => pictureOf(value) === kind;

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'plane', options: VIEWS, group: 'View' },
  { key: 'read', label: 'Read', kind: 'segment', def: 'power', options: READS, group: 'View', when: never },
  { key: 'round', label: 'Rounds', kind: 'slider', def: 6, min: 1, max: ROUNDS, step: 1, show: (v) => `${v} · ${2 ** v} letters`, group: 'View', when: word },
  { key: 'colour', label: 'Colour', kind: 'segment', def: 'letters', options: COLOURS, group: 'View', when: never },
  { key: 'lift', label: 'Lift', kind: 'pick', def: 'Parity', options: LIFTS, group: 'View', when: never },
  { key: 'level', label: 'Level', kind: 'slider', def: 6, min: 1, max: LEVELS, step: 1, show: (v) => `${v} · side ${2 ** v}`, group: 'View', when: reads('plane') },
  { key: 'fold', label: 'Fold', kind: 'pick', def: 'sign', options: FOLDS, group: 'View', when: reads('filter') },
  { key: 'number', label: 'Tile side', kind: 'pick', def: 0, options: sides, group: 'View', when: (value) => DESIGNED.includes(pictureOf(value)) },
  { key: 'power', label: 'Level', kind: 'slider', def: 5, min: 1, max: (value) => cap(sideOf(value)), step: 1, show: (v, value) => `${v} · side ${sideOf(value) ** v}`, group: 'View', when: reads('power') },
  { key: 'depth', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(sideOf(value)) - 1, step: 1, show: (v, value) => `${v} · side ${sideOf(value) ** (v + 1)}`, group: 'View', when: reads('filter') },
  { key: 'cell', label: 'Cell', kind: 'pick', def: 0, options: CELLS, group: 'View', when: (value) => GRIDS.includes(pictureOf(value)) },
];

export const tidied = (value) => ({ ...value, ...tidy(DESIGN, value), ...tidy(SPEC, value) });

export const units = wasm('math', 'num');

export function file(value, given) {
  const one = tidied(value);
  const kind = pictureOf(one);
  const code = DESIGNED.includes(kind) ? coded(given.math, one) : '';
  const parts = { word: ['word', one.round, ...(one.colour === 'runs' ? ['runs'] : [])], plane: [one.lift.toLowerCase(), one.level], power: [code, 'power', one.power], filter: [code, 'filter', one.depth] }[kind];
  return ['morse', ...parts].join('-');
}

export const PAGE = {
  defaults: { base: 2, code: '9' },
  spec: SPEC,
  units,
  name: file,
  design: { dim: DIM, base: 'base', code: 'code', when: (value) => value.view === 'design' },
  about: [
    'The Thue-Morse word is one digit rule built twice. Substitution grows it from a single 0, every 0 becoming 01 and every 1 becoming 10; bit parity reads letter n as t(n), the parity of the one-bits of n, and the two agree letter for letter. Word stacks the rounds, orange for letter 0 (plus one) and blue for letter 1 (minus one), and Play steps them.',
    'Coloured by runs, yellow for a run of one and pink for a run of two, the word shows why no letter comes three times in a row: t(2n) = t(n) and t(2n+1) = 1 - t(n), so it changes at every even place. The places where it changes spell the period-doubling word. A rule that changes with the scale is a word, and its grammar is in [the magic words note](/research/magic/).',
    'Lifts puts the word on the plane four ways. Parity, t(i) xor t(j), is the Kronecker power of one two-by-two tile, and that tile is a design read plus-minus. And is the Walsh-Hadamard pattern, a power too. Xor is the Parity grid again, since t carries xor to xor. Sum carries, and carrying is no digit rule, so it never folds past side two. A tap moves to the next lift.',
    'Design reads any design plus-minus. Sign power draws the power of its own plus-minus tile. Filter blows a level up and draws its difference with the next. In the plus-minus fold what is left is the tile repeated, not Thue-Morse: at tile side two it is off Thue-Morse at exactly half the sites, the score of a coin. [Moire](/moire/) stacks one design over its scales, and [Sequences](/sequences/) holds the ledger the designs write.',
  ],
  keys: [
    { key: 'Enter', label: (value, handle) => (handle?.playing?.() ? 'Stop' : 'Play'), act: 'play', button: true, menu: 'Edit', when: word },
    { key: 'ArrowUp', label: 'Level', act: 'up', menu: 'View' },
    { key: 'ArrowDown', label: 'Level', act: 'down', menu: 'View' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'View', act: 'view' },
  ],
  actions: {
    play: (scene) => scene.toggle?.(),
    up: (scene) => scene.deeper?.(1),
    down: (scene) => scene.deeper?.(-1),
    view: (scene, e) => scene.turn?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

export function inks(canvas) {
  const css = typeof getComputedStyle === 'function' && canvas ? getComputedStyle(canvas) : null;
  const read = (name, fallback) => (css && css.getPropertyValue(name).trim()) || fallback;
  const base = typeof matchMedia === 'function' && matchMedia('(prefers-color-scheme: dark)').matches ? dark : light;
  return { plus: palette.orange, minus: palette.blue, one: palette.yellow, two: palette.pink, diff: palette.pink, unlit: read('--panel', base.panel) };
}

const EDGE = 0.06;

export function bars(ctx, rows, box, colours) {
  rows.forEach((row, k) => {
    const at = box[k];
    colours.forEach((colour, v) => {
      ctx.fillStyle = colour;
      for (let i = 0; i < row.length; i++) {
        if (row[i] !== v) continue;
        const x = Math.round(at.x + i * at.step);
        ctx.fillRect(x, at.y, Math.max(1, Math.round(at.x + (i + 1) * at.step) - x - at.gap), at.h);
      }
    });
  });
}

export function sketch(ctx, w, h, dpr, plan, ink) {
  const pad = Math.round(w * EDGE);
  if (!plan.types) return bars(ctx, plan.rows, tower(w, h, plan.rows, dpr, pad), PAINTS[plan.paint].map((key) => ink[key]));
  const box = lay(w, h, plan.side, 0, pad);
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(stamp(plan.types, plan.side, plan.side, PAINTS[plan.paint].map((key) => rgb(ink[key]))), 0, 0, plan.side, plan.side, box.x, box.y, box.size, box.size);
}

export function stamp(types, w, h, colours) {
  const page = new OffscreenCanvas(w, h);
  const pen = page.getContext('2d');
  const image = pen.createImageData(w, h);
  const data = image.data;
  for (let i = 0; i < w * h; i++) {
    const colour = colours[types[i]];
    if (!colour) continue;
    data[i * 4] = colour[0];
    data[i * 4 + 1] = colour[1];
    data[i * 4 + 2] = colour[2];
    data[i * 4 + 3] = 255;
  }
  pen.putImageData(image, 0, 0);
  return page;
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { num, math } = opts;
  if (!num || !math) throw new Error('morse: the scene wants @mrlyprod/mrly/num and @mrlyprod/mrly/math as opts.num and opts.math');
  const units = { num, math };
  const ctx = canvas.getContext('2d');
  let value = tidied(opts);
  let plan = study(units, value);
  let ink = null;
  let source = null;
  let box = null;
  let dirty = true;
  const paint = () => {
    ink = inks(canvas);
    source = plan.types ? stamp(plan.types, plan.side, plan.side, PAINTS[plan.paint].map((key) => rgb(ink[key]))) : null;
  };
  const fit = () => {
    const { w, h, dpr } = view;
    box = plan.rows.length ? tower(w, h, plan.rows, dpr) : lay(w, h, plan.side, value.cell * dpr, PAD * dpr);
  };
  const letters = () => bars(ctx, plan.rows, box, PAINTS[plan.paint].map((key) => ink[key]));
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    if (!box) fit();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = false;
    if (source) ctx.drawImage(source, 0, 0, plan.side, plan.side, box.x, box.y, box.size, box.size);
    else letters();
  };
  const size = () => {
    box = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    dirty = true;
  };
  const set = (next) => {
    if (word(next) !== word(value)) return false;
    value = tidied(next);
    const was = plan;
    plan = study(units, value);
    if (plan !== was) paint();
    box = null;
    dirty = true;
  };
  const svg = () => (fits(plan) ? picture(plan, ink) : null);
  paint();
  return { draw, size, theme, set, svg, still: !word(value), facts: () => plan.facts };
}
