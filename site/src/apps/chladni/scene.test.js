import { afterAll, expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import * as num from "@mrlyprod/mrly/num";
import { defaults, maxOf, tidy } from "../../lib/app/spec.js";
import { rng } from "../../lib/rand.js";
import {
  COUNT,
  carpets,
  cell,
  codes,
  HOLD,
  highest,
  hits,
  inks,
  ladder,
  lowest,
  NUMBERS,
  POUR,
  RATE,
  REST,
  rungOf,
  SIDE,
  swatch,
  TAP,
} from "./engine.js";
import { fit, make, PAGE, SPEC } from "./scene.js";
import { rows } from "./widget.jsx";

const pen = (log) =>
  new Proxy(
    {},
    {
      get:
        (_, key) =>
        (...args) =>
          log.push([key, ...args.map((arg) => (arg && typeof arg === "object" ? "page" : arg))]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const pages = [];

const kept = { OffscreenCanvas: globalThis.OffscreenCanvas, ImageData: globalThis.ImageData };

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.bytes = null;
    pages.push(this);
  }
  getContext() {
    return { putImageData: (image) => (this.bytes = Uint8ClampedArray.from(image.data)) };
  }
};

globalThis.ImageData = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.data = new Uint8ClampedArray(w * h * 4);
  }
};

afterAll(() => Object.assign(globalThis, kept));

const G = 10000;

const VALUE = { ...defaults(SPEC), seed: 7, grains: G };

const LOOK = { paper: "#000000", accent: "#008cff", ink: "#ffffff" };

const open = (value = {}, { still = false, w = 800, h = 600, dpr = 1, units = { num, math } } = {}) => {
  const log = [];
  const tells = [];
  const walks = [];
  const view = { rand: rng(7), look: () => LOOK, still, lock: false, w, h, dpr, t: 0, fixed: false, woke: 0 };
  view.wake = () => {
    view.woke++;
    view.still = false;
  };
  const mine = [];
  const grab = (fn) => {
    const from = pages.length;
    const out = fn();
    mine.push(...pages.slice(from));
    return out;
  };
  let now = { ...VALUE, ...value };
  const scene = grab(() =>
    make({ getContext: () => pen(log) }, view, {
      ...now,
      ...units,
      onTell: (stat) => tells.push(stat),
      onWalk: (patch) => walks.push(patch),
    }),
  );
  const at = (t) => {
    view.t = t;
    log.length = 0;
    grab(() => scene.draw());
    return log;
  };
  const set = (patch) => {
    now = { ...now, ...patch };
    log.length = 0;
    return grab(() => scene.set(now));
  };
  const resize = (w, h) => {
    Object.assign(view, { w, h });
    grab(() => scene.size());
    return at(view.t);
  };
  const sheet = () => mine.filter((one) => one.bytes && one.width > SIDE).at(-1)?.bytes;
  const cells = () => mine.filter((one) => one.bytes && one.width <= SIDE).at(-1)?.bytes;
  const sheets = () => mine.filter((one) => one.width > SIDE).length;
  return { view, scene, log, tells, walks, at, set, resize, sheet, cells, sheets };
};

const counted = () => {
  const tally = { steps: 0 };
  const shake = (field, side, grains, from, steps, seed) => {
    tally.steps += steps;
    return num.chladni.shake(field, side, grains, from, steps, seed);
  };
  return [tally, { num: { ...num, chladni: { ...num.chladni, shake } }, math }];
};

const blits = (log) => log.filter(([name]) => name === "drawImage");

const lit = (bytes) => {
  let n = 0;
  for (let i = 0; i < bytes.length; i += 4) if (bytes[i + 2] === 255) n++;
  return n;
};

const RUN = (REST / RATE) * 1000;

test("the plate opens on (3, 5, -) as sand, the plate and one sheet of css pixels blitted crisp into the fitted square", () => {
  const live = open();
  const log = live.at(0);
  const box = fit(live.view, 0);
  const blit = ["drawImage", "page", box.x, box.y, box.side, box.side];
  expect(blits(log)).toEqual([blit, blit]);
  expect(log).toContainEqual(["imageSmoothingEnabled", false]);
  expect(log.some(([name]) => ["stroke", "fillText", "arc"].includes(name))).toBe(false);
  expect(lit(live.sheet())).toBeGreaterThan(1000);
  expect(live.tells.at(-1)).toMatchObject({ view: "plate", n: 3, m: 5, sign: -1, grains: G });
  expect(live.tells.at(-1).pitch).toBeCloseTo(Math.hypot(3, 5), 9);
});

