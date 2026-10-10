import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-space-filling-curve.json" with { type: "json" };

const SIDE = 32;
const CELLS = 1024;
const STEPS = CELLS - 1;
const MARGIN = 0.08;
const WEIGHT = 0.42;

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const { pts } = census as { pts: [number, number][] };
  if (pts.length !== CELLS) throw new Error(`wiki-space-filling-curve: ${pts.length} points, want ${CELLS}`);
  for (const [x, y] of pts) {
    if (!Number.isInteger(x) || !Number.isInteger(y) || x < 0 || x >= SIDE || y < 0 || y >= SIDE) throw new Error(`wiki-space-filling-curve: point ${x}, ${y} off the lattice`);
  }
  const area = pen.frame(MARGIN);
  const cell = area.w / SIDE;
  const at = ([x, y]: [number, number]): [number, number] => [area.x + (x + 0.5) * cell, area.y + (SIDE - 1 - y + 0.5) * cell];
  const ramp = ink.Ramp.tone(ink.blue, ink.yellow);
  for (let i = 0; i < STEPS; i++) {
    const t = i / (STEPS - 1);
    pen.segment(at(pts[i]), at(pts[i + 1]), WEIGHT * cell, ramp.at(t));
  }
}
