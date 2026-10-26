import type { Name, Unit } from "../units/index.ts";

/* TYPES */

export type Scalar = string | number;

export type Value = Record<string, Scalar>;

export type Patch = Partial<Value>;

export type Seeded = Value & { seed: number };

export type Raw = Record<string, unknown>;

export type Units = { [N in Name]?: Unit<N> };

export type Fn<T, U = Units> = (value: Value, units?: U) => T | undefined;

export type Live<T, U = Units> = T | Fn<T, U>;

export type Option = Scalar | [Scalar, Scalar];

export type Kind = "slider" | "number" | "toggle" | "pick" | "segment" | "list" | "text";

export type Row<U = Units> = {
  key: string;
  kind: Kind;
  def: Scalar;
  label?: Live<string, U>;
  group?: string;
  when?: (value: Value) => boolean;
  keep?: boolean;
  view?: boolean;
  min?: Live<number, U>;
  max?: Live<number, U>;
  step?: number;
  unit?: Live<string, U>;
  zero?: string;
  show?: (v: number, value: Value) => unknown;
  options?: Live<readonly Option[], U>;
  sep?: string;
  wide?: boolean;
};

export type Spec<U = Units> = readonly Row<U>[];

export type Group<U = Units> = { name: string; rows: Row<U>[] };

export type Places = { lead: string | null; of: (name: string) => number; at: (after?: string) => number };

type Tidier = <U>(row: Row<U>, v: unknown, value: Value, units?: U) => Scalar;

type Walk = { seen: Value; at: Record<string, Value> };

/* ROWS */

const blank = (v: unknown) => v === undefined || v === null || String(v).trim() === "";

const choice = (option: Option): Scalar => (Array.isArray(option) ? option[0] : option);

const sep = <U>(row: Row<U>) => row.sep ?? ".";

const live = <U>(row: Row<U>) => [row.max, row.min, row.options].some((one) => typeof one === "function");

const fn = <T, U>(one: Live<T, U> | undefined): one is Fn<T, U> => typeof one === "function";

const read = <T, U>(one: Live<T, U> | undefined, value: Value, units?: U) => (fn(one) ? one(value, units) : one);

const plain = <T, U>(one: Live<T, U> | undefined) => (fn(one) ? undefined : one);

export const maxOf = <U>(row: Row<U>, value: Value, units?: U) => read(row.max, value, units);

export const minOf = <U>(row: Row<U>, value: Value, units?: U) => read(row.min, value, units);

export const optionsOf = <U>(row: Row<U>, value: Value, units?: U) => read(row.options, value, units);

export const unitOf = <U>(row: Row<U>, value: Value, units?: U) => read(row.unit, value, units);

export const labelOf = <U>(row: Row<U>, value: Value, units?: U) => read(row.label, value, units) ?? row.key;

function number<U>(row: Row<U>, v: unknown, value: Value, units?: U, grid = minOf(row, value, units)): number {
  const n = blank(v) ? NaN : Number(v);
  const { step } = row;
  const min = minOf(row, value, units) ?? -Infinity;
  const max = maxOf(row, value, units) ?? Infinity;
  if (!Number.isFinite(n)) return Math.min(max, Math.max(min, Number(row.def)));
  const from = Number.isFinite(grid) ? grid! : 0;
  const snap = step ? +(from + Math.round((n - from) / step) * step).toPrecision(12) : n;
  return Math.min(max, Math.max(min, snap));
}

function pick<U>(row: Row<U>, v: unknown, value: Value, units?: U): Scalar {
  if (v === undefined || v === null) return row.def;
  const found = optionsOf(row, value, units)!
    .map(choice)
    .find((option) => String(option) === String(v).trim());
  return found === undefined ? row.def : found;
}

export function items<U>(row: Row<U>, v: unknown, value: Value, units?: U): string[] {
  const raw: unknown[] = Array.isArray(v) ? v : blank(v) ? [] : sep(row) ? String(v).split(sep(row)) : [...String(v)];
  const allowed = optionsOf(row, value, units)!.map((option) => String(choice(option)));
  return [...new Set(raw.map((item) => String(item).trim()).filter((item) => allowed.includes(item)))];
}

const KINDS: Record<Kind, Tidier> = {
  slider: number,
  number,
  toggle: (row, v) => number({ ...row, min: 0, max: 1, step: 1 }, v, {}),
  pick,
  segment: pick,
  text: (row, v) => (blank(v) ? row.def : String(v).trim()),
  list: (row, v, value, units) => {
    const kept = items(row, v, value, units);
    return kept.length ? kept.join(sep(row)) : row.def;
  },
};

export function defaults<U>(spec: Spec<U>): Value {
  return Object.fromEntries(spec.map((row): [string, Scalar] => [row.key, row.def]));
}

function walk<U>(spec: Spec<U>, value: Raw | undefined, units?: U): Walk {
  const fixed: Value = Object.fromEntries(
    spec
      .filter((row) => !live(row))
      .map((row): [string, Scalar] => [row.key, KINDS[row.kind](row, value?.[row.key], {})]),
  );
  const start: Walk = { seen: { ...value, ...defaults(spec), ...fixed } as Value, at: {} };
  return spec.filter(live).reduce(
    ({ seen, at }, row) => ({
      seen: { ...seen, [row.key]: KINDS[row.kind](row, value?.[row.key], seen, units) },
      at: { ...at, [row.key]: seen },
    }),
    start,
  );
}

export function tidy<U>(spec: Spec<U>, value?: Raw, units?: U): Value {
  const { seen } = walk(spec, value, units);
  return Object.fromEntries(spec.map((row): [string, Scalar] => [row.key, seen[row.key]]));
}

export function asked<U>(spec: Spec<U>, value: Patch | undefined, units?: U): Value {
  const { at } = walk(spec, value, units);
  const one = (row: Row<U>, v: Scalar | undefined): Scalar => {
    if (!live(row)) return KINDS[row.kind](row, v, {});
    if (row.kind === "slider" || row.kind === "number")
      return number(
        { ...row, max: plain(row.max), min: plain(row.min) },
        v,
        at[row.key],
        units,
        minOf(row, at[row.key], units),
      );
    return blank(v) ? row.def : v!;
  };
  return Object.fromEntries(
    spec.filter((row) => row.keep).map((row): [string, Scalar] => [row.key, one(row, value?.[row.key])]),
  );
}

export function describe<U>(spec: Spec<U>, value: Value = {}): Group<U>[] {
  const groups: Group<U>[] = [];
  for (const row of spec) {
    if (row.when && !row.when(value)) continue;
    const name = row.group ?? "";
    const group = groups.find((one) => one.name === name);
    if (group) group.rows.push(row);
    else groups.push({ name, rows: [row] });
  }
  return groups;
}

export const viewing = <U>(row: Row<U>) => row.key === "view" || row.view === true;

export function places<U>(spec: Spec<U> = []): Places {
  const view = spec.find(viewing);
  const lead = view ? (view.group ?? "") : null;
  const rest = [...new Set(spec.map((row) => row.group ?? ""))].filter((name) => name !== lead);
  const of = (name: string) => (name === lead ? 0 : 2 * rest.indexOf(name) + 3);
  const at = (after?: string) => (after !== undefined && (after === lead || rest.includes(after)) ? of(after) + 1 : 2);
  return { lead, of, at };
}
