import { beforeAll, expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import { run } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import type { Over, State } from "./engine.ts";
import { deal, UP } from "./engine.ts";
import type { Knobs } from "./scene.ts";
import { make, SAVERS, SPEC } from "./scene.ts";

beforeAll(() => {
  Reflect.deleteProperty(globalThis, "OffscreenCanvas");
});

const FRAME = 1000 / 60;

const STOPS = { addColorStop: () => {} };

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) => {
        log.push([key, ...args]);
        return STOPS;
      },
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const open = (seed: number, value: Raw, still = false, lock = false, w = 640, h = 480) => {
  const log: unknown[][] = [];
  const told: unknown[][] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff", ink: "#ffffff" }),
    still,
    lock,
    w,
    h,
    dpr: 1,
    t: 0,
  } as View;
  view.wake = () => {
    view.still = false;
    told.push(["wake"]);
  };
  const hooks = {
    onScore: (score: number) => told.push(["score", score]),
    onOver: (over: Over) => told.push(["over", over]),
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    ...value,
    ...hooks,
  });
  const tick = () => {
    log.length = 0;
    view.t += scene.pace;
    scene.draw();
  };
  const run = (ms: number) => {
    for (let t = 0; t < ms; t += FRAME) {
      log.length = 0;
      view.t += FRAME;
      scene.draw();
    }
  };
  return { log, told, scene, tick, run, view };
};

test("ticks step the game and a game over restarts after the hold", () => {
  const live = open(7, { play: "silly", size: 8, wrap: 0, apples: 10 });
  expect(live.told).toEqual([
    ["score", 0],
    ["over", null],
  ]);
  let ticks = 0;
  while (!live.told.some(([name, over]) => name === "over" && over) && ticks < 500) {
    live.tick();
    ticks++;
  }
  expect(live.told.filter(([name]) => name === "score").length).toBeGreaterThan(2);
  expect(live.told.at(-1)).toEqual(["over", expect.stringMatching(/^(wall|bite)$/)]);
  live.told.length = 0;
  for (let t = live.scene.pace; t < 1000; t += live.scene.pace) live.tick();
  expect(live.told).toEqual([]);
  live.tick();
  expect(live.told).toEqual([
    ["score", 0],
    ["over", null],
  ]);
});

test("one seed and one set of knobs give one board after 20 ticks", () => {
  const board = (seed: number) => {
    const live = open(seed, { play: "smart", size: 12, apples: 3 });
    for (let i = 0; i < 20; i++) live.tick();
    return live.log;
  };
  expect(board(7)).toEqual(board(7));
  expect(board(7)).not.toEqual(board(8));
});

test("under reduced motion me holds the starting board until the first turn wakes the run", () => {
  const live = open(7, { play: "me", player: true }, true);
  const hold = () => {
    live.log.length = 0;
    live.scene.draw();
  };
  hold();
  const board = [...live.log];
  hold();
  expect([live.log, live.scene.state().snake]).toEqual([board, deal(tidy(SPEC, {}) as Knobs, { rand: rng(7) }).snake]);
  live.scene.turn(UP);
  live.tick();
  expect(live.told.at(-1)).toEqual(["wake"]);
  expect(live.log).not.toEqual(board);
});

test("the scene boots the state deal deals from the value and the first draw of the page", () => {
  for (const value of [{}, { size: 10, wrap: 0, apples: 4 }, { size: 32, apples: 10 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { rand: rng(seed) }));
});

test("every colour handed to the pen is finite, at the start and after world seeks, in light, dark and the lock", () => {
  const bad: unknown[] = [];
  const check = (value: unknown) => {
    if (typeof value === "string" && /NaN|Infinity|undefined/.test(value)) bad.push(value);
  };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).endsWith("Gradient")
        ? () => ({ addColorStop: (_at: number, color: string) => check(color) })
        : () => {},
    set: (_, _key, value) => {
      check(value);
      return true;
    },
  });
  for (const paper of ["#000000", "#ffffff"])
    for (const [value, lock] of [
      [{}, false],
      [{ play: "silly", size: 8, apples: 6 }, false],
      [SAVERS[0].pick(rng(2)), true],
      [SAVERS[1].pick(rng(2)), true],
    ] as [Raw, boolean][]) {
      const view = {
        rand: rng(1),
        look: () => ({ paper, accent: "#6768fa", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
        still: false,
        lock,
        w: 800,
        h: 600,
        dpr: 2,
        t: 0,
      } as unknown as View;
      const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => ctx }), view, {
        ...defaults(SPEC),
        seed: 0,
        ...value,
      });
      scene.draw();
      for (const until of [500, 3000, 12000]) {
        while (view.t < until) {
          view.t += FRAME;
          scene.draw();
        }
      }
    }
  expect(bad).toEqual([]);
});

