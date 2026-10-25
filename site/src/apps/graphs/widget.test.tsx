import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { ReactNode } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Accordion } from "../../lib/app/controls.tsx";
import { Frame } from "../../lib/app/frame.tsx";
import type { Three } from "../../lib/app/hooks.ts";
import { Knobs, useAccordion } from "../../lib/app/knobs.tsx";
import type { Patch } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { glimpse, SETS } from "./engine.ts";
import type { Graphs, Relaxed } from "./scene.ts";
import { SPEC } from "./scene.ts";
import type { Told } from "./widget.tsx";
import { etch, lit, note, Pane, rows, Shelves, Widget } from "./widget.tsx";

type Line = [string, ...number[]];

const value = { ...defaults(SPEC), seed: 1 } as Graphs;

const page = (unit?: Mrly, three?: Three, patch: Patch = {}) =>
  renderToStaticMarkup(
    <Frame label="Graphs">
      <Widget value={{ ...value, ...patch }} onChange={() => {}} units={unit && { math: unit }} three={three} />
    </Frame>,
  );

const CUBE = { dim: 3, base: 2, view: "cube" };

const panel = (patch: Patch = {}, relax: Pick<Relaxed, "on"> | null = null) => (
  <Pane value={{ ...value, ...patch }} relax={relax} onRelax={() => {}} onReset={() => {}} onPose={() => {}} />
);

const pane = (patch?: Patch, relax?: Pick<Relaxed, "on"> | null) => renderToStaticMarkup(panel(patch, relax));

function Shared({ children }: { children?: ReactNode }) {
  return <Accordion.Provider value={useAccordion("", SPEC)}>{children}</Accordion.Provider>;
}

const order = (html: string) =>
  [...html.matchAll(/<(?:section|details)([^>]*)>(?:<summary>)?(?:<h3>([^<]*)<\/h3>)?/g)]
    .filter(([, attrs]) => attrs.includes("order:"))
    .map(([, attrs, name]): [number, string] => [Number(attrs.match(/order:(\d+)/)![1]), name ?? ""])
    .sort(([a], [b]) => a - b)
    .map(([, name]) => name);

const placed = (patch: Patch) =>
  order(
    renderToStaticMarkup(
      <Shared>
        {panel(patch)}
        <Knobs spec={SPEC} value={{ ...value, ...patch }} onChange={() => {}} />
      </Shared>,
    ),
  );

test("a cube mounts its stage only once three is in hand too", () => {
  expect([page(math, undefined, CUBE).includes("<canvas"), page(math, {} as Three, CUBE).includes("<canvas")]).toEqual([
    false,
    true,
  ]);
});

test("the pane holds Relax or Pause beside Reset on a force layout and the three poses on a cube, and no design picker", () => {
  const html = pane();
  expect(html).toBe("");
  expect(pane({ layout: "force" }, { on: false })).toContain('>Relax</button><button type="button">Reset</button>');
  expect(pane({ layout: "force" }, { on: true })).toContain(">Pause</button>");
  expect(["Corner", "Edge", "Face"].every((word) => pane(CUBE).includes(word))).toBe(true);
});

test("the pane reads the always-open rows, the layout with relax and reset on a force layout, the camera with its poses on a cube, and the look", () => {
  expect([placed({}), placed({ layout: "force", ...CUBE })]).toEqual([
    ["Design", "Layout", "Look"],
    ["Design", "Layout", "Relax", "Camera", "Pose", "Look"],
  ]);
  expect(
    renderToStaticMarkup(
      <Shared>
        <Knobs spec={SPEC} value={value} onChange={() => {}} />
      </Shared>,
    ),
  ).not.toContain("Graph");
});

test("the library lists the graphs of the space and the sets, the one in hand lit, a set lit only when it is the whole state", () => {
  const html = (patch?: Patch) =>
    renderToStaticMarkup(<Shelves value={{ ...value, ...patch }} onChange={() => {}} math={math} />);
  expect([...html().matchAll(/class="label">([^<]*)</g)].map(([, name]) => name)).toEqual([
    "Core",
    "Edge",
    "Tunnel",
    ...SETS.map(([name]) => name),
  ]);
  expect([...html(CUBE).matchAll(/class="label">([^<]*)</g)].slice(0, 3).map(([, name]) => name)).toEqual([
    "Core",
    "Edge",
    "Tunnel",
  ]);
  expect(
    [...html({ dim: 3, view: "hex" }).matchAll(/class="label">([^<]*)</g)].slice(0, 3).map(([, name]) => name),
  ).toEqual(["Core", "Dual", "Edge"]);
  expect(html().match(/class="on"/g)).toHaveLength(2);
  expect(html({ graph: "edge" }).match(/class="on"/g)).toHaveLength(1);
  expect([lit(value, SETS[0][1]), lit({ ...value, level: 3 }, SETS[0][1]), lit(value, SETS[1][1])]).toEqual([
    true,
    false,
    false,
  ]);
});

test("a glimpse thumb is one stroke of the branches and one fill of the dots, fitted inside its square", () => {
  const log: Line[] = [];
  const ctx = Object.assign({} as CanvasRenderingContext2D, {
    canvas: null,
    ...Object.fromEntries(
      ["beginPath", "moveTo", "lineTo", "arc", "stroke", "fill"].map((name) => [
        name,
        (...args: number[]) => log.push([name, ...args]),
      ]),
    ),
  });
  const seen = glimpse(math, value, "core");
  etch(ctx, seen, 112, 112, 2);
  const count = (name: string) => log.filter(([one]) => one === name).length;
  expect([count("lineTo"), count("arc"), count("stroke"), count("fill")]).toEqual([
    seen.pairs.length / 2,
    seen.count,
    1,
    1,
  ]);
  expect(log.filter(([name]) => name === "lineTo").every(([, x, y]) => x >= 0 && x <= 112 && y >= 0 && y <= 112)).toBe(
    true,
  );
});

test("the status names the design and the network, its counts to the demo's places, and the relax readouts on a force layout", () => {
  const facts: Told = {
    name: "carpet",
    code: "495",
    space: "flat",
    graph: "core",
    level: 2,
    layout: "lattice",
    nodes: 64,
    branches: 88,
    tips: 0,
    junctions: 44,
    pieces: 1,
    length: 88,
    box: 1.5700439,
    euler: null,
  };
  expect(rows(facts, null)).toEqual([
    ["design", "carpet 495"],
    ["network", "core graph, level 2, lattice"],
    ["nodes", 64],
    ["branches", 88],
    ["tips", 0],
    ["junctions", 44],
    ["pieces", 1],
    ["length", "88.00"],
    ["box dimension", "1.570"],
    ["euler", "none"],
  ]);
  expect(rows(facts, { ticks: 0, energy: 0, moved: 0, heat: 2.7 }).slice(10)).toEqual([
    ["ticks", 0],
    ["heat", "2.70e+0"],
  ]);
  expect(
    rows(facts, { ticks: 40, energy: 0.5, moved: 0.01, heat: 0.2 })
      .slice(10)
      .map(([name]) => name),
  ).toEqual(["ticks", "energy", "moved", "heat"]);
  expect([note({ ...facts, name: "", space: "cube" }, "iso"), note({ ...facts, space: "cube" })]).toEqual([
    "core graph, level 2, lattice, isometric",
    "core graph, level 2, lattice, perspective",
  ]);
});
