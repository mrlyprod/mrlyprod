import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { ComponentType } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Bind, Design, Part, Props, Tile } from "../net/shell/design.ts";
import { lacks, OPTIONAL, REQUIRED } from "../net/shell/design.ts";

/* DESIGNS */

const NAMES = readdirSync(import.meta.dir).filter((one) => existsSync(join(import.meta.dir, one, "index.ts")));

const DESIGNS: Design[] = await Promise.all(
  NAMES.map(async (name) => {
    const one: Record<string, unknown> = await import(`./${name}/index.ts`);
    return Object.values(one).find((value): value is Design => Boolean((value as Design)?.parts))!;
  }),
);

/* SPIES */

const spy =
  (name: string): Bind =>
  () => ({ [`data-spy-${name}`]: "" });

const each =
  (name: string) =>
  (_: unknown): Bind =>
    spy(name);

const tile: Tile = { id: "2048", title: "2048", href: "/2048/", art: <svg /> };

const clock = { now: new Date(2026, 10, 2, 21, 4), hours: 24 as const };

const SAMPLES: { [K in Part]: Props[K] } = {
  desk: { canvas: spy("canvas"), home: true },
  bar: {
    site: { name: "Site", logo: "/mark.svg", word: "SITE", cart: true },
    app: "2048",
    cart: 2,
    clock,
    bind: { launch: spy("launch"), mark: spy("mark"), cart: spy("cart"), find: spy("find"), menus: spy("menus") },
  },
  menu: {
    name: "Edit",
    items: [
      { label: "Undo", keys: ["mod", "z"] },
      null,
      { label: "Wrap", on: true },
      { label: "Help", href: "/help/" },
    ],
    mac: false,
    bind: { list: spy("list"), item: each("item") },
  },
  window: {
    kind: "page",
    title: "2048",
    zoomed: false,
    tools: true,
    drawer: true,
    width: 704,
    bind: {
      close: spy("close"),
      zoom: spy("zoom"),
      tools: spy("tools"),
      title: spy("title"),
      menus: spy("menus"),
      edge: each("edge"),
      scroll: spy("scroll"),
      main: spy("main"),
      pane: spy("pane"),
      scrim: spy("scrim"),
    },
    children: <p>Body</p>,
    side: <p>Side</p>,
  },
  app: {
    title: "2048",
    slots: { stage: spy("stage"), left: spy("left"), right: spy("right") },
    open: { left: true, right: true },
    widths: { left: 200, right: 200 },
    float: false,
    onFlip: () => {},
    bind: { root: spy("root"), pane: each("pane"), grip: each("grip"), opener: each("opener") },
    children: <canvas />,
  },
  control: { kind: "slider", label: "Speed", value: 3, min: 0, max: 10, onChange: () => {}, bind: spy("control") },
  group: { name: "Board", open: true, live: "4", bind: spy("group"), children: <span>Size</span> },
  dialog: { title: "Keys", light: false, bind: spy("dialog"), children: <button type="button">OK</button> },
  prose: {
    kind: "hub",
    title: "Research",
    lead: "Lead.",
    contents: [{ id: "a", level: 2, text: "A", on: true }],
    cards: [{ href: "/a/", name: "A", text: "Text", figure: "carpet" }],
  },
  tile: { ...tile, named: true, current: true, bind: spy("tile") },
  launcher: {
    word: "SITE",
    query: "",
    onQuery: () => {},
    chips: ["Games", "Tools"],
    chip: "",
    onChip: () => {},
    naming: null,
    sections: [
      {
        name: "Games",
        tiles: [tile],
        bind: { shelf: spy("shelf"), head: spy("head"), list: spy("list"), tile: each("tile"), drop: spy("drop") },
      },
    ],
    results: null,
    at: 0,
    said: "Saved",
    more: [{ label: "Reset", act: "reset" }],
    menu: null,
    bind: {
      root: spy("root"),
      veil: spy("veil"),
      find: spy("find"),
      body: spy("body"),
      results: spy("results"),
      hit: each("hit"),
      chips: spy("chips"),
      chip: each("chip"),
      rename: spy("rename"),
      add: spy("add"),
      more: { button: spy("more"), list: spy("more-list"), item: each("more-item") },
      said: spy("said"),
      undo: spy("undo"),
    },
  },
  dock: {
    launch: tile,
    groups: [[tile]],
    magnify: false,
    bind: {
      root: spy("root"),
      launch: spy("launch"),
      group: each("group"),
      tile: each("tile"),
      lens: { item: each("lens"), gap: each("gap") },
    },
  },
  widgets: {
    cards: [{ id: "clock", name: "Clock", size: "small", body: <time>21:04</time>, held: false }],
    editing: true,
    gallery: [{ id: "clock", name: "Clock", size: "small", body: null, added: false }],
    onEdit: () => {},
    onAdd: () => {},
    onRemove: () => {},
    bind: {
      root: spy("root"),
      column: spy("column"),
      card: each("card"),
      edit: spy("edit"),
      gallery: spy("gallery"),
      shelf: spy("shelf"),
      pick: each("pick"),
      done: spy("done"),
    },
  },
  lock: { word: "SITE", state: "wake", bind: spy("lock") },
  saver: { clock, stage: spy("stage") },
  preview: { title: "Site", theme: "dark" },
};

