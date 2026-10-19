import { tidy } from '../../lib/app/spec.js';
import { TINTS } from '../../lib/color.js';
import { deep } from '../../lib/fractal/deep.js';
import { parse, print, spanOf, zoomOf } from '../../lib/fractal/view.js';
import { blank, context } from '../../lib/gl/gl2.js';
import { pick } from '../../lib/rand.js';
import { DIVES, KINDS, PLACES } from './places.js';

const HIDE = () => false;
const julian = (value) => value.kind === 'julia';
const FINE = 1e-9;
const OUT = -1.5;
const NEAR = 1;
const TELL = 250;
const SLOW = 0.5;
const ROOM = 8;
const SIDE = 0.25;
const BUILT = ['kind', 'palette', 'speed', 'path', 'depth', 'seed'];
const RABBIT = PLACES.find((one) => one.id === 'rabbit').c;
const four = (v) => v.toFixed(4);

export const DEPTHS = { shallow: 20, deep: 50, deepest: 101 };

export const cap = (value) => DEPTHS[value.depth] ?? DEPTHS.shallow;

export const KEYS = { mandelbrot: ['x', 'y', 'zoom', 'turn'], julia: ['jx', 'jy', 'jzoom', 'jturn'] };

const HOMES = { mandelbrot: ['-0.75', '0'], julia: ['0', '0'] };

