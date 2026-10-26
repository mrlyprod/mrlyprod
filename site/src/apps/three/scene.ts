import type { Palette } from "#view";
import { ink as inks } from "#view";
import type { Had, Readout, Three } from "../../lib/app/hooks.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Opts } from "../../lib/app/run.ts";
import type { Row, Scalar, Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Paints } from "../../lib/color.ts";
import { rgb, TINTS } from "../../lib/color.ts";
import type { Code, Mrly } from "../../lib/designs/engine.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import { dark, light } from "../../lib/theme/theme.ts";
import type {
  Axis,
  Centre,
  Coded,
  Crop,
  Made,
  Merge,
  Paint,
  Plane,
  Policy,
  RampName,
  Stacking,
  Wall,
} from "../../lib/three/engine.ts";
import {
  CARPET,
  COMBINES,
  CUBES,
  cap,
  circle,
  colorizer,
  counted,
  cropped,
  DEN,
  design,
  FACES,
  faces,
  field,
  grown,
  LAYERS,
  load,
  most,
  NUMBERS,
  named,
  PLANES,
  plane,
  RAMPS,
  REACH,
  reading,
  region,
  SHAPES,
  SIZES,
  shell,
  tag,
  tally,
  top,
  walls,
} from "../../lib/three/engine.ts";
import type { Units } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Body, Seen, Stage } from "./stage.ts";
import { space } from "./stage.ts";

/* TYPES */

export type Source = "design" | "field";

export type Knobs = Record<`c${Axis}` | `p${Axis}`, number> & {
  source: Source;
  number: number;
  level: number;
  draw: "shell" | "cubes";
  opacity: number;
  spin: number;
  camera: "eye" | "iso";
  radius: number;
  anti: number;
  policy: Policy;
  edge: number;
  limit: number;
  combine: Merge;
  depth: number;
  size: number;
  shell: number;
  threshold: number;
  ramp: RampName;
  levels: number;
  invert: number;
  circle: number;
  centre: Centre;
  count: number;
  r: number;
  tint: string;
} & ({ crop: "" } | { crop: Crop });

export type Solid = Knobs & { seed?: number; base?: Scalar; code?: Code | null };

export type Told = { rows: Readout[]; error: string };

export type Given = Opts & {
  math?: Mrly | null;
  three?: Three | null | false;
  tell?: (told: Told) => void;
  base?: Scalar;
  code?: Code | null;
};

export type Viewer = Seen & { look: () => Pick<Paints, "accent"> };

export type Study = {
  body: Body | null;
  clip: Wall[] | null;
  planes: Plane[];
  rows: Readout[];
  error: string;
  obj: () => string;
};

export type Hands = {
  deeper: (by: number) => void;
  widen: (by: number) => void;
  toggle: () => void;
  playing: () => boolean;
};

export type Sponge = ReturnType<typeof make>;

export type Held = Sponge & Hands;

const word = (one: string) => one[0].toUpperCase() + one.slice(1);
const fixed = (v: number, k: number) => Number(v).toFixed(k);
const is = (key: string, want: Scalar) => (value: Value) => value[key] === want;
const crop = (value: Value) => Boolean(value.crop);
const HIDE = () => false;
const grows = is("source", "design");
const fills = is("source", "field");
const counting = (value: Value) => grows(value) && Number(value.circle) === 1;

export const CROPS: [Crop | "", string][] = [["", "None"], ...SHAPES.map((one): [Crop, string] => [one, word(one)])];

