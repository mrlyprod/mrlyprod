import { expect, test } from "bun:test";
import * as num from "@mrlyprod/num";
import { rgb } from "../../net/sdk/color.ts";
import type { Handle } from "../../net/sdk/page.tsx";
import { keymap, labels } from "../../net/sdk/page.tsx";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, describe } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Stroke } from "../../net/test/pen.ts";
import { pen, said } from "../../net/test/pen.ts";
import type { Facts } from "./engine.ts";
import { lay, PAD, stamp } from "./engine.ts";
import type { Handled } from "./scene.ts";
import { file, make, PAGE, SPEC, tidied } from "./scene.ts";

type Image = { width: number; height: number; data: Uint8ClampedArray };

type Sheet = { width: number; height: number; image?: Image };

const sheets: Sheet[] = [];

Object.assign(globalThis, {
  OffscreenCanvas: class {
    width: number;
    height: number;
    image?: Image;
    constructor(w: number, h: number) {
      this.width = w;
      this.height = h;
      sheets.push(this);
    }
    getContext() {
      return {
        createImageData: (w: number, h: number) => ({ width: w, height: h, data: new Uint8ClampedArray(w * h * 4) }),
        putImageData: (image: Image) => (this.image = image),
      };
    }
  },
});

const BASE = { ...defaults(SPEC), seed: 1 };

const open = (value: Raw, w = 800, h = 600) => {
  const log: Stroke[] = [];
  const jobs: (() => void)[] = [];
  const told: Facts[] = [];
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#6768fa", ink: "#ffffff" }),
    still: true,
    w,
    h,
    dpr: 1,
    t: 0,
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...BASE,
    ...value,
    num,
    onFacts: (facts) => told.push(facts),
    later: (job) => jobs.push(job),
  });
  const draw = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { view, scene, log, draw, jobs, told };
};

const drawn = (log: Stroke[]) => log.filter(([name]) => name === "drawImage").map((call) => call.slice(2));

const count = (log: Stroke[], name: string) => log.filter(([key]) => key === name).length;

const strokes = (log: Stroke[]) => log.filter(([name]) => name === "stroke").length;

test("the plaid opens as one raster of 64 by 64 cells sampled nearest over a centred box, and a draw after is a no-op", () => {
  const one = open({});
  const log = one.draw();
  const box = lay(800, 600, 64, 64, PAD);
  expect(drawn(log)).toEqual([[0, 0, 64, 64, box.x, box.y, box.w, box.h]]);
  expect(log).toContainEqual(["imageSmoothingEnabled", false]);
  expect(count(log, "fillText")).toBe(0);
  expect(one.draw()).toEqual([]);
  one.scene.size();
  expect(drawn(one.draw())).toHaveLength(1);
});

test("a stamp paints a plus sign orange and a minus sign blue, opaque, and leaves the cells past the stream clear", () => {
  const sheet: Sheet = stamp(Int8Array.from([1, -1]), 3, 1, rgb(palette.orange), rgb(palette.blue));
  expect(Array.from(sheet.image!.data)).toEqual([...rgb(palette.orange), 255, ...rgb(palette.blue), 255, 0, 0, 0, 0]);
});

test("the battery draws a bar for every test, the band under each strip, and outlines the four exact near-heads solid", () => {
  const log = open({ view: "battery" }).draw();
  expect(count(log, "fillRect")).toBe(111 + 1 + 8 + 1);
  expect(count(log, "strokeRect")).toBe(4);
  expect(log.filter(([name]) => name === "setLineDash").every(([, dash]) => Array.isArray(dash) && !dash.length)).toBe(
    true,
  );
  expect(said(log)).toContain("shift, h = 1 to 64");
  expect(said(log)).toContain("kernel and square");
  expect(count(log, "drawImage")).toBe(0);
});

test("a near-head with a carry is outlined dashed", () => {
  const log = open({ view: "battery", rudin: 1, reading: "N", power: 10 }).draw();
  expect(count(log, "strokeRect")).toBe(15);
  expect(log.filter(([name, dash]) => name === "setLineDash" && Array.isArray(dash) && dash.length)).toHaveLength(7);
});

