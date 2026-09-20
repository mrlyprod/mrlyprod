import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const site = resolve(import.meta.dir, "../..");
const dist = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(site, "dist");
const master = resolve(site, "..", "files", "fonts", "symbols.ttf");
const READS = ["wiki", "papers"];
const FACE = /@font-face\s*\{[^}]*?font-family:\s*"Noto Sans Symbols 2"[^}]*?unicode-range:\s*([^;]+);[^}]*\}/;
const DROP = /<(script|style)\b[^>]*>[\s\S]*?<\/\1>/gi;
const TAG = /<[^>]*>/g;
const NAMED: Record<string, string> = { amp: "&", lt: "<", gt: ">", quot: '"', apos: "'", nbsp: " " };

function pages(dir: string): string[] {
  if (!existsSync(dir)) return [];
  const out: string[] = [];
  for (const item of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, item.name);
    if (item.isDirectory()) out.push(...pages(path));
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

function ranged(): Set<number> {
  const found = readFileSync(join(import.meta.dir, "fonts", "fonts.css"), "utf8").match(FACE);
  if (!found) throw new Error("fonts: the symbols face has no unicode-range");
  const out = new Set<number>();
  for (const part of found[1].split(",")) {
    const [a, b] = part.trim().toLowerCase().replace("u+", "").split("-");
    for (let cp = parseInt(a, 16); cp <= parseInt(b ?? a, 16); cp++) out.add(cp);
  }
  return out;
}

function cmap(file: string): Set<number> {
  const data = readFileSync(file);
  const at = (n: number) => data.readUInt32BE(n);
  let head = 0;
  for (let i = 0; i < data.readUInt16BE(4); i++) {
    const row = 12 + i * 16;
    if (data.toString("latin1", row, row + 4) === "cmap") head = at(row + 8);
  }
  const out = new Set<number>();
  for (let i = 0; i < data.readUInt16BE(head + 2); i++) {
    const off = head + at(head + 4 + i * 8 + 4);
    const format = data.readUInt16BE(off);
    if (format === 4) {
      const segs = data.readUInt16BE(off + 6) / 2;
      for (let s = 0; s < segs; s++) {
        const end = data.readUInt16BE(off + 14 + s * 2);
        for (let cp = data.readUInt16BE(off + 16 + segs * 2 + s * 2); cp <= end && cp !== 0xffff; cp++) out.add(cp);
      }
    } else if (format === 12) {
      for (let g = 0; g < at(off + 12); g++) {
        const row = off + 16 + g * 12;
        for (let cp = at(row); cp <= at(row + 4); cp++) out.add(cp);
      }
    }
  }
  return out;
}

test("the shipped symbols subset draws every symbol the wiki and the papers print", () => {
  const files = READS.flatMap((name) => pages(join(dist, name)));
  if (!files.length || !existsSync(master)) return;
  const drawn = cmap(master);
  const kept = ranged();
  const missing = new Set<string>();
  for (const file of files) {
    for (const ch of words(file)) {
      const cp = ch.codePointAt(0)!;
      if (cp <= 0x7f || !drawn.has(cp) || kept.has(cp)) continue;
      missing.add(`U+${cp.toString(16).padStart(4, "0")}`);
    }
  }
  expect([...missing].sort()).toEqual([]);
});
