import type { Patch, Scalar, Value } from "./spec.ts";

/* TYPES */

export type Stack = { past: Value[]; future: Value[]; at: number };

export type Moved = { stack: Stack; to: Value };

export type State = { depth?: number; y?: number; up?: boolean };

/* BROWSER */

const typed = (s: object, field: keyof State, type: "number" | "boolean") => {
  const v: unknown = Reflect.get(s, field);
  return v === undefined || typeof v === type;
};

const isState = (s: unknown): s is State =>
  typeof s === "object" &&
  s !== null &&
  typed(s, "depth", "number") &&
  typed(s, "y", "number") &&
  typed(s, "up", "boolean");

export const state = (): State => {
  const s: unknown = history.state;
  return isState(s) ? s : {};
};

/* HISTORY */

export const MERGE = 500;
const DEPTH = 100;

export const fresh = (): Stack => ({ past: [], future: [], at: -Infinity });

export function record(was: Stack, before: Value, now: number): Stack {
  const merged = was.past.length > 0 && now - was.at < MERGE;
  return { past: merged ? was.past : [...was.past, before].slice(-DEPTH), future: [], at: now };
}

export function undo(was: Stack, current: Value): Moved | null {
  if (!was.past.length) return null;
  return {
    stack: { past: was.past.slice(0, -1), future: [current, ...was.future], at: -Infinity },
    to: was.past.at(-1)!,
  };
}

export function redo(was: Stack, current: Value): Moved | null {
  if (!was.future.length) return null;
  return { stack: { past: [...was.past, current], future: was.future.slice(1), at: -Infinity }, to: was.future[0] };
}

export function patch(from: Value, to: Value): Patch {
  const keys = new Set([...Object.keys(from), ...Object.keys(to)]);
  return Object.fromEntries(
    [...keys].filter((key) => from[key] !== to[key]).map((key): [string, Scalar | undefined] => [key, to[key]]),
  );
}
