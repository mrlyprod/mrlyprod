export const SPACES = [
  [2, 2],
  [2, 3],
  [3, 2],
];

export const MEASURES = [
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

export const AXES = [
  {
    slug: "level",
    label: "Level",
    start: 1,
    step: 1,
    first: 1,
    say: "level L",
    at: (number) => `level L at side ${number}`,
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

const SOLID = { 2: 15, 3: 255 };

const PROFILE = 4;

const blank = (one) => Number(one.code) === 0;

export const measureOf = (slug) => MEASURES.find((one) => one.slug === slug);

export const axisOf = (slug) => AXES.find((one) => one.slug === slug) ?? AXES[0];

export const spaces = (dim, base) =>
  SPACES.filter(([d, b]) => (!dim || d === Number(dim)) && (!base || b === Number(base)));

export function applies(one, dim, base) {
  return spaces(dim, base).some(([d, b]) => one.dims.includes(d) && (!one.base || one.base === b));
}

export const number = (row) => Math.max(row.base, 3);

export const spot = (row, index) =>
  row.axis === "level" ? { number: number(row), level: index + 1 } : { number: 2 * index + 3, level: 1 };

/* KEYS */

export const print = ({ dim, base, code, measure: m, axis }) => `${dim}-${base}-${code}-${m}-${axis}`;

export function parse(key) {
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
  if (!applies(measureOf(m), d, b)) return null;
  return { dim: d, base: b, code, measure: m, axis };
}

export function picks(text) {
  return [
    ...new Set(
      String(text ?? "")
        .split(".")
        .map((one) => one.trim())
        .filter((one) => parse(one)),
    ),
  ].slice(0, MOST);
}

export const plotted = (row, count) =>
  `../plot/?dim=${row.dim}&base=${row.base}&code=${row.code}&measure=${row.measure}&axis=${row.axis}&terms=${count}`;

export const ledgered = (terms) => `../sequences/?q=${encodeURIComponent(terms.join(", "))}`;

/* NUMBERS */

export function numbers(text) {
  const tokens = String(text ?? "")
    .split(/[\s,]+/)
    .filter(Boolean);
  if (!tokens.length || !tokens.every((one) => /^-?\d+$/.test(one))) return null;
  return tokens.map((one) => String(BigInt(one)));
}

function window(haystack, needle) {
  if (!needle.length || haystack.length < needle.length) return -1;
  for (let at = 0; at + needle.length <= haystack.length; at++) {
    let ok = true;
    for (let i = 0; i < needle.length && ok; i++) ok = haystack[at + i] === needle[i];
    if (ok) return at;
  }
  return -1;
}

/* RECORDS */

export function identify(records, terms) {
  const found = [];
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

export function badge(records, row, terms) {
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

export const closed = (records, row) => records.find((record) => record.key && record.key === row.file)?.formula ?? "";

export const keyed = (records, needle) =>
  records.filter((record) => record.key && `${record.id} ${record.name}`.toLowerCase().includes(needle));

/* LEDGER */

const strip = (word) => (/^[hvxyz](tree|line)$/.test(word) ? word.slice(1) : word);

export function ledger(math) {
  const census = new Map();
  const slices = new Map();
  const profiles = new Map();
  const rows = new Map();
  const memos = new Map();
  const swept = new Map();
  const tile = (dim, base, code, side, level) =>
    dim === 2 ? math.two.create(code, side, level, 0, base) : math.three.create(code, side, level, base);
  const memo = (map, id, make) => {
    if (!map.has(id)) map.set(id, make());
    return map.get(id);
  };
  const names = (dim) =>
    memo(memos, `names-${dim}`, () => {
      const total = 1 << (1 << dim);
      const cells = Array.from({ length: total }, (_, code) => tile(dim, 2, String(code), 3, 1).types.join(""));
      const named = [...math.bang.sources("Classics", dim).map((one) => one.design), ...math.bang.catalog.antis(dim)];
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
  const word = (dim, base, code) => {
    if (base !== 2) return "";
    const { exact, orbits } = names(dim);
    return (
      exact.get(Number(code)) ?? orbits.get(Math.min(...math.bang.universe.orbit(String(code), dim).map(Number))) ?? ""
    );
  };
  const design = (dim, base, code) => {
    const name = word(dim, base, code);
    return {
      code: String(code),
      name,
      label: name ? `${name} ${code}` : `code ${code}`,
      cells: [...tile(dim, base, String(code), 3, 1).types],
    };
  };
  const designs = (dim, base) =>
    memo(memos, `${dim}-${base}`, () => {
      const codes =
        base === 2 ? math.bang.universe_codes(dim) : math.bang.baseq.representatives(base, dim).map(([code]) => code);
      const made = codes.map((code) => design(dim, base, code));
      return [...made.filter((one) => !blank(one)), ...made.filter(blank)];
    });
  const holds = (dim, base, code) => {
    try {
      math.counts.fill(String(code), 3, dim, 1, base);
      return true;
    } catch {
      return false;
    }
  };
  const space = (dim, base) =>
    memo(memos, `keys-${dim}-${base}`, () => {
      const out = [];
      for (const one of designs(dim, base)) out.push(...lines(dim, base, one.code));
      return out;
    });
  const keys = (dim, base) => spaces(dim, base).flatMap(([d, b]) => space(d, b));
  const lines = (dim, base, code) => {
    const out = [];
    for (const one of MEASURES) {
      if (!applies(one, dim, base)) continue;
      for (const axis of AXES) out.push(row({ dim, base, code: String(code), measure: one.slug, axis: axis.slug }));
    }
    return out;
  };
  const typed = (dim, base, code) =>
    /^\d+$/.test(String(code))
      ? spaces(dim, base).flatMap(([d, b]) => (holds(d, b, code) ? lines(d, b, code) : []))
      : [];
  const row = (key) =>
    memo(rows, print(key), () => {
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
  const file = (text) => {
    try {
      const named = math.name.Sequence.from_file(String(text).trim()).toJSON();
      const key = {
        dim: named.dim,
        base: named.base ?? 2,
        code: String(named.code),
        measure: named.measure,
        axis: named.axis,
      };
      return parse(print(key)) ? row(key) : null;
    } catch {
      return null;
    }
  };
  const cost = (one, index) => {
    const kind = measureOf(one.measure).kind;
    const { number: side, level } = spot(one, index);
    if (kind === "closed") return 0;
    if (kind === "profile") return (one.dim * (side ** level - 1) + 1) / PROFILE;
    if (kind === "cut") return side ** (2 * level);
    return side ** (one.dim * level);
  };
  const tally = (one, side, level) =>
    memo(census, `${one.dim}-${one.base}-${one.code}-${side}-${level}`, () => {
      const cell = tile(one.dim, one.base, one.code, side, level);
      return one.dim === 2 ? math.two.census(cell) : math.three.census(cell);
    });
  const slice = (one, side, level) =>
    memo(slices, `${one.code}-${side}-${level}`, () => math.six.cut_design(one.code, side, level, one.base));
  const profile = (one, side, level) =>
    memo(profiles, `${one.dim}-${one.base}-${one.code}-${side}-${level}`, () =>
      math.counts.profile_of_tile(math.bang.factory.create(one.code, side, one.dim, one.base, 1), level).map(BigInt),
    );
  const term = (one, index) => {
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
  const step = (one, count, cells) => {
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
      one.error = String(error?.message ?? error);
    }
    return true;
  };
  const capped = (one, count, cells) => one.terms.length < count && (one.stop === Infinity || one.stop > cells);
  const read = (key, count, cells) => {
    const one = typeof key === "string" ? row(parse(key)) : key;
    while (step(one, count, cells));
    return { terms: one.terms.slice(0, count), capped: capped(one, count, cells) };
  };
  const sweep = (dim, base, count = TABLE.terms, cells = TABLE.cells) => {
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
  const progress = (dim, base) =>
    spaces(dim, base).reduce(
      (sum, [d, b]) => {
        const total = space(d, b).length;
        return { done: sum.done + Math.min(swept.get(`${d}-${b}`) ?? 0, total), total: sum.total + total };
      },
      { done: 0, total: 0 },
    );
  return { designs, keys, typed, row, file, read, step, sweep, progress, capped };
}

/* LEAD */

export const flat = (terms) => terms.length > 1 && terms.every((one) => one === terms[0]);

export function lead(book, list, at, not = "") {
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

const mention = (row, hit) =>
  `${row.label} ${row.measure} ${row.axis} ${row.dim}d base ${row.base} ${row.file} ${hit ? `${hit.id} ${hit.name}` : ""}`.toLowerCase();

export function search(list, q, { found = () => null, typed = () => [], named = () => [] } = {}) {
  const text = String(q ?? "").trim();
  if (!text) return list;
  const terms = numbers(text);
  if (terms && terms.length > 1) return list.filter((row) => window(row.terms, terms) >= 0);
  const needle = text.toLowerCase();
  const own = terms ? typed(terms[0]) : named(needle);
  const keys = new Set(own.map((row) => row.key));
  return [...own, ...list.filter((row) => !keys.has(row.key) && mention(row, found(row)).includes(needle))];
}

export function filter(list, measureSlug, axisSlug) {
  return list.filter((row) => (!measureSlug || row.measure === measureSlug) && (!axisSlug || row.axis === axisSlug));
}

/* SHEETS */

const shown = (row) => ({
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

export function json(list) {
  return JSON.stringify(list.map(shown), null, 2);
}

export function csv(list) {
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
