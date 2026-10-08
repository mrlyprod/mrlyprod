import { tidy } from '../../lib/knobs.js';
import { TINTS, board, pick, veil } from '../../lib/scene.js';
import { DESIGNS, NUMBERS, ink, tile } from '../../lib/tiles.js';
import { DOWN, LEFT, RIGHT, UP, rounds, starve, start, step, turn } from './engine.js';
import { STARVED, pick as choose } from './policy.js';

const PARTS = ['head', 'body', 'food'];

const named = (word) => word[0].toUpperCase() + word.slice(1);

const PIECES = [['', 'Random'], ...DESIGNS.map((design) => [design, named(design)])];

export const SPEC = [
  { key: 'size', label: 'Size', kind: 'slider', def: 16, min: 8, max: 32, step: 2, group: 'Board' },
  { key: 'wrap', label: 'Wrap', kind: 'toggle', def: 1, group: 'Board' },
  { key: 'apples', label: 'Apples', kind: 'slider', def: 1, min: 1, max: 10, step: 1, group: 'Board' },
  { key: 'tick', label: 'Tick', kind: 'slider', def: 150, min: 50, max: 500, step: 25, unit: 'ms', group: 'Pace' },
  { key: 'play', label: 'Player', kind: 'segment', def: 'me', options: [['me', 'Me'], ['smart', 'Smart'], ['silly', 'Silly']], group: 'Play' },
  ...PARTS.map((part) => ({ key: part, label: named(part), kind: 'pick', def: '', options: PIECES, group: 'Pieces' })),
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

const ARROWS = { ArrowUp: UP, ArrowDown: DOWN, ArrowLeft: LEFT, ArrowRight: RIGHT };

const WASD = { w: UP, a: LEFT, s: DOWN, d: RIGHT };

const steer = (scene, e) => scene.turn({ ...ARROWS, ...WASD }[e.key.length === 1 ? e.key.toLowerCase() : e.key]);

export const PAGE = {
  spec: SPEC,
  gestures: true,
  keys: [
    { key: Object.keys(ARROWS), label: 'Turn', act: 'arrows' },
    { key: 'r', label: 'New game', act: 'reroll' },
    { key: '?', label: 'Keys', act: 'keys' },
    { key: Object.keys(WASD), label: 'Turn', act: 'wasd' },
  ],
  actions: { arrows: steer, wasd: steer },
};

const TAU = Math.PI * 2;
const HOLD = 1000;
const FIT = 0.92;
const GROUND = 0.07;
const DIM = 0.35;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const value = tidy(SPEC, opts);
  const play = value.play;
  const auto = play !== 'me' || !opts.player;
  let state;
  let marks;
  let ended = 0;
  let last = view.t;
  let gone = false;
  const tell = (name, arg) => !gone && opts[name]?.(arg);
  const begin = () => {
    state = start(value, rand);
    marks = Object.fromEntries(PARTS.map((part) => {
      const design = pick(rand, DESIGNS);
      return [part, tile(value[part] || design, pick(rand, NUMBERS))];
    }));
    ended = 0;
    tell('onScore', state.score);
    tell('onOver', state.over);
  };
  const advance = () => {
    if (state.over) {
      if (auto && view.t - ended >= HOLD) begin();
      return;
    }
    const score = state.score;
    if (auto && !state.queue.length) {
      const dir = choose(state, play, rand);
      state = dir === STARVED ? starve(state) : turn(state, dir);
    }
    state = step(state, rand);
    if (state.score !== score) tell('onScore', state.score);
    if (!state.over) return;
    ended = view.t;
    tell('onOver', state.over);
  };
  const paint = () => {
    const n = state.size;
    const { x: left, y: top, w: side, cell } = board(view, n, n, FIT);
    const { accent } = view.look();
    const piece = (mark, at, corners) => {
      const x = left + (at % n) * cell;
      const y = top + Math.floor(at / n) * cell;
      ctx.save();
      ctx.beginPath();
      if (corners) ctx.roundRect(x, y, cell, cell, corners.map((one) => (one * cell) / 2));
      else ctx.arc(x + cell / 2, y + cell / 2, cell / 2, 0, TAU);
      ctx.clip();
      ink(ctx, mark, x, y, cell);
      ctx.restore();
    };
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = 1;
    ctx.fillStyle = veil(accent, GROUND);
    ctx.fillRect(left, top, side, side);
    ctx.globalAlpha = state.over ? DIM : 1;
    ctx.fillStyle = accent;
    for (const at of state.foods) piece(marks.food, at, null);
    const ends = rounds(state);
    state.snake.forEach((at, i) => piece(i ? marks.body : marks.head, at, ends[i]));
  };
  const draw = () => {
    if (!view.still && view.t !== last) advance();
    last = view.t;
    paint();
  };
  const restart = () => {
    begin();
    paint();
  };
  const stop = () => {
    gone = true;
  };
  const move = (dir) => {
    state = turn(state, dir);
    if (!auto) view.wake?.();
  };
  begin();
  paint();
  return { draw, every: value.tick, size: paint, theme: paint, turn: move, restart, stop };
}
