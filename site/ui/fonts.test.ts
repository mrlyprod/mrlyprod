import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { brotliDecompressSync } from "node:zlib";

const site = resolve(import.meta.dir, "..");
const dist = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(site, "dist");
const fonts = join(import.meta.dir, "fonts");
const FACE = /@font-face\s*\{([^}]*)\}/g;
const SRC = /src:\s*url\("?([^")]+)"?\)/;
const FAMILY = /font-family:\s*"([^"]+)"/;
const PIXEL = "MrlyFont";
const RANGE = /unicode-range:\s*([^;]+);/;
const DROP = /<(head|script|style|annotation)\b[^>]*>[\s\S]*?<\/\1>/gi;
const TAG = /<[^>]*>/g;
const INERT = /[\s\p{Cf}]/u;
const NAMED: Record<string, string> = { amp: "&", lt: "<", gt: ">", quot: '"', apos: "'", nbsp: " " };
const KNOWN = [
  "U+0338", "U+0394", "U+0398", "U+03A3", "U+03A6", "U+03A9", "U+03B1", "U+03B2", "U+03B3", "U+03B4", "U+03B5",
  "U+03B6", "U+03B8", "U+03BA", "U+03BB", "U+03BC", "U+03BD", "U+03C0", "U+03C1", "U+03C3", "U+03C4", "U+03C6",
  "U+03C7", "U+03C8", "U+03C9", "U+03D5", "U+203E", "U+2113", "U+2192", "U+21A6", "U+2200", "U+2208", "U+220F",
  "U+2211", "U+221E", "U+2223", "U+2224", "U+2229", "U+222A", "U+222B", "U+224D", "U+2260", "U+2261", "U+2264",
  "U+2265", "U+226A", "U+2282", "U+2286", "U+2295", "U+2297", "U+22C3", "U+22EF", "U+2308", "U+2309", "U+230A",
  "U+230B", "U+266D", "U+27E8", "U+27E9", "U+27F6", "U+2AAF",
];

function pages(dir: string): string[] {
  if (!existsSync(dir)) return [];
  const out: string[] = [];
  for (const item of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, item.name);
    if (item.isDirectory() && path !== join(dist, "raw")) out.push(...pages(path));
    else if (item.name.endsWith(".html")) out.push(path);
  }
  return out;
}

function words(file: string): string {
  return readFileSync(file, "utf8")
    .replace(DROP, " ")
    .replace(TAG, " ")
    .replace(/&#x([0-9a-f]+);/gi, (_, n) => String.fromCodePoint(parseInt(n, 16)))
    .replace(/&#(\d+);/g, (_, n) => String.fromCodePoint(Number(n)))
    .replace(/&([a-z]+);/gi, (whole, name: string) => NAMED[name.toLowerCase()] ?? whole);
}

function base128(data: Buffer, at: { n: number }): number {
  let value = 0;
  for (let i = 0; i < 5; i++) {
    const byte = data[at.n++]!;
    value = value * 128 + (byte & 0x7f);
    if (!(byte & 0x80)) break;
  }
  return value;
}

function table(file: string): Buffer {
  const data = readFileSync(file);
  if (data.toString("latin1", 0, 4) !== "wOF2") throw new Error(`fonts: ${file} is not woff2`);
  const at = { n: 48 };
  let offset = 0;
  let found: [number, number] | null = null;
  for (let i = 0; i < data.readUInt16BE(12); i++) {
    const flags = data[at.n++]!;
    const tag = flags & 63;
    if (tag === 63) at.n += 4;
    const whole = base128(data, at);
    const changed = tag === 10 || tag === 11 ? flags >> 6 === 0 : flags >> 6 !== 0;
    const length = changed ? base128(data, at) : whole;
    if (tag === 0) found = [offset, whole];
    offset += length;
  }
  if (!found) throw new Error(`fonts: ${file} has no cmap`);
  const stream = brotliDecompressSync(data.subarray(at.n, at.n + data.readUInt32BE(20)));
  return stream.subarray(found[0], found[0] + found[1]);
}

function cmap(data: Buffer): Set<number> {
  const out = new Set<number>();
  for (let i = 0; i < data.readUInt16BE(2); i++) {
    const off = data.readUInt32BE(4 + i * 8 + 4);
    const format = data.readUInt16BE(off);
    if (format === 4) {
      const segs = data.readUInt16BE(off + 6) / 2;
      const ends = off + 14;
      const starts = ends + segs * 2 + 2;
      const deltas = starts + segs * 2;
      const ranges = deltas + segs * 2;
      for (let s = 0; s < segs; s++) {
        const end = data.readUInt16BE(ends + s * 2);
        const start = data.readUInt16BE(starts + s * 2);
        const delta = data.readUInt16BE(deltas + s * 2);
        const range = data.readUInt16BE(ranges + s * 2);
        for (let cp = start; cp <= end && cp !== 0xffff; cp++) {
          const glyph = range ? data.readUInt16BE(ranges + s * 2 + range + (cp - start) * 2) : cp;
          if (glyph && (glyph + delta) & 0xffff) out.add(cp);
        }
      }
    } else if (format === 12) {
      for (let g = 0; g < data.readUInt32BE(off + 12); g++) {
        const row = off + 16 + g * 12;
        for (let cp = data.readUInt32BE(row); cp <= data.readUInt32BE(row + 4); cp++) out.add(cp);
      }
    }
  }
  return out;
}

function faces(): ((cp: number) => boolean)[] {
  const all = [...readFileSync(join(fonts, "fonts.css"), "utf8").matchAll(FACE)];
  return all.filter((face) => face[1]!.match(FAMILY)?.[1] !== PIXEL).map((face) => {
    const src = face[1]!.match(SRC)![1]!;
    const drawn = cmap(table(join(fonts, src)));
    const range = face[1]!.match(RANGE)?.[1]!.split(",").map((part) => {
      const [a, b] = part.trim().toLowerCase().replace("u+", "").split("-");
      return [parseInt(a!, 16), parseInt(b ?? a!, 16)] as const;
    });
    return (cp: number) => drawn.has(cp) && (!range || range.some(([a, b]) => cp >= a && cp <= b));
  });
}

test("every codepoint a built page prints is drawn by a shipped face, but the known gap", () => {
  const files = pages(dist).sort();
  if (!files.length) return;
  const draws = faces();
  const gap = new Map<number, string>();
  for (const file of files) {
    for (const ch of words(file)) {
      const cp = ch.codePointAt(0)!;
      if (cp <= 0x7f || INERT.test(ch) || gap.has(cp) || draws.some((one) => one(cp))) continue;
      gap.set(cp, relative(dist, file));
    }
  }
  const hex = (cp: number) => `U+${cp.toString(16).toUpperCase().padStart(4, "0")}`;
  const found = [...gap.keys()].sort((a, b) => a - b);
  const fresh = found.filter((cp) => !KNOWN.includes(hex(cp))).map((cp) => `${hex(cp)} ${String.fromCodePoint(cp)} ${gap.get(cp)}`);
  const gone = KNOWN.filter((one) => !found.some((cp) => hex(cp) === one)).map((one) => `${one} is drawn or unused now, drop it`);
  expect([...fresh, ...gone]).toEqual([]);
});
