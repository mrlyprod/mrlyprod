import type * as mrly from "@mrlyprod/mrly/math";

/* TYPES */

export type Mrly = typeof mrly;

export type Slug =
  | "fills"
  | "voids"
  | "surface"
  | "peak"
  | "heights"
  | "vertices"
  | "edges"
  | "faces"
  | "euler"
  | "triangles"
  | "cutvoids"
  | "profills"
  | "provoids"
  | "holes"
  | "pieces";

export type Measure = {
  slug: Slug;
  label: string;
  dims: number[];
  base?: number;
  kind: "closed" | "profile" | "grid" | "cut";
};

export type Axis = {
  slug: string;
  label: string;
  start: number;
  step: number;
  first: number;
  say: string;
  at: (number: number) => string;
};

export type Key = { dim: number; base: number; code: string; measure: Slug; axis: string };

export type Entry = {
  id: string;
  name: string;
  offset: number;
  terms: string[];
  key: string;
  shift: number;
  status: string;
  formula: string;
};

export type Match = { record: Entry; at: number };

export type Hit = { id: string; name: string; at: number; shift: number; status: string };

export type Design = { code: string; name: string; label: string; cells: number[] };

export type Row = Key & {
  key: string;
  name: string;
  label: string;
  cells: number[];
  file: string;
  terms: string[];
  stop: number;
  error: string;
  hit?: Hit | null;
  hitAt?: number;
};

export type Finders = {
  found?: (row: Row) => Hit | null | undefined;
  typed?: (code: string) => Row[];
  named?: (needle: string) => Row[];
};

type Census = { vertices: number; edges: number; faces?: number; euler: number };

type Names = { exact: Map<number, string>; orbits: Map<number, string> };

export const SPACES: [number, number][] = [
  [2, 2],
  [2, 3],
  [3, 2],
];

export const MEASURES: Measure[] = [
  { slug: "fills", label: "Fills", dims: [2, 3], kind: "closed" },
  { slug: "voids", label: "Voids", dims: [2, 3], kind: "closed" },
  { slug: "surface", label: "Surface", dims: [2, 3], kind: "closed" },
  { slug: "peak", label: "Peak", dims: [2, 3], kind: "profile" },
  { slug: "heights", label: "Heights", dims: [2, 3], kind: "profile" },
  { slug: "vertices", label: "Vertices", dims: [2, 3], kind: "grid" },
  { slug: "edges", label: "Edges", dims: [2, 3], kind: "grid" },
  { slug: "faces", label: "Faces", dims: [3], kind: "grid" },
  { slug: "euler", label: "Euler", dims: [2, 3], kind: "grid" },
  { slug: "triangles", label: "Cut fills", dims: [3], base: 2, kind: "cut" },
  { slug: "cutvoids", label: "Cut voids", dims: [3], base: 2, kind: "cut" },
  { slug: "profills", label: "Pro fills", dims: [3], base: 2, kind: "cut" },
  { slug: "provoids", label: "Pro voids", dims: [3], base: 2, kind: "cut" },
  { slug: "holes", label: "Cut holes", dims: [3], base: 2, kind: "grid" },
  { slug: "pieces", label: "Cut pieces", dims: [3], base: 2, kind: "grid" },
];

export const AXES: Axis[] = [
  {
    slug: "level",
    label: "Level",
    start: 1,
    step: 1,
    first: 1,
    say: "level L",
    at: (number: number) => `level L at side ${number}`,
  },
  {
    slug: "side",
    label: "Side",
    start: 3,
    step: 2,
    first: 2,
    say: "odd side 2k - 1",
    at: () => "odd side 2k - 1 at level 1",
  },
];

export const TABLE = { terms: 8, cells: 30000 };

export const DEEP = { cells: 500000 };

export const TERMS = { min: 4, max: 32 };

export const MOST = 6;

export const CANON = "2-2-7-fills-side";

const LEAD = 4;

const SOLID: Record<number, number> = { 2: 15, 3: 255 };

const PROFILE = 4;

