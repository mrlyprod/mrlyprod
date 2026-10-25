import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Spec } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { look as aim } from "../../lib/gl/camera.ts";
import { blank, context } from "../../lib/gl/gl2.ts";
import type { Post } from "../../lib/gl/post.ts";
import { post } from "../../lib/gl/post.ts";
import { add } from "../../lib/gl/vec.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import type { State } from "../../lib/space/bang.ts";
import { canon, NAMES, name, route, SPEED, teach } from "../../lib/space/bang.ts";
import type { Hyper } from "../../lib/space/hyper.ts";
import { hyper } from "../../lib/space/hyper.ts";
import type { March, Tone } from "../../lib/space/march.ts";
import { GRADES, march, tone } from "../../lib/space/march.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Hud } from "./hud.ts";
import { hud } from "./hud.ts";
import type { Rail, Trails } from "./trails.ts";
import { trails } from "./trails.ts";

/* TYPES */

export type Knobs = {
  code: number;
  n: number;
  pick: string;
  speed: number;
  path: number;
  pilot: string;
  look: string;
  palette: string;
  quality: string;
  hyper: number;
};

export type Given = Opts & { math?: Mrly | null };

export type Owned = { drop: () => void };

export type Zoom = ReturnType<typeof make>;

export type Zoomed = Zoom & { step?: (by: number) => void };

const HIDE = () => false;
const FAMILY = NAMES.map(([code]) => code);
const GRIDS = [3, 5];
const SLOW = 0.5;

export const SPEC: Spec = [
  { key: "code", label: "Code", kind: "slider", def: 23, min: 0, max: 255, step: 1, group: "Shape", when: HIDE },
  {
    key: "n",
    label: "N",
    kind: "segment",
    def: 3,
    options: [
      [3, "3"],
      [5, "5"],
    ],
    group: "Shape",
  },
  {
    key: "pick",
    label: "Pick",
    kind: "segment",
    def: "all",
    options: [
      ["all", "All"],
      ["flyable", "Flyable"],
      ["named", "Named"],
      ["family", "Family"],
    ],
    group: "Shape",
    when: HIDE,
  },
  { key: "speed", label: "Speed", kind: "slider", def: 1, min: 0.25, max: 2, step: 0.25, group: "Flight" },
  { key: "path", label: "Path", kind: "toggle", def: 1, group: "Flight" },
  {
    key: "pilot",
    label: "Pilot",
    kind: "segment",
    def: "steady",
    options: [
      ["steady", "Steady"],
      ["fighter", "Fighter"],
      ["coaster", "Coaster"],
    ],
    group: "Flight",
  },
  {
    key: "look",
    label: "Look",
    kind: "segment",
    def: "studio",
    options: [
      ["graphic", "Graphic"],
      ["studio", "Studio"],
      ["haze", "Haze"],
    ],
    group: "Look",
  },
  {
    key: "palette",
    label: "Palette",
    kind: "pick",
    def: "",
    options: [
      ["", "Seeded"],
      ["accent", "Accent"],
      ["fire", "Fire"],
      ["ice", "Ice"],
      ["mono", "Mono"],
    ],
    group: "Look",
  },
  {
    key: "quality",
    label: "Quality",
    kind: "segment",
    def: "",
    options: [
      ["", "Auto"],
      ["low", "Low"],
      ["high", "High"],
    ],
    group: "Look",
  },
  { key: "hyper", label: "Hyper", kind: "toggle", def: 0, group: "Look" },
];

export const units = wasm("math");

export const PAGE = {
  spec: SPEC,
  units,
  design: { dim: 3, base: 2, code: "code" },
  keys: [
    { key: "ArrowRight", label: "Next shape", act: "next", menu: "View" },
    { key: "ArrowLeft", label: "Previous shape", act: "prev", menu: "View" },
  ],
  actions: { next: (scene) => scene.step?.(1), prev: (scene) => scene.step?.(-1) },
  record: true,
} satisfies Page<Loaded<"math">, Zoomed>;

const flight = (look: string) => (rand: Rng) => ({
  code: pick(rand, FAMILY),
  n: pick(rand, GRIDS),
  speed: SLOW,
  path: 0,
  pilot: "steady",
  look,
  palette: "accent",
  quality: "low",
});

