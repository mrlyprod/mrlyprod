import { rgb } from "@mrlyprod/ui/kit/color.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Raw, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
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

export type Seen = Pick<View, "w" | "h" | "dpr"> & { look: () => { paper: string; accent: string; ink: string } };

export type Given = Opts & {
  readings?: Num | null;
  onFacts?: (facts: Facts) => void;
  later?: (job: () => void) => void;
};

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

export const units = wasm("readings");

export function file(value: Raw, given?: { readings?: Num }) {
  const one = tidied(value);
  const design = designOf(one);
  const name = given?.readings
    ? slug(given.readings, design)
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
} satisfies Page<Loaded<"readings">, Handled>;

const soon = (job: () => void) => setTimeout(job, 0);

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { readings: num } = opts;
  if (!num) throw new Error("readings: the scene wants @mrlyprod/readings as opts.readings");
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
