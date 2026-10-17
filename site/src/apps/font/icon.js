import * as font from "@mrlyprod/mrly/font";
import { frame, Grid } from "#view";
import { plate } from "../../lib/plate.js";

export const units = { font };

const TEXT = "Aa";
const PEN = 1;
const WRITTEN = 0.6;
const GHOST = 110;

let facts = null;

function study() {
  if (facts) return facts;
  const raster = font.raster(TEXT);
  const rows = raster.length;
  const cols = raster[0]?.length ?? 0;
  const glyphs = [...TEXT];
  const widths = glyphs.map((c) => font.trim(font.glyph(c).rows)[0].length);
  const from = widths.slice(0, PEN).reduce((n, w) => n + w + 1, 0);
  const path = font.path(glyphs[PEN]);
  const lit = [];
  raster.forEach((row, r) => {
    for (let c = 0; c < cols; c++) if (row[c]) lit.push([r, c]);
  });
  const penned = glyphs.reduce((n, c) => n + font.path(c).length, 0);
  if (penned !== lit.length) throw new Error(`font: the pens of ${TEXT} write ${penned} cells, the raster lights ${lit.length}`);
  for (const [r, c] of path) if (!raster[r]?.[from + c]) throw new Error(`font: the pen of ${glyphs[PEN]} leaves the raster at ${r}, ${c}`);
  const done = lit.filter(([, c]) => c < from - 1);
  if (done.length + path.length !== lit.length) throw new Error(`font: ${done.length} cells before the pen and ${path.length} under it, ${lit.length} lit`);
  facts = { rows, cols, done, path, from };
  return facts;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "purple", 0.14);
  const { rows, cols, done, path, from } = study();
  const cell = Math.floor(Math.min(box.w / cols, box.h / rows));
  const w = cell * cols;
  const h = cell * rows;
  const cells = new Grid(frame(Math.round(box.x + (box.w - w) / 2), Math.round(box.y + (box.h - h) / 2), w, h), cols, rows, 0);
  const ghost = [255, 255, 255, GHOST];
  const k = Math.round(path.length * WRITTEN);
  for (const [r, c] of done) cells.fill(pen, c, r, paint.white);
  path.forEach(([r, c], i) => cells.fill(pen, from + c, r, i < k ? paint.light : ghost));
}
