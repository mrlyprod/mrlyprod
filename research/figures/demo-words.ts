import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const SIDE = 105;
const UNIT = 8;

// DEPTH

let memo: Uint8Array | undefined;

function depths() {
  if (memo) return memo;
  const letters = [
    math.bang.MagicLayer.new(new math.name.Bang(7, 2, 2), 3),
    math.bang.MagicLayer.new(new math.name.Bang(14, 2, 2), 7),
    math.bang.MagicLayer.new(new math.name.Bang(9, 2, 2), 5),
  ];
  const first = math.two.create(7, 3, 1, 0, 2);
  const pair = math.bang.magic(letters.slice(0, 2));
  const whole = math.bang.magic(letters);
  if (first.shape.join() !== "3,3") throw new Error(`demo-words: first letter ${first.shape}, want 3,3`);
  if (pair.shape.join() !== "21,21") throw new Error(`demo-words: pair ${pair.shape}, want 21,21`);
  if (whole.shape.join() !== `${SIDE},${SIDE}`) throw new Error(`demo-words: whole ${whole.shape}, want ${SIDE},${SIDE}`);
  const depth = new Uint8Array(SIDE * SIDE);
  const census = [0, 0, 0, 0];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      let deep = 0;
      if (first.types[Math.floor(row / 35) * 3 + Math.floor(col / 35)] !== 0) {
        deep++;
        if (pair.data[Math.floor(row / 5) * 21 + Math.floor(col / 5)] !== 0) {
          deep++;
          if (whole.data[row * SIDE + col] !== 0) deep++;
        }
      }
      depth[row * SIDE + col] = deep;
      census[deep]++;
    }
  }
  if (census.join() !== "1225,3200,3168,3432") throw new Error(`demo-words: census ${census}, want 1225,3200,3168,3432`);
  if (census.reduce((sum, n) => sum + n) !== SIDE * SIDE) throw new Error(`demo-words: census holds ${census.reduce((sum, n) => sum + n)}, want ${SIDE * SIDE}`);
  memo = depth;
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const depth = depths();
  const tones = [ink.line, ink.dim, ink.blue];
  const span = SIDE * UNIT;
  const edge = (pen.width - span) / 2;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const deep = depth[row * SIDE + col];
      if (deep === 0) continue;
      pen.rect(edge + col * UNIT, edge + row * UNIT, UNIT, UNIT, tones[deep - 1]);
    }
  }
}
