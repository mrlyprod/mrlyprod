import { readFileSync } from "node:fs";
import { join } from "node:path";

export type Step = { uri?: string; redirect?: string };

const MOVED = "/*MOVED*/{}";

const code = (moved: Record<string, string>) =>
  readFileSync(join(import.meta.dir, "edge.js"), "utf8").replace(MOVED, JSON.stringify(moved));

export const router = (rows: Record<string, { to: string }>) => {
  const table = Object.fromEntries(Object.entries(rows).map(([from, row]) => [from, row.to]));
  return new Function(`${code(table)}\nreturn { decide, join };`)() as {
    decide: (uri: string) => Step;
    join: (to: string, tail: string) => string;
  };
};
