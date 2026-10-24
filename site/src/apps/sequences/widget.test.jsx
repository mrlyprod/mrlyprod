import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { pen } from "../../../test/pen.js";
import { values } from "../../lib/app/page.jsx";
import { CANON } from "../../lib/sequences/engine.js";
import { SPEC } from "./scene.js";
import { stamp, Widget } from "./widget.jsx";

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
  const squares = (cells) => {
    const log = [];
    stamp(cells)(pen(log), 56, 56);
    return log.filter(([key]) => key === "fillRect").length;
  };
  expect([squares(Array(9).fill(1)), squares(Array(27).fill(0)), squares([1, 0])]).toEqual([9, 27, 0]);
});
