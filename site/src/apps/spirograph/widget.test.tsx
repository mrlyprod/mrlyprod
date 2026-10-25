import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { defaults } from "../../lib/app/spec.ts";
import { glance, TRACKS } from "./engine.ts";
import type { Spirograph } from "./scene.ts";
import { SPEC } from "./scene.ts";
import type { Etcher } from "./widget.tsx";
import { etch, FOLDS, lit, sub, Tracks } from "./widget.tsx";

type Call = [string, ...number[]];

const value = { ...defaults(SPEC), seed: 1 } as Spirograph;

test("every spec row has one home in the pane: a fold, or the design chip for base and code", () => {
  const homes = [...FOLDS.flatMap(([, keys]) => keys), "base", "code"];
  expect([...homes].sort()).toEqual(SPEC.map((row) => row.key).sort());
});

test("the library lists the tracks, the one in hand lit by the keys its kind reads, each with its ratio or sides", () => {
  const html = renderToStaticMarkup(<Tracks value={value} math={math} onChange={() => {}} />);
  expect([...html.matchAll(/class="label">([^<]*)</g)].map(([, name]) => name)).toEqual(TRACKS.map(([name]) => name));
  expect(html.match(/class="on"/g)).toHaveLength(1);
  const classic = TRACKS[0][1];
  expect([
    lit(value, classic),
    lit({ ...value, laps: 9, sides: 9 }, classic),
    lit({ ...value, ring: 8 }, classic),
  ]).toEqual([true, true, false]);
  expect([sub(classic), sub(TRACKS.find(([, p]) => p.kind === "line")![1]), sub(TRACKS.at(-1)![1])]).toEqual([
    "R/r = 7/3",
    "on a line",
    "6 sides",
  ]);
});

test("a track thumb is one stroke of a curve per pencil, fitted inside its square", () => {
  const log: Call[] = [];
  const ctx = {
    canvas: null,
    ...Object.fromEntries(
      ["beginPath", "moveTo", "lineTo", "stroke"].map((name) => [
        name,
        (...args: number[]) => log.push([name, ...args]),
      ]),
    ),
  } as Etcher;
  const one = glance(math, value, "495", TRACKS[0][1]);
  etch(ctx, one, 112, 112, 2);
  const names = log.map(([name]) => name);
  expect([
    names.filter((name) => name === "moveTo").length,
    names.filter((name) => name === "lineTo").length,
    names.filter((name) => name === "stroke").length,
  ]).toEqual([8, 8 * (one.samples - 1), 1]);
  expect(
    log
      .filter(([name]) => name !== "stroke" && name !== "beginPath")
      .every(([, x, y]) => x >= 0 && x <= 112 && y >= 0 && y <= 112),
  ).toBe(true);
});
