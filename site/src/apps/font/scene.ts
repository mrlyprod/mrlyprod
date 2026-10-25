import { text as file } from "../../lib/app/export.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Board, Box, Opts, View } from "../../lib/app/run.ts";
import { board } from "../../lib/app/run.ts";
import type { Scalar, Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { grid, rects } from "../../lib/app/svg.ts";
import { TINTS, veil } from "../../lib/color.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick, rng } from "../../lib/rand.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Font, Plan, Shelved } from "./engine.ts";
import {
  cells,
  common,
  cut,
  GAP,
  gallery,
  index,
  pen,
  plan,
  SAMPLES,
  SLOT,
  sample,
  shift,
  slug,
  written,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  view: string;
  text: string;
  speed: number;
  hold: number;
  loop: number;
  flags: number;
  pad: number;
  cell: number;
  tint: string;
  seed?: number;
};

export type Told = { text: string; rows: number; cols: number; frames: number; fps: number };

export type Given = Opts & { font?: Font | null; kinds?: readonly Shelved[] | null; onPlan?: (plan: Told) => void };

export type Tile = {
  across: number;
  down: number;
  cell: number;
  spare: number;
  n: number;
  x: number;
  y: number;
  w: number;
  h: number;
};

export type Writer = ReturnType<typeof make>;

type Best = Pick<Tile, "across" | "down" | "cell" | "spare">;

type Seen = { view?: Scalar };

export const VIEWS: [string, string][] = [
  ["pens", "Pens"],
  ["text", "Text"],
];

const TEXT = (value: Seen) => value.view === "text";

const PENS = (value: Seen) => !TEXT(value);

