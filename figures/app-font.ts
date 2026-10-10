import * as font from "mrlyjs/font";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { font };

const TEXT = "Aa";
const PEN = 1;
const WRITTEN = 0.6;
const MARGIN = 0.08;

type Facts = { rows: number; cols: number; done: [number, number][]; path: [number, number][]; from: number };

let facts: Facts | null = null;

function study(): Facts {
  if (facts) return facts;
  const raster = font.raster(TEXT);
  const rows = raster.length;
  const cols = raster[0]?.length ?? 0;
  const glyphs = [...TEXT];
  const widths = glyphs.map((c) => font.trim(font.glyph(c)!.rows)[0].length);
  const from = widths.slice(0, PEN).reduce((n, w) => n + w + 1, 0);
  const path = font.path(glyphs[PEN]);
  const lit: [number, number][] = [];
  raster.forEach((row, r) => {
    for (let c = 0; c < cols; c++) if (row[c]) lit.push([r, c]);
  });
  const penned = glyphs.reduce((n, c) => n + font.path(c).length, 0);
  if (penned !== lit.length) throw new Error(`app-font: the pens of ${TEXT} write ${penned} cells, the raster lights ${lit.length}`);
  for (const [r, c] of path) if (!raster[r]?.[from + c]) throw new Error(`app-font: the pen of ${glyphs[PEN]} leaves the raster at ${r}, ${c}`);
  const done = lit.filter(([, c]) => c < from - 1);
  if (done.length + path.length !== lit.length) throw new Error(`app-font: ${done.length} cells before the pen and ${path.length} under it, ${lit.length} lit`);
  facts = { rows, cols, done, path, from };
  return facts;
}

export default function draw(pen: Pen, ink: Ink) {
  const { rows, cols, done, path, from } = study();
  const box = pen.frame(MARGIN);
  const cell = Math.floor(Math.min(box.w / cols, box.h / rows));
  const w = cell * cols;
  const h = cell * rows;
  const cells = new Grid(frame(Math.round(box.x + (box.w - w) / 2), Math.round(box.y + (box.h - h) / 2), w, h), cols, rows, 0);
  const faint = ink.mix(ink.ground, ink.blue, 0.3);
  const k = Math.round(path.length * WRITTEN);
  for (const [r, c] of done) cells.fill(pen, c, r, ink.fg);
  path.forEach(([r, c], i) => cells.fill(pen, from + c, r, i < k ? ink.blue : faint));
}
