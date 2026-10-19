import { afterAll, expect, test } from "bun:test";
import { readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { draw, parse } from "./draw.ts";

const out = join(tmpdir(), `draw-test-${process.pid}`);

const PNG = [137, 80, 78, 71, 13, 10, 26, 10];

afterAll(() => rmSync(out, { recursive: true, force: true }));

test("a figure draws a non-empty svg and a signed png in both themes", async () => {
  const files = await Array.fromAsync(draw({ names: ["research-dilations"], kinds: ["svg", "png"], themes: ["dark", "light"], size: 64, out }));
  expect(files.map((file) => basename(file))).toEqual(["research-dilations.dark.svg", "research-dilations.dark.png", "research-dilations.light.svg", "research-dilations.light.png"]);
  const [darkSvg, darkPng, lightSvg, lightPng] = files.map((file) => readFileSync(file));
  for (const one of [darkSvg!, lightSvg!]) expect(one.toString()).toMatch(/^<svg [^>]*width="64"[\s\S]*<circle [\s\S]*<\/svg>\n$/);
  for (const one of [darkPng!, lightPng!]) expect([...one.subarray(0, 8)]).toEqual(PNG);
  expect(darkPng!.equals(lightPng!)).toBe(false);
});

test("--out with no value, or followed by a flag, takes no folder", () => {
  expect(() => parse(["research-dilations", "--out"])).toThrow("--out takes a folder");
  expect(() => parse(["research-dilations", "--out", "--png"])).toThrow("--out takes a folder");
});
