import type { Mathy, Page } from "../../lib/app/page.tsx";
import type { Opts, Scene, View } from "../../lib/app/run.ts";
import type { Patch, Raw, Spec, Value } from "../../lib/app/spec.ts";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { rgb } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import type { Disc, Plan, Src } from "../../lib/spin/engine.ts";
import {
  BEAT,
  cells,
  edge,
  key,
  LIMIT,
  layout,
  NEEDLES,
  PRESETS,
  pace,
  paint,
  RPM,
  rings,
  seen,
  spun,
  step,
  stops,
  study,
} from "../../lib/spin/engine.ts";
import type { Look } from "../../lib/spin/scene.ts";
import { LOOK, PAD, place, rim, SOURCE, sheet, TAU, tag } from "../../lib/spin/scene.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";

/* TYPES */

export type Spin = Src & Look & { view: string; rpm: number; glow: number };

export type Pace = { hz: number; deg: number; seen: number };

export type Facts = Pick<
  Plan,
  "kind" | "code" | "name" | "side" | "fills" | "mass" | "disc" | "peak" | "reach" | "order"
>;

export type Given = Opts & {
  math?: Mrly | null;
  onPace?: (pace: Pace | null) => void;
  onFacts?: (facts: Facts) => void;
};

export type Spun = Scene & { speed?: (by: number) => void; needle?: (rpm: number) => void };

type Saver = { id: string; title: string; pick: (rand: Rng) => Patch; still?: number };

export const VIEWS: [string, string][] = [
  ["table", "Turntable"],
  ["wheel", "Wheel"],
];

export const spinning = (value?: Value) => value?.view !== "wheel";

const ramped = (value: Value) => !spinning(value) || value.source === "moire";

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "table", options: VIEWS },
  { key: "rpm", label: "Speed", kind: "slider", def: 33, min: 0, max: RPM, step: 1, unit: "rpm", when: spinning },
  { key: "glow", label: "Afterglow", kind: "slider", def: 1, min: 1, max: 120, step: 1, when: spinning },
  ...LOOK.map((row) => (row.key === "tint" ? row : { ...row, when: ramped })),
];

const SPEED: Record<string, number> = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: 10, ArrowDown: -10 };

const ABOUT = [
  "A design on a turntable, drawn once a screen frame. The screen strobes it: at 60 Hz the carpet freezes at 900 rpm, a quarter turn a frame, and just off that speed it drifts slowly backwards.",
  "Afterglow blends each new frame into the old ones, the way the eye does.",
  "The Wheel view spins the design infinitely fast: every pixel becomes the mean of the design on its own circle. It is exact: each circle is cut at the grid lines it crosses, and the rings carry the mass of the design.",
  "A plane wave averaged over a turn is the Bessel function J0, so the wheel is the Hankel transform of the radially averaged spectrum. The identities are in [the spin topic](/research/spin/) of Research.",
];

export const name = (value: Value, units?: Mathy) => `spin-${tag(value, units?.math)}-${value.view}`;

export const units = wasm("math");

export const PAGE = {
  defaults: defaults(SOURCE),
  spec: SPEC,
  units,
  design: { dim: "dim", base: "base", code: "code", when: (value) => value.source !== "moire" },
  record: true,
  about: ABOUT,
  name,
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.playing?.() === false ? "Spin" : "Stop"),
      act: "spin",
      button: true,
      menu: "Edit",
      when: spinning,
    },
    { key: Object.keys(SPEED), label: "Speed", act: "speed", when: spinning },
    {
      key: NEEDLES.map((_, i) => String(i + 1)),
      label: "Needle",
      act: "needle",
      does: `${NEEDLES.join(", ")} rpm`,
      when: spinning,
    },
  ],
  actions: {
    spin: (scene) => scene.toggle?.(),
    speed: (scene, e) => scene.speed?.(SPEED[e.key!]),
    needle: (scene, e) => scene.needle?.(NEEDLES[Number(e.key) - 1]),
  },
} satisfies Page<Loaded<"math">, Spun>;

/* SAVERS */

const FILLS: Record<number, [number, number]> = { 2: [2, 3], 3: [4, 8] };

const DEPTH: Record<number, number> = { 3: 4, 5: 3 };

const TRIES = 16;

const lit = (code: number) => code.toString(2).replaceAll("0", "").length;

const odd = (rand: Rng, from: number, to: number) => from + 2 * Math.floor(rand() * ((to - from) / 2 + 1));

function carve(rand: Rng, base: number) {
  const [least, most] = FILLS[base];
  const total = 2 ** (base ** 2) - 2;
  let code = 1 + Math.floor(rand() * total);
  for (let n = 0; n < TRIES && (lit(code) < least || lit(code) > most); n++) code = 1 + Math.floor(rand() * total);
  return String(code);
}

export const SAVERS: Saver[] = [
  {
    id: "table",
    title: "Turntable",
    pick: (rand) => {
      const base = rand() < 0.75 ? 3 : 2;
      const number = pick(rand, [3, 3, 5]);
      return {
        source: "design",
        dim: 2,
        base,
        code: carve(rand, base),
        number,
        level: DEPTH[number],
        rpm: 6,
        glow: 12,
      };
    },
  },
  {
    id: "moire",
    title: "Moire disc",
    pick: (rand) => ({
      source: "moire",
      preset: pick(rand, PRESETS),
      limit: odd(rand, 9, LIMIT),
      ramp: "tint",
      levels: pick(rand, [12, 24, 64]),
      rpm: 4,
    }),
  },
];

