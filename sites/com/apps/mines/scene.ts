import { blend } from "@mrlyprod/ui/kit/color.ts";
import { glyph } from "@mrlyprod/ui/kit/font/font.ts";
import type { Handle } from "@mrlyprod/ui/kit/page.tsx";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import { stamp } from "../shared/games/sprites.ts";
import type { Given, Told } from "../shared/games/stage.ts";
import { aimed, arena, cellAt, clear, cursor, ground, heading, inks, pixels, slab } from "../shared/games/stage.ts";
import type { Act, State } from "./engine.ts";
import { COLS, deal, MINES, ROWS, step } from "./engine.ts";

export type Mines = ReturnType<typeof make>;

export type Said = Told & { flagging: boolean };

const COVER = 0.28;
const BOOM = 0.55;
const FACE = 0.14;
const DIGIT = 0.56;

const base = aimed<Mines>();

export const PAGE = {
  ...base,
  keys: [...base.keys, { key: "f", label: "Flag", act: "flag" }],
  actions: { ...base.actions, flag: (scene: Handle<Mines>) => scene.flag() },
};

export function make(canvas: HTMLCanvasElement, view: View, opts: Given<Said> = {}) {
  const ctx = canvas.getContext("2d")!;
  const aim = cursor(COLS, ROWS);
  let state: State = deal({}, view);
  let flagging = false;
  const said = () => {
    if (state.over === "won") return "cleared";
    if (state.over === "lost") return "boom";
    return `${MINES - state.flags} mines`;
  };
  const tell = () =>
    opts.tell?.({
      rows: [
        ["game", said()],
        ["open", state.cells.filter((cell) => cell.face === "open" && !cell.mine).length],
      ],
      over: Boolean(state.over),
      flagging,
    });
  const paint = () => {
    const box = arena(view, COLS, ROWS);
    const tone = inks(canvas, view);
    const cover = blend(tone.paper, tone.accent, COVER);
    const blast = blend(tone.paper, tone.hue("red"), BOOM);
    const [mine, flag] = state.faces;
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    for (const [at, cell] of state.cells.entries()) {
      const shut = cell.face !== "open";
      const [x, y, side] = slab(ctx, box, COLS, at, at === state.boom ? blast : shut ? cover : ground(tone));
      const inset = side * FACE;
      if (cell.face === "flag") {
        ctx.fillStyle = tone.hue(flag.hue);
        stamp(ctx, flag, x + inset, y + inset, side - 2 * inset);
      } else if (cell.face === "open" && cell.mine) {
        ctx.fillStyle = tone.hue(mine.hue);
        stamp(ctx, mine, x + inset, y + inset, side - 2 * inset);
      } else if (cell.face === "open" && cell.near) {
        const room = side * DIGIT;
        ctx.fillStyle = tone.ink;
        pixels(ctx, glyph(String(cell.near)), x + (side - room) / 2, y + (side - room) / 2, room);
      }
    }
    aim.ring(ctx, box, tone.ink);
    view.drew();
  };
  const play = (act: Act) => {
    state = step(state, act);
    tell();
    paint();
  };
  const pick = (at: number) => play({ verb: flagging ? "flag" : "reveal", args: { at } });
  const restart = () => {
    state = deal({}, view);
    flagging = false;
    tell();
    paint();
  };
  const spot = (u: number, v: number) => cellAt(view, arena(view, COLS, ROWS), COLS, u, v);
  const lift = (u: number, v: number) => {
    const at = spot(u, v);
    aim.hide();
    if (at >= 0) pick(at);
    else paint();
    return null;
  };
  const alt = (u: number, v: number) => {
    const at = spot(u, v);
    if (at >= 0) play({ verb: "flag", args: { at } });
  };
  const nudge = (key?: string) => {
    aim.nudge(key);
    paint();
  };
  const press = () => {
    if (aim.at() < 0) nudge();
    else pick(aim.at());
  };
  const shift = () => {
    flagging = !flagging;
    tell();
    paint();
  };
  const flag = () => {
    if (aim.at() < 0) shift();
    else play({ verb: "flag", args: { at: aim.at() } });
  };
  tell();
  paint();
  return { draw: paint, still: true, size: paint, theme: paint, restart, lift, alt, nudge, press, flag, shift };
}
