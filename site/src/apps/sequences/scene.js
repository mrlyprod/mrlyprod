import { tidy } from "../../lib/app/spec.js";
import { KINDS } from "../../lib/sequences/chart.js";
import { draw as art, inks, picture, said } from "../../lib/sequences/design.js";
import { AXES, CANON, MEASURES, picks, TERMS } from "../../lib/sequences/engine.js";
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
} from "../../lib/sequences/paint.js";

export const SPEC = [
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
    options: [["", "Any"], ...MEASURES.map((one) => [one.slug, one.label])],
    group: "Rows",
  },
  {
    key: "axis",
    label: "Axis",
    kind: "segment",
    def: "",
    options: [["", "Both"], ...AXES.map((one) => [one.slug, one.label])],
    group: "Rows",
  },
];

export const named = (value) => ["sequences", picks(value.pick)[0] ?? value.seed, value.chart].join("-");

export const PAGE = {
  defaults: { q: "", pick: CANON },
  spec: SPEC,
  name: named,
  about: [
    "A design fills cells. Count them level by level, or widen the odd side at level one, and every count is an integer sequence: the fills, the voids, the exposed faces, the cells on the deepest diagonal plane, the pieces of the middle cut.",
    "This is the ledger of all of them, read live. Type a few terms, a name, a record or a code to find the design that writes them. A picked row is charted beside its design; tap a pin to draw the design at that term.",
    "The badge names the OEIS record holding the terms, and the number after it is the record index of the first term less the ledger index. Proved and verified rows are the record itself; a collision is a record that holds the terms but counts something else.",
    "Terms are read within a budget of cells a term, and a row the budget stops says so. Enter opens the row in Plot, which finds its recurrence and mixes it with a second row.",
  ],
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
    cycle: (scene) => scene.cycle?.(),
  },
};

const KEEP = 24;

const within = (box, x, y) => x >= box.x0 && x <= box.x1 && y >= box.y0 && y <= box.y1;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext("2d");
  const math = opts.math;
  const live = opts.live;
  let { chart: kind, digits: base } = tidy(SPEC, opts);
  const feed = () => live?.current ?? opts;
  const grown = growth(view);
  const pictures = new Map();
  let seen = null;
  let dirty = true;
  let at = -1;
  let lead = "";
  let count = 0;
  let moving = false;
  let hits = { legend: [], slots: null };
  const sheet = (row, index) => {
    if (!math || !row) return null;
    const id = `${row.key}:${index}`;
    if (!pictures.has(id)) {
      let made = null;
      try {
        made = picture(math, row, index);
      } catch {
        made = null;
      }
      pictures.set(id, made);
      if (pictures.size > KEEP) pictures.delete(pictures.keys().next().value);
    }
    return pictures.get(id);
  };
  const paint = (rows) => {
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
  const set = (value) => {
    ({ chart: kind, digits: base } = tidy(SPEC, value));
    dirty = true;
  };
  const tap = (x, y) => {
    const row = hits.legend.find((box) => within(box, x, y));
    if (row) return { row: row.i };
    const j = slotAt(hits.slots, x, y);
    if (j < 0) return null;
    at = j;
    again();
    return { term: j };
  };
  const step = (by) => {
    if (!count) return;
    at = Math.max(0, Math.min(count - 1, (at < 0 ? count - 1 : at) + by));
    again();
  };
  return { draw, size: again, theme: again, set, tap, step, font: FACE };
}