export const SPEC = [
  { key: 'kind', label: 'Set', kind: 'segment', def: 'mandelbrot', options: KINDS, group: 'Fractal', view: true },
  { key: 'depth', label: 'Depth', kind: 'segment', def: 'shallow', options: [['shallow', 'Shallow'], ['deep', 'Deep'], ['deepest', 'Deepest']], group: 'Fractal' },
  { key: 'cx', label: 'c real', kind: 'slider', def: RABBIT[0], min: -2, max: 1, step: FINE, show: four, when: julian, group: 'Fractal' },
  { key: 'cy', label: 'c imaginary', kind: 'slider', def: RABBIT[1], min: -1.5, max: 1.5, step: FINE, show: four, when: julian, group: 'Fractal' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 0.75, min: 0.25, max: 2, step: 0.25, group: 'Flight' },
  { key: 'path', label: 'Path', kind: 'toggle', def: 0, group: 'Flight' },
  { key: 'palette', label: 'Palette', kind: 'pick', def: '', options: [['', 'Classic'], ['seeded', 'Seeded'], ['accent', 'Accent'], ['fire', 'Fire'], ['ice', 'Ice'], ['mono', 'Mono']], group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
  { key: 'x', label: 'x', kind: 'text', def: HOMES.mandelbrot[0], when: HIDE },
  { key: 'y', label: 'y', kind: 'text', def: HOMES.mandelbrot[1], when: HIDE },
  { key: 'zoom', label: 'Zoom', kind: 'number', def: 0, min: OUT, max: cap, keep: true, when: HIDE },
  { key: 'turn', label: 'Turn', kind: 'number', def: 0, when: HIDE },
  { key: 'jx', label: 'x', kind: 'text', def: HOMES.julia[0], when: HIDE },
  { key: 'jy', label: 'y', kind: 'text', def: HOMES.julia[1], when: HIDE },
  { key: 'jzoom', label: 'Zoom', kind: 'number', def: 0, min: OUT, max: cap, keep: true, when: HIDE },
  { key: 'jturn', label: 'Turn', kind: 'number', def: 0, when: HIDE },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  gestures: true,
  name: (value) => `fractals-${value.kind}-${value.seed}`,
  about: [
    'Two famous fractals from one rule, z squared plus c. The Mandelbrot set marks every c for which z, starting at 0, stays small forever; a Julia set fixes c and marks every starting z instead. Each point of the Mandelbrot set is the c of a connected Julia set.',
    'Drag to move, scroll or pinch to zoom, double-click to zoom in at a point. The link holds the place, so it opens exactly where you are. The Library lists famous places of both kinds, and J opens the Julia set of the point at the centre.',
    'Play flies a deep zoom from where you are, steering toward detail and settling on a small copy of the set when it finds one. Depth caps how far the flight and your own zoom go: Shallow about a million times, Deep about 10^15, Deepest about 10^30, where the numbers run out.',
    'Each frame is drawn on the GPU by perturbation: one reference orbit in exact big-number arithmetic, and every pixel a small offset from it.',
  ],
  keys: [
    { key: 'Enter', label: (value, handle) => (handle?.playing?.() ? 'Stop' : 'Play'), act: 'play', button: true, menu: 'Edit' },
    { key: ['+', '='], label: 'Zoom in', act: 'in', menu: 'View' },
    { key: ['-', '_'], label: 'Zoom out', act: 'out', menu: 'View' },
    { key: 'j', label: 'Julia here', act: 'julia', when: (value) => !julian(value) },
  ],
  actions: { play: (handle) => handle.toggle?.(), in: (handle) => handle.zoom?.(1), out: (handle) => handle.zoom?.(-1), julia: (handle) => handle.julia?.() },
};

const FAMOUS = PLACES.filter((one) => one.kind === 'mandelbrot' && one.zoom > 0 && one.zoom <= DEPTHS.shallow - ROOM);

const slow = { depth: 'shallow', speed: SLOW, palette: '', tint: '' };

export const SAVERS = [
  { id: 'dive', title: 'Mandelbrot dive', pick: () => ({ kind: 'mandelbrot', ...slow }) },
  { id: 'places', title: 'Famous places', pick: (rand) => ({ ...go(pick(rand, FAMOUS)), ...slow }) },
  { id: 'julia', title: 'Julia dive', pick: () => ({ kind: 'julia', ...slow }) },
];

const LOCK = {
  mandelbrot: { kind: 'set', home: { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 } },
  julia: { kind: 'julia', home: { xMin: -1.5, xMax: 1.5, yMin: -1.5, yMax: 1.5 }, presets: DIVES },
};

/* VIEW */

export function seat(value, kind = value.kind) {
  const [x, y, zoom, turn] = KEYS[kind].map((key) => value[key]);
  const [hx, hy] = HOMES[kind];
  return { centre: [parse(x) ?? parse(hx), parse(y) ?? parse(hy)], span: spanOf(Number(zoom) || 0), angle: ((Number(turn) || 0) * Math.PI) / 180 };
}

export const home = (kind) => seat({}, kind);

const frame = (value, kind) => {
  const [x, y, zoom] = KEYS[kind].map((key) => Number(value[key]));
  const r = spanOf(zoom) * SIDE;
  return { xMin: x - r, xMax: x + r, yMin: y - r, yMax: y + r };
};

export function spot(kind, v) {
  const [x, y, zoom, turn] = KEYS[kind];
  const deg = ((((v.angle * 180) / Math.PI) % 360) + 540) % 360 - 180;
  return { [x]: print(v.centre[0], v.span), [y]: print(v.centre[1], v.span), [zoom]: +zoomOf(v.span).toFixed(3), [turn]: +deg.toFixed(2) || 0 };
}

export const where = (value) => JSON.stringify([value.kind, ...KEYS[value.kind].map((key) => value[key]), ...(julian(value) ? [value.cx, value.cy] : [])]);

export const go = (one) => ({ kind: one.kind, ...(one.c ? { cx: one.c[0], cy: one.c[1] } : {}), ...Object.fromEntries(KEYS[one.kind].map((key, i) => [key, [one.x, one.y, one.zoom, 0][i]])) });

export const at = (value, one) => where({ ...value, ...go(one) }) === where(value);

export const limits = (value) => [spanOf(cap(value)), spanOf(OUT)];

const read = (opts) => ({ ...tidy(SPEC, opts), seed: Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0 });

const tone = (palette) => (palette === 'seeded' ? '' : palette || 'classic');

/* SCENE */

export function make(canvas, view, opts = {}) {
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  let value = read(opts);
  const lock = view.lock;
  const build = (still) => {
    const kind = lock && !opts.kind ? KINDS[value.seed % 2][0] : value.kind;
    const placed = opts[KEYS[kind][0]] !== undefined;
    const plan = lock ? { ...LOCK[kind], ...(placed ? { home: frame(value, kind) } : {}) } : { kind: kind === 'julia' ? 'julia' : 'set', presets: [[value.cx, value.cy]], still };
    return deep(gl, view, { ...plan, bottom: spanOf(cap(value)), seed: value.seed, palette: tone(value.palette), speed: value.speed, path: value.path });
  };
  let space;
  try {
    space = build(seat(value));
  } catch (error) {
    console.error(error);
    return blank(canvas);
  }
  const stop = () => {
    space.drop();
    gl.getExtension('WEBGL_lose_context')?.loseContext();
  };
  if (lock) return { draw: () => space.draw(), theme: () => space.theme(), stop };
  let flew = false;
  let landed = null;
  let told = -Infinity;
  const draw = () => {
    space.draw();
    const on = space.flying();
    if (on && view.t - told >= TELL) {
      told = view.t;
      opts.onTell?.(space.at());
    }
    if (flew && !on) landed = spot(value.kind, space.at());
    flew = on;
    if (on || view.fixed) return;
    opts.onRest?.(landed);
    landed = null;
  };
  const set = (next) => {
    const was = value;
    value = read(next);
    const moved = where(value) !== where(was);
    const built = BUILT.some((key) => value[key] !== was[key]);
    if (!moved && !built) return;
    const flying = space.flying() && !moved;
    const here = moved ? seat(value) : space.halt();
    if (built) {
      space.drop();
      space = build(here);
    } else space.look(here, [value.cx, value.cy]);
    if (flying) space.fly();
    flew = flying;
    told = -Infinity;
  };
  const fly = () => {
    if (space.flying()) return;
    const v = space.at();
    space.fly(zoomOf(v.span) > cap(value) - NEAR ? home(value.kind) : undefined);
    flew = true;
  };
  const halt = () => {
    flew = false;
    return spot(value.kind, space.halt());
  };
  return { draw, theme: () => space.theme(), set, stop, fly, halt, flying: () => space.flying(), at: () => space.at() };
}
