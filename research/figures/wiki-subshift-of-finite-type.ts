import * as num from "mrlyjs/num";
import type { Color, Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const DEPTH = 8;
const COUNTS = [2, 3, 5, 8, 13, 21, 34, 55];
const TAU = Math.PI * 2;

let memo: boolean[][] | undefined;

function allowed() {
  if (memo) return memo;
  const rule = new num.memory.Rule(1, 2, 7);
  const tally = num.memory.counts(rule, DEPTH);
  if (tally.length !== DEPTH || tally.some((n, i) => n !== BigInt(COUNTS[i]))) throw new Error(`wiki-subshift-of-finite-type: counts ${tally}, want ${COUNTS}`);
  const table: boolean[][] = [];
  for (let length = 1; length <= DEPTH; length++) {
    const row: boolean[] = [];
    for (let word = 0; word < 1 << length; word++) {
      const digits = Array.from({ length }, (_, i) => (word >> (length - 1 - i)) & 1);
      row.push(rule.accepts(digits));
    }
    if (row.filter(Boolean).length !== COUNTS[length - 1]) throw new Error(`wiki-subshift-of-finite-type: ${row.filter(Boolean).length} words accepted at length ${length}, want ${COUNTS[length - 1]}`);
    table.push(row);
  }
  rule.free();
  memo = table;
  return memo;
}

function sector(pen: Pen, center: Point, radii: [number, number], turns: [number, number], gap: number, color: Color) {
  const [inner, outer] = radii;
  const at = (r: number, turn: number): Point => {
    const angle = TAU * turn - Math.PI / 2;
    return [center[0] + r * Math.cos(angle), center[1] + r * Math.sin(angle)];
  };
  const side = (r: number): [number, number] => {
    const trim = gap / (2 * r * TAU);
    return [turns[0] + trim, turns[1] - trim];
  };
  const steps = (r: number, span: [number, number]) => Math.max(Math.ceil(((span[1] - span[0]) * TAU * r) / 3), 1);
  const far = side(outer);
  const near = side(inner);
  const pts: Point[] = [];
  let n = steps(outer, far);
  for (let i = 0; i <= n; i++) pts.push(at(outer, far[0] + ((far[1] - far[0]) * i) / n));
  n = steps(inner, near);
  for (let i = n; i >= 0; i--) pts.push(at(inner, near[0] + ((near[1] - near[0]) * i) / n));
  pen.polygon(pts, color);
}

export default function draw(pen: Pen, ink: Ink) {
  const ok = allowed();
  const frame = pen.frame(0.08);
  const center = frame.center();
  const reach = Math.min(frame.w, frame.h) / 2;
  const hole = reach * 0.16;
  const step = (reach - hole) / DEPTH;
  const rim = 4;
  const gap = 4;
  pen.disc(center[0], center[1], hole * 0.42, ink.dim);
  for (let length = 1; length <= DEPTH; length++) {
    const inner = hole + step * (length - 1) + rim / 2;
    const outer = inner + step - rim;
    const slot = 1 / 2 ** length;
    for (let word = 0; word < 1 << length; word++) {
      const parent = word >> 1;
      if (length > 1 && !ok[length - 2][parent]) continue;
      const color = ok[length - 1][word] ? (word & 1 ? ink.yellow : ink.blue) : ink.fade(ink.line, 0.7);
      sector(pen, center, [inner, outer], [word * slot, (word + 1) * slot], gap, color);
    }
  }
}
