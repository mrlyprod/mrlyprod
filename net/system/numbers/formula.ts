/* TYPES */

export type Q = { n: bigint; d: bigint };

export type Node =
  | { kind: "num"; value: Q }
  | { kind: "ref"; cell: string }
  | { kind: "sum"; cells: string[] }
  | { kind: "neg"; arg: Node }
  | { kind: "op"; op: "+" | "-" | "*" | "/"; left: Node; right: Node };

export type Cells = Record<string, string>;

export const COLS = "ABCDEFGH";

export const ROWS = 20;

export const DIGITS = 60;

export const USAGE = "a formula is =, numbers, cells A1 to H20, + - * / ( ) and SUM(A1:B3)";

class Bad extends Error {}

/* EXACT */

const abs = (a: bigint) => (a < 0n ? -a : a);

function gcd(a: bigint, b: bigint): bigint {
  let [x, y] = [abs(a), abs(b)];
  while (y) [x, y] = [y, x % y];
  return x;
}

function q(n: bigint, d: bigint): Q {
  if (d === 0n) throw new Bad("#DIV/0");
  const g = gcd(n, d) || 1n;
  const sign = d < 0n ? -1n : 1n;
  const out = { n: (sign * n) / g, d: (sign * d) / g };
  if (abs(out.n).toString().length > DIGITS || out.d.toString().length > DIGITS) throw new Bad("#BIG");
  return out;
}

const ZERO: Q = { n: 0n, d: 1n };

const add = (a: Q, b: Q) => q(a.n * b.d + b.n * a.d, a.d * b.d);

const sub = (a: Q, b: Q) => q(a.n * b.d - b.n * a.d, a.d * b.d);

const mul = (a: Q, b: Q) => q(a.n * b.n, a.d * b.d);

const div = (a: Q, b: Q) => q(a.n * b.d, a.d * b.n);

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

const NUMBER = /^-?(\d+(\.\d+)?|\.\d+)$/;

export function exact(text: string): Q | null {
  if (!NUMBER.test(text)) return null;
  const minus = text.startsWith("-");
  const [head = "", tail = ""] = (minus ? text.slice(1) : text).split(".");
  const n = BigInt(`${head || "0"}${tail}`);
  return q(minus ? -n : n, 10n ** BigInt(tail.length));
}

/* CELLS */

export const cellOf = (col: string, row: number) => `${col}${row}`;

const inGrid = (cell: string) => {
  const m = cell.match(/^([A-Z])(\d+)$/);
  return !!m && COLS.includes(m[1]!) && Number(m[2]) >= 1 && Number(m[2]) <= ROWS;
};

function range(from: string, to: string): string[] {
  const [c1, r1] = [COLS.indexOf(from[0]!), Number(from.slice(1))];
  const [c2, r2] = [COLS.indexOf(to[0]!), Number(to.slice(1))];
  const out: string[] = [];
  for (let c = Math.min(c1, c2); c <= Math.max(c1, c2); c++)
    for (let r = Math.min(r1, r2); r <= Math.max(r1, r2); r++) out.push(cellOf(COLS[c]!, r));
  return out;
}

/* PARSE */

const TOKEN = /\s*(?:(\d+(?:\.\d+)?|\.\d+)|([A-Z]+\d*)|([-+*/():,]))/y;

type Token = { kind: "num" | "word" | "mark"; text: string };

function tokens(expr: string): Token[] {
  const out: Token[] = [];
  TOKEN.lastIndex = 0;
  let at = 0;
  while (at < expr.length) {
    if (!expr.slice(at).trim()) break;
    const m = TOKEN.exec(expr);
    if (!m || m.index !== at) throw new Bad(USAGE);
    out.push({ kind: m[1] !== undefined ? "num" : m[2] !== undefined ? "word" : "mark", text: m[1] ?? m[2] ?? m[3]! });
    at = TOKEN.lastIndex;
  }
  return out;
}

export function parse(formula: string): Node {
  const list = tokens(formula.slice(1));
  let i = 0;
  const peek = () => list[i];
  const take = (text?: string): Token => {
    const one = list[i++];
    if (!one || (text !== undefined && one.text !== text)) throw new Bad(USAGE);
    return one;
  };
  const cell = (text: string) => {
    if (!inGrid(text)) throw new Bad(`cells run A1 to ${COLS.at(-1)}${ROWS}, not ${text}`);
    return text;
  };
  const factor = (): Node => {
    const one = take();
    if (one.kind === "num") return { kind: "num", value: exact(one.text)! };
    if (one.text === "-") return { kind: "neg", arg: factor() };
    if (one.text === "(") {
      const inner = expr();
      take(")");
      return inner;
    }
    if (one.text === "SUM") {
      take("(");
      const cells: string[] = [];
      for (;;) {
        const from = cell(take().text);
        if (peek()?.text === ":") {
          take(":");
          cells.push(...range(from, cell(take().text)));
        } else cells.push(from);
        if (peek()?.text !== ",") break;
        take(",");
      }
      take(")");
      return { kind: "sum", cells };
    }
    if (one.kind === "word") return { kind: "ref", cell: cell(one.text) };
    throw new Bad(USAGE);
  };
  const term = (): Node => {
    let left = factor();
    while (peek()?.text === "*" || peek()?.text === "/") {
      const op = take().text as "*" | "/";
      left = { kind: "op", op, left, right: factor() };
    }
    return left;
  };
  const expr = (): Node => {
    let left = term();
    while (peek()?.text === "+" || peek()?.text === "-") {
      const op = take().text as "+" | "-";
      left = { kind: "op", op, left, right: term() };
    }
    return left;
  };
  const out = expr();
  if (i !== list.length) throw new Bad(USAGE);
  return out;
}

export function checked(text: string): string | null {
  if (!text.startsWith("=")) return null;
  try {
    parse(text);
    return null;
  } catch (thrown) {
    if (thrown instanceof Bad) return thrown.message;
    throw thrown;
  }
}

/* EVALUATE */

export function values(cells: Cells): Record<string, string> {
  const done = new Map<string, Q | string>();
  const open = new Set<string>();
  const value = (cell: string): Q | string => {
    if (done.has(cell)) return done.get(cell)!;
    if (open.has(cell)) throw new Bad("#CYCLE");
    const text = cells[cell] ?? "";
    let out: Q | string;
    if (!text) out = ZERO;
    else if (text.startsWith("=")) {
      open.add(cell);
      try {
        out = run(parse(text));
      } catch (thrown) {
        if (!(thrown instanceof Bad)) throw thrown;
        out = thrown.message;
      }
      open.delete(cell);
    } else out = exact(text) ?? text;
    done.set(cell, out);
    return out;
  };
  const num = (cell: string): Q => {
    const got = value(cell);
    if (typeof got === "string") throw new Bad(got.startsWith("#") ? got : "#TEXT");
    return got;
  };
  const run = (node: Node): Q => {
    switch (node.kind) {
      case "num":
        return node.value;
      case "ref":
        return num(node.cell);
      case "sum":
        return node.cells.reduce((sum, cell) => add(sum, num(cell)), ZERO);
      case "neg":
        return sub(ZERO, run(node.arg));
      case "op": {
        const [a, b] = [run(node.left), run(node.right)];
        return node.op === "+" ? add(a, b) : node.op === "-" ? sub(a, b) : node.op === "*" ? mul(a, b) : div(a, b);
      }
    }
  };
  return Object.fromEntries(
    Object.keys(cells)
      .sort()
      .map((cell) => {
        const got = value(cell);
        return [cell, typeof got === "string" ? got : shown(got)];
      }),
  );
}
