import { ink as inks } from '#view';
import { palette } from '../../lib/theme/palette.gen.js';
import { dark, light } from '../../lib/theme/theme.js';
import { tidy } from '../../lib/app/spec.js';
import { TINTS, rgb } from '../../lib/color.js';
import { CARPET, COMBINES, CUBES, DEN, FACES, LAYERS, NUMBERS, PLANES, RAMPS, REACH, SHAPES, SIZES, cap, circle, colorizer, counted, cropped, design, faces, field, grown, load, most, named, plane, reading, region, shell, tag, tally, top, walls } from '../../lib/three/engine.js';
import { space } from './stage.js';
import { wasm } from '../../lib/units/index.js';

const word = (one) => one[0].toUpperCase() + one.slice(1);
const fixed = (v, k) => Number(v).toFixed(k);
const is = (key, want) => (value) => value[key] === want;
const crop = (value) => Boolean(value.crop);
const grows = is('source', 'design');
const fills = is('source', 'field');
const counting = (value) => grows(value) && Number(value.circle) === 1;

export const SPEC = [
  { key: 'source', label: 'Solid', kind: 'segment', def: 'design', options: [['design', 'Design'], ['field', 'Field']], group: 'Solid' },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Solid', when: grows },
  { key: 'crop', label: 'Shape', kind: 'pick', def: '', options: [['', 'None'], ...SHAPES.map((one) => [one, word(one)])], group: 'Crop' },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value, units) => units && cap(units.math, value.number, value.crop), step: 1, group: 'Solid', when: grows },
  { key: 'draw', label: 'Draw', kind: 'segment', def: 'shell', options: [['shell', 'Shell'], ['cubes', 'Cubes']], group: 'Solid', when: grows },
  { key: 'opacity', label: 'Opacity', kind: 'slider', def: 1, min: 0.05, max: 1, step: 0.05, group: 'Solid' },
  { key: 'spin', label: 'Spin', kind: 'toggle', def: 1, group: 'Solid' },
  { key: 'camera', label: 'Camera', kind: 'segment', def: 'eye', options: [['eye', 'Perspective'], ['iso', 'Isometric']], group: 'Solid' },
  { key: 'radius', label: 'Radius', kind: 'slider', def: 60, min: 0, max: REACH, step: 1, unit: `/${DEN}`, group: 'Crop', when: crop },
  { key: 'anti', label: 'Anti', kind: 'toggle', def: 0, group: 'Crop', when: crop },
  { key: 'policy', label: 'Keep', kind: 'segment', def: 'touching', options: [['inside', 'Inside'], ['touching', 'Touching']], group: 'Crop', when: crop },
  { key: 'edge', label: 'Exact edge', kind: 'toggle', def: 0, group: 'Crop', when: (value) => crop(value) && grows(value) && value.crop !== 'ball' && Number(value.anti) !== 1 },
  { key: 'limit', label: 'Scales up to', kind: 'slider', def: 7, min: 1, max: LAYERS, step: 2, group: 'Field', when: fills },
  { key: 'combine', label: 'Combine', kind: 'segment', def: 'sum', options: COMBINES, group: 'Field', when: fills },
  { key: 'depth', label: 'Level', kind: 'slider', def: 1, min: 1, max: 2, step: 1, group: 'Field', when: fills },
  { key: 'size', label: 'Size', kind: 'segment', def: 64, options: SIZES.map((n) => [n, String(n)]), group: 'Field', when: fills },
  { key: 'shell', label: 'Shell', kind: 'toggle', def: 1, group: 'Field', when: fills },
  { key: 'threshold', label: 'Threshold', kind: 'slider', def: LAYERS, min: 0, max: (value, units) => (units && fills(value) ? most(units.math, value) : LAYERS), step: 1, group: 'Field', when: (value) => fills(value) && Number(value.shell) === 1 },
  ...PLANES.flatMap(([key, label]) => [
    { key: `c${key}`, label, kind: 'toggle', def: key === 'd' ? 1 : 0, group: 'Planes', when: fills },
    { key: `p${key}`, label: 'At', kind: 'slider', def: 0.5, min: 0, max: 1, step: 0.01, group: 'Planes', when: (value) => fills(value) && Number(value[`c${key}`]) === 1 },
  ]),
  { key: 'ramp', label: 'Ramp', kind: 'segment', def: 'fire', options: RAMPS, group: 'Colour', when: fills },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 16, min: 2, max: 64, step: 1, group: 'Colour', when: fills },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Colour', when: fills },
  { key: 'circle', label: 'Count', kind: 'toggle', def: 0, group: 'Circle', when: grows },
  { key: 'centre', label: 'Centre', kind: 'segment', def: 'corner', options: [['corner', 'Corner'], ['centre', 'Centre']], group: 'Circle', when: counting },
  { key: 'count', label: 'Count level', kind: 'slider', def: 4, min: 1, max: (value, units) => units && counted(units.math, value.number), step: 1, group: 'Circle', when: counting },
  { key: 'r', label: 'Radius r', kind: 'slider', def: 27, min: 1, max: (value, units) => units && Math.max(1, top(units.math, value.number, value.count, value.centre)), step: 1, group: 'Circle', when: counting },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const units = wasm('math');

