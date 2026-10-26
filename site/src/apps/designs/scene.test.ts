import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { TypedArray } from "three";
import type { Three } from "../../lib/app/hooks.ts";
import type { Scene, View } from "../../lib/app/run.ts";
import type { Raw, Value } from "../../lib/app/spec.ts";
import { defaults, describe as groups } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Designs, Info } from "./scene.ts";
import { make, named, PAGE, SPEC } from "./scene.ts";

type Line = [string | symbol, ...unknown[]];

type Live = { log: Line[]; view: View; scene: Designs & Scene; told: (Info | "wake")[] };

const pen = (log: Line[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const brief = (arg: unknown) =>
  ArrayBuffer.isView(arg)
    ? (arg as TypedArray).length
    : typeof arg === "function"
      ? "thing"
      : typeof arg === "object" && arg !== null
        ? Object.keys(arg).join(",")
        : arg;

function target() {}

const fake = (log: Line[]): Three => {
  const any = (path: string): Three =>
    new Proxy(Object.assign(target, {} as Three), {
      get: (_, key) => (typeof key === "symbol" ? undefined : any(`${path}.${key}`)),
      set: (_, key: string, value: unknown) => {
        log.push([`${path}.${key}=`, brief(value)]);
        return true;
      },
      apply: (_, __, args: unknown[]) => {
        log.push([path, ...args.map(brief)]);
        return any(path);
      },
      construct: (_, args: unknown[]) => {
        log.push([`new ${path}`, ...args.map(brief)]);
        return any(path);
      },
    });
  return any("three");
};

const open = (seed: number, value: Raw, three?: boolean) => {
  const log: Line[] = [];
  const told: (Info | "wake")[] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: true,
    w: 800,
    h: 600,
    dpr: 2,
    t: 0,
  } as View;
  view.wake = () => {
    view.still = false;
    told.push("wake");
  };
  const scene: Designs & Scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    ...value,
    math,
    three: three ? fake(log) : undefined,
    tell: (info) => told.push(info),
  });
  return {
    log,
    view,
    scene,
    told,
    count: (name: string) => log.filter(([key]) => key === name).length,
    names: () => log.map(([key]) => key),
  };
};

const eye = (live: Live) => live.log.find(([key]) => key === "three.OrthographicCamera.position.set");

test("the flat stage draws one rect per run of the grown design, once, then rests until a theme or a size", () => {
  const live = open(1, { dim: 2, base: 2, code: "7", level: 2 });
  live.scene.draw();
  expect([live.count("rect"), live.count("fill"), live.scene.still, live.scene.every]).toEqual([
    20,
    1,
    true,
    undefined,
  ]);
  live.log.length = 0;
  live.view.t = 500;
  live.scene.draw();
  expect(live.log).toEqual([]);
  live.scene.theme();
  expect(live.count("rect")).toBe(20);
  live.log.length = 0;
  live.scene.size!();
  expect(live.count("rect")).toBe(20);
});

test("the solid stage builds one mesh of the exposed faces, rests at the same angle at any seed or t, turns by a drag, and disposes on stop", () => {
  const live = open(3, { dim: 3, base: 2, code: "23", level: 1 }, true);
  const faces = Number(math.three.census(math.three.create(23, 3, 1, 2)).surface);
  expect(live.count("new three.WebGLRenderer")).toBe(1);
  expect(live.log.filter(([key]) => key === "new three.BufferAttribute")).toEqual([
    ["new three.BufferAttribute", faces * 12, 3],
    ["new three.BufferAttribute", faces * 12, 3],
    ["new three.BufferAttribute", faces * 6, 1],
  ]);
  live.log.length = 0;
  live.scene.draw();
  const first = eye(live);
  expect(live.names().at(-1)).toBe("three.WebGLRenderer.render");
  live.log.length = 0;
  live.view.t = 5000;
  live.scene.draw();
  expect(live.log).toEqual([]);
  const other = open(4, { dim: 3, base: 2, code: "23", level: 1 }, true);
  other.scene.draw();
  expect(eye(other)).toEqual(first);
  live.scene.turn!(0.25, 0);
  expect([live.names().at(-1), eye(live)]).toEqual([
    "three.WebGLRenderer.render",
    expect.not.arrayContaining(first!.slice(1)),
  ]);
  live.log.length = 0;
  live.scene.stop!();
  expect(live.names()).toEqual([
    "three.BufferGeometry.dispose",
    "three.MeshLambertMaterial.dispose",
    "three.WebGLRenderer.dispose",
    "three.WebGLRenderer.forceContextLoss",
  ]);
});

test("spin wakes the still scene and turns the resting cube with t; without it the cube never wakes", () => {
  const rest = open(1, { dim: 3, base: 2, code: "23", level: 1 }, true);
  rest.scene.draw();
  expect(rest.told).not.toContain("wake");
  const live = open(1, { dim: 3, base: 2, code: "23", level: 1, spin: 1 }, true);
  live.scene.draw();
  expect(live.told).toContain("wake");
  const first = eye(live);
  live.log.length = 0;
  live.view.t = 1000;
  live.scene.draw();
  expect(eye(live)).not.toEqual(first);
});

