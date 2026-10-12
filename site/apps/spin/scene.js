import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb } from '../../lib/scene.js';
import { BASES, BEAT, CUT, LIMIT, NEEDLES, NUMBERS, PRESETS, RAMPS, RPM, SOURCES, TILES, cap, cells, edge, key, layout, pace, paint, rings, seen, spun, step, stops, study } from './engine.js';

export const SOURCE = [
  { key: 'source', label: 'Source', kind: 'segment', def: 'flat', options: SOURCES },
  { key: 'base', label: 'Base', kind: 'segment', def: 3, options: BASES },
  { key: 'code', label: 'Code', kind: 'text', def: '495' },
  { key: 'number', label: 'Side', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]) },
  { key: 'level', label: 'Level', kind: 'slider', def: 4, min: 1, max: (value) => cap(value.number), step: 1 },
  { key: 'preset', label: 'Preset', kind: 'pick', def: 'weave', options: PRESETS },
  { key: 'limit', label: 'Scales up to', kind: 'slider', def: 9, min: 1, max: LIMIT, step: 2 },
  { key: 'sbase', label: 'Base', kind: 'segment', def: 2, options: BASES },
  { key: 'scode', label: 'Code', kind: 'text', def: '23' },
  { key: 'tile', label: 'Tile', kind: 'pick', def: 3, options: TILES.map((n) => [n, String(n)]) },
  { key: 'slevel', label: 'Level', kind: 'slider', def: 2, min: 1, max: (value) => cap(value.tile, CUT), step: 1 },
];

export const LOOK = [
  { key: 'ramp', label: 'Ramp', kind: 'pick', def: 'heat', options: RAMPS, group: 'Look' },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 64, min: 2, max: 64, step: 1, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const VIEWS = [['table', 'Turntable'], ['wheel', 'Wheel']];

export const spinning = (value) => value?.view !== 'wheel';

const ramped = (value) => !spinning(value) || value.source === 'moire';

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'table', options: VIEWS },
  { key: 'rpm', label: 'Speed', kind: 'slider', def: 33, min: 0, max: RPM, step: 1, unit: 'rpm', when: spinning },
  { key: 'glow', label: 'Afterglow', kind: 'slider', def: 1, min: 1, max: 120, step: 1, when: spinning },
  ...LOOK.map((row) => (row.key === 'tint' ? row : { ...row, when: ramped })),
];

const SPEED = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: 10, ArrowDown: -10 };

const ABOUT = [
  'A design on a turntable, drawn once a screen frame. The screen strobes it: at 60 Hz the carpet freezes at 900 rpm, a quarter turn a frame, and just off that speed it drifts slowly backwards.',
  'Afterglow blends each new frame into the old ones, the way the eye does.',
  'The Wheel view spins the design infinitely fast: every pixel becomes the mean of the design on its own circle. It is exact: each circle is cut at the grid lines it crosses, and the rings carry the mass of the design.',
  'A plane wave averaged over a turn is the Bessel function J0, so the wheel is the Hankel transform of the radially averaged spectrum. The identities are in [the spin topic](/research/spin/) of Research.',
];

export function tag(value) {
  const src = tidy(SOURCE, value);
  const code = { flat: src.code, moire: src.preset, slice: src.scode }[src.source];
  return `${src.source}-${code}`;
}

export const name = (value) => `spin-${tag(value)}-${value.view}`;

export const PAGE = {
  spec: SPEC,
  record: true,
  about: ABOUT,
  name,
  keys: [
    { key: 'Enter', label: (value, handle) => (handle?.playing?.() === false ? 'Spin' : 'Stop'), act: 'spin', button: true, when: spinning },
    { key: Object.keys(SPEED), label: 'Speed', act: 'speed', when: spinning },
    { key: NEEDLES.map((_, i) => String(i + 1)), label: 'Needle', act: 'needle', does: `${NEEDLES.join(', ')} rpm`, when: spinning },
  ],
  actions: {
    spin: (scene) => scene.toggle?.(),
    speed: (scene, e) => scene.speed?.(SPEED[e.key]),
    needle: (scene, e) => scene.needle?.(NEEDLES[Number(e.key) - 1]),
  },
};

export const units = () => import('./unit.js');

export const PAD = 16;
const TAU = Math.PI * 2;
const GROUND = 36;
const LOOKS = ['ramp', 'levels', 'invert', 'tint'];

export const read = (value) => ({ ...tidy(SOURCE, value), ...tidy(SPEC, value) });

/* PEN */

export function rim(ctx, { x, y, r }, line, dpr) {
  ctx.lineWidth = dpr;
  ctx.strokeStyle = line;
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.stroke();
}

export function place(ctx, layer, disc, line, dpr) {
  const { x, y, r } = disc;
  ctx.save();
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.clip();
  ctx.drawImage(layer, x - r, y - r, 2 * r, 2 * r);
  ctx.restore();
  rim(ctx, disc, line, dpr);
}

