import { expect, test } from "bun:test";
import * as life from "@mrlyprod/life";
import * as math from "@mrlyprod/math";
import { codeOfToken, fixed, knobsOf, modes, nameOf, recipeOf, said, thumb, tokenOf, tokens } from "./recipe.ts";

const pen = (log: unknown[][]) =>
  new Proxy({ canvas: {} } as CanvasRenderingContext2D, {
    get: (_, key) =>
      key === "canvas"
        ? {}
        : (...args: unknown[]) => {
            log.push([key, ...args]);
          },
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

test("a chip code names its grammar token, the code word first and the design name after, and a token names its code", () => {
  const made = tokens(life, math);
  expect([tokenOf(made, 7), tokenOf(made, 14), tokenOf(made, 1), tokenOf(made, 11)]).toEqual([
    "code7",
    "net",
    "code1",
    "code11",
  ]);
  expect([
    codeOfToken(made, "code7"),
    codeOfToken(made, "carpet"),
    codeOfToken(made, "net"),
    codeOfToken(made, "star"),
    codeOfToken(made, "code11"),
  ]).toEqual([7, 7, 14, 6, 11]);
});

test("the modes are the board and the grammar's, and a recipe is read from the knobs and back", () => {
  expect(modes(life)).toEqual([
    ["board", "Board"],
    ["island", "Island"],
    ["wallpaper", "Wallpaper"],
  ]);
  expect([fixed({}), fixed({ mode: "board" }), fixed({ mode: "island" })]).toEqual([true, true, false]);
  const key = "star.7.2 x5 island mask:copy birth:random_0_zeros_ones survive:fibonacci_ones x:256 cap:201";
  const patch = knobsOf(life, math, life.recipe.parse(key));
  expect(patch).toEqual({
    mode: "island",
    dim: 2,
    shape: "star",
    side: 7,
    depth: 2,
    tiling: 5,
    x: 256,
    cap: 201,
    mask: "copy",
    born: "random_0_zeros_ones",
    stay: "fibonacci_ones",
  });
  expect(nameOf(life, math, { ...patch, code: "11", number: 9, level: 2 })).toBe(key);
  expect(nameOf(life, math, { ...patch, mask: "design", code: "14", number: 9, level: 2 })).toBe(
    "star.7.2 x5 island mask:net.9.2 birth:random_0_zeros_ones survive:fibonacci_ones x:256 cap:201",
  );
  expect(nameOf(life, math, { ...patch, mode: "board" })).toBe("");
});

test("a side names a count list or a source, and an empty side is an error that names its knob", () => {
  const value = {
    mode: "wallpaper",
    shape: "code7",
    side: 3,
    depth: 1,
    mask: "design",
    code: "7",
    number: 3,
    level: 1,
    x: 8,
  };
  expect(recipeOf(life, math, { ...value, born: "3", stay: "2 3" })).toMatchObject({
    birth: "3",
    survive: "2,3",
    tiling: 1,
    cap: 0,
  });
  expect(() => recipeOf(life, math, { ...value, born: "3", stay: "-" })).toThrow("stay names no count and no source");
  expect(() => recipeOf(life, math, { ...value, mode: "board", born: "3", stay: "23" })).toThrow(
    "board is no recipe mode",
  );
});

test("a stop is told by its kind and generation, with the period, the shift and the wall of a mover", () => {
  const at = (kind: string, generation: number, period: number, dr = 0, dc = 0, wall_by = 0) =>
    said({ kind, generation, period, dr, dc, wall_by });
  expect([at("dead", 2, 1), at("still", 8, 1), at("loop", 15, 3), at("wall", 74, 0), at("limit", 256, 0)]).toEqual([
    "dead after 2",
    "still after 8",
    "loop after 15, period 3",
    "wall after 74",
    "limit after 256",
  ]);
  expect(at("mover", 20, 4, 1, -1, 60)).toBe("mover after 20, period 4, by 1,-1, wall by 60");
});

test("a thumb runs the recipe to its stop or the thumb cap and draws the last frame, a bad name draws nothing", () => {
  const log: unknown[][] = [];
  thumb(pen(log), life, "hline.3.1 x1 island mask:carpet.3.1 birth:3 survive:2,3 x:64 cap:31", 40, "#fff", "#000");
  expect(log.filter(([verb]) => verb === "rect").length).toBe(1);
  log.length = 0;
  thumb(pen(log), life, "nothing", 40, "#fff", "#000");
  expect(log).toEqual([["clearRect", 0, 0, 40, 40]]);
});
