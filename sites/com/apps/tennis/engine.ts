import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Drawn, Sprite } from "../shared/games/sprites.ts";
import { below, shuffled, sprite } from "../shared/games/sprites.ts";

export type Ball = { x: number; y: number; dx: number; dy: number };

export type Spot = { x: number; y: number };

export type Block = Spot & { face: Sprite };

export type Over = "missed" | "landed" | null;

export type Act =
  | { verb: "tick" }
  | { verb: "slide"; args: { dx: number; dy: number } }
  | { verb: "aim"; args: { x: number; y: number } };

export type State = {
  ball: Ball;
  paddle: Spot;
  blocks: Block[];
  faces: { ball: Sprite; paddle: Sprite };
  score: number;
  hits: number;
  due: number;
  over: Over;
  seed: number;
};

export const SIZE = 18;
export const WIDE = 5;
export const BALL = 1;
export const BLOCK = 2;
export const FPS = 32;
export const GLIDE = 1 / 2;
export const TICK: Act = { verb: "tick" };

const SPEED = 1 / 3;
const RUSH = 1 / 2;
const ANGLE = 32;
const SERVE = 30;
const PER_ROW = SIZE / BLOCK;
const ROWS = 4;
const MOST = 12;
const LEAST = 4;
const EASE = 20;
const DENSE = 1 / 2;
const LOW = SIZE - 1;
const HIGH = Math.floor(SIZE * 0.6);
const LANDS = LOW - 1;
const NUDGE = 0.001;
const EDGE = 0.05;
const GRAZE = 0.1;
const WIDTH = 4294967296;

const clamp = (v: number, low: number, high: number) => Math.min(high, Math.max(low, v));

const rad = (deg: number) => (deg * Math.PI) / 180;

const COLS = Array.from({ length: PER_ROW }, (_, col) => col);

function row(y: number, seed: number, cols: readonly number[]): Drawn<Block[]> {
  let at = seed;
  const blocks = cols.map((col) => {
    const [face, next] = sprite(at);
    at = next;
    return { x: col * BLOCK, y, face };
  });
  return [blocks, at];
}

function partial(y: number, seed: number): Drawn<Block[]> {
  const least = Math.ceil(PER_ROW * DENSE);
  const [extra, a] = below(seed, PER_ROW - least + 1);
  const [cols, b] = shuffled(a, COLS);
  return row(
    y,
    b,
    cols.slice(0, least + extra).sort((p, q) => p - q),
  );
}

function pace(score: number, seed: number): Drawn<number> {
  const base = Math.max(LEAST, MOST - Math.floor(score / EASE));
  const [jitter, next] = below(seed, 3);
  return [Math.max(LEAST, base + jitter - 1), next];
}

function lowered(blocks: readonly Block[], seed: number): Drawn<Block[]> {
  const shifted = blocks.map((one) => ({ ...one, y: one.y + 1 }));
  const top = shifted.length ? Math.min(...shifted.map((one) => one.y)) : 1;
  const [fresh, next] = partial(top - 1, seed);
  return [[...fresh, ...shifted], next];
}

function flown({ x, y, dx, dy }: Ball): Ball {
  const out = { x: x + dx, y: y + dy, dx, dy };
  if (out.x <= 0) {
    out.x = 0;
    out.dx = Math.abs(dx);
  } else if (out.x + BALL >= SIZE) {
    out.x = SIZE - BALL;
    out.dx = -Math.abs(dx);
  }
  if (out.y <= 0) {
    out.y = 0;
    out.dy = Math.abs(dy);
  }
  return out;
}

function touching({ x, y, dy }: Ball, paddle: Spot): boolean {
  return y + BALL >= paddle.y && y < paddle.y + 1 && x + BALL > paddle.x && x < paddle.x + WIDE && dy > 0;
}

function returned(ball: Ball, paddle: Spot): Ball {
  const hit = clamp((ball.x + BALL / 2 - (paddle.x + WIDE / 2)) / (WIDE / 2), -1, 1);
  const speed = SPEED * (1 + Math.abs(hit) * RUSH);
  const angle = hit * rad(ANGLE);
  return { x: ball.x, y: paddle.y - BALL, dx: speed * Math.sin(angle), dy: -speed * Math.cos(angle) };
}

function deflected(ball: Ball, paddle: Spot): Ball {
  const way = ball.x + BALL / 2 < paddle.x + WIDE / 2 ? -1 : 1;
  return { ...ball, dx: way * Math.abs(ball.dx || SPEED) };
}

