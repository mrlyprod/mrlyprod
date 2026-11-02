/* TYPES */

export type Q = { n: bigint; d: bigint };

export const DIGITS = 1000;

export const POWER = 1000;

class Bad extends Error {}

/* RATIONALS */

const abs = (a: bigint) => (a < 0n ? -a : a);

function gcd(a: bigint, b: bigint): bigint {
  let [x, y] = [abs(a), abs(b)];
  while (y) [x, y] = [y, x % y];
  return x;
}

const size = (a: bigint) => abs(a).toString().length;

function q(n: bigint, d: bigint): Q {
  if (d === 0n) throw new Bad("division by zero");
  const g = gcd(n, d) || 1n;
  const sign = d < 0n ? -1n : 1n;
  const out = { n: (sign * n) / g, d: (sign * d) / g };
  if (size(out.n) > DIGITS || size(out.d) > DIGITS) throw new Bad(`the result passes ${DIGITS} digits`);
  return out;
}

const add = (a: Q, b: Q) => q(a.n * b.d + b.n * a.d, a.d * b.d);

const sub = (a: Q, b: Q) => q(a.n * b.d - b.n * a.d, a.d * b.d);

const mul = (a: Q, b: Q) => q(a.n * b.n, a.d * b.d);

const div = (a: Q, b: Q) => q(a.n * b.d, a.d * b.n);

function pow(a: Q, b: Q): Q {
  if (b.d !== 1n || abs(b.n) > BigInt(POWER)) throw new Bad(`an exponent is a whole number from -${POWER} to ${POWER}`);
  const e = Number(abs(b.n));
  if ((size(a.n) + size(a.d) - 2) * e > 2 * DIGITS) throw new Bad(`the result passes ${DIGITS} digits`);
  const up = q(a.n ** BigInt(e), a.d ** BigInt(e));
  return b.n < 0n ? q(up.d, up.n) : up;
}

/* TEXT */

export function shown(value: Q): string {
  if (value.d === 1n) return value.n.toString();
  let rest = value.d;
  let twos = 0;
  let fives = 0;
  while (rest % 2n === 0n) [rest, twos] = [rest / 2n, twos + 1];
  while (rest % 5n === 0n) [rest, fives] = [rest / 5n, fives + 1];
  if (rest !== 1n) return `${value.n}/${value.d}`;
  const places = Math.max(twos, fives);
  const scaled = abs((value.n * 10n ** BigInt(places)) / value.d)
    .toString()
    .padStart(places + 1, "0");
  return `${value.n < 0n ? "-" : ""}${scaled.slice(0, -places)}.${scaled.slice(-places)}`;
}

export function exact(text: string): Q {
  const [whole, over] = text.split("/");
  if (over !== undefined) return q(BigInt(whole!), BigInt(over));
  const [head, tail = ""] = whole!.split(".");
  return q(BigInt(`${head}${tail}`), 10n ** BigInt(tail.length));
}

/* PARSE */

const TOKEN = /\s*(?:(\d+(?:\.\d+)?|\.\d+)|(ans)|([-+*/^()]))/y;

type Token = { kind: "number" | "ans" | "op"; text: string; at: number };

function tokens(expr: string): Token[] {
  const out: Token[] = [];
  let at = 0;
  while (at < expr.length) {
    if (!expr.slice(at).trim()) break;
    TOKEN.lastIndex = at;
    const m = TOKEN.exec(expr);
    if (!m) {
      const where = at + (expr.slice(at).length - expr.slice(at).trimStart().length);
      throw new Bad(`unexpected "${expr[where]}" at ${where + 1}`);
    }
    const kind = m[1] ? "number" : m[2] ? "ans" : "op";
    out.push({ kind, text: (m[1] ?? m[2] ?? m[3])!, at: TOKEN.lastIndex - (m[1] ?? m[2] ?? m[3])!.length });
    at = TOKEN.lastIndex;
  }
  return out;
}

export function evaluate(expr: string, ans: Q | null): Q | string {
  try {
    const list = tokens(expr);
    let i = 0;
    const peek = () => list[i];
    const take = (text: string) => {
      if (peek()?.text !== text || peek()?.kind !== "op") return false;
      i++;
      return true;
    };
    const sum = (): Q => {
      let out = product();
      for (;;) {
        if (take("+")) out = add(out, product());
        else if (take("-")) out = sub(out, product());
        else return out;
      }
    };
    const product = (): Q => {
      let out = unary();
      for (;;) {
        if (take("*")) out = mul(out, unary());
        else if (take("/")) out = div(out, unary());
        else return out;
      }
    };
    const unary = (): Q => {
      if (take("-")) {
        const one = unary();
        return { n: -one.n, d: one.d };
      }
      if (take("+")) return unary();
      const base = atom();
      return take("^") ? pow(base, unary()) : base;
    };
    const atom = (): Q => {
      const one = list[i++];
      if (!one) throw new Bad("the expression ends early");
      if (one.kind === "number") return exact(one.text.startsWith(".") ? `0${one.text}` : one.text);
      if (one.kind === "ans") {
        if (!ans) throw new Bad("no answer yet for ans");
        return ans;
      }
      if (one.text === "(") {
        const inner = sum();
        if (!take(")")) throw new Bad(`a ( at ${one.at + 1} is never closed`);
        return inner;
      }
      throw new Bad(`unexpected "${one.text}" at ${one.at + 1}`);
    };
    if (!list.length) throw new Bad("the expression is empty");
    const value = sum();
    const left = peek();
    if (left) throw new Bad(`unexpected "${left.text}" at ${left.at + 1}`);
    return value;
  } catch (thrown) {
    if (thrown instanceof Bad) return thrown.message;
    throw thrown;
  }
}