const draw = (design: Design, part: Part, props: object = SAMPLES[part]) => {
  const Part = design.parts[part] as ComponentType<object> | undefined;
  return Part ? renderToStaticMarkup(<Part {...props} />) : null;
};

const spread = (html: string | null) => [...(html ?? "").matchAll(/data-spy-([\w-]+)=""/g)].map(([, name]) => name!);

/* PARTS */

test("every design draws every required part", () => {
  expect(DESIGNS.flatMap((design) => lacks(design).map((part) => `${design.id}: ${part}`))).toEqual([]);
});

test("every part a design draws renders its sample, and a required one draws something", () => {
  const bad = DESIGNS.flatMap((design) =>
    [...REQUIRED, ...OPTIONAL].flatMap((part) => {
      try {
        const html = draw(design, part);
        return (REQUIRED as readonly string[]).includes(part) && !html ? [`${design.id}: ${part}`] : [];
      } catch (error) {
        return [`${design.id}: ${part}: ${error}`];
      }
    }),
  );
  expect(bad).toEqual([]);
});

test("every control kind renders in every design", () => {
  const kinds = ["slider", "number", "toggle", "pick", "segment", "text", "button"] as const;
  const options: [string, string][] = [
    ["a", "A"],
    ["b", "B"],
  ];
  const bad = DESIGNS.flatMap((design) =>
    kinds
      .filter((kind) => !draw(design, "control", { kind, label: kind, value: "a", options, onChange: () => {} }))
      .map((kind) => `${design.id}: ${kind}`),
  );
  expect(bad).toEqual([]);
});

/* BINDS */

const NEEDS: Partial<Record<Part, string[]>> = {
  bar: ["cart", "find"],
  window: ["close", "zoom", "tools", "title", "scroll", "main", "pane", "scrim"],
  app: ["root", "pane", "grip", "stage", "left", "right"],
  menu: ["list", "item"],
  control: ["control"],
  group: ["group"],
  dialog: ["dialog"],
  tile: ["tile"],
  launcher: ["root", "veil", "find", "body", "chips", "chip", "add", "more", "more-list", "more-item", "said", "undo"],
  dock: ["root", "launch", "group", "tile"],
  widgets: ["column", "card"],
  saver: ["stage"],
};

test("every design spreads each bind the shell needs from a part it draws", () => {
  const bad = DESIGNS.flatMap((design) =>
    (Object.keys(NEEDS) as Part[]).flatMap((part) => {
      const html = draw(design, part);
      if (html === null) return [];
      const lens = part === "dock" && design.measure.lens ? ["lens", "gap"] : [];
      const seen = new Set(spread(html));
      return [...NEEDS[part]!, ...lens].filter((one) => !seen.has(one)).map((one) => `${design.id}: ${part}.${one}`);
    }),
  );
  expect(bad).toEqual([]);
});

test("every design hosts the bar menus exactly once, in its bar or its window", () => {
  const hosts = DESIGNS.map((design) => {
    const html = `${draw(design, "bar")}${draw(design, "window")}`;
    return [design.id, spread(html).filter((one) => one === "menus").length];
  });
  expect(hosts.filter(([, n]) => n !== 1)).toEqual([]);
});

test("every launcher puts a link right inside each hit it marks", () => {
  const results = [
    { name: "Apps", tiles: [tile], hits: [] },
    { name: "Notes", tiles: [], hits: [{ label: "A", href: "/a/" }] },
  ];
  const hit = (i: number) => () => ({ id: `hit-${i}` });
  const bad = DESIGNS.filter((design) => {
    const html = draw(design, "launcher", {
      ...SAMPLES.launcher,
      query: "a",
      results,
      bind: { ...SAMPLES.launcher.bind, hit },
    })!;
    return [0, 1].some((i) => !new RegExp(`id="hit-${i}"[^>]*><a `).test(html));
  }).map((design) => design.id);
  expect(bad).toEqual([]);
});

