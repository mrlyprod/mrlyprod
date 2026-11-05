import type { Color, Frame, Pen, Point } from "@mrlyprod/draw";
import { frame as box, hex, canvas as pen, svg as sheet } from "@mrlyprod/draw";
import type { Cell6d } from "@mrlyprod/math";
import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { rgb, TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Code, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { DESIGN, roll } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Had, Readout } from "@mrlyprod/ui/kit/hooks.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Nudge, Raw, Scalar, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import { DIM, design, memo, named, tag as title } from "../shared/three/engine.ts";

export type Spread = {
  low: number;
  high: number;
  span: number[];
  nonempty: number;
  heights: number;
  least: number;
  most: number;
  central: [number, number];
};

export type Empty = { counts: number[]; side: number; empty: true } & { [K in keyof Spread]?: undefined };

export type Full = { counts: number[]; side: number; empty?: undefined } & Spread;

export type Cut = Empty | Full;

export type Reach = { empty: true } | Pick<Full, "empty" | "low" | "high">;

export type Art = { mesh: Cell6d; svg?: undefined } | { mesh?: undefined; svg: string } | null;

export type Study = ReturnType<typeof study>;

export type Info = Pick<Study, "rows" | "cut" | "heights" | "code" | "base" | "level">;

export type Paint = { fill: number; void: number; accent: Color };

export type Drawn = { ctx: CanvasRenderingContext2D; img: HTMLImageElement };

export type Seen = Pick<View, "w" | "h" | "dpr"> & { look: () => Pick<Paints, "accent"> };

export type Given = Opts & { math?: Mrly | null; tell?: (info: Info) => void };

export type Cuts = ReturnType<typeof make>;

export const GASKET = "126";
export const NUMBER = 2;
export const REACH: Partial<Record<string, number>> & { points: number; section: number } = {
  points: 512,
  section: 64,
};
const SPAN = 512;
const PAD = 0.04;
const FAINT = 18;
const SHELF = 48;
const FIT = 0.9;
const TAU = Math.PI * 2;
const ROOT6 = Math.sqrt(6);

export const lid = (math: Mrly, view: Scalar) => math.bang.level_cap(NUMBER, 1, REACH[view] ?? REACH.points);

export const SPEC: Spec<Had<"math">> = [
  {
    key: "view",
    label: "View",
    kind: "segment",
    def: "points",
    options: [
      ["points", "Points"],
      ["section", "Section"],
    ],
    group: "Cut",
  },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 5,
    min: 1,
    max: (value, units) => units && lid(units.math, value.view),
    step: 1,
    group: "Cut",
  },
  { key: "both", label: "Both central heights", kind: "toggle", def: 0, group: "Cut" },
  {
    key: "tint",
    label: "Tint",
    kind: "pick",
    def: "",
    options: TINTS,
    group: "Look",
    when: (value) => value.view === "section",
  },
];

export const units = wasm("math");

export const name = (value: Value) => `cuts-${String(value.code ?? "").trim() || GASKET}-l${value.level}-${value.view}`;

export const PAGE = {
  defaults: { base: 2, code: "126", height: "" },
  design: { dim: 3, base: "base", code: "code" },
  spec: SPEC,
  units,
  gestures: true,
  name,
  keys: [
    { key: "ArrowLeft", label: "Height down", act: "lower", menu: "View" },
    { key: "ArrowRight", label: "Height up", act: "higher", menu: "View" },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level" },
  ],
  acts: {
    lower: slide(-1),
    higher: slide(1),
    level: (value, key) => ({ level: Number(value.level) + (key === "ArrowDown" ? -1 : 1) }),
  },
  reroll: (value, seed, units) => ({
    code: roll(units!.math, DIM, Number(tidy(DESIGN, value).base), rng(seed)),
    height: "",
  }),
} satisfies Page<Loaded<"math">, Cuts>;

const profiles = memo(6);
const drawings = memo(4);

