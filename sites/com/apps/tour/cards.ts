import type { Raw } from "@mrlyprod/ui/kit/spec.ts";
import type { Unit } from "@mrlyprod/ui/kit/units/index.ts";
import type { Sheet } from "../shared/sequences/design.ts";
import { sheet } from "../shared/sequences/design.ts";
import type { Key, Ledger, Match, Mrly, Row, Slug } from "../shared/sequences/engine.ts";
import { closed, DEEP, identify, ledgered } from "../shared/sequences/engine.ts";
import { RECORDS } from "../shared/sequences/facts.ts";

export type Num = Unit<"num">;

export type Kit = { math: Mrly; num: Num; book: Ledger };

export type Strip = { name: string; sheets: Sheet[] };

export type Art =
  | { kind: "sheet"; sheet: Sheet; ink: string }
  | { kind: "rows"; rows: Strip[] }
  | { kind: "gallery"; sheets: Sheet[] }
  | { kind: "pins"; terms: string[]; say: string }
  | { kind: "dots"; points: number[][] }
  | { kind: "farey"; nodes: [number, number, number][]; order: number };

export type ArtOf<K extends Art["kind"]> = Extract<Art, { kind: K }>;

export type Line = {
  label: string;
  terms: string[];
  closed: string;
  capped?: boolean;
  ledger?: boolean;
  row?: Row;
  axis?: string;
  marks?: boolean[];
};

export type Made = { art: Art; lines: Line[]; sub?: string };

export type Card = {
  key: string;
  title: string;
  label: string;
  min: number;
  max: number;
  first: number;
  show?: (k: number) => string;
  build: (kit: Kit, step: number) => Made;
};

export type Records = { found: Match[]; formula: string; short: boolean };

export type Told = Line & { records: Records; link: string };

export type Built = { art: Art | null; sub?: string; lines: Told[]; error: string };

export type Picked = { card: number; step: number };

export const WINDOW = 6;

export const NAMED = 3;

const FLAT = [
  ["low corner", "1"],
  ["tree", "3"],
  ["carpet", "7"],
  ["void", "9"],
  ["corner and centre", "11"],
  ["solid", "15"],
];

const CAPS = { flat: 5, cube: 3, cut: 6 };

const SPONGE = "23";

const GASKET = "126";

const odds = (k: number) => Array.from({ length: k }, (_, i) => 2 * i + 1);

const upto = (k: number) => Array.from({ length: k }, (_, i) => i + 1);

export const least = (n: number) => Math.max(n, NAMED);

const design = (dim: number, code: string, measure: Slug, axis: string): Key => ({
  dim,
  base: 2,
  code,
  measure,
  axis,
});

function row(book: Ledger, dim: number, code: string, measure: Slug, axis: string, count: number) {
  const one = book.row(design(dim, code, measure, axis));
  const { terms, capped } = book.read(one, count, DEEP.cells);
  return { terms, capped, closed: closed(RECORDS, one), ledger: true, row: one };
}

const item = (read: ReturnType<typeof row>, label: string): Line => ({ label, ...read, axis: read.row.axis });

const census = (math: Mrly, side: number) => math.six.census(math.six.cut_design(SPONGE, side, 1, 2), false);

function profile(math: Mrly, level: number) {
  const counts = math.three.profile(GASKET, 2, level, 2).map(BigInt);
  const lit = counts.map((v, i): [bigint, number] => [v, i]).filter(([v]) => v > 0n);
  let max = 0n;
  for (const [v] of lit) if (v > max) max = v;
  const mid = (counts.length - 1) / 2;
  const central = lit.reduce((best, [, i]) => (Math.abs(i - mid) < Math.abs(best - mid) ? i : best), lit[0]?.[1] ?? 0);
  return { max: String(max), heights: lit.length, constant: lit.every(([v]) => v === lit[0][0]), central };
}

