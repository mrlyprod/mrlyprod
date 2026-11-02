import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const BRAND: [string, RegExp][] = [
  ["name", /mrlyprod(?!\.(?:org|com))/gi],
  ["domain", /mrly\.net|mrlyprod\.(?:org|com)/gi],
  ["prefix", /\bmrly-/g],
  ["mark", /favicon\.(?:svg|png|ico)|apple-touch-icon|og\.png|M0 0h5v1h-5z/g],
  ["font", /MrlyFont/g],
  ["tint", /tint\W{1,4}indigo|indigo\W{1,4}tint/gi],
  ["social", /(?:github|youtube|twitter|x|instagram|tiktok|reddit)\.com\/|discord\.gg\//gi],
  ["email", /[\w.+-]+@[\w-]+(?:\.[\w-]+)+/g],
];

const KNOWN: Record<string, number> = {
  "clean.test.ts: font: MrlyFont": 1,
  "index.html: mark: apple-touch-icon": 2,
  "index.html: mark: favicon.png": 1,
  "index.html: mark: favicon.svg": 1,
  "index.html: mark: og.png": 2,
};

function files() {
  const out = Bun.spawnSync(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", "."], {
    cwd: import.meta.dir,
  });
  return out.stdout
    .toString()
    .split("\0")
    .filter((name) => name && name !== "brand.test.ts" && existsSync(join(import.meta.dir, name)));
}

function hits() {
  return files().flatMap((name) => {
    const text = readFileSync(join(import.meta.dir, name), "utf8");
    return BRAND.flatMap(([kind, look]) => [...text.matchAll(look)].map(([hit]) => `${name}: ${kind}: ${hit}`));
  });
}

test("no brand fact enters the shell beyond the known ones, which only shrink", () => {
  const seen: Record<string, number> = {};
  for (const hit of hits()) seen[hit] = (seen[hit] ?? 0) + 1;
  const fresh = Object.entries(seen).filter(([hit, n]) => n > (KNOWN[hit] ?? 0));
  expect(fresh).toEqual([]);
});
