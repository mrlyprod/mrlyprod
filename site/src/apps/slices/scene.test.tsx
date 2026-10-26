import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import type { Readout } from "../../lib/app/hooks.ts";
import { Knobs } from "../../lib/app/knobs.tsx";
import type { Scene } from "../../lib/app/run.ts";
import type { Value } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Slices, Told } from "./scene.ts";
import { codesOf, glance, layout, make, name, SPEC, scrub, study, TOP, thumb } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

const value = (over: Value) => ({ ...defaults(SPEC), seed: 1, base: 2, code: "23", ...over });

const row = (rows: Readout[], name: string) => rows.find(([one]) => one === name)?.[1];

const pen = (log: Call[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) => (key === "measureText" ? () => ({ width: 10 }) : (...args: unknown[]) => log.push([key, ...args])),
    set: (_, key, v: unknown) => {
      log.push([`${key as string}=`, v]);
      return true;
    },
  });

const open = (over: Value = {}) => {
  const log: Call[] = [];
  const told: Told[] = [];
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: false,
    w: 800,
    h: 600,
    dpr: 1,
    t: 0,
  };
  const scene: Slices & Scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...value(over),
    math,
    tell: (one) => told.push(one),
  });
  return { log, told, view, scene, count: (name: string) => log.filter(([key]) => key === name).length };
};

test("the page opens on the carpet's middle hexagon of side 3, its closed form and its exact split between carpet and net", () => {
  const plan = study(math, value({}));
  expect(plan.rows.slice(0, 9)).toEqual([
    ["name", "carpet 23"],
    ["side", 3],
    ["triangles", 54],
    ["closed form", "54"],
    ["filled", 42],
    ["empty", 12],
    ["pieces", 1],
    ["holes", 1],
    ["largest piece", 42],
  ]);
  expect(plan.rows.slice(-5)).toEqual([
    ["carpet", 42],
    ["net", 12],
    ["together", 54],
    ["hexagon", 54],
    ["partition", "exact"],
  ]);
});

test("the side runs the odd sides to 31: the carpet falls into pieces at odd k and closes round holes at even k", () => {
  const at = (k: number) => study(math, value({ k })).rows;
  expect(row(at(TOP), "side")).toBe(31);
  expect([3, 4, 5, 6].map((k) => [k, row(at(k), "pieces"), row(at(k), "holes")])).toEqual([
    [3, 7, 0],
    [4, 1, 7],
    [5, 19, 0],
    [6, 1, 19],
  ]);
});

test("a fractal tile grows its own level, capped by the tile, and drops the partition past level 1", () => {
  const plan = study(math, value({ tile: 3, level: 9 }));
  expect([row(plan.rows, "side"), row(plan.rows, "closed form"), row(plan.rows, "partition")]).toEqual([
    81,
    "39366",
    undefined,
  ]);
  expect(study(math, value({ tile: 5, level: 9 })).rows[1]).toEqual(["side", 25]);
});

test("the scene is still: it paints the hexagon alone, one fill a triangle, and repaints the same canvas at once on a set", () => {
  const live = open();
  expect([live.scene.still, live.scene.every]).toEqual([true, undefined]);
  live.scene.draw();
  expect([live.count("fillRect"), live.count("fillText"), live.count("fill")]).toEqual([0, 0, 54]);
  live.log.length = 0;
  expect(live.scene.set(value({ k: 4 }))).toBe(true);
  expect([live.count("clearRect"), row(live.told.at(-1)!.rows, "holes")]).toEqual([1, 7]);
});

test("the picture takes the whole stage at any aspect, a phone included", () => {
  const [wide, tall] = [layout(1600, 900), layout(390, 844)];
  expect([wide.x, wide.w, wide.h, tall.x, tall.y, tall.w, tall.h].map((v) => +v.toFixed(1))).toEqual([
    36, 1528, 828, 15.6, 15.6, 358.8, 812.8,
  ]);
});

test("a drag across the stage picks the side under it, from 1 at the left edge to 16 at the right", () => {
  expect([scrub(0, 800), scrub(400, 800), scrub(799, 800), scrub(-20, 800), scrub(900, 800)]).toEqual([
    1,
    9,
    TOP,
    1,
    TOP,
  ]);
  expect(open().scene.seek(799)).toBe(TOP);
});

test("a side's thumb is its own hexagon, one fill a triangle, kept for the next paint", () => {
  const log: Call[] = [];
  thumb(pen(log), 56, glance(math, value({}), 2), codesOf(math), "#008cff");
  expect(log.filter(([key]) => key === "fill").length).toBe(54);
  expect(glance(math, value({}), 2)).toBe(glance(math, value({}), 2));
});

test("the svg holds the hexagon alone, one polygon a triangle", async () => {
  const text = await open().scene.svg();
  expect([text.startsWith("<svg"), (text.match(/<polygon /g) ?? []).length]).toEqual([true, 54]);
});

test("the level knob shows only for a tile and runs to the tile cap", () => {
  const html = (over: Value) =>
    renderToStaticMarkup(<Knobs spec={SPEC} value={value(over)} units={{ math }} onChange={() => {}} />);
  expect([html({}).includes("<span>Level</span>"), html({}).includes("<span>Side k</span>")]).toEqual([false, true]);
  expect([
    /<span>Level<\/span><span class="num">1<\/span><input type="range" min="1" max="(\d)"/.exec(html({ tile: 3 }))![1],
    /max="(\d)"/.exec(html({ tile: 5 }).split("<span>Level</span>")[1])![1],
  ]).toEqual(["4", "2"]);
});

test("an export is named by the design and the side or tile it shows", () => {
  expect([name(value({})), name(value({ tile: 3, level: 2 }))]).toEqual(["slices-23-k2", "slices-23-t3-l2"]);
});
