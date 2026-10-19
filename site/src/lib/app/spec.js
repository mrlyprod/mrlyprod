const blank = (v) => v === undefined || v === null || String(v).trim() === '';

const choice = (option) => (Array.isArray(option) ? option[0] : option);

const sep = (row) => row.sep ?? '.';

const live = (row) => [row.max, row.min, row.options].some((one) => typeof one === 'function');

const read = (one, value, units) => (typeof one === 'function' ? one(value, units) : one);

const plain = (one) => (typeof one === 'function' ? undefined : one);

export const maxOf = (row, value, units) => read(row.max, value, units);

export const minOf = (row, value, units) => read(row.min, value, units);

export const optionsOf = (row, value, units) => read(row.options, value, units);

export const unitOf = (row, value, units) => read(row.unit, value, units);

export const labelOf = (row, value, units) => read(row.label, value, units) ?? row.key;

function number(row, v, value, units, grid = minOf(row, value, units)) {
  const n = blank(v) ? NaN : Number(v);
  const { step } = row;
  const min = minOf(row, value, units) ?? -Infinity;
  const max = maxOf(row, value, units) ?? Infinity;
  if (!Number.isFinite(n)) return Math.min(max, Math.max(min, row.def));
  const from = Number.isFinite(grid) ? grid : 0;
  const snap = step ? +(from + Math.round((n - from) / step) * step).toPrecision(12) : n;
  return Math.min(max, Math.max(min, snap));
}

function pick(row, v, value, units) {
  if (v === undefined || v === null) return row.def;
  const found = optionsOf(row, value, units).map(choice).find((option) => String(option) === String(v).trim());
  return found === undefined ? row.def : found;
}

export function items(row, v, value, units) {
  const raw = Array.isArray(v) ? v : blank(v) ? [] : sep(row) ? String(v).split(sep(row)) : [...String(v)];
  const allowed = optionsOf(row, value, units).map((option) => String(choice(option)));
  return [...new Set(raw.map((item) => String(item).trim()).filter((item) => allowed.includes(item)))];
}

const KINDS = {
  slider: number,
  number,
  toggle: (row, v) => number({ ...row, min: 0, max: 1, step: 1 }, v),
  pick,
  segment: pick,
  text: (row, v) => (blank(v) ? row.def : String(v).trim()),
  list: (row, v, value, units) => {
    const kept = items(row, v, value, units);
    return kept.length ? kept.join(sep(row)) : row.def;
  },
};

export function defaults(spec) {
  return Object.fromEntries(spec.map((row) => [row.key, row.def]));
}

function walk(spec, value, units) {
  const fixed = Object.fromEntries(spec.filter((row) => !live(row)).map((row) => [row.key, KINDS[row.kind](row, value?.[row.key], {})]));
  const start = { seen: { ...value, ...defaults(spec), ...fixed }, at: {} };
  return spec.filter(live).reduce(({ seen, at }, row) => ({ seen: { ...seen, [row.key]: KINDS[row.kind](row, value?.[row.key], seen, units) }, at: { ...at, [row.key]: seen } }), start);
}

export function tidy(spec, value, units) {
  const { seen } = walk(spec, value, units);
  return Object.fromEntries(spec.map((row) => [row.key, seen[row.key]]));
}

export function asked(spec, value, units) {
  const { at } = walk(spec, value, units);
  const one = (row, v) => {
    if (!live(row)) return KINDS[row.kind](row, v, {});
    if (row.kind === 'slider' || row.kind === 'number') return number({ ...row, max: plain(row.max), min: plain(row.min) }, v, at[row.key], units, minOf(row, at[row.key], units));
    return blank(v) ? row.def : v;
  };
  return Object.fromEntries(spec.filter((row) => row.keep).map((row) => [row.key, one(row, value?.[row.key])]));
}

export function describe(spec, value = {}) {
  const groups = [];
  for (const row of spec) {
    if (row.when && !row.when(value)) continue;
    const name = row.group ?? '';
    const group = groups.find((one) => one.name === name);
    if (group) group.rows.push(row);
    else groups.push({ name, rows: [row] });
  }
  return groups;
}

export const viewing = (row) => row.key === 'view' || row.view === true;

export function places(spec = []) {
  const view = spec.find(viewing);
  const lead = view ? (view.group ?? '') : null;
  const rest = [...new Set(spec.map((row) => row.group ?? ''))].filter((name) => name !== lead);
  const of = (name) => (name === lead ? 0 : 2 * rest.indexOf(name) + 3);
  const at = (after) => (after !== undefined && (after === lead || rest.includes(after)) ? of(after) + 1 : 2);
  return { lead, of, at };
}
