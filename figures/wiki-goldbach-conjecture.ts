import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const TOP = 400;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const record = num.prime.goldbach_record(TOP);
  const peak = Math.max(...record);
  const slot = frame.w / record.length;
  const pad = slot * 0.16;
  let sixes = 0;
  record.forEach((pairs, i) => {
    const six = (2 * (i + 2)) % 6 === 0;
    const h = (frame.h * pairs) / peak;
    if (six) sixes++;
    pen.rect(frame.x + i * slot + pad, frame.y + frame.h - h, slot - 2 * pad, h, six ? ink.yellow : ink.blue);
  });
  plot.baseline(pen, frame, ink.line);
  if (record.length !== 199) throw new Error(`wiki-goldbach-conjecture: ${record.length} evens, want 199`);
  if (Math.min(...record) !== 1) throw new Error(`wiki-goldbach-conjecture: least count ${Math.min(...record)}, want 1`);
  if (peak !== 27) throw new Error(`wiki-goldbach-conjecture: peak ${peak}, want 27`);
  if (sixes !== 66) throw new Error(`wiki-goldbach-conjecture: ${sixes} multiples of six, want 66`);
}
