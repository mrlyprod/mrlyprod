import { expect, test } from "bun:test";
import { block, every, screened } from "./shots.ts";

test("--every reads seconds or milliseconds and refuses anything else", () => {
  expect([every("2s"), every("0.5"), every("500ms"), every("3")]).toEqual([2000, 500, 500, 3000]);
  expect(() => every("2m")).toThrow();
});

test("--screen reads WxH with a dpr of 2 unless told, and a shots block needs a screen", () => {
  expect([screened("1000x600"), screened("1440x900@1")]).toEqual([
    [1000, 600, 2],
    [1440, 900, 1],
  ]);
  expect(() => screened("1000")).toThrow();
  expect(() => block({ shots: { routes: ["/"], sizes: {} } })).toThrow();
});
