import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { rank } from "../../../shell/launch.ts";
import type { App } from "../scripts/build.ts";
import { HOME, SITE } from "../scripts/home.ts";

type Given = Partial<Pick<App, "title" | "line" | "aliases">>;

type Route = Pick<App, "id"> & Given;

const ROOTS = SITE.roots.map((one) => resolve(HOME, one));

const FOLDERS = ROOTS.flatMap((root) =>
  readdirSync(root, { withFileTypes: true })
    .filter((one) => one.isDirectory() && one.name !== "shared")
    .map((one) => join(root, one.name)),
);

const ROUTES = FOLDERS.filter((dir) => existsSync(join(dir, "app.json"))).map((dir): Route => {
  const given: Given = JSON.parse(readFileSync(join(dir, "app.json"), "utf8"));
  return { id: basename(dir), ...given };
});

test("every folder of a route root is a route: a root word with a title and a line", () => {
  expect(FOLDERS.filter((dir) => !existsSync(join(dir, "app.json"))).map((dir) => basename(dir))).toEqual([]);
  expect(
    ROUTES.filter((one) => !/^[a-z][a-z0-9]*$/.test(one.id) || !one.title || !one.line).map((one) => one.id),
  ).toEqual([]);
});

test("every line is at most 90 characters", () => {
  expect(ROUTES.filter((one) => one.line!.length > 90).map((one) => one.id)).toEqual([]);
});

test("aliases, when given, are 1 to 12 lowercase words of 1 to 24 characters, none twice", () => {
  const bad = ROUTES.filter((one) => {
    const list = one.aliases;
    if (list === undefined) return false;
    return (
      !Array.isArray(list) ||
      !list.length ||
      list.length > 12 ||
      new Set(list).size !== list.length ||
      list.some(
        (alias) => typeof alias !== "string" || !alias.length || alias.length > 24 || alias !== alias.toLowerCase(),
      )
    );
  });
  expect(bad.map((one) => one.id)).toEqual([]);
});

test("a search for an alias finds its app: watch finds TV and bugs finds Feedback", () => {
  const items = ROUTES.map((one) => ({ id: one.id, title: one.title!, aliases: one.aliases }));
  expect(rank("watch", items).map((one) => one.id)).toContain("tv");
  expect(rank("bugs", items).map((one) => one.id)).toContain("feedback");
});
