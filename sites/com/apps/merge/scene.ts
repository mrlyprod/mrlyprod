import { blend, bright } from "@mrlyprod/ui/kit/color.ts";
import { glyph } from "@mrlyprod/ui/kit/font/font.ts";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Given } from "../shared/games/stage.ts";
import {
  arena,
  clear,
  ground,
  heading,
  inks,
  lower,
  pixels,
  slab,
  steered,
  swiper,
  TURNS,
} from "../shared/games/stage.ts";
import type { State } from "./engine.ts";
import { deal, SIZE, step } from "./engine.ts";

export type Merge = ReturnType<typeof make>;

const TOP = 11;
const FLOOR = 0.2;
const MARGIN = 0.14;
const RING = 0.04;

export const PAGE = steered<Merge>("Slide");

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const words = new Map<number, string[]>();
  let state: State = deal({}, view);
  let spent = false;
  const word = (value: number) => {
    if (!words.has(value)) {
      const letters = [...String(value)].map(glyph);
      words.set(
        value,
        letters[0].map((_, row) => letters.map((rows) => rows[row]).join("0")),
      );
    }
    return words.get(value)!;
  };
  const said = () => (state.over ? "game over" : "score");
  const tell = () => opts.tell?.({ rows: [[said(), state.score]], over: state.over });
  const paint = () => {
    const box = arena(view, SIZE, SIZE);
    const tone = inks(canvas, view);
    clear(ctx, view);
    heading(ctx, canvas, view, box, `${said()} ${state.score}`, tone.ink);
    for (const [at, value] of state.cells.entries()) {
      if (!value) {
        slab(ctx, box, SIZE, at, ground(tone));
        continue;
      }
      const tint = blend(tone.paper, tone.accent, FLOOR + (1 - FLOOR) * Math.min(1, Math.log2(value) / TOP));
      const [x, y, side] = slab(ctx, box, SIZE, at, tint);
      const inset = side * MARGIN;
      ctx.fillStyle = bright(tint) === bright(tone.ink) ? tone.paper : tone.ink;
      pixels(ctx, word(value), x + inset, y + inset, side - 2 * inset);
      if (at !== state.fresh) continue;
      const width = Math.max(2, Math.round(side * RING));
      ctx.strokeStyle = tone.ink;
      ctx.lineWidth = width;
      ctx.strokeRect(x + width / 2, y + width / 2, side - width, side - width);
    }
    view.drew();
  };
  const slide = (dir: number) => {
    const next = step(state, { verb: "slide", args: { dir } });
    if (next === state) return;
    state = next;
    tell();
    paint();
  };
  const swipe = swiper(view, (dir) => {
    if (spent) return;
    spent = true;
    slide(dir);
  });
  const touch = (u: number, v: number, start: boolean) => {
    if (start) spent = false;
    return swipe.touch(u, v, start);
  };
  const steer = (key?: string) => slide(TURNS[lower(key)]);
  const restart = () => {
    state = deal({}, view);
    tell();
    paint();
  };
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, steer, touch, lift: swipe.lift };
}
