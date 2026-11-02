import { site } from "site:routes";
import type { Readout } from "./hooks.ts";
import type { Key } from "./keys.tsx";
import type { Option, Patch, Row, Scalar, Spec, Units, Value } from "./spec.ts";
import { labelOf, maxOf, minOf, optionsOf, tidy } from "./spec.ts";

/* TYPES */

export type Schema = {
  type?: "integer" | "number" | "string" | "object";
  title?: string;
  default?: Scalar;
  minimum?: number;
  maximum?: number;
  multipleOf?: number;
  enum?: Scalar[];
  pattern?: string;
  properties?: Record<string, Schema>;
  required?: string[];
  additionalProperties?: boolean;
};

export type Verb = { name: string; title: string; args: Schema };

export type Model<U = Units> = {
  id: string;
  user?: string;
  spec?: Spec<U>;
  keys?: readonly Key<never>[];
  value: Value;
  units?: U;
  status?: readonly Readout[];
};

export type Ports = { change: (patch: Patch) => void; run: (act: string) => unknown };

export type Read = { value: Value; status: Record<string, string> };

type Acts = { act: string; title: string; keys: string[] };

export const user = () => site.title.toLowerCase().replace(/[^a-z0-9]/g, "");

export const ALWAYS = ["reroll", "pause"] as const;

const CHROME = ["zen", "keys", "info", "export", "record"];

const SEED: Schema = { type: "integer", title: "Seed", minimum: 0, maximum: 4294967295, multipleOf: 1 };

/* SCHEMA */

const choice = (option: Option): Scalar => (Array.isArray(option) ? option[0] : option);

const finite = (n: unknown): n is number => typeof n === "number" && Number.isFinite(n);

const literal = (text: string) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

function numeric<U>(row: Row<U>, value: Value, units?: U): Schema {
  const min = minOf(row, value, units);
  const max = maxOf(row, value, units);
  const { step } = row;
  const grid = finite(step) && step > 0 && Number.isInteger(+((finite(min) ? min : 0) / step).toPrecision(12));
  const whole = grid && Number.isInteger(step) && (!finite(min) || Number.isInteger(min));
  return {
    type: whole ? "integer" : "number",
    ...(finite(min) ? { minimum: min } : {}),
    ...(finite(max) ? { maximum: max } : {}),
    ...(grid ? { multipleOf: step } : {}),
  };
}

function listed<U>(row: Row<U>, value: Value, units?: U): Schema {
  const one = `(?:${(optionsOf(row, value, units) ?? []).map((option) => literal(String(choice(option)))).join("|")})`;
  const sep = row.sep ?? ".";
  return { type: "string", pattern: sep ? `^(?:${one}(?:${literal(sep)}${one})*)?$` : `^${one}*$` };
}

const enumerated = <U>(row: Row<U>, value: Value, units?: U): Schema => ({
  enum: (optionsOf(row, value, units) ?? []).map(choice),
});

const KINDS: Record<Row["kind"], <U>(row: Row<U>, value: Value, units?: U) => Schema> = {
  slider: numeric,
  number: numeric,
  toggle: () => ({ type: "integer", enum: [0, 1] }),
  pick: enumerated,
  segment: enumerated,
  text: () => ({ type: "string" }),
  list: listed,
};

export function schema<U>(row: Row<U>, value: Value, units?: U): Schema {
  return { title: labelOf(row, value, units), default: row.def, ...KINDS[row.kind](row, value, units) };
}

/* VERBS */

const named = <U>(model: Model<U>, verb: string) => `${model.user ?? user()}.${model.id}.${verb}`;

const shown = (row: Key<never>, value: Value) => (typeof row.label === "function" ? row.label(value, null) : row.label);

function acts<U>(model: Model<U>): Acts[] {
  const out: Acts[] = [];
  for (const row of model.keys ?? []) {
    if (row.when && !row.when(model.value)) continue;
    if ([...ALWAYS, ...CHROME].includes(row.act)) continue;
    const keys = ([] as string[]).concat(row.key);
    const title = shown(row, model.value);
    const found = out.find((one) => one.act === row.act);
    if (!found) out.push({ act: row.act, title, keys });
    else {
      found.keys = [...new Set([...found.keys, ...keys])];
      if (!found.title.split(" / ").includes(title)) found.title = `${found.title} / ${title}`;
    }
  }
  return out;
}

const pressed = (keys: string[]): Schema =>
  keys.length > 1
    ? { type: "object", properties: { key: { enum: keys } }, required: ["key"], additionalProperties: false }
    : { type: "object", additionalProperties: false };

export function list<U>(model: Model<U>): Verb[] {
  const rows = model.spec ?? [];
  const set: Verb = {
    name: named(model, "set"),
    title: "Set",
    args: {
      type: "object",
      properties: {
        seed: SEED,
        ...Object.fromEntries(rows.map((row) => [row.key, schema(row, model.value, model.units)])),
      },
      additionalProperties: false,
    },
  };
  return [
    set,
    { name: named(model, "reroll"), title: "Reroll", args: pressed([]) },
    { name: named(model, "pause"), title: "Pause", args: pressed([]) },
    ...acts(model).map((one) => ({ name: named(model, one.act), title: one.title, args: pressed(one.keys) })),
  ];
}

/* READ */

export function text(node: unknown): string {
  if (typeof node === "string" || typeof node === "number" || typeof node === "bigint") return String(node);
  if (Array.isArray(node)) return node.map(text).join("");
  if (node && typeof node === "object" && "props" in node)
    return text((node as { props?: { children?: unknown } }).props?.children);
  return "";
}

export function read<U>(model: Model<U>): Read {
  return {
    value: model.spec ? { ...model.value, ...tidy(model.spec, model.value, model.units) } : model.value,
    status: Object.fromEntries((model.status ?? []).map(([label, v]) => [label, text(v)])),
  };
}

/* CALL */

export function call<U>(model: Model<U>, ports: Ports, name: string, args: Patch = {}): void {
  const verbs = list(model);
  if (!verbs.some((one) => one.name === name))
    throw new Error(`sdk: no verb ${name}; ${model.id} has ${verbs.map((one) => one.name).join(", ")}`);
  const verb = name.slice(named(model, "").length);
  if (verb === "set") {
    const known = ["seed", ...(model.spec ?? []).map((row) => row.key)];
    const odd = Object.keys(args).filter((key) => !known.includes(key));
    if (odd.length) throw new Error(`sdk: ${name} takes no ${odd.join(", ")}; it takes ${known.join(", ")}`);
    ports.change(args);
    return;
  }
  if ((ALWAYS as readonly string[]).includes(verb)) {
    ports.run(verb);
    return;
  }
  const { keys } = acts(model).find((one) => one.act === verb)!;
  const key = args.key === undefined ? keys[0] : String(args.key);
  if (!keys.includes(key)) throw new Error(`sdk: ${name} takes key ${keys.join(", ")}, not ${key}`);
  ports.run(`${verb}@${key}`);
}
