import { expect, test } from "bun:test";
import * as life from "@mrlyprod/mrly/life";
import { card } from "./scene.ts";
import { status } from "./widget.tsx";

test("the card goes to the status as rows, with the gasket row only for a rule that draws one", () => {
  const stat = { rows: 129, cols: 257, live: 1000, how: "one live cell, padded by 128 and cropped to 257" };
  const rows = Object.fromEntries(status(90, card(life, 90), stat));
  expect([rows.design, rows.traits, rows.draws, rows.across]).toEqual([
    "bang dim 3, code 90",
    "affine, onto, not reversible",
    "bang dim 2, code 13",
    257,
  ]);
  expect(Object.fromEntries(status(110, card(life, 110), stat)).draws).toBeUndefined();
  expect(status(110, null, stat)).toEqual([]);
});
