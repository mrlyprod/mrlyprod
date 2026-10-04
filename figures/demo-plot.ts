import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math, num };

const ROWS = 8;
const FIRST = [20, 81, 208, 425, 756, 1225, 1856, 2673];

let memo: number[][] | undefined;

function rows() {
  if (memo) return memo;
  const first = Array.from({ length: ROWS }, (_, i) => Number(math.counts.fill(23, 2 * i + 3, 3, 1, 2)));
  if (first.some((v, i) => v !== FIRST[i])) throw new Error(`demo-plot: fills ${first}, want ${FIRST}`);
  const out = [first];
  while (out.length < ROWS) out.push(num.blend.delta(out[out.length - 1]).map(Number));
  const cells = out.reduce((sum, row) => sum + row.length, 0);
  if (cells !== 36) throw new Error(`demo-plot: ${cells} cells, want 36`);
  if (out[3].length !== 5 || out[3].some((v) => v !== 24)) throw new Error(`demo-plot: row 3 is ${out[3]}, want five 24s`);
  if (out.slice(4).some((row) => row.some((v) => v !== 0))) throw new Error("demo-plot: a row past 3 is not zero");
  memo = out;
  return out;
}

export default function draw(pen: Pen, ink: Ink) {
  const table = rows();
  const frame = pen.frame(0.08);
  const step = frame.w / ROWS;
  const top = Math.max(...table[0]);
  const scale = Math.log(1 + top);
  table.forEach((row, depth) => {
    const y = frame.y + (depth + 0.5) * step;
    row.forEach((value, place) => {
      const x = frame.x + (place + 0.5 + depth / 2) * step;
      if (value === 0) {
        pen.ring(x, y, step * 0.14, step * 0.03, ink.dim);
        return;
      }
      const weight = Math.log(1 + Math.abs(value)) / scale;
      pen.disc(x, y, step * (0.07 + 0.33 * weight), depth === 3 ? ink.yellow : ink.blue);
    });
  });
}
