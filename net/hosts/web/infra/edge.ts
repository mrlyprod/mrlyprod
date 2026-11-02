import { readFileSync } from "node:fs";
import { join } from "node:path";
import { HOME } from "../scripts/home.ts";

export type Step = { uri?: string; redirect?: string };

export const MOVED = "/*MOVED*/{}";

export const LOCK = "/*LOCK*/";

export const HEAD = "import cf from 'cloudfront';\nconst crypto = require('crypto');\nconst kvs = cf.kvs();\n";

const read = (name: string) => readFileSync(join(import.meta.dir, name), "utf8");

export const plain = (text: string) => text.replace(/^export /gm, "");

export const body = (moved: Record<string, string>, locked: boolean) =>
  read("edge.js")
    .replace(MOVED, JSON.stringify(moved))
    .replace(LOCK, locked ? plain(read("guard.js")) : "");

export const code = (moved: Record<string, string>, locked: boolean) => (locked ? HEAD : "") + body(moved, locked);

const rows = JSON.parse(readFileSync(join(HOME, "redirects.json"), "utf8")) as Record<string, { to: string }>;

const table = Object.fromEntries(Object.entries(rows).map(([from, row]) => [from, row.to]));

export const { decide, join: joined } = new Function(`${body(table, false)}\nreturn { decide, join };`)() as {
  decide: (uri: string) => Step;
  join: (to: string, tail: string) => string;
};
