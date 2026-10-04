import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const STRIP = 64;
const SIDE = 32;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const word = num.morse.digits(STRIP);
  if (word.length !== STRIP) throw new Error(`wiki-thue-morse-sequence: ${word.length} letters, want ${STRIP}`);
  const ones = word.filter((bit) => bit === 1).length;
  if (ones !== STRIP / 2) throw new Error(`wiki-thue-morse-sequence: ${ones} ones in the strip, want ${STRIP / 2}`);
  const signs = num.morse.lift("Parity", SIDE);
  if (signs.length !== SIDE * SIDE) throw new Error(`wiki-thue-morse-sequence: ${signs.length} signs, want ${SIDE * SIDE}`);
  if (num.morse.lift("Xor", SIDE).join() !== signs.join()) throw new Error("wiki-thue-morse-sequence: the xor lift differs from the parity lift");
  const cool = signs.filter((bit) => bit === 1).length;
  if (cool !== (SIDE * SIDE) / 2) throw new Error(`wiki-thue-morse-sequence: ${cool} minus signs, want ${(SIDE * SIDE) / 2}`);
  const tone = (bit: number) => (bit === 0 ? ink.orange : ink.blue);
  const cell = Math.floor(frame.h / (2 * SIDE + 5));
  const block = 2 * cell;
  const band = 3 * cell;
  const gap = 2 * cell;
  const run = block * SIDE;
  const left = Math.round((pen.width - run) / 2);
  const top = Math.round((pen.height - run - band - gap) / 2);
  word.forEach((bit, place) => pen.rect(left + place * cell, top, cell, band, tone(bit)));
  const head = top + band + gap;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      pen.rect(left + col * block, head + row * block, block, block, tone(signs[row * SIDE + col]));
    }
  }
}
