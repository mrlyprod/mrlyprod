import * as math from "mrlyjs/math";
import { Grid, frame, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const CODES = [7, 14, 3, 5, 9, 6, 15];
const SIDE = 9;
const GUTTER = 130;
const EDGE = 3;

function place(index: number) {
  if (index === 0) return [0, 1];
  if (index < 4) return [1, index - 1];
  return [2, index - 4];
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const tile = (area.w - 2 * GUTTER) / 3;
  const step = tile + GUTTER;
  const corner = (index: number): [number, number] => {
    const [row, col] = place(index);
    return [area.x + col * step, area.y + row * step];
  };
  const foot = (index: number): [number, number] => {
    const [x, y] = corner(index);
    return [x + tile / 2, y + tile];
  };
  const head = (index: number): [number, number] => {
    const [x, y] = corner(index);
    return [x + tile / 2, y];
  };
  for (let child = 1; child < CODES.length; child++) {
    const parent = child < 4 ? 0 : child - 3;
    pen.segment(foot(parent), head(child), EDGE, ink.line);
  }
  let cells = 0;
  CODES.forEach((code, index) => {
    const design = math.two.create(code, 3, 2, 0, 2);
    if (design.shape.length !== 2 || design.shape[0] !== SIDE || design.shape[1] !== SIDE) throw new Error(`site-wiki: design ${code} shape ${design.shape}, want ${SIDE} square`);
    const tone = index === 0 ? ink.yellow : ink.blue;
    const [x, y] = corner(index);
    new Grid(frame(x, y, tile, tile), SIDE, SIDE, 0).paint(pen, design, (kind) => (kind !== 0 ? tone : null));
    for (const kind of design.types) cells += kind;
  });
  if (cells !== 283) throw new Error(`site-wiki: ${cells} cells, want 283`);
}
