import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { hall, only } from "./locked.ts";

test("the lock page bundle refuses every site module but the design", async () => {
  const table = { "lock:page": 'import { site } from "site:routes";\nconsole.log(site);' };
  const built = await Bun.build({ entrypoints: ["lock:page"], plugins: [only(table, new Map())], throw: false });
  expect([built.success, built.logs.join("\n").includes("imports site:routes")]).toEqual([false, true]);
});

test("the hall holds every file its page loads, all under /_lock/, and tells robots to stay out", async () => {
  const out = mkdtempSync(join(tmpdir(), "lock-"));
  try {
    const files = await hall({ realm: "example.com" }, out);
    const page = readFileSync(join(out, "_lock", "index.html"), "utf8");
    const loads = [...page.matchAll(/(?:href|src)="\/([^"]+)"/g)].map(([, path]) => path!);
    const paths = new Set(files.map((one) => one.path));
    expect([
      files.every((one) => one.path.startsWith("_lock/")),
      loads.length >= 2 && loads.every((one) => paths.has(one)),
      page.includes("<title>example.com</title>"),
      readFileSync(join(out, "_lock", "robots.txt"), "utf8"),
    ]).toEqual([true, true, true, "User-agent: *\nDisallow: /\n"]);
  } finally {
    rmSync(out, { recursive: true, force: true });
  }
});
