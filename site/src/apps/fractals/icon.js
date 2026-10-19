import { plate } from "../../lib/plate.js";

export const units = {};

const SIDE = 63;
const HOME = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };
const VIEW = { xMin: -2.05, xMax: 0.65, yMin: -1.35, yMax: 1.35 };
const MAX = 100;
const HALO = { from: 1, span: 22, gamma: 1.1 };

function escape(cr, ci) {
  let zr = 0;
  let zi = 0;
  let n = 0;
  while (zr * zr + zi * zi <= 4 && n < MAX) {
    const t = zr * zr - zi * zi + cr;
    zi = 2 * zr * zi + ci;
    zr = t;
    n++;
  }
  return n;
}

function count() {
  let inside = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const x = HOME.xMin + ((col + 0.5) / SIDE) * (HOME.xMax - HOME.xMin);
      const y = HOME.yMax - ((row + 0.5) / SIDE) * (HOME.yMax - HOME.yMin);
      if (escape(x, y) === MAX) inside++;
    }
  }
  return inside;
}

function picture(side, white, light) {
  const colors = new Uint8ClampedArray(side * side * 4);
  for (let row = 0; row < side; row++) {
    const y = VIEW.yMax - ((row + 0.5) / side) * (VIEW.yMax - VIEW.yMin);
    for (let col = 0; col < side; col++) {
      const x = VIEW.xMin + ((col + 0.5) / side) * (VIEW.xMax - VIEW.xMin);
      const n = escape(x, y);
      const i = (row * side + col) * 4;
      if (n === MAX) {
        colors[i] = white[0];
        colors[i + 1] = white[1];
        colors[i + 2] = white[2];
        colors[i + 3] = 255;
        continue;
      }
      const t = Math.min(1, Math.max(0, (n - HALO.from) / HALO.span));
      if (t <= 0) continue;
      colors[i] = light[0];
      colors[i + 1] = light[1];
      colors[i + 2] = light[2];
      colors[i + 3] = Math.round(255 * t ** HALO.gamma);
    }
  }
  return { shape: [side, side], colors };
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  const inside = count();
  if (inside !== 695) throw new Error(`fractals: ${inside} cells hold the set, want 695`);
  const side = Math.round(box.w / 2);
  pen.image(box.x, box.y, box.w, box.h, picture(side, paint.white, paint.light));
}
