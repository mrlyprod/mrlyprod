import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { all, mount, text } from "../../../test/dom.js";
import { Frame } from "../../lib/app/frame.jsx";
import { Knobs } from "../../lib/app/knobs.jsx";
import { defaults, tidy } from "../../lib/app/spec.js";
import { Folds } from "../../lib/two/parts.jsx";
import { SHAPES } from "./engine.js";
import { SPEC, WINDOWS } from "./scene.js";
import { Crops, panes, rows } from "./widget.jsx";

const value = { ...defaults(SPEC), seed: 1, base: 2, code: "7" };

const shown = (over) => ({ ...value, ...over, ...tidy(SPEC, { ...value, ...over }, { math }) });

const pane = (over) => renderToStaticMarkup(<Folds groups={panes(shown(over), () => {}, math)} />);

const knobs = (over) =>
  renderToStaticMarkup(<Knobs spec={SPEC} value={shown(over)} units={{ math }} onChange={() => {}} />);

const slider = (html, label) =>
  new RegExp(`<span>${label}</span><span class="num">(\\d+)[^<]*</span><input type="range" min="1" max="(\\d+)"`)
    .exec(html)
    .slice(1)
    .map(Number);

test("the design fold holds the number and the level with no picker, and the spec groups follow: shape, look, then the circle count behind its toggle", () => {
  const design = pane({});
  const order = ["<h3>Design</h3>", ">Number<", ">Level<"].map((one) => design.indexOf(one));
  expect(order.every((at) => at >= 0)).toBe(true);
  expect(order).toEqual([...order].sort((a, b) => a - b));
  for (const gone of ['aria-label="Base"', ">Named<", ">Code<"]) expect(design).not.toContain(gone);
  const rest = knobs({ circle: 1 });
  const after = [
    "<h3>Shape</h3>",
    ">Radius<",
    'aria-label="Mode"',
    ">Policy<",
    ">Exact edge<",
    "<h3>Look</h3>",
    ">Tint<",
    "<h3>Circle</h3>",
    ">Count<",
    'aria-label="Centre"',
    ">Count level<",
    ">Radius r<",
  ].map((one) => rest.indexOf(one));
  expect(after.every((at) => at >= 0)).toBe(true);
  expect(after).toEqual([...after].sort((a, b) => a - b));
  expect(rest).not.toContain("<h3>Design</h3>");
  expect(rest).not.toContain("<span>Shape</span>");
  expect(knobs({})).not.toContain(">Count level<");
});

test("the level, count and r sliders run to their caps and show what the picture holds", () => {
  const big = { number: 7, level: 5, count: 9, r: 700, circle: 1 };
  expect(slider(pane(big), "Level")).toEqual([2, 2]);
  expect(slider(knobs(big), "Count level")).toEqual([3, 3]);
  expect(slider(knobs(big), "Radius r")).toEqual([342, 342]);
  expect(knobs(big)).toContain('<span class="num">342 /342</span>');
  expect(slider(pane({ level: 5 }), "Level")).toEqual([5, 5]);
});

test("the readouts lead with the design and its census, and with the circle count while it is on", () => {
  const facts = {
    code: "7",
    name: "carpet",
    in: 332,
    cut: 76,
    out: 104,
    before: 496,
    after: 400,
    side: 27,
    note: "touching",
    level: 3,
    circle: null,
  };
  expect(rows(facts).slice(0, 4)).toEqual([
    ["design", "carpet 7"],
    ["in", 332],
    ["cut", 76],
    ["out", 104],
  ]);
  const read = rows({
    ...facts,
    circle: { side: 729, r: 27, seen: 385, inside: 363, cut: 42, d: 1.8927892607, mass: 8 },
  });
  expect(read.slice(0, 5)).toEqual([
    ["N(r)", 385],
    ["inside(r)", 363],
    ["C(r)", 42],
    ["count side", 729],
    ["design", "carpet 7"],
  ]);
  expect(Object.fromEntries(read)).toMatchObject({ d: "1.8927893", "fill of one tile": 8 });
});

test("the crops hold the five shapes then the windows, and mark the shape and the window on show", async () => {
  const marked = async (over) =>
    mount(async (draw, host) => {
      await draw(
        <Frame label="Crop">
          <Crops value={shown(over)} onChange={() => {}} unit={math} />
        </Frame>,
      );
      const items = all(host, (at) => "data-item" in (at.attributes ?? {}));
      return [items.map(text), items.filter((at) => at.attributes["aria-current"] === "true").map(text)];
    });
  const [names, on] = await marked({});
  expect(names).toEqual([...SHAPES.map(([, label]) => label), ...WINDOWS.map(([label]) => label)]);
  expect(on).toEqual(["Circle"]);
  expect((await marked({ radius: 36 }))[1]).toEqual(["Circle", "Porthole"]);
  expect((await marked({ shape: "box", radius: 54, mode: "anti" }))[1]).toEqual(["Square", "Frame"]);
});
