import type { RefObject } from "react";
import type { Ready } from "../../lib/app/hooks.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Spec, Units, Value } from "../../lib/app/spec.ts";
import { blend, veil } from "../../lib/color.ts";
import type { Inks } from "../../lib/sequences/design.ts";
import { draw as art, inks } from "../../lib/sequences/design.ts";
import type { Entry } from "../../lib/sequences/engine.ts";
import type { Grown, Rect, Tones } from "../../lib/sequences/paint.ts";
import {
  caption,
  chart,
  clear,
  drawRibbon,
  FACE,
  fit,
  font,
  growth,
  ribbon,
  tones,
  wrap,
} from "../../lib/sequences/paint.ts";
import type { Art, ArtOf, Built } from "./cards.ts";
import { CARDS, pick, shown } from "./cards.ts";

/* TYPES */

export type Feed = { made: null; version: number } | { made: Built; card: number; shown: string; version: number };

export type Given = Opts & (Feed | { made?: null; version?: number }) & { live?: RefObject<Feed> };

export type Found = { record: Pick<Entry, "id" | "name">; at: number };

export type Formed = { closed: string; records: { formula: string; short: boolean; found: readonly Found[] } };

export type Hands = { turn?: (by: number) => void; walk?: (by: number) => void };

export type Tour = ReturnType<typeof make> & Hands;

const at = (value: Value) => CARDS[pick(value).card];

const HIDE = () => false;

export const SPEC: Spec = [
  {
    key: "card",
    label: "Card",
    kind: "pick",
    def: 0,
    options: CARDS.map((one, i) => [i, `${i + 1} · ${one.title}`]),
    group: "Card",
    when: HIDE,
  },
  {
    key: "step",
    label: (value) => at(value).label,
    kind: "slider",
    def: CARDS[0].first,
    min: (value) => at(value).min,
    max: (value) => at(value).max,
    step: 1,
    show: (v, value) => shown(pick(value).card, v),
    group: "Card",
  },
];

export const named = (value: Value) => {
  const { card, step } = pick(value);
  return `tour-${card + 1}-${CARDS[card].key}-${step}`;
};

