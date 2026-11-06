import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given } from "../shared/games/stage.ts";
import { aimed, arena, cellAt, clear, cursor, ground, heading, inks, slab } from "../shared/games/stage.ts";
import type { State } from "./engine.ts";
import { deal, SIDE, step } from "./engine.ts";

export type Memory = ReturnType<typeof make>;

const MATCH = 500;
const MISS = 800;
const MARGIN = 0.1;
const BACK = 0.28;
const WRONG = 0.3;

export const PAGE = aimed<Memory>();

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const aim = cursor(SIDE, SIDE);
  let state: State = deal({}, view);
  let held: State | null = null;
  let timer: ReturnType<typeof setTimeout> | 0 = 0;
  const said = (board: State) => {
    if (board.over) return `mismatch, pairs ${board.pairs}`;
    return board.flash ? "look!" : `pairs ${board.pairs}`;
  };
  const tell = () => {
    const board = held ?? state;
    opts.tell?.({ rows: [[board.over ? "game over" : "pairs", board.pairs]], over: board.over });
  };
  const paint = () => {
    const board = held ?? state;
    const box = arena(view, SIDE, SIDE);
    const tone = inks(canvas, view);
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(board), tone.ink);
    for (const [at, card] of board.cards.entries()) {
      if (card.gone) continue;
      const shown = board.pair.includes(at) && (held !== null || board.over);
      const up = board.flash || board.open === at || shown;
      const back = board.over && shown ? veil(tone.hue("red"), WRONG) : up ? ground(tone) : veil(tone.accent, BACK);
      const [x, y, side] = slab(ctx, box, SIDE, at, back);
      if (!up) continue;
      const inset = side * MARGIN;
      ctx.fillStyle = tone.hue(card.hue);
      stamp(ctx, card, x + inset, y + inset, side - 2 * inset);
    }
    aim.ring(ctx, box, tone.accent);
    view.drew();
  };
  const flip = (at: number) => {
    if (held) return;
    const before = state;
    state = step(state, { verb: "flip", args: { at } });
    if (state.pair !== before.pair) {
      held = { ...before, open: -1, pair: state.pair };
      timer = setTimeout(
        () => {
          held = null;
          tell();
          paint();
        },
        state.over ? MISS : MATCH,
      );
    }
    tell();
    paint();
  };
  const restart = () => {
    clearTimeout(timer);
    held = null;
    state = deal({}, view);
    tell();
    paint();
  };
  const lift = (u: number, v: number) => {
    const at = cellAt(view, arena(view, SIDE, SIDE), SIDE, u, v);
    aim.hide();
    if (at >= 0) flip(at);
    else paint();
    return null;
  };
  const nudge = (key?: string) => {
    aim.nudge(key);
    paint();
  };
  const press = () => {
    if (aim.at() < 0) nudge();
    else flip(aim.at());
  };
  const stop = () => clearTimeout(timer);
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, stop, lift, nudge, press };
}
