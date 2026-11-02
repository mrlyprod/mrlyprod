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

test("the readouts score both players, then name who moves, who won or a draw", () => {
  const told = { points: [40, 32], turn: 2, over: null, moves: 5 };
  expect(rows(told)).toEqual([
    ["squares", 40],
    ["dots", 32],
    ["to move", "dots"],
  ]);
  expect([rows({ ...told, over: "one" })[2], rows({ ...told, over: "draw" })[2]]).toEqual([
    ["won", "squares"],
    ["result", "draw"],
  ]);
});

test("a library thumb fills one rect per filled cell of its fractal in one path", () => {
  const log: unknown[][] = [];
  sample(pen(log), math, CODES[0], 90, "#008cff");
  expect([log.filter(([verb]) => verb === "rect").length, log.filter(([verb]) => verb === "fill").length]).toEqual([
    216, 1,
  ]);
});
