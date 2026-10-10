import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

const COUNT = 8;
const SIZES = [3, 6, 22];
const ROWS = 28;

export const units = { math, num };

// SEQUENCES

type Row = { dim: number; terms: number[] };

let memo: Row[] | undefined;

function sequences() {
  if (memo) return memo;
  const out: Row[] = [];
  SIZES.forEach((size, i) => {
    const dim = i + 1;
    const codes = math.bang.universe_codes(dim);
    if (codes.length !== size) throw new Error(`demo-sequences: ${codes.length} designs in dimension ${dim}, want ${size}`);
    for (const code of codes) {
      const values = Array.from({ length: COUNT }, (_, k) => math.counts.fill(code, 2 * k + 3, dim, 1, 2));
      let fold = values;
      for (let step = 0; step < dim; step++) fold = num.blend.delta(fold);
      if (fold.some((value) => BigInt(value) !== BigInt(fold[0]))) throw new Error(`demo-sequences: code ${code} folds to ${fold}, want one value`);
      if (num.blend.delta(fold).some((value) => BigInt(value) !== 0n)) throw new Error(`demo-sequences: code ${code} folds to ${fold}, want a constant`);
      if (BigInt(values[0]) > 0n) out.push({ dim, terms: values.map(Number) });
    }
  });
  if (out.length !== ROWS) throw new Error(`demo-sequences: ${out.length} rows, want ${ROWS}`);
  memo = out;
  return out;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const rows = sequences();
  const frame = pen.frame(0.08);
  const sides = Array.from({ length: COUNT }, (_, i) => Math.log(2 * i + 3));
  const logs = rows.flatMap((row) => row.terms.map((value) => Math.log(value)));
  const low = logs.reduce((a, b) => Math.min(a, b), Number.MAX_VALUE);
  const high = logs.reduce((a, b) => Math.max(a, b), -Number.MAX_VALUE);
  const at = (index: number, value: number): Point => [
    frame.x + (frame.w * (sides[index] - sides[0])) / (sides[COUNT - 1] - sides[0]),
    frame.y + frame.h * (1 - (Math.log(value) - low) / (high - low)),
  ];
  for (let dimension = 1; dimension <= 3; dimension++) {
    const color = dimension === 1 ? ink.fade(ink.dim, 0.85) : dimension === 2 ? ink.yellow : ink.blue;
    for (const row of rows.filter((r) => r.dim === dimension)) {
      const path = row.terms.map((value, index) => at(index, value));
      pen.polyline(path, 2.5, color);
      for (const [x, y] of path) pen.disc(x, y, 5.5, color);
    }
  }
}
