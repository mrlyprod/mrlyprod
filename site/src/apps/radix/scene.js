import { tidy } from '../../lib/app/spec.js';
import { TINTS } from '../../lib/color.js';
import { DRAWN, GAP, KEEP, KOCH, LETTERS, PAD, RINGS, SMALL, THIN, bases, cap, corners, lattice, lay, pair, picture, reach, spell, study, twisted } from './engine.js';

const OWN = () => false;

export const SPEC = [
  { key: 'ring', label: 'Ring', kind: 'pick', def: KOCH.ring, options: RINGS, group: 'Design', when: OWN },
  { key: 'base', label: 'Base', kind: 'pick', def: KOCH.base, options: (value) => bases(value.ring), group: 'Design', when: OWN },
  { key: 'd', label: 'Digits', kind: 'text', def: KOCH.d, group: 'Design', when: OWN },
  { key: 't', label: 'Twists', kind: 'text', def: KOCH.t, group: 'Design', when: OWN },
  { key: 'level', label: (value) => `Level 1 to ${cap(value)}`, kind: 'slider', def: KOCH.level, min: 1, max: (value) => cap(value), step: 1, group: 'Design', when: OWN },
  { key: 'line', label: 'Drawn as', kind: 'segment', def: KOCH.line, options: DRAWN, group: 'Design', when: OWN },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 32, step: 1, unit: 'px', zero: 'Fit', group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const file = (value) => ['radix', value.ring, spell(LETTERS[value.ring] ?? 'i', ...pair(value.base)).replaceAll(' ', ''), value.level, value.line ? 'line' : 'cells'].join('-');

export const PAGE = {
  spec: SPEC,
  name: file,
  about: [
    'A radix design writes words of digits and places each word in the plane. Pick a ring, Z[i] on the square lattice or Z[w] on the hexagonal one, a base in it, and one digit for some of the classes modulo the base.',
    'Each digit shrinks the plane by the base, turns it by its twist and moves it by the digit. Level L draws every word of L digits, so the fill is always the digit count to the power L.',
    'Tap a digit chip to take it in or out, and twist a digit by a unit of the ring: the Koch curve is four digits at base 3 with two of them turned. When two words land on one point the maps glue, and the distinct count falls below the fill.',
    'The code names the classes held, so it names the design only when every digit is canonical. Tap the picture to go a level deeper; Reroll draws new digits and twists on the same base. [Gaussian](/gaussian/) walks the same two rings, and [Two](/two/) is what the untwisted whole-number base does.',
  ],
  keys: [
    { key: 'Enter', label: 'Drop the twists', act: 'flat', button: true, when: twisted },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Preset', act: 'preset' },
    { key: 'a', label: 'Every residue', act: 'every' },
  ],
  actions: {
    flat: (scene) => scene.flat?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    preset: (scene, e) => scene.step?.(e.key === 'ArrowLeft' ? -1 : 1),
    every: (scene) => scene.every?.(),
  },
};

const CHUNK = 8192;
const DESIGN = ['ring', 'base', 'd', 't'];
const LOOK = ['level', 'line', 'cell', 'tint'];

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, num, onFacts } = opts;
  if (!math || !num) throw new Error('radix: the scene wants @mrlyprod/mrly/math and @mrlyprod/mrly/num as opts.math and opts.num');
  const ctx = canvas.getContext('2d');
  let value = tidy(SPEC, opts);
  let plan = study(num, math, value);
  const held = new Map();
  const spare = [];
  let box = null;
  let accent = '#000';
  let last = '';
  let gone = false;
  const level = () => Math.min(value.level, plan.cap);
  const tell = () => {
    if (!gone) onFacts?.(plan.facts(level()));
  };
  const paint = () => {
    accent = view.look().accent;
  };
  const fit = () => {
    const pinned = value.cell > 0 ? (value.cell * view.dpr) / lattice(plan.norm, plan.sides, level()).spacing : 0;
    box = lay(value.line ? plan.bounds : plan.stage, view.w, view.h, PAD * view.dpr, pinned);
  };
  const page = () => spare.pop() ?? new OffscreenCanvas(view.w, view.h);
  const lays = (sheet, at) => {
    const pen = sheet.getContext('2d');
    const { sides, norm, angle, sign } = plan;
    const pts = plan.levels[at - 1];
    const n = pts.length / 2;
    const { spacing, radius } = lattice(norm, sides, at);
    const [m11, m12, m21, m22] = box.quarter ? [0, -box.k, box.k, 0] : [box.k, 0, 0, box.k];
    const { cx, cy, ox, oy } = box;
    const x = (i) => ox + m11 * (pts[2 * i] - cx) + m12 * (pts[2 * i + 1] - cy);
    const y = (i) => oy + m21 * (pts[2 * i] - cx) + m22 * (pts[2 * i + 1] - cy);
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.globalAlpha = 1;
    pen.clearRect(0, 0, view.w, view.h);
    if (value.line) {
      pen.strokeStyle = accent;
      pen.lineWidth = Math.min(2 * view.dpr, Math.max(0.6 * view.dpr, spacing * box.k * THIN));
      pen.lineJoin = 'round';
      pen.lineCap = 'round';
      pen.beginPath();
      pen.moveTo(x(0), y(0));
      for (let i = 1; i < n; i++) pen.lineTo(x(i), y(i));
      pen.stroke();
      return;
    }
    pen.fillStyle = accent;
    pen.beginPath();
    let open = 0;
    const flush = () => {
      pen.fill();
      pen.beginPath();
      open = 0;
    };
    if (2 * radius * box.k < SMALL * view.dpr) {
      const s = Math.max(spacing * reach(sides, angle, at) * box.k, view.dpr);
      for (let i = 0; i < n; i++) {
        pen.rect(x(i) - s / 2, y(i) - s / 2, s, s);
        if (++open >= CHUNK) flush();
      }
    } else {
      const ring = corners(sides, angle, at, radius * box.k * (1 - GAP));
      for (let j = 0; j < sides; j++) {
        const vx = ring[2 * j];
        const vy = ring[2 * j + 1] * sign;
        ring[2 * j] = box.quarter ? -vy : vx;
        ring[2 * j + 1] = box.quarter ? vx : vy;
      }
      for (let i = 0; i < n; i++) {
        const px = x(i);
        const py = y(i);
        pen.moveTo(px + ring[0], py + ring[1]);
        for (let j = 1; j < sides; j++) pen.lineTo(px + ring[2 * j], py + ring[2 * j + 1]);
        pen.closePath();
        if (++open >= CHUNK) flush();
      }
    }
    pen.fill();
  };
  const want = (at) => {
    if (held.has(at)) {
      const kept = held.get(at);
      held.delete(at);
      held.set(at, kept);
      return kept;
    }
    while (held.size >= KEEP) {
      const [old, sheet] = held.entries().next().value;
      held.delete(old);
      spare.push(sheet);
    }
    const sheet = page();
    lays(sheet, at);
    held.set(at, sheet);
    return sheet;
  };
  const release = () => {
    spare.push(...held.values());
    held.clear();
    last = '';
  };
  const draw = () => {
    const at = level();
    if (String(at) === last) return;
    last = String(at);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(want(at), 0, 0);
  };
  const set = (next) => {
    const was = value;
    value = tidy(SPEC, next);
    const remade = DESIGN.some((key) => value[key] !== was[key]);
    if (!remade && LOOK.every((key) => value[key] === was[key])) return;
    if (remade) {
      plan.radix.free();
      plan = study(num, math, value);
    }
    if (value.tint !== was.tint) paint();
    const scaled = remade || value.cell !== was.cell || value.line !== was.line || (value.cell > 0 && value.level !== was.level);
    if (scaled) fit();
    if (scaled || value.tint !== was.tint) release();
    last = '';
    if (remade || value.level !== was.level) tell();
  };
  const size = () => {
    fit();
    held.clear();
    spare.length = 0;
    last = '';
  };
  const theme = () => {
    paint();
    release();
  };
  const stop = () => {
    gone = true;
  };
  paint();
  fit();
  tell();
  return { draw, set, size, theme, stop, still: true, svg: () => picture(plan, accent, level(), value.line) };
}
