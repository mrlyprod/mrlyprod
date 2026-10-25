import { useEffect, useState } from "react";
import type { Raw, Value } from "../app/spec.ts";
import { held, hold } from "../keep.ts";

/* TYPES */

export type Design = { dim: number; base: number; code: string };

export type Loose = { dim: number | string; base: number | string; code?: string | number | null };

export type Slot = {
  dim: number | string;
  base: number | string;
  code: string;
  dims?: number[];
  bases?: number[];
  spaces?: [number, number][];
  blank?: string;
  when?: (value: Value) => boolean;
  holds?: (design: Design, value: Value) => boolean;
};

export type Stored = { current: Design | null; recent: Design[]; saved: Design[] };

type Key = "dim" | "base" | "code";

/* SLOT */

const KEYS: Key[] = ["dim", "base", "code"];
const MOST = 12;
const TOLD = "designs";

export const dims = (slot: Slot) => (typeof slot.dim === "string" ? (slot.dims ?? [2, 3]) : [slot.dim]);

export const bases = (slot: Slot) => (typeof slot.base === "string" ? (slot.bases ?? [2, 3]) : [slot.base]);

export const keys = (slot: Slot) => KEYS.map((one) => slot[one]).filter((one) => typeof one === "string");

export function read(slot: Slot, value: Raw): Design {
  const at = (one: "dim" | "base") => (typeof slot[one] === "string" ? Number(value[slot[one]]) : slot[one]);
  return { dim: at("dim"), base: at("base"), code: String(value[slot.code] ?? "").trim() };
}

export const allows = (slot: Slot, dim: number | string, base: number | string) =>
  dims(slot).includes(Number(dim)) &&
  bases(slot).includes(Number(base)) &&
  (!slot.spaces || slot.spaces.some(([d, b]) => d === Number(dim) && b === Number(base)));

export const fits = (slot: Slot, design: Loose | null | undefined) =>
  Boolean(design) && allows(slot, design!.dim, design!.base) && String(design!.code ?? "").trim() !== "";

export function place(slot: Slot, design: Loose): Value {
  return Object.fromEntries(
    KEYS.filter((one) => typeof slot[one] === "string").map((one) => [
      slot[one],
      one === "code" ? String(design.code) : Number(design[one]),
    ]),
  );
}

export const same = (a: Loose, b: Loose) =>
  Boolean(a && b) &&
  Number(a.dim) === Number(b.dim) &&
  Number(a.base) === Number(b.base) &&
  String(a.code) === String(b.code);

const clean = (design: Loose): Design => ({
  dim: Number(design.dim),
  base: Number(design.base),
  code: String(design.code),
});

/* STORE */

const list = (name: string) => {
  const out = held<Design[]>(name, []);
  return Array.isArray(out) ? out.filter((one) => one && one.code !== undefined) : [];
};

export const current = () => held<Design | null>("design", null);

export const recent = () => list("designs-recent");

export const saved = () => list("designs-saved");

export const starred = (design: Loose) => saved().some((one) => same(one, design));

const tell = () => typeof dispatchEvent === "function" && dispatchEvent(new Event(TOLD));

export function choose(design: Loose | null | undefined) {
  if (!design || String(design.code ?? "").trim() === "") return;
  const one = clean(design);
  hold("design", one);
  hold("designs-recent", [one, ...recent().filter((was) => !same(was, one))].slice(0, MOST));
  tell();
}

export function star(design: Loose, on = !starred(design)) {
  const one = clean(design);
  const rest = saved().filter((was) => !same(was, one));
  hold("designs-saved", on ? [one, ...rest] : rest);
  tell();
}

export function useDesigns(): Stored {
  const [, bump] = useState(0);
  useEffect(() => {
    const again = () => bump((was) => was + 1);
    addEventListener(TOLD, again);
    addEventListener("storage", again);
    return () => {
      removeEventListener(TOLD, again);
      removeEventListener("storage", again);
    };
  }, []);
  return { current: current(), recent: recent(), saved: saved() };
}
