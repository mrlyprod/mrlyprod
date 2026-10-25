import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { laws, study } from "./engine.ts";
import type { Knobs } from "./scene.ts";
import { Laws, rows } from "./widget.tsx";

const value: Knobs & { seed: number } = {
  seed: 1,
  base: 2,
  code: "",
  level: 5,
  look: "all",
  width: 0,
  cells: 1,
  tint: "orange",
  strand: "blue",
};

test("the readouts lead with the design and the loops, name the law at its level and say whether it meets the count", () => {
  const read = (over: Partial<Knobs>) => Object.fromEntries(rows(study(math, { ...value, ...over }).facts));
  expect(read({})).toEqual({
    design: "carpet 7",
    loops: "50",
    strands: "64 = 2 x 32, proved",
    side: "32",
    filled: "243 of 1024",
    law: "L(n) = 3^(n-1) - 2^n + 1, n >= 1",
    "L(5)": "50, the law meets the count",
  });
  expect(rows(study(math, { ...value, base: 3, code: "500", level: 2 }).facts).map(([name]) => name)).toEqual([
    "design",
    "loops",
    "strands",
    "side",
    "filled",
    "law",
  ]);
  expect(read({ base: 3, code: "500", level: 2 }).law).toBe("no closed form proved for this code");
  expect(read({ code: "99" })).toEqual({
    design: "code 99",
    fault: "code 99 out of range for dimension 2 base 2 (0..15).",
  });
  expect(rows(null)).toEqual([]);
});

test("the library holds the designs whose law counts some loops, by base, each with its count at this level, the one in use lit", () => {
  expect(laws(math).map((one) => `${one.base}:${one.code}`)).toEqual(["2:7", "2:9", "2:11", "2:13", "2:14", "3:495"]);
  const html = renderToStaticMarkup(<Laws math={math} value={value} tick={0} onChange={() => {}} />);
  expect([...html.matchAll(/<h3>([^<]*)<\/h3>/g)].map((hit) => hit[1])).toEqual(["Base 2", "Base 3"]);
  expect(
    [...html.matchAll(/class="label">([^<]*)<\/span><span class="sub">([^<]*)</g)].map((hit) => `${hit[1]}, ${hit[2]}`),
  ).toEqual([
    "carpet 7, 50 loops",
    "void 9, 31 loops",
    "code 11, 65 loops",
    "code 13, 65 loops",
    "net 14, 50 loops",
    "carpet 495, 4444 loops",
  ]);
  expect(html.match(/aria-current="true"[^>]*title="([^"]*)"/)![1]).toBe("L(n) = 3^(n-1) - 2^n + 1, n &gt;= 1");
});
