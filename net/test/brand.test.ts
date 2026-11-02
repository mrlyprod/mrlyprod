import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const NET = join(import.meta.dir, "..");

const BINARY = /\.(png|jpg|ico|woff2?|ttf|wasm|md)$/;

const BRAND: [string, RegExp][] = [
  ["name", /(?<!@)mrlyprod(?!\.(?:org|com))/gi],
  ["domain", /mrly\.net|mrlyprod\.(?:org|com)/gi],
  ["prefix", /\bmrly-/g],
  ["mark", /favicon\.(?:svg|png|ico)|apple-touch-icon|og\.png|M0 0h5v1h-5z/g],
  ["font", /MrlyFont/g],
  ["tint", /tint\W{1,4}indigo|indigo\W{1,4}tint/gi],
  ["social", /(?:github|youtube|twitter|x|instagram|tiktok|reddit)\.com\/|discord\.gg\//gi],
  ["email", /[\w.+-]+@(?!example\.com\b)[\w-]+(?:\.[\w-]+)+/g],
];

const KNOWN: Record<string, number> = {
  "hosts/web/scripts/home.ts: domain: mrly.net": 2,
  "hosts/web/scripts/wasm.ts: prefix: mrly-": 1,
  "shell/index.html: mark: apple-touch-icon": 1,
  "system/settings/about.tsx: social: github.com/": 1,
  "system/settings/appearance.tsx: font: MrlyFont": 1,
};

function files() {
  const out = Bun.spawnSync(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", "."], {
    cwd: NET,
  });
  return out.stdout
    .toString()
    .split("\0")
    .filter(
      (name) =>
        name &&
        !name.startsWith("mrly.net/") &&
        !name.endsWith("brand.test.ts") &&
        !BINARY.test(name) &&
        existsSync(join(NET, name)),
    );
}

function hits() {
  return files().flatMap((name) => {
    const text = readFileSync(join(NET, name), "utf8");
    return BRAND.flatMap(([kind, look]) => [...text.matchAll(look)].map(([hit]) => `${name}: ${kind}: ${hit}`));
  });
}

test("no brand fact enters net/ beyond the known ones, which only shrink; a site config like net/mrly.net/ is the one place for them", () => {
  const seen: Record<string, number> = {};
  for (const hit of hits()) seen[hit] = (seen[hit] ?? 0) + 1;
  const fresh = Object.entries(seen).filter(([hit, n]) => n > (KNOWN[hit] ?? 0));
  expect(fresh).toEqual([]);
});
