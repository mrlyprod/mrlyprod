import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const LEVEL = 16;
const ROWS = 21;
const CELLS = 860;

// FILLS

type Strip = { cells: number; fills: Float64Array };

let memo: Strip[] | undefined;

function strips() {
  if (memo) return memo;
  const sumset = new num.sumset.Sumset(LEVEL);
  if (sumset.count(3 ** 10) !== 45968n) throw new Error(`demo-sumset: count at 3^10 is ${sumset.count(3 ** 10)}, want 45968`);
  if (sumset.count(14348906) !== 10953840n) throw new Error(`demo-sumset: count at 14348906 is ${sumset.count(14348906)}, want 10953840`);
  const made: Strip[] = [];
  for (let row = 0; row < ROWS; row++) {
    const x = Math.round(Math.pow(3, 6 + row / 2));
    const cells = Math.min(CELLS, x + 1);
    const fills = sumset.fills(0, x + 1, cells);
    if (fills.length !== cells) throw new Error(`demo-sumset: ${fills.length} fills, want ${cells}`);
    made.push({ cells, fills });
  }
  sumset.free();
  memo = made;
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const band = area.h / ROWS;
  const gap = Math.max(Math.round(band * 0.22), 2);
  strips().forEach(({ cells, fills }, row) => {
    const y = Math.round(area.y + row * band);
    const h = Math.round(band - gap);
    const step = area.w / cells;
    fills.forEach((share, i) => {
      if (share === 0) return;
      const left = Math.round(area.x + i * step);
      const right = Math.round(area.x + (i + 1) * step);
      pen.rect(left, y, Math.max(right - left, 1), h, ink.mix(ink.ground, ink.blue, share));
    });
  });
}
