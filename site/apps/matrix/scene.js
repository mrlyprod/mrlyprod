import FONT from '../../kit/font/font.json' with { type: 'json' };
import { palette } from '../../kit/theme/palette.js';
import { tidy } from '../../lib/knobs.js';
import { HUES, TINTS, board, pick, rng, veil } from '../../lib/scene.js';

export const SPEC = [
  { key: 'set', label: 'Set', kind: 'list', def: 'lds', sep: '', options: [['l', 'Letters'], ['d', 'Digits'], ['s', 'Specials']], group: 'Glyphs' },
  { key: 'color', label: 'Drops', kind: 'segment', def: 'tint', options: [['tint', 'Tint'], ['rainbow', 'Rainbow']], group: 'Colour' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Colour' },
  { key: 'tick', label: 'Tick', kind: 'slider', def: 33, min: 16, max: 120, step: 1, unit: 'ms', group: 'Motion' },
  { key: 'cell', label: 'Size', kind: 'slider', def: 20, min: 8, max: 48, step: 2, group: 'Grid' },
];

export const PAGE = { spec: SPEC };

const SETS = { l: /^[A-Z]$/, d: /^[0-9]$/, s: /^[^\sA-Za-z0-9]$/u };
const FACE = 'MrlyFont';
const FADE = 0.05;
const SIDE = 5;

const square = (char) => FONT[char].rows.length === SIDE && FONT[char].rows.every((row) => row.length === SIDE);

export function glyphs(set) {
  const rules = [...set].map((one) => SETS[one]);
  return Object.keys(FONT).filter((char) => square(char) && rules.some((rule) => rule.test(char)));
}

export function fell(y, h, roll) {
  return y > h && roll > 0.975;
}

export function make(canvas, view, opts) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const { set, color, tick, cell } = tidy(SPEC, opts);
  const pool = glyphs(set);
  const rainbow = color === 'rainbow';
  const hue = () => (rainbow ? palette[pick(rand, HUES)] : null);
  const drops = [];
  const hues = [];
  const salt = Math.floor(rand() * 4294967296);
  let grid = board(view, 1, 1);
  let cols = 0;
  let rows = 0;
  const wash = () => {
    ctx.fillStyle = view.look().paper;
    ctx.fillRect(0, 0, view.w, view.h);
  };
  const size = () => {
    const unit = Math.max(1, Math.round(cell * view.dpr));
    cols = Math.max(1, Math.floor(view.w / unit));
    rows = Math.max(1, Math.floor(view.h / unit));
    grid = board(view, cols, rows);
    const opening = drops.length === 0;
    for (let i = drops.length; i < cols; i++) {
      drops[i] = opening ? 1 : 1 + Math.floor(rand() * rows);
      hues[i] = hue();
    }
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.font = `${grid.cell}px ${FACE}, monospace`;
    ctx.textBaseline = 'alphabetic';
    wash();
  };
  const field = () => {
    const { accent } = view.look();
    const again = rng(salt);
    wash();
    for (let i = 0; i < cols; i++) {
      ctx.fillStyle = hues[i] ?? accent;
      for (let row = 1; row <= rows; row++) {
        ctx.globalAlpha = (1 - FADE) ** (rows - row);
        ctx.fillText(pick(again, pool), grid.x + i * grid.cell, grid.y + row * grid.cell);
      }
    }
    ctx.globalAlpha = 1;
  };
  const draw = () => {
    if (view.still) return field();
    const { paper, accent } = view.look();
    ctx.fillStyle = veil(paper, FADE);
    ctx.fillRect(0, 0, view.w, view.h);
    for (let i = 0; i < cols; i++) {
      if (drops[i] <= rows) {
        ctx.fillStyle = hues[i] ?? accent;
        ctx.fillText(pick(rand, pool), grid.x + i * grid.cell, grid.y + drops[i] * grid.cell);
      }
      if (fell(drops[i], rows, rand())) {
        drops[i] = 0;
        hues[i] = hue();
      }
      drops[i]++;
    }
  };
  size();
  return { every: tick, font: FACE, draw, size, theme: wash };
}
