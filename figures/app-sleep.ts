import * as math from "mrlyjs/math";
import { frame, Grid, grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const SIDE = 23;
const CELL = 36;
const TILE = 5;
const AT = [5, 17];
const PATH = [[9, 17], [21, 5], [17, 1]];
const STEP = 2;

export default function draw(pen: Pen, ink: Ink) {
  const tile = math.atoms.carpet_2d(TILE);
  if ([...tile.data].join("") !== grid.LOGO.join("")) throw new Error(`app-sleep: the carpet of ${TILE} is not the mark`);
  const [top, left] = AT;
  const [corner, floor, wall] = PATH;
  if (left + TILE !== SIDE - 1 || corner[0] !== top + TILE - 1 || corner[1] !== left) throw new Error("app-sleep: the tile does not lean on the right wall");
  if (floor[0] !== SIDE - 2 || wall[1] !== 1) throw new Error("app-sleep: the path does not bounce off the floor and the left wall");
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  for (let i = 0; i < SIDE; i++) {
    for (const [row, col] of [[0, i], [SIDE - 1, i], [i, 0], [i, SIDE - 1]]) cells.fill(pen, col, row, ink.dim);
  }
  let k = 0;
  for (let leg = 0; leg + 1 < PATH.length; leg++) {
    const [r0, c0] = PATH[leg];
    const [r1, c1] = PATH[leg + 1];
    const n = Math.abs(r1 - r0);
    if (Math.abs(c1 - c0) !== n) throw new Error(`app-sleep: leg ${leg} is not a diagonal`);
    for (let s = leg ? 1 : 0; s <= n; s++, k++) {
      const row = r0 + Math.sign(r1 - r0) * s;
      const col = c0 + Math.sign(c1 - c0) * s;
      if (k % STEP === 0 && k > 0) cells.fill(pen, col, row, ink.blue);
    }
  }
  for (let row = 0; row < TILE; row++) {
    for (let col = 0; col < TILE; col++) if (tile.data[row * TILE + col]) cells.fill(pen, left + col, top + row, ink.fg);
  }
}
