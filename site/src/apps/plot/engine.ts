import type { Unit } from "../../lib/units/index.ts";

/* TYPES */

export type Num = Unit<"num">;

export type Pair = [string, string];

export type Rule = {
  order: number | null;
  recurrence: string;
  polynomial: string;
  root: number;
  growth: number;
  exponent: number;
  from: string;
  read: number;
  error: string;
};

export type Mix = { terms: string[]; cut: boolean; error: string };

export const OPS: [string, string][] = [
  ["add", "Add"],
  ["sub", "Subtract"],
  ["hadamard", "Multiply"],
  ["cauchy", "Convolve"],
  ["shift", "Shift"],
  ["decimate", "Decimate"],
  ["delta", "Differences"],
  ["sigma", "Sums"],
  ["scale", "Scale"],
];

export const PAIRED: readonly string[] = ["add", "sub", "hadamard", "cauchy"];

export const TAKES: readonly string[] = ["shift", "decimate", "scale"];

export const ARG = { min: -8, max: 12 };

const DIGITS = 6;

const WIDE = 37;

/* SPELLING */

const size = ([num, den]: Pair) => {
  const top = String(num).replace(/^-/, "");
  return String(den) === "1" ? top : `${top}/${den}`;
};

const sign = ([num]: Pair) => String(num).startsWith("-");

const zero = ([num]: Pair) => String(num) === "0";

export function recurrence(rule: Pair[]): string {
  let text = "a(n) =";
  rule.forEach((pair, i) => {
    if (zero(pair)) return;
    text += sign(pair) ? " - " : text.endsWith("=") ? " " : " + ";
    const weight = size(pair);
    if (weight !== "1") text += `${weight} `;
    text += `a(n-${i + 1})`;
  });
  return text.endsWith("=") ? `${text} 0` : text;
}

export function polynomial(poly: Pair[]): string {
  const top = poly.length - 1;
  let text = "";
  poly.forEach((pair, i) => {
    if (zero(pair)) return;
    const power = top - i;
    if (!text) text += sign(pair) ? "-" : "";
    else text += sign(pair) ? " - " : " + ";
    const weight = size(pair);
    const shown = weight !== "1" || power === 0;
    if (shown) text += weight;
    if (power > 0) text += `x${power > 1 ? `^${power}` : ""}`;
  });
  return text || "0";
}

export const decimal = (value: number | undefined) => (Number.isFinite(value) ? value!.toFixed(DIGITS) : "none");

/* WIDTH */

const width = (t: string) => String(t).replace(/^-/, "").length;

const most = (list: string[], n: number) => list.slice(0, n + 1).reduce((top, t) => Math.max(top, width(t)), 0);

export const held = (terms: string[]) => {
  const at = terms.findIndex((t) => width(t) > WIDE);
  return at < 0 ? terms : terms.slice(0, at);
};

export function room(op: string, a: string[], b: string[], arg: number): number {
  const n = PAIRED.includes(op) ? Math.min(a.length, b.length) : a.length;
  const fits = (i: number) => {
    if (op === "add" || op === "sub") return Math.max(width(a[i]), width(b[i])) + 1 <= WIDE;
    if (op === "hadamard") return width(a[i]) + width(b[i]) <= WIDE;
    if (op === "cauchy") return most(a, i) + most(b, i) + String(i + 1).length <= WIDE;
    if (op === "sigma") return most(a, i) + String(i + 1).length <= WIDE;
    if (op === "scale") return width(a[i]) + String(Math.abs(arg)).length <= WIDE;
    return width(a[i]) + 1 <= WIDE;
  };
  let at = 0;
  while (at < n && fits(at)) at++;
  return at;
}

/* RULE */

export function rule(num: Num, all: string[]): Rule {
  const terms = held(all);
  const none: Rule = {
    order: null,
    recurrence: "",
    polynomial: "",
    root: NaN,
    growth: NaN,
    exponent: NaN,
    from: "no rule fits, so no growth is read",
    read: terms.length,
    error: "",
  };
  if (terms.length < 2) return none;
  let found: Pair[] | undefined;
  try {
    found = num.blend.recurrence(terms);
  } catch (error) {
    return { ...none, error: String((error as { message?: unknown } | null | undefined)?.message ?? error) };
  }
  if (!found) return none;
  const root = found.length ? num.blend.growth(found) : NaN;
  const real = Number.isFinite(root) && root > 0;
  return {
    order: found.length,
    recurrence: recurrence(found),
    polynomial: polynomial(num.blend.characteristic(found)),
    root,
    growth: real ? root : NaN,
    exponent: real ? Math.log10(root) : NaN,
    from: real ? "the recurrence root" : "no positive real root, so no growth is read",
    read: terms.length,
    error: "",
  };
}

/* MIX */

export function combine(num: Num, left: string[], right: string[], op: string, arg: number): Mix {
  const blend = num.blend;
  const full = PAIRED.includes(op) ? Math.min(left.length, right.length) : left.length;
  const n = room(op, held(left), held(right), arg);
  const a = left.slice(0, n);
  const b = right.slice(0, n);
  const run = (
    {
      add: () => blend.add(a, b),
      sub: () => blend.sub(a, b),
      hadamard: () => blend.hadamard(a, b),
      cauchy: () => blend.cauchy(a, b),
      shift: () => blend.shift(a, arg),
      decimate: () => blend.decimate(a, arg, 0),
      delta: () => blend.delta(a),
      sigma: () => blend.sigma(a),
      scale: () => blend.scale(a, String(arg)),
    } as Partial<Record<string, () => string[]>>
  )[op];
  if (!run) return { terms: [], cut: false, error: `no operation ${op}` };
  try {
    return { terms: n ? run() : [], cut: n < full, error: "" };
  } catch (error) {
    return {
      terms: [],
      cut: false,
      error: String((error as { message?: unknown } | null | undefined)?.message ?? error),
    };
  }
}

export const named = (op: string) => OPS.find(([one]) => one === op)?.[1].toLowerCase() ?? op;
