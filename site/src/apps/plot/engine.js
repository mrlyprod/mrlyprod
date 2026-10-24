export const OPS = [
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

export const PAIRED = ["add", "sub", "hadamard", "cauchy"];

export const TAKES = ["shift", "decimate", "scale"];

export const ARG = { min: -8, max: 12 };

const DIGITS = 6;

const WIDE = 37;

/* SPELLING */

const size = ([num, den]) => {
  const top = String(num).replace(/^-/, "");
  return String(den) === "1" ? top : `${top}/${den}`;
};

const sign = ([num]) => String(num).startsWith("-");

const zero = ([num]) => String(num) === "0";

export function recurrence(rule) {
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

export function polynomial(poly) {
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

export const decimal = (value) => (Number.isFinite(value) ? value.toFixed(DIGITS) : "none");

/* WIDTH */

const width = (t) => String(t).replace(/^-/, "").length;

const most = (list, n) => list.slice(0, n + 1).reduce((top, t) => Math.max(top, width(t)), 0);

export const held = (terms) => {
  const at = terms.findIndex((t) => width(t) > WIDE);
  return at < 0 ? terms : terms.slice(0, at);
};

export function room(op, a, b, arg) {
  const n = PAIRED.includes(op) ? Math.min(a.length, b.length) : a.length;
  const fits = (i) => {
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

export function rule(num, all) {
  const terms = held(all);
  const none = {
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
  let found;
  try {
    found = num.blend.recurrence(terms);
  } catch (error) {
    return { ...none, error: String(error?.message ?? error) };
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

export function combine(num, left, right, op, arg) {
  const blend = num.blend;
  const full = PAIRED.includes(op) ? Math.min(left.length, right.length) : left.length;
  const n = room(op, held(left), held(right), arg);
  const a = left.slice(0, n);
  const b = right.slice(0, n);
  const run = {
    add: () => blend.add(a, b),
    sub: () => blend.sub(a, b),
    hadamard: () => blend.hadamard(a, b),
    cauchy: () => blend.cauchy(a, b),
    shift: () => blend.shift(a, arg),
    decimate: () => blend.decimate(a, arg, 0),
    delta: () => blend.delta(a),
    sigma: () => blend.sigma(a),
    scale: () => blend.scale(a, String(arg)),
  }[op];
  if (!run) return { terms: [], cut: false, error: `no operation ${op}` };
  try {
    return { terms: n ? run() : [], cut: n < full, error: "" };
  } catch (error) {
    return { terms: [], cut: false, error: String(error?.message ?? error) };
  }
}

export const named = (op) => OPS.find(([one]) => one === op)?.[1].toLowerCase() ?? op;