test("the arcs view draws the bias alone while the model is summed, then both lines once the deferred job ran, and tells the facts", () => {
  const one = open({ view: "arcs", base: 4, digits: "13" });
  const first = one.draw();
  expect(strokes(first)).toBe(1 + 7 + 1);
  expect(said(first)).toContain("summing the arcs");
  expect(one.jobs).toHaveLength(1);
  one.log.length = 0;
  one.jobs[0]();
  const after = [...one.log];
  expect(strokes(after)).toBe(1 + 7 + 2);
  expect(said(after)).toContain("major arcs below 1/4");
  expect(one.told).toHaveLength(1);
  expect((one.told[0] as Extract<Facts, { view: "arcs" }>).predicted).not.toBeNull();
  expect(one.draw()).toEqual([]);
});

test("off the squares the arcs view draws one line and says why, with no job to wait for", () => {
  const one = open({ view: "arcs", reading: "Cube" });
  const log = one.draw();
  expect(strokes(log)).toBe(1 + 7 + 1);
  expect(said(log)).toContain("the major arcs predict the squares; along this reading the bias stands alone");
  expect(one.jobs).toHaveLength(0);
});

test("a set keeps the scene and tells new facts once per new plan", () => {
  const one = open({});
  one.draw();
  expect(one.scene.set({ ...BASE, power: 10 })).toBeUndefined();
  expect(one.told).toHaveLength(1);
  expect(one.told[0].count).toBe(1024);
  one.scene.set({ ...BASE, power: 10, seed: 7 });
  expect(one.told).toHaveLength(1);
  expect(drawn(one.draw())[0].slice(0, 4)).toEqual([0, 0, 32, 32]);
});

test("the scene is still and wants the num unit", () => {
  expect(open({}).scene.still).toBe(true);
  expect(() =>
    make(
      Object.assign({} as HTMLCanvasElement, { getContext: () => pen([]) }),
      Object.assign({} as View, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }),
      { ...defaults(SPEC) },
    ),
  ).toThrow("readings");
});

test("Enter plays and stops the powers on every view, its label following the play", () => {
  const enter = (view: string, on = false) =>
    labels(keymap(PAGE.keys), { view }, { playing: () => on } as Handle<Handled>).find((row) => row.key === "Enter");
  expect([enter("plaid", false), enter("arcs", true)].map((row) => [row!.act, row!.label, row!.button])).toEqual([
    ["play", "Play", true],
    ["play", "Stop", true],
  ]);
});

test("the knobs hide the base and the digits under Rudin-Shapiro, the row off the plaid, and tidy keeps the digits of the base", () => {
  const keys = (over: Raw) =>
    describe(SPEC, tidied({ ...BASE, ...over })).flatMap(({ rows }) => rows.map((row) => row.key));
  expect(keys({})).toEqual(["view", "power", "width", "rudin", "base", "digits", "reading"]);
  expect(keys({ rudin: 1, view: "battery" })).toEqual(["view", "power", "rudin", "reading"]);
  expect([
    tidied({ ...BASE, base: 6, digits: "531" }).digits,
    tidied({ ...BASE, base: 6, digits: "7" }).digits,
    tidied({ ...BASE, width: 20, power: 10 }).width,
  ]).toEqual(["531", "1", 10]);
});

test("an export is named by the design, the reading, the view and the power", () => {
  expect([
    file(BASE),
    file({ ...BASE, rudin: 1, reading: "Primes", view: "battery", power: 20 }),
    file({ ...BASE, base: 6, digits: "531" }, { num }),
    file({ ...BASE, base: 6, digits: "531" }),
  ]).toEqual([
    "readings-2-1-square-plaid-12",
    "readings-rudin-shapiro-primes-battery-20",
    "readings-6-135-square-plaid-12",
    "readings-6-135-square-plaid-12",
  ]);
});
