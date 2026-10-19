import { tidy } from '../../lib/app/spec.js';
import { TINTS, rgb } from '../../lib/color.js';
import { canvas as pen, frame as box, hex, svg as sheet } from '#view';
import { design, memo, named, tag as title } from '../../lib/three/engine.js';
import { wasm } from '../../lib/units/index.js';

export const TOP = 16;
export const TILES = [[0, 'Off'], [3, 'Base 3'], [5, 'Base 5']];
const REACH = 100;
const NET = '232';
const CARPET = '23';
const PAD = 0.04;
const FAINT = 18;

export const lid = (math, tile) => (tile ? math.bang.level_cap(tile, 1, REACH) : 1);

const tiled = (value) => Number(value.tile) !== 0;

export const SPEC = [
  { key: 'k', label: 'Side k', kind: 'slider', def: 2, min: 1, max: TOP, step: 1, group: 'Section', when: (value) => !tiled(value) },
  { key: 'tile', label: 'Fractal tile', kind: 'segment', def: 0, options: TILES, group: 'Section' },
  { key: 'level', label: 'Level', kind: 'slider', def: 1, min: 1, max: (value, units) => units && lid(units.math, Number(value.tile)), step: 1, group: 'Section', when: tiled },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const units = wasm('math');

export const name = (value) => `slices-${String(value.code ?? '').trim() || CARPET}-${tiled(value) ? `t${value.tile}-l${value.level}` : `k${value.k}`}`;

export const PAGE = {
  defaults: { base: 2, code: '23' },
  spec: SPEC,
  units,
  gestures: true,
  about: [
    'Cut the cube of odd side 2k - 1 through its centre, square to the main diagonal, and the section is a regular hexagon of 6 side^2 unit triangles.',
    'A design fills some of them: carpet and net split the hexagon between them with nothing left over, and the carpet falls into many pieces at odd k and closes into one piece pierced by holes at even k.',
    'Drag across the picture or press Left and Right to move the side. Every count is read off the section mesh, and the closed form beside the triangle count is evaluated in Rust. More in [the slices note](/research/slices/).',
  ],
  name,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Side', act: 'side' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level', when: tiled },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    side: (scene, e) => scene.step?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

/* STUDY */

const cuts = memo(6);
const splits = memo(8);

function cut(math, code, base, number, level) {
  return cuts(`${code} ${base} ${number} ${level}`, () => {
    const cell = math.three.create(code, number, level, base);
    const mesh = math.six.cut(cell);
    return { mesh, side: cell.shape[0], tally: math.six.census(mesh, false), pieces: math.six.components(mesh), holes: math.six.holes(mesh), giant: math.six.giant(mesh) };
  });
}

export function partition(math, number) {
  return splits(number, () => {
    const carpet = cut(math, CARPET, 2, number, 1).tally;
    const net = cut(math, NET, 2, number, 1).tally;
    return { carpet: carpet.fills, net: net.fills, together: carpet.fills + net.fills, hexagon: carpet.triangles, exact: carpet.fills + net.fills === carpet.triangles };
  });
}

export function study(math, value) {
  const { base, code } = design(math, value);
  const tile = Number(value.tile);
  const k = Math.min(TOP, Math.max(1, Math.round(value.k)));
  const number = tile || 2 * k - 1;
  const level = tile ? Math.min(value.level, lid(math, tile)) : 1;
  const one = cut(math, code, base, number, level);
  const { tally } = one;
  const facts = [
    ['Name', title(named(math, code, base), code, base)],
    ['Side', one.side],
    ['Triangles', tally.triangles],
    ['Closed form', String(math.counts.six.solid_slice_triangles(one.side))],
    ['Filled', tally.fills],
    ['Empty', tally.voids],
    ['Pieces', one.pieces],
    ['Holes', one.holes],
    ['Largest piece', one.giant],
    ['Boundary', tally.boundary_edges],
    ['Edges', tally.edges],
    ['Interior', tally.interior_edges],
    ['Vertices', tally.vertices],
    ['Euler', String(tally.euler)],
  ];
  if (level === 1) {
    const split = partition(math, number);
    facts.push(['Carpet', split.carpet], ['Net', split.net], ['Together', split.together], ['Hexagon', split.hexagon], ['Partition', split.exact ? 'exact' : 'broken']);
  }
  return { mesh: one.mesh, side: one.side, rows: facts };
}

/* PAINT */

export const layout = (w, h) => box(0, 0, w, h).inset(Math.min(w, h) * PAD);

export const scrub = (x, w) => Math.min(TOP, Math.max(1, Math.floor((x / Math.max(1, w)) * TOP) + 1));

export function picture(page, frame, plan, paint) {
  const fill = paint.accent;
  const faint = [...fill.slice(0, 3), FAINT];
  const codes = { [paint.fill]: fill, [paint.void]: faint };
  hex.draw(page, frame, plan.mesh, 0, (type) => codes[type] ?? null);
}

/* SCENE */

const settle = (opts) => ({ ...tidy(SPEC, opts, opts.math && { math: opts.math }), seed: opts.seed, base: opts.base, code: opts.code });

export function make(canvas, view, opts = {}) {
  const { math, tell } = opts;
  if (!math) throw new Error('slices: the scene wants @mrlyprod/mrly/math as opts.math');
  const ctx = canvas.getContext('2d');
  const codes = { fill: math.six.FILL(), void: math.six.VOID() };
  let value = settle(opts);
  let plan = study(math, value);
  let dirty = true;
  const paint = () => ({ ...codes, accent: [...rgb(view.look().accent), 255] });
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    picture(pen(ctx, view.w, view.h), layout(view.w, view.h), plan, paint());
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  tell?.({ rows: plan.rows });
  return {
    still: true,
    draw,
    size: touch,
    theme: touch,
    set: (next) => {
      value = settle({ ...next, math });
      plan = study(math, value);
      tell?.({ rows: plan.rows });
      touch();
      return true;
    },
    seek: (x) => scrub(x, view.w),
    svg: () => {
      const w = view.w / view.dpr;
      const h = view.h / view.dpr;
      const page = sheet(w, h);
      picture(page, layout(w, h), plan, paint());
      return page.text();
    },
  };
}
