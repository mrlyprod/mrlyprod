import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { Frame } from "../../lib/app/frame.tsx";
import type { Three } from "../../lib/app/hooks.ts";
import { Knobs } from "../../lib/app/knobs.tsx";
import type { Patch, Value } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { SPEC } from "./scene.ts";
import { hands, Widget } from "./widget.tsx";

const value = { ...defaults(SPEC), seed: 1, base: 2, code: "23" };

const page = (unit: Mrly, three?: Three) =>
  renderToStaticMarkup(
    <Frame label="Three">
      <Widget value={value} onChange={() => {}} unit={unit} three={three} />
    </Frame>,
  );

const knobs = (patch: Value) =>
  renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, ...patch }} units={{ math }} onChange={() => {}} />);

const folds = (html: string) => [...html.matchAll(/<summary><h3>([^<]*)<\/h3>/g)].map((found) => found[1]);

test("the stage waits for three as well as the unit", () => {
  expect(page(math).includes("<canvas")).toBe(false);
});

test("the level runs to the demo's reach of each number, and a source and a crop show only their own groups", () => {
  const max = (html: string) =>
    /<span>Level<\/span><span class="num">\d+<\/span><input type="range" min="1" max="(\d)"/.exec(html)?.[1];
  expect([2, 3, 4, 5].map((number) => max(knobs({ number })))).toEqual(["7", "4", "3", "3"]);
  expect(folds(knobs({}))).toEqual(["Solid", "Circle", "Look"]);
  expect(folds(knobs({ crop: "ball" }))).toEqual(["Solid", "Crop", "Circle", "Look"]);
  expect(folds(knobs({ source: "field", crop: "ball" }))).toEqual([
    "Solid",
    "Crop",
    "Field",
    "Planes",
    "Colour",
    "Look",
  ]);
});

test("Space is the spin: the scene toggles the spin knob and plays while it is on", () => {
  const asked: Patch[] = [];
  const on = hands(
    () => ({ ...value, spin: 1 }),
    (patch) => asked.push(patch),
  );
  const off = hands(
    () => ({ ...value, spin: 0 }),
    (patch) => asked.push(patch),
  );
  on.toggle();
  off.toggle();
  expect([asked, on.playing(), off.playing()]).toEqual([[{ spin: 0 }, { spin: 1 }], true, false]);
});
