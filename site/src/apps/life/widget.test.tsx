import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { Frame } from "../../lib/app/frame.tsx";
import { defaults } from "../../lib/app/spec.ts";
import { SPEC } from "./scene.ts";
import { status, Widget } from "./widget.tsx";

const stat = {
  gen: 3,
  pop: 5,
  fate: "",
  name: "B3/S23 on carpet",
  counts: [[3], []],
  label: "carpet, side 3, level 1",
  cells: 8,
  lattice: "one",
  error: "",
};

test("the readouts go to the status as rows: a dash for an unread fate, and an error row only on an error", () => {
  expect(status(stat)).toEqual([
    ["gen", 3],
    ["pop", 5],
    ["fate", "-"],
    ["rule", "B3/S23 on carpet"],
    ["born", "3"],
    ["stay", "none"],
    ["mask", "carpet, side 3, level 1"],
    ["cells", 8],
    ["lattice", "one"],
  ]);
  expect(status({ ...stat, error: "boom" }).at(-1)).toEqual(["error", "boom"]);
  expect(status(null)).toEqual([]);
});

test("a recipe's readouts are the stop, the name, the sides, the mask with its budget and reach, and the background once it flips", () => {
  const run = {
    ...stat,
    fate: "loop after 15, period 3",
    name: "Bevens/Sevens",
    counts: [
      [2, 4],
      [2, 4],
    ],
    label: "code1.3.1, budget 4, reach 1",
    recipe: "code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151",
    sides: ["evens", "evens_zeros"] as [string, string],
    budget: 4,
    reach: 1,
    background: 0,
    flips: false,
  };
  expect(status(run)).toEqual([
    ["gen", 3],
    ["pop", 5],
    ["stop", "loop after 15, period 3"],
    ["recipe", "code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151"],
    ["born", "evens: 2 4"],
    ["stay", "evens_zeros: 2 4"],
    ["mask", "code1.3.1, budget 4, reach 1"],
    ["budget", 4],
    ["reach", 1],
  ]);
  expect(status({ ...run, flips: true, background: 1 }).at(-1)).toEqual(["background", "full, flips"]);
  expect(status({ ...run, recipe: "", error: "stay names no count and no source" }).slice(3)).toEqual([
    ["recipe", "-"],
    ["born", "evens: 2 4"],
    ["stay", "evens_zeros: 2 4"],
    ["mask", "code1.3.1, budget 4, reach 1"],
    ["budget", 4],
    ["reach", 1],
    ["error", "stay names no count and no source"],
  ]);
});

test("the widget renders before its unit arrives, with no stage yet", () => {
  expect(
    renderToStaticMarkup(
      <Frame label="Life">
        <Widget value={{ ...defaults(SPEC), seed: 1 }} onChange={() => {}} />
      </Frame>,
    ),
  ).not.toContain("<canvas");
});
