import { palette as PAINT } from '../../lib/theme/palette.gen.js';
import { bright, HUES, rgb } from '../../lib/color.js';
import { art, paint, spread, tiles } from './engine.js';

const FACES = { cube: [0.18, 0, -0.24], mix: [0, -0.12, 0.14] };

const TONES = { tint1: ['paper', 'accent', 0.3, 0.5], tint2: ['paper', 'accent', 0.55, 0.65], tint3: ['paper', 'accent', 0.8, 0.8], ink1: ['paper', 'fg', 0.12, 0.12], ink2: ['paper', 'fg', 0.3, 0.3], ink3: ['paper', 'fg', 0.55, 0.55], ink4: ['paper', 'fg', 0.8, 0.8] };

const DUSK = 0.6;

const GLINT = 0.5;

const BLACK = [0, 0, 0];

const WHITE = [255, 255, 255];

const blend = (a, b, x) => a.map((v, i) => v + (b[i] - v) * x);

export const css = (c) => `rgb(${c.map(Math.round).join(', ')})`;

export function inks(canvas, { paper, accent }) {
  const style = typeof getComputedStyle === 'function' ? getComputedStyle(canvas) : null;
  const get = (name, fallback) => style?.getPropertyValue(name).trim() || fallback;
  const one = { paper: rgb(paper), accent: rgb(accent), fg: rgb(get('--fg', PAINT.black)) };
  for (const hue of HUES) one[hue] = rgb(get(`--${hue}`, PAINT[hue]));
  const day = bright(paper);
  for (const [name, [from, to, night, light]] of Object.entries(TONES)) one[name] = blend(one[from], one[to], day ? light : night);
  one.shadow = day ? blend(one.accent, BLACK, DUSK) : BLACK;
  one.glint = day ? WHITE : blend(one.accent, WHITE, GLINT);
  one.ground = one.paper;
  one.ink = one.fg;
  return one;
}

export function shade(one, shape, name, face) {
  const lift = FACES[shape]?.[face] ?? 0;
  return css(lift ? blend(one[name], lift > 0 ? one.glint : one.shadow, Math.abs(lift)) : one[name]);
}

export function trace(pen, cell) {
  const p = cell.pts;
  pen.moveTo(p[0][0], p[0][1]);
  for (let i = 1; i < p.length; i++) pen.lineTo(p[i][0], p[i][1]);
  pen.closePath();
}

export function swatch(ctx, w, h, one, { shape, palette, seed, noise, cells = shape === 'mix' ? 2 : 3 }) {
  const grid = tiles(shape, w, h, w / cells);
  const look = art(seed, 0, palette);
  const along = spread(look.turn, w, h);
  for (const cell of grid.cells) {
    const col = shade(one, shape, paint(cell, look, along, { noise, spark: 0 }), cell.face);
    ctx.beginPath();
    trace(ctx, cell);
    ctx.fillStyle = col;
    ctx.fill();
    if (grid.crisp) continue;
    ctx.strokeStyle = col;
    ctx.lineWidth = 1;
    ctx.stroke();
  }
}
