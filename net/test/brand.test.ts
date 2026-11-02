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

const OWN = /^(?:shell|system)\/[^.]+(?:\/[^.]+)*\.(?:ts|tsx|css|html)$/;

const WORD = /(?<!@)mrly/gi;

const SPARED: Record<string, number> = {
  "shell/app.tsx: mrly": 1,
};

test("the shell and the system apps never name the brand, in any case, beyond the spared ones, which only shrink", () => {
  const seen: Record<string, number> = {};
  for (const name of files().filter((one) => OWN.test(one) && !one.includes(".test.")))
    for (const [hit] of readFileSync(join(NET, name), "utf8").matchAll(WORD))
      seen[`${name}: ${hit}`] = (seen[`${name}: ${hit}`] ?? 0) + 1;
  expect([
    Object.entries(seen).filter(([hit, n]) => n > (SPARED[hit] ?? 0)),
    Object.keys(SPARED).filter((hit) => !seen[hit]),
  ]).toEqual([[], []]);
});

const HOUSE = /mrlyprod|mrlymath/gi;

test("mrly.net's config names MrlyProd only in its copyright, its llms line and the home of its code", () => {
  const config = (name: string) => readFileSync(join(NET, "mrly.net", name), "utf8");
  const site = { ...JSON.parse(config("site.json")), company: "", llms: "", socials: "", git: "" };
  expect([JSON.stringify(site), config("launchpad.json"), config("redirects.json")].join("\n").match(HOUSE)).toBeNull();
});
