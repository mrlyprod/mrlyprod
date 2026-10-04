import { expect, test } from "bun:test";
import { resolve } from "node:path";
import { drawn } from "../kit/ssg/build.ts";
import { spec } from "./site.ts";

const home = resolve(import.meta.dir, "..");

test("every module the site loads to draw a page is in the stamp, and no test or README is", () => {
  const stamped = new Set(drawn(spec));
  const probe = `await import(${JSON.stringify(resolve(import.meta.dir, "site.ts"))}); console.log(JSON.stringify(Object.keys(require.cache)));`;
  const run = Bun.spawnSync([process.execPath, "-e", probe], { cwd: home, stderr: "ignore" });
  const loaded = (JSON.parse(run.stdout.toString()) as string[]).filter((file) => file.startsWith(`${home}/`) && !file.includes("/node_modules/"));
  expect(loaded.length).toBeGreaterThan(15);
  expect(loaded.filter((file) => !stamped.has(file))).toEqual([]);
  expect([...stamped].filter((file) => /\.test\.|\.md$/.test(file))).toEqual([]);
});