const GROUND = 36;

const LOOKS: (keyof Look)[] = ["ramp", "levels", "invert", "tint"];

export const read = (value: Raw) => ({ ...tidy(SOURCE, value), ...tidy(SPEC, value) }) as Spin;

/* SCENE */

function blank(canvas: HTMLCanvasElement, view: View) {
  const ctx = canvas.getContext("2d")!;
  const draw = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
  };
  return { draw };
}

export const facts = (plan: Plan): Facts => ({
  kind: plan.kind,
  code: plan.code,
  name: plan.name,
  side: plan.side,
  fills: plan.fills,
  mass: plan.mass,
  disc: plan.disc,
  peak: plan.peak,
  reach: plan.reach,
  order: plan.order,
});

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Scene {
  const math = opts.math;
  if (!math) return blank(canvas, view);
  const ctx = canvas.getContext("2d")!;
  let given = opts;
  let value = read(given);
  let plan: Plan | null = null;
  let look = view.look();
  let line = edge(look);
  let disc: Disc | null = null;
  let stamp: OffscreenCanvas | null = null;
  let table: OffscreenCanvas | null = null;
  let wheel: OffscreenCanvas | null = null;
  let fresh = true;
  let dirty = true;
  let phase = 0;
  let last = view.t;
  let beat = view.t;
  let ema = 0;
  let told = false;
  let stopped = false;
  const marked = () => key(value);
  const learn = () => {
    plan = study(math, value);
    given.onFacts?.(facts(plan));
  };
  const coat = () => {
    const bull = rings(math, plan!);
    wheel = sheet(paint(bull.values, bull.top, stops(value.ramp, look), value.levels, value.invert), bull.side);
  };
  const press = () => {
    const { side, data } = plan!;
    const raw = sheet(
      plan!.binary
        ? cells(data, rgb(look.accent), GROUND)
        : paint(data, plan!.top, stops(value.ramp, look), value.levels, value.invert),
      side,
    );
    const unit = Math.max(1, Math.ceil((disc!.r * Math.SQRT2) / side));
    stamp = new OffscreenCanvas(unit * side, unit * side);
    const pen = stamp.getContext("2d")!;
    pen.imageSmoothingEnabled = false;
    pen.drawImage(raw, 0, 0, unit * side, unit * side);
    const d = Math.ceil(disc!.r * 2);
    table = new OffscreenCanvas(d, d);
  };
  const spin = (dt: number) => {
    if (!stamp) press();
    const { x, y, r } = disc!;
    const d = table!.width;
    const pen = table!.getContext("2d")!;
    pen.setTransform(1, 0, 0, 1, 0, 0);
    if (value.glow === 1 || view.still || dt < 0) pen.clearRect(0, 0, d, d);
    else {
      pen.globalAlpha = 1 / value.glow;
      pen.fillStyle = look.paper;
      pen.beginPath();
      pen.arc(d / 2, d / 2, d / 2, 0, TAU);
      pen.fill();
      pen.globalAlpha = 1;
    }
    pen.translate(d / 2, d / 2);
    pen.rotate((phase * Math.PI) / 180);
    const span = r * Math.SQRT2;
    pen.drawImage(stamp!, -span / 2, -span / 2, span, span);
    ctx.drawImage(table!, x - d / 2, y - d / 2);
    rim(ctx, disc!, line, view.dpr);
  };
  const tell = (dt: number) => {
    if (view.still || stopped || !value.rpm || !spinning(value)) {
      if (told) given.onPace?.(null);
      told = false;
      return;
    }
    if (dt <= 0) return;
    ema = pace(dt, ema);
    if (view.t - beat < BEAT) return;
    beat = view.t;
    told = true;
    const deg = step(value.rpm, ema);
    given.onPace?.({ hz: 1000 / ema, deg, seen: seen(deg, plan!.order) });
  };
  const still = () => {
    if (!dirty) return;
    dirty = false;
    if (!wheel) coat();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    place(ctx, wheel!, disc!, line, view.dpr);
  };
  const draw = () => {
    if (fresh) {
      look = view.look();
      line = edge(look);
      wheel = null;
      disc = null;
      fresh = false;
    }
    if (!disc) {
      disc = layout(view.w, view.h, PAD * view.dpr);
      stamp = null;
    }
    const dt = view.t - last;
    last = view.t;
    if (!spinning(value)) {
      tell(dt);
      still();
      return;
    }
    if (dt > 0 && !view.still && !stopped) phase = spun(phase, value.rpm, dt);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    spin(dt);
    tell(dt);
  };
  const set = (next: Opts) => {
    const was = value;
    const at = marked();
    given = { ...given, ...next };
    value = read(given);
    if (marked() !== at) {
      learn();
      fresh = true;
    }
    if (LOOKS.some((name) => was[name] !== value[name])) fresh = true;
    if (was.rpm !== value.rpm) beat = view.t;
    dirty = true;
  };
  const theme = () => {
    fresh = true;
    dirty = true;
  };
  const size = () => {
    disc = null;
    dirty = true;
  };
  const toggle = () => {
    stopped = !stopped;
  };
  learn();
  return { draw, size, theme, set, toggle, playing: () => !stopped };
}
