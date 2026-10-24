import { tidy } from "../../lib/app/spec.js";
import { veil } from "../../lib/color.js";
import { population, sow } from "../../lib/life/engine.js";
import { PANE, reach, runs, SOUP } from "../../lib/life/scene.js";
import { rng } from "../../lib/rand.js";

const STARTS = [
  ["one", "One cell"],
  ["noise", "Soup"],
  ["design", "Carpet row"],
];

export const SPEC = [
  { key: "rule", label: "Rule", kind: "slider", def: 110, min: 0, max: 255, step: 1, group: "Rule", when: PANE },
  { key: "from", label: "Start", kind: "pick", def: "one", options: STARTS, group: "Run" },
  {
    key: "density",
    label: "Density",
    kind: "slider",
    def: 0.35,
    min: 0.05,
    max: 0.95,
    step: 0.01,
    group: "Run",
    when: SOUP,
  },
  { key: "wrap", label: "Wrap", kind: "toggle", def: 0, group: "Run" },
  {
    key: "steps",
    label: "Generations",
    kind: "segment",
    def: 128,
    options: [64, 128, 256, 512].map((n) => [n, String(n)]),
    group: "Run",
  },
];

export const FAMOUS = [
  [30, "chaos"],
  [90, "Sierpinski"],
  [110, "universal"],
  [184, "traffic"],
  [54, "complex"],
  [150, "XOR"],
  [45, "random"],
  [22, "nested"],
  [60, "left XOR"],
  [102, "right XOR"],
  [126, "fractal"],
  [105, "XNOR"],
];

export const PAGE = {
  spec: SPEC,
  design: { dim: 3, base: 2, code: "rule" },
  gestures: true,
  about: [
    "An elementary rule writes one bit on each of the eight neighbourhoods (l, c, r) of a cell, so a rule is a byte. A three-dimensional design writes one bit on each of the eight corners of a cube, so a design is a byte too.",
    "They are the same byte: rule N is the design bang dim 3, code N, bit for bit. So every invariant of a design is already an invariant of a rule: the popcount, the GF(2) degree, the genus, and the fill that Langton calls lambda.",
    "The diagram runs one live cell down the page, a row a generation, on a line padded so the edge never shows; Wrap runs it on a ring. Tap the top row to edit the seed.",
    "Two groups act on the byte: Wolfram's reflection and conjugation, which keeps the dynamics, and the 48 symmetries of the cube, which does not. Pick a rule from the Library to run it, or step the rules with the arrow keys. [Life](/life/) runs the same kind of rule on any mask, [Designs](/designs/) is the gallery the bytes come from, and [the automata note](/research/automata/) holds the class lattice and the census.",
  ],
  name: (value) =>
    [
      "wolfram",
      value.rule,
      ...(value.from === "noise" ? ["soup", value.seed] : value.from === "design" ? ["carpet"] : []),
      ...(value.wrap ? ["ring"] : []),
    ].join("-"),
  keys: [
    { key: ["ArrowRight", "Enter", "."], label: "Next rule", act: "next", button: true, menu: "View" },
    { key: ["ArrowLeft", ","], label: "Previous rule", act: "prev", menu: "View" },
  ],
  actions: { next: (scene) => scene.turn?.(1), prev: (scene) => scene.turn?.(-1) },
};

const FIT = 0.96;
const GROUND = 0.07;
const THUMB = 24;
const MARGIN = 6;

/* CARD */

