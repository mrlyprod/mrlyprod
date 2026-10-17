import { tidy } from '../../lib/app/spec.js';
import { TINTS, veil } from '../../lib/color.js';
import { rects } from '../../lib/app/svg.js';
import { DESIGN } from '../../lib/designs/engine.js';
import { cap, count, design, joins, lay, layers, memo, sheet, shot, stamp, stroke } from '../../lib/two/engine.js';
import { second } from '../../lib/two/scene.js';
import { wasm } from '../../lib/units/index.js';

export const PRESETS = [[5, 5], [3, 9]];

const never = () => false;

export const SPEC = [
  { key: 'number', label: 'Side', kind: 'segment', def: 3, options: [[3, '3'], [5, '5'], [7, '7']], group: 'Design', when: never },
  { key: 'level', label: 'Level', kind: 'slider', def: 2, min: 1, max: (value, units) => units && cap(units.math, value.number, value.x * value.y), step: 1, keep: true, group: 'Design', when: never },
  { key: 'x', label: 'Across', kind: 'slider', def: 5, min: 1, max: 12, step: 1, group: 'Sheet', when: never },
  { key: 'y', label: 'Down', kind: 'slider', def: 5, min: 1, max: 12, step: 1, group: 'Sheet', when: never },
  { key: 'seams', label: 'Seams', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const name = (value) => `two-${String(value.code ?? '').trim() || value.seed}-${value.x}x${value.y}`;

export const units = wasm('math');

export const PAGE = {
  spec: SPEC,
  units,
  about: [
    'One design at its level is the tile, and the sheet repeats it across and down.',
    'The fills multiply by the copy count exactly. The exposed edges do not: every copy buries the edges it shares with a neighbour.',
    'Seams draws where the copies meet in a second ink: a filled cell on both sides of a seam buries two faces.',
    'Why the symmetries of a design belong to its infinite tiling is in [the core](/research/core/). Nearby: [Crop](/crop/) cuts one design to a shape, [Slices](/slices/) counts one hexagon, [Three](/three/) grows the cube.',
  ],
  name,
  keys: [
    { key: 'Enter', label: 'Seams', act: 'seams', button: true, when: (value) => !value.seams },
    { key: 'Enter', label: 'Hide seams', act: 'seams', button: true, when: (value) => Boolean(value.seams) },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Across', act: 'across' },
  ],
  actions: {
    seams: (scene) => scene.seams?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    across: (scene, e) => scene.wider?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

const FIT = 0.9;

const UNIT = 10;

const FAINT = 0.16;

const SEAM = 0.3;

/* PLAN */

export function plan(math, next, rand, stages = {}) {
  const at = (name) => (stages[name] ??= memo());
  const value = { ...tidy(DESIGN, next), ...tidy(SPEC, next, { math }) };
  const code = at('code')(`${value.base}:${value.code}:${next.seed}`, () => design(math, value.base, value.code, rand));
  const kept = [code, value.base, value.number, value.level].join();
  const tile = at('tile')(kept, () => math.two.create(code, value.number, value.level, 0, value.base));
  const whole = `${kept}|${value.x}|${value.y}`;
  const block = at('block')(whole, () => sheet(math, tile, value.x, value.y));
  const lit = `${whole}|${value.invert}`;
  const look = at('look')(lit, () => layers(math, block, value.invert));
  const one = at('one')(`${kept}|${value.invert}`, () => (value.invert ? math.cell.models.invert(tile) : tile));
  const lines = value.seams ? joins(block.shape, tile.shape[0]) : [];
  const read = { ...value, code };
  const facts = () => at('facts')(lit, () => count(math, read, one, look.drawn));
  return { value: read, tile, block, look, lines, facts };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('two: the scene wants @mrlyprod/mrly/math as opts.math');
  const ctx = canvas.getContext('2d');
  const stages = {};
  let now = plan(math, opts, view.rand, stages);
  let dirty = true;
  const paint = (pen, box, w, h) => {
    const { paper, accent } = view.look();
    pen.fillStyle = paper;
    pen.fillRect(0, 0, w, h);
    stamp(pen, now.look.faint, box, veil(accent, FAINT));
    stamp(pen, now.look.lit, box, accent);
    if (now.lines.length) stroke(pen, now.lines, box, second(canvas, now.value.tint, accent), Math.max(1, Math.round(box.px * SEAM)));
  };
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    const [rows, cols] = now.block.shape;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    paint(ctx, lay(cols, rows, view.w, view.h, FIT), view.w, view.h);
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const set = (next) => {
    now = plan(math, next, view.rand, stages);
    touch();
  };
  const svg = () => rects(now.look.drawn, { fill: view.look().accent, unit: UNIT });
  const picture = (size, kind = 'png') => {
    const [rows, cols] = now.block.shape;
    return shot(cols, rows, size, paint, kind);
  };
  return { still: true, draw, set, size: touch, theme: touch, svg, shot: picture, facts: () => now.facts() };
}
