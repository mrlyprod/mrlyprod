import { expect, test } from "bun:test";
import type { Rng } from "../rand.ts";
import { rng } from "../rand.ts";
import type { Opts, View } from "./run.ts";
import { board, run } from "./run.ts";

type Fonts = { load: (font: string) => Promise<unknown> };

const take = (rand: Rng, n: number) => Array.from({ length: n }, () => rand());

const box = (clientWidth = 4, clientHeight = 4) =>
  Object.assign({} as HTMLCanvasElement, { clientWidth, clientHeight });

test("a board is whole device pixels a cell, centred in the view", () => {
  expect(board({ w: 1001, h: 600 }, 16, 16, 0.92)).toEqual({ x: 228, y: 28, w: 544, h: 544, cell: 34 });
  expect(board({ w: 1000, h: 300 }, 40, 10)).toEqual({ x: 0, y: 25, w: 1000, h: 250, cell: 25 });
  expect(board({ w: 10, h: 10 }, 64, 64).cell).toBe(1);
});

const stage = (fonts?: Fonts) => {
  const frames: FrameRequestCallback[] = [];
  Object.assign(globalThis, {
    window: new EventTarget(),
    document: Object.assign(new EventTarget(), { hidden: false, fonts }),
    requestAnimationFrame: (fn: FrameRequestCallback) => frames.push(fn),
    cancelAnimationFrame: () => {},
  });
  return frames;
};

const unstage = () => {
  for (const name of ["window", "document", "requestAnimationFrame", "cancelAnimationFrame"])
    Reflect.deleteProperty(globalThis, name);
};

test("a scene takes its time from the frame loop, capped per frame", () => {
  const frames = stage();
  const seen: number[][] = [];
  const stop = run(box(), (_canvas, view) => ({ draw: () => seen.push([view.t, view.rand()]) }), { seed: 7 });
  for (const now of [1000, 1016, 1500]) frames.shift()!(now);
  stop();
  unstage();
  expect(seen).toEqual([0, 16, 116].map((t, n) => [t, take(rng(7), 3)[n]]));
});

test("pause freezes play time and draws nothing; play resumes from the frozen time", () => {
  stage();
  const queue = new Map<number, FrameRequestCallback>();
  let id = 0;
  Object.assign(globalThis, {
    requestAnimationFrame: (fn: FrameRequestCallback) => {
      queue.set(++id, fn);
      return id;
    },
    cancelAnimationFrame: (n: number) => queue.delete(n),
  });
  const next = (now: number) => {
    const [[n, fn]] = queue;
    queue.delete(n);
    fn(now);
  };
  const seen: number[] = [];
  const stop = run(box(), (_canvas, view) => ({ draw: () => seen.push(view.t) }));
  next(1000);
  next(1016);
  stop.pause();
  const waiting = queue.size;
  stop.play();
  next(9000);
  next(9032);
  stop();
  unstage();
  expect([seen, waiting]).toEqual([[0, 16, 16, 48], 0]);
});

test("the stop from run carries what make returned", () => {
  stage();
  const stop = run(box(), () => ({ draw: () => {}, trigger: () => "fired" }));
  stop();
  unstage();
  expect(stop.scene.trigger()).toBe("fired");
});

test("set reseeds the rand when the seed changes, hands the scene the value, draws a still scene once, and is false for a scene without set", () => {
  stage();
  Object.assign(globalThis, {
    matchMedia: () => ({ matches: true, addEventListener: () => {}, removeEventListener: () => {} }),
  });
  const seen: Opts[] = [];
  const drawn: number[] = [];
  const stop = run(
    box(),
    (_canvas, view) => ({ draw: () => drawn.push(view.rand()), set: (next) => seen.push(next) }),
    { seed: 7 },
  );
  const set = [stop.set({ seed: 8 }), stop.set({ seed: 8, tint: "red" })];
  stop();
  const plain = run(box(), () => ({ draw: () => {} }), { seed: 7 });
  const none = plain.set({ seed: 8 });
  plain();
  Reflect.deleteProperty(globalThis, "matchMedia");
  unstage();
  expect([set, none, seen]).toEqual([[true, true], false, [{ seed: 8 }, { seed: 8, tint: "red" }]]);
  expect(drawn).toEqual([rng(7)(), ...take(rng(8), 2)]);
});

