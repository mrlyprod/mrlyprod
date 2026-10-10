export const SPACES = [[2, 2], [2, 3], [3, 2]];

export const MEASURES = [
  { slug: 'fills', label: 'Fills', dims: [2, 3], kind: 'closed' },
  { slug: 'voids', label: 'Voids', dims: [2, 3], kind: 'closed' },
  { slug: 'surface', label: 'Surface', dims: [2, 3], kind: 'closed' },
  { slug: 'peak', label: 'Peak', dims: [2, 3], kind: 'profile' },
  { slug: 'heights', label: 'Heights', dims: [2, 3], kind: 'profile' },
  { slug: 'vertices', label: 'Vertices', dims: [2, 3], kind: 'grid' },
  { slug: 'edges', label: 'Edges', dims: [2, 3], kind: 'grid' },
  { slug: 'faces', label: 'Faces', dims: [3], kind: 'grid' },
  { slug: 'euler', label: 'Euler', dims: [2, 3], kind: 'grid' },
  { slug: 'triangles', label: 'Cut fills', dims: [3], base: 2, kind: 'cut' },
  { slug: 'cutvoids', label: 'Cut voids', dims: [3], base: 2, kind: 'cut' },
  { slug: 'profills', label: 'Pro fills', dims: [3], base: 2, kind: 'cut' },
  { slug: 'provoids', label: 'Pro voids', dims: [3], base: 2, kind: 'cut' },
  { slug: 'holes', label: 'Cut holes', dims: [3], base: 2, kind: 'grid' },
  { slug: 'pieces', label: 'Cut pieces', dims: [3], base: 2, kind: 'grid' },
];

export const AXES = [
  { slug: 'level', label: 'Level', start: 1, step: 1 },
  { slug: 'side', label: 'Side', start: 3, step: 2 },
];

export const TABLE = { terms: 8, cells: 30000 };

export const DEEP = { cells: 150000 };

export const MOST = 6;

const LEAD = 4;

const SOLID = { 2: 15, 3: 255 };

const PROFILE = 4;

const blank = (one) => Number(one.code) === 0;

const measure = (slug) => MEASURES.find((one) => one.slug === slug);

const axisOf = (slug) => AXES.find((one) => one.slug === slug);

export function applies(one, dim, base) {
  return one.dims.includes(dim) && (!one.base || one.base === base);
}

/* KEYS */

export const print = ({ dim, base, code, measure: m, axis }) => `${dim}-${base}-${code}-${m}-${axis}`;

export function parse(key) {
  const [dim, base, code, m, axis] = String(key).split('-');
  const d = Number(dim);
  const b = Number(base);
  if (!SPACES.some(([x, y]) => x === d && y === b) || !/^\d+$/.test(code ?? '') || !measure(m) || !axisOf(axis)) return null;
  if (!applies(measure(m), d, b)) return null;
  return { dim: d, base: b, code, measure: m, axis };
}

export function picks(text) {
  return [...new Set(String(text ?? '').split('.').map((one) => one.trim()).filter((one) => parse(one)))].slice(0, MOST);
}

export const place = (axis, index) => (axis === 'level' ? { number: null, level: index + 1 } : { number: 2 * index + 3, level: 1 });

/* NUMBERS */

export function numbers(text) {
  const tokens = String(text ?? '').split(/[\s,]+/).filter(Boolean);
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
  return found.sort((a, b) => (a.record.key ? 0 : 1) - (b.record.key ? 0 : 1) || Math.abs(a.at) - Math.abs(b.at) || a.record.id.localeCompare(b.record.id));
}

export function badge(records, row, terms) {
  if (terms.length < 4 || terms.every((one) => one === terms[0])) return null;
  const found = identify(records, terms);
  if (!found.length) return null;
  const own = found.find(({ record }) => record.key && record.key === row.file);
  const { record, at } = own ?? found[0];
  return { id: record.id, name: record.name, at, status: own ? record.status : 'Collision' };
}

/* LEDGER */

const strip = (word) => (/^[hvxyz](tree|line)$/.test(word) ? word.slice(1) : word);