test("the opening is the rested still and waits for a hand: no later draw paints", () => {
  const live = open();
  live.at(0);
  expect([live.at(1000), live.at(60000)]).toEqual([[], []]);
});

test("a rested epoch equals the still: a Reroll to the seed run to rest paints what the page opens on", () => {
  const still = open();
  still.at(0);
  const rolled = open({ seed: 1 });
  rolled.at(0);
  rolled.set({ seed: 7 });
  rolled.at(RUN + 500);
  expect(rolled.sheet()).toEqual(still.sheet());
});

test("a run drawn at t in one jump equals one stepped by 16, 33 and 100 ms frames", () => {
  const end = RUN / 2;
  const jumped = open();
  jumped.at(0);
  const rested = Uint8ClampedArray.from(jumped.sheet());
  jumped.set({ seed: 9 });
  jumped.at(end);
  expect(jumped.sheet()).not.toEqual(rested);
  for (const dt of [16, 33, 100]) {
    const stepped = open();
    stepped.at(0);
    stepped.set({ seed: 9 });
    for (let t = dt; t < end; t += dt) stepped.at(t);
    stepped.at(end);
    expect([dt, stepped.sheet()]).toEqual([dt, jumped.sheet()]);
  }
});

test("a tap sprinkles grains under the finger and the sand runs again to rest", () => {
  const live = open();
  live.at(0);
  live.scene.touch(0.5, 0.5, true);
  expect(live.tells.at(-1).grains).toBe(G + TAP);
  expect(live.at(1000).length).toBeGreaterThan(0);
  live.at(1000 + RUN);
  expect(live.at(2000 + RUN)).toEqual([]);
});

test("a drag pours by the distance it covers in css pixels", () => {
  const live = open();
  live.at(0);
  live.scene.touch(0.3, 0.5, true);
  live.scene.touch(0.4, 0.5, false);
  expect(live.tells.at(-1).grains).toBe(G + TAP + Math.floor(0.1 * 800 * POUR));
});

test("a figure change keeps the sand where it lies", () => {
  const live = open();
  live.at(0);
  const before = Uint8ClampedArray.from(live.sheet());
  live.set({ n: 4 });
  live.at(0);
  expect(live.sheet()).toEqual(before);
  expect(live.tells.at(-1)).toMatchObject({ n: 4, m: 5, grains: G });
});

test("a resize swaps the sand sheet for one of the new size: the scene keeps one sheet, never one a size", () => {
  const live = open();
  live.at(0);
  live.resize(700, 500);
  live.resize(800, 600);
  expect(live.sheets()).toBe(3);
});

test("Play walks the ladder from the figure on show, one figure every hold, and the echo of a pushed figure does not re-base", () => {
  const live = open();
  live.at(0);
  const rungs = ladder(num);
  const from = rungOf(num, VALUE);
  expect(live.scene.walk()).toBe(true);
  expect(live.walks).toEqual([rungs[from + 1]].map(({ n, m, sign }) => ({ n, m, sign })));
  live.at(1000);
  live.set(live.walks[0]);
  live.at(HOLD - 1);
  expect(live.walks).toHaveLength(1);
  live.at(HOLD);
  expect(live.walks.at(-1)).toEqual((({ n, m, sign }) => ({ n, m, sign }))(rungs[from + 2]));
});

test("a slider move while walking re-bases the walk on the new figure", () => {
  const live = open();
  live.at(0);
  live.scene.walk();
  live.at(1000);
  live.set({ n: 0, m: 1, sign: 1 });
  live.at(HOLD);
  expect(live.walks).toHaveLength(1);
  live.at(1000 + HOLD);
  expect(live.walks.at(-1)).toEqual({ n: 0, m: 1, sign: -1 });
  expect(live.scene.walking()).toBe(true);
});

test("Enter is Play or Stop by the walk in the Edit menu, and the scene leaves Space to the lib pause", () => {
  const [row] = PAGE.keys;
  expect([
    row.key,
    row.menu,
    row.button,
    row.label(VALUE, { walking: () => false }),
    row.label(VALUE, { walking: () => true }),
    row.label(VALUE, null),
  ]).toEqual(["Enter", "Edit", true, "Play", "Stop", "Play"]);
  const live = open();
  expect([live.scene.toggle, live.scene.playing]).toEqual([undefined, undefined]);
  live.scene.walk();
  expect(live.scene.walk()).toBe(false);
  expect(live.scene.walking()).toBe(false);
});

