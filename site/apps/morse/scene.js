import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb } from '../../lib/scene.js';
import { DESIGN } from '../designs/scene.js';
import { CELLS, DESIGNED, FOLDS, IN, LIFTS, PAD, VIEWS, camera, fits, lay, panels, phase, picture, reach, sides, study, tower } from './engine.js';

export const SPEC = [
  { key: 'view', label: 'View', kind: 'pick', def: 'plane', options: VIEWS, group: 'View' },
  { key: 'lift', label: 'Lift', kind: 'pick', def: '', options: LIFTS, group: 'View', when: (value) => value.view === 'plane' },
  { key: 'fold', label: 'Fold', kind: 'segment', def: 'sign', options: FOLDS, group: 'View', when: (value) => value.view === 'filter' },
  { key: 'number', label: 'Tile side', kind: 'pick', def: 0, options: sides, group: 'Grid', when: (value) => DESIGNED.includes(value.view) },
  { key: 'level', label: 'Level', kind: 'slider', def: 7, min: 1, max: reach, step: 1, group: 'Grid' },
  { key: 'cell', label: 'Cell', kind: 'pick', def: 0, options: CELLS, group: 'Grid', when: (value) => value.view !== 'word' },
  { key: 'grow', label: 'Grow', kind: 'toggle', def: 1, group: 'Motion' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Again', act: 'again', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'View', act: 'view' },
  ],
  actions: {
    again: (scene) => scene.again(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    view: (scene, e) => scene.turn?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

export const units = () => import('./unit.js');

const REST = 1000;

function stamp(types, w, h, [r, g, b]) {
  const page = new OffscreenCanvas(w, h);
  const pen = page.getContext('2d');
  const image = pen.createImageData(w, h);
  const data = image.data;
  for (let i = 0; i < w * h; i++) {
    if (!types[i]) continue;
    data[i * 4] = r;
    data[i * 4 + 1] = g;
    data[i * 4 + 2] = b;
    data[i * 4 + 3] = 255;
  }
  pen.putImageData(image, 0, 0);
  return page;
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { num, math } = opts;
  if (!num || !math) throw new Error('morse: the scene wants mrlyjs/num and mrlyjs/math as opts.num and opts.math');
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(DESIGN, opts), ...tidy(SPEC, opts) };
  const plan = study({ num, math }, value, view.rand);
  const { level, side, panels: grids, rows } = plan;
  const live = value.grow === 1;
  let origin = 0;
  let dirty = true;
  let boxes = null;
  let sources = [];
  let strips = [];
  const paint = () => {
    const ink = rgb(view.look().accent);
    sources = grids.map((grid) => stamp(grid.types, side, side, ink));
    strips = rows.map((letters) => stamp(letters, letters.length, 1, ink));
  };
  const fit = () => {
    const pad = PAD * view.dpr;
    boxes = plan.view === 'word' ? tower(view.w, view.h, level, pad) : panels(view.w, view.h, grids.length).map((panel) => lay(panel, side, value.cell * view.dpr, pad));
  };
  const grid = (at) => {
    const [sx, sy, s] = camera(plan, at.level, at.p);
    ctx.globalAlpha = at.alpha;
    sources.forEach((source, i) => {
      const box = boxes[i];
      ctx.drawImage(source, sx, sy, s, s, box.x, box.y, box.size, box.size);
    });
  };
  const strip = (at) => {
    strips.forEach((source, k) => {
      const share = k < at.level ? 1 : k === at.level ? at.p : 0;
      if (!share) return;
      ctx.globalAlpha = at.alpha * share;
      const row = boxes.rows[k];
      ctx.drawImage(source, boxes.x, row.y, boxes.w, row.h);
    });
  };
  const draw = () => {
    if (!live && !dirty) return;
    dirty = false;
    if (!boxes) fit();
    const at = phase(view.t - origin, level, view.still || !live);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = false;
    if (plan.view === 'word') strip(at);
    else grid(at);
    ctx.globalAlpha = 1;
  };
  const touch = () => {
    dirty = true;
    if (!live) draw();
  };
  const size = () => {
    boxes = null;
    touch();
  };
  const theme = () => {
    paint();
    touch();
  };
  const again = () => {
    if (!live) return;
    origin = view.t - IN;
    if (view.still) view.wake?.();
    else draw();
  };
  const svg = fits(plan) ? () => picture(plan, view.look().accent) : undefined;
  paint();
  return { draw, every: live ? undefined : REST, size, theme, again, svg, facts: plan.facts };
}
