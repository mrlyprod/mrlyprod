import type { Ink, Pen, Point } from "mrlyjs/view";
import census from "./census/wiki-rep-tiles.json" with { type: "json" };

const DEPTH = 3;
const GAPS = [16, 8, 3];
const OUTLINE: Point[] = [[0, 0], [3, 0], [2, 1], [1, 1], [0, 2]];

export const units = {};

type Tile = { a: number; b: number; c: number; d: number; x: number; y: number };

function levels(): Tile[][] {
  const rows = census.levels;
  if (rows.length !== DEPTH) throw new Error(`wiki-rep-tiles: ${rows.length} levels, want ${DEPTH}`);
  if (census.mirrored.length !== 4 ** DEPTH) throw new Error(`wiki-rep-tiles: ${census.mirrored.length} mirror flags, want ${4 ** DEPTH}`);
  return rows.map((row, i) => {
    if (row.length !== 4 ** (i + 1)) throw new Error(`wiki-rep-tiles: ${row.length} tiles at depth ${i + 1}, want ${4 ** (i + 1)}`);
    return row.map((tile) => {
      if (tile.length !== 6) throw new Error(`wiki-rep-tiles: tile of ${tile.length} numbers, want 6`);
      const [a, b, c, d, x, y] = tile;
      return { a, b, c, d, x, y };
    });
  });
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const rise = Math.sqrt(3) / 2;
  const px = frame.w / 3;
  const top = frame.y + (frame.h - 2 * rise * px) / 2;
  const outline = (tile: Tile, depth: number): Point[] => {
    const scale = 2 ** depth;
    return OUTLINE.map(([p, q]) => {
      const a = (tile.a * p + tile.b * q + tile.x) / scale;
      const b = (tile.c * p + tile.d * q + tile.y) / scale;
      return [frame.x + (a + b / 2) * px, top + (2 * rise - b * rise) * px];
    });
  };
  const rows = levels();
  rows[DEPTH - 1].forEach((tile, i) => pen.polygon(outline(tile, DEPTH), census.mirrored[i] ? ink.orange : ink.blue));
  for (let depth = DEPTH; depth >= 1; depth--) {
    for (const tile of rows[depth - 1]) {
      const ring = outline(tile, depth);
      ring.push(ring[0]);
      pen.polyline(ring, GAPS[depth - 1], ink.ground);
    }
  }
}
