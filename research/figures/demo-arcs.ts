import * as math from "mrlyjs/math";
import { Grid, type Ink, type Pen, type Point } from "mrlyjs/view";

const LEVEL = 5;
const SIDE = 32;

export const units = { math };

type Sweep = [Point, [number, number]];

let memo: ReturnType<typeof math.arcs.draw> | undefined;

function arcs() {
  if (memo) return memo;
  const drawn = math.arcs.draw(7, 2, LEVEL);
  if (drawn.side !== SIDE) throw new Error(`demo-arcs: side ${drawn.side}, want ${SIDE}`);
  if (drawn.loops !== 3 ** (LEVEL - 1) - 2 ** LEVEL + 1) throw new Error(`demo-arcs: ${drawn.loops} loops, want ${3 ** (LEVEL - 1) - 2 ** LEVEL + 1}`);
  if (drawn.loops !== 50) throw new Error(`demo-arcs: ${drawn.loops} loops, want 50`);
  if (drawn.strands !== 2 * SIDE) throw new Error(`demo-arcs: ${drawn.strands} strands, want ${2 * SIDE}`);
  if (drawn.cells.length !== SIDE * SIDE) throw new Error(`demo-arcs: ${drawn.cells.length} cells, want ${SIDE * SIDE}`);
  const filled = drawn.cells.filter((byte) => (byte & 1) === 1).length;
  if (filled !== 243) throw new Error(`demo-arcs: ${filled} filled cells, want 243`);
  memo = drawn;
  return memo;
}

function sweeps(filled: boolean, left: number, top: number, cell: number): [Sweep, Sweep] {
  if (filled) {
    return [
      [[left, top + cell], [-Math.PI / 2, 0]],
      [[left + cell, top], [Math.PI / 2, Math.PI]],
    ];
  }
  return [
    [[left + cell, top + cell], [Math.PI, Math.PI + Math.PI / 2]],
    [[left, top], [0, Math.PI / 2]],
  ];
}

export default function draw(pen: Pen, ink: Ink) {
  const { cells } = arcs();
  const frame = pen.frame(0.08);
  const cell = frame.w / SIDE;
  const grid = new Grid(frame, SIDE, SIDE, 0);
  for (let y = 0; y < SIDE; y++) {
    for (let x = 0; x < SIDE; x++) {
      if ((cells[y * SIDE + x] & 1) === 1) grid.fill(pen, x, SIDE - 1 - y, ink.line);
    }
  }
  for (let y = 0; y < SIDE; y++) {
    for (let x = 0; x < SIDE; x++) {
      const byte = cells[y * SIDE + x];
      const left = frame.x + x * cell;
      const top = frame.y + (SIDE - 1 - y) * cell;
      sweeps((byte & 1) === 1, left, top, cell).forEach(([centre, angles], i) => {
        const color = (byte & [2, 4][i]) === 0 ? ink.blue : ink.orange;
        pen.arc(centre, cell / 2, angles, cell * 0.18, color);
      });
    }
  }
}
