import { expect, test } from "bun:test";
import * as num from "@mrlyprod/mrly/num";
import { shown } from "../../lib/app/knobs.tsx";
import type { View } from "../../lib/app/run.ts";
import type { Value } from "../../lib/app/spec.ts";
import { defaults, describe, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Setting } from "./engine.ts";
import { BEAT, FADE, FLASH, FOLDING, HEAD, HOLD, index, PAD, plane, REACH, snap, study } from "./engine.ts";
import type { Seen, Told } from "./scene.ts";
import { CYCLE, DAWN, DRIFT, hum, inked, make, PAGE, SLOW, SPEC, SPREAD, sketch } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

const pen = (log: Call[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, v: unknown) => {
      log.push([key, v]);
      return true;
    },
  });

const DPR = 2;

const open = (value: Value = {}, { still = false, w = 800, h = 600, lock = false } = {}) => {
  const log: Call[] = [];
  const told: Told[] = [];
  const woke: string[] = [];
  const view: Seen & Pick<View, "rand"> = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#008cff", ink: "#ffffff" }),
    still,
    lock,
    w,
    h,
    dpr: DPR,
    t: 0,
    fixed: false,
  };
  view.wake = () => {
    view.still = false;
    woke.push("wake");
  };
  const start = { ...defaults(SPEC), seed: 1, ...value };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...start,
    num,
    ...(lock ? {} : { onTell: (said: Told) => told.push(said) }),
  });
  const at = (t: number) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  const set = (patch: Value) => {
    Object.assign(start, patch);
    scene.set({ ...start });
  };
  return { view, scene, told, woke, at, set, last: () => told.at(-1)! };
};

const plan = study(num);

const spot = (t: number, w = 800, h = 600) => {
  const i = index(t);
  const box = plane({ x: 0, y: 0, w, h }, plan.peak[i], PAD * DPR);
  return [box.ox + plan.xs[i] * box.k, box.oy - plan.ys[i] * box.k];
};

const dot = (log: Call[]) => log.find(([name, , , r]) => name === "arc" && r === HEAD * DPR)!.slice(1, 3);

const texts = (log: Call[]) => log.filter(([name]) => name === "fillText").map(([, text]) => text);

const strokes = (log: Call[]) => {
  const out: unknown[][] = [];
  let pts: unknown[] = [];
  for (const [name, x, y] of log) {
    if (name === "beginPath") pts = [];
    if (name === "moveTo" || name === "lineTo") pts.push(x, y);
    if (name === "stroke") out.push(pts);
  }
  return out;
};

const flashes = (log: Call[]) =>
  log.filter(([name, v]) => name === "strokeStyle" && /^rgba\(255, 209, 0, 0\.\d+\)$/.test(v as string));

test("the page opens standing at its head and waits, telling the head and the reading", () => {
  const page = open();
  expect(dot(page.at(0))).toEqual(spot(30));
  expect(dot(page.at(5000))).toEqual(spot(30));
  expect([page.scene.playing(), page.scene.word()]).toEqual([false, "Play"]);
  expect(page.last()).toMatchObject({ t: 30, walk: { count: 3 }, stairs: { x: 100, zeros: 10 } });
});

test("go walks the head from where it stands at the speed, stands at the end and offers a replay from zero", () => {
  const page = open({ speed: 4 });
  page.at(0);
  page.scene.go();
  expect(page.scene.word()).toBe("Pause");
  page.at(1000);
  expect(page.last().t).toBe(34);
  page.at(100000);
  expect([page.last().t, page.scene.word()]).toEqual([REACH, "Replay"]);
  expect(dot(page.at(110000))).toEqual(spot(REACH));
  page.scene.go();
  expect([page.last().t, page.scene.word()]).toEqual([0, "Pause"]);
});

test("go while walking pauses the head where it is", () => {
  const page = open({ speed: 4 });
  page.scene.go();
  page.at(500);
  page.scene.go();
  expect([page.last().t, page.scene.word()]).toEqual([32, "Play"]);
  expect(dot(page.at(5000))).toEqual(spot(32));
});