test("under reduced motion a drag only lays sand, and the lift rests it once", () => {
  const [tally, units] = counted();
  const calm = open({}, { still: true, units });
  calm.at(0);
  tally.steps = 0;
  calm.scene.touch(0.3, 0.5, true);
  for (const u of [0.35, 0.4, 0.45, 0.5]) calm.scene.touch(u, 0.5, false);
  expect([tally.steps, calm.tells.at(-1).grains > G + TAP]).toEqual([0, true]);
  calm.scene.lift();
  expect(tally.steps).toBe(REST);
});

test("under reduced motion Stop returns to the still: a tap after the walk rests at once", () => {
  const [tally, units] = counted();
  const calm = open({}, { still: true, units });
  calm.at(0);
  calm.scene.walk();
  calm.at(HOLD);
  calm.scene.walk();
  calm.at(HOLD + 16);
  tally.steps = 0;
  calm.scene.touch(0.5, 0.5, true);
  calm.scene.lift();
  expect([calm.view.woke, tally.steps]).toEqual([1, REST]);
});

test("under reduced motion a Reroll shows the rested sand at once", () => {
  const calm = open({}, { still: true });
  calm.at(0);
  calm.set({ seed: 9 });
  calm.at(0);
  const still = open({ seed: 9 });
  still.at(0);
  expect(calm.sheet()).toEqual(still.sheet());
});

test("a look away from Sand freezes the sand, and Sand runs on from where it stood", () => {
  const live = open();
  live.at(0);
  live.set({ seed: 9 });
  live.at(500);
  const stood = Uint8ClampedArray.from(live.sheet());
  live.set({ look: "lines" });
  live.at(5000);
  live.set({ look: "sand" });
  live.at(5000);
  expect(live.sheet()).toEqual(stood);
  const solo = open();
  solo.at(0);
  solo.set({ seed: 9 });
  solo.at(1000);
  live.at(5500);
  expect(live.sheet()).toEqual(solo.sheet());
});

test("Field paints the signed amplitude smooth: the accent at the top, the opposite hue at the bottom", () => {
  const live = open({ look: "field" });
  const log = live.at(0);
  expect(log).toContainEqual(["imageSmoothingEnabled", true]);
  const field = num.chladni.plate(3, 5, -1, SIDE);
  const top = field.indexOf(Math.max(...field));
  const low = field.indexOf(Math.min(...field));
  const page = live.cells();
  expect([[...page.slice(4 * top, 4 * top + 4)], [...page.slice(4 * low, 4 * low + 4)]]).toEqual([
    [0, 140, 255, 255],
    [255, 115, 0, 255],
  ]);
});

test("Lines strokes the zero set once in the ink, one segment a door segment", () => {
  const live = open({ look: "lines" });
  const log = live.at(0);
  const segments = num.chladni.lines(num.chladni.plate(3, 5, -1, SIDE), SIDE).length / 4;
  expect([log.filter(([name]) => name === "stroke").length, log.filter(([name]) => name === "lineTo").length]).toEqual([
    1,
    segments,
  ]);
  expect(log).toContainEqual(["strokeStyle", "#ffffff"]);
});

test("the sign hides when n equals m, the grains outside Sand, and no row is named modes", () => {
  const row = (key) => SPEC.find((one) => one.key === key);
  expect([
    row("sign").when({ view: "plate", n: 4, m: 4 }),
    row("sign").when({ view: "plate", n: 3, m: 5 }),
    row("grains").when({ look: "field" }),
  ]).toEqual([false, true, false]);
  expect(SPEC.map((one) => one.key)).not.toContain("modes");
  expect([SPEC[0].key, SPEC[0].def, SPEC[0].group]).toEqual(["view", "plate", "Figure"]);
});

test("an export is named by the figure and the seed", () => {
  expect([PAGE.name(VALUE), PAGE.name({ ...VALUE, n: 4, m: 4 }), PAGE.name({ ...VALUE, view: "carpet" })]).toEqual([
    "chladni-3-5m-7",
    "chladni-4-4-7",
    "chladni-7-3-3-k8-7",
  ]);
});

const UNITS = { num, math };

const carpetOf = (level, k = 8) => {
  const door = math.nodal.figures(7n, 3, level, COUNT);
  const size = door.side * door.side;
  return { side: door.side, data: door.fields.slice((k - 1) * size, k * size) };
};

const cellOf = (plate, px, x, y) => Math.floor((y * plate.side) / px) * plate.side + Math.floor((x * plate.side) / px);

const lined =
  ({ side, data }) =>
  (cell) => {
    const [r, c] = [Math.floor(cell / side), cell % side];
    return [
      [r - 1, c],
      [r + 1, c],
      [r, c - 1],
      [r, c + 1],
    ].some(([i, j]) => i >= 0 && j >= 0 && i < side && j < side && data[i * side + j] * data[cell] <= 0);
  };

