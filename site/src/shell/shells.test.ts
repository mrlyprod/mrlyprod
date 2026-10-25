import { expect, test } from "bun:test";
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { App } from "../../scripts/build.ts";
import { entry, NAMES, own, spells, state } from "../../scripts/build.ts";

const apps = new Map(state().apps.map((one): [string | undefined, App] => [one.id, one]));

const source = (one: App) => (spells(one.page, "scene") ? entry(one.dir) : Bun.file(join(one.dir, one.page!)).text());

test("every row is an app or prose, an app row with its index or its scene and widget, a prose row with a source, and each kind has its shell", () => {
  const { rows } = state();
  expect([...new Set(rows.map((row) => row.kind))].sort()).toEqual(["app", "prose"]);
  expect((["app", "prose"] as const).filter((kind) => !existsSync(join(import.meta.dir, NAMES[kind])))).toEqual([]);
  const page = (one: App) =>
    (spells(one.page, "scene") ? (["scene", "widget"] as const) : (["index"] as const)).every((name) =>
      existsSync(join(one.dir, NAMES[name])),
    );
  expect(
    rows.filter((row) => (row.kind === "app" ? !page(apps.get(row.meta.id)!) : !row.source)).map((row) => row.route),
  ).toEqual([]);
});

test("an app page draws as an island under its still or as a page of its own, and unmounts either way", async () => {
  const { rows } = state();
  const bad: (string | undefined)[] = [];
  const pages: (string | undefined)[] = [];
  for (const row of rows.filter((one) => one.kind === "app")) {
    const src = await source(apps.get(row.meta.id)!);
    const page = /^export (async )?function render\b|^export const \{[^}]*\brender\b/m.test(src);
    if (page) pages.push(row.meta.id);
    const parts = page ? ["render", "unmount", "layout"] : ["mount", "unmount"];
    if (
      !parts.every((name) =>
        new RegExp(`^export (async )?(function|const) ${name}\\b|^export const \\{[^}]*\\b${name}\\b`, "m").test(src),
      )
    )
      bad.push(row.meta.id);
  }
  expect(bad).toEqual([]);
  expect<(string | undefined)[]>(own(state().apps).sort()).toEqual(pages.sort());
});
