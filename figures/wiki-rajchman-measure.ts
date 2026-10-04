import { frame as box, plot } from "mrlyjs/view";
import type { Frame, Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-rajchman-measure.json" with { type: "json" };

const TOP = 243;

export const units = {};

function comb(pen: Pen, ink: Ink, area: Frame, values: number[], peak: number, lit: number[]) {
  const slot = area.w / values.length;
  const pad = slot * 0.2;
  values.forEach((v, i) => {
    const h = (area.h * v) / peak;
    const color = lit.includes(i + 1) ? ink.yellow : ink.blue;
    pen.rect(area.x + i * slot + pad, area.y + area.h - h, slot - 2 * pad, h, color);
  });
}

export default function draw(pen: Pen, ink: Ink) {
  const { cantor, bump, powers } = census as { cantor: number[]; bump: number[]; powers: number[] };
  if (cantor.length !== TOP || bump.length !== TOP) throw new Error(`wiki-rajchman-measure: ${cantor.length} and ${bump.length} coefficients, want ${TOP}`);
  const frame = pen.frame(0.08);
  const gap = frame.h * 0.08;
  const half = (frame.h - gap) / 2;
  const upper = box(frame.x, frame.y, frame.w, half);
  const lower = box(frame.x, frame.y + half + gap, frame.w, half);
  const ceiling = cantor[0];
  comb(pen, ink, upper, cantor, ceiling, powers);
  comb(pen, ink, lower, bump, Math.max(ceiling, bump[0]), []);
  plot.axis(pen, upper, ink.line);
  plot.axis(pen, lower, ink.line);
}
