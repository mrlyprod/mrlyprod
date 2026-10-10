import { text } from '../../lib/export.js';
import { tidy } from '../../lib/knobs.js';
import { TINTS, board } from '../../lib/scene.js';
import { grid, rects } from '../../lib/svg.js';
import { at, plan, sample, slug } from './engine.js';

export const SPEC = [
  { key: 'text', label: 'Text', kind: 'text', def: '', wide: true, group: 'Text' },
  { key: 'pad', label: 'Pad', kind: 'slider', def: 1, min: 0, max: 4, step: 1, unit: 'cells', group: 'Text' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 1, min: 0.25, max: 4, step: 0.25, unit: 'x', group: 'Motion' },
  { key: 'hold', label: 'Hold', kind: 'slider', def: 25, min: 0, max: 100, step: 5, unit: 'frames', group: 'Motion' },
  { key: 'loop', label: 'Loop', kind: 'toggle', def: 1, group: 'Motion' },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 40, step: 2, unit: 'px', group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

const file = (scene, kind, body) => text(body, `font-${slug(scene.plan.text)}`, kind);

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'j', label: 'JSON', act: 'json' },
    { key: 's', label: 'SVG', act: 'svg' },
  ],
  actions: {
    json: (scene) => file(scene, 'json', JSON.stringify(scene.json())),
    svg: (scene) => file(scene, 'svg', scene.svg()),
  },
};

export const units = () => import('./unit.js');

const FIT = 0.92;

export function layout(view, { rows, cols }, cell) {
  const fit = board(view, Math.max(1, cols), Math.max(1, rows), FIT);
  if (!cell) return fit;
  const side = Math.min(fit.cell, Math.max(1, Math.round(cell * view.dpr)));
  const w = side * cols;
  const h = side * rows;
  return { x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h, cell: side };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const value = tidy(SPEC, opts);
  const text = value.text || sample(view.rand);
  const show = plan(opts.font, { ...value, text });
  const { frames, cols } = show;
  let box = layout(view, show, value.cell);
  let shown = -1;
  const frame = () => (view.still ? show.still : at(show, view.t, value.speed, value.loop));
  const paint = (k) => {
    const { x, y, cell } = box;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = view.look().accent;
    ctx.beginPath();
    for (const i of frames[k]) ctx.rect(x + (i % cols) * cell, y + Math.floor(i / cols) * cell, cell, cell);
    ctx.fill();
    shown = k;
  };
  const draw = () => {
    const k = frame();
    if (k !== shown) paint(k);
  };
  const size = () => {
    box = layout(view, show, value.cell);
    paint(frame());
  };
  const theme = () => paint(frame());
  const json = () => ({ text: show.text, pad: value.pad, hold: value.hold, loop: value.loop, rows: show.rows, cols: show.cols, fps: show.fps * value.speed, frames });
  const still = () => rects(grid(show.rows, cols, frames[shown < 0 ? frame() : shown]), { fill: view.look().accent, unit: box.cell / view.dpr });
  return { draw, size, theme, plan: show, json, svg: still, box: () => box };
}
