import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { ComponentType } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Part, Props } from "../../net/shell/design.ts";
import { lacks } from "../../net/shell/design.ts";
import { terminal } from "./index.ts";

/* SAMPLES */

const none = () => ({});

const clock = { now: new Date(0), hours: 24 as const };

const tile = { id: "2048", title: "2048", href: "/2048/", art: <svg /> };

const SAMPLES: { [K in Part]: Props[K] } = {
  desk: { canvas: none, home: true },
  bar: {
    site: { name: "MrlyProd", logo: "/favicon.svg", word: "MRLYPROD", cart: true },
    app: "2048",
    menus: [{ name: "File", bind: none }],
    cart: 2,
    clock,
    bind: { launch: () => ({ href: "/menu/" }), mark: none, cart: () => ({ href: "/cart/" }), find: none },
  },
  menu: {
    name: "Edit",
    items: [
      { label: "Undo", keys: ["mod", "z"] },
      null,
      { label: "Wrap", on: true },
      { label: "Fast", radio: true, on: false, off: true },
    ],
    mac: false,
    bind: { list: none, item: () => none },
  },
  window: {
    kind: "page",
    title: "2048",
    zoomed: false,
    tools: true,
    drawer: true,
    width: 704,
    menus: [{ name: "File", bind: none }],
    bind: {
      close: () => ({ href: "/" }),
      zoom: none,
      tools: none,
      edge: () => none,
      scroll: none,
      main: none,
      pane: none,
      scrim: none,
    },
    children: <p>Body</p>,
    side: <p>Side</p>,
  },
  app: {
    title: "2048",
    slots: { stage: none, left: none },
    open: { left: true, right: false },
    widths: { left: 200, right: 200 },
    float: false,
    onFlip: () => {},
    bind: { root: none, pane: () => none, grip: () => none, opener: () => none },
    children: <canvas />,
  },
  control: { kind: "slider", label: "Speed", value: 3, min: 0, max: 10, onChange: () => {} },
  group: { name: "Board", open: true, live: "4", children: <span>Size</span> },
  dialog: { title: "Keys", light: false, bind: none, children: <button type="button">OK</button> },
  prose: {
    kind: "hub",
    title: "Research",
    lead: "Lead.",
    contents: [{ id: "a", level: 2, text: "A", on: true }],
    cards: [{ href: "/a/", name: "A", text: "Text", figure: "carpet" }],
  },
  tile: { ...tile, named: true, current: true, bind: none },
  launcher: {
    word: "MRLYPROD",
    query: "",
    onQuery: () => {},
    chips: ["Games", "Tools"],
    chip: "",
    onChip: () => {},
    naming: "Tools",
    sections: [
      {
        name: "Games",
        tiles: [tile],
        bind: { shelf: none, head: none, list: none, tile: () => none, drop: none },
      },
    ],
    results: null,
    at: 0,
    said: "Saved",
    more: [{ label: "Reset", act: "reset" }],
    menu: { label: "2048", items: [{ label: "Remove" }], bind: { list: none, item: () => none } },
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
    shelves: [{ name: "Games", grid: false, items: [{ label: "2048", on: true, href: "/2048/", bind: none }] }],
  },
  dock: {
    launch: tile,
    groups: [[tile]],
    magnify: false,
    bind: { root: none, group: () => none, tile: () => none, lens: { item: () => none, gap: () => none } },
  },
  widgets: {
    cards: [{ id: "clock", name: "Clock", size: "small", body: <time>21:04</time>, held: false }],
    editing: true,
    gallery: [{ id: "clock", name: "Clock", size: "small", body: null, added: false }],
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

const KINDS = ["slider", "number", "toggle", "pick", "segment", "text", "button"] as const;

const draw = (part: Part, props: object) => {
  const Part = terminal.parts[part] as ComponentType<object>;
  return renderToStaticMarkup(<Part {...props} />);
};

/* CLASSIC */

test("terminal draws every required part", () => {
  expect(lacks(terminal)).toEqual([]);
});

test("every terminal part renders its sample under the terminal scope", () => {
  const drawn = (Object.keys(SAMPLES) as Part[]).map((part) => [part, draw(part, SAMPLES[part]).startsWith("<")]);
  const scoped = KINDS.map((kind) => draw("control", { ...SAMPLES.control, kind, options: [[1, "One"]] }));
  expect(drawn.filter(([, ok]) => !ok)).toEqual([]);
  expect(scoped.filter((html) => !html.includes('class="terminal'))).toEqual([]);
});

test("the terminal window keeps the hooks the shell finds by class", () => {
  const html = draw("window", SAMPLES.window) + draw("bar", SAMPLES.bar);
  const hooks = [
    'class="title"',
    'class="name"',
    "close",
    "zoom",
    'class="menus"',
    "tray",
    'class="clock"',
    "glyph find",
  ];
  expect(hooks.filter((one) => !html.includes(one))).toEqual([]);
});

test("the terminal slider draws its value as a gauge of cells", () => {
  const html = draw("control", { ...SAMPLES.control, value: 5, min: 0, max: 10 });
  expect(html).toContain(`[${"#".repeat(8)}${"-".repeat(8)}]`);
});

test("the terminal launcher keeps the hooks the shell finds by id and class", () => {
  const results = [
    { name: "Apps", tiles: [tile], hits: [] },
    { name: "Notes", tiles: [], hits: [{ label: "A", href: "/a/" }] },
  ];
  const hit = (i: number) => () => ({ id: `hit-${i}` });
  const html = draw("launcher", { ...SAMPLES.launcher, results, bind: { ...SAMPLES.launcher.bind, hit } });
  const anchored = [0, 1].filter((i) => !new RegExp(`id="hit-${i}"[^>]*><a `).test(html));
  expect([html.includes('class="find"'), anchored]).toEqual([true, []]);
});

/* SHEET */

const SHEET = readFileSync(join(import.meta.dir, "terminal.css"), "utf8");

test("the terminal sheet leaves every colour to the skin, so dark follows the shell's switch", () => {
  const set = Object.keys({ ...terminal.skin.light, ...terminal.skin.dark }).filter((k) => SHEET.includes(`${k}:`));
  expect([set, Object.keys(terminal.skin.dark).length > 15]).toEqual([[], true]);
});

test("the terminal sheet scopes every rule under the terminal class", () => {
  const rules = [
    ...SHEET.replace(/\/\*[^*]*\*\//g, "")
      .replace(/@keyframes[^{]+\{(?:[^{}]*\{[^}]*\})*\s*\}/g, "")
      .matchAll(/([^{};]+)\{/g),
  ]
    .map(([, sel]) => sel!.trim())
    .filter((sel) => !sel.startsWith("@") && !/^(from|to)$/.test(sel));
  const loose = rules
    .flatMap((sel) => sel.split(",").map((one) => one.trim()))
    .filter((one) => !one.includes(".terminal"));
  expect([rules.length > 100, loose]).toEqual([true, []]);
});

test("the terminal sheet names no font but the vendored ones", () => {
  const site = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "net", "mrly.net", "site.json"), "utf8"));
  const ok = new Set<string>([
    ...site.fonts.faces.map((face: { family: string }) => face.family),
    "sans-serif",
    "serif",
    "monospace",
  ]);
  const named = [...SHEET.matchAll(/(?:font-family|--sans|--serif|--mono)\s*:\s*([^;]+);/g)]
    .flatMap(([, list]) => list!.split(","))
    .map((one) => one.trim().replace(/^"|"$/g, ""))
    .filter((one) => !one.startsWith("var(") && one !== "inherit");
  const skin = Object.values(terminal.skin.light)
    .filter((v) => v.includes('"'))
    .flatMap((v) => v.split(",").map((one) => one.trim().replace(/^"|"$/g, "")));
  expect([...named, ...skin].filter((one) => !ok.has(one))).toEqual([]);
  expect(SHEET).not.toMatch(/(?:^|[\s;{])font\s*:/);
});