export const CARDS: Card[] = [
  {
    key: "sides",
    title: "The odd-side law",
    label: "Side up to",
    min: 4,
    max: 8,
    first: 4,
    show: (k) => `side ${2 * k - 1}`,
    build: ({ math, book }, k) => {
      const sides = odds(k).slice(1);
      return {
        art: {
          kind: "rows",
          rows: FLAT.map(([name, code]) => ({
            name,
            sheets: sides.map((n) => sheet(math, design(2, code, "fills", "side"), n, 1)),
          })),
        },
        lines: FLAT.map(([name, code]) => item(row(book, 2, code, "fills", "side", k - 1), `${name}, code ${code}`)),
      };
    },
  },
  {
    key: "carpet",
    title: "The carpet",
    label: "Level",
    min: 1,
    max: CAPS.flat,
    first: 3,
    build: ({ math, book }, level) => ({
      art: { kind: "sheet", sheet: sheet(math, design(2, "7", "fills", "level"), 3, level), ink: "fills" },
      lines: [item(row(book, 2, "7", "fills", "level", least(level)), "filled cells")],
    }),
  },
  {
    key: "voids",
    title: "What the carpet drops",
    label: "Level",
    min: 1,
    max: CAPS.flat,
    first: 3,
    build: ({ math, book }, level) => ({
      art: { kind: "sheet", sheet: sheet(math, design(2, "7", "voids", "level"), 3, level), ink: "voids" },
      lines: [item(row(book, 2, "7", "voids", "level", least(level)), "empty cells")],
    }),
  },
  {
    key: "perimeter",
    title: "The perimeter of the carpet",
    label: "Level",
    min: 1,
    max: CAPS.flat,
    first: 3,
    build: ({ math, book }, level) => ({
      art: { kind: "sheet", sheet: sheet(math, design(2, "7", "surface", "level"), 3, level), ink: "edges" },
      lines: [item(row(book, 2, "7", "surface", "level", least(level)), "exposed edges")],
    }),
  },
  {
    key: "sponge",
    title: "The sponge",
    label: "Level",
    min: 1,
    max: CAPS.cube,
    first: 2,
    build: ({ math, book }, level) => ({
      art: { kind: "sheet", sheet: sheet(math, design(3, SPONGE, "fills", "level"), 3, level), ink: "fills" },
      lines: [item(row(book, 3, SPONGE, "fills", "level", least(level)), "filled cells")],
    }),
  },
  {
    key: "surface",
    title: "The surface of the sponge",
    label: "Level",
    min: 1,
    max: CAPS.cube,
    first: 2,
    build: ({ math, book }, level) => ({
      art: { kind: "sheet", sheet: sheet(math, design(3, SPONGE, "profills", "level"), 3, level), ink: "fills" },
      lines: [item(row(book, 3, SPONGE, "surface", "level", least(level)), "exposed faces")],
    }),
  },
  {
    key: "tile",
    title: "The odd sponge tile",
    label: "Side",
    min: 2,
    max: 8,
    first: 3,
    show: (k) => `side ${2 * k - 1}`,
    build: ({ math, book }, k) => ({
      art: { kind: "sheet", sheet: sheet(math, design(3, SPONGE, "fills", "side"), 2 * k - 1, 1), ink: "fills" },
      lines: [
        item(row(book, 3, SPONGE, "fills", "side", least(k - 1)), "filled cells"),
        item(row(book, 3, SPONGE, "voids", "side", least(k - 1)), "empty cells"),
      ],
    }),
  },
  {
    key: "designs",
    title: "How many designs",
    label: "Base up to",
    min: NAMED + 1,
    max: 6,
    first: 4,
    build: ({ math }, top) => {
      const lines: Line[] = [{ label: "base 2, by dimension", terms: math.bang.baseq.sequence(2, 4), closed: "" }];
      ["a line", "a square", "a cube"].forEach((name, i) => {
        const terms: string[] = [];
        for (let q = 2; q <= top; q++) {
          try {
            terms.push(math.bang.baseq.sequence(q, i + 1)[i]);
          } catch {
            break;
          }
        }
        lines.push({ label: `${name}, by base from 2`, terms, closed: "", capped: terms.length < top - 1 });
      });
      return {
        art: {
          kind: "gallery",
          sheets: math.bang.universe_codes(2).map((code) => sheet(math, design(2, code, "fills", "level"), 3, 2)),
        },
        lines,
      };
    },
  },
  {
    key: "classes",
    title: "How many fractals",
    label: "Dimension",
    min: NAMED,
    max: 8,
    first: 4,
    build: ({ math }, dims) => {
      const terms = math.bang.baseq.class_sequence(dims);
      return {
        art: { kind: "pins", terms, say: "by dimension" },
        lines: [{ label: "fill classes, by dimension", terms, closed: "" }],
      };
    },
  },
  {
    key: "slices",
    title: "The middle slice of the sponge",
    label: "Level",
    min: 1,
    max: CAPS.cube,
    first: 2,
    build: ({ math, book }, level) => {
      const read = row(book, 3, SPONGE, "triangles", "level", least(level - 1) + 1);
      return {
        art: { kind: "sheet", sheet: sheet(math, design(3, SPONGE, "triangles", "level"), 3, level), ink: "fills" },
        lines: [
          {
            ...read,
            label: "filled triangles, from the unit hexagon",
            terms: [String(census(math, 1).fills), ...read.terms],
            closed: "",
          },
        ],
      };
    },
  },
  {
    key: "vertices",
    title: "The vertices of the slice",
    label: "Side",
    min: 2,
    max: 8,
    first: 4,
    show: (k) => `side ${2 * k - 1}`,
    build: ({ math, num, book }, k) => {
      const terms = odds(least(k)).map((n) => String(census(math, n).vertices));
      return {
        art: { kind: "sheet", sheet: sheet(math, design(3, SPONGE, "triangles", "side"), 2 * k - 1, 1), ink: "fills" },
        lines: [
          { label: "vertices, by odd side", terms, marks: terms.map((t) => num.prime.is_prime(Number(t))), closed: "" },
          item(row(book, 2, "11", "fills", "side", least(k - 1)), "the corner-and-centre fills, by odd side"),
        ],
      };
    },
  },
  {
    key: "gasket",
    title: "The gasket cut",
    label: "Level",
    min: 1,
    max: CAPS.cut,
    first: 4,
    build: ({ math }, level) => {
      const profiles = upto(least(level)).map((j) => profile(math, j));
      const last = profiles[level - 1];
      return {
        art: {
          kind: "dots",
          points: [...math.three.diagonal_slice(GASKET, 2, level, 2, last.central)].map((p) => [...p]),
        },
        sub: `${last.heights} heights, ${last.constant ? "all alike" : "not all alike"}`,
        lines: [{ label: "points on a diagonal plane, by level", terms: profiles.map((p) => p.max), closed: "" }],
      };
    },
  },
  {
    key: "farey",
    title: "The Farey nodes",
    label: "Order",
    min: 1,
    max: 24,
    first: 12,
    build: ({ num }, order) => {
      const lit = upto(least(order)).map((j) => num.lattice.farey(j).length);
      const fresh = upto(order).reduce((sum, j) => sum + num.lattice.new_nodes(j), 0n);
      const primes = upto(order).filter((j) => num.prime.is_prime(j));
      return {
        art: { kind: "farey", nodes: num.lattice.farey(order).map((one) => [one.num, one.den, one.brightness]), order },
        sub: `${lit[order - 1]} nodes lit, ${BigInt(lit[order - 1]) === fresh ? "as the totients say" : "against the totients"}, primes ${primes.join(" ") || "none yet"}`,
        lines: [{ label: "nodes lit, by order", terms: lit.map(String), closed: "" }],
      };
    },
  },
];

