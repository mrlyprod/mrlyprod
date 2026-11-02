import { afterAll, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { HOME, REPO } from "./home.ts";
import { DEFAULTS, init } from "./init.ts";

const SCRATCH = mkdtempSync(join(tmpdir(), "init-"));

afterAll(() => rmSync(SCRATCH, { recursive: true, force: true }));

test("a scaffolded site builds on the host with its home, about, menu and missing rows", () => {
  const site = join(SCRATCH, "site");
  const dist = join(SCRATCH, "dist");
  init(site, DEFAULTS);
  const wasm = join(HOME, "wasm");
  if (existsSync(wasm)) symlinkSync(wasm, join(site, "wasm"));
  const run = Bun.spawnSync(["bun", join(REPO, "net", "hosts", "web", "scripts", "build.ts")], {
    cwd: REPO,
    env: { ...process.env, MRLY_ROOT: site, MRLY_DIST: dist },
  });
  expect(run.stderr.toString()).toBe("");
  expect(run.stdout.toString()).toMatch(/^site: design glass, 4 rows, /);
  const { site: built, rows } = JSON.parse(readFileSync(join(dist, "routes.json"), "utf8")) as {
    site: { title: string; root: string };
    rows: { route: string; kind: string }[];
  };
  expect(built.title).toBe("Example");
  expect(built.root).toBe("https://example.com");
  expect(rows.map((row) => `${row.route} ${row.kind}`).sort()).toEqual([
    "/ app",
    "/404.html app",
    "/about/ prose",
    "/menu/ app",
  ]);
});

test("init refuses a folder that is not empty", () => {
  const full = join(SCRATCH, "full");
  mkdirSync(full);
  writeFileSync(join(full, "keep.txt"), "");
  expect(() => init(full, DEFAULTS)).toThrow("is not empty");
});
