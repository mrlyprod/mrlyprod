import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import { CODES, rows, sample } from "./widget.tsx";

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, v) => {
      log.push([key, v]);
      return true;
    },
  });

test("the readouts count moves against par, say mended or whole at the end, and name the level", () => {
  const told = { moves: 3, par: 7, over: null, level: 2, of: 3 };
  expect(rows(told)).toEqual([
    ["moves", 3],
    ["par", 7],
    ["level", "2 of 3"],
  ]);
  expect([rows({ ...told, over: "mended" })[0][0], rows({ ...told, over: "mended", par: 0 })[0][0]]).toEqual([
    "mended in",
    "whole",
  ]);
});

test("a library thumb fills one rect per filled cell of its fractal in one path", () => {
  const log: unknown[][] = [];
  sample(pen(log), math, CODES[0], 90, "#008cff");
  expect([log.filter(([verb]) => verb === "rect").length, log.filter(([verb]) => verb === "fill").length]).toEqual([
    216, 1,
  ]);
});
