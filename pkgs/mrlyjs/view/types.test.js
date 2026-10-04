import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dark } from "../../../site/kit/theme/theme.js";
import * as view from "./index.js";

const lines = readFileSync(new URL("../view.d.ts", import.meta.url), "utf8").split("\n");
const SPACES = ["grid", "hex", "iso", "plot", "field"];

// READ

function block(open) {
  const start = lines.findIndex((line) => line.startsWith(open));
  const depth = lines[start].search(/\S/);
  const end = lines.findIndex((line, i) => i > start && line === `${" ".repeat(depth)}}`);
  return lines.slice(start + 1, end);
}

function names(body, indent) {
  const out = [];
  for (const line of body) {
    const m = line.match(new RegExp(`^ {${indent}}(?:export )?(?:readonly )?(?:function |class |const |namespace |import )?(\\w+)[(:?]|^ {${indent}}(?:export )?(?:function|class|const|namespace|import) (\\w+)`));
    if (m && !line.trim().startsWith("/**") && !/^ *(export )?(interface|type) /.test(line)) out.push(m[1] ?? m[2]);
  }
  return out.filter((name) => name !== "constructor" && name !== "new").sort();
}

const top = lines.filter((line) => /^export (function|class|const|namespace|import) /.test(line)).map((line) => line.match(/^export (?:function|class|const|namespace|import) (\w+)/)[1]).sort();

function params(signature) {
  const inside = signature.slice(signature.indexOf("(") + 1, signature.lastIndexOf(")"));
  const out = [];
  let depth = 0;
  let word = "";
  for (const c of inside) {
    if ("([{<".includes(c)) depth++;
    if (")]}>".includes(c) && !(c === ">" && word.endsWith("="))) depth--;
    if (c === "," && depth === 0) {
      out.push(word);
      word = "";
    } else word += c;
  }
  if (word.trim()) out.push(word);
  return out.map((p) => p.trim()).filter(Boolean);
}

const required = (signature) => params(signature).filter((p) => !/^\w+\?:/.test(p) && !p.startsWith("...")).length;

// NAMES

test("view.d.ts declares the values view exports, no more and no fewer", () => {
  expect(Object.keys(view).sort()).toEqual(top);
});

test("each namespace of view.d.ts declares the values its module exports", () => {
  for (const space of SPACES) expect([space, Object.keys(view[space]).sort()]).toEqual([space, names(block(`export namespace ${space} {`), 4)]);
});

test("each pen declares the verbs it has", () => {
  const verbs = names(block("export interface Pen {"), 4);
  const ctx = new Proxy({}, { get: () => () => {}, set: () => true });
  expect(Object.keys(view.raster(4, 4)).sort()).toEqual([...verbs, "pixels"].sort());
  expect(Object.keys(view.svg(4, 4)).sort()).toEqual([...verbs, "text"].sort());
  expect(Object.keys(view.canvas(ctx, 4, 4)).sort()).toEqual(verbs);
});

test("the frame, the ink, the ramp and the patch declare the members they have", () => {
  const box = view.frame(0, 0, 1, 1);
  const own = [...Object.keys(box), ...Object.getOwnPropertyNames(Object.getPrototypeOf(box)).filter((k) => k !== "constructor")];
  expect(own.sort()).toEqual(names(block("export interface Frame {"), 4));
  const theme = view.ink(dark);
  expect(Object.keys(theme).sort()).toEqual(names(block("export interface Ink {"), 4));
  expect(Object.getOwnPropertyNames(theme.Ramp).filter((k) => !["length", "name", "prototype"].includes(k)).sort()).toEqual(["diverge", "fire", "heat", "tone"]);
  expect(names(block("export interface Ramps {"), 4)).toEqual(["diverge", "fire", "heat", "tone"]);
  expect(Object.keys(new view.Ramp([])).sort()).toEqual(["ground", "stops"]);
  expect(names(block("export class Ramp {"), 4)).toEqual(["at", "ground", "stops"]);
  expect(Object.keys(view.field.patch(0, 0, 1, 1)).sort()).toEqual([...names(block("export interface Patch extends Pixels {"), 4), "shape"].sort());
});

// SIGNATURES

test("each function of view.d.ts takes the arguments the function wants", () => {
  const seen = [];
  const check = (label, fn, signature) => {
    seen.push(label);
    expect([label, fn.length]).toEqual([label, required(signature)]);
  };
  for (const line of lines) {
    const top = line.match(/^export function (\w+)\(.*\): /);
    if (top) check(top[1], view[top[1]], line);
  }
  for (const space of SPACES) {
    for (const line of block(`export namespace ${space} {`)) {
      const m = line.match(/^ {4}export function (\w+)\(.*\): /);
      if (m) check(`${space}.${m[1]}`, view[space][m[1]], line);
    }
  }
  check("Ramp", view.Ramp, lines.find((line) => line.trim().startsWith("constructor(stops")));
  check("Grid", view.Grid, lines.find((line) => line.trim().startsWith("constructor(frame")));
  expect(seen.length).toBe(lines.filter((line) => /^\s*export function /.test(line)).length + 2);
});

// DOCS

test("every line of view.d.ts that declares something has one doc line over it", () => {
  const bare = [];
  lines.forEach((line, i) => {
    const text = line.trim();
    if (!text || text === "}" || text.startsWith("/**")) return;
    const above = lines[i - 1]?.trim() ?? "";
    if (!(above.startsWith("/** ") && above.endsWith(" */") && above.length > 8)) bare.push(`${i + 1}: ${text}`);
  });
  expect(bare).toEqual([]);
});
