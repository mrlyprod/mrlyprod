import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODES = [15, 11, 10, 5, 1, 9, 7, 13, 12, 3, 6, 14];
const SIDE = 9;
const PILES = 2;
const LIFT = 2;
const APART = 18;

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[] | undefined;

function shelf() {
  if (memo) return memo;
  const deep = CODES.length / PILES;
  const last = deep - 1;
  const wide = (PILES - 1) * APART + SIDE;
  const high = last * LIFT + 1;
  const sheets = CODES.map((code) => {
    const sheet = math.two.create(code, 3, 2, 0, 2);
    const seed = math.two.create(code, 3, 1, 0, 2);
    if (sheet.shape.length !== 2 || sheet.shape[0] !== SIDE || sheet.shape[1] !== SIDE) throw new Error(`site-papers: design ${code} is ${sheet.shape}, want ${SIDE} by ${SIDE}`);
    const count = math.two.fills(sheet);
    const square = math.two.fills(seed) ** 2;
    if (count !== square) throw new Error(`site-papers: design ${code} fills ${count}, want ${square}`);
    return { count, sheet };
  });
  if (sheets.length !== PILES * deep) throw new Error(`site-papers: ${sheets.length} sheets, want ${PILES * deep}`);
  sheets.sort((a, b) => b.count - a.count);
  const types = new Uint8Array(wide * SIDE * high);
  let want = 0;
  sheets.forEach(({ count, sheet }, i) => {
    want += count;
    const ox = (i % PILES) * APART;
    const oz = Math.floor(i / PILES) * LIFT;
    for (let a = 0; a < SIDE; a++) {
      for (let b = 0; b < SIDE; b++) {
        if (sheet.types[a * SIDE + b] !== 0) types[((ox + a) * SIDE + b) * high + oz] = 1;
      }
    }
  });
  let total = 0;
  for (const kind of types) total += kind;
  if (total !== want) throw new Error(`site-papers: the shelf holds ${total} sites, want ${want}`);
  memo = math.three.quads({ shape: [wide, SIDE, high], types });
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  iso.draw(pen, pen.frame(0.08), shelf(), [ink.fg, ink.blue, ink.blue], null);
}
