const blank = (v) => v === undefined || v === null || String(v).trim() === '';

const choice = (option) => (Array.isArray(option) ? option[0] : option);

const sep = (row) => row.sep ?? '.';

const live = (row) => typeof row.max === 'function' || typeof row.options === 'function';

export const maxOf = (row, value) => (typeof row.max === 'function' ? row.max(value) : row.max);

export const optionsOf = (row, value) => (typeof row.options === 'function' ? row.options(value) : row.options);

function number(row, v, value) {
  const n = blank(v) ? NaN : Number(v);
  const { min = -Infinity, step } = row;
  const max = maxOf(row, value) ?? Infinity;
  if (!Number.isFinite(n)) return Math.min(max, Math.max(min, row.def));
  const from = Number.isFinite(min) ? min : 0;
  const snap = step ? +(from + Math.round((n - from) / step) * step).toPrecision(12) : n;
  return Math.min(max, Math.max(min, snap));
}

function pick(row, v, value) {
  if (v === undefined || v === null) return row.def;
  const found = optionsOf(row, value).map(choice).find((option) => String(option) === String(v).trim());
  return found === undefined ? row.def : found;
}

export function items(row, v, value) {
  const raw = Array.isArray(v) ? v : blank(v) ? [] : sep(row) ? String(v).split(sep(row)) : [...String(v)];
  const allowed = optionsOf(row, value).map((option) => String(choice(option)));
  return [...new Set(raw.map((item) => String(item).trim()).filter((item) => allowed.includes(item)))];
}

const KINDS = {
  slider: number,
  number,
  toggle: (row, v) => number({ ...row, min: 0, max: 1, step: 1 }, v),
  pick,
  segment: pick,
  text: (row, v) => (blank(v) ? row.def : String(v).trim()),
  list: (row, v, value) => {
    const kept = items(row, v, value);
    return kept.length ? kept.join(sep(row)) : row.def;
  },
};

export function defaults(spec) {
  return Object.fromEntries(spec.map((row) => [row.key, row.def]));
}

export function tidy(spec, value) {
  const fixed = Object.fromEntries(spec.filter((row) => !live(row)).map((row) => [row.key, KINDS[row.kind](row, value?.[row.key], {})]));
  const seen = spec.filter(live).reduce((was, row) => ({ ...was, [row.key]: KINDS[row.kind](row, value?.[row.key], was) }), { ...value, ...defaults(spec), ...fixed });
  return Object.fromEntries(spec.map((row) => [row.key, seen[row.key]]));
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
