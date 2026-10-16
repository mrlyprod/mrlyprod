import { afterAll, expect, test } from "bun:test";
import { rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";
import { mrly, roster } from "./site.ts";

const repo = resolve(import.meta.dir, "..", "..");

const out = join(tmpdir(), `figures-${process.pid}`);

afterAll(() => rmSync(out, { recursive: true, force: true }));

test("every figure of the roster loads: default is a draw function and every unit it names is defined", async () => {
  const list = roster();
  const built = await Bun.build({ entrypoints: list.map((one) => one.file), root: repo, outdir: out, target: "bun", plugins: [mrly] });
  expect(built.logs.filter((log) => log.level === "error").map(String)).toEqual([]);
  const bad: string[] = [];
  for (const one of list) {
    try {
      const figure = await import(join(out, relative(repo, one.file).replace(/\.ts$/, ".js")));
      if (typeof figure.default !== "function") bad.push(`${one.name}: no default draw`);
      for (const [unit, value] of Object.entries(figure.units ?? {})) if (value === undefined) bad.push(`${one.name}: units.${unit} is undefined`);
    } catch (error) {
      bad.push(`${one.name}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  expect(list.length).toBeGreaterThan(0);
  expect(bad).toEqual([]);
}, 60_000);
