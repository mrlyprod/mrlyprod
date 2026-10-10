import * as math from "mrlyjs/math";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const BASES = [3, 5];
const DIMS = [2, 3, 4, 5, 6, 7, 8, 9];

let memo: number[][][][] | undefined;

function even(base: number, dimension: number) {
  return math.counts.ladder.even_block(base, dimension).map((line) => line.map(Number));
}

function blocks() {
  if (memo) return memo;
  const ladder = math.counts.ladder;
  if (ladder.cap(3) !== 15) throw new Error(`demo-carry: cap(3) is ${ladder.cap(3)}, want 15`);
  if (ladder.cap(5) !== 11) throw new Error(`demo-carry: cap(5) is ${ladder.cap(5)}, want 11`);
  const small = even(3, 3);
  if (small.join("/") !== "6,6/1,3") throw new Error(`demo-carry: even_block(3, 3) is ${small.join("/")}, want 6,6/1,3`);
  memo = BASES.map((base) =>
    DIMS.map((dimension) => {
      const block = even(base, dimension);
      if (block.length !== Math.ceil(dimension / 2)) throw new Error(`demo-carry: block of base ${base} dimension ${dimension} is ${block.length} wide, want ${Math.ceil(dimension / 2)}`);
      return block;
    }),
  );
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const panels = new Grid(pen.frame(0.08), 4, 4, 0.05);
  let drawn = 0;
  blocks().forEach((row, turn) => {
    const ramp = ink.Ramp.tone(ink.line, BASES[turn] === 3 ? ink.yellow : ink.blue);
    row.forEach((block, step) => {
      const slot = turn * DIMS.length + step;
      const [x, y, w, h] = panels.cell(slot % 4, Math.floor(slot / 4));
      const width = block.length;
      const cells = new Grid(frame(x, y, w, h), width, width, 0.06);
      const peak = block.flat().reduce((top, value) => Math.max(top, value), 0);
      block.forEach((line, r) => {
        line.forEach((value, c) => {
          cells.fill(pen, c, r, ramp.at(Math.log(1 + value) / Math.log(1 + peak)));
          drawn++;
        });
      });
    });
  });
  if (drawn !== 168) throw new Error(`demo-carry: drew ${drawn}, want 168`);
}
