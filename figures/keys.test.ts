import { expect, test } from "bun:test";
import { realpathSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { sources } from "./press.ts";

const MRLYJS = realpathSync(resolve(import.meta.dir, "../pkgs/mrlyjs"));

const reached = (name: string) => [...sources(join(import.meta.dir, `${name}.ts`))].map((file) => relative(MRLYJS, file)).filter((file) => !file.startsWith(".."));

test("a view-only figure reaches the view and no wasm", () => {
  const files = reached("site-cart");
  expect(files).toContain("view/index.js");
  expect(files.filter((file) => file.startsWith("pkg/"))).toEqual([]);
});

test("a math figure reaches the math glue and wasm and never pkg/all", () => {
  const files = reached("site-apps");
  expect(files).toContain("math.js");
  expect(files).toContain("pkg/math/mrlyjs_math.js");
  expect(files).toContain("pkg/math/mrlyjs_math_bg.wasm");
  expect(files.some((file) => file.startsWith("pkg/all/"))).toBe(false);
});

test("the press reaches every view file it draws with and no wasm", () => {
  const files = reached("press");
  expect(files).toContain("view/raster.js");
  expect(files).toContain("view/frame.js");
  expect(files.filter((file) => file.startsWith("pkg/"))).toEqual([]);
});