export const SAVERS = [
  { id: "studio", title: "Studio", pick: flight("studio") },
  { id: "graphic", title: "Graphic", pick: flight("graphic") },
  { id: "haze", title: "Haze", pick: flight("haze") },
];

export function step(pool: readonly number[], code: number, by: number) {
  const at = pool.indexOf(code);
  if (at >= 0) return pool[(at + by + pool.length) % pool.length];
  return (
    (by > 0 ? (pool.find((one) => one > code) ?? pool[0]) : (pool.findLast((one) => one < code) ?? pool.at(-1))) ?? code
  );
}

const FOV = 1.25;
const OPEN = -4000;
const SKY = { density: 0.8, dust: 0.4 };
const SPAN = 8;
const DROP = 0.2;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const { math } = opts;
  if (!math) throw new Error("zoom: the scene wants @mrlyprod/mrly/math as opts.math");
  const gl = context(canvas);
  if (!gl) return blank(canvas);
  const value = tidy(SPEC, opts) as Knobs;
  const seed = Number.isFinite(Number(opts.seed)) ? Number(opts.seed) >>> 0 : 0;
  const { n, code } = value;
  const design = { code, n };
  const info = teach(math, code, n);
  const path = info.kept ? route(math, design, seed, { pilot: value.pilot }) : null;
  const made: Owned[] = [];
  let film: Post | null = null;
  let trip: Hyper | null = null;
  let world: March | null = null;
  let lines: Trails | null = null;
  let face: Hud;
  try {
    face = hud(gl, view);
    made.push(face);
    if (path) {
      world = march(gl, view, design, { math, look: value.look, quality: value.quality, toFrame: path.toFrame });
      made.push(world);
      if (value.path) {
        lines = trails(gl, view);
        made.push(lines);
      }
      if (value.hyper) {
        trip = hyper(gl, view, { seed, sky: SKY, tint: "", look: "cloud", idle: 0, mode: null, floor: "cloud" });
        made.push({ drop: trip.stop });
      } else {
        film = post(gl, view);
        made.push(film);
      }
    }
  } catch (error) {
    for (const one of made) one.drop();
    console.error(error);
    return blank(canvas);
  }
  if (trip) {
    const now = view.t;
    view.t = OPEN;
    trip.trigger();
    view.t = now;
    trip.hold();
  }
  const grade = GRADES[value.look];
  const title = `${NAMES.some(([one]) => canon(math, one) === canon(math, code)) ? name(math, code) : "bang"} ${code}`;
  let tint: string | null = null;
  let paints: Tone | null = null;
  let rail: Required<Rail>[] = [];
  let said: string[] = [];
  const words = (state: State) => [
    title,
    `n ${n}  level ${state.level}`,
    `bits ${info.bits}  fill ${info.kept}/${n ** 3}`,
    `dim ${info.dimension.toFixed(2)}  class ${info.class}`,
    state.mode,
  ];
  const empty = () => {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, view.w, view.h);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    said = ["empty"];
    face.lines(said, "tl");
    face.draw();
  };
  const draw = () => {
    if (!path) return empty();
    const accent = view.look().accent;
    if (accent !== tint) {
      tint = accent;
      paints = tone(value.palette, seed, accent);
    }
    const tau = (view.t / 1000) * value.speed;
    const state = path.at(tau);
    const cam = aim(state.pos, add(state.pos, state.fwd), state.up, state.roll);
    cam.fov = FOV;
    const points = lines ? path.path(tau, SPAN, DROP) : [];
    rail = points.length ? [{ points, born: 0, step: (SPEED * SPAN) / (points.length - 1) }] : [];
    world!.draw(cam, state, paints!);
    const over = () => {
      world!.layer(1);
      lines?.draw(rail, cam, 1, world!.depth());
    };
    if (trip) {
      const ahead = aim([0, 0, 0], [0, 0, 1], [0, 1, 0], state.roll);
      ahead.fov = FOV;
      trip.draw(ahead, null, over, world!.scale(), grade);
    } else {
      film!.begin(world!.scale());
      over();
      film!.end(grade);
    }
    said = words(state);
    if (view.lock) return;
    face.lines(said, "tl");
    face.draw();
  };
  const stop = () => {
    for (const one of made) one.drop();
    gl.getExtension("WEBGL_lose_context")?.loseContext();
  };
  return { draw, stop, code, said: () => said, trails: () => rail };
}