export const name = (value) => `three-${String(value.code ?? '').trim() || CARPET}-${value.source === 'field' ? 'field' : `l${value.level}`}${value.crop ? `-${value.crop}` : ''}`;

export const PAGE = {
  spec: SPEC,
  units,
  gestures: true,
  record: true,
  about: [
    'A code picks the filled corners of a cube, and the cube grows into itself level by level; code 23 grows the Menger sponge.',
    'Drag to orbit, scroll or pinch to zoom, right-drag or shift-drag to pan. The counts come from closed formulas and the mesh from the exposed faces, so nothing is guessed.',
    'A named shape keeps the cells it reaches, inside or touching; anti keeps the outside, and the exact edge clips the whole mesh with the shape walls. Left and Right move the radius; the plane crop lives in [Crop](/crop/).',
    'The field stacks the design at scales 1, 3, 5 and on into one solid: a threshold draws its shell and a plane shows the moire it cuts. More in [the core](/research/core/), [the crop note](/research/crop/) and [the cuts note](/research/cuts/).',
  ],
  name,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level', when: grows },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Radius', act: 'radius', when: crop },
    { key: '1', label: 'Corner', act: 'corner' },
    { key: '2', label: 'Edge', act: 'edge' },
    { key: '3', label: 'Face', act: 'face' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    radius: (scene, e) => scene.widen?.(e.key === 'ArrowLeft' ? -1 : 1),
    corner: (scene) => scene.look?.('corner'),
    edge: (scene) => scene.look?.('edge'),
    face: (scene) => scene.look?.('face'),
  },
};

const ORANGE = rgb(palette.orange);

/* STUDY */

const title = (made, math) => tag(named(math, made.code, made.base), made.code, made.base);

function solid(math, value) {
  const made = design(math, value);
  const level = Math.min(value.level, cap(math, value.number, value.crop));
  const { code, base } = made;
  const { number } = value;
  const cell = grown(math, code, base, number, level);
  const cut = value.crop ? cropped(math, cell, value) : null;
  const exact = Boolean(cut) && Number(value.edge) === 1 && value.crop !== 'ball' && Number(value.anti) !== 1;
  const shown = cut && !exact ? cut.cell : cell;
  const cubes = value.draw === 'cubes';
  const heavy = load(math, shown, cubes);
  const error = heavy > (cubes ? CUBES : FACES) ? `${heavy} ${cubes ? 'cubes' : 'faces'} is more than this page draws; lower the level.` : '';
  const census = tally(math, cell);
  const rows = [
    ['Name', title(made, math)],
    ['Side', cell.shape[0]],
    ['Filled', String(math.counts.fill(code, number, 3, level, base))],
    ['Empty', String(math.counts.void(code, number, 3, level, base))],
    ['Exposed', String(math.counts.surface(code, number, level, base))],
    ['Fill ratio', fixed(math.counts.ratio(code, number, 3, level, base), 4)],
    ['Dimension', fixed(math.counts.dimension(code, number, 3, base), 4)],
    ...(census ? [['Vertices', census.vertices], ['Edges', census.edges], ['Faces', census.faces], ['Euler', String(census.euler)]] : []),
    ...(cut ? [['Filled in', cut.in], ['Filled cut', cut.cut], ['Filled out', cut.out], ['Exposed after', cut.after]] : []),
  ];
  if (Number(value.circle) === 1) {
    const count = circle(math, made, value);
    const read = reading(count, value.r);
    rows.unshift(['N(r)', read.seen], ['Inside(r)', read.inside], ['C(r)', read.cut], ['Count side', count.side]);
    rows.push(['r', read.at], ['d', fixed(count.d, 7)], ['Fill of one tile', count.mass]);
  }
  const body = error ? null : { key: `${code} ${base} ${number} ${level} ${value.draw} ${exact ? 'whole' : value.crop} ${value.radius} ${value.anti} ${value.policy}`, cell: shown, cubes, quads: cubes ? null : faces(math, shown) };
  const clip = exact ? walls(region(math, value.crop, value.radius, false)) : null;
  return { body, clip, rows: error ? [['Error', error], ...rows] : rows, error, obj: () => math.three.to_obj(shown) };
}

