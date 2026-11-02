import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const CLEAN = [
  "active.ts",
  "app.tsx",
  "appicon.tsx",
  "catalog.ts",
  "design.ts",
  "desk.ts",
  "figures.ts",
  "frame.tsx",
  "gate.ts",
  "hub.tsx",
  "idle.ts",
  "launch.ts",
  "lens.ts",
  "measure.ts",
  "mount.ts",
  "panes.ts",
  "prose.tsx",
  "route.ts",
  "router.ts",
  "savers.ts",
  "site.d.ts",
  "tile.tsx",
  "tiles.ts",
];

const LOOKS: [string, RegExp][] = [
  ["hex colour", /#[0-9a-f]{3,8}\b(?![-\w])/i],
  ["colour function", /\b(?:rgba?|hsla?|hwb|lab|lch|oklab|oklch|color-mix)\(/i],
  [
    "palette name",
    /\b(?:red|orange|yellow|green|mint|teal|cyan|blue|indigo|purple|pink|brown|gray|grey|black|white)\b/i,
  ],
  ["font", /\bfont-?family\b|\b(?:sans-serif|serif|monospace|Noto|MrlyFont)\b/i],
  ["px size", /(?:\d|\}|")px\b/],
];

test("a cleaned shell file holds no colour, font or px size", () => {
  const found = CLEAN.flatMap((name) => {
    const text = readFileSync(join(import.meta.dir, name), "utf8");
    return LOOKS.filter(([, look]) => look.test(text)).map(([what]) => `${name}: ${what}`);
  });
  expect(found).toEqual([]);
});