export const SPEC: Spec<Had<"math">> = [
  {
    key: "source",
    label: "Solid",
    kind: "segment",
    def: "design",
    options: [
      ["design", "Design"],
      ["field", "Field"],
    ],
    group: "Solid",
  },
  {
    key: "number",
    label: "Number",
    kind: "pick",
    def: 3,
    options: NUMBERS.map((n) => [n, String(n)]),
    group: "Solid",
    when: grows,
  },
  { key: "crop", label: "Shape", kind: "pick", def: "", options: CROPS, group: "Crop", when: HIDE },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value, units) => units && cap(units.math, value.number as number, value.crop as Crop | ""),
    step: 1,
    group: "Solid",
    when: grows,
  },
  {
    key: "draw",
    label: "Draw",
    kind: "segment",
    def: "shell",
    options: [
      ["shell", "Shell"],
      ["cubes", "Cubes"],
    ],
    group: "Solid",
    when: grows,
  },
  { key: "opacity", label: "Opacity", kind: "slider", def: 1, min: 0.05, max: 1, step: 0.05, group: "Solid" },
  { key: "spin", label: "Spin", kind: "toggle", def: 1, group: "Solid" },
  {
    key: "camera",
    label: "Camera",
    kind: "segment",
    def: "eye",
    options: [
      ["eye", "Perspective"],
      ["iso", "Isometric"],
    ],
    group: "Solid",
  },
  {
    key: "radius",
    label: "Radius",
    kind: "slider",
    def: 60,
    min: 0,
    max: REACH,
    step: 1,
    unit: `/${DEN}`,
    group: "Crop",
    when: crop,
  },
  { key: "anti", label: "Anti", kind: "toggle", def: 0, group: "Crop", when: crop },
  {
    key: "policy",
    label: "Keep",
    kind: "segment",
    def: "touching",
    options: [
      ["inside", "Inside"],
      ["touching", "Touching"],
    ],
    group: "Crop",
    when: crop,
  },
  {
    key: "edge",
    label: "Exact edge",
    kind: "toggle",
    def: 0,
    group: "Crop",
    when: (value) => crop(value) && grows(value) && value.crop !== "ball" && Number(value.anti) !== 1,
  },
  {
    key: "limit",
    label: "Scales up to",
    kind: "slider",
    def: 7,
    min: 1,
    max: LAYERS,
    step: 2,
    group: "Field",
    when: fills,
  },
  { key: "combine", label: "Combine", kind: "segment", def: "sum", options: COMBINES, group: "Field", when: fills },
  { key: "depth", label: "Level", kind: "slider", def: 1, min: 1, max: 2, step: 1, group: "Field", when: fills },
  {
    key: "size",
    label: "Size",
    kind: "segment",
    def: 64,
    options: SIZES.map((n) => [n, String(n)]),
    group: "Field",
    when: fills,
  },
  { key: "shell", label: "Shell", kind: "toggle", def: 1, group: "Field", when: fills },
  {
    key: "threshold",
    label: "Threshold",
    kind: "slider",
    def: LAYERS,
    min: 0,
    max: (value, units) => (units && fills(value) ? most(units.math, value as Coded & Stacking) : LAYERS),
    step: 1,
    group: "Field",
    when: (value) => fills(value) && Number(value.shell) === 1,
  },
  ...PLANES.flatMap(([key, label]): Row<Had<"math">>[] => [
    { key: `c${key}`, label, kind: "toggle", def: key === "d" ? 1 : 0, group: "Planes", when: fills },
    {
      key: `p${key}`,
      label: "At",
      kind: "slider",
      def: 0.5,
      min: 0,
      max: 1,
      step: 0.01,
      group: "Planes",
      when: (value) => fills(value) && Number(value[`c${key}`]) === 1,
    },
  ]),
  { key: "ramp", label: "Ramp", kind: "segment", def: "fire", options: RAMPS, group: "Colour", when: fills },
  { key: "levels", label: "Levels", kind: "slider", def: 16, min: 2, max: 64, step: 1, group: "Colour", when: fills },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Colour", when: fills },
  { key: "circle", label: "Count", kind: "toggle", def: 0, group: "Circle", when: grows },
  {
    key: "centre",
    label: "Centre",
    kind: "segment",
    def: "corner",
    options: [
      ["corner", "Corner"],
      ["centre", "Centre"],
    ],
    group: "Circle",
    when: counting,
  },
  {
    key: "count",
    label: "Count level",
    kind: "slider",
    def: 4,
    min: 1,
    max: (value, units) => units && counted(units.math, value.number as number),
    step: 1,
    group: "Circle",
    when: counting,
  },
  {
    key: "r",
    label: "Radius r",
    kind: "slider",
    def: 27,
    min: 1,
    max: (value, units) =>
      units && Math.max(1, top(units.math, value.number as number, value.count as number, value.centre as Centre)),
    step: 1,
    group: "Circle",
    when: counting,
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const units = wasm("math");

export const name = (value: Value) =>
  `three-${String(value.code ?? "").trim() || CARPET}-${value.source === "field" ? "field" : `l${value.level}`}${value.crop ? `-${value.crop}` : ""}`;

export const PAGE: Page<Units<"math">, Held> = {
  defaults: { base: 2, code: "23" },
  design: { dim: 3, base: "base", code: "code" },
  spec: SPEC,
  units,
  gestures: true,
  record: true,
  about: [
    "A code picks the filled corners of a cube, and the cube grows into itself level by level; code 23 grows the Menger sponge.",
    "Drag to orbit, scroll or pinch to zoom, right-drag or shift-drag to pan. The counts come from closed formulas and the mesh from the exposed faces, so nothing is guessed.",
    "A named shape keeps the cells it reaches, inside or touching; anti keeps the outside, and the exact edge clips the whole mesh with the shape walls. Left and Right move the radius; the plane crop lives in [Crop](/crop/).",
    "The field stacks the design at scales 1, 3, 5 and on into one solid: a threshold draws its shell and a plane shows the moire it cuts. More in [the core](/research/core/), [the crop note](/research/crop/) and [the cuts note](/research/cuts/).",
  ],
  name,
  keys: [
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", when: grows },
    { key: ["ArrowLeft", "ArrowRight"], label: "Radius", act: "radius", when: crop },
    { key: "1", label: "Corner", act: "corner", menu: "View" },
    { key: "2", label: "Edge", act: "edge", menu: "View" },
    { key: "3", label: "Face", act: "face", menu: "View" },
  ],
  actions: {
    level: (scene, e) => scene.deeper?.(e.key === "ArrowDown" ? -1 : 1),
    radius: (scene, e) => scene.widen?.(e.key === "ArrowLeft" ? -1 : 1),
    corner: (scene) => scene.look?.("corner"),
    edge: (scene) => scene.look?.("edge"),
    face: (scene) => scene.look?.("face"),
  },
};

const ORANGE = rgb(palette.orange);

/* STUDY */

const title = (made: Made, math: Mrly) => tag(named(math, made.code, made.base), made.code, made.base);

function solid(math: Mrly, value: Solid): Omit<Study, "planes"> {
  const made = design(math, value);
  const level = Math.min(value.level, cap(math, value.number, value.crop));
  const { code, base } = made;
  const { number } = value;
  const cell = grown(math, code, base, number, level);
  const cut = value.crop ? cropped(math, cell, value) : null;
  const exact = Boolean(cut) && Number(value.edge) === 1 && value.crop !== "ball" && Number(value.anti) !== 1;
  const shown = cut && !exact ? cut.cell : cell;
  const cubes = value.draw === "cubes";
  const heavy = load(math, shown, cubes);
  const error =
    heavy > (cubes ? CUBES : FACES)
      ? `${heavy} ${cubes ? "cubes" : "faces"} is more than this page draws; lower the level.`
      : "";
  const census = tally(math, cell);
  const rows: Readout[] = [
    ["Name", title(made, math)],
    ["Side", cell.shape[0]],
    ["Filled", String(math.counts.fill(code, number, 3, level, base))],
    ["Empty", String(math.counts.void_(code, number, 3, level, base))],
    ["Exposed", String(math.counts.surface(code, number, level, base))],
    ["Fill ratio", fixed(math.counts.ratio(code, number, 3, level, base), 4)],
    ["Dimension", fixed(math.counts.dimension(code, number, 3, base), 4)],
    ...(census
      ? ([
          ["Vertices", census.vertices],
          ["Edges", census.edges],
          ["Faces", census.faces],
          ["Euler", String(census.euler)],
        ] satisfies Readout[])
      : []),
    ...(cut
      ? ([
          ["Filled in", cut.in],
          ["Filled cut", cut.cut],
          ["Filled out", cut.out],
          ["Exposed after", cut.after],
        ] satisfies Readout[])
      : []),
  ];
  if (Number(value.circle) === 1) {
    const count = circle(math, made, value);
    const read = reading(count, value.r);
    rows.unshift(["N(r)", read.seen], ["Inside(r)", read.inside], ["C(r)", read.cut], ["Count side", count.side]);
    rows.push(["r", read.at], ["d", fixed(count.d, 7)], ["Fill of one tile", count.mass]);
  }
  const body = error
    ? null
    : {
        key: `${code} ${base} ${number} ${level} ${value.draw} ${exact ? "whole" : value.crop} ${value.radius} ${value.anti} ${value.policy}`,
        cell: shown,
        cubes,
        quads: cubes ? null : faces(math, shown),
      };
  const clip = exact ? walls(region(math, value.crop, value.radius, false)) : null;
  return { body, clip, rows: error ? [["Error", error], ...rows] : rows, error, obj: () => math.three.to_obj(shown) };
}

function volume(math: Mrly, value: Solid, paint: Paint): Study {
  const made = design(math, value);
  const held = field(math, made, value);
  const threshold = Math.min(value.threshold, held.max);
  const on = Number(value.shell) === 1;
  const kept = on ? shell(math, held, { ...value, threshold }) : null;
  const error =
    kept && kept.faces > FACES
      ? `${kept.faces} faces is more than this page draws; raise the threshold or lower the size.`
      : "";
  const cuts = PLANES.filter(([key]) => Number(value[`c${key}`]) === 1).map(([key]) =>
    plane(math, held, key, value[`p${key}`], paint),
  );
  const rows: Readout[] = [
    ["Name", title(made, math)],
    ["Layers", held.layers],
    ["Voxels", held.voxels],
    ["At or above", held.volume.count(threshold)],
    ...(kept ? ([["Faces", kept.faces]] satisfies Readout[]) : []),
    ["Field", `${held.min} to ${held.max}`],
  ];
  return {
    body:
      kept && !error
        ? {
            key: `field ${made.code} ${made.base} ${value.limit} ${value.combine} ${value.depth} ${value.size} ${threshold} ${value.crop} ${value.radius} ${value.anti} ${value.policy}`,
            cell: kept.cell,
            cubes: false,
            quads: faces(math, kept.cell),
          }
        : null,
    clip: null,
    planes: cuts,
    rows: error ? [["Error", error], ...rows] : rows,
    error,
    obj: () => (kept ? math.three.to_obj(kept.cell) : ""),
  };
}

export function study(math: Mrly, value: Solid, paint: Paint): Study {
  return value.source === "field" ? volume(math, value, paint) : { ...solid(math, value), planes: [] };
}

function night(): boolean {
  const set = typeof document === "undefined" ? "" : document.documentElement.dataset.theme;
  if (set === "dark" || set === "light") return set === "dark";
  return typeof matchMedia === "function" && matchMedia("(prefers-color-scheme: dark)").matches;
}

const ink = (accent: string) => inks({ ...(night() ? dark : light), accent } as Palette);

export function painter(view: Pick<Viewer, "look">, value: Solid): Paint {
  return {
    colorizer: colorizer(ink(view.look().accent), value.ramp, value.levels),
    levels: value.levels,
    invert: value.invert,
  };
}

/* SCENE */

const settle = (opts: Given & { math: Mrly }): Solid => ({
  ...(tidy(SPEC, opts, { math: opts.math }) as Knobs),
  seed: opts.seed,
  base: opts.base,
  code: opts.code,
});

const same = (a: Plane[] = [], b: Plane[] = []) => a.length === b.length && a.every((one, i) => one === b[i]);

export function make(canvas: HTMLCanvasElement, view: Viewer, opts: Given = {}) {
  const { math, three, tell } = opts;
  if (!math) throw new Error("three: the scene wants @mrlyprod/mrly/math as opts.math");
  let value = settle({ ...opts, math });
  let plan: Study | null = null;
  let stage: Stage | null = null;
  if (three) {
    try {
      stage = space(three, canvas, view);
    } catch {
      stage = null;
    }
  }
  const tone = (next: Solid) => (next.draw === "cubes" && next.source === "design" ? ORANGE : rgb(view.look().accent));
  const apply = (next: Solid, old: Solid | null) => {
    const was = plan;
    plan = study(math, next, painter(view, next));
    tell?.({ rows: plan.rows, error: plan.error });
    if (!stage) return;
    if (!was || plan.body?.key !== was.body?.key) stage.show(plan.body);
    stage.paint({ color: tone(next), opacity: next.opacity, clip: plan.clip });
    if (!was || !same(plan.planes, was.planes)) stage.planes(plan.planes);
    stage.project(next.camera);
    if (!old || Number(next.spin) !== Number(old.spin)) stage.spinning(Number(next.spin) === 1);
    stage.mark();
  };
  apply(value, null);
  if (stage && value.camera === "iso") stage.look("corner");
  let pending: [Solid, Solid] | null = null;
  const flush = () => {
    if (!pending) return;
    const [next, old] = pending;
    pending = null;
    apply(next, old);
  };
  const blank = () => {};
  return {
    draw: () => {
      flush();
      stage?.draw();
    },
    size: stage ? stage.size : blank,
    theme: () => {
      pending = [value, pending?.[1] ?? value];
    },
    set: (next: Given) => {
      const old = pending?.[1] ?? value;
      value = settle({ ...next, math });
      pending = [value, old];
      return true;
    },
    turn: stage?.turn,
    fling: stage?.fling,
    zoom: stage?.zoom,
    pan: stage?.pan,
    look: stage?.look,
    obj: () => {
      flush();
      return plan!.obj();
    },
    stop: () => stage?.stop(),
  };
}
