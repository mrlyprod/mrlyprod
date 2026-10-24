import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { all, mount, text } from "../../../test/dom.js";
import { Frame } from "../../lib/app/frame.jsx";
import { defaults } from "../../lib/app/spec.js";
import { DESIGN, SPEC } from "./scene.js";
import { rows, Shelves, split } from "./widget.jsx";

const start = { ...defaults(SPEC), ...DESIGN, seed: 1 };

const items = (value) =>
  mount(async (draw, host) => {
    await draw(
      <Frame label="Moire">
        <Shelves value={{ ...start, ...value }} onChange={() => {}} math={math} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });

test("the field reads name the stack, and the r-matrix reads name the pair on trial", () => {
  expect(
    Object.fromEntries(
      rows({ view: "field", name: "carpet", code: "495", layers: 5, scales: [1, 3, 5, 7, 9], size: 384 }),
    ),
  ).toEqual({ design: "carpet 495", layers: 5, scales: "1 3 5 7 9", pixels: "384 by 384" });
  expect(
    Object.fromEntries(
      rows({ view: "matrix", row: 9, mate: 3, r: 0.2192645048, prime: false, max: 0.2192645048, at: 3, k: 4 }),
    ),
  ).toEqual({
    "scale m": 9,
    "scale n": 3,
    r: "0.219264505",
    "row of m": "largest 0.219264505 at scale 3",
    matrix: "4 by 4",
  });
  expect(
    Object.fromEntries(rows({ view: "matrix", row: 0, mate: 0, r: null, prime: false, max: 0, at: 0, k: 0 }))[
      "row of m"
    ],
  ).toBe("none");
  expect(rows({ error: "no" })).toEqual([["error", "no"]]);
  expect(rows(null)).toEqual([]);
});

test("a scale reads prime or its least split", () => {
  expect([3, 9, 15, 21, 25, 41].map(split)).toEqual(["prime", "3 x 3", "3 x 5", "3 x 7", "5 x 5", "prime"]);
});

test("the field lists the presets of the door and marks the one on show, the r-matrix lists the scales of the stack", async () => {
  const field = await items({ base: 2, code: "1", level: 1, combine: "xor" });
  expect([field.map(text), field.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    ["Heatmap", "Weave", "Hive", "Carpet"],
    ["Weave"],
  ]);
  const matrix = await items({ view: "matrix", limit: 15, scale: 9 });
  expect([matrix.map(text), matrix.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    ["m 3prime", "m 5prime", "m 7prime", "m 93 x 3", "m 11prime", "m 13prime", "m 153 x 5"],
    ["m 93 x 3"],
  ]);
});
