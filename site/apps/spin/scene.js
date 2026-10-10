import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, veil } from '../../lib/scene.js';
import { runs } from '../../lib/svg.js';
import { BEAT, BLENDS, NEEDLES, NUMBERS, SHOWS, angle, cap, layout, pace, petals, pixels, rings, rosette, seen, step, study } from './engine.js';

export const DESIGN = [
  { key: 'base', label: 'Base', kind: 'segment', def: 3, options: [[2, '2'], [3, '3']] },
  { key: 'code', label: 'Code', kind: 'text', def: '' },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]) },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value.number), step: 1 },
];

export const SPEC = [
  { key: 'rpm', label: 'Speed', kind: 'slider', def: 33, min: 0, max: 1800, step: 1, unit: 'rpm', group: 'Turntable' },
  { key: 'glow', label: 'Afterglow', kind: 'slider', def: 1, min: 1, max: 60, step: 1, group: 'Turntable' },
  { key: 'show', label: 'Show', kind: 'pick', def: 'both', options: SHOWS, group: 'Stage' },
  { key: 'copies', label: 'Copies', kind: 'slider', def: 6, min: 1, max: 36, step: 1, group: 'Rosette' },
  { key: 'blend', label: 'Blend', kind: 'pick', def: 'mean', options: BLENDS, group: 'Rosette' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

const SPEED = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: 10, ArrowDown: -10 };

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: Object.keys(SPEED), label: 'Speed', act: 'speed' },
    { key: NEEDLES.map((_, i) => String(i + 1)), label: 'Needle', act: 'needle', does: `${NEEDLES.join(', ')} rpm` },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    speed: (scene, e) => scene.speed?.(SPEED[e.key]),
    needle: (scene, e) => scene.needle?.(NEEDLES[Number(e.key) - 1]),
  },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const PAD = 16;
const RIM = 0.25;
const REST = 1000;

function blank(canvas, view) {
  const ctx = canvas.getContext('2d');
  const draw = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
  };
  return { draw, every: REST };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return blank(canvas, view);
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(DESIGN, opts), ...tidy(SPEC, opts) };
  const { show, rpm, glow } = value;
  const plan = study(math, { ...value, seed: opts.seed >>> 0 }, view.rand);
  const rose = show === 'radial' ? rosette(math, plan, value.copies, value.blend) : null;
  const rest = show === 'wheel' || show === 'radial' || rpm === 0;
  const facts = { code: plan.code, level: plan.level, side: plan.side, fills: plan.fills, mass: plan.mass, reach: plan.reach, inner: plan.inner, disc: plan.disc, peak: plan.peak, profile: plan.profile, order: plan.order, petals: petals(math, plan, value.copies), share: plan.share, leading: plan.leading, power: plan.power };
  let look = view.look();
  let discs = null;
  let stamp = null;
  let table = null;
  let wheel = null;
  let rosy = null;
  let last = view.t;
  let beat = view.t;
  let ema = 0;
  const field = ({ values, side, top }) => {
    const layer = new OffscreenCanvas(side, side);
    layer.getContext('2d').putImageData(new ImageData(pixels(values, top, rgb(look.accent)), side, side), 0, 0);
    return layer;
  };
  const coat = () => {
    if (show === 'both' || show === 'wheel') wheel = field(rings(math, plan));
    if (rose) rosy = field(rose);
  };
  const press = (r) => {
    const unit = Math.max(1, Math.ceil((r * Math.SQRT2) / plan.side));
    stamp = new OffscreenCanvas(unit * plan.side, unit * plan.side);
    const pen = stamp.getContext('2d');
    pen.fillStyle = look.accent;
    pen.beginPath();
    for (const [row, col, len] of runs(plan.cell)) pen.rect(col * unit, row * unit, len * unit, unit);
    pen.fill();
  };
  const lay = () => {
    discs = layout(view.w, view.h, show, PAD * view.dpr);
    if (!discs.turn) return;
    press(discs.turn.r);
    const d = Math.ceil(discs.turn.r * 2);
    table = new OffscreenCanvas(d, d);
  };
  const rim = ({ x, y, r }) => {
    ctx.lineWidth = view.dpr;
    ctx.strokeStyle = veil(look.accent, RIM);
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.stroke();
  };
  const spin = (dt) => {
    const { x, y, r } = discs.turn;
    const d = table.width;
    const pen = table.getContext('2d');
    pen.setTransform(1, 0, 0, 1, 0, 0);
    if (glow === 1 || view.still || dt < 0) pen.clearRect(0, 0, d, d);
    else {
      pen.globalAlpha = 1 / glow;
      pen.fillStyle = look.paper;
      pen.beginPath();
      pen.arc(d / 2, d / 2, d / 2, 0, TAU);
      pen.fill();
      pen.globalAlpha = 1;
    }
    pen.translate(d / 2, d / 2);
    pen.rotate(angle(rpm, view.t));
    const span = r * Math.SQRT2;
    pen.drawImage(stamp, -span / 2, -span / 2, span, span);
    ctx.drawImage(table, x - d / 2, y - d / 2);
    rim(discs.turn);
    if (dt <= 0 || view.still || !rpm) return;
    ema = pace(dt, ema);
    if (view.t - beat < BEAT) return;
    beat = view.t;
    const deg = step(rpm, ema);
    opts.onPace?.({ hz: 1000 / ema, deg, seen: seen(deg, plan.order) });
  };
  const place = (layer, disc) => {
    const { x, y, r } = disc;
    ctx.drawImage(layer, x - r, y - r, 2 * r, 2 * r);
    rim(disc);
  };
  const draw = () => {
    if (!discs) {
      coat();
      lay();
    }
    const dt = view.t - last;
    last = view.t;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (discs.turn) spin(dt);
    if (discs.wheel) place(wheel, discs.wheel);
    if (discs.radial) place(rosy, discs.radial);
  };
  const theme = () => {
    look = view.look();
    coat();
    lay();
  };
  return { draw, size: lay, theme, facts, every: rest ? REST : undefined };
}
