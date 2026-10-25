import { expect, test } from "bun:test";
import { BITS } from "./consts.ts";
import { apart } from "./orbit.ts";
import type { View } from "./view.ts";
import { pan, parse, print, spanOf, WHOLE, zoomAt, zoomOf } from "./view.ts";

test("a decimal parses to exact fixed point and prints back at the precision its span needs", () => {
  const texts = ["-0.75", "0", "0.13182590420533", "-1.7548776662467", "1.000000000000000000000000000001"];
  const spans = [3, 3, 1e-9, 1e-8, 1e-25];
  expect(parse("-0.75")).toBe(-(3n << BigInt(BITS)) / 4n);
  expect(texts.map((text, n) => print(parse(text)!, spans[n]))).toEqual(texts);
  expect([parse("1e-3") === parse("0.001"), parse(""), parse("-"), parse("x1"), parse("1e999")]).toEqual([
    true,
    null,
    null,
    null,
    null,
  ]);
});

test("a drag and its reverse, and a zoom at a point and its inverse, land where they began, turned or not", () => {
  for (const angle of [0, 0.7]) {
    const v: View = { centre: [parse("-0.743643887037151")!, parse("0.13182590420533")!], span: spanOf(40), angle };
    const back = pan(pan(v, [0.3, -0.2]), [-0.3, 0.2]);
    const out = zoomAt(zoomAt(v, [0.4, 0.1], 8), [0.4, 0.1], 1 / 8);
    for (const one of [back, out]) expect(Math.hypot(...apart(one.centre, v.centre)) / v.span).toBeLessThan(1e-9);
    expect(+zoomOf(out.span).toFixed(9)).toBe(40);
  }
  const v: View = { centre: [0n, 0n], span: WHOLE, angle: Math.PI / 2 };
  expect(apart(pan(v, [0.5, 0]).centre, v.centre).map((one) => +one.toFixed(12) || 0)).toEqual([0, -1.5]);
});
