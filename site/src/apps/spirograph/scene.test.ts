import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { Readout } from "../../lib/app/hooks.ts";
import type { Handle } from "../../lib/app/page.tsx";
import { keymap, labels } from "../../lib/app/page.tsx";
import { defaults, describe, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Shape } from "./engine.ts";
import { cover, EASE, fit, HOLD, index, PAD, ROUND, roll, span, study } from "./engine.ts";
import type { Given, Played, Seen, Spirograph, Stat } from "./scene.ts";
import { file, make, PAGE, parked, SPEC, scrub, status } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

type Sheet = { width: number; height: number; log: Call[] };

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

const sheets: Sheet[] = [];

Object.assign(globalThis, {
  OffscreenCanvas: class {
    width: number;
    height: number;
    log: Call[];
    constructor(w: number, h: number) {
      this.width = w;
      this.height = h;
      this.log = [];
      sheets.push(this);
    }
    getContext() {
      return pen(this.log);
    }
  },
});

const BASE = { ...defaults(SPEC), seed: 1 } as Spirograph;

const stage = (opts: Given, still = false, w = 800, h = 600, lock = false) => {
  const log: Call[] = [];
  const woke: string[] = [];
  const view: Seen = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still,
    lock,
    w,
    h,
    dpr: 2,
    t: 0,
  };
  view.wake = () => {
    view.still = false;
    woke.push("wake");
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, { math, ...opts });
  const at = (t: number) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { view, scene, woke, at };
};

const open = (value: Partial<Spirograph> = {}, still?: boolean, w?: number, h?: number) => {
  const told: Stat[] = [];
  const page = stage({ ...BASE, ...value, tell: (one) => told.push(one) }, still, w, h);
  return { ...page, told, last: () => told.at(-1)! };
};

const PLAN = study(math, BASE, "495");
const RAIL = PLAN.track.outline.length - 1;
const LENGTH = span(PLAN.beats);
const WHOLE = PLAN.facts.pencils * (PLAN.samples - 1);

const drawn = () => sheets.at(-1)!.log.filter(([name]) => name === "lineTo").length - RAIL;

const fills = (log: Call[]) => log.filter(([name, , , , w]) => name === "drawImage" && w !== undefined);

test("the page opens on the whole picture with the wheel at the end, and nothing moves after", () => {
  const page = open();
  const log = page.at(0);
  expect(drawn()).toBe(WHOLE);
  const box = fit(PLAN.frame, 800, 600, PAD * 2);
  const [cx, cy] = math.spirograph.pose(PLAN.track, PLAN.track.total);
  const [, x, y, r] = log.find(([name]) => name === "arc")!;
  expect([x, y, r]).toEqual([box.ox + cx * box.k, box.oy - cy * box.k, PLAN.track.wheel * box.k]);
  expect(page.at(60000)).toEqual([]);
  expect(page.last()).toMatchObject({ part: 1, playing: false, fault: null, cover: null });
});

test("play on a whole picture draws it from blank over the span, then stops whole and says so", () => {
  const page = open();
  page.at(0);
  page.scene.toggle();
  expect(page.last().playing).toBe(true);
  page.at(LENGTH / 2);
  expect(drawn()).toBe(PLAN.facts.pencils * index(0.5, PLAN.samples));
  page.at(LENGTH);
  expect(drawn()).toBe(WHOLE);
  expect(page.last()).toMatchObject({ part: 1, playing: false });
  expect(page.at(LENGTH * 3)).toEqual([]);
});

test("stop parks the drawing at its share to a tenth of a percent", () => {
  const page = open();
  page.at(0);
  page.scene.toggle();
  page.at(1234);
  page.scene.toggle();
  expect(page.last()).toMatchObject({ playing: false, part: parked(1234 / LENGTH) / 100 });
  page.at(5000);
  expect(page.last().part).toBe(parked(1234 / LENGTH) / 100);
});

test("a drawn share set from outside parks the picture there, going back on a fresh layer", () => {
  const page = open();
  page.at(0);
  const before = sheets.length;
  page.scene.set({ ...BASE, at: 50 });
  page.at(10);
  expect([sheets.length > before, drawn()]).toEqual([true, PLAN.facts.pencils * index(0.5, PLAN.samples)]);
  expect(page.last().part).toBe(0.5);
});

