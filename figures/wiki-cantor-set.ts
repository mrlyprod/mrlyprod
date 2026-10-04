import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const STAGES = 6;
const KEPT = 63;
const CUT = 31;
const CODE = 5;
const BASE = 3;

function stage(level: number) {
  if (level === 0) return { den: 1, at: [0] };
  const tensor = math.bang.factory.create(CODE, BASE, 1, BASE, level);
  const den = BASE ** level;
  if (tensor.shape.length !== 1 || tensor.shape[0] !== den) throw new Error(`wiki-cantor-set: level ${level} shape ${tensor.shape}, want ${den}`);
  const at: number[] = [];
  for (let i = 0; i < den; i++) if (tensor.data[i] !== 0) at.push(i);
  return { den, at };
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const bar = frame.h / (STAGES + (STAGES - 1) * 0.9);
  const pitch = bar * 1.9;
  const rows = Array.from({ length: STAGES }, (_, level) => stage(level));
  let kept = 0;
  let cut = 0;
  rows.forEach(({ den, at }, level) => {
    const y = frame.y + level * pitch;
    const side = 1 / den;
    if (at.length !== 2 ** level) throw new Error(`wiki-cantor-set: level ${level} keeps ${at.length} intervals, want ${2 ** level}`);
    if (Math.abs(at.length * side - (2 / 3) ** level) >= 1e-12) throw new Error(`wiki-cantor-set: level ${level} length ${at.length * side}, want ${(2 / 3) ** level}`);
    if (level > 0) {
      const before = rows[level - 1];
      for (const n of before.at) {
        pen.rect(frame.x + frame.w * (n / before.den + side), y, frame.w * side, bar, ink.dim);
        cut++;
      }
    }
    for (const n of at) {
      pen.rect(frame.x + frame.w * (n / den), y, frame.w * side, bar, ink.blue);
      kept++;
    }
  });
  if (kept !== KEPT) throw new Error(`wiki-cantor-set: ${kept} kept intervals, want ${KEPT}`);
  if (cut !== CUT) throw new Error(`wiki-cantor-set: ${cut} cut intervals, want ${CUT}`);
}
