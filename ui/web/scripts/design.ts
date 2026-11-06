import { existsSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import type { Design, Tokens } from "../../shell/design.ts";
import { lacks, MAP } from "../../shell/design.ts";
import { HOME, REPO, SITE } from "./home.ts";
import { packages } from "./packages.ts";
import { faults } from "./stamp.ts";

Bun.plugin(packages);

export const LISTED = (SITE.designs ?? []).map((one) => resolve(HOME, one));

const SHEETS = Object.keys({ ...MAP.required, ...MAP.optional }).filter((one) => one.endsWith(".css"));

export type Drawn = { name: string; design: Design };

export function folder(name: string): string {
  const dir = LISTED.find((one) => basename(one) === name);
  if (!dir)
    throw new Error(
      `site: no design ${name || "at all"}; site.json designs lists ${LISTED.map((one) => basename(one)).join(", ")}`,
    );
  return dir;
}

export function names(): string[] {
  const all = LISTED.map((one) => basename(one));
  const lead = process.env.MRLY_DESIGN || all[0] || "";
  folder(lead);
  const twice = all.find((one, at) => all.indexOf(one) !== at);
  if (twice) throw new Error(`site: site.json designs lists ${twice} twice`);
  return [lead, ...all.filter((one) => one !== lead)];
}

const index = (name: string) => join(folder(name), MAP.index);

export async function load(name: string): Promise<Design> {
  const one = (await import(index(name))) as { design?: Design };
  if (!one.design) throw new Error(`site: ${relative(REPO, index(name))} exports no design`);
  return one.design;
}

export async function choose(list = names()): Promise<Drawn[]> {
  const off = list.flatMap((name) => faults(folder(name)));
  if (off.length) throw new Error(off.join("\n"));
  const all = await Promise.all(list.map(async (name) => ({ name, design: await load(name) })));
  const bad = all.flatMap(({ name, design }) =>
    lacks(design).map((part) => `the design ${name} does not draw ${part}`),
  );
  if (bad.length) throw new Error(`site: ${bad.join("; ")}`);
  return all;
}

let drawn: Promise<Drawn[]> | null = null;

export const draw = () => (drawn ??= choose());

export const source = (list: string[]) =>
  [
    `export const names = ${JSON.stringify(list)};`,
    `export const designs = { ${list
      .map((name) => `${JSON.stringify(name)}: () => import(${JSON.stringify(index(name))}).then((one) => one.design)`)
      .join(", ")} };`,
  ].join("\n");

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

export function sheets(name: string, skin: Design["skin"]): Record<string, string> {
  const held = Object.fromEntries(
    SHEETS.map((one) => {
      const file = join(folder(name), one);
      return [`site:design/${one}`, existsSync(file) ? `@import ${JSON.stringify(file)};` : ""];
    }),
  );
  const tokens = [held["site:design/tokens.css"], skinned(skin)].filter(Boolean).join("\n");
  return { ...held, "site:design/tokens.css": tokens };
}