test("a track knob keeps the scene and restudies on the next draw at the same share, telling the new facts", () => {
  const page = open({ at: 50 });
  page.at(0);
  page.scene.set({ ...BASE, at: 50, ring: 8 });
  expect(page.last().facts.ratio).toEqual([7, 3]);
  page.at(10);
  const next = study(math, { ...BASE, ring: 8 }, "495");
  expect([page.last().facts.ratio, drawn()]).toEqual([[8, 3], next.facts.pencils * index(0.5, next.samples)]);
});

test("a knob turned during play keeps the play going from the same share", () => {
  const page = open();
  page.at(0);
  page.scene.toggle();
  page.at(LENGTH / 4);
  page.scene.set({ ...BASE, wheel: 2 });
  page.at(LENGTH / 4 + 1);
  expect(page.last().playing).toBe(true);
  expect(page.last().part).toBeCloseTo(0.25, 2);
});

test("ink by kind paints fills blue and voids orange, six inks cycle by pencil, one ink shades the accent", () => {
  const page = open({ mode: "both" });
  page.at(0);
  const strokes = () => [...new Set(page.scene.svg().match(/stroke="[^"]+"/g))];
  expect(strokes()).toEqual(expect.arrayContaining([`stroke="${palette.blue}"`, `stroke="${palette.orange}"`]));
  page.scene.set({ ...BASE, mode: "both", ink: "wheel" });
  page.at(1);
  expect(
    (["blue", "orange", "yellow", "green", "pink", "indigo"] as const).every((hue) =>
      strokes().includes(`stroke="${palette[hue]}"`),
    ),
  ).toBe(true);
  page.scene.set({ ...BASE, mode: "both", ink: "one" });
  page.at(2);
  expect(strokes().filter((one) => one.startsWith('stroke="rgb(')).length).toBeGreaterThan(1);
});

test("fill lays the shape under the trace at once with no ring drawn round it, and turning it off drops it", () => {
  const page = open({ fill: 1 });
  const log = page.at(0);
  const box = fit(PLAN.frame, 800, 600, PAD * 2);
  const { x, y, radius } = (cover(math, PLAN) as Shape).disc;
  const [shape] = fills(log);
  expect(shape.slice(2)).toEqual([
    box.ox + (x - radius) * box.k,
    box.oy - (y + radius) * box.k,
    2 * radius * box.k,
    2 * radius * box.k,
  ]);
  expect(log.findIndex(([name]) => name === "drawImage")).toBe(log.indexOf(shape));
  expect(log.filter(([name, , , r]) => name === "arc" && r === radius * box.k)).toEqual([]);
  expect(page.last().cover).toMatchObject({ side: 512 });
  expect((page.last().cover as Partial<Shape>).mask).toBeUndefined();
  page.scene.set({ ...BASE, fill: 0 });
  expect(fills(page.at(1))).toEqual([]);
  expect(page.last().cover).toBeNull();
});

test("fill on a line tells why there is no shape and lays none", () => {
  const page = open({ kind: "line", fill: 1 });
  expect(fills(page.at(0))).toEqual([]);
  expect(page.last().cover).toEqual({ fault: ROUND });
});

test("a code the crate cannot read keeps the last picture and tells the fault, and opening on one falls back to the carpet", () => {
  const page = open();
  page.at(0);
  page.scene.set({ ...BASE, code: "abc" });
  page.at(1);
  expect([Boolean(page.last().fault), page.last().facts.code]).toEqual([true, "495"]);
  const bad = open({ base: 2, code: "495" });
  expect([Boolean(bad.last().fault), bad.last().facts.code, bad.last().facts.pencils]).toEqual([true, "495", 8]);
});

test("the lock rolls a design from its seed in one ink, draws it over the span, holds it, then starts over", () => {
  const lock = stage({ seed: 1 }, false, 800, 600, true);
  const plan = study(math, BASE, roll(math, 3, rng(1)));
  const length = span(plan.beats) * EASE;
  lock.at(length);
  const full = drawn();
  expect(full).toBe(plan.facts.pencils * (plan.samples - 1));
  expect(lock.scene.svg()).toContain('stroke="rgb(');
  lock.at(length + HOLD / 2);
  expect(drawn()).toBe(full);
  lock.at(length + HOLD + 100);
  expect(drawn()).toBeLessThan(full / 10);
});

test("a take on a whole picture plays it from blank", () => {
  const page = open();
  page.at(0);
  page.scene.take();
  page.at(100);
  expect(drawn()).toBeLessThan(WHOLE / 10);
  expect(page.last().playing).toBe(true);
});

test("under reduced motion play wakes the scene", () => {
  const page = open({}, true);
  page.at(0);
  page.scene.toggle();
  expect(page.woke).toEqual(["wake"]);
});

test("a resize or a theme change lays the layer again at the same share", () => {
  const page = open({ at: 40 });
  page.at(0);
  const count = drawn();
  page.scene.size();
  page.at(1);
  expect(drawn()).toBe(count);
  page.scene.theme();
  page.at(2);
  expect(drawn()).toBe(count);
});

test("a stage narrower than the padding still lays every sheet at a whole positive size", () => {
  const before = sheets.length;
  const tiny = open({}, false, 30, 20);
  tiny.at(1000);
  tiny.scene.size();
  tiny.at(2000);
  const laid = sheets.slice(before);
  expect(laid.length).toBeGreaterThan(2);
  expect(
    laid.every(({ width, height }) => Number.isInteger(width) && Number.isInteger(height) && width > 0 && height > 0),
  ).toBe(true);
});

test("every seat carries a dot on the wheel at any pencil count", () => {
  const page = open({ number: 7, level: 2, mode: "corners" });
  const arcs = page.at(0).filter(([name]) => name === "arc");
  expect(arcs.length).toBe(2356 + 2);
});

test("the svg is a sheet of css pixels holding the track and one path per pencil up to the drawn share", () => {
  const page = open({ at: 50 });
  page.at(0);
  const text = page.scene.svg();
  expect(text).toContain('width="400" height="300" viewBox="0 0 400 300"');
  expect(text.match(/<path /g)).toHaveLength(1 + PLAN.facts.pencils);
  expect(text).toContain('stroke-dasharray="4 6"');
  expect(open().scene.svg().length).toBeGreaterThan(text.length);
});

test("the lib pane holds no row; sides show on polygon tracks and laps on every track but a circle", () => {
  expect(describe(SPEC, BASE)).toEqual([]);
  const shown = (kind: string) =>
    SPEC.filter((row) => ["sides", "laps"].includes(row.key) && (!row.need || row.need({ kind }))).map(
      (row) => row.key,
    );
  expect(["in", "out", "line", "polyin", "polyout"].map(shown)).toEqual([
    [],
    [],
    ["laps"],
    ["sides", "laps"],
    ["sides", "laps"],
  ]);
});

test("the wheel slider stops short of the ring inside a circle, and the level at what the side allows", () => {
  expect(tidy(SPEC, { ...BASE, ring: 4, wheel: 9 }).wheel).toBe(3);
  expect(tidy(SPEC, { ...BASE, kind: "out", ring: 4, wheel: 9 }).wheel).toBe(9);
  expect([
    tidy(SPEC, { ...BASE, number: 7, level: 3 }, { math }).level,
    tidy(SPEC, { ...BASE, number: 3, level: 3 }, { math }).level,
  ]).toEqual([2, 3]);
});

test("Space and Enter are the one play, the primary reading Play or Stop as the scene plays", () => {
  const page = open();
  const map = () => labels(keymap(PAGE.keys, [], { own: true }), BASE, page.scene as Handle<Played>);
  const label = (key: string) => map().find((row) => row.key === key)!.label;
  expect([label(" "), label("Enter"), map().find((row) => row.key === "Enter")!.button]).toEqual([
    "Play",
    "Play",
    true,
  ]);
  page.scene.toggle();
  expect([page.scene.playing(), label(" "), label("Enter")]).toEqual([true, "Pause", "Stop"]);
});

test("an export is named by the design, the track and the radii, the empty code the carpet", () => {
  const track = { ...BASE, kind: "polyin", ring: 9, wheel: 2 };
  expect([file(track, { math }), file(track), file({ ...track, code: "7", base: 2 }, { math })]).toEqual([
    "spirograph-495-polyin-9-2",
    "spirograph-carpet-polyin-9-2",
    "spirograph-7-polyin-9-2",
  ]);
});

test("the play is an app action on the scene, the reroll is the framework event, and the wheel takes a plane design through the chip", () => {
  const calls: string[] = [];
  PAGE.actions.toggle(Object.assign({} as Handle<Played>, { toggle: () => calls.push("toggle") }));
  expect([calls, "reroll" in PAGE.actions, PAGE.design, PAGE.keys.map((row) => row.menu)]).toEqual([
    ["toggle"],
    false,
    { dim: 2, base: "base", code: "code" },
    ["Edit"],
  ]);
});

test("an empty code is the home design of the base, never a roll", () => {
  const page = open({ code: "" });
  expect(page.last().facts.code).toBe("495");
  expect(open({ code: "", base: 2 }).last().facts.code).toBe("7");
});

const stat = (over?: Partial<Stat>, value: Partial<Spirograph> = {}) => {
  const plan = study(math, { ...BASE, ...value }, value.code ?? "495");
  return { facts: plan.facts, fault: null, cover: null, ms: 3.4, part: 1, ...over };
};

const row = (rows: Readout[], name: string) => rows.find(([one]) => one === name)?.[1];

test("the readouts lead with the wheel and its counts, then the law, the node formula, the seats and the path", () => {
  const rows = status(stat());
  expect(rows.slice(0, 7)).toEqual([
    ["wheel", "carpet 495 base 3 level 1"],
    ["pencils", 8],
    ["curves", 8],
    ["nodes", 1288],
    ["turns", "4"],
    ["drawn", "100%"],
    ["draw", "3 ms"],
  ]);
  expect(row(rows, "law")).toBe("R/r = 7/3, closes after 3 orbits, 7-fold, 8 distinct of 8");
  expect(row(rows, "crossings")).toBe("N = 2ab C(k,2) + k a(b - 1) = 1288");
  expect(row(rows, "seats")).toBe("8 on fills, 0 on voids, 0 on corners, reach 0.90 r, cell 0.4243 r");
  expect(row(rows, "path")).toBe(`length 75.40, ${PLAN.samples} samples a pencil`);
  expect(status(null)).toEqual([]);
});

test("a seat at the centre gives the reason for no node count, a line its shift law, a polygon no law, and a fault leads", () => {
  expect(row(status(stat({}, { mode: "both" })), "crossings")).toStartWith("not the law");
  const line = status(stat({}, { kind: "line" }));
  expect([(row(line, "law") as string).includes("shifts of one shape"), row(line, "crossings")]).toEqual([
    true,
    undefined,
  ]);
  expect(row(status(stat({}, { kind: "polyin" })), "law")).toContain("no coincidence law");
  expect(status(stat({ fault: "boom" }))[0]).toEqual(["fault", "boom"]);
});

test("the cover readout gives the share covered, then wall, hole, winding and disc, or the reason there is none", () => {
  const { mask, ...shape } = cover(math, PLAN) as Shape;
  const rows = status(stat({ cover: shape }));
  expect(row(rows, "covered")).toBe(`${Math.round(shape.covered * 100)}%`);
  expect(row(rows, "cover")).toMatch(
    /^wall [\d.]+%, hole [\d.]+%, winding [\d.]+ of an exact [\d.]+, disc 5\.800 out, 2\.200 in, raster 512$/,
  );
  expect([
    row(status(stat({ cover: { fault: ROUND } })), "cover"),
    row(status(stat({ cover: { fault: ROUND } })), "covered"),
  ]).toEqual([ROUND, undefined]);
});

test("a drag scrubs by the share of the stage it crosses, held between blank and whole, to a tenth of a percent", () => {
  expect([scrub(50, 100, 400), scrub(90, 200, 400), scrub(10, -100, 400), parked(0.12345)]).toEqual([75, 100, 0, 12.3]);
});