export const clamp = (card: number, step: unknown) => {
  const one = CARDS[card];
  const n = Math.round(Number(step));
  return Number.isFinite(n) ? Math.max(one.min, Math.min(one.max, n)) : one.first;
};

export const pick = (value: Raw): Picked => {
  const n = Math.round(Number(value.card));
  const card = Number.isFinite(n) ? Math.max(0, Math.min(CARDS.length - 1, n)) : 0;
  return { card, step: clamp(card, value.step) };
};

export const shown = (card: number, step: number) => (CARDS[card].show ? CARDS[card].show(step) : String(step));

export function records(terms: string[]): Records {
  if (terms.length < NAMED) return { found: [], formula: "", short: true };
  const found = identify(RECORDS, terms.slice(0, WINDOW)).slice(0, 3);
  return { found, formula: found[0]?.record.formula ?? "", short: false };
}

export function build(units: Kit, card: number, step: number): Built {
  const one = CARDS[card];
  try {
    const made = one.build(units, step);
    return {
      ...made,
      lines: made.lines.map((line) => ({
        ...line,
        records: records(line.terms),
        link: line.ledger ? ledgered(line.terms) : "",
      })),
      error: "",
    };
  } catch (error) {
    return {
      art: null,
      lines: [],
      error: String(
        (typeof error === "object" && error !== null && "message" in error ? error.message : undefined) ?? error,
      ),
    };
  }
}
