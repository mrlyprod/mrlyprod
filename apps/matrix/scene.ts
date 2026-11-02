import { blend, bright, HUES, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick, rng } from "../../net/sdk/rand.ts";
import type { Opts, View } from "../../net/sdk/run.ts";
import { board } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Name } from "../../net/sdk/theme/theme.ts";
import type { State } from "./engine.ts";
import { fitted, glyphs, opening, step, TICK } from "./engine.ts";

/* TYPES */

export type Knobs = { set: string; color: string; tint: string; tick: number; cell: number };

export type Rain = ReturnType<typeof make>;

export const SPEC: Spec = [
  {
    key: "set",
    label: "Set",
    kind: "list",
    def: "lds",
    sep: "",
    options: [
      ["l", "Letters"],
      ["d", "Digits"],
      ["s", "Specials"],
    ],
    group: "Glyphs",
  },
  {
    key: "color",
    label: "Drops",
    kind: "segment",
    def: "tint",
    options: [
      ["tint", "Tint"],
      ["rainbow", "Rainbow"],
    ],
    group: "Colour",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Colour" },
  { key: "tick", label: "Tick", kind: "slider", def: 33, min: 16, max: 120, step: 1, unit: "ms", group: "Motion" },
  { key: "cell", label: "Size", kind: "slider", def: 20, min: 8, max: 48, step: 2, group: "Grid" },
];

export const PAGE = { spec: SPEC } satisfies Page<Ready & Units, Rain>;

export const SAVERS = [
  {
    id: "rain",
    title: "Glyph rain",
    pick: (rand: Rng) => ({
      set: pick(rand, ["lds", "ld", "d", "l"]),
      color: "tint",
      tick: 100,
      cell: 14 + 2 * Math.floor(rand() * 5),
    }),
  },
  {
    id: "heavy",
    title: "Heavy rain",
    pick: (rand: Rng) => ({
      set: pick(rand, ["lds", "ds", "s"]),
      color: "tint",
      tick: 100,
      cell: 22 + 2 * Math.floor(rand() * 6),
    }),
  },
];

const FACE = "MrlyFont";
const FADE = 0.05;
const WASH = 0.12;
const DEEP = 0.3;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Opts) {
  const ctx = canvas.getContext("2d")!;
  const { set, color, tick, cell } = tidy(SPEC, opts) as Knobs;
  const pool = glyphs(set);
  const hue = (k: number) => (k < 0 ? null : palette[HUES[k] as Name]);
  let rain: State = opening({ set, color }, view.rand);
  const salt = rain.seed;
  let grid = board(view, 1, 1);
  const look = () => {
    const { paper, accent } = view.look();
    return bright(paper)
      ? { paper: blend(paper, accent, WASH), accent: blend(accent, palette.black, DEEP) }
      : { paper, accent };
  };
  const wash = () => {
    ctx.fillStyle = look().paper;
    ctx.fillRect(0, 0, view.w, view.h);
  };
  const size = () => {
    rain = fitted(rain, cell, view);
    grid = board(view, rain.cols, rain.rows);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.font = `${grid.cell}px ${FACE}, monospace`;
    ctx.textBaseline = "alphabetic";
    wash();
  };
  const field = () => {
    const { accent } = look();
    const again = rng(salt);
    const { cols, rows } = rain;
    wash();
    for (let i = 0; i < cols; i++) {
      ctx.fillStyle = hue(rain.tints[i]) ?? accent;
      for (let row = 1; row <= rows; row++) {
        ctx.globalAlpha = (1 - FADE) ** (rows - row);
        ctx.fillText(pick(again, pool), grid.x + i * grid.cell, grid.y + row * grid.cell);
      }
    }
    ctx.globalAlpha = 1;
  };
  const draw = () => {
    if (view.still) return field();
    const { paper, accent } = look();
    ctx.fillStyle = veil(paper, FADE);
    ctx.fillRect(0, 0, view.w, view.h);
    const was = rain;
    rain = step(rain, TICK);
    for (let i = 0; i < was.cols; i++) {
      if (rain.lit[i] < 0) continue;
      ctx.fillStyle = hue(was.tints[i]) ?? accent;
      ctx.fillText(pool[rain.lit[i]], grid.x + i * grid.cell, grid.y + was.drops[i] * grid.cell);
    }
  };
  size();
  return { every: tick, font: FACE, draw, size, theme: wash, state: () => rain };
}
