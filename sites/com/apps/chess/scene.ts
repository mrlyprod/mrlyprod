import { blend, bright } from "@mrlyprod/ui/kit/color.ts";
import type { Handle } from "@mrlyprod/ui/kit/page.tsx";
import type { View } from "@mrlyprod/ui/kit/run.ts";
import type { Given, Told } from "../shared/games/stage.ts";
import { aimed, arena, cellAt, clear, cursor, heading, inks, pixels, slab } from "../shared/games/stage.ts";
import type { Act, Kind, State } from "./engine.ts";
import { deal, reply, SIZE, step, targets } from "./engine.ts";

export type Chess = ReturnType<typeof make>;

export type Said = Told & { at: number; total: number };

const GLYPHS: Record<Kind, string[]> = {
  K: ["00000", "01110", "01110", "01110", "00000"],
  Q: ["10101", "01110", "11111", "01110", "10101"],
  R: ["00100", "00100", "11111", "00100", "00100"],
  N: ["01010", "10001", "00100", "10001", "01010"],
  B: ["10001", "01010", "00100", "01010", "10001"],
  P: ["00000", "00100", "01110", "00100", "00000"],
};

const FALLEN = ["00000", "01110", "01010", "01110", "00000"];

const AROUND = [
  [-1, -1],
  [0, -1],
  [1, -1],
  [-1, 0],
  [1, 0],
  [-1, 1],
  [0, 1],
  [1, 1],
];

const WAIT = 800;
const LIGHT = 0.15;
const DARK = 0.45;
const LATEST = 0.75;
const FILL = 0.78;
const HALO = 0.04;
const DOT = 0.16;
const TAU = Math.PI * 2;

const base = aimed<Chess>();

export const PAGE = {
  ...base,
  keys: [...base.keys, { key: ",", label: "Back", act: "back" }, { key: ".", label: "Next", act: "next" }],
  actions: {
    ...base.actions,
    back: (scene: Handle<Chess>) => scene.back(),
    next: (scene: Handle<Chess>) => scene.next(),
  },
};

export function make(canvas: HTMLCanvasElement, view: View, opts: Given<Said> = {}) {
  const ctx = canvas.getContext("2d")!;
  const aim = cursor(SIZE, SIZE);
  let state: State = deal({}, view);
  let past = [state];
  let picked = -1;
  let viewing = -1;
  let timer: ReturnType<typeof setTimeout> | undefined;
  const flipped = () => state.human === "black";
  const place = (spot: number) => (flipped() ? SIZE * SIZE - 1 - spot : spot);
  const shown = () => (viewing < 0 ? state : past[viewing]);
  const said = () => {
    if (viewing >= 0) return `move ${viewing} of ${past.length - 1}`;
    if (state.over === "checkmate") return state.winner === state.human ? "checkmate, you win" : "checkmate, you lose";
    if (state.over) return state.over;
    if (state.turn !== state.human) return "thinking";
    return state.check ? "check" : "your move";
  };
  const tell = () =>
    opts.tell?.({
      rows: [
        ["game", said()],
        ["side", state.human],
        ["moves", state.ply],
      ],
      over: Boolean(state.over),
      at: viewing < 0 ? past.length - 1 : viewing,
      total: past.length - 1,
    });
  const paint = () => {
    const box = arena(view, SIZE, SIZE);
    const tone = inks(canvas, view);
    const [pale, deep] = bright(tone.paper) ? [tone.paper, tone.ink] : [tone.ink, tone.paper];
    const now = shown();
    const dots = viewing < 0 && picked >= 0 ? targets(state, picked) : [];
    clear(ctx, view);
    heading(ctx, canvas, view, box, said(), tone.ink);
    for (let spot = 0; spot < SIZE * SIZE; spot++) {
      const at = place(spot);
      const piece = now.board[at];
      const row = Math.floor(at / SIZE);
      const lit = now.last?.from === at || now.last?.to === at;
      const beaten = piece?.kind === "K" && piece.side === now.turn && now.check;
      const shade = (at % SIZE) + row;
      const ground = beaten
        ? tone.hue("red")
        : viewing < 0 && picked === at
          ? tone.accent
          : blend(tone.paper, tone.accent, lit ? LATEST : shade % 2 ? DARK : LIGHT);
      const [x, y, side] = slab(ctx, box, SIZE, spot, ground, 0);
      if (piece) {
        const rows = beaten && now.over === "checkmate" ? FALLEN : GLYPHS[piece.kind];
        const room = side * FILL;
        const left = x + (side - room) / 2;
        const top = y + (side - room) / 2;
        const halo = Math.max(1, Math.round(side * HALO));
        ctx.fillStyle = piece.side === "white" ? deep : pale;
        for (const [dx, dy] of AROUND) pixels(ctx, rows, left + dx * halo, top + dy * halo, room);
        ctx.fillStyle = piece.side === "white" ? pale : deep;
        pixels(ctx, rows, left, top, room);
      }
      if (dots.includes(at)) {
        ctx.fillStyle = tone.ink;
        ctx.beginPath();
        ctx.arc(x + side / 2, y + side / 2, side * DOT, 0, TAU);
        ctx.fill();
      }
    }
    aim.ring(ctx, box, tone.ink);
    view.drew();
  };
  const wake = () => {
    clearTimeout(timer);
    if (state.over || state.turn === state.human) return;
    timer = setTimeout(() => {
      const act = reply(state);
      if (act) play(act);
    }, WAIT);
  };
  const play = (act: Act) => {
    state = step(state, act);
    past = [...past, state];
    picked = -1;
    viewing = -1;
    wake();
    tell();
    paint();
  };
  const choose = (spot: number) => {
    if (viewing >= 0 || state.over || state.turn !== state.human) return;
    const at = place(spot);
    if (picked >= 0 && targets(state, picked).includes(at)) {
      play({ verb: "move", args: { from: picked, to: at } });
      return;
    }
    picked = state.board[at]?.side === state.human ? at : -1;
    paint();
  };
  const move = (by: number) => {
    const at = (viewing < 0 ? past.length - 1 : viewing) + by;
    if (at < 0 || at >= past.length) return;
    viewing = at === past.length - 1 ? -1 : at;
    tell();
    paint();
  };
  const restart = () => {
    state = deal({}, view);
    past = [state];
    picked = -1;
    viewing = -1;
    wake();
    tell();
    paint();
  };
  const lift = (u: number, v: number) => {
    const spot = cellAt(view, arena(view, SIZE, SIZE), SIZE, u, v);
    aim.hide();
    if (spot >= 0) choose(spot);
    paint();
    return null;
  };
  const nudge = (key?: string) => {
    aim.nudge(key);
    paint();
  };
  const press = () => {
    if (aim.at() < 0) nudge();
    else choose(aim.at());
  };
  const stop = () => clearTimeout(timer);
  wake();
  tell();
  paint();
  return {
    draw: paint,
    still: true,
    size: paint,
    theme: paint,
    restart,
    lift,
    nudge,
    press,
    stop,
    back: () => move(-1),
    next: () => move(1),
  };
}
