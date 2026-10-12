import { light } from '../../kit/theme/theme.js';
import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb, rng, veil } from '../../lib/scene.js';
import { num, runs, sheet } from '../../lib/svg.js';
import { turn } from '../../lib/turn.js';
import { HOLD, HOT, NUMBERS, TURN, aim, bake, budget, cap, heat, keeping, lay, plan, roots, series, top } from './engine.js';

export const SPEC = [
  { key: 'a', label: 'Code A', kind: 'text', def: '127', group: 'Race' },
  { key: 'b', label: 'Code B', kind: 'text', def: '239', group: 'Race' },
  { key: 'base', label: 'Base', kind: 'segment', def: 3, options: [[2, '2'], [3, '3']], group: 'Board' },
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Board' },
  { key: 'level', label: 'Level', kind: 'slider', def: 4, min: 1, max: (value) => cap(value.number), step: 1, group: 'Board' },
  { key: 'finish', label: 'Finish', kind: 'slider', def: 20, min: 1, max: (value) => top(value.number ** value.level), step: 1, unit: 'cells', group: 'Race' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 720, min: 6, max: 3000, step: 6, unit: '/s', group: 'Race' },
  { key: 'walkers', label: 'Walkers', kind: 'slider', def: 300, min: 10, max: 1000, step: 10, group: 'Race' },
  { key: 'heat', label: 'Fade', kind: 'slider', def: 0, min: 0, max: 90, step: 5, unit: '%/s', zero: 'Off', group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

const ABOUT = [
  'Two designs that keep the same number of cells at every scale weigh the same and share one fractal dimension. Walkers start at home on both boards, the filled cell nearest the centre, and the first team to wander an average of the finish from home wins.',
  'Each tick every walker takes one blind step, and a step into a hole is a lost turn. The shape alone sets the pace. Distance grows like a power of time whose exponent the shape sets, which [the walk dimension note](/research/walks/) measures.',
  'Code A walks the left board, the top one on a tall screen, in the accent colour; code B walks the other in the opposite hue. The dashed ring is the finish and the solid ring is how far the swarm has spread.',
  'Space, Enter or a tap on the boards starts, pauses and races again on the next seed. Reroll is a new walk on the same pair, n rolls a new pair of one mass, and the codes take any two designs.',
];

export const LABELS = { ready: 'Start', running: 'Pause', held: 'Resume', done: 'Race again' };

export const named = (value) => `race-${value.a}-${value.b}-${value.seed}`;

export const PAGE = {
  spec: SPEC,
  record: true,
  about: ABOUT,
  name: named,
  keys: [
    { key: 'Enter', label: (value, scene) => LABELS[scene?.phase?.() ?? 'ready'], act: 'go', button: true },
    { key: 'n', label: 'New pair', act: 'pair' },
  ],
  actions: { go: (scene) => scene.go?.(), pair: (scene) => scene.pair?.() },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const PAD = 16;
const GAP = 16;
const TOP = 0.55;
const RING = 0.45;
const DASH = [4, 6];
const LINE = 1.5;
const WIN = 3;
const MARK = 2;
const TOLD = 100;
const BOARD = ['a', 'b', 'base', 'number', 'level'];

const mix = (h) => TOP * Math.min(1, h / HOT);

const won = (over, s) => over?.how === 'goal' && over.winner === s;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('race: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const hands = !view.lock;
  let value = tidy(SPEC, opts);
  let seed = opts.seed;
  let laid = null;
  let walk = 0;
  let n = 0;
  let sheets = [];
  let show = null;
  let phase = 'ready';
  let at = 0;
  let clock = { t: 0, tick: 0 };
  let seen = -1;
  let dirty = true;
  let box = null;
  let colours = [];
  let bytes = [];
  let ink = null;
  let ground = [];
  let told = -Infinity;
  let said = '';
  let gone = false;
  const paint = () => {
    const { accent } = view.look();
    const css = typeof getComputedStyle === 'function' && canvas ? getComputedStyle(canvas) : null;
    const read = (name, fallback) => (css && css.getPropertyValue(name).trim()) || fallback;
    colours = [accent, turn(accent, TURN)];
    bytes = colours.map(rgb);
    ink = { fg: read('--fg', light.fg), line: read('--line', light.line), deep: read('--deep', light.deep) };
    ground = [rgb(ink.deep), rgb(ink.line)];
    dirty = true;
  };
  const stat = () => {
    const { state } = show;
    return { phase, tick: state.tick, reach: [...state.reach], over: state.over, goal: laid.goal, side: n, walkers: laid.walkers, sides: laid.sides.map(({ code, name, fills, of, lit, cells, dimension }) => ({ code, name, fills, of, lit, cells, dimension })) };
  };
  const tell = (force) => {
    if (gone || !hands) return;
    if (!force && phase === said && view.t - told < TOLD) return;
    told = view.t;
    said = phase;
    opts.onTell?.(stat());
  };
  const restart = () => {
    show = series(laid, walk, hands ? Infinity : HOLD);
    phase = hands ? 'ready' : 'running';
    at = 0;
    clock = { t: hands ? view.t : 0, tick: 0 };
    seen = -1;
    dirty = true;
  };
  const replan = () => {
    const seeds = roots(rng(seed));
    walk = seeds.walk;
    laid = plan(math, hands ? value : { ...value, a: opts.a ?? '', b: opts.b ?? '' }, seeds.code);
    if (laid.side !== n) {
      n = laid.side;
      sheets = laid.sides.map(() => {
        const page = new OffscreenCanvas(n, n);
        return { page, pen: page.getContext('2d'), image: new ImageData(n, n), hot: new Float32Array(n * n) };
      });
    }
    box = null;
    restart();
  };
  const anchor = () => {
    clock = { t: view.t, tick: at };
  };
  const play = () => {
    if (phase !== 'ready' && phase !== 'held') return;
    anchor();
    phase = 'running';
    dirty = true;
    view.wake?.();
  };
  const go = () => {
    if (phase === 'running') phase = 'held';
    else play();
    dirty = true;
    tell(true);
    return phase;
  };
  const retarget = () => {
    const was = show.state.over;
    aim(show.state, laid);
    if (was && !show.state.over && phase === 'done') {
      phase = 'running';
      anchor();
    }
    if (!was && show.state.over && hands) phase = 'done';
  };
  const level = (s) => {
    const { state } = show;
    const { filled } = laid.sides[s];
    const trail = state.trail[s];
    const stamp = state.stamp[s];
    const out = sheets[s].hot;
    for (let i = 0; i < out.length; i++) out[i] = filled[i] ? heat(trail[i], stamp[i], state.tick, laid.keep) : 0;
    return out;
  };
  const blit = (s) => {
    const { image, pen } = sheets[s];
    const data = image.data;
    const { filled } = laid.sides[s];
    const [r, g, b] = bytes[s];
    const hot = level(s);
    for (let i = 0; i < hot.length; i++) {
      const [br, bg, bb] = ground[filled[i]];
      const k = filled[i] ? mix(hot[i]) : 0;
      const o = i * 4;
      data[o] = br + (r - br) * k;
      data[o + 1] = bg + (g - bg) * k;
      data[o + 2] = bb + (b - bb) * k;
      data[o + 3] = 255;
    }
    for (const i of show.state.at[s]) data.set([r, g, b, 255], i * 4);
    pen.putImageData(image, 0, 0);
  };
  const rings = (s) => {
    const { state } = show;
    const { home } = laid.sides[s];
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
    ctx.arc(hx, hy, laid.goal * cell, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.strokeStyle = colours[s];
    if (state.reach[s] > 0) {
      ctx.lineWidth = (won(state.over, s) ? WIN : LINE) * dpr;
      ctx.beginPath();
      ctx.arc(hx, hy, state.reach[s] * cell, 0, TAU);
      ctx.stroke();
    }
    const m = cell + 4 * dpr;
    ctx.strokeStyle = ink.fg;
    ctx.lineWidth = MARK * dpr;
    ctx.strokeRect(hx - m / 2, hy - m / 2, m, m);
    ctx.restore();
  };
  const draw = () => {
    if (hands && view.fixed && phase === 'ready') play();
    if (view.still && !hands) {
      if (!show.state.over) show.finish(laid.still);
    } else if (phase === 'running') {
      at = Math.max(at, clock.tick + Math.floor(((view.t - clock.t) * value.speed) / 1000));
      show.to(at);
      if (hands && show.state.over) phase = 'done';
    }
    if (!dirty && seen === show.version) return;
    seen = show.version;
    dirty = false;
    const dpr = view.dpr;
    box ??= lay(view.w, view.h, n, GAP * dpr, PAD * dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = false;
    laid.sides.forEach((_, s) => {
      blit(s);
      const { x, y } = box.boards[s];
      ctx.drawImage(sheets[s].page, x, y, box.edge, box.edge);
      rings(s);
    });
    tell(Boolean(show.state.over) || show.state.tick === 0);
  };
  const set = (next) => {
    const was = value;
    value = tidy(SPEC, next);
    const moved = next.seed !== seed;
    seed = next.seed;
    paint();
    if (moved || BOARD.some((key) => value[key] !== was[key])) {
      replan();
      if (moved) play();
    } else {
      if (value.walkers !== was.walkers) {
        laid.walkers = value.walkers;
        laid.still = budget(value.walkers);
        restart();
      }
      if (value.speed !== was.speed || value.heat !== was.heat) {
        bake(show.state, laid.keep);
        laid.keep = keeping(value.heat, value.speed);
        laid.speed = value.speed;
        anchor();
      }
      if (value.finish !== was.finish) {
        laid.goal = value.finish;
        retarget();
      }
    }
    dirty = true;
    tell(true);
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
    const c = css.cell;
    const body = laid.sides.map(({ filled, home }, s) => {
      const { x, y } = css.boards[s];
      const hot = level(s);
      const base = runs({ shape: [n, n], types: filled }).map(([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`);
      const warm = [];
      for (let i = 0; i < hot.length; i++) if (hot[i] > 0) warm.push(`<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1" fill-opacity="${num(mix(hot[i]), 3)}"/>`);
      const dots = Array.from(state.at[s], (i) => `<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1"/>`);
      const cells = `<g transform="translate(${num(x)} ${num(y)}) scale(${num(c, 4)})" shape-rendering="crispEdges"><rect width="${n}" height="${n}" fill="${ink.deep}"/><g fill="${ink.line}">${base.join('')}</g><g fill="${colours[s]}">${warm.join('')}${dots.join('')}</g></g>`;
      if (home < 0) return cells;
      const hx = x + ((home % n) + 0.5) * c;
      const hy = y + (Math.floor(home / n) + 0.5) * c;
      const ring = (r, width, dash) => `<circle cx="${num(hx)}" cy="${num(hy)}" r="${num(r)}" fill="none" stroke="${colours[s]}" stroke-width="${num(width)}"${dash ? ` stroke-dasharray="${DASH.join(' ')}" stroke-opacity="${RING}"` : ''}/>`;
      const m = c + 4;
      const mark = `<rect x="${num(hx - m / 2)}" y="${num(hy - m / 2)}" width="${num(m)}" height="${num(m)}" fill="none" stroke="${ink.fg}" stroke-width="${MARK}"/>`;
      return `${cells}<clipPath id="race-${s}"><rect x="${num(x)}" y="${num(y)}" width="${num(css.edge)}" height="${num(css.edge)}"/></clipPath><g clip-path="url(#race-${s})">${ring(laid.goal * c, 1, true)}${state.reach[s] > 0 ? ring(state.reach[s] * c, won(state.over, s) ? WIN : LINE, false) : ''}${mark}</g>`;
    });
    return sheet(view.w / dpr, view.h / dpr, body.join(''));
  };
  const csv = () => ['step,a,b', ...show.state.log.map(([a, b], i) => `${i},${num(a, 3)},${num(b, 3)}`)].join('\n');
  paint();
  replan();
  return { draw, size, theme, stop, set, svg, csv, stat, go, toggle: go, playing: () => phase === 'running', phase: () => phase };
}
