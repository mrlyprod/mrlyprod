import { expect, test } from "bun:test";
import * as num from "./num.js";

const bytes = await Bun.file(new URL("./pkg/num/mrlyjs_num_bg.wasm", import.meta.url)).arrayBuffer();
num.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/num.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);

test("num::factor::factorial", () => {
  const r = row("num::factor::factorial");
  expect(num.factor.factorial(r.in.number)).toBe(r.out);
});

test("num::factor::gcd", () => {
  const r = row("num::factor::gcd");
  expect(r.in.pairs.map(([a, b]) => num.factor.gcd(a, b))).toEqual(r.out);
});

test("num::factor::divisors", () => {
  const r = row("num::factor::divisors");
  expect(Array.from(num.factor.divisors(r.in.number), Number)).toEqual(r.out);
});

test("num::factor::mobius_sieve", () => {
  const r = row("num::factor::mobius_sieve");
  expect(Array.from(num.factor.mobius_sieve(r.in.limit))).toEqual(r.out);
});

test("num::prime::is_prime", () => {
  const r = row("num::prime::is_prime");
  const kept = [];
  for (let n = r.in.from; n <= r.in.to; n++) {
    if (num.prime.is_prime(n)) kept.push(n);
  }
  expect(kept).toEqual(r.out);
});

test("num::series::zeta", () => {
  const r = row("num::series::zeta");
  expect(num.series.zeta(r.in.s, r.in.terms)).toBeCloseTo(r.out, 12);
});