test("a scene that names a font draws nothing until the font loads", async () => {
  let loaded: ((value: unknown) => void) | undefined;
  const asked: string[] = [];
  const frames = stage({
    load: (font) => {
      asked.push(font);
      return new Promise((r) => (loaded = r));
    },
  });
  const stop = run(box(), () => ({ font: "MrlyFont", draw: () => {} }));
  const early = frames.length;
  loaded!(undefined);
  await Promise.resolve();
  const late = frames.length;
  stop();
  unstage();
  expect([asked, early, late]).toEqual([['1em "MrlyFont"'], 0, 1]);
});

test("under a step every drawn frame adds exactly the step, one per step of wall time and never two in one frame", () => {
  const frames = stage();
  const seen: number[] = [];
  const stop = run(box(), (_canvas, view) => ({ draw: () => seen.push(view.t) }), {
    step: 40,
  });
  for (const now of [1000, 1016, 1032, 1048, 1100, 1200, 1201, 1202, 1203]) frames.shift()!(now);
  stop();
  unstage();
  expect(seen).toEqual([40, 80, 120, 160, 200, 240]);
});

test("a debt over four steps resets and counts the frames lost as late, which after hears with t after each draw", () => {
  const frames = stage();
  const heard: number[][] = [];
  const stop = run(box(), () => ({ draw: () => {} }));
  stop.fix({ step: 40, after: ({ t, late }) => heard.push([t, late]) });
  for (const now of [1000, 1500, 1540]) frames.shift()!(now);
  const late = stop.late;
  stop();
  unstage();
  expect([heard, late]).toEqual([
    [
      [40, 0],
      [80, 11],
      [120, 11],
    ],
    11,
  ]);
});

test("a fixed frame sets the store, a dpr of its short side over 540 and view.fixed, ignores resizes, and fix(null) returns to the box; a frame with its own dpr is a still, not a take", () => {
  stage();
  let resize: (() => void) | undefined;
  Object.assign(globalThis, {
    ResizeObserver: class {
      constructor(fn: () => void) {
        resize = fn;
      }
      observe() {}
      disconnect() {}
    },
  });
  const canvas = box();
  const seen: unknown[][] = [];
  const look = (view: View) => seen.push([view.w, view.h, view.dpr, view.fixed, canvas.width]);
  const stop = run(
    canvas,
    (_c, view) => {
      look(view);
      return { draw: () => {}, size: () => look(view) };
    },
    { frame: [1080, 1920] },
  );
  canvas.clientWidth = 8;
  resize!();
  stop.fix(null);
  stop.fix({ frame: [1280, 720] });
  stop.fix({ frame: [16, 8], dpr: 2 });
  stop();
  Reflect.deleteProperty(globalThis, "ResizeObserver");
  unstage();
  expect(seen).toEqual([
    [1080, 1920, 2, true, 1080],
    [8, 4, 1, false, 8],
    [1280, 720, 720 / 540, true, 1280],
    [16, 8, 2, false, 16],
  ]);
});

test("an every scene draws once at start, then at its interval, and ignores the step", () => {
  stage();
  const ticks: [() => void, number][] = [];
  const real = { setInterval: globalThis.setInterval, clearInterval: globalThis.clearInterval };
  Object.assign(globalThis, {
    setInterval: (fn: () => void, ms: number) => ticks.push([fn, ms]),
    clearInterval: () => {},
  });
  const seen: number[] = [];
  const stop = run(box(), (_canvas, view) => ({ every: 50, draw: () => seen.push(view.t) }), { step: 40 });
  ticks[0][0]();
  ticks[0][0]();
  stop();
  Object.assign(globalThis, real);
  unstage();
  expect([ticks[0][1], seen]).toEqual([50, [0, 50, 100]]);
});

