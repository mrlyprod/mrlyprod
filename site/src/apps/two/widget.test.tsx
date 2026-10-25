import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { all, mount, text } from "../../../test/dom.ts";
import { Frame } from "../../lib/app/frame.tsx";
import type { Value } from "../../lib/app/spec.ts";
import { defaults, describe, tidy } from "../../lib/app/spec.ts";
import { Folds } from "../../lib/two/parts.tsx";
import { SHEETS, SPEC } from "./scene.ts";
import { panes, rows, Sheets } from "./widget.tsx";

const value = { ...defaults(SPEC), seed: 1, base: 3, code: "495" };

const shown = (over: Value) => ({ ...value, ...over, ...tidy(SPEC, { ...value, ...over }, { math }) });

const pane = (over: Value) => renderToStaticMarkup(<Folds groups={panes(shown(over), () => {}, math)} />);

test("the widget folds hold the side and level, then the across and down, with no picker, and the look is a spec group", () => {
  const html = pane({});
  const order = ["<h3>Design</h3>", ">Side<", ">Level<", "<h3>Sheet</h3>", ">Across<", ">Down<"].map((one) =>
    html.indexOf(one),
  );
  expect(order.every((at) => at >= 0)).toBe(true);
  expect(order).toEqual([...order].sort((a, b) => a - b));
  for (const gone of ['aria-label="Base"', ">Named<", ">Code<", 'aria-label="Presets"'])
    expect(html).not.toContain(gone);
  expect(describe(SPEC, value).map((group) => group.name)).toEqual(["Look"]);
});

test("the level slider runs to the cap of the sheet and shows the level the sheet holds", () => {
  const slider = (over: Value) =>
    /<span>Level<\/span><span class="num">(\d+)<\/span><input type="range" min="1" max="(\d+)"/
      .exec(pane(over))!
      .slice(1)
      .map(Number);
  expect(slider({ level: 5, x: 1, y: 1 })).toEqual([5, 5]);
  expect(slider({ level: 5, x: 12, y: 12 })).toEqual([3, 3]);
});

test("the readouts lead with the design, the copies, what the sheet exposes and what one copy does", () => {
  const facts = {
    code: "495",
    name: "carpet",
    copies: 25,
    exposed: 1280,
    one: 80,
    fills: 1600,
    voids: 425,
    side: 9,
    cols: 45,
    rows: 45,
    level: 2,
    corners: 1,
    edges: 2,
    euler: 3,
    dimension: 1.89,
  };
  const read = rows(facts);
  expect(read.slice(0, 4)).toEqual([
    ["design", "carpet 495"],
    ["copies", 25],
    ["exposed", 1280],
    ["one copy exposed", 80],
  ]);
  expect(Object.fromEntries(read).dimension).toBe("1.8900");
});

test("the design fold names the design, a blank or junk code the carpet", () => {
  const live = (code: string) => panes(shown({ code }), () => {}, math)[0].live;
  expect([live(""), live("abc"), live("1.5")]).toEqual([
    "carpet · side 3 · level 2",
    "carpet · side 3 · level 2",
    "1 · side 3 · level 2",
  ]);
});

test("the sheets shelf lists every named sheet and marks the one on show", async () => {
  const items = await mount(async (draw, host) => {
    await draw(
      <Frame label="Two">
        <Sheets value={shown({})} onChange={() => {}} unit={math} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });
  expect(items).toHaveLength(SHEETS.length);
  expect(items.filter((at) => at.attributes["aria-current"] === "true").map(text)).toEqual(["Block5 by 5"]);
});
