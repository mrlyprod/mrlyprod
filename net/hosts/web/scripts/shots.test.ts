import { expect, test } from "bun:test";
import type { Frame } from "./shots.ts";
import { block, every, faults, screened } from "./shots.ts";

test("--every reads seconds or milliseconds into seconds and refuses anything else", () => {
  expect([every("2s"), every("0.5"), every("500ms"), every("3")]).toEqual([2, 0.5, 0.5, 3]);
  expect(() => every("2m")).toThrow();
});

test("--screen reads WxH with a dpr of 2 unless told, and site.json needs shots.routes and a screen", () => {
  expect([screened("1000x600"), screened("1440x900@1")]).toEqual([
    [1000, 600, 2],
    [1440, 900, 1],
  ]);
  expect(() => screened("1000")).toThrow();
  expect(block({ shots: { routes: ["/"] }, screen: [1440, 900, 2] })).toEqual({
    routes: ["/"],
    screen: [1440, 900, 2],
  });
  expect(() => block({ shots: { routes: ["/"], screen: [1440, 900, 2] } })).toThrow();
});

test("a run faults on FAILED, OVERFLOW and NOISE, and on DIFF or new only under --same", () => {
  const frame = (file: string, more: Partial<Frame>): Frame => ({
    file,
    route: "/",
    n: 0,
    t: 0,
    clock: "still",
    sha: "",
    verdict: "same",
    overflow: [],
    noise: [],
    ...more,
  });
  const frames = [
    frame("a.png", { verdict: "DIFF" }),
    frame("b.png", { overflow: ["content width=1500"] }),
    frame("c.png", { noise: ["error: x"], verdict: "new" }),
  ];
  expect(faults({ frames, failures: ["shots: d.png FAILED"] })).toEqual([
    "shots: d.png FAILED",
    "b.png OVERFLOW",
    "c.png NOISE",
  ]);
  expect(faults({ frames, failures: [] }, true)).toEqual(["a.png DIFF", "b.png OVERFLOW", "c.png NOISE", "c.png new"]);
  expect(faults({ frames: [frame("e.png", {})], failures: [] }, true)).toEqual([]);
});
