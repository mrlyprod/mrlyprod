import { grid, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};
export const size = [512, 512];

export default function draw(pen: Pen, ink: Ink) {
  new Grid(pen.frame(0.12), 5, 5, 0).carpet(pen, grid.mask(grid.LOGO, 1), ink.fg);
}
