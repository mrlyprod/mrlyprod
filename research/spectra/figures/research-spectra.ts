import * as math from "@mrlyprod/math";
import * as six from "@mrlyprod/six";
import * as three from "@mrlyprod/three";
import type { Ink, Pen } from "@mrlyprod/draw";
import { hex } from "@mrlyprod/draw";

export const units = { math, six, three };

const BASE = 5;
const LEVEL = 2;
const SIDE = 25;
const MIDDLE = (3 * (SIDE - 1)) / 2;

type Slice = ReturnType<typeof six.cut>;

let memo: { all: Slice; middle: Slice } | undefined;

function slices() {
  if (memo) return memo;
  const carpet = three.carpet(BASE, LEVEL);
  const types = new Uint8Array(carpet.types.length);
  for (const [x, y, z] of three.diagonal_slice(math.factory.levels_code(3, 2, [0, 1]), BASE, LEVEL, 2, MIDDLE)) {
    const i = (x * SIDE + y) * SIDE + z;
    if (carpet.types[i] !== 1)
      throw new Error(`research-spectra: the middle plane cell ${x},${y},${z} is not in the carpet`);
    types[i] = 1;
  }
  memo = { all: six.cut(carpet), middle: six.cut({ shape: carpet.shape, types }) };
  return memo;
}

function at(slice: Slice, row: number, col: number) {
  const width = slice.cell.shape[1];
  return slice.cell.types[row * width + col + (width - hex.row_len(SIDE, row)) / 2];
}

export default function draw(pen: Pen, ink: Ink) {
  const { all, middle } = slices();
  const FILL = six.FILL();
  if (six.height(all) !== 2 * SIDE)
    throw new Error(`research-spectra: the cut is ${six.height(all)} rows, want ${2 * SIDE}`);
  let drawn = 0;
  for (let row = 0; row < 2 * SIDE; row++) {
    for (let col = 0; col < hex.row_len(SIDE, row); col++) {
      if (at(all, row, col) === FILL) drawn++;
      else if (at(middle, row, col) === FILL)
        throw new Error(`research-spectra: a middle triangle at ${row},${col} is not a fill`);
    }
  }
  if (drawn !== six.fills(all))
    throw new Error(`research-spectra: the hexagon holds ${drawn} of ${six.fills(all)} fills`);
  const frame = pen.frame(0.08);
  hex.hexagon(pen, frame, SIDE, 0, (row, col) => (at(middle, row, col) === FILL ? ink.yellow : null));
  hex.hexagon(pen, frame, SIDE, 1.2, (row, col) =>
    at(all, row, col) === FILL && at(middle, row, col) !== FILL ? ink.blue : null,
  );
}
