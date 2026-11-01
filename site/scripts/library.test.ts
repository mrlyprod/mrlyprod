import { afterAll, expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { Extract, Full } from "./library.ts";
import { trim, write } from "./library.ts";

const row = (key: string, more: Partial<Full> = {}): Full => ({
  key,
  fate: "loop",
  class: "island x5",
  family: "code1",
  gen: 15,
  period: 3,
  growth: 0.5,
  heat: [9, 1],
  pop: { first: [100, 125, 60], every: [], last: 16, max: [125, 1] },
  box: { first: [15, 15], every: [], last: 5, max: [15, 0] },
  ...more,
});

const extract: Extract = {
  shelves: 2,
  per_cell: 8,
  rows: [
    row("code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151"),
    row("code1.3.1 x5 island mask:code1.3.1 birth:odds survive:odds x:256 cap:151", { gen: 20, period: 4 }),
    row("code1.3.2 x1 wallpaper mask:code3.3.1 birth:primes survive:primes x:256", {
      fate: "limit",
      class: "wallpaper",
      gen: 256,
      period: undefined,
    }),
  ],
};

const dir = join(tmpdir(), `library-${process.pid}`);

afterAll(() => rmSync(dir, { recursive: true, force: true }));

test("a row keeps its key, fate, class, family, gen, period and the population's last and max, a cell its first per_cell rows", () => {
  expect(trim(extract, 1)).toEqual({
    shelves: 2,
    per_cell: 1,
    rows: [
      {
        key: "code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151",
        fate: "loop",
        class: "island x5",
        family: "code1",
        gen: 15,
        period: 3,
        pop: { last: 16, max: [125, 1] },
      },
      {
        key: "code1.3.2 x1 wallpaper mask:code3.3.1 birth:primes survive:primes x:256",
        fate: "limit",
        class: "wallpaper",
        family: "code1",
        gen: 256,
        period: 0,
        pop: { last: 16, max: [125, 1] },
      },
    ],
  });
  expect(trim(extract).rows.length).toBe(3);
});

test("write reads the extract, writes the trimmed library as one line and tells its size", () => {
  const from = join(dir, "library.json");
  const to = join(dir, "out", "library.gen.json");
  mkdirSync(dir, { recursive: true });
  writeFileSync(from, JSON.stringify(extract));
  const { shelves, rows, bytes } = write(from, to, 1);
  const text = readFileSync(to, "utf8");
  expect([shelves, rows.length, bytes, text.endsWith("\n"), JSON.parse(text).rows.length]).toEqual([
    2,
    2,
    Buffer.byteLength(text),
    true,
    2,
  ]);
});
