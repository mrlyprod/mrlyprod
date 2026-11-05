import { basename, join, relative, resolve } from "node:path";
import type { Design, Tokens } from "../../shell/design.ts";
import { lacks } from "../../shell/design.ts";
import { HOME, REPO, SITE } from "./home.ts";
import { packages } from "./packages.ts";

Bun.plugin(packages);

export const LISTED = (SITE.designs ?? []).map((one) => resolve(HOME, one));

export const SLOTS = ["tokens.css", "saver.css"];

export type Drawn = { name: string; design: Design; by: "env" | "pin" };

export const named = () => process.env.MRLY_DESIGN || (LISTED[0] ? basename(LISTED[0]) : "");

export function folder(name: string): string {
  const dir = LISTED.find((one) => basename(one) === name);
  if (!dir)
    throw new Error(
      `site: no design ${name || "at all"}; site.json designs lists ${LISTED.map((one) => basename(one)).join(", ")}`,
    );
  return dir;
}

const index = (name: string) => join(folder(name), "index.ts");

export async function load(name: string): Promise<Design> {
  const one = (await import(index(name))) as Record<string, Design | undefined>;
  const found = one[name] ?? one.default;
  if (!found) throw new Error(`site: ${relative(REPO, index(name))} exports no design named ${name}`);
  return found;
}

export async function choose(name = named()): Promise<Drawn> {
  const design = await load(name);
  const lost = lacks(design);
  if (lost.length) throw new Error(`site: the design ${name} does not draw ${lost.join(", ")}`);
  return { name, design, by: process.env.MRLY_DESIGN ? "env" : "pin" };
}

let drawn: Promise<Drawn> | null = null;

export const draw = () => (drawn ??= choose());

export const source = (name: string) =>
  `import * as one from ${JSON.stringify(index(name))};\nexport const design = one[${JSON.stringify(name)}] ?? one.default;`;

const at = (name: string, file: string) => (file.startsWith("@mrlyprod/") ? file : join(folder(name), file));

export const pulldown = (name: string, file?: string) =>
  file ? `export { pulldown } from ${JSON.stringify(at(name, file))};` : "export const pulldown = null;";

const sheet = (name: string, file: string) => `@import ${JSON.stringify(at(name, file))};`;

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