test("a stage smaller than its margins never hands the pen a negative radius", () => {
  for (const [w, h] of [
    [20, 20],
    [60, 40],
  ]) {
    const live = open(3, { play: "smart", size: 8, apples: 4 }, false, false, w, h);
    live.run(3000);
    live.scene.size();
    const bent = live.log.filter(
      ([key, , , ...r]) =>
        (key === "arc" && Number(r[0]) < 0) ||
        (key === "ellipse" && (Number(r[0]) < 0 || Number(r[1]) < 0)) ||
        (key === "roundRect" && [r[2]].flat().some((v) => Number(v) < 0)),
    );
    expect([w, h, bent]).toEqual([w, h, []]);
  }
});

test("the scales are drawn once per move and the floor once, not once per frame", () => {
  const sheets: unknown[][][] = [];
  const was = (globalThis as { OffscreenCanvas?: unknown }).OffscreenCanvas;
  class Sheet {
    width: number;
    height: number;
    log: unknown[][] = [];
    constructor(w: number, h: number) {
      this.width = w;
      this.height = h;
      sheets.push(this.log);
    }
    getContext() {
      return pen(this.log);
    }
  }
  Object.assign(globalThis, { OffscreenCanvas: Sheet });
  try {
    const live = open(5, { play: "smart", size: 12, wrap: 0, tick: 200 });
    live.run(4000);
    const drawn = sheets.map((log) => log.filter(([key]) => key === "clearRect").length);
    expect([sheets.length, drawn[0], drawn[1] > 0, drawn[1] <= 4000 / 200 + 1]).toEqual([2, 1, true, true]);
  } finally {
    Object.assign(globalThis, { OffscreenCanvas: was });
  }
});

test("time steps the game once per tick however finely the world is sought", () => {
  const live = open(7, { play: "smart", size: 16, tick: 200 });
  const before = live.scene.state().snake[0];
  live.run(1000);
  const fine = live.scene.state();
  const coarse = open(7, { play: "smart", size: 16, tick: 200 });
  for (let i = 0; i < 5; i++) coarse.tick();
  expect([fine.snake, fine.snake[0] !== before]).toEqual([coarse.scene.state().snake, true]);
});

test("the runner's world seek steps the game as wall-clock play does, under reduced motion too", () => {
  const frames: FrameRequestCallback[] = [];
  Object.assign(globalThis, {
    window: new EventTarget(),
    document: Object.assign(new EventTarget(), { hidden: false }),
    requestAnimationFrame: (fn: FrameRequestCallback) => frames.push(fn),
    cancelAnimationFrame: () => {},
  });
  const value = { ...defaults(SPEC), play: "smart", size: 12, wrap: 0, apples: 6, seed: 3 };
  const walk = (still: boolean, drive: (stop: ReturnType<typeof run>) => void) => {
    const canvas = Object.assign({} as HTMLCanvasElement, {
      clientWidth: 400,
      clientHeight: 400,
      getContext: () => pen([]),
    });
    const stop = run(canvas, (c, view, opts) => make(c, view, { ...opts, player: true }), value, { still });
    drive(stop);
    const state = (stop.scene as unknown as { state: () => State }).state();
    stop();
    return state;
  };
  try {
    const sought = walk(true, (stop) => {
      stop.seek(0);
      stop.seek(4000);
    });
    const played = walk(false, () => {
      for (let now = 1000; now <= 5000 + 1e-9; now += 1000 / 60) frames.shift()!(now);
    });
    const live = walk(false, (stop) => stop.seek(4000));
    expect([sought.score > 0, sought.snake.length > 2]).toEqual([true, true]);
    expect(sought).toEqual(live);
    expect(played.snake[0]).not.toBe(deal(tidy(SPEC, value) as Knobs, { rand: rng(3) }).snake[0]);
  } finally {
    for (const name of ["window", "document", "requestAnimationFrame", "cancelAnimationFrame"])
      Reflect.deleteProperty(globalThis, name);
  }
});