export const named = (value: Value | Knobs) =>
  TEXT(value) ? `font-${slug(value.text || sample(rng(value.seed as number | undefined)))}` : "font-pens";

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "pens", options: VIEWS, group: "View" },
  { key: "text", label: "Text", kind: "text", def: "", wide: true, group: "View", when: TEXT },
  { key: "speed", label: "Speed", kind: "slider", def: 1, min: 0.25, max: 4, step: 0.25, unit: "x", group: "Motion" },
  { key: "hold", label: "Hold", kind: "slider", def: 25, min: 0, max: 100, step: 1, unit: "frames", group: "Motion" },
  { key: "loop", label: "Loop", kind: "toggle", def: 1, group: "Motion" },
  { key: "flags", label: "Flags", kind: "toggle", def: 1, group: "Look", when: PENS },
  {
    key: "pad",
    label: "Pad",
    kind: "slider",
    def: 1,
    min: 0,
    max: 4,
    step: 1,
    unit: "cells",
    group: "Look",
    when: TEXT,
  },
  {
    key: "cell",
    label: "Cell",
    kind: "slider",
    def: 0,
    min: 0,
    max: 40,
    step: 2,
    unit: "px",
    zero: "Fit",
    group: "Look",
    when: TEXT,
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const ABOUT = [
  "Every glyph of MrlyFont writes itself in the order its pen table gives, one cell a frame at 25 a second, then holds and writes again.",
  "Each pen has a stroke count and a floor, the fewest strokes that can write it. A stroke walks 4-adjacent cells; a lift is any step that is not.",
  "A plate marks a flagged pen: written over its floor on purpose, like the wordmark letters, or lifting five times or more, which only a new shape can help.",
  "Text writes your own words cell by cell, then folds them into one stack and back. A tap rewrites a pen or the text; a click in the gallery adds a glyph.",
];

export const PAGE = {
  spec: SPEC,
  name: named,
  about: ABOUT,
  record: true,
  keys: [
    { key: "Enter", label: "Write", act: "write", button: true, menu: "Edit" },
    { key: "j", label: "JSON", act: "json" },
    { key: "s", label: "SVG", act: "svg" },
  ],
  actions: {
    write: (scene) => scene.write(),
    json: (scene) => file(JSON.stringify(scene.json()), scene.name(), "json"),
    svg: (scene) => file(scene.svg(), scene.name(), "svg"),
  },
} satisfies Page<Loaded<"font">, Writer>;

export const SAVERS = [
  {
    id: "pens",
    title: "Pens writing",
    still: 3000,
    pick: (rand: Rng) => ({ view: "pens", speed: 0.5, hold: 30 + Math.floor(rand() * 41), flags: pick(rand, [0, 1]) }),
  },
  {
    id: "words",
    title: "Words folding",
    still: 4500,
    pick: (rand: Rng) => ({
      view: "text",
      speed: 0.75,
      hold: 30 + Math.floor(rand() * 31),
      pad: pick(rand, [1, 2, 3]),
    }),
  },
];

export const units = wasm("font");

const FIT = 0.92;

const PITCH = SLOT + GAP;

const PLATE = 0.14;

export function layout(
  view: Pick<View, "w" | "h" | "dpr">,
  { rows, cols }: Pick<Plan, "rows" | "cols">,
  cell: number,
): Board {
  const fit = board(view, Math.max(1, cols), Math.max(1, rows), FIT);
  if (!cell) return fit;
  const side = Math.min(fit.cell, Math.max(1, Math.round(cell * view.dpr)));
  const w = side * cols;
  const h = side * rows;
  return { x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h, cell: side };
}

export function tile(view: Box, n: number): Tile {
  let best: Best | null = null;
  for (let across = 1; across <= n; across++) {
    const down = Math.ceil(n / across);
    const cell = Math.max(
      1,
      Math.floor(Math.min((view.w * FIT) / (across * PITCH - GAP), (view.h * FIT) / (down * PITCH - GAP))),
    );
    const spare = across * down - n;
    if (!best || cell > best.cell || (cell === best.cell && spare < best.spare)) best = { across, down, cell, spare };
  }
  const w = (best!.across * PITCH - GAP) * best!.cell;
  const h = (best!.down * PITCH - GAP) * best!.cell;
  return { ...best!, n, x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h };
}

export function place(box: Pick<Tile, "across" | "down" | "n">, i: number): [number, number] {
  const row = Math.floor(i / box.across);
  const last = row === box.down - 1 ? box.n - row * box.across : box.across;
  return [(i % box.across) * PITCH + Math.floor(((box.across - last) * PITCH) / 2), row * PITCH];
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const { font, onPlan } = opts;
  const pens = (opts.kinds ?? gallery(font!)).flatMap((kind) => kind.glyphs);
  const starts = pens.map(() => 0);
  let value = { ...tidy(SPEC, opts), seed: opts.seed } as Knobs;
  let said = sample(view.rand);
  let made = written(font!, value.text || said, value.pad);
  let show = plan(font!, value, made);
  let start = 0;
  let base = 0;
  let from = view.t;
  let box: Board | Tile | null = null;
  let shown: number | null = null;
  let last = 0;
  const phase = () => base + ((view.t - from) * show.fps * value.speed) / 1000;
  const tick = () => Math.floor(phase() + 1e-9);
  const now = () => (view.still ? 0 : tick());
  const fit = () => {
    box = TEXT(value) ? layout(view, show, value.cell) : tile(view, pens.length);
  };
  const told = () =>
    onPlan?.({ text: show.text, rows: show.rows, cols: show.cols, frames: show.frames.length, fps: show.fps });
  const build = (words: string) => {
    if (made.text !== words || made.pad !== value.pad) made = written(font!, words, value.pad);
    show = plan(font!, value, made);
  };
  const frame = (k: number) => (view.still ? show.still : index(k - start, show.frames.length, value.loop));
  const at = (k: number, i: number) => {
    const n = pens[i].frames.length;
    return view.still ? n - 1 : pen(k - starts[i], n, value.hold, value.loop);
  };
  const key = (k: number) => {
    if (TEXT(value)) return frame(k);
    if (value.loop || view.still) return k;
    return Math.min(k, Math.max(...starts) + Math.max(...pens.map((one) => one.frames.length)));
  };
  const lit = (k: number) => {
    const out: [number, number][] = [];
    pens.forEach((one, i) => {
      const [cx, cy] = place(box as Tile, i);
      const dx = cx + Math.floor((SLOT - one.cols) / 2);
      const dy = cy + Math.floor((SLOT - one.rows) / 2);
      for (const c of one.frames[at(k, i)]) out.push([dx + (c % one.cols), dy + Math.floor(c / one.cols)]);
    });
    return out;
  };
  const roams = (k: number) =>
    TEXT(value) && !value.text && value.loop && !view.still && k - start >= show.frames.length;
  const turn = (k: number) => {
    said = pick(
      view.rand,
      SAMPLES.filter((one) => one !== said),
    );
    build(said);
    start = k;
    fit();
    shown = null;
    told();
  };
  const drift = () => {
    pens.forEach((one, i) => {
      starts[i] = -Math.floor(view.rand() * (one.frames.length + value.hold));
    });
  };
  const plates = (cell: number) => {
    const pad = Math.round(cell / 2);
    const side = SLOT * cell + 2 * pad;
    ctx.beginPath();
    pens.forEach((one, i) => {
      if (!one.flag) return;
      const [cx, cy] = place(box as Tile, i);
      const x = box!.x + cx * cell - pad;
      const y = box!.y + cy * cell - pad;
      if (ctx.roundRect) ctx.roundRect(x, y, side, side, cell);
      else ctx.rect(x, y, side, side);
    });
    ctx.fill();
  };
  const paint = (k: number) => {
    const { accent } = view.look();
    const { x, y, cell } = box!;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (PENS(value) && value.flags) {
      ctx.fillStyle = veil(accent, PLATE);
      plates(cell);
    }
    ctx.fillStyle = accent;
    ctx.beginPath();
    if (TEXT(value))
      for (const i of show.frames[frame(k)])
        ctx.rect(x + (i % show.cols) * cell, y + Math.floor(i / show.cols) * cell, cell, cell);
    else for (const [cx, cy] of lit(k)) ctx.rect(x + cx * cell, y + cy * cell, cell, cell);
    ctx.fill();
    shown = key(k);
    last = k;
  };
  const draw = () => {
    const k = now();
    if (view.lock && roams(k)) turn(k);
    if (key(k) !== shown) paint(k);
  };
  const size = () => {
    fit();
    paint(now());
  };
  const theme = () => paint(now());
  const rewrite = (was: Knobs, k: number, words: string) => {
    const old = show;
    const j = index(k - start, old.frames.length, was.loop);
    build(words);
    if (show.text !== old.text) {
      const kept = cells(font!, common(old.text, show.text), value.pad);
      return k - (j > old.still ? kept : Math.min(j, kept));
    }
    if (value.loop !== was.loop) return k - Math.min(j, show.still);
    if (value.hold !== was.hold) return k - shift(j, old.moves, old.hold, show.hold);
    return k - Math.min(j, show.frames.length - 1);
  };
  const regrip = (was: Knobs, k: number) => {
    pens.forEach((one, i) => {
      const n = one.frames.length;
      const d = k - starts[i];
      const c = was.loop ? index(d, n + was.hold) : Math.min(Math.max(0, d), n - 1);
      starts[i] = k - Math.min(c, (value.loop ? n + value.hold : n) - 1);
    });
  };
  const shuffle = (k: number) => {
    const most = Math.max(...pens.map((one) => one.frames.length));
    pens.forEach((_, i) => {
      starts[i] = k + Math.floor(view.rand() * most);
    });
  };
  const set = (next: Opts) => {
    const was = value;
    const p = phase();
    value = { ...tidy(SPEC, next), seed: next.seed } as Knobs;
    if (value.speed !== was.speed) {
      base = p;
      from = view.t;
    }
    const reseeded = value.seed !== was.seed;
    if (reseeded) said = sample(view.rand);
    const k = tick();
    const words = cut(value.text || said);
    if (value.view !== was.view) {
      build(words);
      start = k;
      starts.fill(k);
    } else if (
      TEXT(value) &&
      (words !== show.text || value.pad !== was.pad || value.hold !== was.hold || value.loop !== was.loop)
    )
      start = rewrite(was, k, words);
    else if (PENS(value) && reseeded) shuffle(k);
    else if (PENS(value) && (value.hold !== was.hold || value.loop !== was.loop)) regrip(was, k);
    fit();
    shown = null;
    told();
  };
  const tap = (px: number, py: number) => {
    if (view.still) return -1;
    const k = tick();
    let hit = 0;
    if (TEXT(value)) start = k;
    else {
      hit = pens.findIndex((_, i) => {
        const [cx, cy] = place(box as Tile, i);
        const u = (px - box!.x) / box!.cell - cx + GAP / 2;
        const v = (py - box!.y) / box!.cell - cy + GAP / 2;
        return u >= 0 && u < PITCH && v >= 0 && v < PITCH;
      });
      if (hit >= 0) starts[hit] = k;
    }
    shown = null;
    draw();
    return hit;
  };
  const write = () => {
    const k = tick();
    start = k;
    starts.fill(k);
    shown = null;
    draw();
  };
  const json = () => {
    const fps = show.fps * value.speed;
    if (TEXT(value))
      return {
        text: show.text,
        pad: value.pad,
        hold: value.hold,
        loop: value.loop,
        rows: show.rows,
        cols: show.cols,
        fps,
        frames: show.frames,
      };
    return {
      hold: value.hold,
      loop: value.loop,
      fps,
      pens: pens.map(({ char, name, rows, cols, strokes, floor, lifts, flag, frames }) => ({
        char,
        name,
        rows,
        cols,
        strokes,
        floor,
        lifts,
        flag,
        frames,
      })),
    };
  };
  const svg = () => {
    const k = shown === null ? now() : last;
    const fill = view.look().accent;
    const unit = box!.cell / view.dpr;
    if (TEXT(value)) return rects(grid(show.rows, show.cols, show.frames[frame(k)]), { fill, unit });
    const across = (box as Tile).across * PITCH - GAP;
    return rects(
      grid(
        (box as Tile).down * PITCH - GAP,
        across,
        lit(k).map(([cx, cy]) => cy * across + cx),
      ),
      { fill, unit },
    );
  };
  const name = () => named(value);
  if (view.lock && PENS(value)) drift();
  fit();
  told();
  return { draw, size, theme, set, tap, write, json, svg, name, box: () => box, plan: () => show };
}
