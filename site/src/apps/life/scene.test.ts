import { expect, test } from "bun:test";
import * as life from "@mrlyprod/mrly/life";
import * as math from "@mrlyprod/mrly/math";
import type { View } from "../../lib/app/run.ts";
import type { Raw, Scalar, Value } from "../../lib/app/spec.ts";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Life, Stat } from "./scene.ts";
import { HOLD, make, PAGE, PRESETS, RULES, SPEC, wears } from "./scene.ts";

type Said = (value: Value, scene: Life) => string;

type Picks = (value: Value) => [Scalar, Scalar][];

const pen = (log: unknown[][]) =>
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

const open = (value: Raw, { still = false, w = 640, h = 480, lock = false } = {}) => {
  const log: unknown[][] = [];
  const told: Stat[] = [];
  const start = { ...defaults(SPEC), seed: 7, ...value };
  const view = {
    rand: rng(7),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still,
    lock,
    w,
    h,
    dpr: 1,
    t: 0,
    wake: () => {},
  } as View;
  const canvas = Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) });
  const scene = make(canvas, view, { ...start, life, math, onStat: (stat) => told.push(stat) });
  const at = (t: number) => {
    log.length = 0;
    view.t = t;
    scene.draw();
  };
  const last = () => told[told.length - 1];
  const set = (patch: Raw) => scene.set(Object.assign(start, patch));
  return { log, told, scene, view, at, last, set };
};

const CELL = 4;
const LEFT = (640 - 96 * CELL) / 2;
const TOP = (480 - 96 * CELL) / 2;
const u = (c: number) => (LEFT + (c + 0.5) * CELL) / 640;
const v = (r: number) => (TOP + (r + 0.5) * CELL) / 480;

test("the page opens on the glider paused at generation 0, and Play runs it at the speed from where it stood", () => {
  const run = open({ speed: 10 });
  run.at(1000);
  expect([run.last().pop, run.last().gen, run.scene.playing()]).toEqual([5, 0, false]);
  run.scene.toggle();
  run.at(2000);
  expect([run.last().gen, run.scene.playing(), run.log.some(([verb]) => verb === "rect")]).toEqual([10, true, true]);
  run.scene.toggle();
  run.at(5000);
  expect(run.last().gen).toBe(10);
});

test("a take started while paused plays", () => {
  const run = open({ speed: 10 });
  run.view.fixed = true;
  run.at(1000);
  run.at(2000);
  expect([run.scene.playing(), run.last().gen]).toEqual([true, 10]);
});

test("the saver plays from t at the speed, and a draw with no new generation draws nothing", () => {
  const run = open({ speed: 10 }, { lock: true });
  run.at(1000);
  expect(run.last().gen).toBe(10);
  run.at(1050);
  expect([run.last().gen, run.log.length]).toEqual([10, 0]);
  run.at(3000);
  expect(run.last().gen).toBe(30);
});

test("a jump past the budget of steps a frame rebases, so play never chases a lost second", () => {
  const run = open({ speed: 60, code: "15", number: 5, level: 2 }, { lock: true });
  run.at(10000);
  const landed = run.last().gen;
  expect(landed).toBeLessThan(600);
  run.at(11000);
  expect(run.last().gen - landed).toBeLessThanOrEqual(60);
});

test("a small pattern opens on a small board so it reads, and a roomy start and the saver keep the full board", () => {
  const cell = (run: ReturnType<typeof open>) => run.log.find(([verb]) => verb === "rect")![3];
  const run = open({});
  const glider = cell(run);
  run.log.length = 0;
  run.set({ from: "gun" });
  expect([glider, cell(open({ from: "blinker" })), cell(run), cell(open({ from: "blinker" }, { lock: true }))]).toEqual(
    [14, 28, 4, 4],
  );
});

test("the saver opens the default start as a soup, and keeps a start that was chosen", () => {
  expect([
    open({}).last().pop,
    open({}, { lock: true }).last().pop > 100,
    open({ from: "pentomino" }, { lock: true }).last().pop,
  ]).toEqual([5, true, 5]);
});

test("a speed change mid-play keeps the generation count continuous", () => {
  const run = open({ speed: 10 }, { lock: true });
  run.at(1000);
  run.set({ speed: 60 });
  run.at(1016);
  expect(run.last().gen).toBeLessThanOrEqual(11);
  for (let t = 1116; t <= 2016; t += 100) run.at(t);
  expect(run.last().gen).toBe(70);
});

