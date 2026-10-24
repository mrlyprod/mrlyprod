import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { Frame } from "../../lib/app/frame.jsx";
import { classes } from "../../lib/designs/engine.js";
import { plan } from "./scene.js";
import { rows, steps, Widget } from "./widget.jsx";

const value = { seed: 1, dim: 2, base: 2, code: "", level: 3, tint: "degree", spin: 0 };

test("a cube mounts its stage only once three is in hand too", () => {
  const page = (unit, three, patch = {}) =>
    renderToStaticMarkup(
      <Frame label="Designs">
        <Widget value={{ ...value, ...patch }} onChange={() => {}} unit={unit} three={three} />
      </Frame>,
    );
  expect([
    page(math, undefined, { dim: 3 }).includes("<canvas"),
    page(math, {}, { dim: 3 }).includes("<canvas"),
  ]).toEqual([false, true]);
});

test("the side arrows step through the classes, and through the named where the space is too big to class", () => {
  expect([steps(math, 2, 2), steps(math, 3, 3)]).toEqual([
    classes(math, 2, 2).map((one) => one.code),
    math.name.words(3, 3).map(([code]) => code),
  ]);
  expect(steps(math, 2, 2)).toHaveLength(6);
});

test("the readouts name the design, its class, orbit, form and fill out of the grid, the codes and classes of its space, and the distinct counts by dimension", () => {
  const read = (patch) => Object.fromEntries(rows(plan(math, { ...value, ...patch }).info));
  expect(read({ code: "7" })).toMatchObject({
    Design: "carpet 7",
    Orbit: "4",
    Degree: "2",
    "Normal form": "1+xy",
    Filled: "512 of 729",
    Dimension: (Math.log(8) / Math.log(3)).toFixed(4),
    Codes: "16",
    Classes: "6",
    "Distinct, dim 1 to 4": "3, 6, 22, 402",
  });
  expect([read({ code: "7" }).Class, read({ code: "11" }).Class, read({ base: 3, code: "495" }).Degree]).toEqual([
    undefined,
    "7",
    undefined,
  ]);
});