const shares = (bytes, plate) => {
  const px = Math.sqrt(bytes.length / 4);
  const near = lined(plate);
  const tally = { sand: 0, sandNear: 0, face: 0, faceNear: 0 };
  for (let y = 0; y < px; y++) {
    for (let x = 0; x < px; x++) {
      const cell = cellOf(plate, px, x, y);
      if (Number.isNaN(plate.data[cell])) continue;
      const hit = near(cell) ? 1 : 0;
      tally.face++;
      tally.faceNear += hit;
      if (bytes[4 * (y * px + x) + 2] !== 255) continue;
      tally.sand++;
      tally.sandNear += hit;
    }
  }
  return { sand: tally.sandNear / tally.sand, uniform: tally.faceNear / tally.face };
};

test("the first Carpet draw at level 4 solves the design, rests the sand and paints within a budget", () => {
  const start = performance.now();
  const live = open({ view: "carpet", level: 4 });
  live.at(0);
  const took = performance.now() - start;
  expect(blits(live.log)).toHaveLength(2);
  expect(took).toBeLessThan(1500);
});

test("the carpet opens as a page of one pixel a cell, its holes the ground, blitted crisp into whole device pixels a cell under a sheet of grains alone", () => {
  const live = open({ view: "carpet" }, { dpr: 2, h: 620 });
  const log = live.at(0);
  const box = fit(live.view, 27);
  expect([box.side % 27, (box.side / 2) % 27 > 0]).toEqual([0, true]);
  const blit = ["drawImage", "page", box.x, box.y, box.side, box.side];
  expect(blits(log)).toEqual([blit, blit]);
  expect(log).toContainEqual(["imageSmoothingEnabled", false]);
  const cells = live.cells();
  const alpha = (row, col) => cells[4 * (row * 27 + col) + 3];
  expect([cells.length, alpha(13, 13), alpha(0, 0)]).toEqual([27 * 27 * 4, 0, 255]);
  const sheet = live.sheet();
  let stray = 0;
  for (let i = 0; i < sheet.length; i += 4) if (sheet[i + 3] && sheet[i + 2] !== 255) stray++;
  expect([lit(sheet) > 250, stray]).toEqual([true, 0]);
  expect(live.tells.at(-1)).toMatchObject({ view: "carpet", k: 8, count: COUNT });
});

test("the design picker offers only what holds accepts, and a number change lands on a design that holds", () => {
  const codesOf = (number) =>
    Array.from({ length: 16 }, (_, i) => String(i)).filter((code) => holding(number, code).length);
  const holding = (number, code) =>
    Array.from({ length: math.bang.level_cap(number, 2, 128 * 128) }, (_, i) => i + 1).filter((level) =>
      math.nodal.holds(BigInt(code), number, level),
    );
  for (const number of NUMBERS) {
    expect(codes({ number }, UNITS).map(([code]) => code)).toEqual(codesOf(number));
    for (const code of codesOf(number)) {
      const levels = holding(number, code);
      const [low, high] = [lowest({ number, code }, UNITS), highest({ number, code }, UNITS)];
      expect([low, high, levels.length]).toEqual([levels[0], levels.at(-1), high - low + 1]);
    }
    const value = tidy(SPEC, { ...VALUE, view: "carpet", number }, UNITS);
    expect([number, math.nodal.holds(BigInt(value.code), number, value.level)]).toEqual([number, true]);
  }
});

test("a rested Carpet epoch at levels 3 and 4 loses grains to the holes and lays the rest on the zero set", () => {
  for (const level of [3, 4]) {
    const live = open({ view: "carpet", level });
    live.at(0);
    const left = live.tells.at(-1).grains;
    const { sand, uniform } = shares(live.sheet(), carpetOf(level));
    expect([level, left > 0, left < G, sand > 0.9, sand > 2 * uniform]).toEqual([level, true, true, true, true]);
  }
});

test("a grain on a hole is gone: a tap over the centre hole adds none, and no grain lies on a hole", () => {
  const live = open({ view: "carpet" });
  live.at(0);
  const left = live.tells.at(-1).grains;
  live.scene.touch(0.5, 0.5, true);
  expect(live.scene.stat().grains).toBe(left);
  const plate = carpetOf(3);
  const bytes = live.sheet();
  const px = Math.sqrt(bytes.length / 4);
  let wet = 0;
  for (let i = 0; i < px * px; i++)
    if (bytes[4 * i + 2] === 255 && Number.isNaN(plate.data[cellOf(plate, px, i % px, Math.floor(i / px))])) wet++;
  expect(wet).toBe(0);
});