test("a t from the knobs moves the head and stops the walk, while the echo of a told t leaves the walk going", () => {
  const page = open({ speed: 4 });
  page.scene.go();
  page.at(1000);
  page.set({ t: 34 });
  expect(page.scene.playing()).toBe(true);
  page.at(2000);
  expect(page.last().t).toBe(38);
  page.set({ t: 10 });
  expect([page.last().t, page.scene.playing()]).toEqual([10, false]);
  expect(dot(page.at(3000))).toEqual(spot(10));
});

test("each zero the walk passes flashes a ring out of the origin that is gone within the flash", () => {
  const page = open({ t: 14, speed: 4 });
  page.at(0);
  expect(flashes(page.at(10))).toHaveLength(0);
  page.scene.go();
  page.at(100);
  expect(flashes(page.at(150))).toHaveLength(1);
  expect(flashes(page.at(100 + FLASH + 20))).toHaveLength(0);
});

test("the arrows hop the head from zero to zero, counting the zero it lands on, and back to the start", () => {
  const page = open();
  page.scene.hop(1);
  expect(page.last().walk.t).toBe(plan.zeros[3]);
  expect(page.last()).toMatchObject({ t: snap(plan.zeros[3]), walk: { count: 4 } });
  for (let k = 0; k < 4; k++) page.scene.hop(-1);
  expect(page.last().walk.t).toBe(0);
});

test("the lock walks by itself, holds at the end, fades and starts again from zero", () => {
  const lock = open({}, { lock: true });
  lock.at(0);
  expect(dot(lock.at(1000))).toEqual(spot(34));
  const end = 1000 + ((REACH - 34) / 4) * 1000;
  expect(dot(lock.at(end))).toEqual(spot(REACH));
  expect(lock.at(end + HOLD - 300)).toContainEqual(["fillStyle", "rgba(128, 198, 255, 0.5)"]);
  lock.at(end + HOLD);
  expect(dot(lock.at(end + HOLD + 1000))).toEqual(spot(4));
});

test("the lock draws the origin ring in the tint and the head a step toward the ink, while the page keeps yellow and orange", () => {
  const inks = (log: Call[]) =>
    log
      .filter(([name]) => name === "strokeStyle" || name === "fillStyle")
      .slice(-2)
      .map(([, v]) => v);
  const page = open().at(0);
  const lock = open({}, { lock: true }).at(0);
  expect(inks(page)).toEqual(["rgba(255, 209, 0, 1)", "rgba(255, 143, 44, 1)"]);
  expect(inks(lock)).toEqual(["rgba(0, 140, 255, 1)", "rgba(128, 198, 255, 1)"]);
});

test("a take walks the page by itself like the lock", () => {
  const page = open();
  page.view.fixed = true;
  page.at(0);
  page.at(1000);
  expect([page.last().t, page.scene.playing()]).toEqual([34, true]);
});

test("under reduced motion the page draws its still and go wakes it", () => {
  const page = open({}, { still: true });
  expect(dot(page.at(0))).toEqual(spot(30));
  page.scene.go();
  expect(page.woke).toEqual(["wake"]);
  page.at(1000);
  expect(page.last().t).toBe(34);
});

test("the stairs draw every zero the slider asks for at once, and a fold folds them in one by one, telling each, and stands", () => {
  const page = open({ view: "stairs", zeros: 10 });
  page.at(0);
  expect(page.last().stairs.zeros).toBe(10);
  page.set({ zeros: 40 });
  expect(page.last().stairs.zeros).toBe(40);
  page.at(10);
  page.scene.go();
  expect([page.scene.word(), page.last().stairs.zeros]).toEqual(["Fold", 0]);
  page.at(10 + FOLDING / 2);
  expect(page.last().stairs.zeros).toBe(20);
  page.at(10 + FOLDING + 10);
  expect(page.last().stairs.zeros).toBe(40);
  page.at(20 + FOLDING * 3);
  expect(page.last().stairs.zeros).toBe(40);
});

test("Space on the stairs stands a fold with every zero in, and folds again once it stands", () => {
  const page = open({ view: "stairs", zeros: 40 });
  page.at(0);
  page.scene.toggle();
  page.at(FOLDING / 2);
  const mid = [page.scene.playing(), page.last().stairs.zeros];
  page.scene.toggle();
  expect([mid, page.scene.playing(), page.last().stairs.zeros]).toEqual([[true, 20], false, 40]);
  page.scene.toggle();
  expect(page.scene.playing()).toBe(true);
});

