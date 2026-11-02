import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const SRC = resolve(import.meta.dir, "..", "src");

const SPEC = /(?:\bfrom|\bimport)\s*\(?\s*(['"])(\.{1,2}\/[^'"\n]*)\1/g;

const MAY: Partial<Record<string, string[]>> = {
  shell: ["lib", "styles"],
  lib: ["lib"],
  pages: ["lib", "shell"],
  apps: ["lib"],
  designs: ["lib"],
};

const SEAM = ["shell/active.ts", "shell/design.ts", "shell/measure.ts"];

const CONTRACT = ["shell/design.ts"];

const area = (file: string) => {
  const [top, id] = relative(SRC, file).split("/");
  return top === "pages" || top === "apps" || top === "designs" ? `${top}/${id}` : top;
};

const code = readdirSync(SRC, { encoding: "utf8", recursive: true })
  .filter((path) => /\.(js|jsx|ts|tsx)$/.test(path) && !/\.test\./.test(path))
  .map((path) => join(SRC, path));

const edges = code.flatMap((file) =>
  [...readFileSync(file, "utf8").matchAll(SPEC)]
    .map(([, , spec]) => resolve(dirname(file), spec))
    .filter((to) => to.startsWith(`${SRC}/`))
    .map((to) => ({ file: relative(SRC, file), to: relative(SRC, to), from: area(file), into: area(to) }))
    .filter(({ from, into }) => from !== into),
);

const top = (name: string) => name.split("/")[0]!;

const allowed = ({ from, into, to }: (typeof edges)[number]) =>
  (MAY[top(from)]?.includes(top(into)) ?? false) ||
  (top(from) === "lib" && SEAM.includes(to)) ||
  (top(from) === "designs" && CONTRACT.includes(to));

const broken = edges.filter((one) => !allowed(one));

test("apps and lib import only lib, pages lib and the shell, the shell lib and styles, and no route imports another", () => {
  expect(
    [
      ...new Set(
        broken
          .filter(({ from }) => !["lib", "designs"].includes(top(from)))
          .map(({ from, into }) => `${from} -> ${into}`),
      ),
    ].sort(),
  ).toEqual([]);
});

test("lib reaches the shell only through the design seam: active, design and measure", () => {
  expect(broken.filter(({ from }) => top(from) === "lib").map(({ file, to }) => `${file} -> ${to}`)).toEqual([]);
});

test("a design reaches the shell only through its contract", () => {
  expect(broken.filter(({ from }) => top(from) === "designs").map(({ file, to }) => `${file} -> ${to}`)).toEqual([]);
});
