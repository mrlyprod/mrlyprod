import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { Frame } from "../../lib/app/frame.jsx";
import { defaults } from "../../lib/app/spec.js";
import { studio } from "./engine.js";
import { SPEC } from "./scene.js";
import { label, rows, Sheets, Widget } from "./widget.jsx";

const VALUE = { ...defaults(SPEC), seed: 1, base: 2, code: "23" };

test("the picture is labelled for a reader", () => {
  const page = (units) =>
    renderToStaticMarkup(
      <Frame label="Six">
        <Widget value={VALUE} onChange={() => {}} units={units} />
      </Frame>,
    );
  expect(page({ math })).toContain('<canvas role="img" aria-label="a cube design repeated into a sheet"');
  expect(label({ name: "carpet", title: "bang dim 3, code 23" })).toBe("carpet repeated into a sheet");
});

test("the readouts are the demo stats of the sheet", () => {
  const facts = studio(math)(VALUE).facts;
  expect(rows(facts).map(([name]) => name)).toEqual([
    "design",
    "side",
    "level",
    "copies",
    "filled",
    "exposed",
    "one copy exposed",
    "tile",
    "sheet",
    "triangles",
    "empty",
    "pieces",
    "holes",
    "corners",
    "edges",
    "euler",
  ]);
  expect(rows(facts).slice(0, 7)).toEqual([
    ["design", "carpet 23"],
    ["side", "9"],
    ["level", "2 of 2"],
    ["copies", "25"],
    ["filled", "7650"],
    ["exposed", "3042"],
    ["one copy exposed", "162"],
  ]);
  expect(rows(null)).toEqual([]);
});

test("the library holds the sheets with their copies, the one in use lit, and a pick writes the whole sheet", () => {
  const html = renderToStaticMarkup(<Sheets math={math} value={VALUE} tick={0} onChange={() => {}} />);
  expect(
    [...html.matchAll(/class="label">([^<]*)<\/span><span class="sub">([^<]*)</g)].map((hit) => `${hit[1]}, ${hit[2]}`),
  ).toEqual(["tile(5,5), 25 copies", "tile(3,9), 27 copies", "disc(2), 19 copies"]);
  expect(html.match(/aria-current="true"/g)).toHaveLength(1);
  expect(
    renderToStaticMarkup(<Sheets math={math} value={{ ...VALUE, sheet: "disc" }} tick={0} onChange={() => {}} />),
  ).toContain(
    'aria-current="true" class="on" role="listitem"><canvas width="56" height="56" aria-hidden="true"></canvas><span class="label">disc(2)',
  );
  const wrote = [];
  Sheets({ math, value: VALUE, tick: 0, onChange: (patch) => wrote.push(patch) }).props.children[1].props.onPick();
  expect(wrote).toEqual([{ sheet: "rect", x: 3, y: 9 }]);
});
