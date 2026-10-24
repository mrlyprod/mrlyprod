import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { all, mount, text } from "../../../test/dom.js";
import { Frame } from "../../lib/app/frame.jsx";
import { defaults } from "../../lib/app/spec.js";
import { CARPET, DEEP, LEAST, law, MOST, STACKS } from "./engine.js";
import { SPEC } from "./scene.js";
import { drawn, rows, Shelves } from "./widget.jsx";

test("the readings lead with the decay, mark a layer off the law, and say when the slope is refused", () => {
  const star = new math.six.star.Star(CARPET);
  const f = { ...law(math, star, 30, 2), size: 384, low: 0.32, high: 1 };
  star.free();
  const said = Object.fromEntries(rows(f));
  expect(
    rows(f)
      .slice(0, 4)
      .map(([name]) => name),
  ).toEqual(["layers", "excess x L", "slope", "width law"]);
  expect(said).toMatchObject({
    slope: "needs L divisible by four",
    "width law": "-0.166667",
    "L mod 4": "2 mod 4",
    band: "2 cells about it",
    "ink range": "0.3200 to 1.0000",
    pixels: "384 by 384",
  });
  expect(said["arm shares"].startsWith("1/1 5/9 (law 1/3)")).toBe(true);
  expect([rows({ error: "no" }), rows(null)]).toEqual([[["error", "no"]], []]);
});

test("a reroll draws an even L inside the slider and nothing else", () => {
  expect([
    drawn(() => 0).layers,
    drawn(() => 0.9999).layers,
    drawn(() => 0.5).layers % 2,
    Object.keys(drawn(() => 0)),
  ]).toEqual([LEAST, MOST, 0, ["layers"]]);
});

test("the library lists every stack and the deep ones, and marks the L on show", async () => {
  const items = await mount(async (draw, host) => {
    await draw(
      <Frame label="Star">
        <Shelves value={{ ...defaults(SPEC), seed: 1 }} onChange={() => {}} math={math} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });
  expect(items).toHaveLength(STACKS.reduce((sum, [, list]) => sum + list.length, DEEP.length));
  expect(items.filter((at) => at.attributes["aria-current"] === "true").map(text)).toEqual(["L 280 mod 4"]);
});
