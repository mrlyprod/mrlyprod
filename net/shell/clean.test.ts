import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const DIRTY: string[] = [];

const LOOKS: [string, RegExp][] = [
  ["hex colour", /#[0-9a-f]{3,8}\b(?![-\w])/i],
  ["colour function", /\b(?:rgba?|hsla?|hwb|lab|lch|oklab|oklch|color-mix)\(/i],
  [
    "palette name",
    /\b(?:red|orange|yellow|green|mint|teal|cyan|blue|indigo|purple|pink|brown|gray|grey|black|white)\b/i,
  ],
  ["font", /\bfont-?family\b|\b(?:sans-serif|serif|monospace|Noto)\b/i],
  ["font name", /\b[A-Z][A-Za-z0-9]*Font\b/],
  ["px size", /(?:\d|\}|")px\b/],
  ["px literal", /\bpx\(\s*[-+.\d]/],
];

const files = () =>
  readdirSync(import.meta.dir).filter((name) => /\.(?:tsx?|css|html)$/.test(name) && !/\.test\.tsx?$/.test(name));

const clean = (name: string) => !DIRTY.includes(name);

const looks = (name: string) => {
  const text = readFileSync(join(import.meta.dir, name), "utf8");
  return LOOKS.filter(([, look]) => look.test(text)).map(([what]) => `${name}: ${what}`);
};

test("a shell file holds no colour, font or px size, but the known dirty ones", () => {
  expect(files().filter(clean).flatMap(looks)).toEqual([]);
});

test("every known dirty file is still dirty, so the list only shrinks", () => {
  expect(DIRTY.filter((name) => !looks(name).length)).toEqual([]);
});