const blank = (one: { code: string }) => Number(one.code) === 0;

export const measureOf = (slug: string | undefined) => MEASURES.find((one) => one.slug === slug);

export const axisOf = (slug: string) => AXES.find((one) => one.slug === slug) ?? AXES[0];

export const spaces = (dim: number | string, base: number | string) =>
  SPACES.filter(([d, b]) => (!dim || d === Number(dim)) && (!base || b === Number(base)));

export function applies(one: Measure, dim: number | string, base: number | string): boolean {
  return spaces(dim, base).some(([d, b]) => one.dims.includes(d) && (!one.base || one.base === b));
}

export const number = (row: { base: number }) => Math.max(row.base, 3);

export const spot = (row: { axis: string; base: number }, index: number) =>
  row.axis === "level" ? { number: number(row), level: index + 1 } : { number: 2 * index + 3, level: 1 };

/* KEYS */

export const print = ({ dim, base, code, measure: m, axis }: Key) => `${dim}-${base}-${code}-${m}-${axis}`;

export function parse(key: string): Key | null {
  const [dim, base, code, m, axis] = String(key).split("-");
  const d = Number(dim);
  const b = Number(base);
  if (
    !SPACES.some(([x, y]) => x === d && y === b) ||
    !/^\d+$/.test(code ?? "") ||
    !measureOf(m) ||
    !AXES.some((one) => one.slug === axis)
  )
    return null;
  if (!applies(measureOf(m)!, d, b)) return null;
  return { dim: d, base: b, code, measure: m as Slug, axis };
}

export function picks(text: string | undefined): string[] {
  return [
    ...new Set(
      String(text ?? "")
        .split(".")
        .map((one) => one.trim())
        .filter((one) => parse(one)),
    ),
  ].slice(0, MOST);
}

export const plotted = (row: Key, count: number) =>
  `../plot/?dim=${row.dim}&base=${row.base}&code=${row.code}&measure=${row.measure}&axis=${row.axis}&terms=${count}`;

export const ledgered = (terms: string[]) => `../sequences/?q=${encodeURIComponent(terms.join(", "))}`;

/* NUMBERS */

export function numbers(text: string | undefined): string[] | null {
  const tokens = String(text ?? "")
    .split(/[\s,]+/)
    .filter(Boolean);
  if (!tokens.length || !tokens.every((one) => /^-?\d+$/.test(one))) return null;
  return tokens.map((one) => String(BigInt(one)));
}

function window(haystack: string[], needle: string[]): number {
  if (!needle.length || haystack.length < needle.length) return -1;
  for (let at = 0; at + needle.length <= haystack.length; at++) {
    let ok = true;
    for (let i = 0; i < needle.length && ok; i++) ok = haystack[at + i] === needle[i];
    if (ok) return at;
  }
  return -1;
}

/* RECORDS */

export function identify(records: Entry[], terms: string[]): Match[] {
  const found: Match[] = [];
  for (const record of records) {
    const at = window(record.terms, terms);
    if (at >= 0) found.push({ record, at: record.offset + at });
  }
  return found.sort(
    (a, b) =>
      (a.record.key ? 0 : 1) - (b.record.key ? 0 : 1) ||
      Math.abs(a.at) - Math.abs(b.at) ||
      a.record.id.localeCompare(b.record.id),
  );
}

export function badge(records: Entry[], row: Pick<Row, "file" | "axis">, terms: string[]): Hit | null {
  if (terms.length < 4 || terms.every((one) => one === terms[0])) return null;
  const found = identify(records, terms);
  if (!found.length) return null;
  const own = found.find(({ record }) => record.key && record.key === row.file);
  const { record, at } = own ?? found[0];
  return {
    id: record.id,
    name: record.name,
    at,
    shift: at - axisOf(row.axis).first,
    status: own ? record.status : "Collision",
  };
}

export const closed = (records: Entry[], row: Pick<Row, "file">) =>
  records.find((record) => record.key && record.key === row.file)?.formula ?? "";

