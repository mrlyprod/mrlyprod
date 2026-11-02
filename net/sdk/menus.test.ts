import { expect, test } from "bun:test";
import type { Menu } from "./menus.ts";
import { menus } from "./menus.ts";

const labels = (model: Menu[], name: string) =>
  model.find((one) => one.name === name)!.items.map((row) => (row ? row.label : "-"));

test("the menus hold the export rows, undo and redo, the panes an app has, its view and the rows it files under a menu", () => {
  const model = menus({
    exports: ["SVG"],
    undo: true,
    reset: true,
    view: {
      value: "b",
      options: [
        ["a", "A"],
        ["b", "B"],
      ],
    },
    panes: { right: true },
    keys: [{ key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", menu: "View" }],
    told: true,
  });
  expect(model.map((one) => one.name)).toEqual(["File", "Edit", "View", "Help"]);
  expect(labels(model, "File")).toEqual(["Export...", "Export SVG", "-", "Copy Link"]);
  expect(labels(model, "Edit")).toEqual(["Undo", "Redo", "-", "Reroll", "Reset Knobs"]);
  expect(labels(model, "View")).toEqual(["A", "B", "-", "Show Knobs", "-", "Zen", "-", "Level Up", "Level Down"]);
  expect(labels(model, "Help")).toEqual(["Keys", "Info"]);
  const edit = model[1].items;
  expect([edit[0]!.off, edit[1]!.off, model[2].items[1]!.on, model[2].items[8]]).toEqual([
    false,
    true,
    true,
    { label: "Level Down", act: "level@ArrowDown", keys: ["ArrowDown"] },
  ]);
});

test("a row of arrow keys gives one menu item a key, and a row of aliases one item on its first key", () => {
  const keys = [
    { key: ["ArrowLeft", "ArrowRight"], label: "Rule", act: "rule", menu: "View" },
    { key: ["ArrowRight", "Enter", "."], label: "Next", act: "next", menu: "Edit" },
  ];
  const model = menus({ keys });
  expect([labels(model, "View").slice(-2), model[1].items.at(-1)]).toEqual([
    ["Rule Left", "Rule Right"],
    { label: "Next", act: "next", keys: ["ArrowRight"] },
  ]);
});
