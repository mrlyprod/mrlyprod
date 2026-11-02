import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { ComponentType } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Bind, Part, Props, Tile } from "../../shell/design.ts";
import { lacks, OPTIONAL, REQUIRED } from "../../shell/design.ts";
import { blocks } from "./index.ts";

/* SAMPLES */

const none: Bind = () => ({});

const clock = { now: new Date(2026, 10, 2, 21, 4), hours: 24 as const };

const tile: Tile = { id: "2048", title: "2048", href: "/2048/", art: <svg /> };

const item = { label: "New", keys: ["Cmd", "N"], on: true };

const SAMPLES: { [K in Part]: Props[K] } = {
  desk: { canvas: none, home: true },
  bar: {
    site: { name: "MrlyProd", logo: "/favicon.svg", word: "MRLYPROD" },
    app: "2048",
    menus: [{ name: "File", bind: none }],
    cart: 2,
    clock,
    bind: { launch: none, mark: none, cart: none, find: none },
  },
  menu: { name: "File", items: [item, null, { label: "Quit" }], mac: true, bind: { list: none, item: () => none } },
  window: {
    kind: "page",
    title: "About",
    zoomed: false,
    tools: true,
    drawer: true,
    width: 610,
    bind: { close: none, zoom: none, tools: none, edge: () => none, scroll: none, main: none, pane: none, scrim: none },
    children: <p>Body</p>,
  },
  app: {
    title: "2048",
    slots: { stage: none, left: none, right: none, actions: none, status: none },
    open: { left: true, right: false },
    widths: { left: 270, right: 264 },
    float: true,
    onFlip: () => {},
    bind: { root: none, pane: () => none, grip: () => none, opener: () => none },
    children: <canvas />,
  },
  control: { kind: "number", label: "Board", value: 4, min: 2, max: 8, onChange: () => {} },
  group: { name: "Board", open: true, live: "4", children: <p>Rows</p> },
  dialog: { title: "Keys", light: false, bind: none, children: <p>Keys</p> },
  prose: {
    kind: "post",
    title: "Hello",
    lead: "Lede",
    by: "Carlo",
    html: "<p>Body</p>",
    contents: [{ id: "one", level: 2, text: "One", on: true }],
    cards: [{ href: "/a/", name: "A" }],
  },
  tile: { ...tile, current: true, named: true, bind: none },
  launcher: {
    word: "MRLYPROD",
    query: "",
    onQuery: () => {},
    chips: ["All", "Games"],
    chip: "All",
    onChip: () => {},
    naming: "Games",
    sections: [
      { name: "Games", tiles: [tile], bind: { shelf: none, head: none, list: none, tile: () => none } },
      { name: "Empty", tiles: [], bind: { shelf: none, head: none, list: none, tile: () => none, drop: none } },
    ],
    results: null,
    at: 0,
    said: "Saved",
    more: [item],
    menu: { label: "Games", items: [item], bind: { list: none, item: () => none } },
    bind: {
      root: none,
      veil: none,
      find: none,
      body: none,
      results: none,
      hit: () => none,
      chips: none,
      chip: () => none,
      rename: none,
      add: none,
      more: { button: none, list: none, item: () => none },
      said: none,
      undo: none,
    },
  },
  library: {
    label: "Games",
    chips: [["all", "All"]],
    chip: "all",
    onChip: () => {},
    shelves: [{ name: "Saved", grid: true, items: [{ label: "One", on: true, bind: none }] }],
  },
  dock: {
    launch: tile,
    groups: [[tile]],
    magnify: false,
    bind: { root: none, group: () => none, tile: () => none, lens: { item: () => none, gap: () => none } },
  },
  widgets: {
    cards: [{ id: "clock", name: "Clock", size: "small", body: <p>21:04</p>, held: false }],
    editing: true,
    gallery: [{ id: "news", name: "News", size: "wide", body: <p>News</p>, added: false }],
    menu: null,
    onEdit: () => {},
    onAdd: () => {},
    onRemove: () => {},
    bind: {
      root: none,
      column: none,
      card: () => none,
      edit: none,
      gallery: none,
      shelf: none,
      pick: () => none,
      done: none,
    },
  },
  lock: { word: "MRLY", state: "wake", bind: none },
  saver: { clock, stage: none },
};

const draw = <K extends Part>(part: K) => {
  const Part = blocks.parts[part] as ComponentType<Props[K]>;
  return renderToStaticMarkup(<Part {...SAMPLES[part]} />);
};

/* PARTS */

test("blocks draws every required part", () => {
  expect(lacks(blocks)).toEqual([]);
});

test("every part blocks draws renders its sample inside the blocks scope", () => {
  const drawn = [...REQUIRED, ...OPTIONAL].filter((part) => blocks.parts[part]);
  expect(drawn.filter((part) => !draw(part).includes('data-design="blocks"'))).toEqual([]);
});

test("the launcher draws its search results when a query finds something", () => {
  const found = { name: "Apps", tiles: [tile], hits: [{ label: "Help", href: "/help/" }] };
  const Launcher = blocks.parts.launcher;
  const html = renderToStaticMarkup(<Launcher {...SAMPLES.launcher} query="he" results={[found]} at={1} />);
  expect(html).toContain('href="/help/"');
});

/* SHEET */

const SHEET = blocks.css.map((one) => readFileSync(join(import.meta.dir, one), "utf8")).join("\n");

const SITE = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "..", "site.json"), "utf8")) as {
  fonts: { faces: { family: string }[] };
};

test("the blocks sheet names no font outside the vendored ones", () => {
  const vendored = new Set(SITE.fonts.faces.map(({ family }) => family));
  const faces = [...SHEET.matchAll(/(--bk-sans|font-family|font)\s*:\s*([^;{}]+);/g)].map(([, , one]) => one!.trim());
  const named = faces.flatMap((one) => [...one.matchAll(/"([^"]+)"/g)].map(([, name]) => name!));
  expect(named.filter((one) => !vendored.has(one))).toEqual([]);
  expect(faces.filter((one) => !/"|^var\(--bk-sans\)$|^inherit$/.test(one))).toEqual([]);
});

test("every blocks rule is scoped to the design", () => {
  const rules = SHEET.replace(/\/\*[^*]*\*\//g, "")
    .replace(/@keyframes[^{]+\{(?:[^{}]*\{[^}]*\})*[^}]*\}/g, "")
    .replace(/@media[^{]+\{/g, "")
    .matchAll(/([^{}]+)\{/g);
  const loose = [...rules]
    .flatMap(([, one]) => one!.split(/,(?![^(]*\))/))
    .map((one) => one.trim())
    .filter((one) => one && !/^(?::root:has\()?\[data-design="blocks"\]/.test(one));
  expect(loose).toEqual([]);
});
