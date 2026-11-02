import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import type { App } from "../scripts/build.ts";
import { HOME, SITE } from "../scripts/home.ts";

type Given = Partial<Pick<App, "title" | "line">>;

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
