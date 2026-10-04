import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-domino-tilings.json" with { type: "json" };

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const { order, tiles } = census as { order: number; tiles: [number, number, number][] };
  const side = 2 * order;
  for (const [x, y, flat] of tiles) {
    const wide = flat === 1 ? 1 : 0;
    if (x < -order || y < -order || x + wide >= order || y + 1 - wide >= order) throw new Error(`wiki-domino-tilings: tile ${x}, ${y} off the board`);
  }
  const grid = new Grid(pen.frame(0.08), side, side, 0);
  const unit = grid.frame.w / side;
  const pad = unit * 0.1;
  for (const [x, y, flat] of tiles) {
    const [ax, ay, bx, by] = flat === 1 ? [x, y, x + 1, y] : [x, y, x, y + 1];
    const corner = (u: number, v: number) => grid.cell(u + order, order - 1 - v);
    const [x0, y0] = corner(Math.min(ax, bx), Math.max(ay, by));
    const [x1, y1, w1, h1] = corner(Math.max(ax, bx), Math.min(ay, by));
    const w = x1 + w1 - x0;
    const h = y1 + h1 - y0;
    pen.round_rect(x0 + pad, y0 + pad, w - 2 * pad, h - 2 * pad, (unit - 2 * pad) * 0.3, flat === 1 ? ink.blue : ink.yellow);
  }
}
