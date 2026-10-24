import { expect, test } from "bun:test";
import { tidy } from "../../lib/app/spec.js";
import { spanOf } from "../../lib/fractal/view.js";
import { dwell, KINDS, most, PLACES, roll } from "./places.js";
import { at, go, SPEC } from "./scene.js";

const GRID = 24;

test("every place has its own id and its own name, and both kinds have places", () => {
  expect([new Set(PLACES.map((one) => one.id)).size, new Set(PLACES.map((one) => one.name)).size]).toEqual([
    PLACES.length,
    PLACES.length,
  ]);
  expect(KINDS.map(([kind]) => PLACES.filter((one) => one.kind === kind).length >= 8)).toEqual([true, true]);
});

test("every place frames the edge of its set: on a grid over its view the escape times differ at least threefold", () => {
  const edges = PLACES.map((one) => {
    const span = spanOf(one.zoom);
    const counts = [];
    for (let i = 0; i < GRID; i++)
      for (let j = 0; j < GRID; j++) {
        const sl = dwell(
          one,
          Number(one.x) + ((i + 0.5) / GRID - 0.5) * span,
          Number(one.y) + ((j + 0.5) / GRID - 0.5) * span,
          most(one),
        );
        counts.push(sl < 0 ? Infinity : sl);
      }
    return [one.id, Math.max(...counts) > 3 * Math.min(...counts)];
  });
  expect(edges.filter(([, ok]) => !ok)).toEqual([]);
});

test("every place survives the knobs as it is, so a pick marks itself on at the shallow cap", () => {
  const start = tidy(SPEC, {});
  expect(
    PLACES.filter((one) => !at({ ...start, ...tidy(SPEC, { ...start, ...go(one) }) }, one)).map((one) => one.id),
  ).toEqual([]);
});

test("reroll lands on a place of the kind on show", () => {
  const kinds = KINDS.flatMap(([kind]) => [1, 2, 3, 4, 5].map((seed) => roll(kind, seed).kind === kind));
  expect(kinds.every(Boolean)).toBe(true);
});
