import * as math from "mrlyjs/math";
import { Grid, type Color, type Ink, type Pen } from "mrlyjs/view";

const RADIUS = 242;
const LEVEL = 5;
const SIDE = 243;

export const units = { math };

type Shell = { design: Uint8Array; tree: ReturnType<typeof math.shape.crossing_tree> };

let memo: Shell | undefined;

function bytes(tensor: ReturnType<typeof math.bang.factory.create>) {
  if (!(tensor.data instanceof Uint8Array)) throw new Error(`demo-shell: tensor of ${tensor.data.constructor.name}, want Uint8Array`);
  return tensor.data;
}

function shell(): Shell {
  if (memo) return memo;
  const tile = math.bang.factory.create(7, 3, 2, 2, 1);
  const keep = Array.from(bytes(tile), (byte) => byte !== 0);
  const tree = math.shape.crossing_tree(RADIUS, 3, keep);
  if (tree.orphans !== 0) throw new Error(`demo-shell: ${tree.orphans} orphans, want 0`);
  if (tree.levels.length !== LEVEL + 1) throw new Error(`demo-shell: ${tree.levels.length} levels, want ${LEVEL + 1}`);
  tree.levels.forEach((boxes, level) => {
    const want = 2 * Math.floor(RADIUS / 3 ** level) + 1;
    if (boxes.length !== want) throw new Error(`demo-shell: level ${level} holds ${boxes.length} boxes, want ${want}`);
  });
  if (tree.levels[0].length !== 485) throw new Error(`demo-shell: ${tree.levels[0].length} crossed cells, want 485`);

  const whole = math.bang.factory.create(7, 3, 2, 2, LEVEL);
  if (whole.shape[0] !== SIDE || whole.shape[1] !== SIDE || whole.shape.length !== 2) throw new Error(`demo-shell: design ${whole.shape}, want ${SIDE} square`);
  const design = bytes(whole);
  let sum = 0;
  for (const byte of design) sum += byte;
  if (sum !== 32768) throw new Error(`demo-shell: design sums to ${sum}, want 32768`);

  memo = { design, tree };
  return memo;
}

function outline(pen: Pen, grid: Grid, x: number, y: number, step: number, thick: number, color: Color) {
  if ((x + 1) * step > SIDE) throw new Error(`demo-shell: box ${x} at step ${step} leaves the ${SIDE} side`);
  const [left, top] = grid.cell(y * step, SIDE - (x + 1) * step);
  const [right, foot, w, h] = grid.cell((y + 1) * step - 1, SIDE - 1 - x * step);
  const wide = right + w - left;
  const tall = foot + h - top;
  pen.rect(left, top, wide, thick, color);
  pen.rect(left, top + tall - thick, wide, thick, color);
  pen.rect(left, top, thick, tall, color);
  pen.rect(left + wide - thick, top, thick, tall, color);
}

export default function draw(pen: Pen, ink: Ink) {
  const { design, tree } = shell();
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0);
  for (let x = 0; x < SIDE; x++) {
    for (let y = 0; y < SIDE; y++) {
      if (design[x * SIDE + y] !== 0) grid.fill(pen, y, SIDE - 1 - x, ink.line);
    }
  }
  const unit = grid.cell(0, 0)[2];
  for (const [level, thick] of [[2, 0.5 * unit], [3, unit], [4, 2 * unit]]) {
    const step = 3 ** level;
    for (const cell of tree.levels[level]) outline(pen, grid, cell.x, cell.y, step, thick, ink.dim);
  }
  let kept = 0;
  for (const cell of tree.levels[0]) {
    if (cell.live) kept++;
    grid.fill(pen, cell.y, SIDE - 1 - cell.x, cell.live ? ink.yellow : ink.blue);
  }
  if (kept !== 296) throw new Error(`demo-shell: ${kept} live cells, want 296`);
}
