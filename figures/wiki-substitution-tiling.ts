import type { Ink, Pen, Point } from "mrlyjs/view";
import census from "./census/wiki-substitution-tiling.json" with { type: "json" };

const DEPTH = 3;
const GAPS = [22, 11, 4];

export const units = {};

type Chair = { x: number; y: number; side: number; turn: number };

function spin(p: Point, centre: Point, turn: number): Point {
  let dx = p[0] - centre[0];
  let dy = p[1] - centre[1];
  for (let i = 0; i < turn % 4; i++) [dx, dy] = [-dy, dx];
  return [centre[0] + dx, centre[1] + dy];
}

function outline(c: Chair): Point[] {
  const s = c.side;
  const h = c.side / 2;
  const centre: Point = [c.x + h, c.y + h];
  const corners: Point[] = [[0, 0], [s, 0], [s, h], [h, h], [h, s], [0, s]];
  return corners.map(([u, v]) => spin([c.x + u, c.y + v], centre, c.turn));
}

function levels(): Chair[][] {
  const rows = census.levels;
  if (rows.length !== DEPTH) throw new Error(`wiki-substitution-tiling: ${rows.length} levels, want ${DEPTH}`);
  return rows.map((row, i) => {
    if (row.length !== 4 ** (i + 1)) throw new Error(`wiki-substitution-tiling: ${row.length} chairs at depth ${i + 1}, want ${4 ** (i + 1)}`);
    return row.map((chair) => {
      if (chair.length !== 3) throw new Error(`wiki-substitution-tiling: chair of ${chair.length} numbers, want 3`);
      return { x: chair[0], y: chair[1], side: 1 / 2 ** (i + 1), turn: chair[2] };
    });
  });
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const rows = levels();
  const map = (p: Point): Point => [frame.x + p[0] * frame.w, frame.y + (1 - p[1]) * frame.h];
  for (const c of rows[DEPTH - 1]) pen.polygon(outline(c).map(map), c.turn % 2 === 0 ? ink.blue : ink.yellow);
  for (let depth = DEPTH; depth >= 1; depth--) {
    const gap = GAPS[depth - 1];
    for (const c of rows[depth - 1]) {
      const pts = outline(c).map(map);
      pts.forEach((a, i) => {
        const b = pts[(i + 1) % pts.length];
        pen.rect(Math.min(a[0], b[0]) - gap / 2, Math.min(a[1], b[1]) - gap / 2, Math.abs(a[0] - b[0]) + gap, Math.abs(a[1] - b[1]) + gap, ink.ground);
      });
    }
  }
}