export const PAGE = {
  spec: SPEC,
  name: named,
  about: [
    "A design is a rule on the corners of a square or a cube. Grown level by level or widened side by side it counts something, and the count is an integer sequence in the OEIS, some of them new entries.",
    "Every card draws its design live, reads its terms and names the record. Slide the step or tap the picture, and the picture and the terms grow together; Enter turns to the next card.",
    "A line that is a row of the ledger opens in Sequences with its terms filled in. The counts of designs, of fractals, of slice vertices, of gasket points and of Farey nodes are not ledger rows. The record after the terms is found from the terms alone, as a window of the record, with the record index of the first term shown.",
  ],
  keys: [
    { key: "Enter", label: "Next card", act: "next", button: true, menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Card", act: "card" },
    { key: ["ArrowUp", "ArrowDown"], label: "Step", act: "step" },
  ],
  actions: {
    next: (scene) => scene.turn?.(1),
    card: (scene, e) => scene.turn?.(e.key === "ArrowLeft" ? -1 : 1),
    step: (scene, e) => scene.walk?.(e.key === "ArrowDown" ? -1 : 1),
  },
} satisfies Page<Ready & Units, Tour>;

const STORY = 4;

const LINES = 2;

const PRIME = "#f5b400";

const within = (box: Rect | null, x: number, y: number) =>
  box && x >= box.x && x <= box.x + box.w && y >= box.y && y <= box.y + box.h;

/* WORDS */

export function forms(line: Formed) {
  const out = [line.closed, line.records.formula].filter((form, i, all) => form && all.indexOf(form) === i);
  const said = line.records.short
    ? "three terms name the record"
    : line.records.found.length
      ? line.records.found.map(({ record, at }) => `${record.id} ${record.name}, from index ${at}`).join(" · ")
      : "no curated record holds these terms";
  return [...out, said].join(" · ");
}

/* ART */

function ink(kind: string, t: Tones): Inks | null {
  if (kind === "voids") return { fill: blend(t.accent, t.paper, 0.55), void: blend("#ff8a00", t.paper, 0.1) };
  if (kind === "edges") return { fill: blend(PRIME, t.paper, 0.1), void: veil(PRIME, 0.1) };
  return null;
}

function rows(ctx: CanvasRenderingContext2D, box: Rect, one: ArtOf<"rows">, t: Tones) {
  const n = one.rows.length;
  const cols = Math.max(1, ...one.rows.map((row) => row.sheets.length));
  ctx.font = font(t, 0.95);
  const name = Math.round(
    Math.min(box.w * 0.4, Math.max(t.px * 9, ...one.rows.map((row) => ctx.measureText(row.name).width + t.px))),
  );
  ctx.font = font(t);
  const cell = Math.min((box.h - (n - 1) * t.px * 0.5) / n, (box.w - name) / cols);
  one.rows.forEach((row, i) => {
    const y = box.y + i * (cell + t.px * 0.5);
    caption(ctx, row.name, box.x, y + cell / 2 + t.px * 0.35, name - t.px * 0.5, t, "left");
    row.sheets.forEach((sheet, j) => {
      const slot = { x: box.x + name + j * cell, y, w: cell * 0.9, h: cell * 0.9 };
      art(ctx, slot, sheet, inks({}, t));
    });
  });
}

function gallery(ctx: CanvasRenderingContext2D, box: Rect, one: ArtOf<"gallery">, t: Tones) {
  const n = one.sheets.length;
  const cols = Math.ceil(Math.sqrt(n * (box.w / Math.max(1, box.h))));
  const lines = Math.ceil(n / cols);
  const cell = Math.min(box.w / cols, box.h / lines);
  one.sheets.forEach((sheet, i) => {
    const slot = {
      x: box.x + (i % cols) * cell + cell * 0.05,
      y: box.y + Math.floor(i / cols) * cell + cell * 0.05,
      w: cell * 0.9,
      h: cell * 0.9,
    };
    art(ctx, slot, sheet, inks({}, t));
  });
}

function dots(ctx: CanvasRenderingContext2D, box: Rect, one: ArtOf<"dots">, t: Tones) {
  const pts = one.points.map(([x, y, z]) => [(x - y) * 0.8660254037844386, z - (x + y) / 2]);
  if (!pts.length) return;
  const xs = pts.map(([x]) => x);
  const ys = pts.map(([, y]) => y);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const k = Math.min(box.w / (x1 - x0 || 1), box.h / (y1 - y0 || 1)) * 0.9;
  const r = Math.max(t.dpr, Math.min(t.px * 0.6, k * 0.32));
  const cx = box.x + box.w / 2 - ((x0 + x1) / 2) * k;
  const cy = box.y + box.h / 2 + ((y0 + y1) / 2) * k;
  ctx.fillStyle = t.accent;
  ctx.beginPath();
  for (const [x, y] of pts) {
    ctx.moveTo(cx + x * k + r, cy - y * k);
    ctx.arc(cx + x * k, cy - y * k, r, 0, Math.PI * 2);
  }
  ctx.fill();
}

function farey(ctx: CanvasRenderingContext2D, box: Rect, one: ArtOf<"farey">, t: Tones) {
  ctx.strokeStyle = t.accent;
  ctx.lineWidth = Math.max(t.dpr, t.dpr * 1.6);
  for (const [num, den, bright] of one.nodes) {
    const x = box.x + (box.w * num) / den;
    ctx.globalAlpha = 0.25 + (0.75 * bright) / one.order;
    ctx.beginPath();
    ctx.moveTo(x, box.y + box.h);
    ctx.lineTo(x, box.y + box.h - (box.h * bright) / one.order);
    ctx.stroke();
  }
  ctx.globalAlpha = 1;
}

function picture(ctx: CanvasRenderingContext2D, box: Rect, one: Art | null, t: Tones, shown: (row: Grown) => number) {
  if (!one) return;
  if (one.kind === "sheet") art(ctx, box, one.sheet, ink(one.ink, t) ?? inks({}, t));
  else if (one.kind === "rows") rows(ctx, box, one, t);
  else if (one.kind === "gallery") gallery(ctx, box, one, t);
  else if (one.kind === "dots") dots(ctx, box, one, t);
  else if (one.kind === "farey") farey(ctx, box, one, t);
  else if (one.kind === "pins")
    chart(ctx, box, [{ key: "pins", terms: one.terms, start: 1, say: one.say }], [t.accent], t, {
      kind: "log",
      at: -1,
      shown: () => shown({ key: "pins", terms: one.terms }),
    });
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const feed = () => opts.live?.current ?? opts;
  const grown = growth(view);
  let seen: number | null = null;
  let dirty = true;
  let moving = false;
  let pic: Rect | null = null;
  const paint = (data: Feed | Given) => {
    const t = tones(view);
    clear(ctx, view, t);
    const { px } = t;
    const { x: pad, top: high, bottom: low } = t.edge;
    const made = data.made;
    if (!made) {
      ctx.fillStyle = t.dim;
      ctx.textAlign = "center";
      ctx.fillText("reading", view.w / 2, view.h / 2);
      ctx.textAlign = "left";
      pic = null;
      return;
    }
    const card = CARDS[data.card];
    const width = view.w - 2 * pad;
    ctx.font = font(t, 1.45, "600");
    ctx.fillStyle = t.accent;
    ctx.fillText(fit(ctx, `${data.card + 1} · ${card.title}`, width), pad, high + px * 1.3);
    ctx.font = font(t);
    ctx.fillStyle = t.dim;
    ctx.fillText(fit(ctx, [data.shown, made.sub].filter(Boolean).join(" · "), width), pad, high + px * 2.9);
    const top = high + px * 4;
    const wide = width > (view.h - top) * 1.2;
    const text = wide ? { x: pad + width * 0.5 + px * 1.5, w: width * 0.5 - px * 1.5 } : { x: pad, w: width };
    ctx.font = font(t, 0.95);
    const story = wrap(ctx, card.say, text.w).slice(0, wide ? STORY + 3 : STORY);
    const lay = made.lines.map((line) => ribbon(ctx, line, t, text.w, LINES));
    const tall = lay.reduce((sum, one) => sum + one.height + px * 3.2, 0);
    const words = story.length * px * 1.45 + px * 0.8;
    pic = wide
      ? { x: pad, y: top, w: width * 0.5, h: view.h - top - low }
      : { x: pad, y: top + words, w: width, h: Math.max(px * 6, view.h - top - words - tall - low - px) };
    picture(ctx, pic, made.art, t, (row) => grown.shown(row));
    let y = wide ? top + px : top + px;
    ctx.font = font(t, 0.95);
    ctx.fillStyle = blend(t.accent, t.paper, 0.3);
    story.forEach((line) => {
      ctx.fillText(line, text.x, y);
      y += px * 1.45;
    });
    y = wide ? y + px * 0.8 : pic.y + pic.h + px * 1.2;
    made.lines.forEach((line, i) => {
      if (y > view.h - low) return;
      ctx.font = font(t, 0.85);
      ctx.fillStyle = t.dim;
      ctx.fillText(fit(ctx, `${line.label}${line.capped ? ", to the budget" : ""}`, text.w), text.x, y);
      drawRibbon(ctx, lay[i], text.x, y + px * 0.2, t, -1, t.accent);
      y += lay[i].height + px * 1.3;
      ctx.font = font(t, 0.8);
      ctx.fillStyle = t.dim;
      ctx.fillText(fit(ctx, forms(line), text.w), text.x, y);
      y += px * 1.9;
    });
    if (made.error) {
      ctx.fillStyle = t.dim;
      ctx.fillText(fit(ctx, made.error, text.w), text.x, view.h - low);
    }
    ctx.font = font(t);
  };
  const draw = () => {
    const data = feed();
    const version = data.version ?? 0;
    const rows = data.made?.art?.kind === "pins" ? [{ key: "pins", terms: data.made.art.terms }] : [];
    const busy = grown.busy(rows);
    if (!view.still && !dirty && version === seen && !busy && !moving) return;
    moving = busy;
    seen = version;
    dirty = false;
    paint(data);
  };
  const again = () => {
    dirty = true;
    draw();
  };
  const tap = (x: number, y: number) => (within(pic, x, y) ? { picture: true } : null);
  return { draw, size: again, theme: again, tap, font: FACE, still: true };
}
