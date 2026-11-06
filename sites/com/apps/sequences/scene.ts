import type { Crate } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts } from "@mrlyprod/ui/kit/run.ts";
import type { Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { RefObject } from "react";
import { KINDS } from "../shared/sequences/chart.ts";
import type { Picture, Plan } from "../shared/sequences/design.ts";
import { draw as art, inks, picture, said } from "../shared/sequences/design.ts";
import type { Mrly } from "../shared/sequences/engine.ts";
import { AXES, booked, CANON, filter, lead, MEASURES, picks, TERMS } from "../shared/sequences/engine.ts";
import type { Grown, Series, Slots, Spot, View } from "../shared/sequences/paint.ts";
import {
  caption,
  chart,
  clear,
  drawRibbon,
  FACE,
  growth,
  legend,
  ribbon,
  shade,
  slotAt,
  tones,
} from "../shared/sequences/paint.ts";

export type Knobs = {
  chart: string;
  terms: number;
  digits: number;
  dim: number | "";
  base: number | "";
  measure: string;
  axis: string;
};

export type Line = Plan & Grown & Series;

export type Feed = { rows: Line[]; version: number };

export type Fed = Opts & Partial<Feed> & { mrly?: Mrly | null; live?: RefObject<Feed> };

export type Hits = { legend: Spot[]; slots: Slots | null };

export type Hands = { browse: (by: number) => void; find: () => void; plot: () => void };

export type Stage = ReturnType<typeof make>;

export type Held = Stage & Hands;

export const SPEC: Spec = [
  { key: "chart", label: "Chart", kind: "pick", def: "pins", options: KINDS, group: "Chart" },
  { key: "terms", label: "Terms", kind: "slider", def: 12, min: TERMS.min, max: TERMS.max, step: 1, group: "Chart" },
  {
    key: "digits",
    label: "Digits",
    kind: "segment",
    def: 2,
    options: [
      [2, "2"],
      [3, "3"],
      [10, "10"],
    ],
    group: "Chart",
    when: (value) => value.chart === "digits",
  },
  {
    key: "dim",
    label: "Space",
    kind: "segment",
    def: "",
    options: [
      ["", "Any"],
      [2, "Plane"],
      [3, "Cube"],
    ],
    group: "Rows",
  },
  {
    key: "base",
    label: "Base",
    kind: "segment",
    def: "",
    options: (value) =>
      value.dim === 3
        ? [
            ["", "Any"],
            [2, "2"],
          ]
        : [
            ["", "Any"],
            [2, "2"],
            [3, "3"],
          ],
    group: "Rows",
    when: (value) => value.dim !== 3,
  },
  {
    key: "measure",
    label: "Measure",
    kind: "pick",
    def: "",
    options: [["", "Any"], ...MEASURES.map((one): [string, string] => [one.slug, one.label])],
    group: "Rows",
  },
  {
    key: "axis",
    label: "Axis",
    kind: "segment",
    def: "",
    options: [["", "Both"], ...AXES.map((one): [string, string] => [one.slug, one.label])],
    group: "Rows",
  },
];

export const named = (value: Value) =>
  ["sequences", picks(value.pick as string)[0] ?? value.seed, value.chart].join("-");

export const units = wasm(...CRATES);

export const roll = (seed: number, n: number) => Math.floor(rng(seed >>> 0)() * n);

export const PAGE: Page<Loaded<Crate>, Held> = {
  defaults: { q: "", pick: CANON },
  spec: SPEC,
  units,
  name: named,
  keys: [
    { key: "Enter", label: "Plot", act: "plot", button: true, menu: "File" },
    { key: ["ArrowDown", "ArrowUp"], label: "Browse", act: "browse" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Term", act: "term" },
    { key: "/", label: "Search", act: "search" },
    { key: "c", label: "Chart", act: "cycle", menu: "View" },
  ],
  actions: {
    plot: (scene) => scene.plot?.(),
    browse: (scene, e) => scene.browse?.(e.key === "ArrowUp" ? -1 : 1),
    term: (scene, e) => scene.step?.(e.key === "ArrowLeft" ? -1 : 1),
    search: (scene) => scene.find?.(),
  },
  acts: {
    cycle: (value) => ({ chart: KINDS[(KINDS.findIndex(([slug]) => slug === value.chart) + 1) % KINDS.length][0] }),
  },
  reroll: (value, seed, units) => {
    const book = booked(units!);
    const space = filter(book.keys(value.dim, value.base), String(value.measure), String(value.axis));
    if (!space.length) return {};
    const held = picks(String(value.pick))[0] ?? lead(book, space, roll(Number(value.seed), space.length))?.key;
    const row = lead(book, space, roll(seed, space.length), held);
    return row ? { pick: row.key } : {};
  },
};

const KEEP = 24;

const within = (box: Spot, x: number, y: number) => x >= box.x0 && x <= box.x1 && y >= box.y0 && y <= box.y1;

export function make(canvas: HTMLCanvasElement, view: View, opts: Fed = {}) {
  const ctx = canvas.getContext("2d")!;
  const mrly = opts.mrly;
  const live = opts.live;
  let { chart: kind, digits: base } = tidy(SPEC, opts) as Knobs;
  const feed = () => live?.current ?? opts;
  const grown = growth(view);
  const pictures = new Map<string, Picture | null>();
  let seen: number | null = null;
  let dirty = true;
  let at = -1;
  let lead = "";
  let count = 0;
  let moving = false;
  let hits: Hits = { legend: [], slots: null };
  const sheet = (row: Line, index: number) => {
    if (!mrly || !row) return null;
    const id = `${row.key}:${index}`;
    if (!pictures.has(id)) {
      let made: Picture | null = null;
      try {
        made = picture(mrly, row, index);
      } catch {
        made = null;
      }
      pictures.set(id, made);
      if (pictures.size > KEEP) pictures.delete(pictures.keys().next().value!);
    }
    return pictures.get(id);
  };
  const paint = (rows: Line[]) => {
    const t = tones(view);
    clear(ctx, view, t);
    const { px } = t;
    const { x: pad, top: high, bottom: low } = t.edge;
    if (!rows.length) {
      ctx.fillStyle = t.dim;
      ctx.textAlign = "center";
      ctx.fillText("pick a row", view.w / 2, view.h / 2);
      ctx.textAlign = "left";
      hits = { legend: [], slots: null };
      return;
    }
    const first = rows[0];
    if (first.key !== lead) {
      lead = first.key;
      at = -1;
    }
    count = first.terms.length;
    const index = count ? (at < 0 ? count - 1 : Math.min(at, count - 1)) : -1;
    const shades = rows.map((_, i) => shade(t.accent, t.paper, i, rows.length));
    const width = view.w - 2 * pad;
    const top = legend(ctx, rows, shades, t, pad, high, width);
    const laid = ribbon(ctx, first, t, width);
    const body = {
      x: pad,
      y: high + top.height + px * 0.4,
      w: width,
      h: Math.max(px * 8, view.h - high - low - top.height - laid.height - px * 1.6),
    };
    const wide = body.w >= body.h * 1.25;
    const room = wide ? Math.min(body.h, body.w * 0.42) : Math.min(body.w, body.h * 0.42);
    const pic = wide
      ? { x: body.x, y: body.y, w: room, h: body.h - px * 2 }
      : { x: body.x, y: body.y, w: body.w, h: room - px * 2 };
    const plot = wide
      ? { x: body.x + room + px * 1.5, y: body.y, w: body.w - room - px * 1.5, h: body.h }
      : { x: body.x, y: body.y + room, w: body.w, h: body.h - room };
    const one = index >= 0 ? sheet(first, index) : null;
    if (one) {
      const drawn = art(ctx, pic, one, inks(first, t, shades[0]));
      caption(
        ctx,
        `${said(one)}${one.at < index ? ", as large as the picture draws" : ""}`,
        pic.x,
        Math.min(pic.y + pic.h + px * 1.5, drawn.y + drawn.h + px * 1.5),
        pic.w,
        t,
      );
    }
    const slots = chart(ctx, plot, rows, shades, t, { kind, base, at: index, shown: (i) => grown.shown(rows[i]) });
    drawRibbon(ctx, laid, pad, view.h - low - laid.height, t, index, shades[0]);
    hits = { legend: rows.length > 1 ? top.hits : [], slots };
  };
  const draw = () => {
    const data = feed();
    const rows = Array.isArray(data.rows) ? data.rows : [];
    const version = data.version ?? 0;
    const busy = grown.busy(rows);
    if (!view.still && !dirty && version === seen && !busy && !moving) return;
    moving = busy;
    seen = version;
    dirty = false;
    paint(rows);
  };
  const again = () => {
    dirty = true;
    draw();
  };
  const set = (value: Opts) => {
    ({ chart: kind, digits: base } = tidy(SPEC, value) as Knobs);
    dirty = true;
  };
  const tap = (x: number, y: number) => {
    const row = hits.legend.find((box) => within(box, x, y));
    if (row) return { row: row.i };
    const j = slotAt(hits.slots, x, y);
    if (j < 0) return null;
    at = j;
    again();
    return { term: j };
  };
  const step = (by: number) => {
    if (!count) return;
    at = Math.max(0, Math.min(count - 1, (at < 0 ? count - 1 : at) + by));
    again();
  };
  return { draw, size: again, theme: again, set, tap, step, font: FACE };
}