test("the Carpet status reads k, lambda, domains d of b and the grains left, every number from the door", () => {
  const live = open({ view: "carpet" });
  live.at(0);
  const door = math.nodal.figures(7n, 3, 3, COUNT);
  const stat = live.tells.at(-1);
  expect([door.domains[7], door.bound[7]]).toEqual([5, 8]);
  expect(rows(stat)).toEqual([
    ["k", `8 of ${COUNT}`],
    ["lambda", door.values[7].toFixed(6)],
    ["domains", "5 of 8"],
    ["grains left", String(stat.grains)],
  ]);
});

test("Play on Carpet walks k from the figure on show and wraps from the last figure to 2, past the constant figure", () => {
  const live = open({ view: "carpet", k: COUNT - 1 });
  live.at(0);
  live.scene.walk();
  live.at(HOLD);
  expect(live.walks).toEqual([{ k: COUNT }, { k: 2 }]);
});

test("a Carpet figure change tops the drained sand up to the preset before it runs", () => {
  const live = open({ view: "carpet" });
  live.at(0);
  expect(live.tells.at(-1).grains).toBeLessThan(G / 2);
  live.set({ k: 9 });
  expect([live.tells.at(-1).k, G - live.tells.at(-1).grains < 40]).toEqual([9, true]);
});

test("k past a small design reads its last figure, and the echo of the tidied k leaves the sand at rest", () => {
  const small = { view: "carpet", number: 5, code: "14", level: 1 };
  const live = open({ ...small, k: 40 });
  live.at(0);
  expect(live.tells.at(-1)).toMatchObject({ k: 16, count: 16 });
  expect(
    maxOf(
      SPEC.find((row) => row.key === "k"),
      { ...VALUE, ...small },
      UNITS,
    ),
  ).toBe(16);
  const rested = Uint8ClampedArray.from(live.sheet());
  live.set({ k: 16 });
  live.at(60000);
  expect(live.sheet()).toEqual(rested);
});

test("the design slot is the code of a base 2 plane design shown on a carpet alone, and its row hides in the pane while the url keeps it", () => {
  expect([
    PAGE.design.dim,
    PAGE.design.base,
    PAGE.design.code,
    PAGE.design.when({ view: "plate" }),
    PAGE.design.when({ view: "carpet" }),
  ]).toEqual([2, 2, "code", false, true]);
  const row = SPEC.find((one) => one.key === "code");
  expect([row.when({ view: "carpet" }), tidy(SPEC, { ...VALUE, view: "carpet", code: "10" }, UNITS).code]).toEqual([
    false,
    "10",
  ]);
});

test("the carpet shelf lists every design that holds, shown at a level that draws more than 25 cells, and its cell is the figure mask", () => {
  const list = carpets(math);
  expect(list.length).toBe(NUMBERS.reduce((sum, number) => sum + codes({ number }, UNITS).length, 0));
  expect(list.find((one) => one.number === 3 && one.code === "7")).toEqual({
    number: 3,
    code: "7",
    level: 3,
    name: "carpet",
  });
  expect(list.every((one) => math.nodal.holds(BigInt(one.code), one.number, one.level))).toBe(true);
  for (const one of list.filter((pick) => pick.number > 3)) expect(one.level).toBe(Math.min(2, highest(one, UNITS)));
  const one = list.find((pick) => pick.number === 3 && pick.code === "10");
  const figure = math.nodal.figures(BigInt(one.code), one.number, one.level, 2).fields.slice(0, 27 * 27);
  expect(Array.from(cell(math, one).types, (type) => type === 0)).toEqual(Array.from(figure, (v) => Number.isNaN(v)));
});

test("the plate shelf finds a figure by its numbers, and a swatch paints the field in the ink to a square of the asked side", () => {
  const one = ladder(num).find((rung) => rung.n === 3 && rung.m === 5 && rung.sign === -1);
  expect([hits(one, ""), hits(one, "3 5"), hits(one, "5,3"), hits(one, "3"), hits(one, "4 5")]).toEqual([
    true,
    true,
    true,
    true,
    false,
  ]);
  const bytes = swatch(num, one, inks(LOOK), 24);
  expect(bytes.length).toBe(24 * 24 * 4);
  expect(bytes.every((v, i) => i % 4 !== 3 || v === 255)).toBe(true);
  expect(bytes.some((v, i) => i % 4 === 2 && v === 255)).toBe(true);
});
