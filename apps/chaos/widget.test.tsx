import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import * as num from "@mrlyprod/num";
import type { ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Patch, Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { study } from "./engine.ts";
import type { Facts } from "./scene.ts";
import { SPEC, tidied } from "./scene.ts";
import { deeper, faster, flipped, rows, Shelves, Widget } from "./widget.tsx";

const units = { math, num };

const value = (over: Raw) => tidied({ ...defaults(SPEC), seed: 1, base: 2, code: "7", ...over }, units);

const FACTS: Facts = {
  n: 1000,
  k: 2,
  counted: 999,
  distance: 0.0123,
  painted: 9,
  total: 9,
  outside: 0,
  agree: 999,
  checked: 999,
  lap: 0,
  laps: 0,
  perfect: false,
  mode: "random",
  law: "uniform",
  stuck: false,
  lost: false,
};

const library = (over: Raw) =>
  renderToStaticMarkup(<Shelves units={units} value={value(over)} tick={0} onChange={() => {}} />);

test("the readouts lead with the draws, the distance from the law and the cells painted, then the source's own facts", () => {
  const read = rows(FACTS, study(units, value({})));
  expect(read.slice(0, 4)).toEqual([
    ["draws", 1000],
    ["distance", "0.0123 from uniform"],
    ["painted", "9 of 9 cells at level 2"],
    ["register", "999 of 999 cells agree with the last 2 draws"],
  ]);
  expect(read.map(([label]) => label)).toEqual([
    "draws",
    "distance",
    "painted",
    "register",
    "design",
    "digits",
    "dimension",
  ]);
  const laps = rows({ ...FACTS, mode: "bruijn", lap: 9, laps: 3, perfect: true }, study(units, value({})));
  expect(laps[3]).toEqual(["laps", "3 of 9 draws, every cell painted exactly once per lap"]);
  const odd = rows(
    { ...FACTS, agree: 998, outside: 2, stuck: true },
    study(units, value({ view: "rule", game: "typed", rule: "0" })),
  );
  expect(odd[0]).toEqual(["note", "no window is allowed, so the game stands still"]);
  expect(odd.map(([label]) => label)).toContain("outside the law");
  expect(odd.map(([label]) => label)).toContain("stuck");
  expect(odd.find(([label]) => label === "register")?.[1]).toBe(
    "998 of 999 cells agree with the last 2 draws, the rest a rounding at a cell edge",
  );
  expect(rows(null, null)).toEqual([]);
});

test("the arrows move the level within the source's cap and the speed along its steps, and g flips the grid", () => {
  expect([
    deeper(value({ level: 2 }), 1, 6),
    deeper(value({ level: 6 }), 1, 6),
    deeper(value({ level: 1 }), -1, 6),
    deeper(value({ level: 5 }), 1, 4),
  ]).toEqual([{ level: 3 }, { level: 6 }, { level: 1 }, { level: 4 }]);
  expect([
    faster(value({ speed: 100 }), 1),
    faster(value({ speed: 1000 }), 1),
    faster(value({ speed: 1 }), -1),
    faster({ speed: 7 }, 1),
  ]).toEqual([{ speed: 1000 }, { speed: 1000 }, { speed: 1 }, { speed: 10 }]);
  expect([flipped(value({ grid: 0 })), flipped(value({ grid: 1 }))]).toEqual([{ grid: 1 }, { grid: 0 }]);
});

test("the library shelves the sources, the hex bases, the mosaics and the named rules, lights the one on the stage, and a pick writes its view", () => {
  const html = library({});
  expect(html.match(/<h3>(\w+(?: \w+)?)<\/h3>/g)).toEqual([
    "<h3>Sources</h3>",
    "<h3>Hex bases</h3>",
    "<h3>Mosaics</h3>",
    "<h3>Rules</h3>",
  ]);
  const labels = [...html.matchAll(/class="label">([^<]*)</g)].map((hit) => hit[1]);
  expect(labels.slice(0, 11)).toEqual([
    "Design",
    "Word",
    "Mosaic",
    "Hex",
    "Slice",
    "Rule",
    "Gasket",
    "Koch curve",
    "Terdragon",
    "Flowsnake",
    "Twindragon",
  ]);
  expect(labels).toHaveLength(6 + 5 + 4 + 8);
  expect(html).toContain('class="label">Golden</span><span class="sub">base 2, 2 slots</span>');
  const lit = (over: Raw) =>
    [...library(over).matchAll(/aria-current="true"[\s\S]*?class="label">([^<]*)</g)].map((hit) => hit[1]);
  expect([lit({}), lit({ view: "hex", hex: "koch" }), lit({ view: "rule", game: "shift" })]).toEqual([
    ["Design"],
    ["Hex", "Koch curve"],
    ["Rule", "The shift rule"],
  ]);
  const wrote: Patch[] = [];
  const shelf = Shelves({ units, value: value({}), tick: 0, onChange: (patch) => wrote.push(patch) })[2];
  (shelf.props as { children: ReactElement<{ onPick: () => void }>[] }).children[1].props.onPick();
  expect(wrote).toEqual([{ view: "mosaic", mosaic: "twist" }]);
});

test("before the units land the widget renders nothing on the server", () => {
  expect(
    renderToStaticMarkup(
      <Widget
        value={{ ...value({}), seed: 1 }}
        onChange={() => {}}
        onReady={() => {}}
        onStatus={() => {}}
        onExport={() => {}}
      />,
    ),
  ).toBe("");
});
