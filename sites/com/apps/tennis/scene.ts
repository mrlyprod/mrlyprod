import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import { arena, clear, ground, heading, holder, inks, lower, steered, TURNS } from "../shared/games/stage.ts";
import type { Over, State } from "./engine.ts";
import { BALL, BLOCK, deal, FPS, GLIDE, SIZE, step, TICK, WIDE } from "./engine.ts";

export type Tennis = ReturnType<typeof make>;

const SAID: Record<NonNullable<Over>, string> = { missed: "missed", landed: "the wall landed" };
const WAYS = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];
const DIM = 0.35;
const TAU = Math.PI * 2;

export const PAGE = steered<Tennis>();

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const keys = holder();
  let state: State = deal({}, view);
  let tone = inks(canvas, view);
  let last = view.t;
  const said = () => (state.over ? `${SAID[state.over]}: ${state.score}` : `bricks ${state.score}`);
  const tell = () => opts.tell?.({ rows: [["bricks", state.score]], over: Boolean(state.over) });
  const paint = () => {
    const box = arena(view, SIZE, SIZE);
    const { x: left, y: top, cell } = box;
    const mark = (face: Sprite, col: number, row: number) => {
      ctx.fillStyle = tone.hue(face.hue);
      stamp(ctx, face, left + col * cell, top + row * cell, cell);
    };
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    ctx.fillStyle = ground(tone);
    ctx.fillRect(left, top, box.w, box.h);
    ctx.globalAlpha = state.over ? DIM : 1;
    for (const one of state.blocks) for (let i = 0; i < BLOCK; i++) mark(one.face, one.x + i, one.y);
    const { paddle, ball, faces } = state;
    ctx.save();
    ctx.beginPath();
    ctx.roundRect(left + paddle.x * cell, top + paddle.y * cell, WIDE * cell, cell, cell / 2);
    ctx.clip();
    for (let i = 0; i < WIDE; i++) mark(faces.paddle, paddle.x + i, paddle.y);
    ctx.restore();
    ctx.save();
    ctx.beginPath();
    ctx.arc(left + (ball.x + BALL / 2) * cell, top + (ball.y + BALL / 2) * cell, (BALL * cell) / 2, 0, TAU);
    ctx.clip();
    ctx.fillStyle = tone.ink;
    ctx.fillRect(left + ball.x * cell, top + ball.y * cell, cell, cell);
    mark(faces.ball, ball.x, ball.y);
    ctx.restore();
    ctx.globalAlpha = 1;
  };
  const slide = (key: string | null) => {
    const way = key ? WAYS[TURNS[key]] : null;
    if (way) state = step(state, { verb: "slide", args: { dx: way[0] * GLIDE, dy: way[1] * GLIDE } });
  };
  const advance = () => {
    const { score, over } = state;
    slide(keys.top());
    state = step(state, TICK);
    if (state.score !== score || state.over !== over) tell();
  };
  const draw = () => {
    if (!view.still && view.t !== last) advance();
    last = view.t;
    paint();
  };
  const steer = (key?: string) => {
    if (keys.top() !== lower(key)) slide(lower(key));
    keys.press(key);
    view.wake?.();
  };
  const touch = (u: number, v: number) => {
    const { x, y, cell } = arena(view, SIZE, SIZE);
    state = step(state, {
      verb: "aim",
      args: { x: (u * view.w - x) / cell - WIDE / 2, y: (v * view.h - y) / cell - 1 / 2 },
    });
    view.wake?.();
    paint();
    return null;
  };
  const restart = () => {
    state = deal({}, view);
    tell();
    paint();
  };
  const theme = () => {
    tone = inks(canvas, view);
    paint();
  };
  tell();
  paint();
  return {
    draw,
    every: 1000 / FPS,
    size: paint,
    theme,
    restart,
    steer,
    touch,
    stop: keys.stop,
  };
}
