import { rgb } from "../../net/sdk/color.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Raw, Spec, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Units as Loaded } from "../shared/units/index.ts";
import { wasm } from "../shared/units/index.ts";
import type { Facts, Inks, Knobs, Num, Plan } from "./engine.ts";
import {
  BASES,
  baseOf,
  batteryCsv,
  biasCsv,
  cap,
  designOf,
  digitsOf,
  inks,
  POWERS,
  READINGS,
  sketch,
  slug,
  stamp,
  stepped,
  study,
  turned,
  VIEWS,
  warm,
} from "./engine.ts";

/* TYPES */

export type Seen = Pick<View, "w" | "h" | "dpr"> & { look: () => { paper: string; accent: string; ink: string } };

export type Given = Opts & { num?: Num | null; onFacts?: (facts: Facts) => void; later?: (job: () => void) => void };

export type Hands = {
  toggle: () => void;
  playing: () => boolean;
};

export type Stage = ReturnType<typeof make>;

export type Handled = Stage & Hands;

const digital = (value: Value) => Number(value.rudin) !== 1;

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "plaid", options: VIEWS, group: "View" },
  {
    key: "power",
    label: "N",
    kind: "slider",
    def: 12,
    min: POWERS.min,
    max: POWERS.max,
    step: 1,
    show: (v) => `2^${v} = ${2 ** v}`,
    group: "View",
  },
  {
    key: "width",
    label: "Row",
    kind: "slider",
    def: 0,
    min: 0,
    max: (value) => cap(baseOf(value), 2 ** Number(value.power)),
    step: 1,
    zero: "Fit",
    show: (v, value) => `${baseOf(value)}^${v} = ${baseOf(value) ** v} wide`,
    group: "View",
    when: (value) => value.view === "plaid",
  },
  { key: "rudin", label: "Rudin-Shapiro", kind: "toggle", def: 0, group: "Design" },
  { key: "base", label: "Base", kind: "segment", def: 2, options: BASES, group: "Design", when: digital },
  {
    key: "digits",
    label: "Digits",
    kind: "list",
    def: "1",
    sep: "",
    options: (value) => digitsOf(Number(value.base)),
    group: "Design",
    when: digital,
  },
  { key: "reading", label: "Reading", kind: "segment", def: "Square", options: READINGS, group: "Reading" },
];

export const tidied = (value: Raw) => tidy(SPEC, value) as Knobs;

export const units = wasm("num");

export function file(value: Raw, given?: { num?: Num }) {
  const one = tidied(value);
  const design = designOf(one);
  const name = given?.num
    ? slug(given.num, design)
    : design.rudin
      ? "rudin-shapiro"
      : `${design.base}-${digitsOf(design.base)
          .filter((d) => design.mask & (1 << (Number(d) - 1)))
          .join("")}`;
  return ["readings", name, one.reading.toLowerCase(), one.view, one.power].join("-");
}

export const PAGE = {
  spec: SPEC,
  units,
  name: file,
  about: [
    "A design is a digit rule: Thue-Morse is the parity of the binary ones, b:D the parity of the base-b digits of n that fall in D, Rudin-Shapiro the parity of the 11 blocks. A reading feeds the design a polynomial or the primes, y(n) = x(P(n)). Plaid lays y(n) for n below N as a raster, a row a power of the base, orange for +1 and blue for -1. Read along n the picture is a plaid, since the digit rule factors over the rows; read along the squares it is noise, and Thue-Morse along the squares is proved normal. Play steps N up through the powers of two, and so does a tap or Reroll.",
    "Battery runs the three tests of the design ladder on the stream: the shift test compares y(n) with y(n + h) for h up to 64, the dilation test with y(Bn + r) for B from 2 to 9, the kernel test y(bn) with y(b²n) for b = 2 and 3, and the square reads y(n) y(2n) y(3n) y(6n). Each bar is z = |r| times the square root of N on a log scale, the band marks z = 2 and 3, and a bar above it lights. A pair is outlined when it is a near-head, P(Bn + r) = b^L (P(n) + q) + s, where the digits of the design concatenate and the correlation tends to a constant the digits decide; a solid outline reads exactly plus or minus one. Along n² the dilation at (2, 0) reads 1 on a normal stream: normal is not random.",
    "Arcs draws the bias S(N), the sum of y(n) over n < N, at every power of two in units of the square root of N, beside the prediction of the circle method's major arcs: the design's periodic orbits below the exponent 1/4, each weighted by a quadratic Gauss sum and a B-adic integral. The model is for the squares; along another reading the bias stands alone. The dashed line runs ahead of the data to 2^20, so the law and the measurement sit on one picture, and the residual is in the readouts.",
    "The numbers come from the readings door of the number crate. The mathematics is in [the random chapters](/research/random/): the readings, along the squares, along the primes and the major arcs.",
  ],
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.playing?.() ? "Stop" : "Play"),
      act: "play",
      button: true,
      menu: "Edit",
    },
    { key: "ArrowUp", label: "N up", act: "up", menu: "View" },
    { key: "ArrowDown", label: "N down", act: "down", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "View", act: "view" },
  ],
  actions: {
    play: (scene) => scene.toggle?.(),
  },
  acts: {
    up: (value) => stepped(value, 1),
    down: (value) => stepped(value, -1),
    view: (value, key) => turned(value, key === "ArrowLeft" ? -1 : 1),
  },
  reroll: (value) => stepped(value, 1),
} satisfies Page<Loaded<"num">, Handled>;

const soon = (job: () => void) => setTimeout(job, 0);

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { num } = opts;
  if (!num) throw new Error("readings: the scene wants @mrlyprod/num as opts.num");
  const later = opts.later ?? soon;
  const ctx = canvas.getContext("2d")!;
  let value = tidied(opts);
  let plan = study(num, value);
  let ink: Inks = inks(view.look());
  let source: OffscreenCanvas | null = null;
  let dirty = true;
  let gone = false;
  let asked = false;
  const tell = () => opts.onFacts?.(plan.facts);
  const paint = () => {
    ink = inks(view.look());
    source = plan.view === "plaid" ? stamp(plan.stream, plan.width, plan.rows, rgb(ink.plus), rgb(ink.minus)) : null;
  };
  const settle = () => {
    if (plan.view !== "arcs" || !plan.facts.pending || asked) return;
    asked = true;
    later(() => {
      asked = false;
      if (gone || plan.view !== "arcs" || !plan.facts.pending) return;
      warm(num, designOf(value));
      plan = study(num, value);
      paint();
      dirty = true;
      draw();
      tell();
    });
  };
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    sketch(ctx, view.w, view.h, view.dpr, plan, ink, source);
    settle();
  };
  const size = () => {
    dirty = true;
  };
  const theme = () => {
    paint();
    dirty = true;
  };
  const set = (next: Opts) => {
    value = tidied(next);
    const was = plan;
    plan = study(num, value);
    if (plan !== was) paint();
    dirty = true;
    if (plan !== was) tell();
  };
  const stop = () => {
    gone = true;
  };
  const csv = (kind: "battery" | "bias") => (kind === "battery" ? batteryCsv(num, value) : biasCsv(num, value));
  paint();
  return { draw, size, theme, set, stop, csv, still: true, facts: (): Plan["facts"] => plan.facts };
}
