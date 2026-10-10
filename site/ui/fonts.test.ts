import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { brotliDecompressSync } from "node:zlib";
import katex from "katex";
import { front, render } from "../kit/md/md.ts";
import { state } from "../scripts/site.ts";

const site = resolve(import.meta.dir, "..");
const repo = resolve(site, "..");
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

const mathml = (tex: string, display: boolean) => katex.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display });

function pages(): [string, string][] {
  const shell: [string, string] = ["ui/index.html", readFileSync(join(import.meta.dir, "index.html"), "utf8")];
  const code = ["ui", "lib", "apps"].flatMap((top) => [...new Bun.Glob(`${top}/**/*.{js,jsx}`).scanSync(site)].filter((file) => !file.includes(".test.")).sort());
  return [shell, ...state().rows.map((row): [string, string] => {
    const body = row.source ? render(front(readFileSync(join(repo, row.source), "utf8")).body, { math: mathml }) : "";
    return [row.route, `<h1>${row.title}</h1><p>${row.lead}</p>${body}`];
  }), ...code.map((file): [string, string] => [file, readFileSync(join(site, file), "utf8")])];
}

function words(html: string): string {
  return html
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

test("every codepoint a route prints is drawn by a shipped face", () => {
  const list = pages();
  const draws = faces();
  const gap = new Map<number, string>();
  for (const [route, html] of list) {
    for (const ch of words(html)) {
      const cp = ch.codePointAt(0)!;
      if (cp <= 0x7f || INERT.test(ch) || gap.has(cp) || draws.some((one) => one(cp))) continue;
      gap.set(cp, route);
    }
  }
  const hex = (cp: number) => `U+${cp.toString(16).toUpperCase().padStart(4, "0")}`;
  expect([...gap].sort(([a], [b]) => a - b).map(([cp, route]) => `${hex(cp)} ${String.fromCodePoint(cp)} ${route}`)).toEqual([]);
});