export function profile(math: Mrly, code: Code, base: number, level: number): Cut {
  return profiles(`${code} ${base} ${level}`, (): Cut => {
    const counts = math.three.profile(code, NUMBER, level, base).map(Number);
    const side = Number(math.counts.grid(NUMBER, 1, level));
    const support = math.three.support(counts);
    if (!support) return { counts, side, empty: true };
    const [low, high] = support;
    const span = counts.slice(low, high + 1);
    const live = span.filter((count) => count > 0);
    const least = Math.min(...live);
    const most = Math.max(...live);
    const mid = Math.floor((low + high) / 2);
    return {
      counts,
      side,
      low,
      high,
      span,
      nonempty: live.length,
      heights: span.length,
      least,
      most,
      central: [mid, Math.min(high, mid + 1)],
    };
  });
}

export function heights(cut: Cut, value: Raw): number[] {
  if (cut.empty) return [];
  if (Number(value.both) === 1) return cut.central;
  const asked = String(value.height ?? "").trim();
  const at = asked === "" || !Number.isFinite(Number(asked)) ? cut.central[0] : Math.round(Number(asked));
  return [Math.min(cut.high, Math.max(cut.low, at))];
}

export function points(math: Mrly, code: Code, base: number, level: number, list: number[]) {
  return drawings(`${code} ${base} ${level} ${list}`, () =>
    math.three.diagonal_svg(
      code,
      NUMBER,
      level,
      base,
      Uint32Array.from(list),
      Math.max(1, Math.round(SPAN / 2 ** level)),
    ),
  );
}

export function section(math: Mrly, code: Code, base: number, level: number) {
  return drawings(`section ${code} ${base} ${level}`, () => math.six.cut_design(code, NUMBER, level, base));
}

export function plane(math: Mrly, value: Raw) {
  const { base, code } = design(math, {
    ...value,
    base: Number(value.base),
    code: String(value.code ?? "").trim() || GASKET,
  });
  const level = Math.min(Number(value.level), lid(math, String(value.view)));
  const cut = profile(math, code, base, level);
  return { base, code, level, cut, list: heights(cut, value) };
}

function slide(by: number): Nudge<Had<"math">> {
  return (value, _key, units) => {
    const { cut, list } = plane(units!.math, value);
    return cut.empty ? {} : { height: Math.min(cut.high, Math.max(cut.low, list[0] + by)), both: 0 };
  };
}

export function study(math: Mrly, value: Raw) {
  const { base, code, level, cut, list } = plane(math, value);
  const counts = list.map((h) => cut.counts[h] ?? 0);
  const drawn = value.view === "section" ? "-" : String(counts.reduce((a, b) => a + b, 0));
  const facts: Readout[] = cut.empty
    ? [
        ["name", title(named(math, code, base), code, base)],
        ["side", cut.side],
        ["support", "none"],
      ]
    : [
        ["name", title(named(math, code, base), code, base)],
        ["height", list.join(" and ")],
        ["this slice", counts.join(" and ")],
        ["offset in binary", list.map((h) => (h - cut.low).toString(2)).join(" and ")],
        ["side", cut.side],
        ["support", `[${cut.low}, ${cut.high}]`],
        ["non-empty", `${cut.nonempty} of ${cut.heights}`],
        ["min", cut.least],
        ["max", cut.most],
        ["points drawn", drawn],
      ];
  const art: Art = cut.empty
    ? null
    : value.view === "section"
      ? { mesh: section(math, code, base, level) }
      : { svg: points(math, code, base, level, list) };
  return { code, base, level, cut, heights: list, rows: facts, art, view: value.view };
}

export function shelf(cut: Reach, count = SHELF): number[] {
  if (cut.empty) return [];
  const { low, high } = cut;
  if (high - low < count) return Array.from({ length: high - low + 1 }, (_, i) => low + i);
  return Array.from({ length: count }, (_, i) => low + Math.round((i * (high - low)) / (count - 1)));
}

export function near(list: number[], at: number) {
  return list.reduce((best, one) => (Math.abs(one - at) < Math.abs(best - at) ? one : best), list[0]);
}

export function plot(math: Mrly, code: Code, base: number, level: number, height: number) {
  return math.three
    .diagonal_slice(code, NUMBER, level, base, height)
    .map(([x, y, z]): Point => [(x - y) / Math.SQRT2, (x + y - 2 * z) / ROOT6]);
}

