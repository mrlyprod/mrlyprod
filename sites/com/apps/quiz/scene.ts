import type { Ready } from "@mrlyprod/ui/kit/hooks.ts";
import type { Fired } from "@mrlyprod/ui/kit/keys.tsx";
import type { Handle, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given, Told } from "../shared/games/stage.ts";
import { arena, clear, ground, heading, inks, slab } from "../shared/games/stage.ts";
import type { State } from "./engine.ts";
import { deal, step } from "./engine.ts";

export type Quiz = ReturnType<typeof make>;

export type Asked = Told & { options?: string[] };

const MARGIN = 0.08;

export const PAGE = {
  keys: [
    { key: ["1", "2", "3"], label: "Answer", act: "answer" },
    { key: "r", label: "New game", act: "reroll" },
    { key: "?", label: "Keys", act: "keys" },
  ],
  actions: {
    answer: (scene: Handle<Quiz>, e: Fired) => scene.answer(Number(e.key) - 1),
  },
} satisfies Page<Ready & Units, Quiz>;

export function make(canvas: HTMLCanvasElement, view: View, opts: Given<Asked> = {}) {
  const ctx = canvas.getContext("2d")!;
  let state: State = deal({}, view);
  const said = () => (state.over ? `wrong, this is ${state.options[state.answer]}` : `score ${state.score}`);
  const tell = () =>
    opts.tell?.({
      rows: [[state.over ? "game over" : "score", state.score]],
      over: state.over,
      options: state.options,
    });
  const paint = () => {
    const box = arena(view, 1, 1);
    const tone = inks(canvas, view);
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    const [x, y, side] = slab(ctx, box, 1, 0, ground(tone), 0);
    const inset = side * MARGIN;
    ctx.fillStyle = tone.hue(state.sprite.hue);
    stamp(ctx, state.sprite, x + inset, y + inset, side - 2 * inset);
    view.drew();
  };
  const answer = (at: number) => {
    state = step(state, { verb: "answer", args: { at } });
    tell();
    paint();
  };
  const restart = () => {
    state = deal({}, view);
    tell();
    paint();
  };
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, answer };
}
