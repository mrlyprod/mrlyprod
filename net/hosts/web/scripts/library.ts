import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { REPO } from "./home.ts";

/* TYPES */

export type Full = {
  key: string;
  fate: string;
  class: string;
  family: string;
  gen: number;
  period?: number;
  pop: { last: number; max: [number, number]; [curve: string]: unknown };
  [reading: string]: unknown;
};

export type Extract = { shelves: number; per_cell: number; rows: Full[] };

export type Row = {
  key: string;
  fate: string;
  class: string;
  family: string;
  gen: number;
  period: number;
  pop: { last: number; max: [number, number] };
};

export type Library = { shelves: number; per_cell: number; rows: Row[] };

/* PATHS */

const repo = REPO;
const FROM = join(repo, "data", "research", "automata", "lab", "rs", "encyclopedia", "library.json");
const TO = join(repo, "apps", "life", "library.gen.json");
const PER_CELL = 4;

/* TRIM */

const cell = (row: Full) => `${row.fate}\n${row.class}\n${row.family}`;

export function trim(extract: Extract, perCell = PER_CELL): Library {
  const held = new Map<string, number>();
  const rows: Row[] = [];
  for (const row of extract.rows) {
    const at = cell(row);
    const n = held.get(at) ?? 0;
    if (n >= perCell) continue;
    held.set(at, n + 1);
    rows.push({
      key: row.key,
      fate: row.fate,
      class: row.class,
      family: row.family,
      gen: row.gen,
      period: row.period ?? 0,
      pop: { last: row.pop.last, max: row.pop.max },
    });
  }
  return { shelves: held.size, per_cell: perCell, rows };
}

export function write(from = FROM, to = TO, perCell = PER_CELL): Library & { bytes: number } {
  const library = trim(JSON.parse(readFileSync(from, "utf8")), perCell);
  const text = `${JSON.stringify(library)}\n`;
  mkdirSync(dirname(to), { recursive: true });
  writeFileSync(to, text);
  return { ...library, bytes: Buffer.byteLength(text) };
}

/* MAIN */

if (import.meta.main) {
  const { shelves, rows, bytes } = write();
  console.log(`library: wrote apps/life/library.gen.json, ${shelves} shelf cells, ${rows.length} rows, ${bytes} bytes`);
}
