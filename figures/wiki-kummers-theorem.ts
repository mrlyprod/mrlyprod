import type { Ink, Pen } from "mrlyjs/view";

const P = 3;
const ROWS = 27;
const CELLS = 378;
const CLEAR = 216;

function carries(a: number, b: number) {
  let count = 0;
  let carry = 0;
  while (a > 0 || b > 0 || carry > 0) {
    carry = (a % P) + (b % P) + carry >= P ? 1 : 0;
    count += carry;
    a = Math.floor(a / P);
    b = Math.floor(b / P);
  }
  return count;
}

function valuation(m: number) {
  let v = 0;
  while (m % P === 0) {
    m /= P;
    v++;
  }
  return v;
}

function room(n: number) {
  let product = 1;
  while (n > 0) {
    product *= (n % P) + 1;
    n = Math.floor(n / P);
  }
  return product;
}

export default function draw(pen: Pen, ink: Ink) {
  const triangle: number[][] = [];
  for (let n = 0; n < ROWS; n++) {
    triangle.push(Array.from({ length: n + 1 }, (_, k) => (k === 0 || k === n ? 1 : triangle[n - 1][k - 1] + triangle[n - 1][k])));
  }
  const frame = pen.frame(0.06);
  const step = frame.w / ROWS;
  const [cx] = frame.center();
  let cells = 0;
  let clear = 0;
  let shallow = 0;
  let top = 0;
  for (let n = 0; n < ROWS; n++) {
    let across = 0;
    for (let k = 0; k <= n; k++) {
      const count = carries(k, n - k);
      if (valuation(triangle[n][k]) !== count) throw new Error(`wiki-kummers-theorem: v_3(C(${n}, ${k})) is ${valuation(triangle[n][k])}, carries ${count}`);
      const x = cx + (k - n / 2) * step;
      const y = frame.y + (n + 0.5) * step;
      cells++;
      if (count === 0) {
        pen.disc(x, y, step * 0.42, ink.blue);
        clear++;
        across++;
        if (n < 9) shallow++;
        if (n < 3) top++;
      } else {
        pen.disc(x, y, step * 0.11, ink.dim);
      }
    }
    if (across !== room(n)) throw new Error(`wiki-kummers-theorem: row ${n} has ${across} clear cells, digits give ${room(n)}`);
    if (n === ROWS - 1 && across !== ROWS) throw new Error(`wiki-kummers-theorem: row ${n} has ${across} clear cells, want ${ROWS}`);
  }
  if (cells !== CELLS) throw new Error(`wiki-kummers-theorem: ${cells} cells, want ${CELLS}`);
  if (clear !== CLEAR) throw new Error(`wiki-kummers-theorem: ${clear} clear cells, want ${CLEAR}`);
  if (top !== 6 || shallow !== 36 || clear !== 6 * shallow) throw new Error(`wiki-kummers-theorem: blocks ${top}, ${shallow}, ${clear}, want 6, 36, 216`);
}
