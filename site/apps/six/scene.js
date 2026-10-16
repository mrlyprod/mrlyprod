import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, rng } from '../../lib/scene.js';
import { settle } from '../designs/engine.js';
import { DESIGN } from '../designs/scene.js';
import { DIM, NUMBERS, REPEATS, RINGS, SCALE, SHEETS, VIEWS, cap, copies, croppable, fit, kinds, mesh, studio, wrap } from './engine.js';

const rect = (value) => value.sheet !== 'disc';

export const SPEC = [
  { key: 'view', label: 'View', kind: 'segment', def: 'cut', options: VIEWS },
  { key: 'sheet', label: 'Shape', kind: 'segment', def: 'rect', options: SHEETS, group: 'Sheet' },
  { key: 'x', label: 'Across', kind: 'slider', def: 5, min: 1, max: REPEATS, step: 1, group: 'Sheet', when: rect },
  { key: 'y', label: 'Down', kind: 'slider', def: 5, min: 1, max: REPEATS, step: 1, group: 'Sheet', when: rect },
  { key: 'radius', label: 'Rings', kind: 'slider', def: 2, min: 0, max: RINGS, step: 1, group: 'Sheet', when: (value) => !rect(value) },
  { key: 'overhang', label: 'Keep the overhang', kind: 'toggle', def: 1, group: 'Sheet', when: croppable },
  { key: 'number', label: 'Side', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Tile' },
  { key: 'level', label: 'Level', kind: 'slider', def: 2, min: 1, max: (value, units) => units && cap(units.math, value.number, copies(units.math, value)), step: 1, group: 'Tile', keep: true },
  { key: 'turn', label: 'Quarter turn', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const units = () => import('./unit.js');

const ABOUT = [
  'One cube design seen on the triangular lattice makes a hexagon: its middle slice, its three facing sides, or its isometric skin. Copies of the hexagon interlock into a sheet.',
  'The fills multiply by the copy count. The exposed edges, the rim of the filled triangles, do not: each copy buries the edges it shares with a neighbour, so the sheet exposes less than one copy times the copies.',
  'Crop trims the jagged overhang to a clean rectangle, and cuts fills away with it.',
  'Tap the picture or press the arrows for the next view. Reroll rolls a new design. [Slices](/slices/) counts a single hexagon, and [the core](/research/core/) says why a design\'s symmetries belong to the infinite tiling.',
];

export function name(value, units) {
  const { base, code } = tidy(DESIGN, { ...value, dim: DIM });
  return `six-${units?.math ? settle(units.math, DIM, base, code, rng(value.seed)) : code}-${value.view}`;
}

export const PAGE = {
  spec: SPEC,
  units,
  about: ABOUT,
  name,
  keys: [
    { key: 'Enter', label: 'Crop', act: 'crop', button: true, when: (value) => croppable(value) && value.overhang === 1 },
    { key: 'Enter', label: 'Uncrop', act: 'crop', button: true, when: (value) => croppable(value) && value.overhang !== 1 },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'View', act: 'view' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    crop: (scene) => scene.crop?.(),
    view: (scene, e) => scene.view?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

const PAD = 24;
const VOID = 0.1;
const LEFT = 0.72;
const RIGHT = 0.5;

/* COLOUR */

export function tones(look) {
  const a = rgb(look.accent);
  const p = rgb(look.paper);
  const toward = (k) => a.map((v, i) => Math.round(p[i] + (v - p[i]) * k));
  return { FILL: [...a, 255], VOID: [...a, Math.round(255 * VOID)], UP: [...a, 255], LEFT: [...toward(LEFT), 255], RIGHT: [...toward(RIGHT), 255] };
}

const css = ([r, g, b, a]) => `rgba(${r}, ${g}, ${b}, ${Math.round((a / 255) * 1000) / 1000})`;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('six: the scene wants @mrlyprod/mrly/math as opts.math');
  const ctx = canvas.getContext('2d');
  const tell = opts.onFacts;
  const study = studio(math);
  const codes = kinds(math);
  let now = null;
  let built = null;
  let made = '';
  let turn = false;
  const take = (next) => {
    const value = { ...tidy(SPEC, next, { math }), ...tidy(DESIGN, { ...next, dim: DIM }), seed: next.seed };
    now = study(value);
    turn = value.turn === 1;
    const key = `${now.key}${turn}`;
    if (key !== made) {
      built = mesh(math, now.drawn, turn);
      made = key;
    }
    tell?.(now.facts);
  };
  take(opts);
  const paint = () => {
    const look = tones(view.look());
    const { k, ox, oy } = fit(built.box, view.w, view.h, PAD * view.dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    for (const { name, points } of built.groups) {
      ctx.fillStyle = css(look[name]);
      ctx.beginPath();
      for (let i = 0; i < points.length; i += 6) {
        ctx.moveTo(ox + points[i] * k, oy + points[i + 1] * k);
        ctx.lineTo(ox + points[i + 2] * k, oy + points[i + 3] * k);
        ctx.lineTo(ox + points[i + 4] * k, oy + points[i + 5] * k);
        ctx.closePath();
      }
      ctx.fill();
    }
  };
  const custom = () => {
    const look = tones(view.look());
    return Object.fromEntries(Object.entries(codes).map(([name, type]) => [String(type), [look[name] ?? [0, 0, 0, 0]]]));
  };
  const svg = () => wrap(math.six.svg(math.six.paint(now.drawn, custom()), SCALE, undefined, 0, undefined), now.drawn.orientation === 'Horizontal', turn);
  const tile = () => wrap(math.six.rect_svg(math.six.paint(now.one, custom()), SCALE), now.one.orientation === 'Horizontal', turn);
  return { draw: paint, still: true, set: take, svg, tile };
}
