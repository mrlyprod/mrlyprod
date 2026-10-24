import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { thumb } from "./thumb.js";

test("a 1D design draws as a centred band", () => {
  const rects = [];
  const ctx = { setTransform() {}, clearRect() {}, beginPath() {}, rect: (...box) => rects.push(box), fill() {} };
  expect(() => thumb(ctx, math, 1, 2, "1", 56, { accent: "#000", paper: "#fff" })).not.toThrow();
  expect(rects.length > 0 && rects.every(([, top, , tall]) => top >= 18 && top + tall <= 38)).toBe(true);
});
