import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, veil } from '../../lib/scene.js';
import { num, runs, sheet } from '../../lib/svg.js';
import { HOT, LEAST, NOTCH, NUMBERS, TURN, cap, heat, lay, most, plan, roots, series, turn } from './engine.js';

export const SPEC = [
  { key: 'a', label: 'Code A', kind: 'text', def: '', group: 'Designs' },
  { key: 'b', label: 'Code B', kind: 'text', def: '', group: 'Designs' },
  { key: 'base', label: 'Base', kind: 'segment', def: 3, options: [[2, '2'], [3, '3']], group: 'Designs' },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Board' },
  { key: 'level', label: 'Level', kind: 'slider', def: 4, min: 1, max: (value) => cap(value.number), step: 1, group: 'Board' },
  { key: 'walkers', label: 'Walkers', kind: 'slider', def: 300, min: 10, max: 1000, step: 10, group: 'Race' },
  { key: 'finish', label: 'Finish', kind: 'slider', def: 50, min: LEAST, max: (value) => most(value.number ** value.level), step: NOTCH, unit: '%', group: 'Race' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 60, min: 6, max: 600, step: 6, unit: '/s', group: 'Race' },
  { key: 'heat', label: 'Fade', kind: 'slider', def: 20, min: 0, max: 90, step: 5, unit: '%/s', group: 'Trail' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [{ key: 'Enter', label: 'Random', act: 'random', button: true }],
  actions: { random: (scene) => scene.random?.() },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const PAD = 16;
const GAP = 16;
const FAINT = 0.1;
const TOP = 0.55;
const RING = 0.45;
const DASH = [4, 6];
const LINE = 1.5;
const WIN = 3;
const HOME = 6;
const TOLD = 100;

const alpha = (h) => FAINT + (TOP - FAINT) * Math.min(1, h / HOT);

const won = (over, s) => over?.how === 'goal' && over.winner === s;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('race: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const value = tidy(SPEC, opts);
  const seeds = roots(view.rand);
  const laid = plan(math, value, seeds.code);
  const { sides, goal, side: n, keep } = laid;
  const sheets = sides.map(() => {
    const page = new OffscreenCanvas(n, n);
    return { page, pen: page.getContext('2d'), image: new ImageData(n, n), hot: new Float32Array(n * n) };
  });
  let show = series(laid, seeds.walk);
  let seen = -1;
  let dirty = true;
  let box = null;
  let colours = [];
  let bytes = [];
  let told = -Infinity;
  let gone = false;
  const tickOf = (t) => Math.floor((t * value.speed) / 1000);
  const paint = () => {
    const { accent } = view.look();
    colours = [accent, turn(accent, TURN)];
    bytes = colours.map(rgb);
    dirty = true;
  };
  const stat = () => {
    const { state } = show;
    return { race: show.k, tick: state.tick, reach: [...state.reach], over: state.over, tally: [...show.tally], goal, side: n };
  };
  const tell = (force) => {
    if (gone || !opts.onStat) return;
    if (!force && view.t - told < TOLD) return;
    told = view.t;
    opts.onStat(stat());
  };
  const level = (s) => {
    const { state } = show;
    const { filled } = sides[s];
    const trail = state.trail[s];
    const stamp = state.stamp[s];
    const tick = state.tick;
    const out = sheets[s].hot;
    for (let i = 0; i < out.length; i++) out[i] = filled[i] ? heat(trail[i], stamp[i], tick, keep) : 0;
    return out;
  };
  const blit = (s) => {
    const { image, pen } = sheets[s];
    const data = image.data;
    const { filled } = sides[s];
    const [r, g, b] = bytes[s];
    const hot = level(s);
    for (let i = 0; i < hot.length; i++) {
      if (!filled[i]) continue;
      const o = i * 4;
      data[o] = r;
      data[o + 1] = g;
      data[o + 2] = b;
      data[o + 3] = Math.round(255 * alpha(hot[i]));
    }
    for (const at of show.state.at[s]) data[at * 4 + 3] = 255;
    pen.putImageData(image, 0, 0);
  };
  const rings = (s) => {
    const { state } = show;
    const { home } = sides[s];
    if (home < 0) return;
    const dpr = view.dpr;
    const { cell, edge } = box;
    const { x, y } = box.boards[s];
    const hx = x + ((home % n) + 0.5) * cell;
    const hy = y + (Math.floor(home / n) + 0.5) * cell;
    ctx.save();
    ctx.beginPath();
    ctx.rect(x, y, edge, edge);
    ctx.clip();
    ctx.strokeStyle = veil(colours[s], RING);
    ctx.lineWidth = dpr;
    ctx.setLineDash(DASH.map((d) => d * dpr));
    ctx.beginPath();
    ctx.arc(hx, hy, goal * cell, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.strokeStyle = colours[s];
    if (state.reach[s] > 0) {
      ctx.lineWidth = (won(state.over, s) ? WIN : LINE) * dpr;
      ctx.beginPath();
      ctx.arc(hx, hy, state.reach[s] * cell, 0, TAU);
      ctx.stroke();
    }
    const m = Math.max(cell, HOME * dpr);
    ctx.lineWidth = dpr;
    ctx.strokeRect(hx - m / 2, hy - m / 2, m, m);
    ctx.restore();
  };
  const draw = () => {
    if (view.still) {
      if (!show.state.over) show.finish(laid.still);
    } else {
      const at = tickOf(view.t);
      if (at < show.start + show.state.tick) show = series(laid, seeds.walk);
      show.to(at);
    }
    if (!dirty && seen === show.version) return;
    seen = show.version;
    dirty = false;
    box ??= lay(view.w, view.h, n, GAP * view.dpr, PAD * view.dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = false;
    sides.forEach((_, s) => {
      blit(s);
      const { x, y } = box.boards[s];
      ctx.drawImage(sheets[s].page, x, y, box.edge, box.edge);
      rings(s);
    });
    tell(Boolean(show.state.over) || show.state.tick === 0);
  };
  const size = () => {
    box = null;
    dirty = true;
  };
  const theme = () => paint();
  const stop = () => {
    gone = true;
  };
  const svg = () => {
    const dpr = view.dpr;
    const css = lay(view.w / dpr, view.h / dpr, n, GAP, PAD);
    const { state } = show;
    const body = sides.map(({ filled, home }, s) => {
      const { x, y } = css.boards[s];
      const hot = level(s);
      const base = runs({ shape: [n, n], types: filled }).map(([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`);
      const warm = [];
      for (let i = 0; i < hot.length; i++) if (hot[i] > 0) warm.push(`<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1" fill-opacity="${num((alpha(hot[i]) - FAINT) / (1 - FAINT))}"/>`);
      const dots = Array.from(state.at[s], (i) => `<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1"/>`);
      const cells = `<g transform="translate(${num(x)} ${num(y)}) scale(${num(css.cell, 4)})" fill="${colours[s]}" shape-rendering="crispEdges"><g fill-opacity="${FAINT}">${base.join('')}</g>${warm.join('')}${dots.join('')}</g>`;
      if (home < 0) return cells;
      const hx = x + ((home % n) + 0.5) * css.cell;
      const hy = y + (Math.floor(home / n) + 0.5) * css.cell;
      const ring = (r, width, dash) => `<circle cx="${num(hx)}" cy="${num(hy)}" r="${num(r)}" fill="none" stroke="${colours[s]}" stroke-width="${num(width)}"${dash ? ` stroke-dasharray="${DASH.join(' ')}" stroke-opacity="${RING}"` : ''}/>`;
      const m = Math.max(css.cell, HOME);
      const mark = `<rect x="${num(hx - m / 2)}" y="${num(hy - m / 2)}" width="${num(m)}" height="${num(m)}" fill="none" stroke="${colours[s]}" stroke-width="1"/>`;
      return `${cells}<clipPath id="race-${s}"><rect x="${num(x)}" y="${num(y)}" width="${num(css.edge)}" height="${num(css.edge)}"/></clipPath><g clip-path="url(#race-${s})">${ring(goal * css.cell, 1, true)}${state.reach[s] > 0 ? ring(state.reach[s] * css.cell, won(state.over, s) ? WIN : LINE, false) : ''}${mark}</g>`;
    });
    return sheet(view.w / dpr, view.h / dpr, body.join(''));
  };
  const csv = () => ['step,a,b', ...show.state.log.map(([a, b], i) => `${i},${num(a, 3)},${num(b, 3)}`)].join('\n');
  const facts = { goal, side: n, walkers: laid.walkers, sides: sides.map(({ code, name, title, fills, of, lit, cells, rms, dimension }) => ({ code, name, title, fills, of, lit, cells, rms, dimension })) };
  paint();
  return { draw, size, theme, stop, svg, csv, stat, facts };
}