export function thumb(ctx: CanvasRenderingContext2D, side: number, dots: Point[], edge: number, accent: string) {
  const reach = (edge - 1) * Math.sqrt(2 / 3);
  const scale = (FIT * side) / 2 / (reach + Math.SQRT1_2);
  const radius = Math.max(0.5, scale * Math.SQRT1_2 * FIT);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  ctx.fillStyle = accent;
  ctx.beginPath();
  for (const [u, v] of dots) {
    const x = side / 2 + u * scale;
    const y = side / 2 - v * scale;
    ctx.moveTo(x + radius, y);
    ctx.arc(x, y, radius, 0, TAU);
  }
  ctx.fill();
}

export const layout = (w: number, h: number) => box(0, 0, w, h).inset(Math.min(w, h) * PAD);

export function scrub(cut: Reach, x: number, w: number) {
  if (cut.empty) return null;
  const frac = Math.min(1, Math.max(0, x / Math.max(1, w)));
  return Math.round(cut.low + frac * (cut.high - cut.low));
}

export function fit(w: number, h: number, frame: Frame): [number, number, number, number] {
  const scale = Math.min(frame.w / w, frame.h / h);
  return [frame.x + (frame.w - w * scale) / 2, frame.y + (frame.h - h * scale) / 2, w * scale, h * scale];
}

export function picture(page: Pen, frame: Frame, plan: Pick<Study, "art">, paint: Paint, image: Drawn | null) {
  if (plan.art?.mesh) {
    const [r, g, b] = paint.accent;
    const faint: Color = [r, g, b, FAINT];
    hex.draw(page, frame, plan.art.mesh, 0, (type) =>
      type === paint.fill ? paint.accent : type === paint.void ? faint : null,
    );
  } else if (image?.ctx && plan.art?.svg) {
    image.ctx.drawImage(image.img, ...fit(image.img.naturalWidth || 1, image.img.naturalHeight || 1, frame));
  }
}

const settle = (opts: Given & { math: Mrly }) => ({
  ...tidy(SPEC, opts, { math: opts.math }),
  seed: opts.seed,
  base: opts.base,
  code: opts.code,
  height: opts.height,
});

const drawable = () =>
  typeof Image === "function" && typeof Blob === "function" && typeof URL?.createObjectURL === "function";

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { math, tell } = opts;
  if (!math) throw new Error("cuts: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const codes = { fill: math.six.FILL(), void: math.six.VOID() };
  let value = settle({ ...opts, math });
  let plan = study(math, value);
  let dirty = true;
  let img: HTMLImageElement | null = null;
  let wanted = "";
  const paint = (): Paint => ({ ...codes, accent: [...rgb(view.look().accent), 255] });
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    picture(pen(ctx, view.w, view.h), layout(view.w, view.h), plan, paint(), img && { ctx, img });
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const fetch = () => {
    const svg = plan.art?.svg;
    if (!svg || svg === wanted || !drawable()) return;
    wanted = svg;
    const one = new Image();
    const link = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml" }));
    one.onload = () => {
      URL.revokeObjectURL(link);
      if (wanted !== svg) return;
      img = one;
      touch();
    };
    one.src = link;
  };
  const report = () =>
    tell?.({
      rows: plan.rows,
      cut: plan.cut,
      heights: plan.heights,
      code: plan.code,
      base: plan.base,
      level: plan.level,
    });
  report();
  fetch();
  return {
    still: true,
    draw,
    size: touch,
    theme: touch,
    set: (next: Raw) => {
      value = settle({ ...next, math });
      plan = study(math, value);
      report();
      fetch();
      touch();
      return true;
    },
    seek: (x: number) => scrub(plan.cut, x, view.w),
    svg: async () => {
      if (plan.art?.svg) return plan.art.svg;
      const w = view.w / view.dpr;
      const h = view.h / view.dpr;
      const page = sheet(w, h);
      picture(page, layout(w, h), plan, paint(), null);
      return page.text();
    },
  };
}