function volume(math, value, paint) {
  const made = design(math, value);
  const held = field(math, made, value);
  const threshold = Math.min(value.threshold, held.max);
  const on = Number(value.shell) === 1;
  const kept = on ? shell(math, held, { ...value, threshold }) : null;
  const error = kept && kept.faces > FACES ? `${kept.faces} faces is more than this page draws; raise the threshold or lower the size.` : '';
  const cuts = PLANES.filter(([key]) => Number(value[`c${key}`]) === 1).map(([key]) => plane(math, held, key, value[`p${key}`], paint));
  const rows = [
    ['Name', title(made, math)],
    ['Layers', held.layers],
    ['Voxels', held.voxels],
    ['At or above', held.volume.count(threshold)],
    ...(kept ? [['Faces', kept.faces]] : []),
    ['Field', `${held.min} to ${held.max}`],
  ];
  return {
    body: kept && !error ? { key: `field ${made.code} ${made.base} ${value.limit} ${value.combine} ${value.depth} ${value.size} ${threshold} ${value.crop} ${value.radius} ${value.anti} ${value.policy}`, cell: kept.cell, cubes: false, quads: faces(math, kept.cell) } : null,
    clip: null,
    planes: cuts,
    rows: error ? [['Error', error], ...rows] : rows,
    error,
    obj: () => (kept ? math.three.to_obj(kept.cell) : ''),
  };
}

export function study(math, value, paint) {
  return value.source === 'field' ? volume(math, value, paint) : { ...solid(math, value), planes: [] };
}

function night() {
  const set = typeof document === 'undefined' ? '' : document.documentElement.dataset.theme;
  if (set === 'dark' || set === 'light') return set === 'dark';
  return typeof matchMedia === 'function' && matchMedia('(prefers-color-scheme: dark)').matches;
}

const ink = (accent) => inks({ ...(night() ? dark : light), accent });

export function painter(view, value) {
  return { colorizer: colorizer(ink(view.look().accent), value.ramp, value.levels), levels: value.levels, invert: value.invert };
}

/* SCENE */

const settle = (opts) => ({ ...tidy(SPEC, opts, opts.math && { math: opts.math }), seed: opts.seed, base: opts.base, code: opts.code });

const same = (a = [], b = []) => a.length === b.length && a.every((one, i) => one === b[i]);

export function make(canvas, view, opts = {}) {
  const { math, three, tell } = opts;
  if (!math) throw new Error('three: the scene wants @mrlyprod/mrly/math as opts.math');
  let value = settle(opts);
  let plan = null;
  let stage = null;
  if (three) {
    try {
      stage = space(three, canvas, view);
    } catch {
      stage = null;
    }
  }
  const tone = (next) => (next.draw === 'cubes' && next.source === 'design' ? ORANGE : rgb(view.look().accent));
  const apply = (next, old) => {
    const was = plan;
    plan = study(math, next, painter(view, next));
    tell?.({ rows: plan.rows, error: plan.error });
    if (!stage) return;
    if (!was || plan.body?.key !== was.body?.key) stage.show(plan.body);
    stage.paint({ color: tone(next), opacity: next.opacity, clip: plan.clip });
    if (!was || !same(plan.planes, was.planes)) stage.planes(plan.planes);
    stage.project(next.camera);
    if (!old || Number(next.spin) !== Number(old.spin)) stage.spinning(Number(next.spin) === 1);
    stage.mark();
  };
  apply(value, null);
  if (stage && value.camera === 'iso') stage.look('corner');
  let pending = null;
  const flush = () => {
    if (!pending) return;
    const [next, old] = pending;
    pending = null;
    apply(next, old);
  };
  const blank = () => {};
  return {
    draw: () => {
      flush();
      stage?.draw();
    },
    size: stage ? stage.size : blank,
    theme: () => {
      pending = [value, pending?.[1] ?? value];
    },
    set: (next) => {
      const old = pending?.[1] ?? value;
      value = settle({ ...next, math });
      pending = [value, old];
      return true;
    },
    turn: stage?.turn,
    fling: stage?.fling,
    zoom: stage?.zoom,
    pan: stage?.pan,
    look: stage?.look,
    obj: () => (flush(), plan.obj()),
    stop: () => stage?.stop(),
  };
}