export function ledger(math) {
  const census = new Map();
  const slices = new Map();
  const profiles = new Map();
  const rows = new Map();
  const spaces = new Map();
  const swept = new Map();
  const tile = (dim, base, code, number, level) => (dim === 2 ? math.two.create(code, number, level, 0, base) : math.three.create(code, number, level, base));
  const memo = (map, id, make) => {
    if (!map.has(id)) map.set(id, make());
    return map.get(id);
  };
  const names = (dim) =>
    memo(spaces, `names-${dim}`, () => {
      const total = 1 << (1 << dim);
      const cells = Array.from({ length: total }, (_, code) => tile(dim, 2, String(code), 3, 1).types.join(''));
      const named = [...math.bang.sources('Classics', dim).map((one) => one.design), ...math.bang.catalog.antis(dim)];
      const exact = new Map([[0, 'empty'], [SOLID[dim], 'solid']]);
      const orbits = new Map(exact);
      for (const name of named) {
        const drawn = (dim === 2 ? math.two.named(name, 3, 1, 0) : math.three.named(name, 3, 1)).types.join('');
        const code = cells.indexOf(drawn);
        const canon = Math.min(...math.bang.universe.orbit(String(code), dim).map(Number));
        const word = strip(name.toLowerCase());
        exact.set(code, word);
        if (code === canon || !orbits.has(canon)) orbits.set(canon, word);
      }
      return { exact, orbits };
    });
  const word = (dim, base, code) => {
    if (base !== 2) return '';
    const { exact, orbits } = names(dim);
    return exact.get(Number(code)) ?? orbits.get(Math.min(...math.bang.universe.orbit(String(code), dim).map(Number))) ?? '';
  };
  const design = (dim, base, code) => {
    const name = word(dim, base, code);
    return { code: String(code), name, label: name ? `${name} ${code}` : `code ${code}`, cells: [...tile(dim, base, String(code), 3, 1).types] };
  };
  const designs = (dim, base) =>
    memo(spaces, `${dim}-${base}`, () => {
      const codes = base === 2 ? math.bang.universe_codes(dim) : math.bang.baseq.representatives(base, dim).map(([code]) => code);
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
  const keys = (dim, base) =>
    memo(spaces, `keys-${dim}-${base}`, () => {
      const out = [];
      for (const one of designs(dim, base)) out.push(...lines(dim, base, one.code));
      return out;
    });
  const lines = (dim, base, code) => {
    const out = [];
    for (const one of MEASURES) {
      if (!applies(one, dim, base)) continue;
      for (const axis of AXES) out.push(row({ dim, base, code: String(code), measure: one.slug, axis: axis.slug }));
    }
    return out;
  };
  const typed = (dim, base, code) => (/^\d+$/.test(String(code)) && holds(dim, base, code) ? lines(dim, base, code) : []);
  const row = (key) =>
    memo(rows, print(key), () => {
      const { dim, base, code, measure: m, axis } = key;
      const { name, label, cells } = design(dim, base, code);
      return { key: print(key), dim, base, code, measure: m, axis, name, label, cells, file: new math.name.Sequence(code, dim, base, m, axis).to_file(), terms: [], stop: 0 };
    });
  const spot = (one, index) => {
    const at = place(one.axis, index);
    return { number: at.number ?? Math.max(one.base, 3), level: at.level };
  };
  const cost = (one, index) => {
    const kind = measure(one.measure).kind;
    const { number, level } = spot(one, index);
    if (kind === 'closed') return 0;
    if (kind === 'profile') return (one.dim * (number ** level - 1) + 1) / PROFILE;
    if (kind === 'cut') return number ** (2 * level);
    return number ** (one.dim * level);
  };
  const tally = (one, number, level) =>
    memo(census, `${one.dim}-${one.base}-${one.code}-${number}-${level}`, () => {
      const cell = tile(one.dim, one.base, one.code, number, level);
      return one.dim === 2 ? math.two.census(cell) : math.three.census(cell);
    });
  const slice = (one, number, level) => memo(slices, `${one.code}-${number}-${level}`, () => math.six.cut_design(one.code, number, level, one.base));
  const profile = (one, number, level) =>
    memo(profiles, `${one.dim}-${one.base}-${one.code}-${number}-${level}`, () => math.counts.profile_of_tile(math.bang.factory.create(one.code, number, one.dim, one.base, 1), level).map(BigInt));
  const term = (one, index) => {
    const { dim, base, code } = one;
    const { number, level } = spot(one, index);
    switch (one.measure) {
      case 'fills':
        return math.counts.fill(code, number, dim, level, base);
      case 'voids':
        return math.counts.void_(code, number, dim, level, base);
      case 'surface':
        return math.counts.exposure(code, number, dim, level, base);
      case 'peak':
        return profile(one, number, level).reduce((top, v) => (v > top ? v : top), 0n).toString();
      case 'heights':
        return String(profile(one, number, level).filter((v) => v > 0n).length);
      case 'triangles':
        return math.counts.cut_fills(code, number, level);
      case 'cutvoids':
        return math.counts.cut_voids(code, number, level);
      case 'profills':
        return math.counts.pro_fills(code, number, level);
      case 'provoids':
        return math.counts.pro_voids(code, number, level);
      case 'holes':
        return String(math.six.holes(slice(one, number, level)));
      case 'pieces':
        return String(math.six.components(slice(one, number, level)));
      default:
        return String(tally(one, number, level)[one.measure]);
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
    } catch {
      one.stop = Infinity;
    }
    return true;
  };
  const capped = (one, count, cells) => one.terms.length < count && (one.stop === Infinity || one.stop > cells);
  const read = (key, count, cells) => {
    const one = typeof key === 'string' ? row(parse(key)) : key;
    while (step(one, count, cells));
    return { terms: one.terms.slice(0, count), capped: capped(one, count, cells) };
  };
  const sweep = (dim, base, count = TABLE.terms, cells = TABLE.cells) => {
    const id = `${dim}-${base}`;
    const list = keys(dim, base);
    let at = swept.get(id) ?? 0;
    while (at < list.length) {
      if (step(list[at], count, cells)) {
        swept.set(id, at);
        return true;
      }
      at++;
    }
    swept.set(id, at);
    return false;
  };
  const progress = (dim, base) => ({ done: Math.min(swept.get(`${dim}-${base}`) ?? 0, keys(dim, base).length), total: keys(dim, base).length });
  return { designs, keys, typed, row, read, step, sweep, progress, capped };
}

/* LEAD */

export const flat = (terms) => terms.length > 1 && terms.every((one) => one === terms[0]);

export function lead(book, list, at) {
  for (let i = 0; i < list.length; i++) {
    const row = list[(at + i) % list.length];
    if (!blank(row) && !flat(book.read(row, LEAD, TABLE.cells).terms)) return row;
  }
  return list[at] ?? null;
}

/* SEARCH */

const mention = (row, hit) => `${row.label} ${row.measure} ${row.axis} ${row.dim}d base ${row.base} ${hit ? `${hit.id} ${hit.name}` : ''}`.toLowerCase();

export function search(list, q, found, typed = () => []) {
  const text = String(q ?? '').trim();
  if (!text) return list;
  const terms = numbers(text);
  if (terms && terms.length > 1) return list.filter((row) => window(row.terms, terms) >= 0);
  const needle = text.toLowerCase();
  const own = terms ? typed(terms[0]) : [];
  const keys = new Set(own.map((row) => row.key));
  return [...own, ...list.filter((row) => !keys.has(row.key) && mention(row, found(row)).includes(needle))];
}

export function filter(list, measureSlug, axisSlug) {
  return list.filter((row) => (!measureSlug || row.measure === measureSlug) && (!axisSlug || row.axis === axisSlug));
}

/* SHEETS */

const shown = (row) => ({ name: row.file, design: row.name, code: row.code, dim: row.dim, base: row.base, measure: row.measure, axis: row.axis, start: axisOf(row.axis).start, step: axisOf(row.axis).step, record: row.hit?.id ?? '', at: row.hit?.at ?? null, status: row.hit?.status ?? '', terms: row.terms });

export function json(list) {
  return JSON.stringify(list.map(shown), null, 2);
}

export function csv(list) {
  const head = 'name,design,code,dim,base,measure,axis,start,step,record,status,terms';
  const lines = list.map(shown).map((one) => [one.name, one.design, one.code, one.dim, one.base, one.measure, one.axis, one.start, one.step, one.record, one.status, `"${one.terms.join(', ')}"`].join(','));
  return [head, ...lines].join('\n');
}