export const keyed = (records: Entry[], needle: string) =>
  records.filter((record) => record.key && `${record.id} ${record.name}`.toLowerCase().includes(needle));

/* LEDGER */

const strip = (word: string) => (/^[hvxyz](tree|line)$/.test(word) ? word.slice(1) : word);

export function ledger(math: Mrly) {
  const census = new Map<string, Census>();
  const slices = new Map<string, mrly.Cell6d>();
  const profiles = new Map<string, bigint[]>();
  const rows = new Map<string, Row>();
  const titles = new Map<number, Names>();
  const books = new Map<string, Design[]>();
  const shelves = new Map<string, Row[]>();
  const swept = new Map<string, number>();
  const tile = (dim: number, base: number, code: string, side: number, level: number): mrly.Cell =>
    dim === 2 ? math.two.create(code, side, level, 0, base) : math.three.create(code, side, level, base);
  const memo = <K, T>(map: Map<K, T>, id: K, make: () => T): T => {
    const held = map.get(id);
    if (held !== undefined) return held;
    const made = make();
    map.set(id, made);
    return made;
  };
  const names = (dim: number) =>
    memo(titles, dim, (): Names => {
      const total = 1 << (1 << dim);
      const cells = Array.from({ length: total }, (_, code) => tile(dim, 2, String(code), 3, 1).types.join(""));
      const named = [
        ...math.bang.sources("Classics", dim).flatMap((one) => ("design" in one ? [one.design] : [])),
        ...math.bang.catalog.antis(dim),
      ];
      const exact = new Map([
        [0, "empty"],
        [SOLID[dim], "solid"],
      ]);
      const orbits = new Map(exact);
      for (const name of named) {
        const drawn = (dim === 2 ? math.two.named(name, 3, 1, 0) : math.three.named(name, 3, 1)).types.join("");
        const code = cells.indexOf(drawn);
        const canon = Math.min(...math.bang.universe.orbit(String(code), dim).map(Number));
        const word = strip(name.toLowerCase());
        exact.set(code, word);
        if (code === canon || !orbits.has(canon)) orbits.set(canon, word);
      }
      return { exact, orbits };
    });
  const word = (dim: number, base: number, code: string): string => {
    if (base !== 2) return "";
    const { exact, orbits } = names(dim);
    return (
      exact.get(Number(code)) ?? orbits.get(Math.min(...math.bang.universe.orbit(String(code), dim).map(Number))) ?? ""
    );
  };
  const design = (dim: number, base: number, code: string): Design => {
    const name = word(dim, base, code);
    return {
      code: String(code),
      name,
      label: name ? `${name} ${code}` : `code ${code}`,
      cells: [...tile(dim, base, String(code), 3, 1).types],
    };
  };
  const designs = (dim: number, base: number) =>
    memo(books, `${dim}-${base}`, (): Design[] => {
      const codes =
        base === 2 ? math.bang.universe_codes(dim) : math.bang.baseq.representatives(base, dim).map(([code]) => code);
      const made = codes.map((code) => design(dim, base, code));
      return [...made.filter((one) => !blank(one)), ...made.filter(blank)];
    });
  const holds = (dim: number, base: number, code: string) => {
    try {
      math.counts.fill(String(code), 3, dim, 1, base);
      return true;
    } catch {
      return false;
    }
  };
  const space = (dim: number, base: number) =>
    memo(shelves, `${dim}-${base}`, () => {
      const out: Row[] = [];
      for (const one of designs(dim, base)) out.push(...lines(dim, base, one.code));
      return out;
    });
  const keys = (dim: number | string, base: number | string) => spaces(dim, base).flatMap(([d, b]) => space(d, b));
  const lines = (dim: number, base: number, code: string): Row[] => {
    const out: Row[] = [];
    for (const one of MEASURES) {
      if (!applies(one, dim, base)) continue;
      for (const axis of AXES) out.push(row({ dim, base, code: String(code), measure: one.slug, axis: axis.slug }));
    }
    return out;
  };
  const typed = (dim: number | string, base: number | string, code: string) =>
    /^\d+$/.test(String(code))
      ? spaces(dim, base).flatMap(([d, b]) => (holds(d, b, code) ? lines(d, b, code) : []))
      : [];
  const row = (key: Key) =>
    memo(rows, print(key), (): Row => {
      const { dim, base, code, measure: m, axis } = key;
      const { name, label, cells } = design(dim, base, code);
      return {
        key: print(key),
        dim,
        base,
        code,
        measure: m,
        axis,
        name,
        label,
        cells,
        file: new math.name.Sequence(code, dim, base, m, axis).to_file(),
        terms: [],
        stop: 0,
        error: "",
      };
    });
  const file = (text: string) => {
    try {
      const named = math.name.Sequence.from_file(String(text).trim()).toJSON();
      const parsed = parse(`${named.dim}-${named.base ?? 2}-${named.code}-${named.measure}-${named.axis}`);
      return parsed ? row(parsed) : null;
    } catch {
      return null;
    }
  };
  const cost = (one: Row, index: number) => {
    const kind = measureOf(one.measure)!.kind;
    const { number: side, level } = spot(one, index);
    if (kind === "closed") return 0;
    if (kind === "profile") return (one.dim * (side ** level - 1) + 1) / PROFILE;
    if (kind === "cut") return side ** (2 * level);
    return side ** (one.dim * level);
  };
  const tally = (one: Row, side: number, level: number) =>
    memo(census, `${one.dim}-${one.base}-${one.code}-${side}-${level}`, (): Census => {
      const cell = tile(one.dim, one.base, one.code, side, level);
      return one.dim === 2 ? math.two.census(cell) : math.three.census(cell);
    });
  const slice = (one: Row, side: number, level: number) =>
    memo(slices, `${one.code}-${side}-${level}`, () => math.six.cut_design(one.code, side, level, one.base));
  const profile = (one: Row, side: number, level: number) =>
    memo(profiles, `${one.dim}-${one.base}-${one.code}-${side}-${level}`, () =>
      math.counts.profile_of_tile(math.bang.factory.create(one.code, side, one.dim, one.base, 1), level).map(BigInt),
    );
  const term = (one: Row, index: number): string => {
    const { dim, base, code } = one;
    const { number: side, level } = spot(one, index);
    switch (one.measure) {
      case "fills":
        return math.counts.fill(code, side, dim, level, base);
      case "voids":
        return math.counts.void_(code, side, dim, level, base);
      case "surface":
        return math.counts.exposure(code, side, dim, level, base);
      case "peak": {
        let top = 0n;
        for (const v of profile(one, side, level)) if (v > top) top = v;
        return top.toString();
      }
      case "heights":
        return String(profile(one, side, level).filter((v) => v > 0n).length);
      case "triangles":
        return math.counts.cut_fills(code, side, level);
      case "cutvoids":
        return math.counts.cut_voids(code, side, level);
      case "profills":
        return math.counts.pro_fills(code, side, level);
      case "provoids":
        return math.counts.pro_voids(code, side, level);
      case "holes":
        return String(math.six.holes(slice(one, side, level)));
      case "pieces":
        return String(math.six.components(slice(one, side, level)));
      default:
        return String(tally(one, side, level)[one.measure]);
    }
  };
  const step = (one: Row, count: number, cells: number) => {
    const index = one.terms.length;
    if (index >= count || one.stop === Infinity) return false;
    const price = cost(one, index);
    if (price > cells) {
      one.stop = price;
      return false;
    }
    try {
      one.terms.push(term(one, index));
      one.stop = index + 1 < count ? cost(one, index + 1) : 0;
    } catch (error) {
      one.stop = Infinity;
      one.error = String((error as { message?: unknown } | null | undefined)?.message ?? error);
    }
    return true;
  };
  const capped = (one: Row, count: number, cells: number) =>
    one.terms.length < count && (one.stop === Infinity || one.stop > cells);
  const find = (key: string) => {
    const parsed = parse(key);
    if (!parsed) throw new Error(`sequences: bad key ${key}`);
    return row(parsed);
  };
  const read = (key: string | Row, count: number, cells: number) => {
    const one = typeof key === "string" ? find(key) : key;
    while (step(one, count, cells));
    return { terms: one.terms.slice(0, count), capped: capped(one, count, cells) };
  };
  const sweep = (dim: number | string, base: number | string, count = TABLE.terms, cells = TABLE.cells) => {
    for (const [d, b] of spaces(dim, base)) {
      const id = `${d}-${b}`;
      const list = space(d, b);
      let at = swept.get(id) ?? 0;
      while (at < list.length) {
        if (step(list[at], count, cells)) {
          swept.set(id, at);
          return true;
        }
        at++;
      }
      swept.set(id, at);
    }
    return false;
  };
  const progress = (dim: number | string, base: number | string) =>
    spaces(dim, base).reduce(
      (sum, [d, b]) => {
        const total = space(d, b).length;
        return { done: sum.done + Math.min(swept.get(`${d}-${b}`) ?? 0, total), total: sum.total + total };
      },
      { done: 0, total: 0 },
    );
  return { designs, keys, typed, row, file, read, step, sweep, progress, capped };
}

