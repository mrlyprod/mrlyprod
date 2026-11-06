import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import type { Color, Pen, Point } from "@mrlyprod/draw";
import { raster } from "@mrlyprod/draw";
import { board } from "@mrlyprod/draw/frame.ts";
import { letters } from "../../kit/font/font.ts";
import type { Glyphs } from "../../shell/design.ts";
import { EMOJIS, ICONS } from "../../shell/design.ts";
import { png, straight } from "./png.ts";

const REPO = resolve(import.meta.dir, "..", "..");
const TOP = dirname(REPO);
const DATA_DIR = join(TOP, "data", relative(TOP, import.meta.dir));
const KINDS = { icons: [...ICONS], emojis: Object.keys(EMOJIS) } as const;
const USAGE = "usage: bun run glyphs <design> [icons|emojis] [--cols n]";
const CELL = [248, 190] as const;
const PANEL = 112;
const BIG = 96;
const SMALL = [24, 16];
const PAGE: Color = [128, 128, 128, 255];
const WHITE: Color = [255, 255, 255, 255];
const TONES: { ground: Color; ink: Color }[] = [
  { ground: [255, 255, 255, 255], ink: [17, 17, 17, 255] },
  { ground: [17, 17, 17, 255], ink: [242, 242, 242, 255] },
];

type Kind = keyof typeof KINDS;

function lens(pen: Pen, ox: number, oy: number, k: number, box: number): Pen {
  const p = ([x, y]: Point): Point => [ox + x * k, oy + y * k];
  return {
    ...board(box, box),
    rect: (x, y, w, h, c) => pen.rect(ox + x * k, oy + y * k, w * k, h * k, c),
    round_rect: (x, y, w, h, r, c) => pen.round_rect(ox + x * k, oy + y * k, w * k, h * k, r * k, c),
    disc: (cx, cy, r, c) => pen.disc(ox + cx * k, oy + cy * k, r * k, c),
    ring: (cx, cy, r, thick, c) => pen.ring(ox + cx * k, oy + cy * k, r * k, thick * k, c),
    segment: (a, b, thick, c) => pen.segment(p(a), p(b), thick * k, c),
    polyline: (pts, thick, c) => pen.polyline(pts.map(p), thick * k, c),
    triangle: (a, b, c, color) => pen.triangle(p(a), p(b), p(c), color),
    polygon: (pts, c) => pen.polygon(pts.map(p), c),
    arc: (center, r, angles, thick, c) => pen.arc(p(center), r * k, angles, thick * k, c),
    image: (x, y, w, h, pixels) => pen.image(ox + x * k, oy + y * k, w * k, h * k, pixels),
  };
}

function label(pen: Pen, text: string, x: number, y: number) {
  const { grid } = letters(text.replaceAll("-", " "));
  grid.forEach((row, r) => {
    row.forEach((on, c) => {
      if (on) pen.rect(x + c * 2, y + r * 2, 2, 2, WHITE);
    });
  });
}

const scale = (set: Glyphs<string>, size: number) =>
  set.crisp ? Math.max(1, Math.round(size / set.box)) : size / set.box;

function cell(pen: Pen, set: Glyphs<string>, name: string, x: number, y: number) {
  label(pen, name, x + 8, y + 6);
  TONES.forEach(({ ground, ink }, i) => {
    const left = x + 8 + i * (PANEL + 8);
    const stamp = (size: number, cx: number, cy: number) => {
      const k = scale(set, size);
      const side = set.box * k;
      set.draw[name]!(lens(pen, Math.round(cx - side / 2), Math.round(cy - side / 2), k, set.box), ink);
    };
    pen.rect(left, y + 24, PANEL, PANEL, ground);
    pen.rect(left, y + 24 + PANEL + 6, PANEL, 40, ground);
    stamp(BIG, left + PANEL / 2, y + 24 + PANEL / 2);
    SMALL.forEach((size, n) => {
      stamp(size, left + 28 + n * 48, y + 24 + PANEL + 26);
    });
  });
}

export function sheet(set: Glyphs<string>, names: readonly string[], cols: number) {
  const lost = names.filter((name) => typeof set.draw[name] !== "function");
  if (lost.length) throw new Error(`glyphs: the set does not draw ${lost.join(", ")}`);
  const extra = Object.keys(set.draw).filter((name) => !names.includes(name));
  if (extra.length) throw new Error(`glyphs: the set draws ${extra.join(", ")}, which the contract does not name`);
  const pen = raster(cols * CELL[0], Math.ceil(names.length / cols) * CELL[1], PAGE);
  names.forEach((name, i) => {
    cell(pen, set, name, (i % cols) * CELL[0], Math.floor(i / cols) * CELL[1]);
  });
  return png(straight(pen.pixels()));
}

async function main(argv: string[]) {
  const at = argv.indexOf("--cols");
  const cols = at < 0 ? 6 : Number(argv[at + 1]);
  const [design, only] = argv.filter((one, i) => !one.startsWith("--") && argv[i - 1] !== "--cols");
  if (!design || !(cols >= 1) || (only && !Object.hasOwn(KINDS, only))) throw new Error(USAGE);
  for (const kind of (only ? [only] : Object.keys(KINDS)) as Kind[]) {
    const file = join(TOP, "designs", design, kind, "index.ts");
    const held = (await import(file)) as Record<string, Glyphs<string> | undefined>;
    const set = held[kind];
    if (!set) throw new Error(`glyphs: ${relative(TOP, file)} exports no ${kind}`);
    const out = join(DATA_DIR, "glyphs", `${design}-${kind}.png`);
    mkdirSync(dirname(out), { recursive: true });
    writeFileSync(out, sheet(set, KINDS[kind], cols));
    console.log(`glyphs: ${out} box ${set.box}${set.crisp ? " crisp" : ""}, ${cols} per row`);
    KINDS[kind].forEach((name, i) => {
      console.log(`  row ${Math.floor(i / cols) + 1} col ${(i % cols) + 1}  ${name}`);
    });
  }
}

if (import.meta.main) {
  try {
    await main(process.argv.slice(2));
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