test("the Enter row is the primary and reads its word from the scene, or from the view before one is ready", () => {
  const [enter] = PAGE.keys;
  const label = enter.label as Exclude<typeof enter.label, string>;
  const page = open();
  page.scene.go();
  expect([
    enter.button,
    label({ view: "walk" }, page.scene),
    label({ view: "walk" }, null),
    label({ view: "stairs" }, null),
  ]).toEqual([true, "Pause", "Play", "Fold"]);
});

test("the export is named by the view shown: the head on the walk, x and the zeros on the stairs", () => {
  expect([
    PAGE.name({ view: "walk", t: 30.05, x: 100, zeros: 10 }),
    PAGE.name({ view: "stairs", t: 30, x: 250, zeros: 12 }),
  ]).toEqual(["zeta-walk-30.05", "zeta-stairs-250-12"]);
});

test("the t slider reads with two decimals", () => {
  const row = SPEC.find((one) => one.key === "t")!;
  expect([shown(row, 30, undefined), shown(row, 34.45, undefined)]).toEqual(["30.00", "34.45"]);
});

test("the wave is the explicit formula less x over the root of x, its last zero folding in by its fraction", () => {
  const x = 37.5;
  const u = Math.log(x);
  const less = (k: number) =>
    (num.zeta.psi_formula(x, plan.zeros.subarray(0, k)) - x + Math.log(2 * Math.PI) + 0.5 * Math.log(1 - 1 / (x * x))) /
    Math.sqrt(x);
  expect(hum(plan.zeros, 0, u)).toBeCloseTo(0, 12);
  expect(hum(plan.zeros, 10, u)).toBeCloseTo(less(10), 9);
  expect(hum(plan.zeros, 9.5, u)).toBeCloseTo((less(9) + less(10)) / 2, 9);
});

test("the lock draws the stairs as one wave in the tint, flat at the start, folding its zeros in SLOW times slower, then holding, fading and starting flat again", () => {
  const lock = open({ view: "stairs", zeros: 10 }, { lock: true });
  const length = 11 * BEAT * SLOW;
  const restart = length + HOLD * SLOW;
  const wave = (t: number) => {
    const log = lock.at(t);
    const lines = strokes(log);
    const inks = log.filter(([name]) => name === "strokeStyle").map(([, v]) => v);
    expect([lines.length, inks.length]).toEqual([1, 1]);
    return { ys: lines[0].filter((_, i) => i % 2), ink: inks[0] };
  };
  const first = (k: number, t: number) => 300 - hum(plan.zeros, k, DAWN + (DRIFT * t) / 1000) * (300 - PAD * DPR);
  expect(wave(0)).toMatchObject({ ink: "rgba(0, 140, 255, 1)" });
  expect(new Set(wave(0).ys)).toEqual(new Set([300]));
  expect(wave(length / 2).ys[0]).toBeCloseTo(first(5, length / 2), 9);
  const full = wave(length);
  expect([full.ys[0], full.ink]).toEqual([expect.closeTo(first(10, length), 9), "rgba(0, 140, 255, 1)"]);
  expect(wave(restart - (FADE * SLOW) / 2).ink).toBe("rgba(0, 140, 255, 0.5)");
  const again = wave(restart);
  expect([new Set(again.ys), again.ink]).toEqual([new Set([300]), "rgba(0, 140, 255, 1)"]);
});

test("a narrow stage shows less of log x so the top zero keeps CYCLE points a turn, while a wide one shows all of it", () => {
  const ys = (w: number) =>
    strokes(open({ view: "stairs", zeros: 40, x: 150 }, { lock: true, still: true, w, h: 1688 }).at(0))[0].filter(
      (_, i) => i % 2,
    );
  const y = (u: number) => 844 - hum(plan.zeros, 40, u) * (844 - PAD * DPR);
  const span = 390 / SPREAD;
  const phone = ys(780);
  expect(phone.at(-1)).toBeCloseTo(y(DAWN + span), 9);
  expect(((phone.length - 1) * 2 * Math.PI) / (plan.zeros[39] * span)).toBeGreaterThanOrEqual(CYCLE);
  expect(ys(2880).at(-1)).toBeCloseTo(y(Math.log(150)), 9);
});