export function sheet(pixels, side) {
  const layer = new OffscreenCanvas(side, side);
  layer.getContext('2d').putImageData(new ImageData(pixels, side, side), 0, 0);
  return layer;
}

/* SCENE */

function blank(canvas, view) {
  const ctx = canvas.getContext('2d');
  const draw = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
  };
  return { draw };
}

export const facts = (plan) => ({ kind: plan.kind, code: plan.code, name: plan.name, side: plan.side, fills: plan.fills, mass: plan.mass, disc: plan.disc, peak: plan.peak, reach: plan.reach, order: plan.order });

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return blank(canvas, view);
  const ctx = canvas.getContext('2d');
  let given = opts;
  let value = read(given);
  let plan = null;
  let look = view.look();
  let line = edge(look);
  let disc = null;
  let stamp = null;
  let table = null;
  let wheel = null;
  let fresh = true;
  let dirty = true;
  let phase = 0;
  let last = view.t;
  let beat = view.t;
  let ema = 0;
  let told = false;
  let stopped = false;
  const marked = () => key({ ...value, seed: given.seed >>> 0 });
  const learn = () => {
    plan = study(math, { ...value, seed: given.seed >>> 0 }, view.rand);
    given.onFacts?.(facts(plan));
  };
  const coat = () => {
    const bull = rings(math, plan);
    wheel = sheet(paint(bull.values, bull.top, stops(value.ramp, look), value.levels, value.invert), bull.side);
  };
  const press = () => {
    const { side, data } = plan;
    const raw = sheet(plan.binary ? cells(data, rgb(look.accent), GROUND) : paint(data, plan.top, stops(value.ramp, look), value.levels, value.invert), side);
    const unit = Math.max(1, Math.ceil((disc.r * Math.SQRT2) / side));
    stamp = new OffscreenCanvas(unit * side, unit * side);
    const pen = stamp.getContext('2d');
    pen.imageSmoothingEnabled = false;
    pen.drawImage(raw, 0, 0, unit * side, unit * side);
    const d = Math.ceil(disc.r * 2);
    table = new OffscreenCanvas(d, d);
  };
  const spin = (dt) => {
    if (!stamp) press();
    const { x, y, r } = disc;
    const d = table.width;
    const pen = table.getContext('2d');
    pen.setTransform(1, 0, 0, 1, 0, 0);
    if (value.glow === 1 || view.still || dt < 0) pen.clearRect(0, 0, d, d);
    else {
      pen.globalAlpha = 1 / value.glow;
      pen.fillStyle = look.paper;
      pen.beginPath();
      pen.arc(d / 2, d / 2, d / 2, 0, TAU);
      pen.fill();
      pen.globalAlpha = 1;
    }
    pen.translate(d / 2, d / 2);
    pen.rotate((phase * Math.PI) / 180);
    const span = r * Math.SQRT2;
    pen.drawImage(stamp, -span / 2, -span / 2, span, span);
    ctx.drawImage(table, x - d / 2, y - d / 2);
    rim(ctx, disc, line, view.dpr);
  };
  const tell = (dt) => {
    if (view.still || stopped || !value.rpm || !spinning(value)) {
      if (told) given.onPace?.(null);
      told = false;
      return;
    }
    if (dt <= 0) return;
    ema = pace(dt, ema);
    if (view.t - beat < BEAT) return;
    beat = view.t;
    told = true;
    const deg = step(value.rpm, ema);
    given.onPace?.({ hz: 1000 / ema, deg, seen: seen(deg, plan.order) });
  };
  const still = () => {
    if (!dirty) return;
    dirty = false;
    if (!wheel) coat();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    place(ctx, wheel, disc, line, view.dpr);
  };
  const draw = () => {
    if (fresh) {
      look = view.look();
      line = edge(look);
      wheel = null;
      disc = null;
      fresh = false;
    }
    if (!disc) {
      disc = layout(view.w, view.h, PAD * view.dpr);
      stamp = null;
    }
    const dt = view.t - last;
    last = view.t;
    if (!spinning(value)) {
      tell(dt);
      still();
      return;
    }
    if (dt > 0 && !view.still && !stopped) phase = spun(phase, value.rpm, dt);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    spin(dt);
    tell(dt);
  };
  const set = (next) => {
    const was = value;
    const at = marked();
    given = { ...given, ...next };
    value = read(given);
    if (marked() !== at) {
      learn();
      fresh = true;
    }
    if (LOOKS.some((name) => was[name] !== value[name])) fresh = true;
    if (was.rpm !== value.rpm) beat = view.t;
    dirty = true;
  };
  const theme = () => {
    fresh = true;
    dirty = true;
  };
  const size = () => {
    disc = null;
    dirty = true;
  };
  const toggle = () => {
    stopped = !stopped;
  };
  learn();
  return { draw, size, theme, set, toggle, playing: () => !stopped };
}
