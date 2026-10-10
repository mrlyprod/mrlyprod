import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = {};

const LOWER = 0.447597813453;
const UPPER = 0.6402121938;
const WALLS = [0.447931, 0.5, 0.605303];

function arm(pen: Pen, ink: Ink, x: number, y: number, w: number, h: number) {
  pen.rect(x, y, w, h, ink.panel);
  const edge: Point[] = [[x, y], [x + w, y], [x + w, y + h], [x, y + h], [x, y]];
  pen.polyline(edge, 2, ink.line);
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const [, cy] = frame.center();
  const thick = frame.h / 3;
  const top = cy - thick / 2;
  const at = (beta: number) => frame.x + frame.w * beta;

  arm(pen, ink, at(0), top, at(LOWER) - at(0), thick);
  arm(pen, ink, at(UPPER), top, at(1) - at(UPPER), thick);
  pen.rect(at(LOWER), top, at(UPPER) - at(LOWER), thick, ink.orange);

  for (const wall of WALLS) {
    const x = at(wall);
    pen.rect(x - 1, frame.y, 2, frame.h, ink.fade(ink.dim, 0.85));
    pen.rect(x - 1, top, 2, thick, ink.line);
  }
}
