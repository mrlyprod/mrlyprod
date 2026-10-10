import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const WIDTH = 3;
const CODE = 23;
const LEVELS = 9;

export default function draw(pen: Pen, ink: Ink) {
  const rule = new num.memory.Rule(1, WIDTH, CODE);
  const tally = num.memory.counts(rule, LEVELS);
  if (tally.length !== LEVELS || tally.slice(0, 8).join() !== "2,4,4,6,9,13,19,28") throw new Error(`demo-memory: counts ${tally.slice(0, 8)}, want 2,4,4,6,9,13,19,28`);
  const area = pen.frame(0.08);
  const band = area.h / LEVELS;
  const gap = Math.round(Math.max(band * 0.16, 2));
  for (let depth = 1; depth <= LEVELS; depth++) {
    const word = num.memory.cells(rule, depth);
    if (BigInt(word.length) !== tally[depth - 1]) throw new Error(`demo-memory: level ${depth} has ${word.length} cells, want ${tally[depth - 1]}`);
    const span = area.w / 2 ** depth;
    const y = Math.round(area.y + (depth - 1) * band);
    const h = Math.max(Math.round(band - gap), 1);
    const paint = depth === LEVELS ? ink.orange : ink.yellow;
    for (const seat of word) {
      const x = Math.round(area.x + Number(seat) * span);
      const w = Math.round(area.x + (Number(seat) + 1) * span) - x;
      pen.rect(x, y, Math.max(w, 1), h, paint);
    }
  }
}
