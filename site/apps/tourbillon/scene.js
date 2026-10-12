import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { RAMPS, colorizer, paint } from '../moire/engine.js';
import { fit, shell } from '../moire/sheet.js';
import { BLENDS, DRAG, MODES, NUDGE, READ, SCHEDULES, SETS, SIZES, SLOP, STAMP, WEIGHTS, angle, live, reading, settle, shade, spin, trim } from './engine.js';

const OWN = () => false;

export const SPEC = [
  { key: 'increment', label: 'Increment', kind: 'slider', def: 1, min: 0, max: 360, unit: 'deg', group: 'Spin', when: OWN },
  { key: 'sched', label: 'Schedule', kind: 'pick', def: 'degrees', options: SCHEDULES, group: 'Spin', when: OWN },
  { key: 'limit', label: 'Scales up to', kind: 'slider', def: 55, min: 3, max: 99, step: 2, group: 'Stack' },
  { key: 'set', label: 'Layers', kind: 'pick', def: 'odd', options: SETS, group: 'Stack' },
  { key: 'weights', label: 'Weights', kind: 'pick', def: 'plain', options: WEIGHTS, group: 'Stack' },
  { key: 'size', label: 'Size', kind: 'segment', def: 512, options: SIZES.map((n) => [n, String(n)]), group: 'Stack' },
  { key: 'mode', label: 'Mode', kind: 'pick', def: 'cells', options: MODES, group: 'Draw' },
  { key: 'blend', label: 'Blend', kind: 'pick', def: 'mean', options: BLENDS, group: 'Draw' },
  { key: 'ramp', label: 'Ramp', kind: 'pick', def: 'auto', options: [['auto', 'Auto'], ...RAMPS], group: 'Look' },
  { key: 'levels', label: 'Levels', kind: 'slider', def: 0, min: 0, max: 64, step: 1, zero: 'Auto', group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  gestures: true,
  name: (value) => `tourbillon-${trim(value.increment)}`,
  about: [
    'The odd parity carpet at the scales 1, 3, 5 and on, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.',
    'The centre is the one point every rotation fixes, so its value never moves. The carpet is unchanged by a quarter turn, so at an increment of 90a/q the layers fall into q angle classes and the spin goes dead: pick a quarter turn and watch it lock.',
    'Play sweeps the increment at 6 degrees a second and Stop lands the angle in the knobs and the link. Drag across the picture to turn it by hand, tap it to play or stop; the arrows nudge the angle by 0.05 degrees.',
    'Unspun, the stack is the one [Moire](/moire/) shows; spinning it breaks the shared grid, and the exact nodes that survive a turn are the Gaussian-integer angles. What a stack is lives in [the stack note](/research/stack/).',
  ],
  keys: [
    { key: 'Enter', label: (value, scene) => (scene?.playing?.() ? 'Stop' : 'Play'), act: 'play', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Nudge', act: 'nudge' },
  ],
  actions: { play: (scene) => scene.toggle?.(), nudge: (scene, e) => scene.nudge?.(e.key === 'ArrowLeft' ? -1 : 1), reroll: (scene) => scene.random?.() },
};

export const units = () => import('./unit.js');

const read = (opts) => ({ ...tidy(SPEC, opts), seed: opts.seed >>> 0 });

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  let value = read(opts);
  let playing = Boolean(view.lock);
  if (playing) value = { ...value, sched: 'degrees' };
  let from = value.increment;
  let origin = 0;
  let held = null;
  let stats = null;
  let made = 0;
  let painted = '';
  let dirty = true;
  let error = '';
  let failed = '';
  let tone = 0;
  let sets = 0;
  let streak = 0;
  let stamped = null;
  let stampAt = 0;
  let readAt = -Infinity;
  let told = '';
  let grab = null;
  let gone = false;
  const planKey = () => [value.limit, value.sched, value.set, value.weights, value.mode, value.blend, value.seed].join(' ');
  const shown = () => (playing ? angle(from, view.t - origin) : value.increment);
  const facts = () => {
    if (error) return { error };
    if (!stats) return null;
    const { layers, scales, angles, mean, rms, faded, centre, weighted, peaks, period, classes, pairs } = stats;
    return { angle: held.at, playing, size: held.size, layers, scales, angles, mean, rms, faded, centre, peaks, period: period ?? null, classes, pairs, unused: !weighted && value.weights !== 'plain' };
  };
  const tell = (now) => {
    if (gone || !opts.onFacts) return;
    if (playing && !now && view.t - readAt < READ) return;
    const next = facts();
    const key = JSON.stringify(next);
    if (key === told) return;
    told = key;
    readAt = view.t;
    opts.onFacts(next);
  };
  const turn = (patch) => {
    if ('increment' in patch) stamped = patch.increment;
    opts.onTurn?.(patch);
  };
  const compute = (at, size) => {
    held?.field.free();
    held = null;
    held = { ...spin(math, value, at, size), key: planKey() };
    stats = reading(math, value, held);
    made += 1;
  };
  const sync = () => {
    const at = shown();
    const fine = value.size;
    const quick = view.fixed || view.still ? fine : live(stats?.layers ?? 1, fine);
    const busy = playing || streak > 1;
    const want = `${planKey()} ${at} ${busy ? quick : fine}`;
    if (want === failed) return;
    try {
      if (!held || held.key !== planKey() || held.at !== at) compute(at, busy ? quick : fine);
      else if (!busy && held.size !== fine) compute(at, fine);
      const how = shade(value, stats);
      const key = [made, how.ramp, how.levels, how.signed, value.invert, value.tint, tone].join(' ');
      if (key !== painted) {
        pad.put(paint(math, held, colorizer(how.ramp, how.levels, view.look()), how.levels, value.invert, how.signed), held.size);
        painted = key;
        dirty = true;
      }
      error = '';
      failed = '';
    } catch (fault) {
      error = String(fault?.message ?? fault);
      failed = want;
      dirty = true;
    }
  };
  const draw = () => {
    if (view.fixed && !playing) toggle();
    streak = sets ? streak + 1 : 0;
    sets = 0;
    sync();
    if (playing && view.t - stampAt >= STAMP) {
      stampAt = view.t;
      turn({ increment: Number(shown().toFixed(2)) });
    }
    tell(false);
    if (!dirty) return;
    pad.clear();
    if (!error && held) pad.blit(held.size, fit(view.w, view.h), true);
    dirty = false;
  };
  const land = (at) => {
    playing = false;
    value = { ...value, increment: settle(at) };
    turn({ increment: value.increment });
    dirty = true;
    return value.increment;
  };
  const toggle = () => {
    if (playing) {
      land(shown());
      tell(true);
      return false;
    }
    playing = true;
    from = value.increment;
    origin = view.t;
    stampAt = view.t;
    if (value.sched !== 'degrees') {
      value = { ...value, sched: 'degrees' };
      turn({ sched: 'degrees' });
    }
    view.wake?.();
    tell(true);
    return true;
  };
  const nudge = (by) => land(shown() + by * NUDGE);
  const set = (next) => {
    const was = value;
    value = read(next);
    sets += 1;
    if (value.increment !== was.increment && value.increment !== stamped) playing = false;
    if (value.sched !== 'degrees') playing = false;
    if (!playing) from = value.increment;
    dirty = true;
  };
  const touch = (u, v, start) => {
    if (start) {
      grab = { u, at: shown(), moved: false };
      return null;
    }
    if (!grab || (!grab.moved && Math.abs(u - grab.u) < SLOP)) return null;
    grab.moved = true;
    playing = false;
    return { increment: settle(grab.at + (u - grab.u) * DRAG) };
  };
  const lift = () => {
    const tap = grab && !grab.moved;
    grab = null;
    if (tap) toggle();
    return null;
  };
  const theme = () => {
    tone += 1;
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const stop = () => {
    gone = true;
    held?.field.free();
    held = null;
  };
  return { draw, set, toggle, nudge, touch, lift, theme, size, stop, playing: () => playing, angle: shown };
}
