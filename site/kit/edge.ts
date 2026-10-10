import { readFileSync } from "node:fs";
import { join } from "node:path";

export type Step = { uri?: string; redirect?: string };

const MARK = "/*MOVED*/{}";

const rows = JSON.parse(readFileSync(join(import.meta.dir, "..", "redirects.json"), "utf8")) as Record<string, { to: string }>;

const table = JSON.stringify(Object.fromEntries(Object.entries(rows).map(([from, row]) => [from, row.to])));

const text = readFileSync(join(import.meta.dir, "edge.js"), "utf8");

export const decide = new Function(`${text.replace(MARK, table)}\nreturn decide;`)() as (uri: string) => Step;
