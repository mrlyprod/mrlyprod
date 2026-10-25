import { expect, test } from "bun:test";
import type { Opts, View } from "../../lib/app/run.ts";
import { rng } from "../../lib/rand.ts";
import { BAND, CROSS, FIRST, SHAPES } from "./engine.ts";
import { fit, make, SAVERS } from "./scene.ts";

type Line = [string | symbol, ...unknown[]];

const pen = (log: Line[]) =>
  new Proxy<Record<string | symbol, unknown>>(
    {},
    {
      get: (target, key) =>
        key in target
          ? target[key]
          : (...args: unknown[]) => log.push([key, ...args.filter((one) => typeof one !== "object")]),
      set: (target, key, value: unknown) => {
        if (key === "fillStyle") log.push(["fillStyle", value]);
        target[key] = value;
        return true;
      },
    },
  );

function stage(opts: Opts = {}, extra: Partial<View> = {}) {
  const shown: Line[] = [];
  const drawn: Line[] = [];
  const cache = { width: 0, height: 0, getContext: () => pen(drawn) };
  const canvas = Object.assign({} as HTMLCanvasElement, {
    getContext: () => pen(shown),
    ownerDocument: { createElement: () => cache },
  });
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#101010", accent: "#008cff" }),
    still: false,
    w: 320,
    h: 200,
    dpr: 1,
    t: 0,
    ...extra,
  } as View;
  const scene = make(canvas, view, { seed: 7, ...opts });
  const at = (t: number) => {
    view.t = t;
    scene.draw();
  };
  return { shown, drawn, at };
}

const fills = (log: Line[]) => log.filter(([verb]) => verb === "fill").length;

test("every shape paints the mosaic once on the first frame and shows it", () => {
  for (const shape of SHAPES) {
    const { shown, drawn, at } = stage({ shape });
    at(0);
    expect([
      shape,
      shown.filter(([verb]) => verb === "drawImage").length,
      drawn.filter(([verb]) => verb === "fillRect").length,
      fills(drawn) > 0,
    ]).toEqual([shape, 1, 1, true]);
  }
});

test("one seed and t paint one picture, waves and all", () => {
  const [one, two] = [stage(), stage()];
  for (const { at } of [one, two]) for (const t of [0, FIRST / 2, FIRST + CROSS]) at(t);
  expect([one.drawn.length > 0, one.drawn, one.shown]).toEqual([true, two.drawn, two.shown]);
});

test("a still draw under the lock paints the mosaic as it settles after the first wave", () => {
  const moved = stage({}, { lock: true });
  moved.at(FIRST + CROSS + BAND);
  const held = stage({}, { lock: true, still: true });
  held.at(0);
  expect([held.drawn.length > 0, held.drawn, held.shown]).toEqual([true, moved.drawn, moved.shown]);
});

test("a frame before the first wave is not shown again", () => {
  const { shown, at } = stage();
  at(0);
  at(FIRST / 2);
  expect(shown.filter(([verb]) => verb === "drawImage").length).toBe(1);
});

test("the desk keeps twenty cells across its long side and the app holds one count of cells on any canvas", () => {
  expect([fit(1440, 900, true), fit(900, 1440, true)]).toEqual([72, 72]);
  const count = (w: number, h: number) => Math.round((w * h) / fit(w, h, false) ** 2);
  expect([count(832, 760), count(2560, 1440), count(390, 844)]).toEqual([600, 600, 600]);
});

test("each saver paints its own palette in the site tint", () => {
  expect(SAVERS.map((one) => [one.id, one.pick(rng(1)).palette, one.pick(rng(1)).tint])).toEqual([
    ["cubes", "soft", ""],
    ["honeycomb", "tint", ""],
    ["facets", "deep", ""],
  ]);
});