export type Ledger = ReturnType<typeof ledger>;

/* LEAD */

export const flat = (terms: string[]) => terms.length > 1 && terms.every((one) => one === terms[0]);

export function lead(book: Pick<Ledger, "read">, list: Row[], at: number, not = ""): Row | null {
  for (let i = 0; i < list.length; i++) {
    const row = list[(at + i) % list.length];
    if (row.key !== not && !blank(row) && !flat(book.read(row, LEAD, TABLE.cells).terms)) return row;
  }
  for (let i = 0; i < list.length; i++) {
    const row = list[(at + i) % list.length];
    if (row.key !== not) return row;
  }
  return null;
}

/* SEARCH */

const mention = (row: Row, hit: Hit | null | undefined) =>
  `${row.label} ${row.measure} ${row.axis} ${row.dim}d base ${row.base} ${row.file} ${hit ? `${hit.id} ${hit.name}` : ""}`.toLowerCase();

export function search(
  list: Row[],
  q: string | undefined,
  { found = () => null, typed = () => [], named = () => [] }: Finders = {},
): Row[] {
  const text = String(q ?? "").trim();
  if (!text) return list;
  const terms = numbers(text);
  if (terms && terms.length > 1) return list.filter((row) => window(row.terms, terms) >= 0);
  const needle = text.toLowerCase();
  const own = terms ? typed(terms[0]) : named(needle);
  const keys = new Set(own.map((row) => row.key));
  return [...own, ...list.filter((row) => !keys.has(row.key) && mention(row, found(row)).includes(needle))];
}

export function filter(list: Row[], measureSlug: string, axisSlug: string): Row[] {
  return list.filter((row) => (!measureSlug || row.measure === measureSlug) && (!axisSlug || row.axis === axisSlug));
}

/* SHEETS */

const shown = (row: Row) => ({
  name: row.file,
  design: row.name,
  code: row.code,
  dim: row.dim,
  base: row.base,
  measure: row.measure,
  axis: row.axis,
  start: axisOf(row.axis).start,
  step: axisOf(row.axis).step,
  record: row.hit?.id ?? "",
  at: row.hit?.at ?? null,
  status: row.hit?.status ?? "",
  terms: row.terms,
});

export function json(list: Row[]): string {
  return JSON.stringify(list.map(shown), null, 2);
}

export function csv(list: Row[]): string {
  const head = "name,design,code,dim,base,measure,axis,start,step,record,status,terms";
  const lines = list
    .map(shown)
    .map((one) =>
      [
        one.name,
        one.design,
        one.code,
        one.dim,
        one.base,
        one.measure,
        one.axis,
        one.start,
        one.step,
        one.record,
        one.status,
        `"${one.terms.join(", ")}"`,
      ].join(","),
    );
  return [head, ...lines].join("\n");
}