/* ASK */

const asking = (wrong: boolean) => ({
  ...SAMPLES.lock,
  ask: { wrong, busy: false, bind: { form: spy("form"), field: spy("field") } },
});

const FIELD = /<form[^>]*data-spy-form=""[^>]*>(?:(?!<\/form>)[\s\S])*<input[^>]*data-spy-field=""[\s\S]*<\/form>/;

test("every design draws a lock that asks: one field inside one form, and a wrong password says so plainly", () => {
  const bad = DESIGNS.filter((design) => {
    const asked = draw(design, "lock", asking(false)) ?? "";
    const wrong = draw(design, "lock", asking(true)) ?? "";
    const fields = spread(asked).filter((one) => one === "field").length;
    return !FIELD.test(asked) || fields !== 1 || /wrong password/i.test(asked) || !/wrong password/i.test(wrong);
  }).map((design) => design.id);
  expect(bad).toEqual([]);
});

/* SHEETS */

const sheet = (design: Design) =>
  design.css
    .map((file) => readFileSync(join(import.meta.dir, design.id, file), "utf8"))
    .join("\n")
    .replace(/\/\*[\s\S]*?\*\//g, "");

const FACES = new Set([
  ...[
    ...readFileSync(join(import.meta.dir, "shared", "fonts", "fonts.css"), "utf8").matchAll(
      /font-family:\s*"([^"]+)"/g,
    ),
  ].map(([, name]) => name!),
]);

const LOCAL: Record<string, string[]> = {
  modern: [
    "Segoe UI",
    "Helvetica Neue",
    "Iowan Old Style",
    "Palatino Linotype",
    "Times New Roman",
    "SF Mono",
    "Liberation Mono",
  ],
};

test("every design names no font beyond the shipped faces, but its known local ones, which only shrink", () => {
  const named = DESIGNS.map((design) => {
    const css = sheet(design);
    const all = [...css.matchAll(/(?:font-family|--[\w-]+)\s*:\s*([^;{}]*"[^;{}]*);/g)].flatMap(([, value]) =>
      [...value!.matchAll(/"([^"]+)"/g)].map(([, name]) => name!),
    );
    return [design.id, [...new Set(all.filter((one) => !FACES.has(one)))]];
  });
  expect(named).toEqual(DESIGNS.map((design) => [design.id, LOCAL[design.id] ?? []]));
});

test("every design sheet leaves its skin tokens to the skin, so dark follows the shell's switch", () => {
  const set = DESIGNS.flatMap((design) => {
    const css = sheet(design);
    return Object.keys({ ...design.skin.light, ...design.skin.dark })
      .filter((token) => css.includes(`${token}:`))
      .map((token) => `${design.id}: ${token}`);
  });
  expect(set).toEqual([]);
});

const scoped = (css: string) =>
  [
    ...css
      .replace(/@keyframes[^{]+\{(?:[^{}]*\{[^}]*\})*[^}]*\}/g, "")
      .replace(/@(?:media|supports)[^{]+\{/g, "")
      .matchAll(/([^{}]+)\{/g),
  ]
    .flatMap(([, one]) => one!.split(/,(?![^(]*\))/))
    .map((one) => one.trim())
    .filter((one) => one && !/^(?:from|to|\d+%)$/.test(one));

test("every design but the pin scopes each rule to itself, by class or by data-design", () => {
  const loose = DESIGNS.filter((design) => design.id !== "modern").flatMap((design) =>
    scoped(sheet(design))
      .filter((one) => !one.includes(`.${design.id}`) && !one.includes(`[data-design="${design.id}"]`))
      .map((one) => `${design.id}: ${one}`),
  );
  expect(loose).toEqual([]);
});

test("every part a scoped design draws carries the scope its sheet keys on", () => {
  const marked = (id: string, html: string) =>
    html.includes(`data-design="${id}"`) ||
    [...html.matchAll(/class="([^"]*)"/g)].some(([, list]) => list!.split(" ").includes(id));
  const bare = DESIGNS.filter((design) => design.id !== "modern").flatMap((design) =>
    [...REQUIRED, ...OPTIONAL]
      .filter((part) => {
        const html = draw(design, part);
        return html !== null && !marked(design.id, html);
      })
      .map((part) => `${design.id}: ${part}`),
  );
  expect(bare).toEqual([]);
});
