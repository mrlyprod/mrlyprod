import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { Handle } from "../../lib/app/page.tsx";
import type { View } from "../../lib/app/run.ts";
import type { Value } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { rng } from "../../lib/rand.ts";
import { OUT } from "../../lib/spin/engine.ts";
import { PAD, SOURCE } from "../../lib/spin/scene.ts";
import type { Facts, Rosette } from "./scene.ts";
import { make, PAGE, SPEC, units } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

type Image = { data: Uint8ClampedArray; width: number };

type Options = { unit?: Mrly | null; w?: number; h?: number };

const pen = (log: Call[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args.map((arg) => (arg && typeof arg === "object" ? "layer" : arg))]),
    set: (_, key, v: unknown) => {
      log.push([key, v]);
      return true;
    },
  });

const images: Image[] = [];

Object.assign(globalThis, {
  OffscreenCanvas: class {
    width: number;
    height: number;
    constructor(w: number, h: number) {
      this.width = w;
      this.height = h;
    }
    getContext() {
      return pen([]);
    }
  },
});

Object.assign(globalThis, {
  ImageData: class {
    data: Uint8ClampedArray;
    width: number;
    constructor(data: Uint8ClampedArray, w: number) {
      this.data = data;
      this.width = w;
      images.push(this);
    }
  },
});

const open = (value: Value, { unit = math, w = 1600, h = 900 }: Options = {}) => {
  const log: Call[] = [];
  const facts: Facts[] = [];
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: false,
    w,
    h,
    dpr: 2,
    t: 0,
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view as View, {
    ...defaults(SOURCE),
    ...defaults(SPEC),
    seed: 1,
    level: 2,
    math: unit,
    onFacts: (one) => facts.push(one),
    ...value,
  });
  const draw = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { scene, facts, draw, view };
};

const draws = (log: Call[]) => log.filter(([name]) => name === "drawImage");

const reach = (w: number, h: number) => Math.min(w, h) / 2 - PAD * 2;

test("the stack alone is the stage: the rosette of the copies in one disc at any aspect, no text, no bars", () => {
  const live = open({});
  const log = live.draw();
  expect(images.at(-1)!.width).toBe(OUT);
  expect(draws(log)).toEqual([
    ["drawImage", "layer", 800 - reach(1600, 900), 450 - reach(1600, 900), 2 * reach(1600, 900), 2 * reach(1600, 900)],
  ]);
  expect(log.some(([name]) => name === "fillText" || name === "fillRect")).toBe(false);
  expect(draws(open({}, { w: 780, h: 1688 }).draw())).toEqual([
    ["drawImage", "layer", 390 - reach(780, 1688), 844 - reach(780, 1688), 2 * reach(780, 1688), 2 * reach(780, 1688)],
  ]);
  expect(live.facts.at(-1)).toMatchObject({ name: "carpet", copies: 6, step: 60, order: 4, petals: 12 });
});

test("the stack is a still: a stack change repaints the same canvas from a new rosette, a size lays it again from the same one", () => {
  const live = open({});
  live.draw();
  expect(live.scene.still).toBe(true);
  const before = images.length;
  live.scene.set!({ copies: 4, full: 0, step: 45 });
  expect(draws(live.draw())).toHaveLength(1);
  expect(images.length).toBe(before + 1);
  expect(live.facts.at(-1)).toMatchObject({ copies: 4, step: 45, petals: "partial orbit" });
  live.view.w = 900;
  live.scene.size!();
  expect(draws(live.draw())).toEqual([
    ["drawImage", "layer", 450 - reach(900, 900), 450 - reach(900, 900), 2 * reach(900, 900), 2 * reach(900, 900)],
  ]);
  expect(images.length).toBe(before + 1);
});

test("an export is named by the source, its design and the copies", () => {
  expect([
    PAGE.name({ ...defaults(SOURCE), copies: 6 }, { math }),
    PAGE.name({ ...defaults(SOURCE), copies: 6 }),
  ]).toEqual(["radial-flat-495-6", "radial-flat-carpet-6"]);
});

test("the page takes a design of either dimension, the blend lives in the library, and the empty code opens the carpet", () => {
  const { when, ...slot } = PAGE.design;
  expect([slot, PAGE.units]).toEqual([{ dim: "dim", base: "base", code: "code" }, units]);
  expect([when({ source: "flat" }), when({ source: "moire" })]).toEqual([true, false]);
  expect(SPEC.find((row) => row.key === "blend")!.when!({})).toBe(false);
  expect(open({}).facts.at(-1)).toMatchObject({ name: "carpet", order: 4 });
});

test("Enter snaps back to a full turn only from a partial orbit; the arrows step the copies and the step", () => {
  const enter = (full: number) => PAGE.keys.filter((row) => row.key === "Enter" && row.when!({ full })).length;
  expect([enter(1), enter(0)]).toEqual([0, 1]);
  expect(PAGE.keys.filter((row) => row.menu).map((row) => row.act)).toEqual(["whole", "copies"]);
  const got: (string | number)[] = [];
  const hands = Object.assign({} as Handle<Rosette>, {
    whole: () => got.push("whole"),
    copies: (by: number) => got.push(by),
    step: (by: number) => got.push(by),
  });
  PAGE.actions.copies(hands, { key: "ArrowLeft" });
  PAGE.actions.step(hands, { key: "ArrowUp" });
  PAGE.actions.whole(hands);
  expect(got).toEqual([-1, 0.5, "whole"]);
});

test("without the unit the scene clears and throws nothing", () => {
  expect(open({}, { unit: null }).draw()).toEqual([["clearRect", 0, 0, 1600, 900]]);
});
