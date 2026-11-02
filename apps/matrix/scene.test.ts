import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { Opts, View } from "../../net/sdk/run.ts";
import { tidy } from "../../net/sdk/spec.ts";
import FONT from "../shared/font/font.gen.json" with { type: "json" };
import type { Entry } from "../shared/font/font.ts";
import { deal, glyphs } from "./engine.ts";
import type { Knobs } from "./scene.ts";
import { make, SAVERS, SPEC } from "./scene.ts";

function stage(still: boolean, opts: Opts = {}, seed = 7, lock = false) {
  const marks: [string, number, number][] = [];
  const ctx = {
    fillText: (text: string, x: number, y: number) => marks.push([text, x, y]),
    fillRect() {},
    setTransform() {},
  };
  const canvas = Object.assign({} as HTMLCanvasElement, { getContext: () => ctx });
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#00ff00" }),
    w: 100,
    h: 100,
    dpr: 1,
    t: 0,
    still,
    lock,
  } as View;
  return { marks, scene: make(canvas, view, opts) };
}

test("the glyph pool is the chosen sets of MrlyFont", () => {
  expect(glyphs("d").join("")).toBe("0123456789");
  expect(glyphs("ld").length).toBe(36);
  const all = glyphs("lds");
  expect([all.includes("#"), all.includes("Z"), all.includes("a"), all.includes(" ")]).toEqual([
    true,
    true,
    false,
    false,
  ]);
});

test("every pooled glyph is 5 by 5", () => {
  for (const char of glyphs("lds")) {
    const { rows } = (FONT as Record<string, Entry>)[char];
    expect([char, rows.length, rows.every((row) => row.length === 5)]).toEqual([char, 5, true]);
  }
  expect([glyphs("l").length, glyphs("d").length, glyphs("s").length]).toEqual([26, 10, 35]);
});

test("matrix opens with a drop in every column, then the trickle", () => {
  const play = () => {
    const { marks, scene } = stage(false);
    const lit: number[] = [];
    const seen: [string, number, number][] = [];
    for (let frame = 0; frame < 300; frame++) {
      marks.length = 0;
      scene.draw();
      lit.push(new Set(marks.map(([, x]) => x)).size);
      seen.push(...marks);
    }
    return { lit, seen };
  };
  const { lit, seen } = play();
  expect(lit.slice(0, 5)).toEqual([5, 5, 5, 5, 5]);
  expect(lit[5]).toBeLessThan(5);
  expect(Math.max(...lit.slice(6))).toBeGreaterThan(0);
  expect(play().seen).toEqual(seen);
});

test("under reduced motion the still is the full field, the same on every draw", () => {
  const { marks, scene } = stage(true);
  scene.draw();
  const first = [...marks];
  expect([first.length, new Set(first.map(([, x]) => x)).size, new Set(first.map(([, , y]) => y)).size]).toEqual([
    25, 5, 5,
  ]);
  marks.length = 0;
  scene.draw();
  expect(marks).toEqual(first);
});

test("every matrix saver draws in the site tint", () => {
  for (const saver of SAVERS) {
    for (let seed = 0; seed < 20; seed++) expect([saver.id, saver.pick(rng(seed)).color]).toEqual([saver.id, "tint"]);
  }
});

test("the scene boots the state deal deals from the value, the first draw of the page and the stage", () => {
  for (const opts of [{}, { set: "d", color: "rainbow", cell: 8 }, { set: "ls", cell: 48 }])
    for (const lock of [false, true])
      expect(stage(false, opts, 3, lock).scene.state()).toEqual(
        deal(tidy(SPEC, opts) as Knobs, { rand: rng(3), w: 100, h: 100, dpr: 1, lock }),
      );
});