test("painting a blinker on a cleared board and stepping turns it, and the live fate reads loop 2", () => {
  const run = open({ from: "pentomino" });
  run.scene.clear();
  expect(run.last()).toMatchObject({ gen: 0, pop: 0, fate: "dead" });
  run.scene.touch(u(47), v(48), true);
  run.scene.touch(u(48), v(48), false);
  run.scene.touch(u(49), v(48), false);
  expect(run.last()).toMatchObject({ pop: 3, fate: "" });
  run.scene.step();
  run.scene.step();
  expect(run.last()).toMatchObject({ gen: 2, pop: 3, fate: "loop 2" });
  run.scene.touch(u(48), v(48), true);
  expect(run.last().pop).toBe(2);
});

test("run to fate says how the board ends, and a hand on the board clears the verdict", () => {
  const run = open({ from: "blinker" });
  run.scene.doom();
  expect(run.last().fate).toBe("loops after 2, period 2");
  run.scene.touch(u(10), v(10), true);
  expect(run.last().fate).toBe("");
});

test("reset sows the same start again at generation 0 and pauses", () => {
  const run = open({});
  const start = run.last().pop;
  run.scene.toggle();
  run.at(2000);
  run.scene.reset();
  expect([run.last().gen, run.last().pop, run.scene.playing()]).toEqual([0, start, false]);
});

test("a rule or mask change keeps the board, and a seed or start change sows a new one", () => {
  const run = open({ from: "pentomino" });
  run.scene.step();
  run.scene.step();
  const pop = run.last().pop;
  run.set({ born: "36" });
  expect([run.last().gen, run.last().pop, run.last().name]).toEqual([2, pop, "B36/S23 on carpet"]);
  run.set({ code: "14" });
  expect([run.last().gen, run.last().pop, run.last().label]).toEqual([2, pop, "net, side 3, level 1"]);
  run.set({ seed: 9, from: "glider" });
  expect([run.last().gen, run.last().pop]).toEqual([0, 5]);
});

test("a line runs the masked rule down a space-time sheet: rule 150 from one cell opens into three", () => {
  const rule150 = Object.fromEntries(PRESETS)["Rule 150"];
  const run = open({ ...rule150, from: "one" }, { w: 640, h: 400 });
  expect([run.last().pop, run.last().mask.shape, run.last().name]).toEqual([1, [1, 3], "B1/S02"]);
  run.scene.step();
  expect([run.last().gen, run.last().pop]).toEqual([1, 3]);
  run.scene.step();
  expect(run.last().pop).toBe(3);
});

test("a line tap edits the newest row, and only within a thumb of it", () => {
  const run = open({ ...Object.fromEntries(PRESETS)["Rule 150"], from: "one" }, { w: 640, h: 400 });
  const cell = 1;
  const rows = Math.floor(400 * 0.96);
  const top = Math.floor((400 - rows * cell) / 2);
  const left = Math.floor((640 - 320 * 1) / 2);
  const x = (left + 10.5) / 640;
  run.scene.touch(x, (top + 0.5) / 400, true);
  expect(run.last().pop).toBe(2);
  run.scene.touch(x, (top + 200) / 400, true);
  expect(run.last().pop).toBe(2);
});

test("the status names the rule on the named mask, a sequence by its name, and the mask with its cells and lattice", () => {
  expect(open({ born: "3", stay: "2 3" }).last()).toMatchObject({
    name: "B3/S23 on carpet",
    label: "carpet, side 3, level 1",
    cells: 8,
    lattice: "one",
  });
  expect(open({ born: "34-45", stay: "34-58", code: "15", number: 9 }).last().name).toBe("B34-45/S34-58");
  expect(open({ born: "primes", stay: "23", code: "14" }).last().name).toBe("Bprimes/S23 on net");
  expect(open({ dim: 1, code: "1", number: 3, level: 2 }).last()).toMatchObject({
    label: "code 1, side 3, level 2",
    cells: 4,
    lattice: "2 copies",
  });
});

