export type Shape = {
  dim: number;
  side: number;
  cells: number[][];
  neighbors: number[][];
};

export const CARPET = 7;
export const SPONGE = 23;
const NUMBER = 3;
const BASE = 2;

function digits(flat: number, dim: number, side: number): number[] {
  const out = new Array<number>(dim);
  for (let axis = dim - 1; axis >= 0; axis--) {
    out[axis] = flat % side;
    flat = Math.floor(flat / side);
  }
  return out;
}

export function tile(code: number, dim: number): Uint8Array {
  const out = new Uint8Array(NUMBER ** dim);
  for (let flat = 0; flat < out.length; flat++) {
    const residue = digits(flat, dim, NUMBER).map((d) => d % BASE);
    const bit = residue.reduce((acc, r) => acc * BASE + r, 0);
    out[flat] = (code >> bit) & 1;
  }
  return out;
}

export function shape(code: number, dim: number, level: number): Shape {
  const seed = tile(code, dim);
  const side = NUMBER ** level;
  const total = side ** dim;
  const filled = new Int32Array(total).fill(-1);
  const cells: number[][] = [];
  for (let flat = 0; flat < total; flat++) {
    const coord = digits(flat, dim, side);
    let on = true;
    for (let step = 1; step <= level && on; step++) {
      const unit = NUMBER ** (level - step);
      const sub = coord.reduce((acc, c) => acc * NUMBER + (Math.floor(c / unit) % NUMBER), 0);
      on = seed[sub] === 1;
    }
    if (!on) continue;
    filled[flat] = cells.length;
    cells.push(coord);
  }
  const neighbors = cells.map((coord) => {
    const out: number[] = [];
    for (let axis = 0; axis < dim; axis++) {
      for (const step of [-1, 1]) {
        const c = coord[axis] + step;
        if (c < 0 || c >= side) continue;
        const other = coord.map((v, i) => (i === axis ? c : v));
        const flat = other.reduce((acc, v) => acc * side + v, 0);
        if (filled[flat] >= 0) out.push(filled[flat]);
      }
    }
    return out;
  });
  return { dim, side, cells, neighbors };
}