export function card(life, rule) {
  const mates = Array.from(life.cube_orbit(rule), (one) => ({ rule: one, onto: life.surjective(one) }));
  const told = life.outer_totalistic(rule);
  return {
    name: life.rule_name(rule),
    corners: Uint8Array.from(life.corner_bits(rule)),
    popcount: life.popcount(rule),
    lambda: life.lambda(rule),
    degree: life.rule_degree(rule),
    genus: life.genus(rule),
    affine: life.affine(rule),
    onto: life.surjective(rule),
    reversible: life.reversible(rule),
    bs: told ? `B${Array.from(told[0]).join("")}/S${Array.from(told[1]).join("")}` : "",
    mates,
    mixed: mates.some((one) => one.onto) && mates.some((one) => !one.onto),
    klass: Array.from(life.wolfram_class(rule)),
    npn: life.npn_class(rule)[0],
    gasket: life.gasket(rule) ?? "",
  };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const ctx = canvas.getContext("2d");
  const { life, math } = opts;
  let value = { ...tidy(SPEC, opts), seed: opts.seed };
  let edited = null;
  let grid = null;
  let how = "";
  let skin = null;
  let lay = null;
  let span = null;
  let dirty = true;
  let gone = false;
  let brush = 1;
  const build = () => {
    const { rule, steps, from } = value;
    const wrap = Boolean(value.wrap);
    const width = 2 * steps + 1;
    const ring = wrap ? ` on a ring of ${width}` : "";
    let made;
    if (edited) {
      made = life.history(edited, rule, steps, wrap);
      how = `a seed row edited by hand${ring}`;
    } else if (from === "one" && !wrap) {
      made = life.single_seed(rule, steps);
      how = `one live cell, padded by ${steps} and cropped to ${width}`;
    } else {
      made = life.history(sow({ ...value, code: "7", number: 3 }, rng(value.seed), math, width), rule, steps, wrap);
      how = `${from === "noise" ? `a soup of ${width} cells at density ${value.density}` : from === "design" ? "the middle row of the carpet" : "one live cell"}${ring}`;
    }
    grid = { rows: made.shape[0], cols: made.shape[1], data: made.data };
  };
  const tell = () => !gone && opts.onStat?.({ rows: grid.rows, cols: grid.cols, live: population(grid.data), how });
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const lit = () => {
    const box = reach(grid.data, grid.cols, grid.rows);
    return box ? [Math.max(0, box.x0 - MARGIN), Math.min(grid.cols - 1, box.x1 + MARGIN)] : [0, grid.cols - 1];
  };
  const layout = () => {
    const [lo, hi] = span;
    const cols = hi - lo + 1;
    const cell = Math.min((view.w * FIT) / cols, (view.h * FIT) / grid.rows);
    const w = cell * cols;
    const h = cell * grid.rows;
    return { x: (view.w - w) / 2, y: (view.h - h) / 2, w, h, cell, lo, cols };
  };
  const paint = () => {
    lay = layout();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = skin.dim;
    ctx.fillRect(lay.x, lay.y, lay.w, lay.h);
    ctx.fillStyle = skin.ink;
    ctx.save();
    ctx.beginPath();
    ctx.rect(lay.x, lay.y, lay.w, lay.h);
    ctx.clip();
    runs(ctx, grid.data, grid.cols, grid.rows, lay.x - lay.lo * lay.cell, lay.y, lay.cell);
    ctx.restore();
    dirty = false;
  };
  const draw = () => {
    if (dirty) paint();
  };
  const theme = () => {
    dress();
    paint();
  };
  const redo = (keep) => {
    build();
    if (!keep || !span) span = lit();
    tell();
    paint();
  };
  const touch = (u, v, start) => {
    if (!lay) return;
    const col = Math.floor((u * view.w - lay.x) / lay.cell) + lay.lo;
    if (col < lay.lo || col >= lay.lo + lay.cols) return;
    if (Math.abs(v * view.h - lay.y - lay.cell / 2) > Math.max(lay.cell, THUMB * view.dpr)) return;
    edited ??= Uint8Array.from(grid.data.subarray(0, grid.cols));
    if (start) brush = edited[col] ? 0 : 1;
    if (edited[col] === brush) return;
    edited[col] = brush;
    redo(true);
  };
  const set = (next) => {
    const was = value;
    value = { ...tidy(SPEC, next), seed: next.seed };
    if (["steps", "from", "density", "seed"].some((key) => String(value[key]) !== String(was[key]))) edited = null;
    redo();
  };
  const stop = () => {
    gone = true;
  };
  dress();
  build();
  span = lit();
  tell();
  return { still: true, draw, size: paint, theme, set, touch, stop };
}
