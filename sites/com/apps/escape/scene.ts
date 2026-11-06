import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import { arena, clear, ground, heading, holder, inks, lower, steered, swiper, TURNS } from "../shared/games/stage.ts";
import type { State } from "./engine.ts";
import { DOOR, deal, LAYOUTS, SIDE, step, TICK, TICK_MS, WALL } from "./engine.ts";

export type Escape = ReturnType<typeof make>;

const DIM = 0.35;
const DOT = 1 / 6;
const OPEN = 0.45;
const RING = 0.08;
const TAU = Math.PI * 2;

export const PAGE = steered<Escape>();

const design = (near: (dx: number, dy: number) => boolean) => {
  if (near(0, -1) && near(0, 1)) return "vtree";
  return near(-1, 0) && near(1, 0) ? "htree" : "carpet";
};

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const keys = holder();
  let state: State = deal({}, view);
  let tone = inks(canvas, view);
  let last = view.t;
  let swiped = -1;
  const said = () => {
    if (state.over) return `caught on level ${state.level}`;
    return state.ending === "door" ? "out" : `level ${state.level}`;
  };
  const tell = () =>
    opts.tell?.({
      rows: [
        ["level", state.level],
        ["dots", state.dots.length],
      ],
      over: state.over,
    });
  const paint = () => {
    const box = arena(view, SIDE, SIDE);
    const { x: left, y: top, cell } = box;
    const cells = LAYOUTS[state.layout];
    const locked = state.dots.length > 0;
    const solid = (at: number) => cells[at] === WALL || (cells[at] === DOOR && locked);
    const place = (at: number): [number, number] => [left + (at % SIDE) * cell, top + Math.floor(at / SIDE) * cell];
    const mark = (face: Sprite, at: number) => {
      const [x, y] = place(at);
      ctx.fillStyle = tone.hue(face.hue);
      stamp(ctx, face, x, y, cell);
    };
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    ctx.fillStyle = ground(tone);
    ctx.fillRect(left, top, box.w, box.h);
    ctx.globalAlpha = state.over ? DIM : 1;
    for (const [at, kind] of cells.entries()) {
      const [x, y] = place(at);
      if (kind === DOOR && !locked) {
        ctx.fillStyle = veil(tone.accent, OPEN);
        ctx.fillRect(x, y, cell, cell);
      }
      if (!solid(at)) continue;
      const col = at % SIDE;
      const row = Math.floor(at / SIDE);
      const near = (dx: number, dy: number) => {
        const c = col + dx;
        const r = row + dy;
        return c >= 0 && c < SIDE && r >= 0 && r < SIDE && solid(r * SIDE + c);
      };
      ctx.fillStyle = tone.ink;
      stamp(ctx, { design: design(near), number: state.faces.number }, x, y, cell);
    }
    ctx.fillStyle = tone.ink;
    for (const at of state.dots) {
      const [x, y] = place(at);
      ctx.beginPath();
      ctx.arc(x + cell / 2, y + cell / 2, cell * DOT, 0, TAU);
      ctx.fill();
    }
    for (const [i, at] of state.hunters.entries()) mark(state.faces.hunters[i], at);
    mark(state.faces.runner, state.runner);
    const [x, y] = place(state.runner);
    const width = Math.max(2, Math.round(cell * RING));
    ctx.strokeStyle = tone.accent;
    ctx.lineWidth = width;
    ctx.strokeRect(x + width / 2, y + width / 2, cell - width, cell - width);
    ctx.globalAlpha = 1;
  };
  const go = (dir: number | undefined) => {
    if (dir !== undefined && dir >= 0) state = step(state, { verb: "go", args: { dir } });
  };
  const advance = () => {
    const { level, over, dots } = state;
    const key = keys.top();
    go(key ? TURNS[key] : swiped);
    state = step(state, TICK);
    if (state.level !== level || state.over !== over || state.dots.length !== dots.length) tell();
  };
  const draw = () => {
    if (!view.still && view.t !== last) advance();
    last = view.t;
    paint();
  };
  const steer = (key?: string) => {
    go(TURNS[lower(key)]);
    keys.press(key);
    view.wake?.();
  };
  const swipes = swiper(view, (dir) => {
    swiped = dir;
    go(dir);
    view.wake?.();
  });
  const lift = () => {
    swiped = -1;
    return swipes.lift();
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
    every: TICK_MS,
    size: paint,
    theme,
    restart,
    steer,
    touch: swipes.touch,
    lift,
    stop: keys.stop,
  };
}
