import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, EMPTY, front, frosty, STONE } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { FIT, make, SAVERS, SPEC } from "./scene.ts";

const FRAME = 16;

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).endsWith("Gradient")
        ? () => ({ addColorStop: () => {} })
        : (...args: unknown[]) => log.push([key, ...args]),
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const open = (seed: number, value: Raw, still = false, lock = false, paper = "#000000") => {
  const log: unknown[][] = [];
  const told: Told[] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper, accent: "#008cff", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
    still,
    lock,
    w: 640,
    h: 640,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    seed,
    ...value,
    onTell: (one: Told) => told.push(one),
  });
  const run = (ms: number, frame = FRAME) => {
    for (let t = 0; t < ms; t += frame) {
      log.length = 0;
      view.t += frame;
      scene.draw();
    }
  };
  return { log, told, scene, run, view };
};

const at = (size: number, p: number): [number, number] => {
  const cell = (640 * FIT) / size;
  const left = (640 - size * cell) / 2;
  return [(left + ((p % size) + 0.5) * cell) / 640, (left + (Math.floor(p / size) + 0.5) * cell) / 640];
};

test("smart plays the field turn by turn, saves flowers, ends it, then deals again", () => {
  const live = open(7, { play: "smart" });
  live.run(90000, 32);
  const end = live.told.findIndex((one) => one.over === "done");
  const again = live.told.slice(end).find((one) => one.over === null)!;
  expect([end > 3, live.told[end].alive > 0, live.told[end].stones > 0, again.turn]).toEqual([true, true, true, 0]);
});

test("one url deals one field, also after a knob goes and comes back", () => {
  const board = (seed: number) => open(seed, {}).scene.state().board;
  expect(board(7)).toBe(board(7));
  expect(board(7)).not.toBe(board(8));
  const live = open(7, {});
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, flowers: 9 });
  live.scene.set({ ...value });
  expect(live.scene.state().board).toBe(board(7));
});

const tip = (state: Parameters<typeof front>[0]) => front(state).find((p) => state.board[p] === EMPTY)!;

test("a tap on a needle's tip drops a stone and the frost grows, and a tap elsewhere does nothing", () => {
  const live = open(7, {}, true);
  const first = live.scene.state();
  const empty = tip(first);
  live.scene.tap(...at(15, empty));
  const after = live.scene.state();
  expect([after.board[empty], after.turn, after.stones]).toEqual([STONE, 1, 1]);
  for (const p of [[...after.board].findIndex(frosty), after.board.lastIndexOf(EMPTY)]) live.scene.tap(...at(15, p));
  expect(live.scene.state()).toBe(after);
});

test("the arrows walk the field, X drops a stone in the ring, and Enter's wait grows the frost alone", () => {
  const live = open(7, {}, true);
  const [x, y] = live.scene.ring();
  const target = tip(live.scene.state());
  for (let i = x; i < target % 15; i++) live.scene.move("ArrowRight");
  for (let i = x; i > target % 15; i--) live.scene.move("ArrowLeft");
  for (let i = y; i < Math.floor(target / 15); i++) live.scene.move("ArrowDown");
  for (let i = y; i > Math.floor(target / 15); i--) live.scene.move("ArrowUp");
  live.scene.press();
  expect(live.scene.state().board[target]).toBe(STONE);
  live.scene.wait();
  expect([live.scene.state().turn, live.scene.state().stones]).toEqual([2, 1]);
});

test("pause holds smart, and play lets it go on", () => {
  const live = open(7, { play: "smart" });
  live.scene.toggle();
  live.run(5000);
  expect([live.scene.playing(), live.scene.state().turn]).toEqual([false, 0]);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().turn).toBeGreaterThan(0);
});

test("under reduced motion a seek of the world clock still lets smart play, frame by frame", () => {
  const live = open(3, { play: "smart", size: 21, frost: 3, flowers: 8 }, true);
  const turns = [0, 1500, 3000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().turn;
  });
  expect(turns[0]).toBe(0);
  expect(turns[1]).toBeGreaterThan(turns[0]);
  expect(turns[2]).toBeGreaterThan(turns[1]);
});

test("the scene deals the state deal deals from the value and the seed", () => {
  for (const value of [{}, { size: 9, frost: 1, flowers: 1 }, { size: 25, frost: 4, flowers: 12 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed }));
});

test("the frost is drawn in a lighter ice on dark paper than on light", () => {
  const ice = (paper: string) => {
    const live = open(5, {}, true, false, paper);
    live.scene.tap(...at(15, tip(live.scene.state())));
    return live.log.filter(([key]) => key === "strokeStyle").map(([, value]) => String(value));
  };
  const light = (color: string) => (color.match(/\d+/g) ?? []).slice(0, 3).reduce((a, b) => a + Number(b), 0);
  expect(light(ice("#000000")[0])).toBeGreaterThan(light(ice("#ffffff")[0]));
});

test("the savers Garden and Winter boot in the lock from their picks and paint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["garden", "Garden"],
    ["winter", "Winter"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    expect([saver.id, live.scene.state().over, live.log.some(([key]) => key === "fill")]).toEqual([
      saver.id,
      null,
      true,
    ]);
  }
});

test("in the lock smart plays field after field", () => {
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(150000, 50);
    expect([saver.id, live.told.filter((one) => one.over === "done").length >= 2]).toEqual([saver.id, true]);
  }
});

test("the lock's exit: hold stops smart, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(3000);
  live.scene.hold();
  const turn = live.scene.state().turn;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().turn, landed - from <= 1000, still]).toEqual([turn, true, true]);
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
      [{ play: "smart" }, false],
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
        fixed: false,
        wake: () => {},
      } as unknown as View;
      const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => ctx }), view, {
        ...defaults(SPEC),
        seed: 0,
        ...value,
      });
      scene.draw();
      for (const until of [500, 3000, 12000]) {
        while (view.t < until) {
          view.t = Math.min(until, view.t + 1000 / 60);
          scene.draw();
        }
        scene.size();
      }
    }
  expect(bad).toEqual([]);
});
