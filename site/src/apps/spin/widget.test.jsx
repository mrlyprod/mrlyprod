import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { Accordion } from "../../lib/app/controls.jsx";
import { Knobs, useAccordion } from "../../lib/app/knobs.jsx";
import { Source } from "../../lib/spin/source.jsx";
import { SPEC } from "./scene.js";
import { Needles, next, rows } from "./widget.jsx";

function Shared({ children }) {
  return <Accordion.Provider value={useAccordion("", SPEC)}>{children}</Accordion.Provider>;
}

const order = (html) =>
  [...html.matchAll(/<(?:section|details)([^>]*)>(?:<summary>)?(?:<h3>([^<]*)<\/h3>)?/g)]
    .filter(([, attrs]) => attrs.includes("order:"))
    .map(([, attrs, name]) => [Number(attrs.match(/order:(\d+)/)[1]), name ?? ""])
    .sort(([a], [b]) => a - b)
    .map(([, name]) => name);

const value = {
  seed: 1,
  source: "design",
  dim: 2,
  base: 3,
  code: "",
  number: 3,
  level: 4,
  rpm: 33,
  glow: 1,
  ramp: "heat",
  levels: 64,
  invert: 0,
  tint: "",
};

test("the needles are the demo presets as library rows, the one at the rpm lit, and a tap moves to the next", () => {
  const html = renderToStaticMarkup(<Needles rpm={900} onChange={() => {}} />);
  expect(html.match(/<button /g)).toHaveLength(5);
  expect(html).toContain('class="on"');
  const wrote = [];
  Needles({ rpm: 33, onChange: (patch) => wrote.push(patch) }).props.children[4].props.onPick();
  expect(wrote).toEqual([{ rpm: 900 }]);
  expect([next(0), next(33), next(500), next(900)]).toEqual([33, 45, 899, 33]);
});

test("the readouts are the demo stats, the pace leading them while the platter turns", () => {
  const facts = {
    kind: "flat",
    code: "495",
    name: "carpet",
    side: 81,
    fills: 4096,
    mass: 4096.04,
    disc: 0,
    peak: 1,
    reach: 57.28,
    order: 4,
  };
  expect(rows(facts, null).map(([name]) => name)).toEqual([
    "name",
    "side",
    "filled",
    "mass of the rings",
    "dark disc to",
    "brightest ring",
    "last ring at",
    "rotation order",
  ]);
  expect(rows(facts, { hz: 60, deg: 90, seen: 0 }).slice(0, 4)).toEqual([
    ["name", "carpet 495"],
    ["screen", "60 Hz"],
    ["per frame", "90.0°"],
    ["seen", "0.0°"],
  ]);
  expect(
    rows(
      { ...facts, kind: "moire", code: "weave", name: "weave", fills: null, order: 0 },
      { hz: 60, deg: 3, seen: 0 },
    ).filter(([name]) => ["name", "filled", "seen", "rotation order"].includes(name)),
  ).toEqual([
    ["name", "weave"],
    ["rotation order", "round"],
  ]);
  expect(rows(null, null)).toEqual([]);
});

test("the pane reads the view and its speed rows, the source, then the look, with no design picker in it", () => {
  const pane = (view) =>
    renderToStaticMarkup(
      <Shared>
        <Source value={{ ...value, view }} onChange={() => {}} math={null} />
        <Knobs spec={SPEC} value={{ ...value, view }} onChange={() => {}} />
      </Shared>,
    );
  expect([order(pane("table")), order(pane("wheel"))]).toEqual([
    ["", "Source", "Look"],
    ["", "Source", "Look"],
  ]);
  expect(pane("table")).not.toContain("Code");
});
