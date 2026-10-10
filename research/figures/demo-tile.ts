import * as math from "mrlyjs/math";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 495;
const NUMBER = 3;
const BASE = 3;
const LEVEL = 2;
const COPIES = 5;
const TILE = 9;
const SIDE = 45;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const tile = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  if (tile.shape.join() !== `${TILE},${TILE}` || math.two.fills(tile) !== 64) throw new Error(`demo-tile: tile ${tile.shape} with ${math.two.fills(tile)} fills, want 9,9 with 64`);
  if (math.two.perimeter(tile) !== "80") throw new Error(`demo-tile: tile perimeter ${math.two.perimeter(tile)}, want 80`);
  const sheet = math.cell.models.tile(tile, COPIES, COPIES);
  if (sheet.shape.join() !== `${SIDE},${SIDE}` || math.two.fills(sheet) !== 1600) throw new Error(`demo-tile: sheet ${sheet.shape} with ${math.two.fills(sheet)} fills, want 45,45 with 1600`);
  if (math.two.perimeter(sheet) !== "1280") throw new Error(`demo-tile: sheet perimeter ${math.two.perimeter(sheet)}, want 1280`);
  const buried = 25 * 80 - Number(math.two.perimeter(sheet));
  if (buried !== 720) throw new Error(`demo-tile: ${buried} buried, want 720`);
  const types = sheet.types;
  const on = (row: number, col: number) => types[row * SIDE + col] !== 0;
  const grid = new Grid(frame, SIDE, SIDE, 0);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      if (on(row, col)) grid.fill(pen, col, row, ink.blue);
    }
  }
  const thick = frame.cell(SIDE) * 0.26;
  let seams = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      if (!on(row, col)) continue;
      const [x, y, w, h] = grid.cell(col, row);
      if (col % TILE === TILE - 1 && col + 1 < SIDE && on(row, col + 1)) {
        pen.segment([x + w, y], [x + w, y + h], thick, ink.orange);
        seams++;
      }
      if (row % TILE === TILE - 1 && row + 1 < SIDE && on(row + 1, col)) {
        pen.segment([x, y + h], [x + w, y + h], thick, ink.orange);
        seams++;
      }
    }
  }
  if (seams * 2 !== buried) throw new Error(`demo-tile: ${seams} seams, want ${buried / 2}`);
  if (seams !== 360) throw new Error(`demo-tile: ${seams} seams, want 360`);
}
