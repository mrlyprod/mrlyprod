import type { Ready } from "@mrlyprod/ui/kit/hooks.ts";
import type { Fired } from "@mrlyprod/ui/kit/keys.tsx";
import type { Handle, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import {
  arena,
  clear,
  DOWN,
  ground,
  heading,
  inks,
  LEFT,
  lower,
  RIGHT,
  swiper,
  TURNS,
  UP,
} from "../shared/games/stage.ts";
import type { Act, State, Tile } from "./engine.ts";
import { COLS, CRUSH, DROP, deal, ROWS, shift, step, TICK } from "./engine.ts";

export type Crush = ReturnType<typeof make>;

const EVERY = 250;
const DIM = 0.35;
const GAP = 0.04;

const steer = (scene: Handle<Crush>, e: Fired) => scene.steer(e.key);

export const PAGE = {
  gestures: true,
  keys: [
    { key: ["ArrowLeft", "ArrowRight", "a", "d"], label: "Move", act: "shift" },
    { key: ["ArrowUp", "w"], label: "Crush", act: "crush" },
    { key: ["ArrowDown", "s"], label: "Drop", act: "drop" },
    { key: "r", label: "New game", act: "reroll" },
    { key: "?", label: "Keys", act: "keys" },
  ],
  actions: { shift: steer, crush: steer, drop: steer },
} satisfies Page<Ready & Units, Crush>;

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let state: State = deal({}, view);
  let last = view.t;
  let told = "";
  let spent = false;
  const said = () => (state.over ? "game over" : "crushed");
  const tell = () => {
    const now = `${said()} ${state.crushed}`;
    if (now === told) return;
    told = now;
    opts.tell?.({ rows: [[said(), state.crushed]], over: state.over });
  };
  const paint = () => {
    const box = arena(view, COLS, ROWS);
    const tone = inks(canvas, view);
    const inset = Math.round(box.cell * GAP);
    const piece = (tile: Tile, col: number, row: number) => {
      ctx.fillStyle = tile.ripe ? tone.ink : tone.hue(tile.hue);
      stamp(ctx, tile, box.x + col * box.cell + inset, box.y + row * box.cell + inset, box.cell - 2 * inset);
    };
    clear(ctx, view);
    heading(ctx, canvas, view, box, told, tone.ink);
    ctx.fillStyle = ground(tone);
    ctx.fillRect(box.x, box.y, box.w, box.h);
    ctx.globalAlpha = state.over ? DIM : 1;
    for (const [at, tile] of state.grid.entries()) if (tile) piece(tile, at % COLS, Math.floor(at / COLS));
    if (state.falling) piece(state.falling.tile, state.falling.x, state.falling.y);
    ctx.globalAlpha = 1;
    view.drew();
  };
  const play = (act: Act) => {
    state = step(state, act);
    tell();
    paint();
    view.wake?.();
  };
  const turn = (dir: number) => {
    if (dir === LEFT) play(shift(-1));
    if (dir === RIGHT) play(shift(1));
    if (dir === UP) play(CRUSH);
    if (dir === DOWN) play(DROP);
  };
  const swipe = swiper(view, (dir) => {
    const once = dir === UP || dir === DOWN;
    if (once && spent) return;
    spent = once;
    turn(dir);
  });
  const touch = (u: number, v: number, start: boolean) => {
    if (start) spent = false;
    return swipe.touch(u, v, start);
  };
  const steer = (key?: string) => turn(TURNS[lower(key)]);
  const draw = () => {
    if (!view.still && view.t !== last) {
      state = step(state, TICK);
      tell();
    }
    last = view.t;
    paint();
  };
  const restart = () => {
    state = deal({}, view);
    tell();
    paint();
  };
  tell();
  paint();
  return { draw, every: EVERY, size: paint, theme: paint, restart, steer, touch, lift: swipe.lift };
}
