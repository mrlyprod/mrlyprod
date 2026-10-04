import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-spectral-radius.json" with { type: "json" };

const ARROWS = 8;

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const { angles, settled } = census;
  if (angles.length !== ARROWS) throw new Error(`wiki-spectral-radius: ${angles.length} angles, want ${ARROWS}`);
  const frame = pen.frame(0.08);
  const [cx, cy] = frame.at(0.3, 0.59);
  const reach = frame.w * 0.62;
  const far = frame.w * 0.7;
  const guide = ink.fade(ink.dim, 0.8);
  pen.segment([cx - far * Math.cos(settled), cy + far * Math.sin(settled)], [cx + far * Math.cos(settled), cy - far * Math.sin(settled)], 3, guide);
  pen.arc([cx, cy], reach * 1.05, [-settled - 0.12, -angles[0] + 0.12], 3, guide);
  angles.forEach((angle, index) => {
    const paint = ink.mix(ink.blue, ink.yellow, index / (ARROWS - 1));
    const tip: [number, number] = [cx + reach * Math.cos(angle), cy - reach * Math.sin(angle)];
    const way = [Math.cos(angle), -Math.sin(angle)];
    const side = [-way[1], way[0]];
    const head = frame.w * 0.045;
    const base: [number, number] = [tip[0] - way[0] * head, tip[1] - way[1] * head];
    pen.segment([cx, cy], base, 7, paint);
    pen.triangle(tip, [base[0] + side[0] * head * 0.5, base[1] + side[1] * head * 0.5], [base[0] - side[0] * head * 0.5, base[1] - side[1] * head * 0.5], paint);
  });
  pen.disc(cx, cy, frame.w * 0.018, ink.fg);
}
