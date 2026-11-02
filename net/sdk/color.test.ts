import { expect, test } from "bun:test";
import { blend, hex, paints } from "./color.ts";
import { palette } from "./theme/palette.gen.js";
import { light } from "./theme/theme.ts";

test("a scene reads the tint from --accent, or --<hue> when its tint names one of the settings hues, and falls back to the palette", () => {
  const canvas = {} as Element;
  const css: Partial<Record<string, string>> = { "--accent": " #00ff00 ", "--red": "#ff0000" };
  Object.assign(globalThis, { getComputedStyle: () => ({ getPropertyValue: (name: string) => css[name] ?? "" }) });
  expect([
    paints(canvas).accent,
    paints(canvas, "").accent,
    paints(canvas, "red").accent,
    paints(canvas, "gray").accent,
  ]).toEqual(["#00ff00", "#00ff00", "#ff0000", "#00ff00"]);
  Reflect.deleteProperty(globalThis, "getComputedStyle");
  expect([paints(canvas).accent, paints(canvas, "mint").accent]).toEqual([light.accent, palette.mint]);
});

test("look hands the theme foreground as ink beside paper and accent, from --fg or the theme", () => {
  Object.assign(globalThis, {
    getComputedStyle: () => ({ getPropertyValue: (name: string) => (name === "--fg" ? "#123456" : "") }),
  });
  const read = paints({} as Element).ink;
  Reflect.deleteProperty(globalThis, "getComputedStyle");
  expect([read, paints({} as Element).ink]).toEqual(["#123456", light.fg]);
});

test("blend mixes two colour strings into an rgb string, and hex writes rgb bytes clamped and rounded", () => {
  expect([blend("#000000", "#ffffff", 0.5), blend("rgb(10, 20, 30)", "#000", 0)]).toEqual([
    "rgb(128, 128, 128)",
    "rgb(10, 20, 30)",
  ]);
  expect([hex([255, 0, 128]), hex([300, -4, 12.6, 9])]).toEqual(["#ff0080", "#ff000d"]);
});
