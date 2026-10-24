import { useEffect, useState } from "react";
import { held, hold } from "../keep.js";

/* SLOT */

const KEYS = ["dim", "base", "code"];
const MOST = 12;
const TOLD = "designs";

export const dims = (slot) => (typeof slot.dim === "string" ? (slot.dims ?? [2, 3]) : [slot.dim]);

export const bases = (slot) => (typeof slot.base === "string" ? (slot.bases ?? [2, 3]) : [slot.base]);

export const keys = (slot) => KEYS.map((one) => slot[one]).filter((one) => typeof one === "string");

export function read(slot, value) {
  const at = (one) => (typeof slot[one] === "string" ? Number(value[slot[one]]) : slot[one]);
  return { dim: at("dim"), base: at("base"), code: String(value[slot.code] ?? "").trim() };
}

export const allows = (slot, dim, base) =>
  dims(slot).includes(Number(dim)) &&
  bases(slot).includes(Number(base)) &&
  (!slot.spaces || slot.spaces.some(([d, b]) => d === Number(dim) && b === Number(base)));

export const fits = (slot, design) =>
  Boolean(design) && allows(slot, design.dim, design.base) && String(design.code ?? "").trim() !== "";

export function place(slot, design) {
  return Object.fromEntries(
    KEYS.filter((one) => typeof slot[one] === "string").map((one) => [
      slot[one],
      one === "code" ? String(design.code) : Number(design[one]),
    ]),
  );
}

export const same = (a, b) =>
  Boolean(a && b) &&
  Number(a.dim) === Number(b.dim) &&
  Number(a.base) === Number(b.base) &&
  String(a.code) === String(b.code);

const clean = (design) => ({ dim: Number(design.dim), base: Number(design.base), code: String(design.code) });

/* STORE */

const list = (name) => {
  const out = held(name, []);
  return Array.isArray(out) ? out.filter((one) => one && one.code !== undefined) : [];
};

export const current = () => held("design");

export const recent = () => list("designs-recent");

export const saved = () => list("designs-saved");

export const starred = (design) => saved().some((one) => same(one, design));

const tell = () => typeof dispatchEvent === "function" && dispatchEvent(new Event(TOLD));

export function choose(design) {
  if (!design || String(design.code ?? "").trim() === "") return;
  const one = clean(design);
  hold("design", one);
  hold("designs-recent", [one, ...recent().filter((was) => !same(was, one))].slice(0, MOST));
  tell();
}

export function star(design, on = !starred(design)) {
  const one = clean(design);
  const rest = saved().filter((was) => !same(was, one));
  hold("designs-saved", on ? [one, ...rest] : rest);
  tell();
}

export function useDesigns() {
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
