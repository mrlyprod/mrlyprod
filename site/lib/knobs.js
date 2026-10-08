const blank = (v) => v === undefined || v === null || String(v).trim() === '';

const choice = (option) => (Array.isArray(option) ? option[0] : option);

const sep = (row) => row.sep ?? '.';

function number(row, v) {
  const n = blank(v) ? NaN : Number(v);
  if (!Number.isFinite(n)) return row.def;
  const { min = -Infinity, max = Infinity, step } = row;
  const from = Number.isFinite(min) ? min : 0;
  const snap = step ? +(from + Math.round((n - from) / step) * step).toPrecision(12) : n;
  return Math.min(max, Math.max(min, snap));
}

function pick(row, v) {
  if (v === undefined || v === null) return row.def;
  const found = row.options.map(choice).find((option) => String(option) === String(v).trim());
  return found === undefined ? row.def : found;
}

export function items(row, v) {
  const raw = Array.isArray(v) ? v : blank(v) ? [] : sep(row) ? String(v).split(sep(row)) : [...String(v)];
  const allowed = row.options.map((option) => String(choice(option)));
  return [...new Set(raw.map((item) => String(item).trim()).filter((item) => allowed.includes(item)))];
}

const KINDS = {
  slider: number,
  number,
  toggle: (row, v) => number({ ...row, min: 0, max: 1, step: 1 }, v),
  pick,
  segment: pick,
  text: (row, v) => (blank(v) ? row.def : String(v).trim()),
  list: (row, v) => {
    const kept = items(row, v);
    return kept.length ? kept.join(sep(row)) : row.def;
  },
};

export function defaults(spec) {
  return Object.fromEntries(spec.map((row) => [row.key, row.def]));
}

export function tidy(spec, value) {
  return Object.fromEntries(spec.map((row) => [row.key, KINDS[row.kind](row, value?.[row.key])]));
}

export function describe(spec) {
  const groups = [];
  for (const row of spec) {
    const name = row.group ?? '';
    const group = groups.find((one) => one.name === name);
    if (group) group.rows.push(row);
    else groups.push({ name, rows: [row] });
  }
  return groups;
}
