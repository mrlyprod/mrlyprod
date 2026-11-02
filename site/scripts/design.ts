import { existsSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import SITE from "../site.json";
import type { Design, Tokens } from "../src/shell/design.ts";
import { lacks } from "../src/shell/design.ts";

/* DESIGNS */

export const DESIGNS = resolve(import.meta.dir, "..", "src", "designs");

export const SLOTS = ["tokens.css", "saver.css"];

export type Drawn = { name: string; design: Design; by: "env" | "pin" | "draw" };

const index = (name: string, dir = DESIGNS) => join(dir, name, "index.ts");

export const folders = (dir = DESIGNS) =>
  readdirSync(dir, { withFileTypes: true })
    .filter((one) => one.isDirectory() && existsSync(index(one.name, dir)))
    .map((one) => one.name)
    .sort();

export async function load(name: string, dir = DESIGNS): Promise<Design> {
  const one = (await import(index(name, dir))) as Record<string, Design | undefined>;
  const found = one[name] ?? one.default;
  if (!found) throw new Error(`site: src/designs/${name}/index.ts exports no design named ${name}`);
  return found;
}

/* DRAW */

export async function choose(
  env = process.env.MRLY_DESIGN,
  pin = (SITE as { design?: string }).design,
  pick = Math.random,
  dir = DESIGNS,
): Promise<Drawn> {
  const named = env || pin;
  if (named) {
    if (!folders(dir).includes(named)) throw new Error(`site: no design ${named} in src/designs/`);
    const design = await load(named, dir);
    const lost = lacks(design);
    if (lost.length) throw new Error(`site: the design ${named} does not draw ${lost.join(", ")}`);
    return { name: named, design, by: env ? "env" : "pin" };
  }
  const whole: Drawn[] = [];
  for (const name of folders(dir)) {
    const design = await load(name, dir).catch(() => null);
    if (design && !lacks(design).length) whole.push({ name, design, by: "draw" });
  }
  if (!whole.length) throw new Error("site: no folder of src/designs/ draws every required part");
  return whole[Math.floor(pick() * whole.length)]!;
}

let drawn: Promise<Drawn> | null = null;

export const draw = () => (drawn ??= choose());

/* MODULES */

export const source = (name: string) =>
  `import * as one from ${JSON.stringify(index(name))};\nexport const design = one[${JSON.stringify(name)}] ?? one.default;`;

export const pulldown = (name: string, file?: string) =>
  file ? `export { pulldown } from ${JSON.stringify(join(DESIGNS, name, file))};` : "export const pulldown = null;";

const sheet = (name: string, file: string) => `@import ${JSON.stringify(join(DESIGNS, name, file))};`;

const rule = (selector: string, tokens: Tokens, pad = "") => {
  const lines = Object.entries(tokens).map(([name, value]) => `${pad}  ${name}: ${value};`);
  return lines.length ? `${pad}${selector} {\n${lines.join("\n")}\n${pad}}` : "";
};

export function skinned({ light, dark }: Design["skin"]): string {
  const night = rule(':root:not([data-theme="light"])', dark, "  ");
  return [
    rule(":root", light),
    night && `@media (prefers-color-scheme: dark) {\n${night}\n}`,
    rule(':root[data-theme="dark"]', dark),
  ]
    .filter(Boolean)
    .join("\n\n");
}

export function sheets(
  name: string,
  css: string[],
  skin: Design["skin"] = { light: {}, dark: {} },
): Record<string, string> {
  const slot = (one: string) => (css.includes(one) ? sheet(name, one) : "");
  return {
    "site:design.css": css
      .filter((one) => !SLOTS.includes(one))
      .map((one) => sheet(name, one))
      .join("\n"),
    ...Object.fromEntries(SLOTS.map((one) => [`site:design/${one}`, slot(one)])),
    "site:design/tokens.css": [slot("tokens.css"), skinned(skin)].filter(Boolean).join("\n"),
  };
}

export const runtime = (name: string, file?: string) =>
  Bun.plugin({
    name: "design",
    setup(build) {
      build.module("site:design", () => ({ contents: source(name), loader: "js" }));
      build.module("site:design/pulldown", () => ({ contents: pulldown(name, file), loader: "js" }));
    },
  });
