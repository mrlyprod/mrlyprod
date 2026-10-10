import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 495;
const BASE = 3;
const LEVEL = 2;
const MARGIN = 0.08;
const THICK = 0.16;
const FILL = 0.3;
const STRAND = 0.5;
const WANT = { side: 9, loops: 3, strands: 18 };
const QUARTER = Math.PI / 4;

let cells: Uint8Array | null = null;

function study(): Uint8Array {
  if (cells) return cells;
  const drawn = math.arcs.draw(CODE, BASE, LEVEL);
  const law = math.arcs.law(CODE, BASE, LEVEL);
  if (drawn.side !== WANT.side || drawn.loops !== WANT.loops || drawn.strands !== WANT.strands || law?.loops !== drawn.loops) throw new Error(`app-arcs: side ${drawn.side}, ${drawn.loops} loops, ${drawn.strands} strands, the law gives ${law?.loops}, want ${WANT.side}, ${WANT.loops}, ${WANT.strands}`);
  cells = Uint8Array.from(drawn.cells);
  return cells;
}

export default function draw(pen: Pen, ink: Ink) {
  const bytes = study();
  const side = WANT.side;
  const box = pen.frame(MARGIN);
  const px = box.w / side;
  const r = px / 2;
  const faint = ink.mix(ink.ground, ink.dim, FILL);
  const strand = ink.mix(ink.ground, ink.blue, STRAND);
  for (let i = 0; i < side * side; i++) {
    if (bytes[i] & 1) pen.rect(box.x + (i % side) * px, box.y + Math.floor(i / side) * px, px, px, faint);
  }
  for (const loop of [0, 1]) {
    for (let i = 0; i < side * side; i++) {
      const x = i % side;
      const y = (i - x) / side;
      const filled = bytes[i] & 1;
      for (let which = 0; which < 2; which++) {
        if (((bytes[i] >> (1 + which)) & 1) !== loop) continue;
        const cx = which ? (filled ? x + 1 : x) : filled ? x : x + 1;
        const cy = which ? y + 1 : y;
        const mid = Math.atan2(cy === y ? 1 : -1, cx === x ? 1 : -1);
        pen.arc([box.x + cx * px, box.y + cy * px], r, [mid - QUARTER, mid + QUARTER], THICK * px, loop ? ink.blue : strand);
      }
    }
  }
}