test("the status hands the resolved counts, so a picker can write them back by hand", () => {
  expect(open({ born: "primes", stay: "23" }).last().counts).toEqual([
    [2, 3, 5, 7],
    [2, 3],
  ]);
});

test("a preset is worn when every key it sets matches the value, numbers and strings alike", () => {
  expect([
    wears({ dim: 2, code: "7" }, { dim: 2, code: "7" }),
    wears({ dim: "2", code: 7 }, { dim: 2, code: "7" }),
    wears({ dim: 1, code: "7" }, { dim: 2, code: "7" }),
  ]).toEqual([true, true, false]);
});

test("the primary is the Enter row, reading Play or Pause from the scene", () => {
  const run = open({});
  const enter = PAGE.keys.find((row) => row.act === "play")!;
  const before = (enter.label as Said)({}, run.scene);
  run.scene.toggle();
  expect([enter.button, before, (enter.label as Said)({}, run.scene)]).toEqual([true, "Play", "Pause"]);
});

test("the export is named by the preset or the rule, with the seed only for a soup", () => {
  const value = { ...defaults(SPEC), seed: 7 };
  const soup = { ...value, from: "noise" };
  expect([PAGE.name(value), PAGE.name(soup), PAGE.name({ ...soup, born: "primes" })]).toEqual([
    "life-conway-glider",
    "life-conway-7",
    "life-2d-7-3-1-bprimes-s23-7",
  ]);
});

test("the presets and the rules round-trip through the value, and a line keeps only the starts a line can take", () => {
  for (const [, preset] of [...PRESETS, ...RULES])
    expect(tidy(SPEC, { ...defaults(SPEC), ...preset })).toMatchObject(preset);
  expect([1, 2].map((dim) => (SPEC.find((row) => row.key === "from")!.options as Picks)({ dim }).length)).toEqual([
    2, 15,
  ]);
});

test("the Level pick offers the levels whose mask fits the span of its dimension", () => {
  const row = SPEC.find((one) => one.key === "level")!;
  expect([3, 5, 7, 9].map((number) => (row.options as Picks)({ number, dim: 2 }).map(([level]) => level))).toEqual([
    [1, 2, 3],
    [1, 2],
    [1],
    [1],
  ]);
  expect([3, 5, 7, 9].map((number) => (row.options as Picks)({ number, dim: 1 }).map(([level]) => level))).toEqual([
    [1, 2, 3],
    [1, 2],
    [1, 2],
    [1, 2],
  ]);
});

test("under reduced motion the page still is the start and the saver still is a burst", () => {
  expect(open({ from: "pentomino" }, { still: true }).last().gen).toBe(0);
  expect(open({ from: "pentomino" }, { still: true, lock: true }).last().gen).toBe(24);
});

test("a settled board in the saver reseeds after the hold, never once a hand has painted, and never on the page", () => {
  const run = open({ from: "blinker", speed: 10 }, { lock: true });
  run.at(300);
  expect(run.last()).toMatchObject({ gen: 3, pop: 3, fate: "loop 2" });
  run.at(300 + HOLD - 50);
  expect(run.last().gen).toBeGreaterThan(3);
  run.at(400 + HOLD);
  expect(run.last()).toMatchObject({ gen: 0, pop: 3, fate: "" });
  const held = open({ from: "blinker", speed: 10 }, { lock: true });
  held.at(300);
  held.scene.clear();
  held.at(400 + 2 * HOLD);
  expect(held.last()).toMatchObject({ pop: 0, fate: "dead" });
  const page = open({ from: "blinker", speed: 10 });
  page.scene.toggle();
  page.at(300);
  page.at(400 + HOLD);
  expect(page.last().gen).toBeGreaterThan(30);
});

test("the paints are read once at make and again on theme, never on a draw", () => {
  let looks = 0;
  const view = {
    rand: rng(7),
    look: () => {
      looks++;
      return { paper: "#000000", accent: "#008cff" };
    },
    still: false,
    w: 640,
    h: 480,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen([]) }), view, {
    ...defaults(SPEC),
    seed: 7,
    speed: 60,
    life,
    math,
  });
  const made = looks;
  for (let t = 100; t <= 1000; t += 100) {
    view.t = t;
    scene.draw();
  }
  expect([made, looks]).toEqual([1, 1]);
  scene.theme();
  expect(looks).toBe(2);
});
