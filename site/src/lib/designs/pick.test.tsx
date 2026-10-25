import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Pick as Select, Text } from "../app/controls.tsx";
import type { Value } from "../app/spec.ts";
import { Pick, Picker, Thumbs, watch } from "./pick.tsx";

type Tell = (rows: { target: Element; isIntersecting: boolean }[]) => void;

const options = (html: string) =>
  [...html.matchAll(/<option value="([^"]*)"[^>]*>([^<]*)<\/option>/g)].map((found) => [found[1], found[2]]);

test("Pick renders the named options of 2D base 2 and writes a chosen code", () => {
  const html = renderToStaticMarkup(<Pick value={{ dim: 2, base: 2, code: "7" }} onChange={() => {}} math={math} />);
  const named = options(html).slice(1);
  expect(named.length).toBe(10);
  expect(named).toContainEqual(["7", "carpet"]);
  expect(named).toContainEqual(["8", "point"]);
  expect(html).toContain('<option value="7" selected="">carpet</option>');
  const wrote: Value[] = [];
  const tree = Picker({ value: { dim: 2, base: 2, code: "7" }, onChange: (patch) => wrote.push(patch), math }).props
    .children;
  tree.find((node: ReactElement | false) => node && node.type === Select).props.onChange("14");
  tree.find((node: ReactElement | false) => node && node.type === Text).props.onChange(" 3 ");
  expect(wrote).toEqual([
    { dim: 2, base: 2, code: "14" },
    { dim: 2, base: 2, code: "3" },
  ]);
});

test("Pick shows a strip of the classes for a small space and none for the cube at base 3", () => {
  const strip = (value: Value) =>
    (renderToStaticMarkup(<Picker value={value} onChange={() => {}} math={math} />).match(/<canvas /g) ?? []).length;
  expect([
    strip({ dim: 2, base: 2, code: "7" }),
    strip({ dim: 3, base: 2, code: "23" }),
    strip({ dim: 3, base: 3, code: "1" }),
  ]).toEqual([6, 22, 0]);
});

test("Thumbs mark the picked code and name the named ones", () => {
  const html = renderToStaticMarkup(
    <Thumbs math={math} dim={2} base={2} codes={["11", "7"]} code="7" onPick={() => {}} />,
  );
  expect(html).toContain('aria-pressed="true" aria-label="carpet 7"');
  expect(html).toContain('aria-pressed="false" aria-label="bang 11"');
  expect([...html.matchAll(/<small>([^<]*)<\/small>/g)].map((found) => found[1])).toEqual(["11", "carpet"]);
});

test("Thumbs take the full width of their row, so a wrapping flex column cannot size the grid as one column", () => {
  const html = renderToStaticMarkup(<Thumbs math={math} dim={2} base={2} codes={["5"]} code="5" onPick={() => {}} />);
  expect(html).toContain('<div class="designs" style="width:100%">');
});

test("Thumbs given a note caption each code with its name and its normal form", () => {
  const html = renderToStaticMarkup(
    <Thumbs
      math={math}
      dim={2}
      base={2}
      codes={["7", "11"]}
      code="7"
      onPick={() => {}}
      note={(one) => `f${one}`}
      tone={() => "pink"}
    />,
  );
  expect([...html.matchAll(/<small>([^<]*)<\/small>/g)].map((found) => found[1])).toEqual([
    "7 carpet",
    "f7",
    "11",
    "f11",
  ]);
  expect(html).toContain('aria-label="carpet 7, f7"');
});

test("watch calls back once, when a node first comes into sight, and does nothing without an observer", () => {
  const real = globalThis.IntersectionObserver;
  const seen: string[] = [];
  expect(watch({} as Element, () => seen.push("none"))).toBeUndefined();
  let tell: Tell | null = null;
  Object.assign(globalThis, {
    IntersectionObserver: class {
      constructor(fn: Tell) {
        tell = fn;
      }
      observe() {}
      unobserve() {}
    },
  });
  try {
    const [near, far] = [{} as Element, {} as Element];
    watch(near, () => seen.push("near"));
    watch(far, () => seen.push("far"));
    tell!([
      { target: near, isIntersecting: true },
      { target: far, isIntersecting: false },
    ]);
    tell!([{ target: near, isIntersecting: true }]);
    expect(seen).toEqual(["near"]);
  } finally {
    globalThis.IntersectionObserver = real;
  }
});
