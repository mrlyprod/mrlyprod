import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import { named, stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import { aimed, arena, cellAt, clear, cursor, ground, heading, inks, slab } from "../shared/games/stage.ts";
import type { State } from "./engine.ts";
import { deal, SIDE, step } from "./engine.ts";

export type Captcha = ReturnType<typeof make>;

const MARGIN = 0.1;
const MARKED = 0.3;

export const PAGE = aimed<Captcha>();

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const aim = cursor(SIDE, SIDE);
  let state: State = deal({}, view);
  const wanted = () => named(state.items[state.target]);
  const said = () => (state.over ? `wrong, it was ${wanted()}` : `find ${wanted()}`);
  const tell = () => opts.tell?.({ rows: [[state.over ? "game over" : "score", state.score]], over: state.over });
  const paint = () => {
    const box = arena(view, SIDE, SIDE);
    const tone = inks(canvas, view);
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    for (const [at, item] of state.items.entries()) {
      const back =
        at === state.wrong
          ? veil(tone.hue("red"), MARKED)
          : state.over && at === state.target
            ? veil(tone.accent, MARKED)
            : ground(tone);
      const [x, y, side] = slab(ctx, box, SIDE, at, back);
      const inset = side * MARGIN;
      ctx.fillStyle = tone.hue(item.hue);
      stamp(ctx, item, x + inset, y + inset, side - 2 * inset);
    }
    aim.ring(ctx, box, tone.accent);
    view.drew();
  };
  const pick = (at: number) => {
    state = step(state, { verb: "pick", args: { at } });
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
    if (at >= 0) pick(at);
    else paint();
    return null;
  };
  const nudge = (key?: string) => {
    aim.nudge(key);
    paint();
  };
  const press = () => {
    if (aim.at() < 0) nudge();
    else pick(aim.at());
  };
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, lift, nudge, press };
}