test("the site's Reduce motion stills a scene without the OS's", () => {
  const frames = stage();
  Object.assign(document, { documentElement: { dataset: { motion: "reduce" } } });
  const seen: boolean[] = [];
  const stop = run(box(), (_canvas, view) => ({ draw: () => seen.push(view.still) }));
  stop();
  unstage();
  expect([frames.length, seen]).toEqual([0, [true]]);
});

test("a take with a step wakes a reduced-motion scene, which then draws one step a frame", () => {
  const frames = stage();
  Object.assign(globalThis, {
    matchMedia: () => ({ matches: true, addEventListener: () => {}, removeEventListener: () => {} }),
  });
  const seen: [number, boolean][] = [];
  const stop = run(box(), (_canvas, view) => ({
    draw: () => seen.push([view.t, view.still]),
  }));
  const idle = frames.length;
  stop.fix({ step: 40, after: () => {} });
  for (const now of [1000, 1040]) frames.shift()!(now);
  stop();
  Reflect.deleteProperty(globalThis, "matchMedia");
  unstage();
  expect([idle, seen]).toEqual([
    0,
    [
      [0, true],
      [40, false],
      [80, false],
    ],
  ]);
});

test("fix(null) after a take puts a reduced-motion scene back to stillness, drawn once with no frame loop", () => {
  stage();
  const queue = new Map<number, FrameRequestCallback>();
  let id = 0;
  Object.assign(globalThis, {
    requestAnimationFrame: (fn: FrameRequestCallback) => {
      queue.set(++id, fn);
      return id;
    },
    cancelAnimationFrame: (n: number) => queue.delete(n),
    matchMedia: () => ({ matches: true, addEventListener: () => {}, removeEventListener: () => {} }),
  });
  const seen: [number, boolean][] = [];
  const stop = run(box(), (_canvas, view) => ({
    draw: () => seen.push([view.t, view.still]),
  }));
  stop.fix({ step: 40, after: () => {} });
  const [[n, fn]] = queue;
  queue.delete(n);
  fn(1000);
  stop.fix(null);
  const waiting = queue.size;
  stop();
  Reflect.deleteProperty(globalThis, "matchMedia");
  unstage();
  expect([seen, waiting]).toEqual([
    [
      [0, true],
      [40, false],
      [40, true],
    ],
    0,
  ]);
});

test("a still scene draws on make, set, size and theme and never asks for a frame; a page can ask the same through run", () => {
  const frames = stage();
  let resize: (() => void) | undefined;
  Object.assign(globalThis, {
    ResizeObserver: class {
      constructor(fn: () => void) {
        resize = fn;
      }
      observe() {}
      disconnect() {}
    },
  });
  const canvas = box();
  const drawn: string[] = [];
  const stop = run(canvas, () => ({ still: true, draw: () => drawn.push("draw"), set: () => drawn.push("set") }));
  stop.set({ seed: 2 });
  canvas.clientWidth = 8;
  resize!();
  window.dispatchEvent(new Event("theme"));
  stop();
  const page = run(canvas, (_c, view) => ({ draw: () => drawn.push(`page ${view.still}`) }), {}, { still: true });
  page();
  Reflect.deleteProperty(globalThis, "ResizeObserver");
  unstage();
  expect([drawn, frames.length]).toEqual([["draw", "set", "draw", "draw", "draw", "page true"], 0]);
});

test("a scene whose set answers false makes set false, so the page remounts it", () => {
  stage();
  const stop = run(box(), () => ({ draw: () => {}, set: (next) => (next.seed === 9 ? false : undefined) }), {
    seed: 1,
  });
  const answers = [stop.set({ seed: 2 }), stop.set({ seed: 9 })];
  stop();
  unstage();
  expect(answers).toEqual([true, false]);
});