const overlaps = (one: Spot, { x, y }: Spot) =>
  x < one.x + BLOCK && x + BALL > one.x && y < one.y + 1 && y + BALL > one.y;

const reach = (one: Spot, { x, y }: Spot) =>
  (x + BALL / 2 - (one.x + BLOCK / 2)) ** 2 + (y + BALL / 2 - (one.y + 1 / 2)) ** 2;

function struck(blocks: readonly Block[], ball: Ball, from: Ball): Block | null {
  const hit = blocks.filter((one) => overlaps(one, ball));
  return hit.length ? hit.reduce((best, one) => (reach(one, from) < reach(best, from) ? one : best)) : null;
}

function bounced(ball: Ball, from: Ball, one: Spot): Ball {
  const { x, y, dx, dy } = ball;
  const top = one.y - BALL - NUDGE;
  const bottom = one.y + 1 + NUDGE;
  const left = one.x - BALL - NUDGE;
  const right = one.x + BLOCK + NUDGE;
  if (from.y + BALL <= one.y + EDGE && dy > 0) return { ...ball, dy: -dy, y: top };
  if (from.y >= one.y + 1 - EDGE && dy < 0) return { ...ball, dy: -dy, y: bottom };
  if (from.x + BALL <= one.x + EDGE && dx > 0) return { ...ball, dx: -dx, x: left };
  if (from.x >= one.x + BLOCK - EDGE && dx < 0) return { ...ball, dx: -dx, x: right };
  const cx = x + BALL / 2;
  const cy = y + BALL / 2;
  const bx = one.x + BLOCK / 2;
  const by = one.y + 1 / 2;
  const across = BALL / 2 + BLOCK / 2 - Math.abs(cx - bx);
  const down = BALL / 2 + 1 / 2 - Math.abs(cy - by);
  if (across < down) return { ...ball, dx: -dx, x: cx < bx ? left : right };
  return { ...ball, dy: -dy, y: cy < by ? top : bottom };
}

function tick(state: State): State {
  if (state.over) return state;
  const { ball: from, paddle } = state;
  let ball = flown(from);
  let met = false;
  if (touching(ball, paddle)) {
    met = from.y + BALL <= paddle.y + Math.abs(ball.dy) + GRAZE;
    ball = met ? returned(ball, paddle) : deflected(ball, paddle);
  }
  const hit = struck(state.blocks, ball, from);
  if (hit) ball = bounced(ball, from, hit);
  const blocks = hit ? state.blocks.filter((one) => one !== hit) : state.blocks;
  const score = state.score + (hit ? 1 : 0);
  if (ball.y > SIZE) return { ...state, ball, blocks, score, over: "missed" };
  const hits = state.hits + (met ? 1 : 0);
  if (hits < state.due) return { ...state, ball, blocks, score, hits };
  const [wall, a] = lowered(blocks, state.seed);
  const [due, seed] = pace(score, a);
  const over = wall.some((one) => one.y >= LANDS) ? "landed" : null;
  return { ...state, ball, blocks: wall, score, hits: 0, due, seed, over };
}

const placed = (x: number, y: number): Spot => ({ x: clamp(x, 0, SIZE - WIDE), y: clamp(y, HIGH, LOW) });

export function step(state: State, act: Act): State {
  if (state.over) return state;
  if (act.verb === "tick") return tick(state);
  if (act.verb === "aim") return { ...state, paddle: placed(act.args.x, act.args.y) };
  return { ...state, paddle: placed(state.paddle.x + act.args.dx, state.paddle.y + act.args.dy) };
}

function start(given: number): State {
  let seed = mix(given);
  const blocks: Block[] = [];
  for (let y = 0; y < ROWS; y++) {
    const [line, next] = row(y, seed, COLS);
    blocks.push(...line);
    seed = next;
  }
  const [one, a] = sprite(seed);
  const [two, b] = sprite(a);
  const [lean, c] = below(b, 2 * SERVE + 1);
  const [due, d] = pace(0, c);
  const angle = rad(lean - SERVE);
  const x = (SIZE - WIDE) / 2;
  return {
    ball: {
      x: x + WIDE / 2 - BALL / 2,
      y: LOW - BALL,
      dx: SPEED * Math.sin(angle),
      dy: -SPEED * Math.cos(angle),
    },
    paddle: { x, y: LOW },
    blocks,
    faces: { ball: one, paddle: two },
    score: 0,
    hits: 0,
    due,
    over: null,
    seed: d,
  };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => start(Math.floor(rand() * WIDTH));
