import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const SRC = resolve(import.meta.dir, "..", "src");

const SPEC = /(?:\bfrom|\bimport)\s*\(?\s*(['"])(\.{1,2}\/[^'"\n]*)\1/g;

const MAY = { shell: ["lib", "styles"], lib: ["lib"], pages: ["lib", "shell"], apps: ["lib"] };

const area = (file) => {
  const [top, id] = relative(SRC, file).split("/");
  return top === "pages" || top === "apps" ? `${top}/${id}` : top;
};

const code = readdirSync(SRC, { recursive: true })
  .filter((path) => /\.(js|jsx|ts|tsx)$/.test(path) && !/\.test\./.test(path))
  .map((path) => join(SRC, path));

const crossings = code.flatMap((file) =>
  [...readFileSync(file, "utf8").matchAll(SPEC)]
    .map(([, , spec]) => resolve(dirname(file), spec))
    .filter((to) => to.startsWith(`${SRC}/`))
    .map((to) => [area(file), area(to)])
    .filter(([from, to]) => from !== to),
);

const allowed = ([from, to]) => MAY[from.split("/")[0]]?.includes(to.split("/")[0]) ?? false;

test("apps import only lib, pages lib and the shell, the shell lib and styles, lib only lib, and no route imports another", () => {
  expect([...new Set(crossings.filter((one) => !allowed(one)).map(([from, to]) => `${from} -> ${to}`))].sort()).toEqual(
    [],
  );
});
