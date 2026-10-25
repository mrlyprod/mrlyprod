export type Scale = { log: boolean; floor: number; top: number; hi: number; max: string; at: (t: string) => number };

export const KINDS: [string, string][] = [
  ["pins", "Pins"],
  ["log", "Log pins"],
  ["steps", "Steps"],
  ["sums", "Sums"],
  ["ratios", "Ratios"],
  ["diffs", "Differences"],
  ["digits", "Digits"],
];

const HEAD = 15;

const text = (t: string) => String(t ?? "").trim();

export const negative = (t: string) => text(t).startsWith("-");

const bare = (t: string) => {
  const s = text(t);
  return (negative(s) ? s.slice(1) : s).replace(/^0+(?=\d)/, "");
};

/* SCALE */

export function log10(t: string): number {
  const d = bare(t);
  if (!/^\d+$/.test(d)) return -Infinity;
  if (d === "0") return -Infinity;
  return Math.log10(Number(d.slice(0, HEAD))) + Math.max(0, d.length - HEAD);
}

export function span(values: string[]): { hi: number; lo: number } {
  const tops = values.map(log10).filter(Number.isFinite);
  return tops.length ? { hi: Math.max(...tops), lo: Math.min(...tops) } : { hi: 0, lo: 0 };
}

export function scale(rows: { terms: string[] }[], force = false): Scale {
  const all = rows.flatMap((row) => row.terms);
  const { hi, lo } = span(all);
  const max = all.reduce((best, t) => (log10(t) > log10(best) ? t : best), all[0] ?? "0");
  const log = force || hi - lo >= 2;
  const floor = log ? Math.floor(lo) : 0;
  const top = log ? Math.ceil(hi) : hi;
  const at = (t: string) => {
    const l = log10(t);
    if (l === -Infinity) return 0;
    if (!log) return Math.min(1, 10 ** (l - hi));
    return top > floor ? (l - floor) / (top - floor) : 1;
  };
  return { log, floor, top, hi, max, at };
}

export function decade(k: number): string {
  return k < 4 ? String(10 ** k) : `1e${k}`;
}

export function label(t: string): string {
  const d = bare(t);
  const sign = negative(t) ? "-" : "";
  if (d.length <= 7) return sign + d;
  return `${sign}${d[0]}.${d.slice(1, 3)}e${d.length - 1}`;
}

/* SERIES */

export function sums(terms: string[]): string[] {
  let total = 0n;
  return terms.map((t) => {
    total += BigInt(text(t));
    return total.toString();
  });
}

export function ratios(terms: string[]): (number | null)[] {
  const out: (number | null)[] = [];
  for (let i = 1; i < terms.length; i++) {
    const a = BigInt(text(terms[i - 1]));
    const b = BigInt(text(terms[i]));
    out.push(a === 0n ? null : Number((b * 100000n) / a) / 100000);
  }
  return out;
}

export function diffs(terms: string[], depth = 6): string[][] {
  const rows = [terms.map((t) => BigInt(text(t)))];
  while (rows.length <= depth && rows[rows.length - 1].length > 1) {
    const last = rows[rows.length - 1];
    rows.push(last.slice(1).map((v, i) => v - last[i]));
  }
  return rows.map((row) => row.map(String));
}

export function digits(t: string, base: number): number[] {
  const d = bare(t);
  if (!/^\d+$/.test(d)) return [];
  const q = Math.max(2, Math.min(36, Math.round(base)));
  return [...(q === 10 ? d : BigInt(d).toString(q))].map((c) => parseInt(c, 36));
}

/* AXIS */

export function ticks(
  n: number,
  start: number,
  step: number,
  every = Math.max(1, Math.ceil(n / 12)),
): [number, string][] {
  const out: [number, string][] = [];
  for (let i = 0; i < n; i += every) out.push([i, String(start + i * step)]);
  return out;
}
