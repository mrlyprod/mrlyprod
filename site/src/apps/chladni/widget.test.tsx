import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import * as num from "@mrlyprod/mrly/num";
import { all, mount, text } from "../../../test/dom.ts";
import { Frame } from "../../lib/app/frame.tsx";
import { defaults } from "../../lib/app/spec.ts";
import { carpets, ladder } from "./engine.ts";
import type { Knobs } from "./scene.ts";
import { SPEC } from "./scene.ts";
import { rows, Shelves } from "./widget.tsx";

const items = (value: Partial<Knobs>) =>
  mount(async (draw, host) => {
    await draw(
      <Frame label="Chladni">
        <Shelves value={{ ...defaults(SPEC), seed: 1, ...value } as Knobs} onChange={() => {}} units={{ num, math }} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });

test("the plate reads its figure, pitch and grains, the sign left out when n equals m", () => {
  expect(rows({ view: "plate", n: 3, m: 5, sign: -1, pitch: 5.8309518948, grains: 40000 })).toEqual([
    ["figure", "(3, 5, -)"],
    ["pitch", "5.831"],
    ["grains", "40000"],
  ]);
  expect(rows({ view: "plate", n: 4, m: 4, sign: 0, pitch: 5.656854, grains: 1 })[0]).toEqual(["figure", "(4, 4)"]);
});

test("the carpet reads k, lambda, its domains against the bound as one row, and the grains left", () => {
  expect(rows({ view: "carpet", k: 8, count: 48, lambda: 0.0123456789, domains: 5, bound: 8, grains: 31234 })).toEqual([
    ["k", "8 of 48"],
    ["lambda", "0.012346"],
    ["domains", "5 of 8"],
    ["grains left", "31234"],
  ]);
});

test("an error reads as its error alone", () => {
  expect(rows({ error: "the carpet waits for its door" })).toEqual([["error", "the carpet waits for its door"]]);
});

test("the plate view lists every figure of the ladder and marks the one on show, the carpet view lists the designs that hold", async () => {
  const plates = await items({});
  expect([plates.length, plates.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    ladder(num).length,
    ["3, 5-"],
  ]);
  const designs = await items({ view: "carpet" });
  expect([designs.length, designs.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    carpets(math).length,
    ["carpetno. 3"],
  ]);
});
