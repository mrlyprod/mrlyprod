import { plate } from "../../lib/icon.js";

export const units = {};

const COLS = 63;
const ROWS = 40;
const SUB = 3;
const HOME = { xMin: -1.575, xMax: 1.575, yMin: -1, yMax: 1 };
const SEED = [-0.8, 0.156];
const MAX = 100;
const RIM = [3, 24];
const GLOW = 0.8;
const PIXELS = 256;
const REACH = 1.6;
const TURN = Math.PI / 4;
const WANT = { inside: 688 };

function escape(x, y) {
  let zr = x;
  let zi = y;
  let n = 0;
  while (zr * zr + zi * zi <= 4 && n < MAX) {
    const t = zr * zr - zi * zi + SEED[0];
    zi = 2 * zr * zi + SEED[1];
    zr = t;
    n++;
  }
  return n;
}

let memo;

function field() {
  if (memo) return memo;
  const w = COLS * SUB;
  const h = ROWS * SUB;
  const fine = new Uint8Array(w * h);
  for (let row = 0; row < h; row++) {
    const y = HOME.yMax - ((row + 0.5) / h) * (HOME.yMax - HOME.yMin);
    for (let col = 0; col < w; col++) {
      const x = HOME.xMin + ((col + 0.5) / w) * (HOME.xMax - HOME.xMin);
      fine[row * w + col] = escape(x, y);
    }
  }
  const out = new Uint8Array(COLS * ROWS);
  for (let row = 0; row < ROWS; row++) {
    for (let col = 0; col < COLS; col++) {
      let most = 0;
      for (let j = 0; j < SUB; j++) for (let i = 0; i < SUB; i++) most = Math.max(most, fine[(row * SUB + j) * w + col * SUB + i]);
      out[row * COLS + col] = most;
    }
  }
  for (let at = 0; at < out.length; at++) if (out[at] !== out[out.length - 1 - at]) throw new Error(`julia: cell ${at} breaks the symmetry z to -z`);
  const inside = out.filter((n) => n === MAX).length;
  if (inside !== WANT.inside) throw new Error(`julia: ${inside} cells hold the set, want ${WANT.inside}`);
  memo = turned();
  return memo;
}

function turned() {
  const steps = new Uint8Array(PIXELS * PIXELS);
  const c = Math.cos(TURN);
  const s = Math.sin(TURN);
  for (let row = 0; row < PIXELS; row++) {
    const v = REACH * (1 - (2 * (row + 0.5)) / PIXELS);
    for (let col = 0; col < PIXELS; col++) {
      const u = REACH * ((2 * (col + 0.5)) / PIXELS - 1);
      steps[row * PIXELS + col] = escape(u * c + v * s, v * c - u * s);
    }
  }
  return { shape: [PIXELS, PIXELS], steps };
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "purple", 0.04);
  const { shape, steps } = field();
  const [lo, hi] = RIM;
  const colors = new Uint8ClampedArray(steps.length * 4);
  for (let at = 0; at < steps.length; at++) {
    const n = steps[at];
    if (n >= hi) {
      colors.set(paint.white, at * 4);
      continue;
    }
    const share = Math.max(0, (n - lo) / (hi - lo)) ** GLOW;
    if (share <= 0) continue;
    colors.set(paint.light.slice(0, 3), at * 4);
    colors[at * 4 + 3] = Math.round(255 * share);
  }
  pen.image(box.x, box.y, box.w, box.h, { shape, colors });
}
