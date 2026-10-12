import { draw as hexes } from '../../../pkgs/mrlyjs/view/hex.js';
import { veil } from '../../lib/scene.js';
import { measureOf, spot } from './engine.js';
import { mix } from './paint.js';

export const BUDGET = { 2: 250000, 3: 600000, cut: 60000 };

const CUT = new Set(['triangles', 'cutvoids', 'holes', 'pieces']);

const PRO = new Set(['profills', 'provoids']);

const KINDS = { VOID: 0, FILL: 1, GRID: 2, UP: 3, LEFT: 4, RIGHT: 5 };

export const view = (row) => (row.dim === 2 ? 'grid' : CUT.has(row.measure) ? 'cut' : PRO.has(row.measure) ? 'pro' : 'iso');

const budget = (row) => (view(row) === 'cut' ? BUDGET.cut : BUDGET[row.dim]);

export function fits(math, row, index) {
  let at = Math.max(0, index);
  while (at > 0) {
    const { number, level } = spot(row, at);
    if (math.bang.level_cap(number, row.dim, budget(row)) >= level) break;
    at--;
  }
  return at;
}

export function sheet(math, row, number, level) {
  const kind = view(row);
  if (kind === 'grid') {
    const cell = math.two.create(String(row.code), number, level, 0, row.base);
    return { kind, number, level, shape: cell.shape, types: cell.types };
  }
  const made = math.six[`${kind}_design`](String(row.code), number, level, row.base);
  return { kind, number, level, cell: made.cell, start: made.start };
}

export function picture(math, row, index) {
  const at = fits(math, row, index);
  const { number, level } = spot(row, at);
  return { at, ...sheet(math, row, number, level) };
}

export const named = (one) => (one.kind === 'grid' || one.kind === 'iso' ? '' : one.kind === 'cut' ? ', the middle cut' : ', face on');

export const said = (one) => `side ${one.number}, level ${one.level}${named(one)}`;

export function inks(row, tones, on = tones.accent) {
  const off = veil(on, 0.12);
  const voids = measureOf(row.measure)?.slug === 'voids' || row.measure === 'cutvoids' || row.measure === 'provoids';
  return {
    fill: voids ? veil(on, 0.28) : on,
    void: voids ? mix(on, tones.paper, 0.15) : off,
    up: on,
    left: mix(on, tones.paper, 0.32),
    right: mix(on, tones.paper, 0.58),
  };
}

function grid(ctx, box, one, ink) {
  const [rows, cols] = one.shape;
  const cell = Math.min(box.w / cols, box.h / rows);
  const w = cell * cols;
  const h = cell * rows;
  const x0 = box.x + (box.w - w) / 2;
  const y0 = box.y + (box.h - h) / 2;
  const gap = cell >= 6 ? Math.max(1, cell * 0.08) : 0;
  ctx.fillStyle = ink.void;
  ctx.fillRect(x0, y0, w, h);
  ctx.fillStyle = ink.fill;
  ctx.beginPath();
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (one.types[r * cols + c]) ctx.rect(x0 + c * cell + gap / 2, y0 + r * cell + gap / 2, cell - gap + (gap ? 0 : 0.5), cell - gap + (gap ? 0 : 0.5));
    }
  }
  ctx.fill();
  return { x: x0, y: y0, w, h };
}

function hex(ctx, box, one, ink) {
  const tone = { [KINDS.FILL]: ink.fill, [KINDS.VOID]: one.kind === 'iso' ? null : ink.void, [KINDS.UP]: ink.up, [KINDS.LEFT]: ink.left, [KINDS.RIGHT]: ink.right };
  const runs = new Map();
  const pen = {
    triangle: (a, b, c, color) => {
      if (!runs.has(color)) runs.set(color, []);
      runs.get(color).push(a, b, c);
    },
  };
  const [height, width] = one.cell.shape;
  const across = width > height;
  const big = (across ? box.w / width : box.h / height) * 2;
  hexes(pen, box, { cell: one.cell, start: one.start }, big > 14 ? Math.max(0.6, big * 0.03) : 0, (type) => tone[type] ?? null);
  for (const [color, pts] of runs) {
    ctx.fillStyle = color;
    ctx.strokeStyle = color;
    ctx.lineWidth = big > 14 ? 0 : 0.6;
    ctx.beginPath();
    for (let i = 0; i < pts.length; i += 3) {
      ctx.moveTo(pts[i][0], pts[i][1]);
      ctx.lineTo(pts[i + 1][0], pts[i + 1][1]);
      ctx.lineTo(pts[i + 2][0], pts[i + 2][1]);
      ctx.closePath();
    }
    ctx.fill();
    if (big <= 14) ctx.stroke();
  }
  return box;
}

export function draw(ctx, box, one, ink) {
  if (!one) return box;
  return one.kind === 'grid' ? grid(ctx, box, one, ink) : hex(ctx, box, one, ink);
}