test("the stage carries no words: neither view writes a label or a number on the picture", () => {
  expect(texts(open().at(0))).toEqual([]);
  expect(texts(open({ view: "stairs" }).at(0))).toEqual([]);
});

test("the view puts one picture on the stage, and a walk left running waits while the stairs show", () => {
  const page = open({ speed: 4 });
  page.at(0);
  page.scene.go();
  page.at(1000);
  page.set({ view: "stairs" });
  const stairs = page.at(5000);
  expect(stairs.some(([name]) => name === "arc")).toBe(false);
  expect(page.scene.word()).toBe("Fold");
  page.set({ view: "walk" });
  const walk = page.at(5100);
  expect(walk.some(([name]) => name === "clip")).toBe(false);
  expect(dot(walk)).toEqual(spot(34.4));
});

test("a tint or a resize reaches the running scene: the new ink paints the walk and the walk centres on the new stage, phone aspect too", () => {
  let accent = "#008cff";
  const page = open();
  page.view.look = () => ({ paper: "#000000", accent });
  page.at(0);
  accent = "#ff3d40";
  page.set({ tint: "red" });
  expect(page.at(10)).toContainEqual(["strokeStyle", "rgba(255, 61, 64, 1)"]);
  Object.assign(page.view, { w: 390, h: 844 });
  expect(dot(page.at(20))).toEqual(spot(30, 390, 844));
});

test("the defaults are a fixed point of tidy, the view leads the first group and each view shows its own knobs", () => {
  const value: Value = { ...defaults(SPEC), seed: 1 };
  expect({ ...value, ...tidy(SPEC, value) }).toEqual(value);
  expect(value.view).toBe("walk");
  const names = (view: string) =>
    describe(SPEC, { ...value, view }).map((group) => [group.name, group.rows.map((row) => row.key)]);
  expect(names("walk")).toEqual([
    ["Zeta", ["view", "t", "speed"]],
    ["Look", ["look", "trail", "tint"]],
  ]);
  expect(names("stairs")).toEqual([
    ["Zeta", ["view", "zeros", "x"]],
    ["Look", ["tint"]],
  ]);
});

test("the svg is the picture shown", () => {
  const page = open();
  page.at(0);
  const text = page.scene.svg();
  expect(text).toContain('width="400" height="300"');
  expect(text).toContain('stroke="#008cff"');
});

test("a canvas smaller than its padding draws every arc with a radius of zero or more", () => {
  for (const [w, h] of [
    [1, 150],
    [300, 1],
    [1, 1],
  ]) {
    for (const still of [false, true]) {
      const page = open({}, { still, w, h });
      const log = [...page.at(0), ...page.at(5000)];
      const arcs = log.filter(([name]) => name === "arc");
      expect(arcs.length).toBeGreaterThan(0);
      expect(arcs.every(([, x, y, r]) => Number.isFinite(x) && Number.isFinite(y) && (r as number) >= 0)).toBe(true);
    }
  }
});

test("the Enter and zero to zero rows also sit in the bar menus", () => {
  expect(PAGE.keys.filter((row) => row.menu).map((row) => [row.act, row.menu])).toEqual([
    ["go", "Edit"],
    ["hop", "View"],
  ]);
});

test("a library thumb strokes the walk to its zero with a ring at the origin and a dot at the head, or the staircase with its curve and steps", () => {
  const thumb = (set: Setting) => {
    const log: Call[] = [];
    sketch(num, set, pen(log), 112, 112, 1, inked(null, "#008cff"));
    return log;
  };
  const walk = thumb({ view: "walk", t: plan.zeros[0] });
  expect(walk.filter(([name]) => name === "arc")).toHaveLength(2);
  expect(walk.filter(([name]) => name === "lineTo")).toHaveLength(index(plan.zeros[0]));
  const stairs = thumb({ view: "stairs", x: 100, zeros: 10 });
  expect(stairs.filter(([name]) => name === "stroke")).toHaveLength(3);
});
