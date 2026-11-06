import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import { aimed, arena, cellAt, clear, cursor, ground, heading, inks, slab } from "../shared/games/stage.ts";
import type { Over, State } from "./engine.ts";
import { deal, SIDE, step } from "./engine.ts";

export type Ttt = ReturnType<typeof make>;

const SAID: Record<NonNullable<Over>, string> = { won: "you win", lost: "you lose", draw: "draw" };
const MARGIN = 0.1;
const SHINE = 0.3;

export const PAGE = aimed<Ttt>();

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const aim = cursor(SIDE, SIDE);
  let state: State = deal({}, view);
  const said = () => (state.over ? SAID[state.over] : "your move");
  const tell = () => opts.tell?.({ rows: [["game", said()]], over: Boolean(state.over) });
  const paint = () => {
    const box = arena(view, SIDE, SIDE);
    const tone = inks(canvas, view);
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    for (const [at, one] of state.cells.entries()) {
      const [x, y, side] = slab(ctx, box, SIDE, at, state.line.includes(at) ? veil(tone.accent, SHINE) : ground(tone));
      if (one === null) continue;
      const face = state.faces[one];
      const inset = side * MARGIN;
      ctx.fillStyle = tone.hue(face.hue);
      stamp(ctx, face, x + inset, y + inset, side - 2 * inset);
    }
    aim.ring(ctx, box, tone.accent);
    view.drew();
  };
  const place = (at: number) => {
    state = step(state, { verb: "place", args: { at } });
    tell();
    paint();
  };
  const restart = () => {
    state = deal({}, view);
    tell();
    paint();
  };
  const lift = (u: number, v: number) => {
    const at = cellAt(view, arena(view, SIDE, SIDE), SIDE, u, v);
    aim.hide();
    if (at >= 0) place(at);
    else paint();
    return null;
  };
  const nudge = (key?: string) => {
    aim.nudge(key);
    paint();
  };
  const press = () => {
    if (aim.at() < 0) nudge();
    else place(aim.at());
  };
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, lift, nudge, press };
}
