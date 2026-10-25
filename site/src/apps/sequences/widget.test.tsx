import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import type { Stroke } from "../../../test/pen.ts";
import { pen } from "../../../test/pen.ts";
import { values } from "../../lib/app/page.tsx";
import { CANON } from "../../lib/sequences/engine.ts";
import { SPEC } from "./scene.ts";
import { stamp, Widget } from "./widget.tsx";

test("before the unit loads the widget renders no stage and claims no bar on the server", () => {
  const value = values({ seed: 0, q: "", pick: CANON }, SPEC);
  expect(Object.keys(value)).toEqual([
    "seed",
    "q",
    "pick",
    "chart",
    "terms",
    "digits",
    "dim",
    "base",
    "measure",
    "axis",
  ]);
  expect(renderToStaticMarkup(<Widget value={value} onChange={() => {}} onReady={() => {}} />)).toBe("");
});

test("a row thumb draws one square a cell, a plane in one slice and a cube in three, and nothing for a shape it does not know", () => {
  const squares = (cells: number[]) => {
    const log: Stroke[] = [];
    stamp(cells)(pen(log), 56, 56);
    return log.filter(([key]) => key === "fillRect").length;
  };
  expect([squares(Array<number>(9).fill(1)), squares(Array<number>(27).fill(0)), squares([1, 0])]).toEqual([9, 27, 0]);
});