test("a set regrows the design in the same stage: the plane swaps its cell, the cube its geometry and keeps the renderer and its pose", () => {
  const flat = open(1, { dim: 2, base: 2, code: "7", level: 2 });
  flat.scene.set({ ...defaults(SPEC), dim: 2, base: 2, code: "15", level: 2 });
  expect([flat.count("rect"), (flat.told.at(-1) as Info).code]).toEqual([9, "15"]);
  const live = open(1, { dim: 3, base: 2, code: "23", level: 1 }, true);
  live.scene.turn!(0.1, 0.05);
  const pose = live.scene.pose!();
  live.log.length = 0;
  live.scene.set({ ...defaults(SPEC), dim: 3, base: 2, code: "1", level: 2 });
  expect([
    live.count("new three.WebGLRenderer"),
    live.count("three.BufferGeometry.dispose"),
    live.count("new three.BufferGeometry"),
    live.scene.pose!(),
    (live.told.at(-1) as Info).code,
  ]).toEqual([0, 1, 1, pose, "1"]);
  live.log.length = 0;
  live.scene.set({ ...defaults(SPEC), dim: 3, base: 2, code: "1", level: 2, tint: "pink" });
  expect([live.count("new three.BufferGeometry"), live.count("three.WebGLRenderer.render")]).toEqual([0, 1]);
});

test("a set to the other dim asks the host for a fresh stage", () => {
  expect(
    open(1, { dim: 2, base: 2, code: "7", level: 2 }).scene.set({
      ...defaults(SPEC),
      dim: 3,
      base: 2,
      code: "23",
      level: 2,
    }),
  ).toBe(false);
});

test("an export is named by the dim, a base off 2, the code and the level", () => {
  expect(PAGE.name).toBe(named);
  expect([
    named({ dim: 2, base: 2, code: "7", level: 3 }),
    named({ dim: 2, base: 3, code: " 495", level: 2 }),
    named({ dim: 3, base: 2, code: "", level: 1 }),
  ]).toEqual(["designs-2d-7-level3", "designs-2d-base3-495-level2", "designs-3d-level1"]);
});

test("a blank code opens on the carpet of its space, and the counts tell the design, its class and its fill", () => {
  const told = (value: Raw) => open(1, value).told.at(-1) as Info;
  expect([
    told({ dim: 2, base: 2, code: "" }).code,
    told({ dim: 3, base: 2, code: "" }).code,
    told({ dim: 2, base: 3, code: "x" }).code,
    told({ dim: 2, base: 2, code: "23" }).code,
  ]).toEqual(["7", "23", "495", "7"]);
  const cube = told({ dim: 3, base: 2, code: "23", level: 5 });
  expect(cube).toMatchObject({
    dim: 3,
    base: 2,
    code: "23",
    level: 3,
    side: 27,
    name: "carpet",
    rep: "23",
    orbit: 8,
    fills: 8000,
    voids: 11683,
    surface: 18048,
    grid: 19683,
    upto: 4,
    terms: ["3", "6", "22", "402"],
  });
  expect(cube.dimension).toBeCloseTo(Math.log(20) / Math.log(3), 6);
});

test("the colour follows the degree of the normal form unless a hue is picked", () => {
  const ink = (value: Raw) => {
    const one = open(1, { dim: 2, base: 2, level: 1, ...value });
    one.scene.draw();
    return one.log.find(([key]) => key === "fillStyle")![1];
  };
  expect([ink({ code: "7" }), ink({ code: "3" }), ink({ code: "7", tint: "pink" })]).toEqual([
    palette.yellow,
    palette.orange,
    palette.pink,
  ]);
});

test("the still exports as rects in the plane and as lit iso faces in the cube", () => {
  const flat = open(1, { dim: 2, base: 2, code: "7", level: 1 }).scene.svg()!;
  expect([flat.startsWith("<svg"), flat.match(/<rect /g)!.length, flat.includes('fill="#000000"')]).toEqual([
    true,
    5,
    true,
  ]);
  const live = open(1, { dim: 3, base: 2, code: "23", level: 1 }, true);
  const iso = live.scene.svg()!;
  expect([iso.startsWith("<svg"), (iso.match(/<path /g) ?? []).length > 0, live.scene.obj!().startsWith("v ")]).toEqual(
    [true, true, true],
  );
});

test("a cube without three in hand is a blank stage that still tells its counts", () => {
  const { scene, log, told } = open(1, { dim: 3, base: 2, code: "23", level: 2 });
  scene.draw();
  expect([log, scene.svg(), (told.at(-1) as Info).fills]).toEqual([[], undefined, 400]);
});

test("the design keys stay out of the knobs, which hold the level and the look", () => {
  const shown = (value: Value) =>
    groups(SPEC, { ...defaults(SPEC), ...value }).map(({ name, rows }) => [name, rows.map((row) => row.key)]);
  expect([shown({ dim: 2 }), shown({ dim: 3 })]).toEqual([
    [
      ["Design", ["level"]],
      ["Look", ["tint"]],
    ],
    [
      ["Design", ["level"]],
      ["Look", ["tint", "spin"]],
    ],
  ]);
});
