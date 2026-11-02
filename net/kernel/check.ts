import type { Schema, Type } from "../spec/index.ts";
import type { Json } from "./json.ts";
import { canon } from "./json.ts";

/* SUBSET */

const KEYS = new Set([
  "spec",
  "type",
  "title",
  "line",
  "default",
  "enum",
  "const",
  "minimum",
  "maximum",
  "multipleOf",
  "minLength",
  "maxLength",
  "pattern",
  "properties",
  "required",
  "additionalProperties",
  "items",
  "minItems",
  "maxItems",
]);

/* NOTES */

const shown = (value: unknown) => (typeof value === "string" ? JSON.stringify(value) : canon(value));

const named = (at: string) => at || "the args";

const either = (list: unknown[]) =>
  list.length === 1 ? shown(list[0]) : `${list.slice(0, -1).map(shown).join(", ")} or ${shown(list.at(-1))}`;

function kind(value: unknown): Type | "other" {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  if (typeof value === "number") return Number.isInteger(value) ? "integer" : "number";
  if (typeof value === "string" || typeof value === "boolean" || typeof value === "object") return typeof value as Type;
  return "other";
}

const fits = (type: Type, value: unknown) => {
  const k = kind(value);
  return k === type || (type === "number" && k === "integer");
};

function between(s: Schema, at: string): string {
  if (s.minimum !== undefined && s.maximum !== undefined)
    return `${named(at)} must be between ${s.minimum} and ${s.maximum}`;
  if (s.minimum !== undefined) return `${named(at)} must be at least ${s.minimum}`;
  return `${named(at)} must be at most ${s.maximum}`;
}

/* CHECK */

function number(s: Schema, v: number, at: string): string | null {
  if ((s.minimum !== undefined && v < s.minimum) || (s.maximum !== undefined && v > s.maximum)) return between(s, at);
  if (s.multipleOf !== undefined) {
    const q = v / s.multipleOf;
    if (Math.abs(q - Math.round(q)) > 1e-9 * Math.max(1, Math.abs(q)))
      return `${named(at)} must be a multiple of ${s.multipleOf}`;
  }
  return null;
}

function string(s: Schema, v: string, at: string): string | null {
  if (s.minLength !== undefined && v.length < s.minLength)
    return `${named(at)} must be at least ${s.minLength} characters`;
  if (s.maxLength !== undefined && v.length > s.maxLength)
    return `${named(at)} must be at most ${s.maxLength} characters`;
  if (s.pattern !== undefined && !new RegExp(s.pattern, "u").test(v))
    return `${named(at)} must match ${s.pattern}, not ${shown(v)}`;
  return null;
}

function object(s: Schema, v: Record<string, Json>, at: string): string | null {
  const props = s.properties ?? {};
  for (const key of s.required ?? []) if (!(key in v)) return `${at ? `${at}/` : ""}${key} is required`;
  for (const key of Object.keys(v)) {
    const one = props[key];
    if (one) {
      const note = check(one, v[key], at ? `${at}/${key}` : key);
      if (note) return note;
    } else if (s.additionalProperties === false) {
      const known = Object.keys(props);
      return `${named(at)} take${at ? "s" : ""} no ${key}${known.length ? `; known: ${known.join(", ")}` : "; it takes nothing"}`;
    }
  }
  return null;
}

function array(s: Schema, v: Json[], at: string): string | null {
  if (s.minItems !== undefined && v.length < s.minItems) return `${named(at)} must hold at least ${s.minItems} items`;
  if (s.maxItems !== undefined && v.length > s.maxItems) return `${named(at)} must hold at most ${s.maxItems} items`;
  if (!s.items) return null;
  for (let i = 0; i < v.length; i++) {
    const note = check(s.items, v[i], `${at}/${i}`);
    if (note) return note;
  }
  return null;
}

export function check(s: Schema, v: unknown, at = ""): string | null {
  if (s.const !== undefined && canon(s.const) !== canon(v)) return `${named(at)} must be ${shown(s.const)}`;
  if (s.enum && !s.enum.some((one) => canon(one) === canon(v))) return `${named(at)} must be ${either(s.enum)}`;
  if (s.type && !fits(s.type, v)) return `${named(at)} must be ${s.type === "integer" ? "an integer" : `a ${s.type}`}`;
  if (typeof v === "number") return number(s, v, at);
  if (typeof v === "string") return string(s, v, at);
  if (Array.isArray(v)) return array(s, v as Json[], at);
  if (v && typeof v === "object") return object(s, v as Record<string, Json>, at);
  return null;
}

/* SCHEMA */

export function schemaNote(s: unknown, at = "schema"): string | null {
  if (!s || typeof s !== "object" || Array.isArray(s)) return `${at} must be an object`;
  for (const key of Object.keys(s)) if (!KEYS.has(key)) return `${at} uses ${key}, outside the subset`;
  const one = s as Schema;
  for (const [key, sub] of Object.entries(one.properties ?? {})) {
    const note = schemaNote(sub, `${at}/properties/${key}`);
    if (note) return note;
  }
  return one.items ? schemaNote(one.items, `${at}/items`) : null;
}
