import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Color, Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const ROWS = 9;
const WIDEST = 6.5;
const COUNTS = [1, 1, 3, 3, 6, 3, 3, 1, 1];

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[][][] | undefined;

function rows() {
  if (memo) return memo;
  const codes: string[][] = Array.from({ length: ROWS }, () => []);
  for (const design of math.bang.bang(3).canonical()) codes[design.rule().length].push(design.i);
  if (codes.some((row, r) => row.length !== COUNTS[r])) throw new Error(`research-method: rows of ${codes.map((row) => row.length)}, want ${COUNTS}`);
  memo = codes.map((row, r) =>
    row.map((code) => {
      const cube = math.three.create(code, 2, 1, 2);
      if (math.three.fills(cube) !== r) throw new Error(`research-method: design ${code} fills ${math.three.fills(cube)}, want ${r}`);
      return math.three.quads(cube);
    }),
  );
  return memo;
}

function cage(pen: Pen, cx: number, cy: number, s: number, color: Color) {
  const corner = (i: number): Point => {
    const p = iso.project((i & 1) * 2 - 1, ((i >> 1) & 1) * 2 - 1, ((i >> 2) & 1) * 2 - 1);
    return [cx + p[0] * s, cy + p[1] * s];
  };
  for (let a = 0; a < 8; a++) {
    for (const bit of [1, 2, 4]) if (!(a & bit)) pen.segment(corner(a), corner(a | bit), s / 20, color);
  }
}

function stamp(pen: Pen, quads: Quad[], cx: number, cy: number, s: number, shade: Color[], edge: Color) {
  const faces = [];
  for (const { normal, verts } of quads) {
    if (normal.x + normal.y + normal.z <= 0) continue;
    const tone = normal.z > 0 ? 0 : normal.y > 0 ? 1 : 2;
    let depth = 0;
    for (const v of verts) depth += Math.fround(Math.fround(v.x + v.y) + v.z);
    const pts = verts.map((v): Point => {
      const p = iso.project(v.x, v.y, v.z);
      return [cx + p[0] * s, cy + p[1] * s];
    });
    faces.push({ depth, tone, pts });
  }
  faces.sort((a, b) => a.depth - b.depth);
  for (const { tone, pts } of faces) {
    pen.polygon(pts, shade[tone]);
    pen.polyline([...pts, pts[0]], s / 14, edge);
  }
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const shade = [ink.blue, ink.mix(ink.blue, ink.ground, 0.4), ink.mix(ink.blue, ink.ground, 0.65)];
  const wire = ink.mix(ink.line, ink.dim, 0.3);
  const [mx, my] = frame.center();
  const pitch = frame.h / ROWS;
  const step = frame.w / WIDEST;
  const s = pitch * 0.225;
  rows().forEach((row, r) => {
    const cy = my + (r - (ROWS - 1) / 2) * pitch;
    row.forEach((quads, j) => {
      const cx = mx + (j - (row.length - 1) / 2) * step;
      cage(pen, cx, cy, s, wire);
      stamp(pen, quads, cx, cy, s, shade, ink.ground);
    });
  });
}
