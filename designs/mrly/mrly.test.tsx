import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { ComponentType } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import site from "../../net/mrly.net/site.json";
import type { Bind, Part, Props, Tile } from "../../net/shell/design.ts";
import { lacks } from "../../net/shell/design.ts";
import { mrly } from "./index.ts";

/* SAMPLES */

const none: Bind = () => ({});

const clock = { now: new Date(2026, 10, 2, 21, 4), hours: 24 as const };

const tile: Tile = { id: "2048", title: "2048", href: "/2048/", art: <svg /> };

const SAMPLES: { [K in Part]: Props[K] } = {
  desk: { canvas: none, home: true },
  bar: {
    site: { name: "MrlyProd", logo: "/favicon.svg", word: "MRLYPROD", cart: true },
    app: "2048",
    menus: [{ name: "File", bind: none }],
    cart: 2,
    clock,
    bind: { launch: none, mark: () => ({ href: "/" }), cart: () => ({ "data-cart": "true" }), find: none },
  },
  menu: {
    name: "File",
    items: [{ label: "New", keys: ["Cmd", "N"], on: true }, null, { label: "Help", href: "/help/" }],
    mac: true,
    bind: { list: none, item: () => none },
  },
  window: {
    kind: "wide",
    title: "2048",
    zoomed: false,
    tools: true,
    drawer: true,
    width: 980,
    bind: {
      close: () => ({ href: "/" }),
      zoom: none,
      tools: () => ({ "data-pane": "right" }),
      edge: () => none,
      scroll: () => ({ id: "scroll" }),
      main: () => ({ id: "main" }),
      pane: () => ({ id: "right" }),
      scrim: none,
    },
    children: <p>body</p>,
    side: <p>side</p>,
  },
  app: {
    title: "2048",
    slots: { stage: none, left: none, right: none, tail: none, actions: none, status: none, hints: none },
    open: { left: true, right: false },
    widths: { left: 280, right: 320 },
    float: false,
    onFlip: () => {},
    bind: { root: none, pane: () => none, grip: () => none, opener: () => none },
    children: <canvas />,
  },
  control: { kind: "number", label: "Size", value: 4, min: 2, max: 8, unit: "x4" },
  group: { name: "Board", open: true, live: "4x4", children: <p>knob</p> },
  dialog: { title: "Keys", light: false, bind: none, children: <p>keys</p> },
  prose: {
    kind: "post",
    title: "Launch",
    lead: "Lead",
    by: "Carlo",
    dates: ["2026-11-02"],
    html: "<p>body</p>",
    contents: [{ id: "one", level: 2, text: "One", on: true }],
    cards: [{ href: "/a/", name: "A", text: "a" }],
  },
  tile: { ...tile, soon: true, current: true, named: true, bind: none },
  launcher: {
    word: "MRLYPROD",
    query: "",
    onQuery: () => {},
    chips: ["Games"],
    chip: "",
    onChip: () => {},
    naming: null,
    sections: [
      { name: "Games", tiles: [tile], bind: { shelf: none, head: none, list: none, tile: () => none, drop: none } },
    ],
    results: null,
    at: 0,
    said: "Saved",
    more: [{ label: "Reset", act: "reset" }],
    menu: null,
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
    label: "Library",
    chips: [["all", "All"]],
    chip: "all",
    onChip: () => {},
    shelves: [{ name: "Shelf", grid: true, items: [{ label: "One", on: true, bind: none }] }],
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
    gallery: [{ id: "research", name: "Research", size: "wide", body: null, added: false }],
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
  lock: { word: "MRLYPROD", state: "wake", bind: none },
  saver: { clock, stage: none },
};

const draw = (part: Part, props: object = SAMPLES[part]) => {
  const Part = mrly.parts[part] as ComponentType<object>;
  return renderToStaticMarkup(<Part {...props} />);
};

/* PARTS */

test("mrly draws every required part", () => {
  expect(lacks(mrly)).toEqual([]);
});

test("every part mrly draws renders its sample under the mrly root", () => {
  const drawn = (Object.keys(mrly.parts) as Part[]).filter((part) => draw(part).includes('data-design="mrly"'));
  expect(drawn).toEqual(Object.keys(mrly.parts) as Part[]);
});

test("every control kind renders", () => {
  const kinds = ["slider", "number", "toggle", "pick", "segment", "text", "button"] as const;
  const options: [string, string][] = [
    ["a", "A"],
    ["b", "B"],
  ];
  const drawn = kinds.filter((kind) => draw("control", { kind, label: kind, value: "a", options }).includes(kind));
  expect(drawn).toEqual([...kinds]);
});

test("the bar and window keep every hook the shell finds by id, class or data", () => {
  const html = draw("bar") + draw("window");
  const hooks = [
    'id="main"',
    'id="right"',
    'id="scroll"',
    'data-pane="right"',
    'class="date',
    'class="clock',
    'class="glyph find',
    'class="close',
    'class="menus"',
    'data-cart="true"',
    'class="mark',
    'class="trio"',
    'class="tray"',
    'class="title"',
    'class="scrim"',
  ];
  expect(hooks.filter((one) => !html.includes(one))).toEqual([]);
});

/* SHEET */

const sheet = readFileSync(join(import.meta.dir, "mrly.css"), "utf8").replace(/\/\*[\s\S]*?\*\//g, "");

test("the sheet names no font outside the vendored faces", () => {
  const vendored = new Set([...site.fonts.faces.map(({ family }) => family), "MrlyFont"]);
  const generic = new Set(["monospace", "math", "inherit"]);
  const stacks = [
    ...sheet.matchAll(/[\s;{](?:font-family|font|--mono|--sans|--serif|--math|--face|--m-pixel)\s*:\s*([^;]+);/g),
  ].map(([, one]) => one!);
  const quoted = stacks.flatMap((one) => [...one.matchAll(/"([^"]+)"/g)].map(([, name]) => name!));
  const bare = stacks.flatMap((one) =>
    one
      .replace(/"[^"]*"/g, "")
      .split(",")
      .map((name) => name.trim())
      .filter((name) => name && !name.startsWith("var(")),
  );
  expect([...quoted.filter((name) => !vendored.has(name)), ...bare.filter((name) => !generic.has(name))]).toEqual([]);
  expect(sheet).not.toMatch(/@import|@font-face|url\(/);
});

test("the sheet scopes every rule to the mrly root", () => {
  const selectors = [...sheet.matchAll(/^([^\s@}][^{]*)\{/gm)].map(([, one]) => one!.trim());
  expect(
    selectors.filter((one) => !one.split(/,(?![^(]*\))/).every((part) => part.includes('[data-design="mrly"]'))),
  ).toEqual([]);
});
