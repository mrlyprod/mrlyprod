/* TYPES */

export type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

export type Obj = { [key: string]: Json };

/* CANON */

function put(value: unknown, at: string): string {
  if (value === null) return "null";
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number") {
    if (!Number.isFinite(value)) throw new Error(`json: ${at || "the value"} is ${value}, which JSON cannot hold`);
    return JSON.stringify(Object.is(value, -0) ? 0 : value);
  }
  if (Array.isArray(value)) return `[${value.map((one, i) => put(one, `${at}/${i}`)).join(",")}]`;
  if (typeof value === "object" && Object.getPrototypeOf(value) === Object.prototype) {
    const keys = Object.keys(value as object)
      .filter((key) => (value as Record<string, unknown>)[key] !== undefined)
      .sort();
    return `{${keys.map((key) => `${JSON.stringify(key)}:${put((value as Record<string, unknown>)[key], `${at}/${key}`)}`).join(",")}}`;
  }
  const kind = typeof value === "object" ? (value as object).constructor?.name : typeof value;
  throw new Error(
    `json: ${at || "the value"} is a ${kind}; a world holds plain JSON only (big integers as decimal strings)`,
  );
}

export const canon = (value: unknown): string => put(value, "");

export const same = (a: unknown, b: unknown): boolean => canon(a) === canon(b);

export const copy = <T>(value: T): T => JSON.parse(canon(value));

/* DIGEST */

const BASIS = 0xcbf29ce484222325n;
const PRIME = 0x100000001b3n;
const MASK = 0xffffffffffffffffn;

export function digest(value: unknown): string {
  let h = BASIS;
  for (const byte of new TextEncoder().encode(canon(value))) h = ((h ^ BigInt(byte)) * PRIME) & MASK;
  return h.toString(16).padStart(16, "0");
}

export function fnv32(text: string): number {
  let h = 0x811c9dc5;
  for (const byte of new TextEncoder().encode(text)) h = Math.imul(h ^ byte, 0x01000193) >>> 0;
  return h >>> 0;
}
