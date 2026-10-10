import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SIDE = 32;

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const signs = num.morse.lift("Parity", SIDE);
  if (signs.length !== SIDE * SIDE) throw new Error(`demo-morse: ${signs.length} signs, want ${SIDE * SIDE}`);
  if (num.morse.lift("Xor", SIDE).join() !== signs.join()) throw new Error("demo-morse: the xor lift differs from the parity lift");
  const cool = signs.filter((sign) => sign === 1).length;
  if (cool !== (SIDE * SIDE) / 2) throw new Error(`demo-morse: ${cool} cool signs, want ${(SIDE * SIDE) / 2}`);
  const scale = Math.max(Math.floor(area.w / SIDE), 1);
  const block = SIDE * scale;
  const ox = Math.round((pen.width - block) / 2);
  const oy = Math.round((pen.height - block) / 2);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const tone = signs[row * SIDE + col] === 0 ? ink.orange : ink.blue;
      pen.rect(ox + col * scale, oy + row * scale, scale, scale, tone);
    }
  }
}
