import type { Schema } from "@mrlyprod/os/contract/spec/index.ts";
import type { Act, Json, Move, Obj } from "@mrlyprod/os/kernel/index.ts";
import { below } from "@mrlyprod/os/kernel/index.ts";

type Draw = () => number;

const SPAN = 9;
const LETTERS = "abcdefghijklmnopqrstuvwxyz";

function decimals(n: number): number {
  const [head, exp] = String(n).split("e");
  const after = head.split(".")[1]?.length ?? 0;
  return Math.max(0, after - Number(exp ?? 0));
}

function integer(s: Schema, draw: Draw): number | undefined {
  const step = s.multipleOf && Number.isInteger(s.multipleOf) ? s.multipleOf : 1;
  const lo = Math.ceil((s.minimum ?? (s.maximum !== undefined ? s.maximum - SPAN : 0)) / step);
  const hi = Math.floor((s.maximum ?? (s.minimum !== undefined ? s.minimum + SPAN : SPAN)) / step);
  if (hi < lo) return undefined;
  return (lo + below(draw, Math.min(hi - lo + 1, 4294967296))) * step;
}

function number(s: Schema, draw: Draw): number | undefined {
  if (s.multipleOf) {
    const lo = Math.ceil((s.minimum ?? 0) / s.multipleOf);
    const hi = Math.floor((s.maximum ?? (s.minimum ?? 0) + SPAN) / s.multipleOf);
    if (hi < lo) return undefined;
    const places = Math.min(20, decimals(s.multipleOf));
    return Number(((lo + below(draw, Math.min(hi - lo + 1, 4294967296))) * s.multipleOf).toFixed(places));
  }
  const lo = s.minimum ?? 0;
  const hi = s.maximum ?? lo + SPAN;
  return lo + (draw() / 4294967296) * (hi - lo);
}

function string(s: Schema, draw: Draw): string | undefined {
  if (s.pattern !== undefined) return undefined;
  const lo = s.minLength ?? 1;
  const hi = Math.max(lo, Math.min(s.maxLength ?? 8, lo + 7));
  const n = lo + below(draw, hi - lo + 1);
  return Array.from({ length: n }, () => LETTERS[below(draw, LETTERS.length)]).join("");
}

function object(s: Schema, draw: Draw): Obj | undefined {
  const out: Obj = {};
  const need = new Set(s.required ?? []);
  for (const [key, sub] of Object.entries(s.properties ?? {})) {
    if (!need.has(key) && draw() & 1) continue;
    const value = fill(sub, draw);
    if (value === undefined) {
      if (need.has(key)) return undefined;
      continue;
    }
    out[key] = value;
  }
  return out;
}

function array(s: Schema, draw: Draw): Json[] | undefined {
  const lo = s.minItems ?? 0;
  const hi = Math.max(lo, Math.min(s.maxItems ?? lo + 3, lo + 3));
  const n = lo + below(draw, hi - lo + 1);
  const out: Json[] = [];
  for (let i = 0; i < n; i++) {
    const value = s.items ? fill(s.items, draw) : null;
    if (value === undefined) return undefined;
    out.push(value);
  }
  return out;
}

export function fill(s: Schema, draw: Draw): Json | undefined {
  if (s.const !== undefined) return s.const as Json;
  if (s.enum) return s.enum.length ? (s.enum[below(draw, s.enum.length)] as Json) : undefined;
  const type = s.type ?? (s.properties ? "object" : undefined);
  if (type === "integer") return integer(s, draw);
  if (type === "number") return number(s, draw);
  if (type === "string") return string(s, draw);
  if (type === "boolean") return Boolean(draw() & 1);
  if (type === "null") return null;
  if (type === "object") return object(s, draw);
  if (type === "array") return array(s, draw);
  return undefined;
}

export function goose(moves: readonly Move[], draw: Draw, skip: readonly string[] = []): Act | null {
  const open = moves.filter((move) => !skip.includes(move.verb));
  const order = open.map((_, i) => i);
  for (let i = order.length - 1; i > 0; i--) {
    const j = below(draw, i + 1);
    [order[i], order[j]] = [order[j], order[i]];
  }
  for (const i of order) {
    const args = fill(open[i].args, draw);
    if (args && typeof args === "object" && !Array.isArray(args)) return { verb: open[i].verb, args };
  }
  return null;
}
